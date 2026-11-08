// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit tests of the AST-to-Core desugarer, grouped by topic (row 691, D1): expressions,
//! sugar, case binders, and bindings with strictness. The shared helpers stay here, private,
//! and each topic module reaches them through `use super::*`.

use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

mod bindings_strictness_tests_chirho;
mod case_binders_tests_chirho;
mod expressions_tests_chirho;
mod input_tests_chirho;
mod sugar_tests_chirho;

/// The alternatives a case was written with: a trailing default that only
/// raises the pattern-match failure is the row compiler's "no row matched",
/// not a source alternative.
fn explicit_alts_chirho(alts_chirho: &[CoreAltChirho]) -> &[CoreAltChirho] {
    match alts_chirho.last() {
        Some(CoreAltChirho {
            con_chirho: AltConChirho::DefaultChirho,
            rhs_chirho: CoreExprChirho::AppChirho { arg_chirho, .. },
            ..
        }) if matches!(
            arg_chirho.as_ref(),
            CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(reason_chirho))
                if reason_chirho.starts_with("Non-exhaustive")
        ) =>
        {
            &alts_chirho[..alts_chirho.len() - 1]
        }
        _ => alts_chirho,
    }
}

fn dummy_name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}
