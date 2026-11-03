// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compiler's own promoted constructors and the readable form of keys in diagnostics.

use super::*;
use crate::name_chirho::RawNameChirho;
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

#[test]
fn the_compilers_constructors_need_no_scope_chirho() {
    let empty_chirho = PromotionScopeChirho::default();
    for (spelling_chirho, key_chirho) in [
        (":", PromotedKeyChirho::builtin_cons_chirho()),
        ("[]", PromotedKeyChirho::builtin_nil_chirho()),
        ("()", PromotedKeyChirho::builtin_unit_chirho()),
        (
            "(,,)",
            PromotedKeyChirho::builtin_chirho(BUILTIN_TUPLE_MODULE_CHIRHO, "(,,)"),
        ),
    ] {
        assert_eq!(
            quoted_promoted_spelling_chirho(&name_chirho(spelling_chirho), Some(&empty_chirho)),
            Some(key_chirho.as_str_chirho().to_string()),
            "'{spelling_chirho}"
        );
    }
    assert!(
        is_unknown_promoted_key_chirho(
            &quoted_promoted_spelling_chirho(&name_chirho("(x)"), Some(&empty_chirho)).unwrap()
        ),
        "only commas make a tuple constructor"
    );
    assert_eq!(
        unquoted_promoted_spelling_chirho(&name_chirho("()"), Some(&empty_chirho)),
        None,
        "an unquoted `()` is the unit type"
    );
}

#[test]
fn diagnostics_name_constructors_as_written_chirho() {
    let source_chirho = PromotedKeyChirho::source_chirho("My.Mod", "Z");
    let builtin_chirho = PromotedKeyChirho::builtin_bool_chirho(true);
    let cons_chirho = PromotedKeyChirho::builtin_cons_chirho();
    let unit_chirho = PromotedKeyChirho::builtin_unit_chirho();
    let unknown_chirho = PromotedKeyChirho::unknown_chirho("Q", "Text");
    let text_chirho = format!(
        "expected `({} {})`, found `{}` and `({} x xs)`; also {}'",
        source_chirho.as_str_chirho(),
        unknown_chirho.as_str_chirho(),
        builtin_chirho.as_str_chirho(),
        cons_chirho.as_str_chirho(),
        unit_chirho.as_str_chirho(),
    );
    assert_eq!(
        readable_promoted_keys_chirho(&text_chirho),
        "expected `('Z 'Q.Text)`, found `'True` and `(': x xs)`; also '()'"
    );
}

#[test]
fn text_without_a_whole_key_is_kept_chirho() {
    assert_eq!(
        readable_promoted_keys_chirho("expected `Maybe Int`"),
        "expected `Maybe Int`"
    );
    assert_eq!(
        readable_promoted_keys_chirho("%promoted:source:99:Short"),
        "%promoted:source:99:Short",
        "a module length past the end is not a key"
    );
}

#[test]
fn two_constructors_sharing_a_spelling_are_qualified_chirho() {
    let main_chirho = PromotedKeyChirho::source_chirho("Main", "TChirho");
    let def_chirho = PromotedKeyChirho::source_chirho("DefChirho", "TChirho");
    let text_chirho = format!(
        "expected `{}`, found `{}`",
        main_chirho.as_str_chirho(),
        def_chirho.as_str_chirho()
    );
    assert_eq!(
        readable_promoted_keys_chirho(&text_chirho),
        "expected `'Main.TChirho`, found `'DefChirho.TChirho`",
        "F02's mismatch must not read as 'TChirho against 'TChirho"
    );
    let repeated_chirho = format!("{0} and {0}", main_chirho.as_str_chirho());
    assert_eq!(
        readable_promoted_keys_chirho(&repeated_chirho),
        "'TChirho and 'TChirho",
        "one constructor named twice needs no module"
    );
}
