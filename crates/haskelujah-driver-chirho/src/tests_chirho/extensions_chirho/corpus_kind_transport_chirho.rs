// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact immutable corpus movers, measured independently by root with GHC9.14.1.

fn check_chirho(axis_chirho: &str, name_chirho: &str) -> Result<crate::CheckSummaryChirho, String> {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ghc-tests-chirho/typecheck-chirho")
        .join(axis_chirho)
        .join(name_chirho);
    check_path_chirho(path_chirho)
}

fn check_path_chirho(path_chirho: std::path::PathBuf) -> Result<crate::CheckSummaryChirho, String> {
    crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho).map_err(
        |diagnostics_chirho| {
            diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        },
    )
}

fn class_fixture_chirho(name_chirho: &str) -> Result<crate::CheckSummaryChirho, String> {
    check_path_chirho(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/tests_chirho/extensions_chirho/kind_repair_fixtures_chirho/class_contracts_chirho")
        .join(name_chirho).join(format!("{name_chirho}.hs")))
}

#[test]
fn class_contract_cold_polykind_keeps_its_full_supplier_telescope_chirho() {
    class_fixture_chirho("PolyClassColdChirho")
        .expect("fresh GHC-positive cold class signature checks");
}

#[test]
fn class_contract_cold_written_kind_cannot_be_generalized_away_chirho() {
    let error_chirho = class_fixture_chirho("FixedClassColdWrongChirho")
        .expect_err("fresh GHC-negative cold class kind rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn corpus_t10562_chirho() {
    check_chirho("should_compile", "T10562.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10635_chirho() {
    check_chirho("should_compile", "T10635.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10776_chirho() {
    check_chirho("should_compile", "T10776.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10009_chirho() {
    check_chirho("should_compile", "T10009.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10195_chirho() {
    check_chirho("should_compile", "T10195.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t11811_chirho() {
    check_chirho("should_compile", "T11811.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_polytypedecomp_chirho() {
    let error_chirho = check_chirho("should_compile", "PolytypeDecomp.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t14048a_chirho() {
    let error_chirho = check_chirho("should_fail", "T14048a.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t16059c_chirho() {
    let error_chirho = check_chirho("should_fail", "T16059c.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t22560_fail_b_chirho() {
    let error_chirho = check_chirho("should_fail", "T22560_fail_b.hs")
        .err()
        .expect("root's exact GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t4524_finishes_without_resource_exhaustion_chirho() {
    check_chirho("should_compile", "T4524.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t16512a_rejects_with_a_coded_diagnostic_chirho() {
    let error_chirho = check_chirho("should_fail", "T16512a.hs")
        .err()
        .expect("the retained GHC-rejected source must reject");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_t23543_keeps_universal_kind_annotations_chirho() {
    check_chirho("should_compile", "T23543.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_tc269_keeps_alpha_equivalent_kinds_and_universe_producers_chirho() {
    check_chirho("should_compile", "tc269.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_visflag2_keeps_required_quantifiers_distinct_chirho() {
    let error_chirho = check_chirho("should_fail", "VisFlag2.hs")
        .err()
        .expect("fresh GHC visibility negative rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn corpus_visflag3_keeps_associated_quantifier_visibility_chirho() {
    let error_chirho = check_chirho("should_fail", "VisFlag3.hs")
        .err()
        .expect("fresh GHC associated visibility negative rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn corpus_t8856_checks_annotation_with_its_owned_scope_chirho() {
    check_chirho("should_compile", "T8856.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t10031_checks_context_beneath_annotation_forall_chirho() {
    check_chirho("should_compile", "T10031.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t23501b_instantiates_variable_kind_per_application_chirho() {
    check_chirho("should_compile", "T23501b.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t18986a_instantiates_kind_through_family_application_chirho() {
    check_chirho("should_compile", "T18986a.hs")
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn corpus_t18640a_does_not_erase_family_visibility_chirho() {
    let error_chirho = check_chirho("should_fail", "T18640a.hs")
        .err()
        .expect("fresh GHC family visibility negative rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn corpus_specificity8_does_not_make_inferred_binders_required_chirho() {
    let error_chirho = check_chirho("should_fail", "ExplicitSpecificity8.hs")
        .err()
        .expect("fresh GHC specificity negative rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn checked_alias_result_prefix_positive_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/result-prefix-chirho/AliasResultPrefixPositiveChirho.hs"))
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn checked_alias_result_prefix_wrong_kind_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/result-prefix-chirho/AliasResultPrefixWrongChirho.hs"))
        .err().expect("actual GHC wrong-kind alias rejects");
    assert!(error_chirho.contains("error[E030"), "{error_chirho}");
}

#[test]
fn class_contract_liftedclasscontractchirho_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/LiftedClassContractChirho.hs"
    ))
    .expect("fresh GHC-positive class kind contract checks");
}

#[test]
fn class_contract_polyclasscontractchirho_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/PolyClassContractChirho.hs"
    ))
    .expect("fresh GHC-positive class kind contract checks");
}

#[test]
fn class_contract_repphantomclasscontractchirho_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/RepPhantomClassContractChirho.hs"
    ))
    .expect("fresh GHC-positive class kind contract checks");
}

#[test]
fn class_contract_wrongclasscontractresultchirho_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/WrongClassContractResultChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative class kind contract rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn class_contract_wrongclassbindercontractchirho_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/WrongClassBinderContractChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative class kind contract rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn class_contract_written_representation_cannot_be_specialized_by_a_method_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/RepPolyMethodChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative representation-polymorphic method rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn class_contract_lev_poly_bounded_keeps_its_standalone_representation_chirho() {
    let error_chirho = check_chirho("should_fail", "LevPolyBounded.hs")
        .expect_err("fresh GHC-negative standalone class kind stays universal");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn class_contract_classassoccontractchirho_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/ClassAssocContractChirho.hs"
    ))
    .expect("fresh GHC-positive associated class contract checks");
}

#[test]
fn class_contract_classassocdefaultcontractchirho_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/ClassAssocDefaultContractChirho.hs"
    ))
    .expect("fresh GHC-positive associated class contract checks");
}

#[test]
fn class_contract_classassocdefaultwrongchirho_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/ClassAssocDefaultWrongChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative associated default result rejects");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn class_contract_classkindwildcardchirho_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/ClassKindWildcardChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative standalone kind wildcard rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn class_contract_classkindpartialwildcardchirho_chirho() {
    let error_chirho = super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/class_contracts_chirho/ClassKindPartialWildcardChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative standalone kind wildcard rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

fn representation_field_fixture_chirho(
    name_chirho: &str,
) -> Result<crate::CheckSummaryChirho, String> {
    check_path_chirho(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/tests_chirho/extensions_chirho/kind_repair_fixtures_chirho/representation_chirho/fields_chirho")
        .join(format!("{name_chirho}.hs")))
}

#[test]
fn representation_open_arrow_keeps_a_class_parameter_universal_chirho() {
    check_chirho("should_compile", "T12987.hs")
        .expect("retained GHC-positive open class method arrow checks");
}

#[test]
fn representation_newtype_forall_uses_the_parent_result_kind_chirho() {
    check_chirho("should_compile", "UnliftedNewtypesForall.hs")
        .expect("retained GHC-positive forall field keeps its newtype result representation");
}

#[test]
fn representation_field_boxedlevityfield_chirho() {
    let error_chirho = representation_field_fixture_chirho("BoxedLevityFieldChirho")
        .expect_err("fresh GHC-negative nonconcrete field rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_boxedunliftedfield_chirho() {
    representation_field_fixture_chirho("BoxedUnliftedFieldChirho")
        .expect("fresh GHC-positive fixed field checks");
}

#[test]
fn representation_field_tuplerepresentationfield_chirho() {
    representation_field_fixture_chirho("TupleRepresentationFieldChirho")
        .expect("fresh GHC-positive fixed field checks");
}

#[test]
fn representation_field_tupletailfield_chirho() {
    let error_chirho = representation_field_fixture_chirho("TupleTailFieldChirho")
        .expect_err("fresh GHC-negative nonconcrete field rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_closedfamilyfield_chirho() {
    representation_field_fixture_chirho("ClosedFamilyFieldChirho")
        .expect("fresh GHC-positive fixed field checks");
}

#[test]
fn representation_field_openfamilyfield_chirho() {
    let error_chirho = representation_field_fixture_chirho("OpenFamilyFieldChirho")
        .expect_err("fresh GHC-negative nonconcrete field rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_refinedrepresentationfield_chirho() {
    let error_chirho = representation_field_fixture_chirho("RefinedRepresentationFieldChirho")
        .expect_err("fresh GHC-negative nonconcrete field rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_forgetfulrepresentationfield_chirho() {
    representation_field_fixture_chirho("ForgetfulRepresentationFieldChirho")
        .expect("fresh GHC-positive fixed field checks");
}

#[test]
fn representation_field_cold_private_family_uses_its_producer_chirho() {
    representation_field_fixture_chirho(
        "handoff_chirho/checked_chirho/ColdCheckedRepresentationFieldChirho",
    )
    .expect("fresh GHC-positive supplier keeps its private checked RuntimeRep family");
}

#[test]
fn representation_field_invalid_cold_supplier_is_rejected_chirho() {
    let error_chirho = representation_field_fixture_chirho(
        "handoff_chirho/wrong_chirho/ColdWrongRepresentationFieldChirho",
    )
    .expect_err(
        "fresh GHC-negative supplier field cannot be authorized by importer specialization",
    );
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_deferral_does_not_authorize_variable_levity_chirho() {
    let error_chirho =
        representation_field_fixture_chirho("handoff_chirho/DeferredBoxedLevityFieldChirho")
            .expect_err("fresh GHC structural field error remains fatal with defer-type-errors");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_deferral_does_not_authorize_an_open_family_chirho() {
    let error_chirho =
        representation_field_fixture_chirho("handoff_chirho/DeferredOpenFamilyFieldChirho")
            .expect_err(
                "fresh GHC stuck representation family remains fatal with defer-type-errors",
            );
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_field_unlifted_alias_carries_its_constructor_dependencies_chirho() {
    representation_field_fixture_chirho("aliases_chirho/UnliftedAliasFieldChirho")
        .expect("fresh GHC-positive UnliftedType alias field keeps its producer dependencies");
}

#[test]
fn representation_field_zero_bit_alias_carries_its_checked_list_prefix_chirho() {
    representation_field_fixture_chirho("aliases_chirho/ZeroBitAliasFieldTypedChirho").expect(
        "fresh GHC-positive DataKinds ZeroBitType field keeps its complete checked representation",
    );
}

#[test]
fn representation_forall_alias_may_infer_its_own_result_kind_chirho() {
    representation_field_fixture_chirho("forall_chirho/ProperForallAliasChirho")
        .expect("fresh GHC-positive forall alias checks");
}

#[test]
fn representation_forall_alias_may_have_a_higher_result_kind_chirho() {
    representation_field_fixture_chirho("forall_chirho/HigherForallAliasWrongChirho")
        .expect("fresh GHC ACCEPT refutes the original negative prediction; forall alias result need not be a value kind");
}
