// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! # Source-type validity checks
//!
//! GHC rejects some *well-kinded* types outright because the extension that
//! would license them is not enabled. This module implements that family of
//! checks (GHC error code `GHC-91510`): a polymorphic type (`forall`) or a
//! qualified type (`ctx =>`) is only legal in certain syntactic positions.
//!
//! Haskell2010 allows a `forall`/context only at the very top of a signature.
//! Nesting one under an arrow is rank-N and needs `RankNTypes`; putting one in
//! a type-constructor argument is impredicative and needs `ImpredicativeTypes`.
//!
//! This lives in its own module rather than inside `infer_chirho.rs` on
//! purpose: these are *syntactic* well-formedness rules that need no
//! inference state, and `infer_chirho.rs` is already far past the size a
//! single file should carry.

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

/// `E0206`: a type form that needs an extension which is not enabled.
const ILLEGAL_TYPE_FORM_CODE_CHIRHO: u16 = 206;

/// A source type that is well-kinded but not licensed by the enabled
/// extensions.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidityErrorChirho {
    /// Human-readable message, phrased like GHC's `GHC-91510`.
    pub message_chirho: String,
    /// The extension that would license this type, if any.
    pub suggested_extension_chirho: Option<&'static str>,
    /// Where the offending type appears.
    pub span_chirho: SpanChirho,
}

/// Result of the source-type validity pass.
pub struct ValidityResultChirho {
    /// Diagnostics produced by the pass.
    pub diagnostics_chirho: DiagnosticBundleChirho,
}

/// Run the validity pass and package the findings as diagnostics.
pub fn check_module_type_validity_diagnostics_chirho(
    module_chirho: &ModuleChirho,
) -> ValidityResultChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    for error_chirho in check_module_type_validity_chirho(module_chirho) {
        let mut diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(ILLEGAL_TYPE_FORM_CODE_CHIRHO),
            error_chirho.message_chirho,
            error_chirho.span_chirho,
        );
        if let Some(extension_chirho) = error_chirho.suggested_extension_chirho {
            diagnostic_chirho = diagnostic_chirho.with_note_chirho(format!(
                "perhaps you intended to use the `{extension_chirho}` extension"
            ));
        }
        diagnostics_chirho.push_chirho(diagnostic_chirho);
    }
    ValidityResultChirho { diagnostics_chirho }
}

/// Where a type sits relative to the enclosing signature. The position
/// decides which extension (if any) is needed to license a `forall` or a
/// context appearing there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TyPositionChirho {
    /// The very top of a signature. Haskell2010 already allows a leading
    /// `forall` and a leading context here.
    TopChirho,
    /// Nested beneath a function arrow — rank-N. Needs `RankNTypes`.
    RankNChirho,
    /// An argument of a type constructor, list, or tuple — impredicative.
    /// Needs `ImpredicativeTypes`.
    ImpredicativeChirho,
}

/// Check every source type in a module for extension-licensed forms.
///
/// Only signatures and type-synonym right-hand sides are checked. Data
/// constructor fields are deliberately left alone for now: existential and
/// GADT field types have their own rules, and widening this check without
/// them would produce false rejections.
pub fn check_module_type_validity_chirho(module_chirho: &ModuleChirho) -> Vec<ValidityErrorChirho> {
    // GHC: ImpredicativeTypes implies RankNTypes. `Rank2Types` is a legacy
    // spelling of `RankNTypes` and still widely used in real source.
    let impredicative_enabled_chirho =
        extension_enabled_chirho(module_chirho, "ImpredicativeTypes");
    let rank_n_enabled_chirho = impredicative_enabled_chirho
        || extension_enabled_chirho(module_chirho, "RankNTypes")
        || extension_enabled_chirho(module_chirho, "Rank2Types")
        // A module that never declares a language edition is compiled by
        // modern GHC under GHC2021, which enables RankNTypes. We do not model
        // language editions, so assuming rank-N is OFF would reject ordinary
        // modern code. Only an explicit legacy edition turns it off.
        || !declares_legacy_edition_chirho(module_chirho);

    let mut errors_chirho = Vec::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::TypeSigChirho { ty_chirho, .. } => {
                walk_type_chirho(
                    ty_chirho,
                    TyPositionChirho::TopChirho,
                    rank_n_enabled_chirho,
                    impredicative_enabled_chirho,
                    &mut errors_chirho,
                );
            }
            DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } => {
                // A synonym RHS is not a signature top: `type C a = Num a => a`
                // is rejected by GHC without RankNTypes.
                walk_type_chirho(
                    rhs_chirho,
                    TyPositionChirho::RankNChirho,
                    rank_n_enabled_chirho,
                    impredicative_enabled_chirho,
                    &mut errors_chirho,
                );
            }
            _ => {}
        }
    }
    errors_chirho
}

/// True when the module explicitly asks for a pre-GHC2021 language edition,
/// where rank-N types are genuinely off unless requested.
fn declares_legacy_edition_chirho(module_chirho: &ModuleChirho) -> bool {
    extension_enabled_chirho(module_chirho, "Haskell2010")
        || extension_enabled_chirho(module_chirho, "Haskell98")
}

fn extension_enabled_chirho(module_chirho: &ModuleChirho, name_chirho: &str) -> bool {
    module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| extension_chirho == name_chirho)
}

fn walk_type_chirho(
    ty_chirho: &TypeChirho,
    position_chirho: TyPositionChirho,
    rank_n_enabled_chirho: bool,
    impredicative_enabled_chirho: bool,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    match ty_chirho {
        TypeChirho::ForallChirho {
            body_chirho,
            span_chirho,
            ..
        } => {
            report_if_unlicensed_chirho(
                "polymorphic",
                ty_chirho,
                position_chirho,
                *span_chirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
            // A leading `forall` does not itself move us out of top position.
            walk_type_chirho(
                body_chirho,
                position_chirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
        }
        TypeChirho::QualChirho {
            body_chirho,
            span_chirho,
            ..
        } => {
            report_if_unlicensed_chirho(
                "qualified",
                ty_chirho,
                position_chirho,
                *span_chirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
            walk_type_chirho(
                body_chirho,
                position_chirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            // Both sides of an arrow are rank-N positions: `(forall a. a) -> b`
            // and `b -> forall a. a` both need RankNTypes.
            for side_chirho in [arg_chirho, result_chirho] {
                walk_type_chirho(
                    side_chirho,
                    TyPositionChirho::RankNChirho,
                    rank_n_enabled_chirho,
                    impredicative_enabled_chirho,
                    errors_chirho,
                );
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            // The head keeps the current position; the argument is
            // impredicative (`Maybe (forall a. a)`).
            walk_type_chirho(
                fun_chirho,
                position_chirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
            walk_type_chirho(
                arg_chirho,
                TyPositionChirho::ImpredicativeChirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            walk_type_chirho(
                element_chirho,
                TyPositionChirho::ImpredicativeChirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                walk_type_chirho(
                    element_chirho,
                    TyPositionChirho::ImpredicativeChirho,
                    rank_n_enabled_chirho,
                    impredicative_enabled_chirho,
                    errors_chirho,
                );
            }
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            walk_type_chirho(
                inner_chirho,
                position_chirho,
                rank_n_enabled_chirho,
                impredicative_enabled_chirho,
                errors_chirho,
            );
        }
        TypeChirho::VarChirho(_)
        | TypeChirho::ConChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn report_if_unlicensed_chirho(
    kind_chirho: &str,
    ty_chirho: &TypeChirho,
    position_chirho: TyPositionChirho,
    span_chirho: SpanChirho,
    rank_n_enabled_chirho: bool,
    impredicative_enabled_chirho: bool,
    errors_chirho: &mut Vec<ValidityErrorChirho>,
) {
    let needed_chirho = match position_chirho {
        TyPositionChirho::TopChirho => None,
        TyPositionChirho::RankNChirho if !rank_n_enabled_chirho => Some("RankNTypes"),
        TyPositionChirho::ImpredicativeChirho if !impredicative_enabled_chirho => {
            Some("ImpredicativeTypes")
        }
        _ => None,
    };
    let Some(extension_chirho) = needed_chirho else {
        return;
    };
    errors_chirho.push(ValidityErrorChirho {
        message_chirho: format!(
            "Illegal {kind_chirho} type: {}",
            render_type_chirho(ty_chirho)
        ),
        suggested_extension_chirho: Some(extension_chirho),
        span_chirho,
    });
}

/// Compact rendering used only inside diagnostics.
fn render_type_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) | TypeChirho::ConChirho(name_chirho) => {
            name_chirho.text_chirho().to_string()
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => format!(
            "{} {}",
            render_type_chirho(fun_chirho),
            render_type_chirho(arg_chirho)
        ),
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => format!(
            "{} -> {}",
            render_type_chirho(arg_chirho),
            render_type_chirho(result_chirho)
        ),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => format!(
            "({})",
            elements_chirho
                .iter()
                .map(render_type_chirho)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", render_type_chirho(element_chirho))
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => render_type_chirho(inner_chirho),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            let rendered_chirho: Vec<String> =
                context_chirho.iter().map(render_constraint_chirho).collect();
            let context_text_chirho = match rendered_chirho.len() {
                1 => rendered_chirho[0].clone(),
                _ => format!("({})", rendered_chirho.join(", ")),
            };
            format!(
                "{context_text_chirho} => {}",
                render_type_chirho(body_chirho)
            )
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let names_chirho: Vec<String> = vars_chirho
                .iter()
                .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
                .collect();
            format!(
                "forall {}. {}",
                names_chirho.join(" "),
                render_type_chirho(body_chirho)
            )
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            format!("'{}", name_chirho.text_chirho())
        }
        TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => format!(
            "'[{}]",
            elements_chirho
                .iter()
                .map(render_type_chirho)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeChirho::WildcardChirho { .. } => "_".to_string(),
        TypeChirho::LitChirho { value_chirho, .. } => value_chirho.clone(),
    }
}

fn render_constraint_chirho(constraint_chirho: &ConstraintChirho) -> String {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => {
            let mut text_chirho = class_chirho.text_chirho().to_string();
            for arg_chirho in args_chirho {
                text_chirho.push(' ');
                text_chirho.push_str(&render_type_chirho(arg_chirho));
            }
            text_chirho
        }
        ConstraintChirho::QuantifiedChirho { body_chirho, .. } => {
            format!("forall … {}", render_constraint_chirho(body_chirho))
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::TyVarChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn mk_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_module_chirho(
        decls_chirho: Vec<DeclChirho>,
        extensions_chirho: Vec<String>,
    ) -> ModuleChirho {
        ModuleChirho {
            name_chirho: mk_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho,
            extensions_chirho,
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_var_chirho(text_chirho: &str) -> TypeChirho {
        TypeChirho::VarChirho(mk_name_chirho(text_chirho))
    }

    fn mk_con_chirho(text_chirho: &str) -> TypeChirho {
        TypeChirho::ConChirho(mk_name_chirho(text_chirho))
    }

    /// `forall a. a -> a`
    fn mk_poly_id_chirho() -> TypeChirho {
        TypeChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho {
                name_chirho: mk_name_chirho("a"),
                kind_annotation_chirho: None,
            }],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(mk_var_chirho("a")),
                mult_chirho: None,
                result_chirho: Box::new(mk_var_chirho("a")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn mk_sig_chirho(ty_chirho: TypeChirho) -> DeclChirho {
        DeclChirho::TypeSigChirho {
            name_chirho: mk_name_chirho("f"),
            ty_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn top_level_forall_is_always_allowed_chirho() {
        // `f :: forall a. a -> a` is plain Haskell2010 with ExplicitForAll.
        let module_chirho = mk_module_chirho(vec![mk_sig_chirho(mk_poly_id_chirho())], vec![]);
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }

    #[test]
    fn rank_two_argument_needs_rank_n_types_chirho() {
        // `f :: (forall a. a -> a) -> Int`
        let ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(mk_poly_id_chirho()),
            mult_chirho: None,
            result_chirho: Box::new(mk_con_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let without_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho.clone())],
            vec!["Haskell2010".to_string()],
        );
        let errors_chirho = check_module_type_validity_chirho(&without_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("RankNTypes")
        );

        let with_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["RankNTypes".to_string()],
        );
        assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
    }

    #[test]
    fn polytype_as_constructor_argument_needs_impredicative_types_chirho() {
        // `f :: Maybe (forall a. a -> a)` — RankNTypes is NOT enough here.
        let ty_chirho = TypeChirho::AppChirho {
            fun_chirho: Box::new(mk_con_chirho("Maybe")),
            arg_chirho: Box::new(mk_poly_id_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let rank_n_only_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho.clone())],
            vec!["RankNTypes".to_string()],
        );
        let errors_chirho = check_module_type_validity_chirho(&rank_n_only_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert_eq!(
            errors_chirho[0].suggested_extension_chirho,
            Some("ImpredicativeTypes")
        );

        let impredicative_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["ImpredicativeTypes".to_string()],
        );
        assert!(check_module_type_validity_chirho(&impredicative_chirho).is_empty());
    }

    #[test]
    fn qualified_type_synonym_rhs_needs_rank_n_types_chirho() {
        // `type Constrd a = Num a => a`
        let rhs_chirho = TypeChirho::QualChirho {
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: mk_name_chirho("Num"),
                args_chirho: vec![mk_var_chirho("a")],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(mk_var_chirho("a")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let decl_chirho = DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("Constrd"),
            type_vars_chirho: vec![TyVarChirho {
                name_chirho: mk_name_chirho("a"),
                kind_annotation_chirho: None,
            }],
            rhs_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(
            vec![decl_chirho.clone()],
            vec!["Haskell2010".to_string()],
        );
        let errors_chirho = check_module_type_validity_chirho(&module_chirho);
        assert_eq!(errors_chirho.len(), 1);
        assert!(
            errors_chirho[0].message_chirho.contains("Num a"),
            "diagnostic should name the constraint, got {}",
            errors_chirho[0].message_chirho
        );

        let with_chirho =
            mk_module_chirho(vec![decl_chirho], vec!["RankNTypes".to_string()]);
        assert!(check_module_type_validity_chirho(&with_chirho).is_empty());
    }

    #[test]
    fn impredicative_types_implies_rank_n_types_chirho() {
        // GHC: ImpredicativeTypes implies RankNTypes, so a rank-2 argument is
        // licensed by ImpredicativeTypes alone.
        let ty_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(mk_poly_id_chirho()),
            mult_chirho: None,
            result_chirho: Box::new(mk_con_chirho("Int")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = mk_module_chirho(
            vec![mk_sig_chirho(ty_chirho)],
            vec!["ImpredicativeTypes".to_string()],
        );
        assert!(check_module_type_validity_chirho(&module_chirho).is_empty());
    }
}

#[cfg(test)]
mod tests_edition_chirho {
    use super::*;
    use haskelujah_ast_chirho::decl_chirho::TyVarChirho;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn mk_name_chirho(text_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    /// `f :: (forall a. a -> a) -> Int` under the given extensions.
    fn rank_two_is_accepted_with_chirho(extensions_chirho: Vec<String>) -> bool {
        let poly_chirho = TypeChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho {
                name_chirho: mk_name_chirho("a"),
                kind_annotation_chirho: None,
            }],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let sig_chirho = DeclChirho::TypeSigChirho {
            name_chirho: mk_name_chirho("f"),
            ty_chirho: TypeChirho::FunChirho {
                arg_chirho: Box::new(poly_chirho),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Int"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("Test"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![sig_chirho],
            extensions_chirho,
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        check_module_type_validity_chirho(&module_chirho).is_empty()
    }

    // Regression guard: the first version of this check assumed rank-N types
    // were off whenever `RankNTypes` was absent. That wrongly rejected 13
    // previously-passing GHC should_compile files, because modern GHC compiles
    // an undeclared module under GHC2021, which enables RankNTypes — and
    // because `Rank2Types` is a legacy spelling of the same extension.
    #[test]
    fn modern_default_edition_allows_rank_n_chirho() {
        assert!(rank_two_is_accepted_with_chirho(vec![]));
    }

    #[test]
    fn legacy_rank2types_spelling_is_honoured_chirho() {
        assert!(rank_two_is_accepted_with_chirho(vec![
            "Rank2Types".to_string()
        ]));
    }

    #[test]
    fn explicit_haskell2010_still_rejects_rank_n_chirho() {
        assert!(!rank_two_is_accepted_with_chirho(vec![
            "Haskell2010".to_string()
        ]));
    }
}
