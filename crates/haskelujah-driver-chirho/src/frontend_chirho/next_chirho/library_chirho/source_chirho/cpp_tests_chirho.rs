// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;

#[test]
fn selected_groups_and_header_must_be_unconditional_and_macro_free_chirho() {
    let source_chirho = "module MChirho (fChirho) where\n#if TARGET_CHIRHO\nfChirho = 'a'\n#endif\ngChirho = 'b'\n#define gChirho changedChirho\n";
    let analysis_chirho = AnalysisChirho::new_chirho(source_chirho, false).unwrap();
    assert!(
        analysis_chirho
            .region_chirho((0, source_chirho.find("#if").unwrap()))
            .is_ok()
    );
    let start_chirho = source_chirho.find("fChirho =").unwrap();
    assert!(
        analysis_chirho
            .region_chirho((start_chirho, start_chirho + 14))
            .is_err()
    );
    let start_chirho = source_chirho.find("gChirho =").unwrap();
    assert!(
        analysis_chirho
            .region_chirho((start_chirho, start_chirho + 14))
            .is_err()
    );
    assert!(AnalysisChirho::new_chirho("#if X\n", false).is_err());
    assert!(AnalysisChirho::new_chirho("#else\n", false).is_err());
}
#[test]
fn conditional_imports_are_withheld_without_choosing_a_branch_chirho() {
    let source_chirho = "module MChirho (xChirho) where\n#if TARGET_CHIRHO\nimport AChirho\n#else\nimport BChirho\n#endif\nimport CChirho\nxChirho = 'x'\n";
    let analysis_chirho = AnalysisChirho::new_chirho(source_chirho, false).unwrap();
    let facade_chirho = analysis_chirho.facade_chirho();
    assert!(!facade_chirho.contains("import AChirho"));
    assert!(!facade_chirho.contains("import BChirho"));
    assert!(facade_chirho.contains("import CChirho"));
    assert_eq!(facade_chirho.len(), source_chirho.len());
    assert!(super::super::facade_chirho::read_chirho(&facade_chirho, "MChirho").is_ok());
}
#[test]
fn show_remains_checked_but_its_macro_helper_cannot_be_selected_chirho() {
    validate_chirho(&SHOW_CHIRHO).unwrap();
    let manifest_chirho = SourceManifestChirho {
        spans_chirho: &[(19050, 19407)],
        ..SHOW_CHIRHO
    };
    assert!(validate_chirho(&manifest_chirho).is_err());
    let facade_chirho = facade_chirho(&SHOW_CHIRHO).unwrap();
    assert!(facade_chirho.contains("module GHC.Internal.Show"));
    assert!(!facade_chirho.contains("#define"));
    assert!(
        SHOW_CHIRHO
            .render_chirho()
            .unwrap()
            .source_chirho
            .contains("class  Show a")
    );
}
#[test]
fn a_macro_after_the_selection_and_a_macro_before_a_prime_are_not_missed_chirho() {
    let source_chirho = "fChirho' = 'a'\n#define fChirho changedChirho\n";
    assert!(
        AnalysisChirho::new_chirho(source_chirho, false)
            .unwrap()
            .region_chirho((0, 15))
            .is_err()
    );
    assert!(AnalysisChirho::new_chirho("#include \"unknown-chirho.h\"\n", false).is_err());
    let source_chirho = "{-# LANGUAGE CPP\n  #-}\nmodule MChirho where\n";
    assert!(
        AnalysisChirho::new_chirho(source_chirho, false)
            .unwrap()
            .facade_chirho()
            .contains("module MChirho")
    );
}

#[test]
fn a_macro_in_an_unconditional_import_cannot_enlarge_scope_chirho() {
    let source_chirho = "module MChirho (xChirho) where\nimport NChirho hiding (hiddenChirho)\nxChirho = 'a'\n#define hiddenChirho otherChirho\n";
    let analysis_chirho = AnalysisChirho::new_chirho(source_chirho, false).unwrap();
    assert!(
        super::super::facade_chirho::read_with_macros_chirho(
            &analysis_chirho.facade_chirho(),
            "MChirho",
            &analysis_chirho.macros_chirho
        )
        .is_err()
    );
    let source_chirho = source_chirho.replace("#define hiddenChirho", "#define xBodyChirho");
    let analysis_chirho = AnalysisChirho::new_chirho(&source_chirho, false).unwrap();
    assert!(
        super::super::facade_chirho::read_with_macros_chirho(
            &analysis_chirho.facade_chirho(),
            "MChirho",
            &analysis_chirho.macros_chirho
        )
        .is_ok()
    );
}
