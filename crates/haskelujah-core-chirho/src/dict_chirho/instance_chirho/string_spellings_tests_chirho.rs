// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The `String`-written row of a ground dictionary's slot: taken only when it
//! is the one row under another spelling of the typed key, after the exact
//! typed row (agy_chirho #27325, gpt_chirho #27399). And the engine spelling a
//! proven key and an argument-shape key are compared under.

use std::collections::HashMap;

use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::CoreIdChirho;

/// A pass whose source holds exactly these rows.
fn pass_with_rows_chirho(rows_chirho: &[(&str, u32)]) -> DictPassCtxChirho {
    let mut pass_chirho = DictPassCtxChirho::new_chirho(10, HashMap::new(), HashMap::new());
    for (row_name_chirho, id_chirho) in rows_chirho {
        pass_chirho
            .body_backed_names_chirho
            .insert(row_name_chirho.to_string(), CoreIdChirho(*id_chirho));
    }
    pass_chirho
}

fn written_row_chirho(rows_chirho: &[(&str, u32)], typed_key_chirho: &str) -> Option<CoreIdChirho> {
    pass_with_rows_chirho(rows_chirho).written_method_row_chirho(
        "TagChirho",
        "tagChirho",
        typed_key_chirho,
    )
}

#[test]
fn a_string_written_row_serves_only_when_it_is_the_one_chirho() {
    assert_eq!(
        written_row_chirho(&[("$prim_TagChirho_tagChirho_[String]", 1)], "[[Char]]"),
        Some(CoreIdChirho(1))
    );
    assert_eq!(
        written_row_chirho(&[("$prim_TagChirho_tagChirho_String", 1)], "[Char]"),
        Some(CoreIdChirho(1))
    );
    // Two spellings of one typed key are ambiguous, never a lowest-id pick.
    assert_eq!(
        written_row_chirho(
            &[
                ("$prim_TagChirho_tagChirho_(String, [Char])", 2),
                ("$prim_TagChirho_tagChirho_([Char], String)", 3),
            ],
            "([Char], [Char])",
        ),
        None
    );
    // The exact typed row is the resolver's earlier step, not this one's.
    assert_eq!(
        written_row_chirho(&[("$prim_TagChirho_tagChirho_[[Char]]", 4)], "[[Char]]"),
        None
    );
    // A type whose name merely contains `String`, and a key with no `[Char]`.
    assert_eq!(
        written_row_chirho(
            &[("$prim_TagChirho_tagChirho_[StringBoxChirho]", 5)],
            "[[Char]]"
        ),
        None
    );
    assert_eq!(
        written_row_chirho(
            &[("$prim_TagChirho_tagChirho_StringBoxChirho", 6)],
            "StringBoxChirho"
        ),
        None
    );
}

#[test]
fn the_exact_typed_row_wins_over_a_string_spelling_chirho() {
    let resolve_chirho = |rows_chirho: &[(&str, u32)]| {
        pass_with_rows_chirho(rows_chirho).resolve_or_missing_method_id_chirho(
            &ClassEnvChirho::new_chirho(),
            "TagChirho",
            "tagChirho",
            "[[Char]]",
            "$prim_TagChirho_tagChirho_[[Char]]",
            None,
        )
    };
    assert_eq!(
        resolve_chirho(&[
            ("$prim_TagChirho_tagChirho_[[Char]]", 4),
            ("$prim_TagChirho_tagChirho_[String]", 1),
        ]),
        CoreIdChirho(4)
    );
    assert_eq!(
        resolve_chirho(&[("$prim_TagChirho_tagChirho_[String]", 1)]),
        CoreIdChirho(1)
    );
}

#[test]
fn the_engine_spelling_respells_whole_words_only_chirho() {
    let spelled_chirho = DictPassCtxChirho::engine_spelled_key_chirho;
    // The defaulted `Integer` meets the rows' `Int`, at any depth.
    assert_eq!(spelled_chirho("[Integer]"), "[Int]");
    assert_eq!(spelled_chirho("[[Integer]]"), "[[Int]]");
    assert_eq!(spelled_chirho("Maybe Integer"), "Maybe Int");
    assert_eq!(spelled_chirho("[String]"), "[[Char]]");
    // A type whose name merely contains either word is another type.
    assert_eq!(spelled_chirho("[IntegerBoxChirho]"), "[IntegerBoxChirho]");
    assert_eq!(spelled_chirho("[StringBoxChirho]"), "[StringBoxChirho]");
    // The registry's `String` spelling leaves `Integer` alone.
    assert_eq!(
        DictPassCtxChirho::string_spelled_out_chirho("[Integer]"),
        "[Integer]"
    );
}
