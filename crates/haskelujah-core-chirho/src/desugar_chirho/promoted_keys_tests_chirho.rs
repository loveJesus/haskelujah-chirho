// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Core's one type-key helper on promoted constructors (row 686): a ticked or unquoted
//! promoted instance head keys as its constructor's identity under the module's scope,
//! as the checker's types are spelled, and an ordinary type keeps its key. Without a
//! scope the legacy keys remain, which are not corrected.

use super::*;
use haskelujah_ast_chirho::promotion_chirho::{PromotedKeyChirho, PromotionResolutionChirho};
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn scope_chirho() -> PromotionScopeChirho {
    PromotionScopeChirho::new_chirho(
        HashMap::from([(
            "True".to_string(),
            PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::builtin_bool_chirho(true)),
        )]),
        HashSet::from(["Maybe".to_string()]),
    )
}

fn quoted_true_chirho() -> TypeChirho {
    TypeChirho::PromotedConChirho {
        name_chirho: name_chirho("True"),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn a_promoted_instance_head_keys_as_its_constructor_chirho() {
    let key_chirho = PromotedKeyChirho::builtin_bool_chirho(true);
    let scope_chirho = scope_chirho();
    assert_eq!(
        DesugarCtxChirho::instance_type_key_chirho(&[quoted_true_chirho()], Some(&scope_chirho)),
        key_chirho.as_str_chirho(),
        "`instance C 'True`"
    );
    assert_eq!(
        DesugarCtxChirho::instance_type_key_chirho(
            &[TypeChirho::ConChirho(name_chirho("True"))],
            Some(&scope_chirho)
        ),
        key_chirho.as_str_chirho(),
        "`instance C True` names the same constructor"
    );
}

#[test]
fn an_ordinary_type_keeps_its_key_chirho() {
    assert_eq!(
        DesugarCtxChirho::type_key_from_ast_chirho(
            &TypeChirho::ConChirho(name_chirho("Maybe")),
            Some(&scope_chirho())
        ),
        "Maybe"
    );
}

#[test]
fn without_a_scope_a_ticked_head_keeps_the_legacy_key_chirho() {
    // Legacy, not corrected: every ticked head keyed `_` before row 686.
    assert_eq!(
        DesugarCtxChirho::instance_type_key_chirho(&[quoted_true_chirho()], None),
        "_"
    );
}
