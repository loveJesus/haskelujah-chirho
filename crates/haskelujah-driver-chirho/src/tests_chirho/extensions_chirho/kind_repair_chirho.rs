// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact frozen product counterexamples. Fixture bytes match the independent
//! f6845284 GHC checkpoint; these tests exercise the actual frontend only.

use super::ordinary_elaboration_chirho::frontend_chirho;

#[test]
fn promoted_constructor_annotation_survives_open_equation_lowering_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/P01KProxyChirho.hs"
    ))
    .expect("frozen P01 GHC-positive source checks");
}

#[test]
fn promoted_constructor_synonym_keeps_its_kind_prefix_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/P04SynonymKindChirho.hs"
    ))
    .expect("frozen P04 GHC-positive source checks");
}

#[test]
fn written_kind_variables_keep_specified_visibility_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/P05ExplicitKindsChirho.hs"
    ))
    .expect("frozen P05 GHC-positive source checks");
}

#[test]
fn proxy_term_and_typeable_instance_use_the_complete_kind_prefix_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/TypeablePolykindPrefixChirho.hs"
    ))
    .expect("frozen polymorphic Typeable GHC-positive source checks");
}

#[test]
fn promoted_equation_wrong_result_is_a_real_type_error_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/N01WrongKProxyResultChirho.hs"
    ))
    .err()
    .expect("wrong result rejected");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn promoted_same_kind_overlap_is_a_real_coherence_error_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/N02SameKindOverlapChirho.hs"
    ))
    .err()
    .expect("overlap rejected");
    assert!(error_chirho.contains("overlap"), "{error_chirho}");
}

#[test]
fn qualified_proxy_type_application_skips_its_inferred_kind_chirho() {
    frontend_chirho("{-# LANGUAGE PolyKinds, TypeApplications #-}\nmodule Main where\nimport qualified Data.Proxy as PChirho\nvalueChirho :: PChirho.Proxy Maybe\nvalueChirho = PChirho.Proxy @Maybe\nmain = pure ()\n").expect("qualified Proxy applies its specified type parameter");
}

#[test]
fn inferred_constructor_kind_is_not_an_extra_visible_type_argument_chirho() {
    let error_chirho = frontend_chirho("{-# LANGUAGE PolyKinds, TypeApplications #-}\nmodule Main where\nimport Data.Proxy (Proxy(..))\nvalueChirho = Proxy @Maybe @Bool\nmain = pure ()\n").err().expect("inferred kind binder cannot be explicitly applied");
    assert!(
        error_chirho.contains("no specified constructor type binder"),
        "{error_chirho}"
    );
}

#[test]
fn source_constructor_inferred_kinds_stay_invisible_chirho() {
    frontend_chirho("{-# LANGUAGE PolyKinds, KindSignatures, TypeApplications #-}\nmodule Main where\ndata PhantomChirho (a :: k) = MkChirho\nvalueChirho :: PhantomChirho Maybe\nvalueChirho = MkChirho @Maybe\nmain = pure ()\n").expect("source constructor skips its inferred kind parameter");
}

#[test]
fn constructor_visible_type_application_checks_parameter_kinds_chirho() {
    let error_chirho = frontend_chirho("{-# LANGUAGE TypeApplications #-}\nmodule Main where\nvalueChirho = Nothing @Maybe\nmain = pure ()\n").err().expect("Maybe cannot fill a Type constructor parameter");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn deferred_source_class_kind_substitution_remains_linked_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC1DeferredKindSubstitutionChirho.hs"
    ))
    .expect("GHC-positive deferred predicate uses the substituted kind");
}

#[test]
fn source_class_instance_entailment_keeps_the_full_prefix_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC2InstanceEntailmentPrefixChirho.hs"
    ))
    .expect("GHC-positive source class givens remain in scope");
}

#[test]
fn source_class_method_uses_the_instance_kind_argument_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC3MethodKindOwnershipChirho.hs"
    ))
    .expect("GHC-positive method specializes the class kind");
}

#[test]
fn direct_source_class_twin_remains_accepted_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC1DirectPassingTwinChirho.hs"
    ))
    .expect("GHC-positive direct twin stays accepted");
}

#[test]
fn source_class_method_cannot_use_an_incompatible_kind_chirho() {
    let source_chirho = include_str!("kind_repair_fixtures_chirho/PC3MethodKindOwnershipChirho.hs")
        .replace(
            "instance NameChirho Maybe where",
            "instance NameChirho Bool where",
        )
        .replace(
            "TagChirho :: TagChirho Maybe",
            "TagChirho :: TagChirho Bool",
        );
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("unary method cannot serve a Type-kinded instance");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn source_class_deferred_predicate_requires_the_matching_instance_chirho() {
    let source_chirho =
        include_str!("kind_repair_fixtures_chirho/PC1DeferredKindSubstitutionChirho.hs")
            .replace("TagChirho :: TagChirho Maybe", "TagChirho :: TagChirho []");
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("same-kind list has no source class instance");
    assert!(error_chirho.contains("error[E0204]"), "{error_chirho}");
}

#[test]
fn associated_default_preserves_shared_hidden_kinds_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/HiddenDefaultChirho.hs"
    ))
    .expect("associated default uses the class's complete telescope");
}

#[test]
fn associated_default_renaming_retains_reordered_and_family_local_slots_chirho() {
    frontend_chirho(include_str!("kind_repair_fixtures_chirho/associated-chirho/MonokindedReorderedDefaultChirho.hs")).expect("producer correspondence retains subset/reordered class parameters and family-only variables");
}

#[test]
fn explicit_associated_equation_overrides_the_checked_default_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/MonokindedOverrideDefaultChirho.hs"
    ))
    .expect("explicit canonical family equation wins over its class default");
}

#[test]
fn associated_default_requires_the_checked_result_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/MonokindedWrongDefaultResultChirho.hs"
    ))
    .err()
    .expect("Bool cannot stand in for the default Maybe Int");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn an_associated_default_cannot_reduce_without_its_class_instance_chirho() {
    let source_chirho = include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/MonokindedWrongDefaultResultChirho.hs"
    )
    .replace("instance CChirho Int\n", "")
    .replace("valueChirho = True", "valueChirho = Just (1 :: Int)");
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("a default is not an abstract synonym equation");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn a_polykinded_default_cannot_specialize_the_class_header_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/PolyKindSpecializingDefaultChirho.hs"
    ))
    .err()
    .expect("a default cannot constrain an already generalized hidden kind");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}
