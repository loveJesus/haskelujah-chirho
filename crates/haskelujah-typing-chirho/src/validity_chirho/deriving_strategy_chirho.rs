// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 7 to 15 (row 694): GHC's checks on deriving strategies, the extensions they need and
//! the classes they derive. Root's GHC 9.14.1 contracts pin rules 7 to 12, and the boundary
//! probes measured at the cf6f7976 gate pin rules 13 and 14 (W2, S1) and rule 15's gap (W3).
//!
//! 7. a written strategy needs DerivingStrategies, or DerivingVia (`GHC-87139`, A1-A3);
//! 8. the `newtype` strategy derives only for a newtype (`GHC-10333`);
//! 9. the `newtype` strategy needs GeneralizedNewtypeDeriving (`GHC-26557`, O1);
//! 10. the `anyclass` strategy needs DeriveAnyClass (`GHC-38178`, B3);
//! 11. with no strategy written, a data type derives a class that is not stock derivable only
//!     with DeriveAnyClass (`GHC-00158`, B1);
//! 12. with no strategy written, a newtype derives such a class only with DeriveAnyClass or
//!     GeneralizedNewtypeDeriving (`GHC-82023`, C1, C4, O3);
//! 13. a derived class takes exactly one parameter after its written arguments: a stock class
//!     takes none, a local or proven builtin class one fewer than its parameters (`GHC-73993`,
//!     W2). A standalone head that gives a class the wrong number of types is refused with
//!     GHC's code still unmeasured;
//! 14. the `stock` strategy derives only a stock derivable builtin class K01 proved for the
//!     spelling: the module's own class, or a proven builtin class GHC does not derive that
//!     way, is `GHC-00158` (S1); a spelling K01 cannot judge (an imported or re-exported
//!     source class, Read, Generic) is a named Unsupported gap, never a presumed stock class
//!     or a GHC claim (#31786);
//! 15. a class applied to arguments does not derive through GND yet, because the dictionary
//!     pass keys a representation dictionary by one head type. This is a compatibility gap, not
//!     a GHC rule: GHC accepts W3 and prints `MeterChirho 12`.
//!
//! Rule 7 points at the strategy keyword and every other rule at the class, as GHC's caret
//! does. An entry is judged by the first rule that refuses it, in GHC's order: the renamer's
//! rule 7, then rule 13, then its strategy's rules. Stock derivability is
//! `stock_identity_chirho.rs`'s judgment, the one the deriving pass routes by. A standalone
//! declaration derives for its head's last type, with the types before it as the class's
//! arguments. Rule 13 judges every standalone declaration; the other rules judge one with a
//! written strategy for a local target.
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, DerivedClassChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, NameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::{DeclarationErrorChirho, UNMEASURED_GHC_CODE_CHIRHO, UNSUPPORTED_SHAPE_CHIRHO};
use crate::deriving_chirho::extension_policy_chirho::DerivingExtensionsChirho;
use crate::deriving_chirho::stock_identity_chirho::StockIdentityChirho;
use crate::validity_chirho::render_type_chirho;

/// What judges one module's deriving requests: its extensions and its class identities.
struct DerivingJudgeChirho<'a> {
    extensions_chirho: DerivingExtensionsChirho,
    identity_chirho: StockIdentityChirho<'a>,
}

/// Rules 7 to 15 over every deriving clause and standalone deriving declaration, with the
/// builtin classes naming proved for the module.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn check_deriving_strategies_chirho(
    module_chirho: &ModuleChirho,
    builtin_classes_chirho: &BuiltinClassScopeChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let judge_chirho = DerivingJudgeChirho {
        extensions_chirho: DerivingExtensionsChirho::of_module_chirho(module_chirho),
        identity_chirho: StockIdentityChirho::of_module_chirho(
            module_chirho,
            builtin_classes_chirho,
        ),
    };
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            }
            | DeclChirho::NewtypeDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            } => {
                let is_newtype_chirho = matches!(decl_chirho, DeclChirho::NewtypeDeclChirho { .. });
                for derived_chirho in deriving_chirho {
                    judge_chirho.check_class_chirho(
                        derived_chirho,
                        name_chirho.text_chirho(),
                        is_newtype_chirho,
                        false,
                        errors_chirho,
                    );
                }
            }
            DeclChirho::StandaloneDerivingDeclChirho {
                strategy_chirho,
                class_chirho,
                types_chirho,
                ..
            } => {
                let Some((target_type_chirho, arguments_chirho)) = types_chirho.split_last() else {
                    continue;
                };
                let derived_chirho = DerivedClassChirho::applied_chirho(
                    class_chirho.clone(),
                    arguments_chirho.to_vec(),
                    strategy_chirho.clone(),
                );
                let local_target_chirho = crate::deriving_chirho::target_name_chirho(
                    target_type_chirho,
                )
                .and_then(|target_chirho| {
                    Some((
                        target_chirho,
                        declared_newtype_chirho(module_chirho, target_chirho)?,
                    ))
                });
                match local_target_chirho {
                    Some((target_chirho, is_newtype_chirho)) if strategy_chirho.is_some() => {
                        judge_chirho.check_class_chirho(
                            &derived_chirho,
                            target_chirho,
                            is_newtype_chirho,
                            true,
                            errors_chirho,
                        );
                    }
                    _ => errors_chirho
                        .extend(judge_chirho.parameter_error_chirho(&derived_chirho, true)),
                }
            }
            _ => {}
        }
    }
}

impl DerivingJudgeChirho<'_> {
    /// Rules 7 to 15 for one derived class of a newtype (`is_newtype_chirho`) or data type,
    /// from a deriving clause or, when `standalone_chirho`, a standalone declaration.
    fn check_class_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
        type_text_chirho: &str,
        is_newtype_chirho: bool,
        standalone_chirho: bool,
        errors_chirho: &mut Vec<DeclarationErrorChirho>,
    ) {
        let extensions_chirho = self.extensions_chirho;
        if let Some(strategy_chirho) = &derived_chirho.strategy_chirho
            && !extensions_chirho.strategies_chirho
        {
            errors_chirho.push(DeclarationErrorChirho {
                ghc_code_chirho: "GHC-87139",
                message_chirho: format!(
                    "Illegal deriving strategy: {}",
                    strategy_chirho.keyword_chirho().text_chirho()
                ),
                span_chirho: strategy_chirho.keyword_chirho().span_chirho(),
                related_span_chirho: None,
            });
            return;
        }
        if let Some(error_chirho) = self.parameter_error_chirho(derived_chirho, standalone_chirho) {
            errors_chirho.push(error_chirho);
            return;
        }
        let class_chirho = &derived_chirho.class_chirho;
        let head_chirho = format!(
            "`{} {type_text_chirho}`",
            applied_class_text_chirho(derived_chirho)
        );
        // The legacy route's judgment, for an unwritten strategy.
        let stock_chirho = self
            .identity_chirho
            .legacy_stock_class_chirho(class_chirho)
            .is_some();
        let unsupported_chirho = || {
            class_error_chirho(
                UNSUPPORTED_SHAPE_CHIRHO,
                format!(
                    "Deriving {head_chirho} through the newtype strategy is not supported yet: its class takes arguments, and the representation dictionary is keyed by one head type"
                ),
                class_chirho,
            )
        };
        let Some(strategy_chirho) = &derived_chirho.strategy_chirho else {
            let other_route_chirho = extensions_chirho.any_class_chirho
                || (is_newtype_chirho && extensions_chirho.newtype_chirho);
            if !stock_chirho && !other_route_chirho {
                errors_chirho.push(class_error_chirho(
                    if is_newtype_chirho { "GHC-82023" } else { "GHC-00158" },
                    format!(
                        "Can't make a derived instance of {head_chirho}: `{}` is not a stock derivable class (Eq, Show, etc.)",
                        class_chirho.text_chirho()
                    ),
                    class_chirho,
                ));
            } else if !stock_chirho
                && !extensions_chirho.any_class_chirho
                && is_newtype_chirho
                && derived_chirho.has_class_arguments_chirho()
            {
                errors_chirho.push(unsupported_chirho());
            }
            return;
        };
        if strategy_chirho.is_newtype_chirho() && !is_newtype_chirho {
            errors_chirho.push(class_error_chirho(
                "GHC-10333",
                format!(
                    "Can't make a derived instance of {head_chirho} with the newtype strategy: GeneralizedNewtypeDeriving cannot be used on non-newtypes"
                ),
                class_chirho,
            ));
        } else if strategy_chirho.is_newtype_chirho() && !extensions_chirho.newtype_chirho {
            errors_chirho.push(class_error_chirho(
                "GHC-26557",
                format!(
                    "Can't make a derived instance of {head_chirho} with the newtype strategy: GeneralizedNewtypeDeriving is not enabled"
                ),
                class_chirho,
            ));
        } else if strategy_chirho.is_newtype_chirho() && derived_chirho.has_class_arguments_chirho()
        {
            errors_chirho.push(unsupported_chirho());
        } else if strategy_chirho.is_anyclass_chirho() && !extensions_chirho.any_class_chirho {
            errors_chirho.push(class_error_chirho(
                "GHC-38178",
                format!(
                    "Can't make a derived instance of {head_chirho} with the anyclass strategy: DeriveAnyClass is not enabled"
                ),
                class_chirho,
            ));
        } else if strategy_chirho.is_stock_chirho()
            && self
                .identity_chirho
                .proven_stock_class_chirho(class_chirho)
                .is_none()
        {
            let known_chirho = self.identity_chirho.proves_builtin_chirho(class_chirho)
                || self.identity_chirho.names_local_chirho(class_chirho);
            errors_chirho.push(if known_chirho {
                class_error_chirho(
                    "GHC-00158",
                    format!(
                        "Can't make a derived instance of {head_chirho} with the stock strategy: `{}` is not a stock derivable class (Eq, Show, etc.)",
                        class_chirho.text_chirho()
                    ),
                    class_chirho,
                )
            } else {
                class_error_chirho(
                    UNSUPPORTED_SHAPE_CHIRHO,
                    format!(
                        "Deriving {head_chirho} with the stock strategy is not supported yet: Haskelujah cannot prove that `{}` is a builtin stock derivable class",
                        class_chirho.full_name_chirho()
                    ),
                    class_chirho,
                )
            });
        }
    }

    /// Rule 13: the class must take exactly one parameter after its written arguments, where
    /// the module knows how many it takes.
    fn parameter_error_chirho(
        &self,
        derived_chirho: &DerivedClassChirho,
        standalone_chirho: bool,
    ) -> Option<DeclarationErrorChirho> {
        let class_chirho = &derived_chirho.class_chirho;
        let parameters_chirho = self
            .identity_chirho
            .parameter_count_chirho(derived_chirho)?;
        let given_chirho = derived_chirho.class_arguments_chirho.len() + 1;
        if parameters_chirho == given_chirho {
            return None;
        }
        Some(if standalone_chirho {
            class_error_chirho(
                UNMEASURED_GHC_CODE_CHIRHO,
                format!(
                    "`{}` takes {parameters_chirho} type arguments, but this instance head gives it {given_chirho}",
                    class_chirho.text_chirho()
                ),
                class_chirho,
            )
        } else {
            class_error_chirho(
                "GHC-73993",
                format!(
                    "`{}` is not a unary constraint, as expected by a deriving clause",
                    applied_class_text_chirho(derived_chirho)
                ),
                class_chirho,
            )
        })
    }
}

/// The class with its written arguments: `PairChirho Int`, `C (Maybe Int)`.
fn applied_class_text_chirho(derived_chirho: &DerivedClassChirho) -> String {
    std::iter::once(derived_chirho.class_text_chirho().to_string())
        .chain(
            derived_chirho
                .class_arguments_chirho
                .iter()
                .map(argument_text_chirho),
        )
        .collect::<Vec<_>>()
        .join(" ")
}

/// One argument, parenthesized when it is itself an application or a function.
fn argument_text_chirho(argument_chirho: &TypeChirho) -> String {
    let mut bare_chirho = argument_chirho;
    while let TypeChirho::ParenChirho { inner_chirho, .. } = bare_chirho {
        bare_chirho = inner_chirho;
    }
    match bare_chirho {
        TypeChirho::AppChirho { .. } | TypeChirho::FunChirho { .. } => {
            format!("({})", render_type_chirho(bare_chirho))
        }
        _ => render_type_chirho(bare_chirho),
    }
}

/// Whether the module declares `name_chirho` as a newtype (`Some(true)`) or a data type
/// (`Some(false)`); `None` when it declares neither.
fn declared_newtype_chirho(module_chirho: &ModuleChirho, name_chirho: &str) -> Option<bool> {
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho: declared_chirho,
                ..
            } if declared_chirho.text_chirho() == name_chirho => Some(true),
            DeclChirho::DataDeclChirho {
                name_chirho: declared_chirho,
                ..
            } if declared_chirho.text_chirho() == name_chirho => Some(false),
            _ => None,
        })
}

/// A refusal pointing at the class.
fn class_error_chirho(
    ghc_code_chirho: &'static str,
    message_chirho: String,
    class_chirho: &NameChirho,
) -> DeclarationErrorChirho {
    DeclarationErrorChirho {
        ghc_code_chirho,
        message_chirho,
        span_chirho: class_chirho.span_chirho(),
        related_span_chirho: None,
    }
}

#[cfg(test)]
#[path = "deriving_strategy_tests_chirho.rs"]
mod tests_chirho;
