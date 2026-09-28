// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Data-family constructors and the pattern-given versus expression-wanted boundary.
//! Oracle: data-instances-chirho/reference-chirho.json, GHC 9.14.1.
use super::*;

#[test]
fn data_instance_gadt_does_not_prove_unrelated_pattern_constraints_chirho() {
    let source_chirho = include_str!(
        "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/UnrelatedGivenChirho.hs"
    );
    let prelude_chirho = source_chirho.split("badTupleChirho ::").next().unwrap();
    for (start_chirho, end_chirho) in [
        ("badTupleChirho ::", "badPairChirho ::"),
        ("badPairChirho ::", "badFieldChirho ::"),
        ("badFieldChirho ::", "\0"),
    ] {
        let binding_chirho = source_chirho
            .split_once(start_chirho)
            .unwrap()
            .1
            .split(end_chirho)
            .next()
            .unwrap();
        let isolated_chirho = format!("{prelude_chirho}{start_chirho}{binding_chirho}");
        assert!(
            typecheck_source_chirho(
                &isolated_chirho,
                &mut SourceMapChirho::new_chirho(),
                "UnrelatedGivenChirho.hs"
            )
            .is_err(),
            "{isolated_chirho}"
        );
    }
}

#[test]
fn data_instance_gadt_regression_t16188_chirho() {
    let source_chirho =
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T16188.hs");
    assert_compile_success_chirho("T16188.hs", source_chirho);
}

#[test]
fn data_instance_pattern_givens_do_not_become_expression_proofs_chirho() {
    let source_chirho = include_str!(
        "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/PatternGivenChirho.hs"
    );
    assert_compile_success_chirho("Main.hs", source_chirho);
    assert_execution_chirho(source_chirho, "1\n2\n");
    for invalid_chirho in [
        include_str!(
            "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/PatternSiblingChirho.hs"
        ),
        include_str!(
            "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/ExpressionWantedChirho.hs"
        ),
    ] {
        let error_chirho = typecheck_source_chirho(
            invalid_chirho,
            &mut SourceMapChirho::new_chirho(),
            "Main.hs",
        )
        .err()
        .expect("a True witness requires proof in this branch");
        assert!(
            error_chirho.to_string().contains("type mismatch"),
            "{error_chirho}"
        );
    }
}

#[test]
fn data_instance_invalid_result_and_kind_are_rejected_chirho() {
    for invalid_chirho in [
        include_str!(
            "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/WrongResultChirho.hs"
        ),
        include_str!(
            "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/WrongKindChirho.hs"
        ),
    ] {
        assert!(
            typecheck_source_chirho(
                invalid_chirho,
                &mut SourceMapChirho::new_chirho(),
                "InvalidChirho.hs"
            )
            .is_err(),
            "{invalid_chirho}"
        );
    }
}

#[test]
fn data_instance_constructors_have_their_written_result_chirho() {
    let source_chirho = include_str!(
        "../../../../../test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/data-instances-chirho/ConstructorsChirho.hs"
    );
    assert_compile_success_chirho("Main.hs", source_chirho);
    assert_execution_chirho(source_chirho, "7\nTrue\n");
    let invalid_chirho =
        source_chirho.replace("intChirho (IntChirho 7)", "intChirho (BoolChirho True)");
    assert!(
        typecheck_source_chirho(
            &invalid_chirho,
            &mut SourceMapChirho::new_chirho(),
            "Main.hs"
        )
        .is_err()
    );
}
