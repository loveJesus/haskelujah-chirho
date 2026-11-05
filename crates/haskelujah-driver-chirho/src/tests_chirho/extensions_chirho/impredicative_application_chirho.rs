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
        result_chirho.unwrap_or_else(|diagnostics_chirho| {
            panic!("GHC accepts {name_chirho}: {diagnostics_chirho:?}")
        });
    } else {
        let diagnostics_chirho = result_chirho
            .err()
            .unwrap_or_else(|| panic!("GHC rejects {name_chirho}"));
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

#[test]
fn checked_synonyms_supply_the_actual_required_binder_scope_chirho() {
    check_chirho(
        "required-chirho/aliases-chirho",
        "RequiredAliasPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/aliases-chirho",
        "RequiredAliasWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/aliases-chirho",
        "RequiredDefaultAliasPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/aliases-chirho",
        "RequiredDefaultAliasWrongChirho",
        false,
    );
}

#[test]
fn repeated_synonym_origins_do_not_conflate_nested_required_binders_chirho() {
    check_chirho(
        "required-chirho/aliases-chirho/nested-chirho",
        "RequiredAliasNestedPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/aliases-chirho/nested-chirho",
        "RequiredAliasNestedWrongChirho",
        false,
    );
}

#[test]
fn cold_imported_synonym_retains_required_binder_correspondence_chirho() {
    check_chirho(
        "required-chirho/aliases-chirho/cold-chirho",
        "RequiredImportedAliasPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/aliases-chirho/cold-chirho",
        "RequiredImportedAliasWrongChirho",
        false,
    );
}

#[test]
fn mixed_value_and_required_slots_follow_the_checked_spine_chirho() {
    for name_chirho in [
        "MixedRequired",
        "MixedInvisible",
        "MixedContext",
        "MixedDependent",
    ] {
        check_chirho(
            "required-chirho/body-scopes-chirho/mixed-chirho",
            &format!("{name_chirho}PositiveChirho"),
            true,
        );
        check_chirho(
            "required-chirho/body-scopes-chirho/mixed-chirho",
            &format!("{name_chirho}WrongChirho"),
            false,
        );
    }
}

#[test]
fn equations_can_return_an_unconsumed_required_signature_chirho() {
    for name_chirho in ["LeadingRequiredEta", "MixedRequiredEta"] {
        check_chirho(
            "required-chirho/body-scopes-chirho/eta-chirho",
            &format!("{name_chirho}PositiveChirho"),
            true,
        );
        check_chirho(
            "required-chirho/body-scopes-chirho/eta-chirho",
            &format!("{name_chirho}WrongChirho"),
            false,
        );
    }
}

#[test]
fn body_prefix_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleDirectPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleDirectWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleDirectGivenPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleDirectUngivenWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleAliasPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleAliasWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleAliasGivenPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/prefix-chirho",
        "InvisibleAliasUngivenWrongChirho",
        false,
    );
}

#[test]
fn body_deferral_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/deferral-chirho",
        "InvisibleDeferredPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/deferral-chirho",
        "InvisibleDeferredWrongChirho",
        true,
    );
}

#[test]
fn body_evidence_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/evidence-chirho",
        "InvisibleBranchPositiveChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/evidence-chirho",
        "InvisibleBranchWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/evidence-chirho",
        "FamilyKindGivenPositiveChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/evidence-chirho",
        "FamilyKindGivenWrongChirho",
        false,
    );
}

#[test]
fn body_distinctions_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "InvisibleBranchStandalonePositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "FamilyKindAllowedGivenPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "InvisibleBranchStandaloneWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "FamilyKindAllowedGivenWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "StructuralBodyKindWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "StructuralBodyKindDeferredWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "ScopedRequiredKindWrongChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "ScopedRequiredKindDeferredChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "ImplicitKindRequiredPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/distinctions-chirho",
        "ImplicitKindRequiredWrongChirho",
        false,
    );
}

#[test]
fn source_equality_signatures_require_determination_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/ambiguity-heads-chirho",
        "DeterminedFamilyKindGivenChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/ambiguity-heads-chirho",
        "ConstantFamilyKindGivenChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/ambiguity-heads-chirho",
        "PlainEqualityGivenChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/ambiguity-heads-chirho",
        "NoninjectiveResultGivenChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/ambiguity-heads-chirho",
        "FundepDeterminedEqualityChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/ambiguity-heads-chirho",
        "EmptyImportRigidConstraintWrongChirho",
        false,
    );
}

#[test]
fn source_signature_probe_assigns_whole_family_terms_without_inversion_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/whole-meta-chirho",
        "WholeFamilyMetaPositiveChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/whole-meta-chirho",
        "WholeFamilyMetaAmbiguousChirho",
        false,
    );
}

#[test]
fn source_signature_probe_preserves_family_dependency_authority_chirho() {
    check_chirho(
        "required-chirho/body-scopes-chirho/dependencies-chirho",
        "InjectiveFamilyDeterminedChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/dependencies-chirho",
        "NoninjectiveFamilyAmbiguousChirho",
        false,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/dependencies-chirho",
        "CyclicFamilyGivenChirho",
        true,
    );
    check_chirho(
        "required-chirho/body-scopes-chirho/dependencies-chirho",
        "InconsistentFamilyGivenChirho",
        false,
    );
}
