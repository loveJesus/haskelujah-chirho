// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual constructor declarations retain their own fields through selected interfaces.
//! Workflow: compiler-pipeline-chirho/value-scope-resolution-chirho.

use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;
use haskelujah_driver::compile_modules_chirho;
use haskelujah_naming_chirho::iface_chirho::{
    ConstructorFieldsChirho, IfaceValueAvailabilityChirho, merge_module_ifaces_chirho,
    select_import_iface_chirho,
};
use haskelujah_naming_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
use haskelujah_parser_chirho::{
    cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
};
use haskelujah_span_chirho::{FileIdChirho, SourceMapChirho};

const RECORD_SOURCE_CHIRHO: &str = "module FieldsChirho where\ndata ChoiceChirho = FirstChirho { firstFieldChirho :: Bool, sharedFieldChirho :: Int } | SecondChirho { secondFieldChirho :: Int } | EmptyChirho\nnewtype WrapperChirho = WrapperChirho { wrappedFieldChirho :: Bool }\nordinaryChirho :: Bool\nordinaryChirho = True\n";

fn source_iface_chirho(
    source_chirho: &str,
    imports_chirho: &[ModuleIfaceChirho],
) -> ModuleIfaceChirho {
    let green_chirho = ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
        .parse_checked_chirho()
        .expect("valid source interface fixture Chirho");
    let module_chirho = lower_module_chirho(&green_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    build_iface_with_imports_chirho(&module_chirho, imports_chirho)
        .expect("actual source constructor interface Chirho")
}

fn known_fields_chirho<'iface_chirho>(
    iface_chirho: &'iface_chirho ModuleIfaceChirho,
    name_chirho: &str,
) -> (
    &'iface_chirho TypeConstructorDeclarationChirho,
    &'iface_chirho [String],
) {
    iface_chirho.exports_chirho.values_chirho[name_chirho]
        .constructor_fields_chirho
        .as_ref()
        .and_then(ConstructorFieldsChirho::known_chirho)
        .expect("the actual selected declaration publishes known fields Chirho")
}

#[test]
fn source_constructor_fields_keep_order_owner_and_known_empty_chirho() {
    let iface_chirho = source_iface_chirho(RECORD_SOURCE_CHIRHO, &[]);
    let owner_chirho = iface_chirho.exports_chirho.types_chirho["ChoiceChirho"]
        .declaration_chirho
        .as_ref()
        .unwrap();
    for (constructor_chirho, expected_chirho) in [
        ("FirstChirho", vec!["firstFieldChirho", "sharedFieldChirho"]),
        ("SecondChirho", vec!["secondFieldChirho"]),
        ("EmptyChirho", vec![]),
    ] {
        let (actual_owner_chirho, labels_chirho) =
            known_fields_chirho(&iface_chirho, constructor_chirho);
        assert_eq!(actual_owner_chirho, owner_chirho);
        assert_eq!(
            labels_chirho.iter().map(String::as_str).collect::<Vec<_>>(),
            expected_chirho
        );
    }
    let (wrapper_owner_chirho, wrapper_labels_chirho) =
        known_fields_chirho(&iface_chirho, "WrapperChirho");
    assert_eq!(wrapper_labels_chirho, &["wrappedFieldChirho".to_owned()]);
    assert_eq!(
        Some(wrapper_owner_chirho),
        iface_chirho.exports_chirho.types_chirho["WrapperChirho"]
            .declaration_chirho
            .as_ref(),
    );
    assert!(
        iface_chirho.exports_chirho.values_chirho["ordinaryChirho"]
            .constructor_fields_chirho
            .is_none()
    );
    let mut unknown_chirho = iface_chirho.exports_chirho.values_chirho["EmptyChirho"].clone();
    unknown_chirho.constructor_fields_chirho = None;
    assert_ne!(
        unknown_chirho,
        iface_chirho.exports_chirho.values_chirho["EmptyChirho"]
    );
}

#[test]
fn existential_label_availability_does_not_erase_constructor_fields_chirho() {
    let iface_chirho = source_iface_chirho(
        "{-# LANGUAGE GADTs, ExplicitForAll #-}\nmodule HiddenFieldsChirho where\ndata HiddenChirho where\n  MkHiddenChirho :: forall valueChirho. { hiddenFieldChirho :: valueChirho } -> HiddenChirho\n",
        &[],
    );
    assert_eq!(
        iface_chirho.exports_chirho.values_chirho["hiddenFieldChirho"].availability_chirho,
        IfaceValueAvailabilityChirho::FieldLabelOnlyChirho,
    );
    assert_eq!(
        known_fields_chirho(&iface_chirho, "MkHiddenChirho").1,
        &["hiddenFieldChirho".to_owned()],
    );
}

#[test]
fn selected_and_forwarded_constructors_keep_physical_fields_without_exposing_labels_chirho() {
    let provider_chirho = source_iface_chirho(RECORD_SOURCE_CHIRHO, &[]);
    let forwarded_chirho = source_iface_chirho(
        "module ForwardedFieldsChirho (module FieldsChirho) where\nimport FieldsChirho (ChoiceChirho(FirstChirho, firstFieldChirho))\n",
        std::slice::from_ref(&provider_chirho),
    );
    assert_eq!(
        forwarded_chirho.exports_chirho.values_chirho["FirstChirho"].constructor_fields_chirho,
        provider_chirho.exports_chirho.values_chirho["FirstChirho"].constructor_fields_chirho,
    );
    assert!(
        !forwarded_chirho
            .exports_chirho
            .values_chirho
            .contains_key("SecondChirho")
    );
    assert!(
        forwarded_chirho
            .exports_chirho
            .values_chirho
            .contains_key("firstFieldChirho")
    );
    assert!(
        !forwarded_chirho
            .exports_chirho
            .values_chirho
            .contains_key("sharedFieldChirho")
    );
    let only_chirho = source_iface_chirho(
        "module ConstructorOnlyChirho (ChoiceChirho(FirstChirho)) where\nimport FieldsChirho\n",
        std::slice::from_ref(&provider_chirho),
    );
    assert_eq!(
        only_chirho.exports_chirho.values_chirho["FirstChirho"].constructor_fields_chirho,
        provider_chirho.exports_chirho.values_chirho["FirstChirho"].constructor_fields_chirho,
    );
    assert!(
        !only_chirho
            .exports_chirho
            .values_chirho
            .contains_key("firstFieldChirho")
    );
    let green_chirho = ParserChirho::new_chirho(
        "module ImporterChirho where\nimport qualified ForwardedFieldsChirho as SelectedChirho (ChoiceChirho(FirstChirho))\n",
        FileIdChirho::SYNTHETIC_CHIRHO,
    ).parse_checked_chirho().unwrap();
    let importer_chirho = lower_module_chirho(&green_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    let selected_chirho = select_import_iface_chirho(
        std::slice::from_ref(&forwarded_chirho),
        &importer_chirho.imports_chirho[0],
    )
    .unwrap();
    let names_chirho = haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
        &selected_chirho.exports_chirho,
        &importer_chirho.imports_chirho[0].spec_chirho,
    );
    assert!(
        names_chirho
            .iter()
            .any(|(name_chirho, _, _)| name_chirho == "FirstChirho")
    );
    assert!(
        !names_chirho
            .iter()
            .any(|(name_chirho, _, _)| name_chirho == "firstFieldChirho")
    );
}

#[test]
fn fragment_conflicts_drop_field_authority_instead_of_unioning_labels_chirho() {
    let provider_chirho = source_iface_chirho(RECORD_SOURCE_CHIRHO, &[]);
    let same_chirho =
        merge_module_ifaces_chirho(vec![provider_chirho.clone(), provider_chirho.clone()]);
    assert_eq!(
        same_chirho[0].exports_chirho.values_chirho["FirstChirho"].constructor_fields_chirho,
        provider_chirho.exports_chirho.values_chirho["FirstChirho"].constructor_fields_chirho,
    );
    let mut unknown_chirho = provider_chirho.clone();
    unknown_chirho
        .exports_chirho
        .values_chirho
        .get_mut("FirstChirho")
        .unwrap()
        .constructor_fields_chirho = None;
    let mut reordered_chirho = provider_chirho.clone();
    if let Some(ConstructorFieldsChirho::KnownChirho { labels_chirho, .. }) = &mut reordered_chirho
        .exports_chirho
        .values_chirho
        .get_mut("FirstChirho")
        .unwrap()
        .constructor_fields_chirho
    {
        labels_chirho.reverse();
    } else {
        panic!("actual source record has known fields Chirho");
    }
    let other_chirho = source_iface_chirho(
        RECORD_SOURCE_CHIRHO
            .replace("FieldsChirho where", "OtherFieldsChirho where")
            .as_str(),
        &[],
    );
    let mut foreign_owner_chirho = provider_chirho.clone();
    foreign_owner_chirho
        .exports_chirho
        .values_chirho
        .get_mut("FirstChirho")
        .unwrap()
        .constructor_fields_chirho = other_chirho.exports_chirho.values_chirho["FirstChirho"]
        .constructor_fields_chirho
        .clone();
    for conflicting_chirho in [unknown_chirho, reordered_chirho, foreign_owner_chirho] {
        for fragments_chirho in [
            vec![provider_chirho.clone(), conflicting_chirho.clone()],
            vec![conflicting_chirho.clone(), provider_chirho.clone()],
            vec![
                provider_chirho.clone(),
                conflicting_chirho.clone(),
                provider_chirho.clone(),
            ],
        ] {
            let merged_chirho = merge_module_ifaces_chirho(fragments_chirho);
            assert!(
                merged_chirho[0].exports_chirho.values_chirho["FirstChirho"]
                    .constructor_fields_chirho
                    .as_ref()
                    .and_then(ConstructorFieldsChirho::known_chirho)
                    .is_none()
            );
        }
    }
}

#[test]
fn imported_wildcards_keep_valid_fields_and_do_not_invent_sibling_binders_chirho() {
    let accepted_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule AcceptedFieldsChirho where\nimport FieldsChirho\nfirstChirho :: ChoiceChirho -> Bool\nfirstChirho FirstChirho{..} = firstFieldChirho\nsecondChirho :: ChoiceChirho -> Int\nsecondChirho SecondChirho{..} = secondFieldChirho\n";
    compile_modules_chirho(
        &[
            ("FieldsChirho.hs", RECORD_SOURCE_CHIRHO),
            ("AcceptedFieldsChirho.hs", accepted_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .expect("each real constructor retains its own valid wildcard fields Chirho");
    for rejected_chirho in [
        "{-# LANGUAGE RecordWildCards #-}\nmodule RejectedFieldsChirho where\nimport FieldsChirho\nbadChirho :: ChoiceChirho -> Int\nbadChirho FirstChirho{..} = secondFieldChirho\n",
        "{-# LANGUAGE RecordWildCards #-}\nmodule RejectedFieldsChirho where\nimport FieldsChirho\nbadChirho :: ChoiceChirho -> Bool\nbadChirho SecondChirho{..} = secondFieldChirho\n",
    ] {
        assert!(
            compile_modules_chirho(
                &[
                    ("FieldsChirho.hs", RECORD_SOURCE_CHIRHO),
                    ("RejectedFieldsChirho.hs", rejected_chirho)
                ],
                &mut SourceMapChirho::new_chirho()
            )
            .is_err(),
            "a sibling or displaced physical field must not acquire an unconstrained wildcard binder Chirho"
        );
    }
}

#[test]
fn qualified_constructor_fields_require_the_exact_selected_payload_chirho() {
    let local_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule LocalFieldsChirho where\ndata LocalChirho = MkLocalChirho { localFieldChirho :: Bool }\nreadChirho :: LocalChirho -> Bool\nreadChirho LocalFieldsChirho.MkLocalChirho{..} = localFieldChirho\n";
    compile_modules_chirho(
        &[("LocalFieldsChirho.hs", local_chirho)],
        &mut SourceMapChirho::new_chirho(),
    )
    .expect("self-qualified local constructors retain their actual field payload Chirho");
    let accepted_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule QualifiedFieldsChirho where\nimport qualified FieldsChirho as SelectedChirho\nreadChirho :: SelectedChirho.ChoiceChirho -> Bool\nreadChirho SelectedChirho.FirstChirho{..} = firstFieldChirho\n";
    compile_modules_chirho(
        &[
            ("FieldsChirho.hs", RECORD_SOURCE_CHIRHO),
            ("QualifiedFieldsChirho.hs", accepted_chirho),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .expect("a qualified-only constructor reads its actual selected field payload Chirho");
    let absent_chirho = "module AbsentFieldsChirho where\ndata AbsentChirho = AbsentChirho\n";
    let rejected_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule MissingQualifiedFieldsChirho where\nimport FieldsChirho\nimport qualified AbsentFieldsChirho as MissingChirho\nbadChirho :: ChoiceChirho -> Bool\nbadChirho MissingChirho.FirstChirho{..} = firstFieldChirho\n";
    assert!(
        compile_modules_chirho(
            &[
                ("FieldsChirho.hs", RECORD_SOURCE_CHIRHO),
                ("AbsentFieldsChirho.hs", absent_chirho),
                ("MissingQualifiedFieldsChirho.hs", rejected_chirho)
            ],
            &mut SourceMapChirho::new_chirho()
        )
        .is_err(),
        "a missing qualified constructor cannot borrow a bare namesake's fields Chirho"
    );
    let fieldless_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule FieldlessShadowChirho where\nimport FieldsChirho\ndata LocalChirho = FirstChirho\nbadChirho :: LocalChirho -> Bool\nbadChirho FirstChirho{..} = firstFieldChirho\n";
    assert!(
        compile_modules_chirho(
            &[
                ("FieldsChirho.hs", RECORD_SOURCE_CHIRHO),
                ("FieldlessShadowChirho.hs", fieldless_chirho)
            ],
            &mut SourceMapChirho::new_chirho()
        )
        .is_err(),
        "a real local fieldless constructor clears an imported record namesake's wildcard fields Chirho"
    );
}

#[test]
fn legacy_builtin_record_typing_does_not_publish_field_authority_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let identity_chirho = ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.Functor.Identity")
        .unwrap();
    assert!(
        identity_chirho.exports_chirho.values_chirho["Identity"]
            .constructor_fields_chirho
            .is_none()
    );
    let source_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule LegacyBuiltinFieldsChirho where\nimport Data.Functor.Identity\nunwrapChirho :: Identity Bool -> Bool\nunwrapChirho Identity{..} = runIdentity\n";
    compile_modules_chirho(
        &[("LegacyBuiltinFieldsChirho.hs", source_chirho)],
        &mut SourceMapChirho::new_chirho(),
    )
    .expect("the explicit unproven builtin compatibility path retains valid record typing Chirho");
}

#[test]
fn blocked_constructor_account_cannot_reenter_legacy_typing_chirho() {
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let provider_chirho = haskelujah_driver::run_frontend_chirho(
        RECORD_SOURCE_CHIRHO,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &std::collections::HashMap::new(),
    )
    .expect("actual record source produces its checked constructor schemes Chirho");
    let iface_chirho =
        build_iface_with_imports_chirho(&provider_chirho.module_chirho, &ifaces_chirho).unwrap();
    let mut schemes_chirho = std::collections::HashMap::new();
    for name_chirho in iface_chirho.exports_chirho.values_chirho.keys() {
        if let Some(scheme_chirho) = provider_chirho
            .infer_result_chirho
            .env_chirho
            .lookup_chirho(name_chirho)
        {
            schemes_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
            schemes_chirho.insert(format!("FieldsChirho.{name_chirho}"), scheme_chirho.clone());
        }
    }
    let mut raw_chirho = iface_chirho.clone();
    for value_chirho in raw_chirho.exports_chirho.values_chirho.values_mut() {
        value_chirho.constructor_fields_chirho = None;
    }
    for type_chirho in raw_chirho.exports_chirho.types_chirho.values_mut() {
        type_chirho.declaration_chirho = None;
    }
    ifaces_chirho.push(raw_chirho.clone());
    let positive_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule RawFieldsChirho where\nimport FieldsChirho\nreadChirho valueChirho = case valueChirho of\n  FirstChirho{..} -> if firstFieldChirho then True else False\n";
    haskelujah_driver::run_frontend_chirho(
        positive_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &schemes_chirho,
    )
    .expect("a valid wildcard keeps the explicitly unproven raw compatibility path Chirho");
    ifaces_chirho
        .pop()
        .expect("remove the prior raw fixture Chirho");
    let merged_chirho =
        merge_module_ifaces_chirho(vec![iface_chirho.clone(), raw_chirho, iface_chirho]);
    ifaces_chirho.extend(merged_chirho);
    let negative_chirho = "{-# LANGUAGE RecordWildCards #-}\nmodule BlockedFieldsChirho where\nimport FieldsChirho\nreadChirho valueChirho = case valueChirho of\n  SecondChirho{..} -> if secondFieldChirho then True else False\n";
    assert!(
        haskelujah_driver::run_frontend_chirho(
            negative_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
            &ifaces_chirho,
            &schemes_chirho
        )
        .is_err(),
        "a lost account must not reopen type-wide legacy fields and invent a Bool binder for an Int field Chirho"
    );
}
