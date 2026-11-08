// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Every retained default argument participates in the shared AST preflight.

use super::*;
use crate::name_chirho::RawNameChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

#[test]
fn associated_default_arguments_are_measured_before_class_projection_chirho() {
    for hidden_chirho in [false, true] {
        let heavy_chirho = TypeChirho::ConChirho(name_chirho(&"x".repeat(4096)));
        let arguments_chirho = if hidden_chirho {
            (vec![heavy_chirho], vec![])
        } else {
            (vec![], vec![heavy_chirho])
        };
        let declaration_chirho = DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: name_chirho("CChirho"),
            type_vars_chirho: vec![],
            methods_chirho: vec![],
            associated_tfs_chirho: vec![AssocTypeFamilyChirho {
                injectivity_chirho: None,
                name_chirho: name_chirho("FChirho"),
                type_vars_chirho: vec![],
                result_kind_chirho: None,
                data_family_chirho: false,
                default_rhs_chirho: None,
                default_arguments_chirho: Some(arguments_chirho),
                default_params_chirho: vec![],
                default_binders_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert!(
            check_declaration_projection_chirho(
                &declaration_chirho,
                &mut GadtProjectionBudgetChirho::new_chirho(),
                1
            )
            .is_ok()
        );
        let mut budget_chirho =
            GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
                work_chirho: 1 << 20,
                output_chirho: 1024,
                depth_chirho: 128,
            });
        assert_eq!(
            check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho, 1)
                .unwrap_err()
                .limit_chirho,
            GadtProjectionLimitChirho::OutputChirho
        );
        assert!(
            check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho, 1)
                .is_err(),
            "resource refusal stays latched"
        );
    }
}

#[test]
fn standalone_class_kind_shares_the_projection_allowance_and_terminal_failure_chirho() {
    let mut declaration_chirho = DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![],
        name_chirho: name_chirho("CChirho"),
        type_vars_chirho: vec![],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 1 << 20,
            output_chirho: 1024,
            depth_chirho: 128,
        });
    check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho, 1).unwrap();
    let DeclChirho::ClassDeclChirho {
        standalone_kind_chirho,
        ..
    } = &mut declaration_chirho
    else {
        unreachable!()
    };
    *standalone_kind_chirho = Some(TypeChirho::ConChirho(name_chirho(&"x".repeat(4096))));
    assert_eq!(
        check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho, 1)
            .unwrap_err()
            .limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    if let DeclChirho::ClassDeclChirho {
        standalone_kind_chirho,
        ..
    } = &mut declaration_chirho
    {
        *standalone_kind_chirho = None;
    }
    assert!(
        check_declaration_projection_chirho(&declaration_chirho, &mut budget_chirho, 1).is_err()
    );
}
