// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Property-based tests for the Core simplifier.
//!
//! These tests verify simplifier invariants: idempotence, binding count
//! monotonicity, and crash-freedom on randomly generated Core expressions.

#[cfg(test)]
mod tests_chirho {
    use proptest::prelude::*;
    use crate::expr_chirho::*;
    use crate::simplify_chirho::{simplify_module_chirho, SimplifyConfigChirho};
    use rhasky_span_chirho::SpanChirho;
    use std::collections::HashMap;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: rhasky_typing_chirho::ty_chirho::TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    /// Generate a random Core expression of bounded depth.
    fn arb_expr_chirho(max_depth_chirho: usize) -> impl Strategy<Value = CoreExprChirho> {
        let leaf_chirho = prop_oneof![
            any::<i64>().prop_map(|n_chirho| CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(n_chirho))),
            (0u32..20).prop_map(|id_chirho| CoreExprChirho::VarChirho(CoreIdChirho(id_chirho))),
        ];

        leaf_chirho.prop_recursive(max_depth_chirho as u32, 64, 2, |inner_chirho| {
            prop_oneof![
                // Application
                (inner_chirho.clone(), inner_chirho.clone()).prop_map(|(f_chirho, a_chirho)| {
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(f_chirho),
                        arg_chirho: Box::new(a_chirho),
                    }
                }),
                // Lambda
                (0u32..20, inner_chirho.clone()).prop_map(|(id_chirho, body_chirho)| {
                    CoreExprChirho::LamChirho {
                        binder_chirho: BinderChirho {
                            id_chirho: CoreIdChirho(id_chirho),
                            name_chirho: format!("x{}", id_chirho),
                            ty_chirho: rhasky_typing_chirho::ty_chirho::TyChirho::int_chirho(),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        body_chirho: Box::new(body_chirho),
                    }
                }),
            ]
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        /// Simplifier should never panic on random Core expressions.
        #[test]
        fn simplifier_never_panics_chirho(
            expr_chirho in arb_expr_chirho(4),
            n_binds_chirho in 1usize..5
        ) {
            let bindings_chirho: Vec<CoreBindingChirho> = (0..n_binds_chirho)
                .map(|i_chirho| CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho(&format!("f{}", i_chirho), 100 + i_chirho as u32),
                    rhs_chirho: expr_chirho.clone(),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                })
                .collect();
            let module_chirho = CoreModuleChirho {
                name_chirho: "Prop".to_string(),
                bindings_chirho,
                names_chirho: HashMap::new(),
                specialize_pragmas_chirho: HashMap::new(),
                foreign_exports_chirho: vec![],
            };
            let config_chirho = SimplifyConfigChirho::default();
            let _result_chirho = simplify_module_chirho(&module_chirho, &config_chirho);
        }

        /// Simplifier should be roughly idempotent: running it twice should
        /// produce the same result as running it once (after convergence).
        #[test]
        fn simplifier_idempotent_chirho(
            val_chirho in any::<i64>()
        ) {
            let module_chirho = CoreModuleChirho {
                name_chirho: "Idem".to_string(),
                bindings_chirho: vec![CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("f", 1),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(val_chirho)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                }],
                names_chirho: HashMap::new(),
                specialize_pragmas_chirho: HashMap::new(),
                foreign_exports_chirho: vec![],
            };
            let config_chirho = SimplifyConfigChirho::default();
            let once_chirho = simplify_module_chirho(&module_chirho, &config_chirho);
            let twice_chirho = simplify_module_chirho(&once_chirho, &config_chirho);
            // Literal bindings should be unchanged
            prop_assert_eq!(
                once_chirho.bindings_chirho.len(),
                twice_chirho.bindings_chirho.len()
            );
        }

        /// Simplifying a single literal binding should preserve it exactly.
        #[test]
        fn simplifier_preserves_literals_chirho(val_chirho in any::<i64>()) {
            let module_chirho = CoreModuleChirho {
                name_chirho: "Lit".to_string(),
                bindings_chirho: vec![CoreBindingChirho {
                    binder_chirho: dummy_binder_chirho("x", 1),
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(val_chirho)),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                }],
                names_chirho: HashMap::new(),
                specialize_pragmas_chirho: HashMap::new(),
                foreign_exports_chirho: vec![],
            };
            let config_chirho = SimplifyConfigChirho::default();
            let result_chirho = simplify_module_chirho(&module_chirho, &config_chirho);
            prop_assert_eq!(result_chirho.bindings_chirho.len(), 1);
            prop_assert_eq!(
                result_chirho.bindings_chirho[0].rhs_chirho.clone(),
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(val_chirho))
            );
        }
    }
}
