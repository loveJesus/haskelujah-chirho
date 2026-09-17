// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Agreement is alpha-renaming, not unification; exhaustion never proves a row.
use super::*;

#[test]
fn row_agreement_preserves_shared_hidden_and_visible_variables_chirho() {
    let left_chirho = TypeFamilyClauseChirho {
        kind_inputs_chirho: vec![TyChirho::VarChirho(TyVarChirho(90))],
        type_inputs_chirho: vec![
            TyChirho::VarChirho(TyVarChirho(2)),
            TyChirho::VarChirho(TyVarChirho(90)),
        ],
        result_chirho: TyChirho::VarChirho(TyVarChirho(2)),
    };
    let mut right_chirho = TypeFamilyClauseChirho {
        kind_inputs_chirho: vec![TyChirho::VarChirho(TyVarChirho(1))],
        type_inputs_chirho: vec![
            TyChirho::VarChirho(TyVarChirho(80)),
            TyChirho::VarChirho(TyVarChirho(1)),
        ],
        result_chirho: TyChirho::VarChirho(TyVarChirho(80)),
    };
    let left_chirho = equation_scheme_chirho(&left_chirho, &mut 16_384).unwrap();
    assert!(
        left_chirho
            .alpha_equivalent_chirho(&equation_scheme_chirho(&right_chirho, &mut 16_384).unwrap())
    );
    right_chirho.result_chirho = TyChirho::VarChirho(TyVarChirho(1));
    assert!(
        !left_chirho
            .alpha_equivalent_chirho(&equation_scheme_chirho(&right_chirho, &mut 16_384).unwrap())
    );
    right_chirho.kind_inputs_chirho.clear();
    assert!(
        !left_chirho
            .alpha_equivalent_chirho(&equation_scheme_chirho(&right_chirho, &mut 16_384).unwrap())
    );
}

#[test]
fn equation_contract_work_is_shared_and_exhaustion_is_unproved_chirho() {
    let equation_chirho = TypeFamilyClauseChirho::ordinary_chirho(
        vec![TyChirho::VarChirho(TyVarChirho(0))],
        TyChirho::VarChirho(TyVarChirho(0)),
    );
    let mut budget_chirho = 3;
    assert!(equation_scheme_chirho(&equation_chirho, &mut budget_chirho).is_some());
    assert!(equation_scheme_chirho(&equation_chirho, &mut budget_chirho).is_none());
    let mut deep_chirho = equation_chirho.clone();
    for _depth_chirho in 0..258 {
        deep_chirho.result_chirho = TyChirho::ListChirho(Box::new(deep_chirho.result_chirho));
    }
    assert!(equation_scheme_chirho(&deep_chirho, &mut 16_384).is_none());
}
