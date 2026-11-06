// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The standard Monad's own declared default for `>>`, as GHC's class declares it:
//! `m >> k = m >>= \_ -> k` (row 699, gpt_chirho #32286). The seeded Monad carries these arms as
//! its declared default. The dictionary pass implements exactly these arms over an instance's own
//! `>>=` row, and only where naming proves the bare spelling names the compiler's Monad (K01).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

/// An unqualified name the compiler writes, with no source span.
fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `m >> k = m >>= \_ -> k`: the standard Monad's declared default for `>>`, one clause.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub fn monad_then_default_arms_chirho() -> Vec<MatchArmChirho> {
    vec![MatchArmChirho {
        pats_chirho: vec![
            PatChirho::VarChirho(name_chirho("m")),
            PatChirho::VarChirho(name_chirho("k")),
        ],
        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::VarChirho(name_chirho("m"))),
            op_chirho: name_chirho(">>="),
            right_chirho: Box::new(ExprChirho::LamChirho {
                pats_chirho: vec![PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)],
                body_chirho: Box::new(ExprChirho::VarChirho(name_chirho("k"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        where_binds_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]
}
