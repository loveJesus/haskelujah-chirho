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

#[test]
fn inversion_requires_the_proofs_ordered_source_rows_chirho() {
    use crate::kind_chirho::ClosedFamilyInjectivityChirho;
    use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho};
    let span_chirho = |offset_chirho| {
        SpanChirho::new_chirho(
            FileIdChirho::SYNTHETIC_CHIRHO,
            ByteOffsetChirho::new_chirho(offset_chirho),
            ByteOffsetChirho::new_chirho(offset_chirho + 1),
        )
    };
    let source_rows_chirho = vec![span_chirho(1), span_chirho(2)];
    let mut contracts_chirho = DeclarationContractsChirho::default();
    contracts_chirho.families_chirho.insert(
        "FChirho".into(),
        FamilyContractChirho::ClosedChirho(
            [("Int", "Bool"), ("Char", "Int")]
                .into_iter()
                .enumerate()
                .map(
                    |(index_chirho, (input_chirho, result_chirho))| FamilyEquationContractChirho {
                        source_chirho: source_rows_chirho[index_chirho],
                        scheme_chirho: equation_scheme_chirho(
                            &TypeFamilyClauseChirho::ordinary_chirho(
                                vec![TyChirho::ConChirho(input_chirho.into())],
                                TyChirho::ConChirho(result_chirho.into()),
                            ),
                            &mut 16_384,
                        )
                        .unwrap(),
                    },
                )
                .collect(),
        ),
    );
    let mut proof_chirho = ClosedFamilyInjectivityChirho {
        positions_chirho: vec![0],
        source_rows_chirho,
    };
    let argument_chirho = TyChirho::VarChirho(TyVarChirho(100));
    let invert_chirho = |proof_chirho: &ClosedFamilyInjectivityChirho| {
        contracts_chirho.inverse_closed_family_chirho(
            "FChirho",
            &[(&argument_chirho, false)],
            &TyChirho::bool_chirho(),
            proof_chirho,
            &|_| false,
        )
    };
    assert_eq!(
        invert_chirho(&proof_chirho),
        Some(vec![(argument_chirho.clone(), TyChirho::int_chirho())])
    );
    proof_chirho.source_rows_chirho.reverse();
    assert!(
        invert_chirho(&proof_chirho).is_none(),
        "a reordered row list is not the validated body"
    );
    proof_chirho.source_rows_chirho.reverse();
    proof_chirho.source_rows_chirho.pop();
    assert!(
        invert_chirho(&proof_chirho).is_none(),
        "an omitted row is not the validated body"
    );
    proof_chirho.source_rows_chirho.push(span_chirho(3));
    assert!(
        invert_chirho(&proof_chirho).is_none(),
        "another source row is not the validated body"
    );
}
