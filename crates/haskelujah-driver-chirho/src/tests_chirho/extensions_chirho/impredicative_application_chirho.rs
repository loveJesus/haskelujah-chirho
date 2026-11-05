// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC-measured controls: explicit polytypes, Quick Look and ordinary metas.

fn check_chirho(group_chirho: &str, name_chirho: &str, accepts_chirho: bool) {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/tests_chirho/extensions_chirho/kind_repair_fixtures_chirho/impredicative-application-chirho")
        .join(group_chirho)
        .join(format!("{name_chirho}.hs"));
    let result_chirho =
        crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho);
    if accepts_chirho {
        result_chirho.unwrap_or_else(|diagnostics_chirho| panic!("{diagnostics_chirho:?}"));
    } else {
        let diagnostics_chirho = result_chirho
            .err()
            .expect("GHC rejects this implicit instantiation");
        assert!(
            diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diagnostic_chirho| diagnostic_chirho.to_string().contains("error[E")),
            "{diagnostics_chirho:?}"
        );
    }
}

#[test]
fn inferred_family_wrapper_rejects_chirho() {
    check_chirho("family-chirho", "InferredFamilyWrapperChirho", false);
}
#[test]
fn explicit_poly_family_argument_accepts_chirho() {
    check_chirho("family-chirho", "DirectPolyFamilyArgumentChirho", true);
}
#[test]
fn ordinary_impredicative_list_accepts_chirho() {
    check_chirho("family-chirho", "OrdinaryImpredicativeListChirho", true);
}
#[test]
fn implicit_constructor_poly_argument_rejects_chirho() {
    check_chirho(
        "family-chirho",
        "PolymorphicFamilyResultApplicationChirho",
        false,
    );
}
#[test]
fn nested_list_is_not_implicitly_polymorphic_chirho() {
    check_chirho("application-chirho", "OrdinaryLengthNestedChirho", false);
}
#[test]
fn explicit_family_length_accepts_chirho() {
    check_chirho("application-chirho", "ExplicitFamilyLengthChirho", true);
}
#[test]
fn opaque_family_blocks_quick_look_chirho() {
    check_chirho("application-chirho", "VariableFamilyLengthChirho", false);
}
#[test]
fn known_variable_drives_quick_look_chirho() {
    check_chirho("application-chirho", "OrdinaryLengthVariableChirho", true);
}

#[test]
fn nested_sigma_does_not_hide_guarded_information_chirho() {
    check_chirho("guarded-chirho", "GuardedTupleQuickLookChirho", true);
}
#[test]
fn later_argument_drives_quick_look_chirho() {
    check_chirho("guarded-chirho", "QuickLookLaterArgumentChirho", true);
}
#[test]
fn swapped_argument_drives_quick_look_chirho() {
    check_chirho("guarded-chirho", "QuickLookSwappedArgumentChirho", true);
}
#[test]
fn rank_n_does_not_enable_implicit_impredicativity_chirho() {
    check_chirho("guarded-chirho", "RankNBottomChirho", false);
}
