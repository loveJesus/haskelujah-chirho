// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Whether a do bind's pattern can actually fail.
//!
//! `p <- m` selects `fail` only when `p` can fail to match. Getting this wrong
//! in either direction costs something real: call an irrefutable pattern
//! failable and every `(a, b) <- m` acquires a MonadFail obligation GHC does
//! not impose, so programs GHC accepts are rejected; call a failable pattern
//! irrefutable and a failing match has nowhere to go.
//!
//! The rule below was MEASURED against GHC 9.14.1 rather than recalled — the
//! seventeen cases in the tests are that measurement. Two of them are easy to
//! get wrong: a lazy pattern is irrefutable even when what it wraps is not
//! (`~(Just x)` selects no `fail`), and a view pattern is exactly as refutable
//! as the pattern on its right.
//!
//! Failability of a constructor pattern is not syntactic: it depends on how
//! many constructors the type has, which is why this lives beside the
//! constructor environment and runs after name resolution rather than in
//! lowering.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::pat_chirho::PatChirho;

use crate::exhaust_chirho::TypeConEnvChirho;

/// Whether matching `pat_chirho` can fail, and so whether its statement selects
/// `fail`. A constructor this environment does not know is treated as failable:
/// that is the safe direction, and it is what the desugarer already assumed for
/// every non-variable pattern.
pub fn pattern_can_fail_chirho(pat_chirho: &PatChirho, env_chirho: &TypeConEnvChirho) -> bool {
    match pat_chirho {
        // Nothing to match against.
        PatChirho::VarChirho(_) | PatChirho::WildcardChirho(_) => false,
        // A lazy pattern defers the match, so the bind itself never fails —
        // even when the pattern inside it is refutable.
        PatChirho::LazyChirho { .. } => false,
        // Transparent wrappers.
        PatChirho::ParenChirho { inner_chirho, .. }
        | PatChirho::BangChirho { inner_chirho, .. } => {
            pattern_can_fail_chirho(inner_chirho, env_chirho)
        }
        PatChirho::AsChirho { pattern_chirho, .. } => {
            pattern_can_fail_chirho(pattern_chirho, env_chirho)
        }
        PatChirho::TypeAnnotChirho { pat_chirho, .. } => {
            pattern_can_fail_chirho(pat_chirho, env_chirho)
        }
        // A view pattern fails exactly when the pattern it feeds fails.
        PatChirho::ViewChirho { pat_chirho, .. } => {
            pattern_can_fail_chirho(pat_chirho, env_chirho)
        }
        // A tuple has one constructor, so it fails only through its parts.
        PatChirho::TupleChirho {
            elements_chirho, ..
        } => elements_chirho
            .iter()
            .any(|element_chirho| pattern_can_fail_chirho(element_chirho, env_chirho)),
        // A list pattern fixes the length, so even `[]` can fail.
        PatChirho::ListChirho { .. } => true,
        // A literal is a comparison.
        PatChirho::LitChirho(_) | PatChirho::NegChirho { .. } => true,
        PatChirho::ConChirho {
            con_chirho,
            args_chirho,
            ..
        } => {
            !is_sole_constructor_chirho(con_chirho.text_chirho(), env_chirho)
                || args_chirho
                    .iter()
                    .any(|arg_chirho| pattern_can_fail_chirho(arg_chirho, env_chirho))
        }
        PatChirho::InfixConChirho {
            left_chirho,
            op_chirho,
            right_chirho,
            ..
        } => {
            !is_sole_constructor_chirho(op_chirho.text_chirho(), env_chirho)
                || pattern_can_fail_chirho(left_chirho, env_chirho)
                || pattern_can_fail_chirho(right_chirho, env_chirho)
        }
        // A field left out by `..` or by omission binds a variable, so only the
        // fields actually written can make the match fail.
        PatChirho::RecordChirho {
            con_chirho,
            fields_chirho,
            ..
        } => {
            !is_sole_constructor_chirho(con_chirho.text_chirho(), env_chirho)
                || fields_chirho.iter().any(|field_chirho| {
                    pattern_can_fail_chirho(&field_chirho.pattern_chirho, env_chirho)
                })
        }
    }
}

/// Whether `con_chirho` is the only constructor of its type. An unknown
/// constructor is not treated as sole.
fn is_sole_constructor_chirho(con_chirho: &str, env_chirho: &TypeConEnvChirho) -> bool {
    env_chirho
        .siblings_of_chirho(con_chirho)
        .is_some_and(|siblings_chirho| siblings_chirho.len() == 1)
}
