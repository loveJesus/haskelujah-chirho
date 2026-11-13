// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The independent next pipeline. Legacy inference and dictionary rewriting are never called.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

#[cfg(test)]
#[path = "tests_chirho/class_evidence_tests_chirho.rs"]
mod class_evidence_tests_chirho;
#[cfg(test)]
#[path = "tests_chirho/context_instances_tests_chirho.rs"]
mod context_instances_tests_chirho;
mod core_chirho;
#[cfg(test)]
mod data_patterns_tests_chirho;
#[cfg(test)]
mod generalization_tests_chirho;
mod parse_chirho;
#[cfg(test)]
#[path = "tests_chirho/signature_tests_chirho.rs"]
mod signature_tests_chirho;
#[cfg(test)]
#[path = "tests_chirho/superclass_tests_chirho.rs"]
mod superclass_tests_chirho;
#[cfg(test)]
mod tests_chirho;
mod types_chirho;

use std::fmt;

use haskelujah_core_chirho::CoreModuleChirho;
use haskelujah_hir_chirho::{HirErrorKindChirho, LimitsChirho, UnitIdentityChirho};
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_tc_chirho::{CheckedModuleChirho, TcErrorChirho, TcErrorKindChirho};

#[cfg(test)]
fn assert_character_chirho(source_chirho: &str, expected_chirho: char) {
    let (value_chirho, _) = eval_binding_chirho(source_chirho, "resultChirho", 50_000).unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho(expected_chirho)
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextErrorKindChirho {
    InvalidChirho,
    UnsupportedChirho,
    TypeChirho,
    ResourceChirho,
    ExecutionChirho,
}

#[derive(Debug)]
pub struct NextErrorChirho {
    pub kind_chirho: NextErrorKindChirho,
    pub message_chirho: String,
}

impl NextErrorChirho {
    pub(crate) fn new_chirho(
        kind_chirho: NextErrorKindChirho,
        message_chirho: impl Into<String>,
    ) -> Self {
        Self {
            kind_chirho,
            message_chirho: message_chirho.into(),
        }
    }
}

impl fmt::Display for NextErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter_chirho,
            "next[{:?}]: {}",
            self.kind_chirho, self.message_chirho
        )
    }
}

impl std::error::Error for NextErrorChirho {}

impl From<TcErrorChirho> for NextErrorChirho {
    fn from(error_chirho: TcErrorChirho) -> Self {
        let kind_chirho = match error_chirho.kind_chirho {
            TcErrorKindChirho::UnsupportedChirho => NextErrorKindChirho::UnsupportedChirho,
            TcErrorKindChirho::ResourceChirho => NextErrorKindChirho::ResourceChirho,
            TcErrorKindChirho::InvalidReferenceChirho => NextErrorKindChirho::InvalidChirho,
            _ => NextErrorKindChirho::TypeChirho,
        };
        Self::new_chirho(kind_chirho, error_chirho.message_chirho)
    }
}

/// Parse, resolve identity-bearing HIR, generate constraints, and solve before publication.
/// S0 checks library modules; Main's IO contract requires the source-library slice.
pub fn check_source_chirho(source_chirho: &str) -> Result<CheckedModuleChirho, NextErrorChirho> {
    let parsed_chirho = parse_chirho::parse_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)?;
    let module_chirho = parsed_chirho.module_chirho;
    if module_chirho.name_chirho.full_name_chirho() == "Main" {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "next needs the checked IO source declaration to validate Main.main; S0 accepts library modules only",
        ));
    }
    let hir_chirho = haskelujah_hir_chirho::lower_module_with_integer_literals_chirho(
        &module_chirho,
        UnitIdentityChirho::StandaloneHomeChirho,
        LimitsChirho::default(),
        &parsed_chirho.integer_literals_chirho,
    )
    .map_err(|error_chirho| {
        let kind_chirho = match error_chirho.kind_chirho {
            HirErrorKindChirho::InvalidChirho => NextErrorKindChirho::InvalidChirho,
            HirErrorKindChirho::UnsupportedChirho => NextErrorKindChirho::UnsupportedChirho,
            HirErrorKindChirho::ResourceChirho => NextErrorKindChirho::ResourceChirho,
        };
        NextErrorChirho::new_chirho(kind_chirho, error_chirho.message_chirho)
    })?;
    // Profile/import declines precede validation; no unsupported syntax gains an invalid verdict.
    if module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| extension_chirho == "RebindableSyntax")
    {
        for token_chirho in parsed_chirho.integer_literals_chirho.values() {
            haskelujah_runtime_chirho::integer_literal_chirho::IntegerValueChirho::validate_literal_chirho(token_chirho).map_err(|error_chirho| {
                let kind_chirho = match error_chirho {
                    haskelujah_runtime_chirho::integer_literal_chirho::IntegerLiteralErrorChirho::ResourceChirho => NextErrorKindChirho::ResourceChirho,
                    haskelujah_runtime_chirho::integer_literal_chirho::IntegerLiteralErrorChirho::MalformedChirho => NextErrorKindChirho::InvalidChirho,
                };
                NextErrorChirho::new_chirho(kind_chirho, error_chirho.to_string())
            })?;
        }
    }
    Ok(haskelujah_tc_chirho::check_module_chirho(
        hir_chirho,
        1 << 20,
    )?)
}

/// Elaborate checked HIR directly; the Core type compatibility adapter is data conversion only.
pub fn compile_source_chirho(source_chirho: &str) -> Result<CoreModuleChirho, NextErrorChirho> {
    let checked_chirho = check_source_chirho(source_chirho)?;
    core_chirho::elaborate_chirho(&checked_chirho)
}

/// Evaluate an explicitly selected library binding, like GHC's -e, not an IO-main claim.
pub fn eval_binding_chirho(
    source_chirho: &str,
    entry_chirho: &str,
    step_limit_chirho: u64,
) -> Result<
    (
        haskelujah_runtime_chirho::ValueChirho,
        haskelujah_runtime_chirho::MachineChirho,
    ),
    NextErrorChirho,
> {
    let core_chirho = compile_source_chirho(source_chirho)?;
    if !core_chirho
        .bindings_chirho
        .iter()
        .any(|binding_chirho| binding_chirho.binder_chirho.name_chirho == entry_chirho)
    {
        return Err(NextErrorChirho::new_chirho(
            NextErrorKindChirho::InvalidChirho,
            "the requested entry has no top-level binding",
        ));
    }
    crate::stg_lower_chirho::lower_and_run_with_step_limit_chirho(
        &core_chirho,
        Some(entry_chirho),
        Default::default(),
        step_limit_chirho,
    )
    .map_err(|error_chirho| {
        NextErrorChirho::new_chirho(NextErrorKindChirho::ExecutionChirho, error_chirho)
    })
}

#[cfg(test)]
#[path = "tests_chirho/default_methods_tests_chirho.rs"]
mod default_methods_tests_chirho;

#[cfg(test)]
#[path = "tests_chirho/integer_literals_tests_chirho.rs"]
mod integer_literals_tests_chirho;
