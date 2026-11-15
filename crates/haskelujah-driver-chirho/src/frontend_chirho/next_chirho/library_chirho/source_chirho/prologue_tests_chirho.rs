// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;

#[test]
fn pinned_show_prologue_affects_neither_checked_header_nor_selected_bodies_chirho() {
    let source_chirho = validate_chirho(&SHOW_CHIRHO).unwrap();
    assert!(source_chirho.contains("module GHC.Internal.Show"));
    assert!(!source_chirho.starts_with("#if"));
    let rendered_chirho = SHOW_CHIRHO.render_chirho().unwrap();
    assert!(rendered_chirho.source_chirho.contains("class  Show a"));
    assert!(!rendered_chirho.source_chirho.contains("#define"));
}

#[test]
fn a_selected_macro_use_and_a_partial_directive_span_decline_chirho() {
    // The actual excluded decimal helper depends on BASE. Selecting it would
    // cross the layout/CPP boundary and must fail before semantic checking.
    let manifest_chirho = SourceManifestChirho {
        spans_chirho: &[(19050, 19407)],
        ..SHOW_CHIRHO
    };
    assert_eq!(
        validate_chirho(&manifest_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    for span_chirho in [(177, 574), (176, 573)] {
        let manifest_chirho = SourceManifestChirho {
            facade_prologue_chirho: Some(FacadePrologueChirho {
                span_chirho,
                header_end_chirho: 1855,
            }),
            ..SHOW_CHIRHO
        };
        assert_eq!(
            validate_chirho(&manifest_chirho).unwrap_err().kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}

#[test]
fn a_later_directive_cannot_enclose_a_selected_source_group_chirho() {
    let manifest_chirho = SourceManifestChirho {
        bytes_chirho: concat!(
            include_str!("../vendor-chirho/ghc-internal-chirho/Show.hs"),
            "\n#if 1\n#define LATER_CHIRHO 1\n#endif\n"
        ),
        ..SHOW_CHIRHO
    };
    assert_eq!(
        validate_chirho(&manifest_chirho).unwrap_err().kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}
