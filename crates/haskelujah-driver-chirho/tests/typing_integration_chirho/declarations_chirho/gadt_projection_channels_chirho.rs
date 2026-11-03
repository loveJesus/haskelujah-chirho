// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! These are resource/error-channel controls, not GHC output parity cells.
//! A small source representation can imply a deep amplified type; every
//! checked public consumer must refuse it before producing a partial value.

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, ConstructorSignatureErrorChirho, DeclChirho, FieldDeclChirho, GadtConArgsChirho,
    GadtConMalformedChirho, GadtConSignatureChirho, GadtProjectionBudgetChirho,
    GadtProjectionLimitChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn fixture_chirho(
    labels_chirho: usize,
    malformed_chirho: Option<GadtConMalformedChirho>,
) -> (ModuleChirho, ConDeclChirho, GadtConSignatureChirho) {
    let signature_chirho = GadtConSignatureChirho {
        prefix_chirho: vec![],
        args_chirho: GadtConArgsChirho::RecordChirho(vec![FieldDeclChirho {
            names_chirho: (0..labels_chirho)
                .map(|index_chirho| name_chirho(&format!("field{index_chirho}Chirho")))
                .collect(),
            ty_chirho: TypeChirho::ConChirho(name_chirho("Int")),
            strictness_chirho: StrictnessChirho::LazyChirho,
            multiplicity_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]),
        result_ty_chirho: TypeChirho::ConChirho(name_chirho("BoxChirho")),
        malformed_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let constructor_chirho = ConDeclChirho::GadtChirho {
        name_chirho: name_chirho("MkBoxChirho"),
        signature_chirho: signature_chirho.clone(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = ModuleChirho {
        name_chirho: name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: name_chirho("BoxChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![constructor_chirho.clone()],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec!["GADTs".to_owned()],
        inline_pragmas_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    (module_chirho, constructor_chirho, signature_chirho)
}

fn assert_depth_refusal_chirho(error_chirho: ConstructorSignatureErrorChirho) {
    assert!(
        matches!(error_chirho, ConstructorSignatureErrorChirho::ProjectionChirho(projection_chirho) if projection_chirho.limit_chirho == GadtProjectionLimitChirho::DepthChirho),
        "{error_chirho}"
    );
    assert!(
        error_chirho.ghc_code_chirho().is_none(),
        "a compiler resource boundary is not a GHC invalidity code"
    );
}

fn th_source_error_chirho(
    error_chirho: haskelujah_th_chirho::reify_chirho::ThReificationErrorChirho,
) -> ConstructorSignatureErrorChirho {
    match error_chirho {
        haskelujah_th_chirho::reify_chirho::ThReificationErrorChirho::ConstructorSignatureChirho(error_chirho) => error_chirho,
        other_chirho => panic!("expected the source/resource error, found {other_chirho}"),
    }
}

#[test]
fn amplified_signature_depth_reaches_every_existing_error_channel_chirho() {
    let (module_chirho, constructor_chirho, signature_chirho) = fixture_chirho(1_000, None);
    assert_depth_refusal_chirho(
        signature_chirho
            .try_as_type_chirho(&mut GadtProjectionBudgetChirho::new_chirho())
            .expect_err("expanded arrow depth is bounded"),
    );
    assert_depth_refusal_chirho(
        haskelujah_core_chirho::desugar_module_chirho(&module_chirho)
            .expect_err("Core must not return a partial module"),
    );
    assert_depth_refusal_chirho(
        haskelujah_naming_chirho::build_iface_chirho(&module_chirho)
            .expect_err("an interface must not publish partial availability"),
    );
    assert_depth_refusal_chirho(th_source_error_chirho(
        haskelujah_th_chirho::reify_chirho::ast_con_to_th_chirho(
            &constructor_chirho,
            &mut GadtProjectionBudgetChirho::new_chirho(),
        )
        .expect_err("TH must not return a partial constructor"),
    ));
    assert_depth_refusal_chirho(th_source_error_chirho(
        haskelujah_th_chirho::reify_chirho::reify_decl_chirho(
            &module_chirho.decls_chirho[0],
            &mut GadtProjectionBudgetChirho::new_chirho(),
        )
        .expect_err("TH exhaustion must not be unsupported-None"),
    ));
    let inferred_chirho =
        haskelujah_typing_chirho::infer_chirho::infer_module_chirho(&module_chirho);
    assert!(
        inferred_chirho
            .diagnostics_chirho
            .to_string()
            .contains("expanded depth")
    );
    let kinds_chirho = haskelujah_typing_chirho::infer_module_kinds_chirho(&module_chirho);
    assert!(
        kinds_chirho
            .diagnostics_chirho
            .to_string()
            .contains("expanded depth")
    );
    let validity_chirho =
        haskelujah_typing_chirho::check_module_type_validity_diagnostics_chirho(&module_chirho);
    assert!(
        validity_chirho
            .diagnostics_chirho
            .to_string()
            .contains("expanded depth")
    );
    let declarations_chirho = haskelujah_typing_chirho::validity_chirho::declarations_chirho::check_module_declaration_validity_diagnostics_chirho(&module_chirho);
    assert!(
        declarations_chirho
            .diagnostics_chirho
            .to_string()
            .contains("expanded depth")
    );
}

#[test]
fn malformed_record_arrows_stay_distinct_from_resource_exhaustion_chirho() {
    let (module_chirho, constructor_chirho, signature_chirho) =
        fixture_chirho(1, Some(GadtConMalformedChirho::IllegalRecordArrowChirho));
    let errors_chirho = [
        signature_chirho
            .try_as_type_chirho(&mut GadtProjectionBudgetChirho::new_chirho())
            .expect_err("malformed recovery is not a type"),
        haskelujah_core_chirho::desugar_module_chirho(&module_chirho)
            .expect_err("malformed recovery is not Core"),
        haskelujah_naming_chirho::build_iface_chirho(&module_chirho)
            .expect_err("malformed recovery is not an interface"),
        th_source_error_chirho(
            haskelujah_th_chirho::reify_chirho::ast_con_to_th_chirho(
                &constructor_chirho,
                &mut GadtProjectionBudgetChirho::new_chirho(),
            )
            .expect_err("malformed recovery is not TH"),
        ),
    ];
    for error_chirho in errors_chirho {
        assert!(matches!(
            error_chirho,
            ConstructorSignatureErrorChirho::MalformedChirho {
                reason_chirho: GadtConMalformedChirho::IllegalRecordArrowChirho,
                ..
            }
        ));
        assert_eq!(error_chirho.ghc_code_chirho(), Some("GHC-37475"));
    }
}
