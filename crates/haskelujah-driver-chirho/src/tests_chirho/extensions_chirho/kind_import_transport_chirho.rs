// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import transport keeps checked owners and soft metadata refusals while hard
//! semantic transport limits stop the real frontend before error deferral.

use crate::ImportQualificationChirho;
use haskelujah_ast_chirho::decl_chirho::{
    DeclChirho, GadtProjectionBudgetChirho, GadtProjectionLimitsChirho,
};
use haskelujah_ast_chirho::name_chirho::{CompilerClassChirho, PredicateMetadataRefusalChirho};
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};
use haskelujah_typing_chirho::ty_chirho::SchemePredPartsChirho;
use haskelujah_typing_chirho::{SchemeChirho, SchemePredChirho, TyChirho};
use std::collections::{HashMap, HashSet};

fn list_chirho(depth_chirho: usize) -> TyChirho {
    (0..depth_chirho).fold(
        TyChirho::ConChirho("Int".into()),
        |inner_chirho, _index_chirho| TyChirho::ListChirho(Box::new(inner_chirho)),
    )
}

#[test]
fn soft_predicate_metadata_refusal_does_not_become_an_ordinary_import_error_chirho() {
    let deep_chirho = list_chirho(256);
    let mut predicate_chirho = SchemePredChirho::from_compiler_chirho(
        CompilerClassChirho::EqChirho,
        SchemePredPartsChirho {
            class_name_chirho: "Eq".into(),
            kind_tys_chirho: vec![deep_chirho.clone()],
            ty_chirho: deep_chirho.clone(),
            extra_tys_chirho: vec![TyChirho::ConChirho("Bool".into())],
        },
    );
    predicate_chirho
        .refuse_metadata_chirho(PredicateMetadataRefusalChirho::DepthChirho { limit_chirho: 128 });
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![].into(),
        preds_chirho: vec![predicate_chirho],
        ty_chirho: deep_chirho,
    };
    let names_chirho = HashSet::new();
    let preferred_chirho = HashMap::new();
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 1 << 25,
            output_chirho: 1 << 24,
            depth_chirho: 1024,
        });
    let copied_chirho = ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: &names_chirho,
        qualifier_chirho: "CallerChirho",
        safe_chirho: &names_chirho,
        preferred_chirho: &preferred_chirho,
        budget_chirho: &mut budget_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
    .scheme_chirho(&scheme_chirho, false)
    .expect("metadata exhaustion stays soft");
    assert_eq!(copied_chirho.ty_chirho, scheme_chirho.ty_chirho);
    assert_eq!(
        copied_chirho.preds_chirho[0].kind_tys_chirho,
        scheme_chirho.preds_chirho[0].kind_tys_chirho
    );
    assert_eq!(
        copied_chirho.preds_chirho[0].extra_tys_chirho,
        scheme_chirho.preds_chirho[0].extra_tys_chirho
    );
    assert_eq!(
        copied_chirho.preds_chirho[0].class_identity_chirho(),
        scheme_chirho.preds_chirho[0].class_identity_chirho()
    );
}

#[test]
fn hard_import_transport_failure_cannot_be_deferred_by_the_module_chirho() {
    let source_chirho = "module OriginChirho where\nfooChirho = undefined\n";
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let parser_chirho =
        crate::ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    let module_chirho = crate::lower_module_chirho(
        &parser_chirho.parse_chirho(),
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    ifaces_chirho.push(
        haskelujah_naming_chirho::build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho)
            .expect("actual source producer"),
    );
    let types_chirho = HashMap::from([(
        "OriginChirho.fooChirho".to_owned(),
        SchemeChirho {
            vars_chirho: vec![].into(),
            preds_chirho: vec![],
            ty_chirho: list_chirho(1030),
        },
    )]);
    let error_chirho = crate::run_frontend_chirho("{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule Main where\nimport qualified OriginChirho as OChirho\nmainChirho = OChirho.fooChirho\n", FileIdChirho::SYNTHETIC_CHIRHO, &ifaces_chirho, &types_chirho).err().expect("hard semantic transport limit is fatal");
    let rendered_chirho = error_chirho
        .diagnostics_chirho()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered_chirho.contains("error[E0300]"),
        "{rendered_chirho}"
    );
    assert!(
        rendered_chirho.contains("imported type transport exceeded"),
        "{rendered_chirho}"
    );
}

#[test]
fn checked_ast_qualification_keeps_constructor_and_lexical_variable_owners_chirho() {
    let result_chirho = super::ordinary_elaboration_chirho::frontend_chirho("{-# LANGUAGE ScopedTypeVariables, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ndata NominalChirho aChirho = MkNominalChirho\nfChirho :: forall (aChirho :: Type). Eq aChirho => NominalChirho aChirho -> NominalChirho aChirho\nfChirho xChirho = xChirho\nmain = pure ()\n").expect("checked producer AST");
    let original_chirho = result_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| {
            if let DeclChirho::TypeSigChirho { ty_chirho, .. } = declaration_chirho {
                Some(ty_chirho)
            } else {
                None
            }
        })
        .expect("actual checked signature");
    let names_chirho = HashSet::from(["NominalChirho".to_owned(), "Type".to_owned()]);
    let safe_chirho = HashSet::new();
    let preferred_chirho = HashMap::new();
    let mut budget_chirho = GadtProjectionBudgetChirho::new_chirho();
    let copied_chirho = ImportQualificationChirho {
        iface_chirho: None,
        type_names_chirho: &names_chirho,
        qualifier_chirho: "UnrelatedChirho",
        safe_chirho: &safe_chirho,
        preferred_chirho: &preferred_chirho,
        budget_chirho: &mut budget_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
    .ast_chirho(original_chirho)
    .expect("checked transport");
    // Debug includes owner metadata, unlike Name's legacy syntax equality.
    assert_eq!(format!("{copied_chirho:?}"), format!("{original_chirho:?}"));
}

fn id_supplier_chirho() -> (
    Vec<haskelujah_naming_chirho::ModuleIfaceChirho>,
    SchemeChirho,
) {
    let source_chirho =
        include_str!("../../source_chirho/value_telescopes_chirho/SourceIdSupplierChirho.hs");
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let checked_chirho = crate::run_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &HashMap::new(),
    )
    .expect("actual checked source supplier");
    let scheme_chirho = checked_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("id")
        .expect("source id scheme")
        .clone();
    let iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
        &checked_chirho.module_chirho,
        &ifaces_chirho,
    )
    .expect("actual source interface");
    ifaces_chirho.retain(|iface_chirho| iface_chirho.name_chirho != "Data.Function");
    ifaces_chirho.push(iface_chirho);
    (ifaces_chirho, scheme_chirho)
}

#[test]
fn selected_builtin_value_telescope_reaches_ordinary_type_application_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let good_chirho =
        include_str!("../../source_chirho/value_telescopes_chirho/QualifiedIdCorrectChirho.hs");
    let bad_chirho =
        include_str!("../../source_chirho/value_telescopes_chirho/QualifiedIdWrongChirho.hs");
    crate::run_frontend_chirho(
        good_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &HashMap::new(),
    )
    .expect("selected builtin id accepts Type argument");
    assert!(
        crate::run_frontend_chirho(
            bad_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
            &ifaces_chirho,
            &HashMap::new()
        )
        .is_err(),
        "selected builtin id must reject a Type -> Type argument"
    );
}

#[test]
fn selected_source_value_keeps_the_checked_cold_telescope_chirho() {
    let (ifaces_chirho, scheme_chirho) = id_supplier_chirho();
    let types_chirho = HashMap::from([("Data.Function.id".to_owned(), scheme_chirho)]);
    for (good_chirho, bad_chirho) in [
        (
            include_str!(
                "../../source_chirho/value_telescopes_chirho/QualifiedSourceIdCorrectChirho.hs"
            ),
            include_str!(
                "../../source_chirho/value_telescopes_chirho/QualifiedSourceIdWrongChirho.hs"
            ),
        ),
        (
            include_str!(
                "../../source_chirho/value_telescopes_chirho/BareSourceIdCorrectChirho.hs"
            ),
            include_str!("../../source_chirho/value_telescopes_chirho/BareSourceIdWrongChirho.hs"),
        ),
    ] {
        crate::run_frontend_chirho(
            good_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
            &ifaces_chirho,
            &types_chirho,
        )
        .expect("checked source higher-kind binder accepts Maybe");
        assert!(
            crate::run_frontend_chirho(
                bad_chirho,
                FileIdChirho::SYNTHETIC_CHIRHO,
                &ifaces_chirho,
                &types_chirho
            )
            .is_err(),
            "source higher-kind binder must reject Int"
        );
    }
}

#[test]
fn selected_missing_type_metadata_shadows_generic_seeds_without_new_kind_proof_chirho() {
    let (mut ifaces_chirho, _scheme_chirho) = id_supplier_chirho();
    for definer_known_chirho in [true, false] {
        if !definer_known_chirho {
            let iface_chirho = ifaces_chirho
                .iter_mut()
                .find(|iface_chirho| iface_chirho.name_chirho == "Data.Function")
                .unwrap();
            iface_chirho.exposure_chirho = None;
            iface_chirho
                .exports_chirho
                .values_chirho
                .get_mut("id")
                .unwrap()
                .defining_module_chirho = None;
        }
        for (source_chirho, key_chirho) in [
            (
                include_str!(
                    "../../source_chirho/value_telescopes_chirho/QualifiedSourceIdCorrectChirho.hs"
                ),
                "FunctionChirho.id",
            ),
            (
                include_str!(
                    "../../source_chirho/value_telescopes_chirho/BareSourceIdCorrectChirho.hs"
                ),
                "id",
            ),
        ] {
            let unrelated_bare_types_chirho = HashMap::from([(
                "id".to_owned(),
                haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho()["id"]
                    .clone(),
            )]);
            let result_chirho = crate::run_frontend_chirho(
                source_chirho,
                FileIdChirho::SYNTHETIC_CHIRHO,
                &ifaces_chirho,
                &unrelated_bare_types_chirho,
            )
            .expect("missing payload remains ordinary Unknown");
            let selected_chirho = result_chirho
                .infer_result_chirho
                .env_chirho
                .lookup_chirho(key_chirho)
                .expect("actual selected spelling");
            assert!(
                matches!(selected_chirho.ty_chirho, TyChirho::VarChirho(_)),
                "actual selected Unknown must not borrow the generic id function scheme: {selected_chirho:?}"
            );
            assert!(
                selected_chirho
                    .vars_chirho
                    .records_chirho()
                    .iter()
                    .all(|record_chirho| record_chirho.kind_chirho.is_none()
                        && record_chirho.source_chirho.is_none()
                        && record_chirho.specificity_chirho.is_none())
            );
        }
    }
}

#[test]
fn selected_source_reexport_uses_its_actual_definer_telescope_chirho() {
    let (mut ifaces_chirho, scheme_chirho) = id_supplier_chirho();
    let parser_chirho = crate::ParserChirho::new_chirho(
        "module ForwardChirho (id) where\nimport Data.Function (id)\n",
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let module_chirho = crate::lower_module_chirho(
        &parser_chirho.parse_chirho(),
        FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let iface_chirho =
        haskelujah_naming_chirho::build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho)
            .expect("actual reexport");
    assert_eq!(
        iface_chirho.exports_chirho.values_chirho["id"]
            .defining_module_chirho
            .as_deref(),
        Some("Data.Function")
    );
    ifaces_chirho.push(iface_chirho);
    let types_chirho = HashMap::from([("Data.Function.id".to_owned(), scheme_chirho)]);
    for (argument_chirho, accepted_chirho) in [("Maybe", true), ("Int", false)] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeApplications #-}}\nmodule CallerChirho where\nimport Prelude hiding (id)\nimport qualified ForwardChirho as FunctionChirho\nvalueChirho = FunctionChirho.id @{argument_chirho}\n"
        );
        assert_eq!(
            crate::run_frontend_chirho(
                &source_chirho,
                FileIdChirho::SYNTHETIC_CHIRHO,
                &ifaces_chirho,
                &types_chirho
            )
            .is_ok(),
            accepted_chirho
        );
    }
}

#[test]
fn local_higher_kind_id_keeps_its_own_lexical_telescope_chirho() {
    super::ordinary_elaboration_chirho::frontend_chirho(include_str!(
        "../../source_chirho/value_telescopes_chirho/LocalHigherKindIdChirho.hs"
    ))
    .expect("source local higher-kind id does not inherit builtin Type binder");
}

#[test]
fn cold_source_discovery_transports_the_actual_value_telescope_chirho() {
    let search_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/source_chirho/value_telescopes_chirho");
    for (name_chirho, source_chirho, accepted_chirho) in [
        (
            "ColdSourceIdCorrectChirho.hs",
            include_str!(
                "../../source_chirho/value_telescopes_chirho/ColdSourceIdCorrectChirho.hs"
            ),
            true,
        ),
        (
            "ColdSourceIdWrongChirho.hs",
            include_str!("../../source_chirho/value_telescopes_chirho/ColdSourceIdWrongChirho.hs"),
            false,
        ),
    ] {
        let result_chirho = crate::compile_source_with_search_path_chirho(
            source_chirho,
            &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
            name_chirho,
            &search_chirho,
        );
        assert_eq!(
            result_chirho.is_ok(),
            accepted_chirho,
            "actual cold supplier kind contract for {name_chirho}"
        );
    }
}
