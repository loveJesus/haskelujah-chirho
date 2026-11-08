// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The compiler's own promoted constructors and the readable form of keys in diagnostics.

use super::*;
use crate::name_chirho::{
    CompilerTypeConstructorChirho, RawNameChirho, TypeConstructorIdentityChirho,
};
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

#[test]
fn imported_promotion_stamps_survive_another_modules_scope_chirho() {
    for quoted_chirho in [false, true] {
        for original_chirho in [
            PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::source_chirho(
                "DefChirho",
                "TChirho",
            )),
            PromotionResolutionChirho::UnresolvedChirho(PromotedKeyChirho::unknown_chirho(
                "DefChirho",
                "TChirho",
            )),
        ] {
            let mut imported_name_chirho = name_chirho("TChirho");
            imported_name_chirho.set_promotion_chirho(original_chirho.clone());
            let mut ty_chirho = if quoted_chirho {
                TypeChirho::PromotedConChirho {
                    name_chirho: imported_name_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }
            } else {
                TypeChirho::ConChirho(imported_name_chirho)
            };
            let importing_scope_chirho = PromotionScopeChirho::new_chirho(
                HashMap::from([(
                    "TChirho".to_string(),
                    PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::source_chirho(
                        "ImporterChirho",
                        "TChirho",
                    )),
                )]),
                HashSet::new(),
            );
            stamp_promotions_chirho(&mut ty_chirho, &importing_scope_chirho);
            let stamped_name_chirho = match &ty_chirho {
                TypeChirho::PromotedConChirho { name_chirho, .. }
                | TypeChirho::ConChirho(name_chirho) => name_chirho,
                _ => unreachable!("the name shape is unchanged"),
            };
            assert_eq!(
                stamped_name_chirho.promotion_chirho(),
                Some(&original_chirho)
            );
        }
    }
}

/// A source type constructor's key, from its producer rather than written by hand.
fn source_constructor_key_chirho(module_chirho: &str, name_chirho: &str) -> String {
    TypeConstructorIdentityChirho::SourceChirho {
        defining_module_chirho: module_chirho.to_owned(),
        name_chirho: name_chirho.to_owned(),
    }
    .key_chirho()
}

#[test]
fn diagnostics_name_type_constructors_as_written_chirho() {
    let local_chirho = source_constructor_key_chirho("Test", "Maybe");
    let proxy_chirho =
        TypeConstructorIdentityChirho::CompilerChirho(CompilerTypeConstructorChirho::ProxyChirho)
            .key_chirho();
    let text_chirho =
        format!("expected `{local_chirho}`, found `Prelude.Maybe`; in `{proxy_chirho} Int`");
    assert_eq!(
        readable_constructor_keys_chirho(&text_chirho),
        "expected `Maybe`, found `Prelude.Maybe`; in `Proxy Int`"
    );
}

#[test]
fn two_type_constructors_sharing_a_spelling_are_qualified_chirho() {
    let def_chirho = source_constructor_key_chirho("DefChirho", "KChirho");
    let main_chirho = source_constructor_key_chirho("Main", "KChirho");
    assert_eq!(
        readable_constructor_keys_chirho(&format!(
            "expected `{def_chirho}`, found `{main_chirho}`"
        )),
        "expected `DefChirho.KChirho`, found `Main.KChirho`",
        "F02's hidden kinds must not read as KChirho against KChirho"
    );
    assert_eq!(
        readable_constructor_keys_chirho(&format!("{main_chirho} and {main_chirho}")),
        "KChirho and KChirho",
        "one constructor named twice needs no module"
    );
}

#[test]
fn text_without_a_whole_constructor_key_is_kept_chirho() {
    for text_chirho in [
        "expected `Maybe Int`",
        "%constructor:",
        "%constructor:source:99:Short",
        "%constructor:source:1:\u{e9}:1:T",
        "%constructor:source:99999999999999999999999999:M:1:T",
        "%constructor:source:+1:M:1:T",
        "%constructor:source:1:M1:T",
        "%constructor:source:1:M:0:",
        "%constructor:compiler:4:Bool",
        "%constructor:other:1:M:1:T",
    ] {
        assert_eq!(
            readable_constructor_keys_chirho(text_chirho),
            text_chirho,
            "a malformed, misaligned or overflowing key is kept verbatim: {text_chirho}"
        );
    }
}

#[test]
fn each_renderer_rewrites_only_its_own_keys_chirho() {
    let promoted_chirho = PromotedKeyChirho::source_chirho("Main", "TChirho");
    let constructor_chirho = source_constructor_key_chirho("Main", "LChirho");
    let text_chirho = format!(
        "`{}` has kind `{constructor_chirho}`",
        promoted_chirho.as_str_chirho()
    );
    let constructors_chirho = readable_constructor_keys_chirho(&text_chirho);
    assert_eq!(
        constructors_chirho,
        format!("`{}` has kind `LChirho`", promoted_chirho.as_str_chirho())
    );
    assert_eq!(
        readable_promoted_keys_chirho(&constructors_chirho),
        "`'TChirho` has kind `LChirho`"
    );
}
