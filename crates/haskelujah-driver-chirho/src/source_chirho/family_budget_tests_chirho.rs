// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Retained GHC-paired ambiguity controls through the actual source frontend.

use super::language_options_tests_chirho::check_chirho;

#[test]
fn anchored_stuck_families_finish_under_one_actual_work_allowance_chirho() {
    let source_chirho = include_str!(
        "../tests_chirho/extensions_chirho/kind_repair_fixtures_chirho/impredicative-application-chirho/improvement-chirho/ImprovementAnchoredChoiceChirho.hs"
    );
    for options_chirho in [&[][..], &["-XPolyKinds"][..], &["-XNoPolyKinds"][..]] {
        let result_chirho = check_chirho(source_chirho, options_chirho);
        if let Err(error_chirho) = result_chirho {
            panic!("GHC-positive anchored family signature {options_chirho:?}: {error_chirho}");
        }
    }
}

#[test]
fn unanchored_recursive_equality_remains_an_ordinary_ambiguity_error_chirho() {
    let source_chirho = include_str!(
        "../tests_chirho/extensions_chirho/kind_repair_fixtures_chirho/impredicative-application-chirho/improvement-chirho/ImprovementUnanchoredChirho.hs"
    );
    for options_chirho in [&[][..], &["-XPolyKinds"][..], &["-XNoPolyKinds"][..]] {
        let error_chirho = check_chirho(source_chirho, options_chirho)
            .err()
            .expect("GHC-negative unanchored signature cannot acquire an ambiguity proof");
        let rendered_chirho = error_chirho.to_string();
        assert!(rendered_chirho.contains("error[E"), "{rendered_chirho}");
        assert!(
            !rendered_chirho.contains("resource bound"),
            "{rendered_chirho}"
        );
    }
}
