// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC9.14 alias RHS kind quantification, bounded to the outermost ascription.
//! Workflow: compiler-pipeline-chirho/type-scope-resolution-chirho.
use super::SourceMapChirho;
use haskelujah_driver::typecheck_source_chirho;

const PREFIX_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, TypeOperators, KindSignatures, RankNTypes #-}
module AliasChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
import Data.Type.Equality ((:~~:))
"#;

fn assert_alias_accepts_chirho(body_chirho: &str) {
    typecheck_source_chirho(
        &format!("{PREFIX_CHIRHO}{body_chirho}"),
        &mut SourceMapChirho::new_chirho(),
        "AliasChirho.hs",
    )
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

fn assert_alias_scope_error_chirho(body_chirho: &str, name_chirho: &str) {
    let error_chirho = typecheck_source_chirho(
        &format!("{PREFIX_CHIRHO}{body_chirho}"),
        &mut SourceMapChirho::new_chirho(),
        "AliasChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("the independently GHC-rejected name must remain out of scope");
    assert!(
        error_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diagnostic_chirho| {
                diagnostic_chirho.code_chirho
                    == Some(haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(101))
                    && diagnostic_chirho.message_chirho.contains(name_chirho)
            }),
        "{error_chirho}"
    );
}

// Exact sources and GHC9.14.1 outcomes: producers-chirho/alias-reference-chirho.jsonl.
// LegacyRhs and FirstRhs emit GHC-16382: accepted today, not a future-version promise.

#[test]
fn outer_alias_annotation_quantifies_its_free_kind_chirho() {
    assert_alias_accepts_chirho(
        r#"type HeteroEqPrefixChirho (aChirho :: k1Chirho) = ((:~~:) (aChirho :: k1Chirho) :: k2Chirho -> Type)
"#,
    );
}

#[test]
fn outer_alias_annotation_reuses_explicit_head_binders_chirho() {
    assert_alias_accepts_chirho(
        r#"type HeteroEqPrefixChirho k2Chirho (aChirho :: k1Chirho) = ((:~~:) (aChirho :: k1Chirho) :: k2Chirho -> Type)
"#,
    );
}

#[test]
fn standalone_alias_annotation_has_its_own_kind_scope_chirho() {
    assert_alias_accepts_chirho(
        r#"type FirstChirho = (Proxy :: kChirho -> Type)
"#,
    );
}

#[test]
fn nested_alias_annotation_cannot_introduce_a_kind_chirho() {
    assert_alias_scope_error_chirho(
        r#"type SharedChirho = (Proxy (Proxy :: kChirho -> Type), Proxy kChirho)
"#,
        "kChirho",
    );
}

#[test]
fn free_alias_rhs_type_variables_are_not_implicitly_bound_chirho() {
    assert_alias_scope_error_chirho(
        r#"type MissingChirho aChirho = Maybe bChirho
"#,
        "bChirho",
    );
}

#[test]
fn unknown_alias_classifier_is_not_a_kind_variable_chirho() {
    assert_alias_scope_error_chirho(
        r#"type MissingChirho = (Proxy :: NoSuchKindChirho -> Type)
"#,
        "NoSuchKindChirho",
    );
}

#[test]
fn nested_forall_kind_cannot_escape_into_alias_rhs_chirho() {
    assert_alias_scope_error_chirho(
        r#"type ScopedChirho = (forall kChirho (aChirho :: kChirho). Proxy aChirho -> Proxy aChirho) -> Proxy kChirho
"#,
        "kChirho",
    );
}

#[test]
fn implicit_alias_kind_cannot_escape_into_the_next_alias_chirho() {
    assert_alias_scope_error_chirho(
        r#"type FirstChirho = (Proxy :: kChirho -> Type)
type SecondChirho = Proxy kChirho
"#,
        "kChirho",
    );
}
