// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expanded copy counts and direct-API resource boundaries.

use super::gadt_projection_chirho::*;
use super::{
    AstKindChirho, ConDeclChirho, DataKindSigChirho, DeclChirho, FieldDeclChirho,
    GadtConArgsChirho, GadtConFieldChirho, GadtConPrefixChirho, GadtConSignatureChirho,
    StrictnessChirho, TyVarChirho,
};
use crate::module_chirho::ModuleChirho;
use crate::name_chirho::{DefIdChirho, NameChirho, RawNameChirho, ResolvedNameChirho};
use crate::promotion_chirho::{PromotedKeyChirho, PromotionResolutionChirho};
use crate::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn con_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::ConChirho(name_chirho(text_chirho))
}

fn signature_chirho() -> GadtConSignatureChirho {
    GadtConSignatureChirho {
        prefix_chirho: vec![],
        args_chirho: GadtConArgsChirho::PositionalChirho(vec![]),
        result_ty_chirho: con_chirho("RChirho"),
        malformed_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn budget_chirho(output_chirho: usize) -> GadtProjectionBudgetChirho {
    GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
        work_chirho: 1 << 25,
        output_chirho,
        depth_chirho: 512,
    })
}

fn grouped_chirho(labels_chirho: usize, elements_chirho: usize) -> GadtConSignatureChirho {
    let mut signature_chirho = signature_chirho();
    signature_chirho.args_chirho = GadtConArgsChirho::RecordChirho(vec![FieldDeclChirho {
        names_chirho: (0..labels_chirho)
            .map(|index_chirho| name_chirho(&format!("f{index_chirho}Chirho")))
            .collect(),
        ty_chirho: TypeChirho::TupleChirho {
            elements_chirho: (0..elements_chirho)
                .map(|_| con_chirho("XChirho"))
                .collect(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        strictness_chirho: StrictnessChirho::LazyChirho,
        multiplicity_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    signature_chirho
}

fn assert_output_error_chirho(signature_chirho: &GadtConSignatureChirho) {
    let error_chirho =
        check_projection_chirho(signature_chirho, &mut budget_chirho(128), 1).unwrap_err();
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    assert_eq!(error_chirho.span_chirho, signature_chirho.span_chirho);
}

#[test]
fn grouped_label_and_type_sizes_multiply_before_projection_chirho() {
    for signature_chirho in [grouped_chirho(1, 16), grouped_chirho(16, 1)] {
        assert!(check_projection_chirho(&signature_chirho, &mut budget_chirho(512), 1).is_ok());
    }
    let error_chirho =
        check_projection_chirho(&grouped_chirho(16, 16), &mut budget_chirho(512), 1).unwrap_err();
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
}

#[test]
fn a_stream_shares_its_output_allowance_and_keeps_the_first_error_chirho() {
    let mut budget_chirho = budget_chirho(19);
    assert!(check_projection_chirho(&signature_chirho(), &mut budget_chirho, 1).is_ok());
    let first_chirho =
        check_projection_chirho(&signature_chirho(), &mut budget_chirho, 1).unwrap_err();
    assert_eq!(
        first_chirho.limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    let mut tiny_chirho = signature_chirho();
    tiny_chirho.result_ty_chirho = TypeChirho::WildcardChirho {
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        check_projection_chirho(&tiny_chirho, &mut budget_chirho, 1),
        Err(first_chirho)
    );
}

#[test]
fn full_name_transport_metadata_is_charged_for_raw_and_resolved_names_chirho() {
    for resolved_chirho in [false, true] {
        for component_chirho in 0..4 {
            let mut raw_chirho =
                RawNameChirho::unqualified_chirho("XChirho", SpanChirho::DUMMY_CHIRHO);
            match component_chirho {
                0 => raw_chirho.text_chirho = "x".repeat(1024),
                1 => raw_chirho.qualifier_chirho = Some("q".repeat(1024)),
                2 => raw_chirho.family_key_chirho = Some("f".repeat(1024)),
                _ => {
                    raw_chirho.promotion_chirho = Some(PromotionResolutionChirho::ResolvedChirho(
                        PromotedKeyChirho::source_chirho(&"p".repeat(1024), "CChirho"),
                    ))
                }
            }
            let mut signature_chirho = signature_chirho();
            signature_chirho.result_ty_chirho = TypeChirho::ConChirho(if resolved_chirho {
                NameChirho::ResolvedChirho(ResolvedNameChirho {
                    raw_chirho,
                    def_id_chirho: DefIdChirho::new_chirho(1),
                })
            } else {
                NameChirho::RawChirho(raw_chirho)
            });
            assert_output_error_chirho(&signature_chirho);
        }
    }
}

#[test]
fn type_literals_and_record_label_bytes_are_charged_chirho() {
    let mut literal_chirho = signature_chirho();
    literal_chirho.result_ty_chirho = TypeChirho::LitChirho {
        value_chirho: "x".repeat(1024),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_output_error_chirho(&literal_chirho);
    let mut record_chirho = grouped_chirho(1, 1);
    if let GadtConArgsChirho::RecordChirho(fields_chirho) = &mut record_chirho.args_chirho {
        fields_chirho[0].names_chirho[0] = name_chirho(&"f".repeat(1024));
    }
    assert_output_error_chirho(&record_chirho);
}

#[test]
fn binder_kind_and_arrow_multiplicity_names_are_charged_chirho() {
    let mut forall_chirho = signature_chirho();
    forall_chirho
        .prefix_chirho
        .push(GadtConPrefixChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho::annotated_chirho(
                name_chirho("aChirho"),
                AstKindChirho::VarChirho("k".repeat(1024)),
            )],
            required_chirho: true,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });
    assert_output_error_chirho(&forall_chirho);
    let mut multiplicity_chirho = signature_chirho();
    multiplicity_chirho.args_chirho =
        GadtConArgsChirho::PositionalChirho(vec![GadtConFieldChirho {
            strictness_chirho: StrictnessChirho::LazyChirho,
            multiplicity_chirho: Some(MultiplicityChirho::MultVarChirho(name_chirho(
                &"m".repeat(1024),
            ))),
            ty_chirho: con_chirho("XChirho"),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]);
    assert_output_error_chirho(&multiplicity_chirho);
}

#[test]
fn quantified_context_bodies_and_nested_binder_kinds_are_charged_chirho() {
    let mut signature_chirho = signature_chirho();
    signature_chirho
        .prefix_chirho
        .push(GadtConPrefixChirho::ContextChirho {
            constraints_chirho: vec![ConstraintChirho::QuantifiedChirho {
                vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("aChirho"))],
                context_chirho: vec![],
                body_chirho: Box::new(ConstraintChirho::ClassChirho {
                    class_chirho: name_chirho("CChirho"),
                    args_chirho: vec![TypeChirho::PromotedListChirho {
                        elements_chirho: vec![TypeChirho::LitChirho {
                            value_chirho: "x".repeat(1024),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });
    assert_output_error_chirho(&signature_chirho);
}

#[test]
fn expanded_arrows_and_prefixes_obey_the_depth_bound_chirho() {
    let limits_chirho = GadtProjectionLimitsChirho {
        work_chirho: 1 << 20,
        output_chirho: 1 << 20,
        depth_chirho: 16,
    };
    assert!(
        check_projection_chirho(
            &grouped_chirho(1, 1),
            &mut GadtProjectionBudgetChirho::with_limits_chirho(limits_chirho),
            1
        )
        .is_ok()
    );
    let error_chirho = check_projection_chirho(
        &grouped_chirho(16, 1),
        &mut GadtProjectionBudgetChirho::with_limits_chirho(limits_chirho),
        1,
    )
    .unwrap_err();
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::DepthChirho
    );
    let mut prefixed_chirho = signature_chirho();
    prefixed_chirho.prefix_chirho = (0..16)
        .map(|_| GadtConPrefixChirho::ContextChirho {
            constraints_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })
        .collect();
    assert_eq!(
        check_projection_chirho(
            &prefixed_chirho,
            &mut GadtProjectionBudgetChirho::with_limits_chirho(limits_chirho),
            1
        )
        .unwrap_err()
        .limit_chirho,
        GadtProjectionLimitChirho::DepthChirho
    );
}

#[test]
fn wide_fanout_stops_at_the_input_work_bound_chirho() {
    let mut signature_chirho = signature_chirho();
    signature_chirho.result_ty_chirho = TypeChirho::TupleChirho {
        elements_chirho: (0..1000)
            .map(|_| TypeChirho::WildcardChirho {
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 16,
            output_chirho: 1 << 20,
            depth_chirho: 512,
        });
    assert_eq!(
        check_projection_chirho(&signature_chirho, &mut budget_chirho, 1)
            .unwrap_err()
            .limit_chirho,
        GadtProjectionLimitChirho::WorkChirho
    );
}

#[test]
fn representation_expansion_and_arithmetic_never_wrap_chirho() {
    assert!(check_projection_chirho(&signature_chirho(), &mut budget_chirho(20), 1).is_ok());
    assert_eq!(
        check_projection_chirho(&signature_chirho(), &mut budget_chirho(20), 3)
            .unwrap_err()
            .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    for factor_chirho in [0, usize::MAX] {
        let mut budget_chirho =
            GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
                work_chirho: usize::MAX,
                output_chirho: usize::MAX,
                depth_chirho: usize::MAX,
            });
        assert_eq!(
            check_projection_chirho(&grouped_chirho(2, 1), &mut budget_chirho, factor_chirho)
                .unwrap_err()
                .limit_chirho,
            GadtProjectionLimitChirho::ArithmeticChirho
        );
    }
}

fn data_chirho(constructors_chirho: usize, binders_chirho: usize) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("TChirho"),
        type_vars_chirho: (0..binders_chirho)
            .map(|_| TyVarChirho::plain_chirho(name_chirho("pChirho")))
            .collect(),
        constructors_chirho: (0..constructors_chirho)
            .map(|index_chirho| ConDeclChirho::OrdinaryChirho {
                name_chirho: name_chirho(&format!("C{index_chirho}Chirho")),
                fields_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect(),
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn ordinary_constructor_headers_multiply_per_constructor_chirho() {
    for declaration_chirho in [data_chirho(1, 16), data_chirho(16, 1)] {
        assert!(
            check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho(1024), 1)
                .is_ok()
        );
    }
    assert_eq!(
        check_declaration_projection_chirho(&data_chirho(16, 16), &mut budget_chirho(1024), 1)
            .unwrap_err()
            .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
}

#[test]
fn declaration_kind_signatures_are_charged_before_constructor_binding_chirho() {
    for standalone_chirho in [false, true] {
        let mut declaration_chirho = data_chirho(1, 0);
        if let DeclChirho::DataDeclChirho {
            kind_sig_chirho, ..
        } = &mut declaration_chirho
        {
            let kind_chirho = TypeChirho::LitChirho {
                value_chirho: "x".repeat(1024),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            *kind_sig_chirho = Some(if standalone_chirho {
                DataKindSigChirho::StandaloneChirho {
                    signature_chirho: con_chirho("KChirho"),
                    result_chirho: Some(kind_chirho),
                }
            } else {
                DataKindSigChirho::ResultChirho(kind_chirho)
            });
        }
        assert_eq!(
            check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho(128), 1)
                .unwrap_err()
                .limit_chirho,
            GadtProjectionLimitChirho::OutputChirho
        );
    }
}

#[test]
fn ordinary_records_share_the_group_expansion_guard_chirho() {
    let GadtConArgsChirho::RecordChirho(fields_chirho) = grouped_chirho(16, 16).args_chirho else {
        panic!("record fixture");
    };
    let constructor_chirho = ConDeclChirho::RecordChirho {
        name_chirho: name_chirho("CChirho"),
        fields_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert_eq!(
        check_constructor_projection_chirho(&constructor_chirho, &mut budget_chirho(512), 1)
            .unwrap_err()
            .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
}

#[test]
fn a_module_uses_one_budget_for_its_constructor_stream_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: name_chirho("MChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![data_chirho(1, 0), data_chirho(1, 0)],
        extensions_chirho: vec![],
        inline_pragmas_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    assert!(
        check_module_projection_chirho(
            &module_chirho,
            &mut GadtProjectionBudgetChirho::new_chirho(),
            1
        )
        .is_ok()
    );
    assert_eq!(
        check_module_projection_chirho(&module_chirho, &mut budget_chirho(60), 1)
            .unwrap_err()
            .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
}

fn record_fields_chirho(labels_chirho: usize) -> Vec<FieldDeclChirho> {
    match grouped_chirho(labels_chirho, 1).args_chirho {
        GadtConArgsChirho::RecordChirho(fields_chirho) => fields_chirho,
        GadtConArgsChirho::PositionalChirho(_) => panic!("record fixture"),
    }
}

#[test]
fn record_accessors_precharge_quadratic_growth_before_generation_chirho() {
    let constructor_chirho = name_chirho("RecordChirho");
    assert!(
        check_record_accessors_projection_chirho(
            &constructor_chirho,
            &record_fields_chirho(8),
            SpanChirho::DUMMY_CHIRHO,
            &mut budget_chirho(1 << 16)
        )
        .is_ok()
    );
    assert_eq!(
        check_record_accessors_projection_chirho(
            &constructor_chirho,
            &record_fields_chirho(32),
            SpanChirho::DUMMY_CHIRHO,
            &mut budget_chirho(1 << 16)
        )
        .unwrap_err()
        .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
}

#[test]
fn record_accessor_streams_share_failure_without_resetting_the_budget_chirho() {
    let constructor_chirho = name_chirho("RecordChirho");
    let fields_chirho = record_fields_chirho(8);
    let mut budget_chirho = budget_chirho(12_000);
    assert!(
        check_record_accessors_projection_chirho(
            &constructor_chirho,
            &fields_chirho,
            SpanChirho::DUMMY_CHIRHO,
            &mut budget_chirho
        )
        .is_ok()
    );
    let error_chirho = check_record_accessors_projection_chirho(
        &constructor_chirho,
        &fields_chirho,
        SpanChirho::DUMMY_CHIRHO,
        &mut budget_chirho,
    )
    .unwrap_err();
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    assert_eq!(
        check_projection_chirho(&signature_chirho(), &mut budget_chirho, 1),
        Err(error_chirho)
    );
}

#[test]
fn a_large_flat_record_fits_explicit_core_limits_without_deepening_each_pair_chirho() {
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 1 << 27,
            output_chirho: 1 << 24,
            depth_chirho: 16,
        });
    assert!(
        check_record_accessors_projection_chirho(
            &name_chirho("RecordChirho"),
            &record_fields_chirho(128),
            SpanChirho::DUMMY_CHIRHO,
            &mut budget_chirho
        )
        .is_ok()
    );
}

#[test]
fn accessor_amplification_counts_full_constructor_and_field_name_metadata_chirho() {
    let mut constructor_chirho = name_chirho("RecordChirho");
    constructor_chirho.set_family_key_chirho("x".repeat(1 << 18));
    assert_eq!(
        check_record_accessors_projection_chirho(
            &constructor_chirho,
            &record_fields_chirho(2),
            SpanChirho::DUMMY_CHIRHO,
            &mut budget_chirho(1 << 20)
        )
        .unwrap_err()
        .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    let mut fields_chirho = record_fields_chirho(1);
    fields_chirho[0].names_chirho[0].set_family_key_chirho("x".repeat(1 << 18));
    assert_eq!(
        check_record_accessors_projection_chirho(
            &name_chirho("RecordChirho"),
            &fields_chirho,
            SpanChirho::DUMMY_CHIRHO,
            &mut budget_chirho(1 << 20)
        )
        .unwrap_err()
        .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
}
