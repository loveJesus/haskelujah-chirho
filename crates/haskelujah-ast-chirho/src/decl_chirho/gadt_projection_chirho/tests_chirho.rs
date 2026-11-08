// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Raw quotation headers use actual stored syntax, not expanded GADT arrows.
use super::super::{GadtConMalformedChirho, StrictnessChirho};
use super::*;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn declaration_chirho(
    malformed_chirho: Option<GadtConMalformedChirho>,
    labels_chirho: usize,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho("BoxChirho"),
        type_vars_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        constructors_chirho: vec![ConDeclChirho::GadtChirho {
            name_chirho: name_chirho("MkBoxChirho"),
            signature_chirho: Box::new(GadtConSignatureChirho {
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
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn quoted_record_labels_do_not_expand_into_arrow_depth_or_copied_output_chirho() {
    let declaration_chirho = declaration_chirho(None, 1_000);
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 100_000,
            output_chirho: 0,
            depth_chirho: 32,
        });
    check_borrowed_declaration_syntax_chirho(&declaration_chirho, &mut budget_chirho)
        .expect("flat written labels remain flat quotation data");
    assert_eq!(budget_chirho.remaining_limits_chirho().output_chirho, 0);
    assert!(budget_chirho.remaining_limits_chirho().work_chirho < 100_000);
}

#[test]
fn shallow_parser_exhaustion_marker_remains_terminal_under_quotation_chirho() {
    let mut budget_chirho = GadtProjectionBudgetChirho::new_chirho();
    let error_chirho = check_borrowed_declaration_syntax_chirho(
        &declaration_chirho(Some(GadtConMalformedChirho::ResourceChirho), 1),
        &mut budget_chirho,
    )
    .expect_err("truncated parser syntax must not become an ordinary healthy header");
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::ProducerResourceChirho
    );
    assert_eq!(
        check_borrowed_declaration_syntax_chirho(&declaration_chirho(None, 1), &mut budget_chirho),
        Err(error_chirho)
    );
}
