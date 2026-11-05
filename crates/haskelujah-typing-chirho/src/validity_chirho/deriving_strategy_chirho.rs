// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rules 7 and 8 (row 694): GHC's checks on a written `newtype` deriving strategy.
//!
//! 7. the `newtype` strategy derives only for a newtype (`GHC-10333`);
//! 8. it needs GeneralizedNewtypeDeriving (`GHC-26557`), which GHC2021 enables and an
//!    explicit Haskell2010 or Haskell98 leaves off.
//!
//! Both point at the class, as GHC's caret does. A standalone `deriving newtype instance`
//! is judged against the local declaration it names; an imported target is not judged.
//! Root measured both codes with GHC 9.14.1 on the D01 controls. Not judged yet, until
//! their GHC contracts are measured: `DerivingStrategies` itself (this compiler still
//! expands GHC2024 as GHC2021), the `anyclass` strategy's DeriveAnyClass requirement, and
//! an unwritten strategy's Haskell2010 refusal.
//! workflow: language-features-chirho/deriving-strategies-chirho

use haskelujah_ast_chirho::decl_chirho::{DeclChirho, DerivingStrategyChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::DeclarationErrorChirho;

/// GHC's code for the newtype strategy on a data type.
const NEWTYPE_ON_DATA_CODE_CHIRHO: &str = "GHC-10333";
/// GHC's code for the newtype strategy without GeneralizedNewtypeDeriving.
const NEWTYPE_WITHOUT_EXTENSION_CODE_CHIRHO: &str = "GHC-26557";

/// Rules 7 and 8 over every deriving clause and standalone deriving declaration.
/// workflow: language-features-chirho/deriving-strategies-chirho
pub(super) fn check_deriving_strategies_chirho(
    module_chirho: &ModuleChirho,
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let enabled_chirho = newtype_deriving_enabled_chirho(module_chirho);
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            } => {
                for derived_chirho in deriving_chirho.iter().filter(|derived_chirho| {
                    writes_newtype_chirho(derived_chirho.strategy_chirho.as_ref())
                }) {
                    errors_chirho.push(newtype_on_data_chirho(
                        &derived_chirho.class_chirho,
                        name_chirho.text_chirho(),
                    ));
                }
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                deriving_chirho,
                ..
            } if !enabled_chirho => {
                for derived_chirho in deriving_chirho.iter().filter(|derived_chirho| {
                    writes_newtype_chirho(derived_chirho.strategy_chirho.as_ref())
                }) {
                    errors_chirho.push(newtype_without_extension_chirho(
                        &derived_chirho.class_chirho,
                        name_chirho.text_chirho(),
                    ));
                }
            }
            DeclChirho::StandaloneDerivingDeclChirho {
                strategy_chirho,
                class_chirho,
                types_chirho,
                ..
            } if writes_newtype_chirho(strategy_chirho.as_ref()) => {
                let Some(target_chirho) = types_chirho
                    .first()
                    .and_then(crate::deriving_chirho::target_name_chirho)
                else {
                    continue;
                };
                match declared_newtype_chirho(module_chirho, target_chirho) {
                    Some(false) => {
                        errors_chirho.push(newtype_on_data_chirho(class_chirho, target_chirho));
                    }
                    Some(true) if !enabled_chirho => {
                        errors_chirho.push(newtype_without_extension_chirho(
                            class_chirho,
                            target_chirho,
                        ));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

/// Whether a written strategy is `newtype`.
fn writes_newtype_chirho(strategy_chirho: Option<&DerivingStrategyChirho>) -> bool {
    strategy_chirho.is_some_and(DerivingStrategyChirho::is_newtype_chirho)
}

/// GeneralizedNewtypeDeriving in either spelling: the last explicit flag wins, as GHC's
/// flags override the language edition in any order; without one, GHC2021 (the default
/// edition) enables it and an explicit Haskell2010 or Haskell98 does not.
fn newtype_deriving_enabled_chirho(module_chirho: &ModuleChirho) -> bool {
    module_chirho
        .extensions_chirho
        .iter()
        .fold(
            None,
            |written_chirho, extension_chirho| match extension_chirho.as_str() {
                "GeneralizedNewtypeDeriving" | "GeneralisedNewtypeDeriving" => Some(true),
                "NoGeneralizedNewtypeDeriving" | "NoGeneralisedNewtypeDeriving" => Some(false),
                _ => written_chirho,
            },
        )
        .unwrap_or_else(|| !crate::validity_chirho::declares_legacy_edition_chirho(module_chirho))
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

/// Rule 7, at the class.
fn newtype_on_data_chirho(
    class_chirho: &NameChirho,
    type_text_chirho: &str,
) -> DeclarationErrorChirho {
    DeclarationErrorChirho {
        ghc_code_chirho: NEWTYPE_ON_DATA_CODE_CHIRHO,
        message_chirho: format!(
            "Can't make a derived instance of `{} {type_text_chirho}` with the newtype strategy: GeneralizedNewtypeDeriving cannot be used on non-newtypes",
            class_chirho.text_chirho()
        ),
        span_chirho: class_chirho.span_chirho(),
        related_span_chirho: None,
    }
}

/// Rule 8, at the class.
fn newtype_without_extension_chirho(
    class_chirho: &NameChirho,
    type_text_chirho: &str,
) -> DeclarationErrorChirho {
    DeclarationErrorChirho {
        ghc_code_chirho: NEWTYPE_WITHOUT_EXTENSION_CODE_CHIRHO,
        message_chirho: format!(
            "Can't make a derived instance of `{} {type_text_chirho}` with the newtype strategy: GeneralizedNewtypeDeriving is not enabled",
            class_chirho.text_chirho()
        ),
        span_chirho: class_chirho.span_chirho(),
        related_span_chirho: None,
    }
}

#[cfg(test)]
#[path = "deriving_strategy_tests_chirho.rs"]
mod tests_chirho;
