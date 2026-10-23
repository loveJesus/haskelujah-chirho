// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Comparisons by constructor order for enumerations whose constructors carry no fields.

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, InlineAnnotationChirho,
};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

/// Builtin types whose `compare` is their constructors' declaration order. Their
/// values are constructors, which the Int primop `compare#` cannot order.
const CONSTRUCTOR_ORDERS_CHIRHO: &[(&str, &[&str])] = &[
    ("Bool", &["False", "True"]),
    ("Ordering", &["LT", "EQ", "GT"]),
];

/// The constructors, in order, of a builtin type compared by constructor order.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(super) fn constructor_order_chirho(type_key_chirho: &str) -> Option<&'static [&'static str]> {
    CONSTRUCTOR_ORDERS_CHIRHO
        .iter()
        .find(|(key_chirho, _)| *key_chirho == type_key_chirho)
        .map(|(_, constructors_chirho)| *constructors_chirho)
}

impl DictPassCtxChirho {
    /// Generate `$prim_Ord_compare_<T> = \x y -> ...` for at least two fieldless
    /// constructors given in declaration order. The outer case names every
    /// constructor of `x`. Against `y`, the first constructor answers EQ for
    /// itself and LT otherwise, the last answers EQ for itself and GT otherwise,
    /// and each one between names every constructor. For Ordering this is the
    /// table LT < EQ < GT has always had: LT vs LT -> EQ, LT vs _ -> LT,
    /// EQ vs LT -> GT, EQ vs EQ -> EQ, EQ vs GT -> LT, GT vs GT -> EQ, GT vs _ -> GT.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_constructor_order_compare_chirho(
        &mut self,
        prim_name_chirho: &str,
        constructors_chirho: &[&str],
    ) {
        debug_assert!(constructors_chirho.len() >= 2);
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", ord_ty_chirho.clone());
        let scr_x_chirho = self.fresh_binder_chirho("_sx", ord_ty_chirho.clone());
        let scr_ys_chirho: Vec<_> = (1..=constructors_chirho.len())
            .map(|index_chirho| {
                self.fresh_binder_chirho(&format!("_sy{index_chirho}"), ord_ty_chirho.clone())
            })
            .collect();

        let ordering_chirho = |name_chirho: &str| CoreExprChirho::ConAppChirho {
            con_name_chirho: name_chirho.to_string(),
            args_chirho: vec![],
        };
        let data_con_chirho =
            |name_chirho: &str| AltConChirho::DataConChirho(name_chirho.to_string());
        let alt_chirho = |con_chirho: AltConChirho, rhs_chirho: CoreExprChirho| CoreAltChirho {
            con_chirho,
            binders_chirho: vec![],
            rhs_chirho,
        };
        let last_chirho = constructors_chirho.len() - 1;

        let outer_alts_chirho: Vec<CoreAltChirho> = constructors_chirho
            .iter()
            .zip(scr_ys_chirho)
            .enumerate()
            .map(|(index_chirho, (constructor_chirho, scr_y_chirho))| {
                let inner_alts_chirho = if index_chirho == 0 {
                    vec![
                        alt_chirho(data_con_chirho(constructor_chirho), ordering_chirho("EQ")),
                        alt_chirho(AltConChirho::DefaultChirho, ordering_chirho("LT")),
                    ]
                } else if index_chirho == last_chirho {
                    vec![
                        alt_chirho(data_con_chirho(constructor_chirho), ordering_chirho("EQ")),
                        alt_chirho(AltConChirho::DefaultChirho, ordering_chirho("GT")),
                    ]
                } else {
                    constructors_chirho
                        .iter()
                        .enumerate()
                        .map(|(other_chirho, other_name_chirho)| {
                            let answer_chirho = match other_chirho.cmp(&index_chirho) {
                                std::cmp::Ordering::Less => "GT",
                                std::cmp::Ordering::Equal => "EQ",
                                std::cmp::Ordering::Greater => "LT",
                            };
                            alt_chirho(
                                data_con_chirho(other_name_chirho),
                                ordering_chirho(answer_chirho),
                            )
                        })
                        .collect()
                };
                alt_chirho(
                    data_con_chirho(constructor_chirho),
                    CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
                        bind_chirho: scr_y_chirho,
                        result_ty_chirho: ord_ty_chirho.clone(),
                        alts_chirho: inner_alts_chirho,
                    },
                )
            })
            .collect();

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_x_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: outer_alts_chirho,
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(body_chirho),
            }),
        };

        let fn_ty_chirho = TyChirho::fun_chirho(
            ord_ty_chirho.clone(),
            TyChirho::fun_chirho(ord_ty_chirho.clone(), ord_ty_chirho),
        );
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, fn_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
