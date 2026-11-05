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

#[test]
fn imported_nested_quantifier_is_consumed_before_implicit_instantiation_chirho() {
    check_chirho(
        "quantifiers-chirho/cold-chirho",
        "NestedImportedPositiveChirho",
        true,
    );
    check_chirho(
        "quantifiers-chirho/cold-chirho",
        "NestedImportedWrongChirho",
        false,
    );
}

#[test]
fn required_arguments_check_the_unused_binders_full_kind_chirho() {
    check_chirho("required-chirho", "RequiredHigherKindPositiveChirho", true);
    check_chirho("required-chirho", "RequiredHigherKindWrongChirho", false);
}

#[test]
fn required_dependent_prefix_checks_the_later_kind_chirho() {
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredDependentPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredDependentWrongChirho",
        false,
    );
}

#[test]
fn required_equation_patterns_keep_their_actual_type_scope_chirho() {
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredLexicalPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredLexicalWrongChirho",
        false,
    );
}

#[test]
fn required_names_keep_term_namespace_and_refuse_ordinary_values_chirho() {
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredHomonymPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredHomonymWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredTermVariableWrongChirho",
        false,
    );
}

#[test]
fn required_wildcards_lists_and_tuples_keep_their_meaning_chirho() {
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredWildcardPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredListPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredListWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/scope-chirho",
        "RequiredTuplePositiveChirho",
        true,
    );
}

#[test]
fn required_equation_scopes_restore_between_arms_and_declarations_chirho() {
    check_chirho(
        "required-chirho/restoration-chirho",
        "RequiredEquationsPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/restoration-chirho",
        "RequiredScopeLeakWrongChirho",
        false,
    );
}

#[test]
fn required_default_methods_use_the_checked_method_telescope_chirho() {
    check_chirho(
        "required-chirho/restoration-chirho",
        "RequiredMethodPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/restoration-chirho",
        "RequiredMethodWrongChirho",
        false,
    );
}

#[test]
fn required_kind_refinements_need_the_actual_pattern_evidence_chirho() {
    check_chirho(
        "required-chirho/restoration-chirho",
        "RequiredGivenKindPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/restoration-chirho",
        "RequiredUngivenKindWrongChirho",
        false,
    );
}

#[test]
fn required_kind_errors_stay_fatal_under_real_deferral_options_chirho() {
    check_chirho(
        "required-chirho/deferral-chirho",
        "RequiredDeferredPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/deferral-chirho",
        "RequiredDeferredWrongChirho",
        false,
    );
}

#[test]
fn required_contexts_and_invisible_prefixes_stay_signed_chirho() {
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredQualPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredQualWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredInvisiblePositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredInvisibleWrongChirho",
        false,
    );
}

#[test]
fn required_kind_variables_are_universal_in_equations_chirho() {
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredUniversalKindPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredUniversalKindWrongChirho",
        false,
    );
}

#[test]
fn required_universals_do_not_capture_outer_metas_chirho() {
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredCapturePositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/prefix-chirho",
        "RequiredCaptureWrongChirho",
        false,
    );
}
