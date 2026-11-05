// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real qualified synonym contexts and nested universal ownership.

use super::ordinary_elaboration_chirho::frontend_chirho;

#[test]
fn checkedcontextalias_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/CheckedContextAliasChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn wrongcontextalias_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/WrongContextAliasChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("missing context or monomorphic body must reject");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn checkedrankcontextalias_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/CheckedRankContextAliasChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn wronguniversalcontextalias_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/WrongUniversalContextAliasChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("missing context or monomorphic body must reject");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn curriedcontextalias_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/CurriedContextAliasChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}
#[test]
fn record_context_alias_uses_fresh_universals_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/CheckedRecordContextAliasChirho.hs"
    ))
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn record_context_alias_rejects_specialized_body_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualified-context-chirho/WrongRecordContextAliasChirho.hs"
    ))
    .err()
    .expect("a universal record field cannot specialize its binder");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}
