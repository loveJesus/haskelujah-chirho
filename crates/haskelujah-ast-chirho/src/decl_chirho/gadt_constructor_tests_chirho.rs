// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::name_chirho::{NameChirho, RawNameChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn con_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(name_chirho(text_chirho))
}

#[test]
fn a_signature_round_trip_keeps_prefix_scope_and_arrow_multiplicity_chirho() {
    let original_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![name_chirho("aChirho").into()],
        body_chirho: Box::new(TypeChirho::QualChirho {
            context_chirho: vec![ConstraintChirho::ClassChirho {
                class_chirho: name_chirho("Eq"),
                args_chirho: vec![TypeChirho::VarChirho(name_chirho("aChirho"))],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
                mult_chirho: Some(MultiplicityChirho::OneChirho),
                result_chirho: Box::new(con_chirho("ResultChirho")),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let signature_chirho = GadtConSignatureChirho::from_type_chirho(original_chirho.clone());
    assert!(signature_chirho.has_context_chirho());
    assert_eq!(
        signature_chirho
            .try_as_type_chirho(&mut GadtProjectionBudgetChirho::new_chirho())
            .expect("small fixture fits projection budget"),
        original_chirho
    );
}

#[test]
fn grouped_record_labels_keep_argument_order_and_field_annotations_chirho() {
    let signature_chirho = GadtConSignatureChirho {
        prefix_chirho: vec![],
        args_chirho: GadtConArgsChirho::RecordChirho(vec![
            FieldDeclChirho {
                names_chirho: vec![name_chirho("firstChirho"), name_chirho("secondChirho")],
                ty_chirho: con_chirho("Int"),
                strictness_chirho: StrictnessChirho::StrictChirho,
                multiplicity_chirho: Some(MultiplicityChirho::OneChirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            FieldDeclChirho {
                names_chirho: vec![name_chirho("thirdChirho")],
                ty_chirho: con_chirho("Bool"),
                strictness_chirho: StrictnessChirho::LazyChirho,
                multiplicity_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ]),
        result_ty_chirho: con_chirho("ResultChirho"),
        malformed_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let fields_chirho = signature_chirho
        .positional_fields_chirho()
        .collect::<Vec<_>>();
    assert_eq!(fields_chirho.len(), 3);
    assert_eq!(fields_chirho[0].ty_chirho, &con_chirho("Int"));
    assert_eq!(fields_chirho[1].ty_chirho, &con_chirho("Int"));
    assert_eq!(fields_chirho[2].ty_chirho, &con_chirho("Bool"));
    assert_eq!(
        fields_chirho[0].strictness_chirho,
        StrictnessChirho::StrictChirho
    );
    assert_eq!(
        fields_chirho[1].multiplicity_chirho,
        Some(&MultiplicityChirho::OneChirho)
    );
    let rebuilt_chirho = GadtConSignatureChirho::from_type_chirho(
        signature_chirho
            .try_as_type_chirho(&mut GadtProjectionBudgetChirho::new_chirho())
            .expect("small fixture fits projection budget"),
    );
    assert_eq!(rebuilt_chirho.positional_fields_chirho().count(), 3);
    assert_eq!(rebuilt_chirho.result_ty_chirho, con_chirho("ResultChirho"));
    assert!(signature_chirho.record_fields_chirho().is_some());
}

#[test]
fn selector_recoverability_requires_positive_generative_authority_chirho() {
    let field_chirho = FieldDeclChirho {
        names_chirho: vec![name_chirho("payloadChirho")],
        ty_chirho: TypeChirho::VarChirho(name_chirho("aChirho")),
        strictness_chirho: StrictnessChirho::LazyChirho,
        multiplicity_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let app_chirho = |fun_chirho: TypeChirho, arg_chirho: TypeChirho| TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut signature_chirho = GadtConSignatureChirho {
        prefix_chirho: vec![],
        args_chirho: GadtConArgsChirho::RecordChirho(vec![field_chirho.clone()]),
        result_ty_chirho: app_chirho(
            con_chirho("BoxChirho"),
            TypeChirho::VarChirho(name_chirho("aChirho")),
        ),
        malformed_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let local_chirho =
        |head_chirho: &NameChirho| ["BoxChirho", "WrapChirho"].contains(&head_chirho.text_chirho());
    assert!(
        signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &local_chirho
            )
            .expect("small selector fixture fits projection budget")[0]
    );
    signature_chirho.result_ty_chirho = app_chirho(con_chirho("BoxChirho"), con_chirho("Int"));
    assert!(
        !signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &local_chirho
            )
            .expect("small selector fixture fits projection budget")[0]
    );
    signature_chirho.result_ty_chirho = app_chirho(
        con_chirho("BoxChirho"),
        app_chirho(
            con_chirho("WrapChirho"),
            TypeChirho::VarChirho(name_chirho("aChirho")),
        ),
    );
    assert!(
        signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &local_chirho
            )
            .expect("small selector fixture fits projection budget")[0]
    );
    signature_chirho.result_ty_chirho = app_chirho(
        con_chirho("BoxChirho"),
        app_chirho(
            con_chirho("OpaqueChirho"),
            TypeChirho::VarChirho(name_chirho("aChirho")),
        ),
    );
    assert!(
        !signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &local_chirho
            )
            .expect("small selector fixture fits projection budget")[0]
    );
    signature_chirho.result_ty_chirho = app_chirho(
        con_chirho("BoxChirho"),
        TypeChirho::ListChirho {
            element_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    );
    assert!(
        signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &local_chirho
            )
            .expect("small selector fixture fits projection budget")[0]
    );
}

#[test]
fn malformed_recovery_is_never_a_successful_signature_projection_chirho() {
    let mut signature_chirho = GadtConSignatureChirho::from_type_chirho(con_chirho("BoxChirho"));
    signature_chirho.malformed_chirho = Some(GadtConMalformedChirho::IllegalRecordArrowChirho);
    let mut budget_chirho = GadtProjectionBudgetChirho::new_chirho();
    let error_chirho = signature_chirho
        .try_as_type_chirho(&mut budget_chirho)
        .expect_err("a recovered signature is not a validated type");
    assert!(matches!(
        error_chirho,
        ConstructorSignatureErrorChirho::MalformedChirho {
            reason_chirho: GadtConMalformedChirho::IllegalRecordArrowChirho,
            ..
        }
    ));
    assert_eq!(error_chirho.ghc_code_chirho(), Some("GHC-37475"));
    assert!(
        signature_chirho
            .selector_availability_chirho(&mut budget_chirho, &|_| true)
            .is_err()
    );
}

#[test]
fn selector_fields_keep_free_kinds_and_respect_inner_quantifiers_chirho() {
    let mut variable_chirho = TyVarChirho::plain_chirho(name_chirho("xChirho"));
    variable_chirho.kind_annotation_chirho =
        Some(super::super::AstKindChirho::VarChirho("kChirho".to_owned()));
    let mut field_ty_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![variable_chirho],
        body_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(con_chirho("Proxy")),
            arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("xChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut free_chirho = HashSet::new();
    assert!(field_free_vars_chirho(
        &field_ty_chirho,
        &mut HashSet::new(),
        &mut free_chirho
    ));
    assert_eq!(free_chirho, HashSet::from(["kChirho".to_owned()]));
    if let TypeChirho::ForallChirho { vars_chirho, .. } = &mut field_ty_chirho {
        vars_chirho.insert(0, TyVarChirho::plain_chirho(name_chirho("kChirho")));
    }
    free_chirho.clear();
    assert!(field_free_vars_chirho(
        &field_ty_chirho,
        &mut HashSet::new(),
        &mut free_chirho
    ));
    assert!(
        free_chirho.is_empty(),
        "a field-local forall owns its kind binder"
    );
}

#[test]
fn used_result_binders_determine_direct_kinds_transitively_but_not_kind_families_chirho() {
    let mut a_chirho = TyVarChirho::plain_chirho(name_chirho("aChirho"));
    a_chirho.kind_annotation_chirho =
        Some(super::super::AstKindChirho::VarChirho("kChirho".to_owned()));
    let mut k_chirho = TyVarChirho::plain_chirho(name_chirho("kChirho"));
    k_chirho.kind_annotation_chirho =
        Some(super::super::AstKindChirho::VarChirho("jChirho".to_owned()));
    let mut x_chirho = TyVarChirho::plain_chirho(name_chirho("xChirho"));
    x_chirho.kind_annotation_chirho =
        Some(super::super::AstKindChirho::VarChirho("jChirho".to_owned()));
    let mut signature_chirho = GadtConSignatureChirho {
        prefix_chirho: vec![GadtConPrefixChirho::ForallChirho {
            vars_chirho: vec![
                TyVarChirho::plain_chirho(name_chirho("jChirho")),
                k_chirho,
                a_chirho,
            ],
            required_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        args_chirho: GadtConArgsChirho::RecordChirho(vec![FieldDeclChirho {
            names_chirho: vec![name_chirho("payloadChirho")],
            ty_chirho: TypeChirho::ForallChirho {
                vars_chirho: vec![x_chirho],
                body_chirho: Box::new(TypeChirho::VarChirho(name_chirho("xChirho"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            strictness_chirho: StrictnessChirho::LazyChirho,
            multiplicity_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]),
        result_ty_chirho: TypeChirho::AppChirho {
            fun_chirho: Box::new(con_chirho("BoxChirho")),
            arg_chirho: Box::new(TypeChirho::VarChirho(name_chirho("aChirho"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        malformed_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let authority_chirho = |head_chirho: &NameChirho| head_chirho.text_chirho() == "BoxChirho";
    assert_eq!(
        signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &authority_chirho
            )
            .expect("small kind dependency closure"),
        [true]
    );
    if let GadtConPrefixChirho::ForallChirho { vars_chirho, .. } =
        &mut signature_chirho.prefix_chirho[0]
    {
        vars_chirho[2].kind_annotation_chirho = Some(super::super::AstKindChirho::AppChirho(
            Box::new(super::super::AstKindChirho::VarChirho(
                "FamilyChirho".to_owned(),
            )),
            Box::new(super::super::AstKindChirho::VarChirho("kChirho".to_owned())),
        ));
    }
    assert_eq!(
        signature_chirho
            .selector_availability_chirho(
                &mut GadtProjectionBudgetChirho::new_chirho(),
                &authority_chirho
            )
            .expect("an unknown kind application is semantic absence"),
        [false]
    );
}
