// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Alias admission is observable through values, nominal identity and failure classes.
use super::super::{check_source_chirho, eval_binding_chirho};
use super::*;

fn fixture_chirho(group_chirho: &str, name_chirho: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/frontend_chirho/next_chirho/fixtures_chirho/synonyms_chirho")
            .join(group_chirho)
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}

fn string_value_chirho(
    mut value_chirho: haskelujah_runtime_chirho::ValueChirho,
    machine_chirho: &mut haskelujah_runtime_chirho::MachineChirho,
) -> String {
    use haskelujah_runtime_chirho::ValueChirho;
    let mut text_chirho = String::new();
    for _step_chirho in 0..1024 {
        match machine_chirho.force_to_prim_chirho(value_chirho) {
            ValueChirho::StringChirho(tail_chirho) => {
                text_chirho.push_str(&tail_chirho);
                return text_chirho;
            }
            ValueChirho::HeapPtrChirho(address_chirho) => {
                let cell_chirho = machine_chirho
                    .heap_chirho
                    .read_chirho(machine_chirho.heap_chirho.follow_ind_chirho(address_chirho))
                    .clone();
                match cell_chirho.info_chirho.name_chirho.as_str() {
                    "[]" => return text_chirho,
                    ":" => {
                        assert_eq!(cell_chirho.payload_chirho.len(), 2);
                        let ValueChirho::CharChirho(character_chirho) = machine_chirho
                            .force_to_prim_chirho(cell_chirho.payload_chirho[0].clone())
                        else {
                            panic!("string has a non-character head")
                        };
                        text_chirho.push(character_chirho);
                        value_chirho = cell_chirho.payload_chirho[1].clone();
                    }
                    _ => panic!("expected an evaluated String, got {cell_chirho:?}"),
                }
            }
            other_chirho => panic!("expected String, got {other_chirho:?}"),
        }
    }
    panic!("string observation exceeded its test allowance")
}

#[test]
fn actual_source_aliases_preserve_string_values_chirho() {
    for (name_chirho, expected_chirho) in [
        ("SourceAliasesChirho", "source aliases"),
        ("PreludeStringChirho", "prelude string"),
        ("SelectedStringChirho", "selected string"),
    ] {
        let (value_chirho, mut machine_chirho) = eval_binding_chirho(
            &fixture_chirho("positive-chirho", name_chirho),
            "resultChirho",
            100_000,
        )
        .unwrap();
        assert_eq!(
            string_value_chirho(value_chirho, &mut machine_chirho),
            expected_chirho
        );
    }
}

#[test]
fn parameters_forward_dependencies_and_overapplication_chirho() {
    for (name_chirho, expected_chirho) in [
        ("ParametersChirho", 'p'),
        ("ExtraArgumentsChirho", 'e'),
        ("Instance2021Chirho", 'i'),
    ] {
        let (value_chirho, _) = eval_binding_chirho(
            &fixture_chirho("positive-chirho", name_chirho),
            "resultChirho",
            100_000,
        )
        .unwrap();
        assert_eq!(
            value_chirho,
            haskelujah_runtime_chirho::ValueChirho::CharChirho(expected_chirho),
            "{name_chirho}"
        );
    }
    check_source_chirho(&fixture_chirho("positive-chirho", "ErasedTypeChirho")).unwrap();
}

#[test]
fn discarded_arguments_and_expanded_ambiguity_are_checked_chirho() {
    for (name_chirho, reason_chirho) in [
        ("ErasedBadKindChirho", "kind"),
        ("AmbiguousErasedChirho", "ambiguous"),
        ("CycleChirho", "recursive type synonym"),
        ("UnsaturatedChirho", "saturated"),
        ("Phantom2010Chirho", "kind"),
        ("SourceStringShadowChirho", "type mismatch"),
    ] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("boundary-chirho", name_chirho)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{name_chirho}: {error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains(reason_chirho),
            "{name_chirho}: {error_chirho}"
        );
    }
}

#[test]
fn unsupported_kind_and_constraint_forms_never_become_verdicts_chirho() {
    for name_chirho in [
        "ConstraintAliasChirho",
        "Phantom2021Chirho",
        "HigherNominalAliasChirho",
        "HigherField2010Chirho",
        "PhantomResult2021Chirho",
    ] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("boundary-chirho", name_chirho)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{name_chirho}: {error_chirho}"
        );
    }
    let error_chirho =
        check_source_chirho(&fixture_chirho("boundary-chirho", "Instance2010Chirho")).unwrap_err();
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    assert!(error_chirho.message_chirho.contains("TypeSynonymInstances"));
}

#[test]
fn closed_alias_interfaces_keep_hidden_nominal_owners_chirho() {
    let library_chirho = fixture_chirho("interfaces-chirho", "ALibChirho");
    let consumer_chirho = fixture_chirho("interfaces-chirho", "BUseChirho");
    let (value_chirho, _) = eval_sources_binding_chirho(
        &[&library_chirho, &consumer_chirho],
        "BUseChirho",
        "resultChirho",
        100_000,
    )
    .unwrap();
    assert_eq!(
        value_chirho,
        haskelujah_runtime_chirho::ValueChirho::CharChirho('a')
    );
    let bad_chirho = fixture_chirho("interfaces-chirho", "BBadChirho");
    assert_eq!(
        check_sources_chirho(&[&library_chirho, &bad_chirho], "BBadChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::TypeChirho
    );
    let scope_chirho = fixture_chirho("interfaces-chirho", "BScopeChirho");
    assert_eq!(
        check_sources_chirho(&[&library_chirho, &scope_chirho], "BScopeChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn shared_expansion_has_bounded_interface_materialization_chirho() {
    let mut source_chirho = String::from(
        "{-# LANGUAGE Haskell2010, NoImplicitPrelude #-}\nmodule GrowthChirho where\ndata UnitChirho = UnitChirho\ntype A0Chirho = UnitChirho\n",
    );
    for index_chirho in 1..=16 {
        let previous_chirho = index_chirho - 1;
        source_chirho.push_str(&format!(
            "type A{index_chirho}Chirho = (A{previous_chirho}Chirho, A{previous_chirho}Chirho)\n"
        ));
    }
    let parsed_chirho = super::super::parse_chirho::parse_chirho(
        &source_chirho,
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    )
    .unwrap();
    let hir_chirho = haskelujah_hir_chirho::lower_module_chirho(
        &parsed_chirho.module_chirho,
        haskelujah_hir_chirho::UnitIdentityChirho::StandaloneHomeChirho,
        Default::default(),
    )
    .unwrap();
    let checked_chirho = haskelujah_tc_chirho::check_module_chirho(hir_chirho, 10_000).unwrap();
    let mut allowance_chirho = 500;
    let error_chirho =
        haskelujah_tc_chirho::project_interface_chirho(&checked_chirho, &mut allowance_chirho)
            .unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::ResourceChirho
    );
}
