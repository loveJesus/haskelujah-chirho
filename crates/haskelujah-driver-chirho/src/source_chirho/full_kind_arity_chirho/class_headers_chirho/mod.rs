// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual written invisible class binders need a complete licensed header.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::check_chirho;

#[test]
fn a_written_invisible_class_binder_needs_a_complete_kind_contract_chirho() {
    for source_chirho in [
        include_str!("ClassNoSaksChirho.hs"),
        include_str!("ClassInferredCuskChirho.hs"),
    ] {
        for options_chirho in [&[][..], &["-fdefer-type-errors"][..]] {
            let error_chirho = check_chirho(source_chirho, options_chirho)
                .err()
                .expect("fresh GHC92337: this written header has no complete kind contract");
            assert!(error_chirho.to_string().contains("error[E0300]"));
        }
    }
}

#[test]
fn actual_complete_class_headers_keep_their_written_invisible_binders_chirho() {
    for (source_chirho, options_chirho) in [
        (include_str!("ClassCuskChirho.hs"), &[][..]),
        (
            include_str!("ScopedClassVisibilityChirho.hs"),
            &["-XNoCUSKs"][..],
        ),
    ] {
        if let Err(error_chirho) = check_chirho(source_chirho, options_chirho) {
            panic!("GHC-positive complete class kind contract: {error_chirho}");
        }
    }
}

#[test]
fn a_complete_kind_contract_does_not_enable_type_abstractions_chirho() {
    for options_chirho in [&[][..], &["-fdefer-type-errors"][..]] {
        let error_chirho = check_chirho(
            include_str!("ClassNoTypeAbstractionsChirho.hs"),
            options_chirho,
        )
        .err()
        .expect("fresh GHC rejection: written invisible binders require TypeAbstractions");
        assert!(error_chirho.to_string().contains("error[E0300]"));
    }
}
