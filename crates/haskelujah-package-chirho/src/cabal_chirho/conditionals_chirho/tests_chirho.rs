// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn an_unmatched_conditional_contributes_no_fields_chirho() {
    let source_chirho = "library\n  if false\n    cpp-options: -DWRONG_CHIRHO\n  elif false\n    cpp-options: -DOTHER_WRONG_CHIRHO\n  build-depends: base\n";
    let result_chirho = preprocess_cabal_conditionals_chirho(source_chirho);
    assert!(!result_chirho.contains("cpp-options"), "{result_chirho}");
    assert!(
        result_chirho.contains("build-depends: base"),
        "{result_chirho}"
    );
}

#[test]
fn nested_false_conditions_do_not_reappear_after_dedenting_chirho() {
    let source_chirho = "library\n  if true\n    if false\n      cpp-options: -DWRONG_CHIRHO\n    exposed-modules: RealChirho\n  build-depends: base\n";
    let result_chirho = preprocess_cabal_conditionals_chirho(source_chirho);
    assert!(!result_chirho.contains("cpp-options"), "{result_chirho}");
    assert!(
        result_chirho.contains("exposed-modules: RealChirho"),
        "{result_chirho}"
    );
    assert!(
        result_chirho.contains("build-depends: base"),
        "{result_chirho}"
    );
}

#[test]
fn elif_selects_one_target_branch_and_keeps_sibling_fields_chirho() {
    let source_chirho = "library\n  if os(windows)\n    c-sources: windows-chirho.c\n  elif os(osx) || os(ios)\n    c-sources: apple-chirho.c\n  else\n    c-sources: unix-chirho.c\n  build-depends: base\n";
    for (os_chirho, chosen_chirho) in [
        ("windows", "windows-chirho.c"),
        ("osx", "apple-chirho.c"),
        ("ios", "apple-chirho.c"),
        ("linux", "unix-chirho.c"),
    ] {
        let result_chirho = preprocess_cabal_conditionals_with_env_chirho(
            source_chirho,
            &HashMap::new(),
            os_chirho,
            "aarch64",
        );
        assert!(
            result_chirho.contains(chosen_chirho),
            "{os_chirho}: {result_chirho}"
        );
        assert_eq!(
            result_chirho.matches("c-sources:").count(),
            1,
            "{result_chirho}"
        );
        assert!(
            result_chirho.contains("build-depends: base"),
            "{result_chirho}"
        );
        assert!(!result_chirho.contains("elif"), "{result_chirho}");
    }
}

#[test]
fn declared_false_flag_cannot_select_a_body_without_else_chirho() {
    let source_chirho = "flag debug-chirho\n  default: false\nlibrary\n  if flag(debug-chirho)\n    cpp-options: -DDEBUG_CHIRHO\n  exposed-modules: PublicChirho\n";
    let result_chirho = preprocess_cabal_conditionals_chirho(source_chirho);
    assert!(!result_chirho.contains("cpp-options"), "{result_chirho}");
    assert!(
        result_chirho.contains("exposed-modules: PublicChirho"),
        "{result_chirho}"
    );
}
