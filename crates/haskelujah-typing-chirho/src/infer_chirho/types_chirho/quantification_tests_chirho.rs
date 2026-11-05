// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn known_chirho(variable_chirho: u32, kind_chirho: TyChirho) -> QuantifiedVariableChirho {
    QuantifiedVariableChirho {
        variable_chirho: TyVarChirho(variable_chirho),
        kind_chirho: Some(Arc::new(kind_chirho)),
        source_chirho: None,
        specificity_chirho: None,
    }
}

#[test]
fn unknown_quantifiers_do_not_bypass_the_instantiation_allowance_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let mut budget_chirho = EqualityBudgetChirho::new_chirho();
    budget_chirho
        .node_chirho(16_384)
        .expect("drain output allowance");
    assert_eq!(
        context_chirho.instantiate_quantifier_kind_chirho(
            &QuantifiedVariableChirho::unknown_chirho(TyVarChirho(1)),
            &SubstChirho::empty_chirho(),
            &mut budget_chirho,
        ),
        None
    );
}

#[test]
fn committed_kind_solutions_are_not_generalized_again_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .insert(
            TyVarChirho(0),
            known_chirho(0, TyChirho::ConChirho("Type".into())),
        );
    context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .insert(
            TyVarChirho(1),
            known_chirho(1, TyChirho::VarChirho(TyVarChirho(0))),
        );
    context_chirho.apply_subst_all_chirho(&SubstChirho::singleton_chirho(
        TyVarChirho(0),
        TyChirho::ConChirho("Type".into()),
    ));
    let variables_chirho = context_chirho.quantify_variables_chirho(
        vec![TyVarChirho(1)],
        &[],
        &[],
        &SubstChirho::empty_chirho(),
        TyVarSpecificityChirho::InferredChirho,
        &mut EqualityBudgetChirho::new_chirho(),
    );
    assert_eq!(variables_chirho.to_vec(), vec![TyVarChirho(1)]);
    assert_eq!(
        variables_chirho.records_chirho()[0].kind_chirho.as_deref(),
        Some(&TyChirho::ConChirho("Type".into()))
    );
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn hidden_kind_dependencies_precede_specified_source_binders_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .insert(
            TyVarChirho(0),
            known_chirho(0, TyChirho::ConChirho("Type".into())),
        );
    let mut first_chirho = known_chirho(1, TyChirho::VarChirho(TyVarChirho(0)));
    first_chirho.specificity_chirho = Some(TyVarSpecificityChirho::SpecifiedChirho);
    let mut second_chirho = known_chirho(2, TyChirho::ConChirho("Type".into()));
    second_chirho.specificity_chirho = Some(TyVarSpecificityChirho::SpecifiedChirho);
    let variables_chirho = context_chirho.quantify_variables_chirho(
        vec![TyVarChirho(1), TyVarChirho(0), TyVarChirho(2)],
        &[first_chirho.clone(), second_chirho.clone()],
        &[],
        &SubstChirho::empty_chirho(),
        TyVarSpecificityChirho::SpecifiedChirho,
        &mut EqualityBudgetChirho::new_chirho(),
    );
    assert_eq!(
        variables_chirho.to_vec(),
        vec![TyVarChirho(0), TyVarChirho(1), TyVarChirho(2)]
    );
    assert_eq!(
        variables_chirho.records_chirho()[0].specificity_chirho,
        Some(TyVarSpecificityChirho::InferredChirho)
    );
    assert_eq!(variables_chirho.records_chirho()[1], first_chirho);
    assert_eq!(variables_chirho.records_chirho()[2], second_chirho);
}

#[test]
fn conflicting_mirrors_refuse_without_fabricating_metadata_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let known_chirho = known_chirho(1, TyChirho::ConChirho("Type".into()));
    let copied_chirho = context_chirho.quantify_variables_chirho(
        vec![TyVarChirho(1)],
        &[known_chirho.clone(), known_chirho.clone()],
        &[],
        &SubstChirho::empty_chirho(),
        TyVarSpecificityChirho::SpecifiedChirho,
        &mut EqualityBudgetChirho::new_chirho(),
    );
    assert_eq!(copied_chirho.records_chirho(), &[known_chirho.clone()]);
    let refused_chirho = context_chirho.quantify_variables_chirho(
        vec![TyVarChirho(1)],
        &[
            QuantifiedVariableChirho::unknown_chirho(TyVarChirho(1)),
            known_chirho,
        ],
        &[],
        &SubstChirho::empty_chirho(),
        TyVarSpecificityChirho::SpecifiedChirho,
        &mut EqualityBudgetChirho::new_chirho(),
    );
    assert!(refused_chirho.is_empty());
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .is_empty_chirho()
    );
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn cyclic_kind_dependencies_are_explicit_invariant_refusals_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let variables_chirho = context_chirho.quantify_variables_chirho(
        vec![TyVarChirho(1)],
        &[known_chirho(1, TyChirho::VarChirho(TyVarChirho(1)))],
        &[],
        &SubstChirho::empty_chirho(),
        TyVarSpecificityChirho::SpecifiedChirho,
        &mut EqualityBudgetChirho::new_chirho(),
    );
    assert!(variables_chirho.is_empty());
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .is_empty_chirho()
    );
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}

#[test]
fn scheme_opening_avoids_outer_image_capture_by_a_nested_quantifier_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let outer_chirho = known_chirho(1, TyChirho::ConChirho("Type".into()));
    let inner_chirho = known_chirho(0, TyChirho::ConChirho("Type".into()));
    let scheme_chirho = crate::ty_chirho::SchemeChirho {
        vars_chirho: QuantifiedVariablesChirho::from_records_chirho(vec![outer_chirho]),
        preds_chirho: Vec::new(),
        ty_chirho: TyChirho::ForallChirho {
            vars_chirho: QuantifiedVariablesChirho::from_records_chirho(vec![inner_chirho.clone()]),
            body_chirho: Box::new(TyChirho::TupleChirho(vec![
                TyChirho::VarChirho(TyVarChirho(1)),
                TyChirho::VarChirho(TyVarChirho(0)),
            ])),
        },
    };
    let (opened_chirho, _) = context_chirho.instantiate_scheme_parts_chirho(&scheme_chirho);
    let TyChirho::ForallChirho {
        vars_chirho,
        body_chirho,
    } = opened_chirho
    else {
        panic!("nested sigma retained")
    };
    let TyChirho::TupleChirho(elements_chirho) = body_chirho.as_ref() else {
        panic!("ordinary shape retained")
    };
    assert_eq!(elements_chirho[0], TyChirho::VarChirho(TyVarChirho(0)));
    assert_ne!(vars_chirho[0], TyVarChirho(0));
    assert_eq!(elements_chirho[1], TyChirho::VarChirho(vars_chirho[0]));
    assert_eq!(
        vars_chirho.records_chirho()[0].kind_chirho,
        inner_chirho.kind_chirho
    );
    assert!(!context_chirho.type_normalization_exhausted_chirho.get());
}
