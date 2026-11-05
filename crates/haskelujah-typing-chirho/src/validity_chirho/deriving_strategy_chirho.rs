// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 7 to 12 (row 694): GHC's checks on deriving strategies and the extensions they need,
//! each pinned by root's GHC 9.14.1 contracts.
//!
//! 7. a written strategy needs DerivingStrategies, or DerivingVia (`GHC-87139`, A1-A3);
//! 8. the `newtype` strategy derives only for a newtype (`GHC-10333`);
//! 9. the `newtype` strategy needs GeneralizedNewtypeDeriving (`GHC-26557`, O1);
//! 10. the `anyclass` strategy needs DeriveAnyClass (`GHC-38178`, B3);
//! 11. with no strategy written, a data type derives a class that is not stock derivable only
//!     with DeriveAnyClass (`GHC-00158`, B1);
//! 12. with no strategy written, a newtype derives such a class only with DeriveAnyClass or
//!     GeneralizedNewtypeDeriving (`GHC-82023`, C1, C4, O3).
//!
//! Rule 7 points at the strategy keyword and every other rule at the class, as GHC's caret
//! does. An entry rule 7 refuses is judged no further. A standalone declaration is judged
//! against the local type it names; an imported target, and a standalone declaration with no
//! written strategy, are not judged here.
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, DerivedClassChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::DeclarationErrorChirho;
use crate::deriving_chirho::extension_policy_chirho::{
    DerivingExtensionsChirho, STOCK_DERIVABLE_CLASSES_CHIRHO,
};

/// Rules 7 to 12 over every deriving clause and standalone deriving declaration.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn check_deriving_strategies_chirho(
    module_chirho: &ModuleChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let extensions_chirho = DerivingExtensionsChirho::of_module_chirho(module_chirho);
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            } => {
                for derived_chirho in deriving_chirho {
                    check_clause_class_chirho(
                        derived_chirho,
                        name_chirho.text_chirho(),
                        false,
                        extensions_chirho,
                        errors_chirho,
                    );
                }
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            } => {
                for derived_chirho in deriving_chirho {
                    check_clause_class_chirho(
                        derived_chirho,
                        name_chirho.text_chirho(),
                        true,
                        extensions_chirho,
                        errors_chirho,
                    );
                }
            }
            DeclChirho::StandaloneDerivingDeclChirho {
                strategy_chirho: Some(strategy_chirho),
                class_chirho,
                types_chirho,
                ..
            } => {
                let Some(target_chirho) = types_chirho
                    .first()
                    .and_then(crate::deriving_chirho::target_name_chirho)
                else {
                    continue;
                };
                let Some(is_newtype_chirho) = declared_newtype_chirho(module_chirho, target_chirho)
                else {
                    continue;
                };
                let derived_chirho = DerivedClassChirho::new_chirho(
                    class_chirho.clone(),
                    Some(strategy_chirho.clone()),
                );
                check_clause_class_chirho(
                    &derived_chirho,
                    target_chirho,
                    is_newtype_chirho,
                    extensions_chirho,
                    errors_chirho,
                );
            }
            _ => {}
        }
    }
}

/// Rules 7 to 12 for one derived class of a newtype (`is_newtype_chirho`) or data type.
fn check_clause_class_chirho(
    derived_chirho: &DerivedClassChirho,
    type_text_chirho: &str,
    is_newtype_chirho: bool,
    extensions_chirho: DerivingExtensionsChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let class_chirho = &derived_chirho.class_chirho;
    let head_chirho = format!("`{} {type_text_chirho}`", class_chirho.text_chirho());
    let Some(strategy_chirho) = &derived_chirho.strategy_chirho else {
        let stock_chirho =
            STOCK_DERIVABLE_CLASSES_CHIRHO.contains(&derived_chirho.class_text_chirho());
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
        }
        return;
    };
    if !extensions_chirho.strategies_chirho {
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
    } else if strategy_chirho.is_anyclass_chirho() && !extensions_chirho.any_class_chirho {
        errors_chirho.push(class_error_chirho(
            "GHC-38178",
            format!(
                "Can't make a derived instance of {head_chirho} with the anyclass strategy: DeriveAnyClass is not enabled"
            ),
            class_chirho,
        ));
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
