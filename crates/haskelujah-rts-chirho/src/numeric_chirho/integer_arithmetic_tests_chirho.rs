// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn integer_chirho(text_chirho: &str) -> IntegerValueChirho {
    IntegerValueChirho::from_literal_chirho(text_chirho).unwrap()
}

#[test]
fn exact_operations_cross_fixed_widths_and_preserve_sign_chirho() {
    let huge_chirho = integer_chirho("340282366920938463463374607431768211457");
    let one_chirho = integer_chirho("1");
    let mut budget_chirho = IntegerBudgetChirho::default();
    for (operation_chirho, expected_chirho) in [
        (
            IntegerOperationChirho::AddChirho,
            "340282366920938463463374607431768211458",
        ),
        (
            IntegerOperationChirho::SubtractChirho,
            "340282366920938463463374607431768211456",
        ),
        (
            IntegerOperationChirho::MultiplyChirho,
            "340282366920938463463374607431768211457",
        ),
    ] {
        assert_eq!(
            huge_chirho
                .apply_chirho(operation_chirho, Some(&one_chirho), &mut budget_chirho)
                .unwrap()
                .to_string(),
            expected_chirho
        );
    }
    let negative_chirho = huge_chirho
        .apply_chirho(
            IntegerOperationChirho::NegateChirho,
            None,
            &mut budget_chirho,
        )
        .unwrap();
    assert_eq!(
        negative_chirho.to_string(),
        "-340282366920938463463374607431768211457"
    );
    assert_eq!(
        negative_chirho
            .apply_chirho(
                IntegerOperationChirho::AbsoluteChirho,
                None,
                &mut budget_chirho
            )
            .unwrap(),
        huge_chirho
    );
    assert_eq!(
        negative_chirho
            .apply_chirho(
                IntegerOperationChirho::AddChirho,
                Some(&huge_chirho),
                &mut budget_chirho
            )
            .unwrap()
            .to_string(),
        "0"
    );
    assert_eq!(
        negative_chirho
            .apply_chirho(
                IntegerOperationChirho::MultiplyChirho,
                Some(&negative_chirho),
                &mut budget_chirho
            )
            .unwrap()
            .to_string(),
        "115792089237316195423570985008687907853950549399482440966384333222776666062849"
    );
}

#[test]
fn signum_and_zero_have_exact_integer_results_chirho() {
    let zero_chirho = integer_chirho("0");
    let one_chirho = integer_chirho("1");
    let mut budget_chirho = IntegerBudgetChirho::default();
    let negative_chirho = one_chirho
        .apply_chirho(
            IntegerOperationChirho::NegateChirho,
            None,
            &mut budget_chirho,
        )
        .unwrap();
    for (value_chirho, expected_chirho) in [
        (&zero_chirho, "0"),
        (&one_chirho, "1"),
        (&negative_chirho, "-1"),
    ] {
        assert_eq!(
            value_chirho
                .apply_chirho(
                    IntegerOperationChirho::SignumChirho,
                    None,
                    &mut budget_chirho
                )
                .unwrap()
                .to_string(),
            expected_chirho
        );
    }
    assert_eq!(
        negative_chirho
            .apply_chirho(
                IntegerOperationChirho::MultiplyChirho,
                Some(&zero_chirho),
                &mut budget_chirho
            )
            .unwrap(),
        zero_chirho
    );
}

#[test]
fn allowances_are_cumulative_and_exhaustion_is_sticky_chirho() {
    let one_chirho = integer_chirho("1");
    let mut budget_chirho = IntegerBudgetChirho::limited_chirho(6, 2);
    assert!(
        one_chirho
            .apply_chirho(
                IntegerOperationChirho::AddChirho,
                Some(&one_chirho),
                &mut budget_chirho
            )
            .is_ok()
    );
    assert!(
        one_chirho
            .apply_chirho(
                IntegerOperationChirho::AddChirho,
                Some(&one_chirho),
                &mut budget_chirho
            )
            .is_ok()
    );
    assert_eq!(
        one_chirho.apply_chirho(
            IntegerOperationChirho::AddChirho,
            Some(&one_chirho),
            &mut budget_chirho
        ),
        Err(IntegerArithmeticErrorChirho::ResourceChirho)
    );
    assert!(budget_chirho.exhausted_chirho());
    assert_eq!(
        one_chirho.apply_chirho(
            IntegerOperationChirho::SignumChirho,
            None,
            &mut budget_chirho
        ),
        Err(IntegerArithmeticErrorChirho::ResourceChirho)
    );
    assert_eq!(one_chirho.to_string(), "1");
}

#[test]
fn refused_output_or_work_never_changes_shared_inputs_chirho() {
    let original_chirho = integer_chirho("340282366920938463463374607431768211457");
    let shared_chirho = original_chirho.clone();
    for (work_chirho, output_chirho) in [(0, 100), (100, 0)] {
        let mut budget_chirho = IntegerBudgetChirho::limited_chirho(work_chirho, output_chirho);
        assert_eq!(
            original_chirho.apply_chirho(
                IntegerOperationChirho::MultiplyChirho,
                Some(&shared_chirho),
                &mut budget_chirho
            ),
            Err(IntegerArithmeticErrorChirho::ResourceChirho)
        );
        assert_eq!(original_chirho, shared_chirho);
        assert_eq!(
            original_chirho.to_string(),
            "340282366920938463463374607431768211457"
        );
    }
    let mut budget_chirho = IntegerBudgetChirho::default();
    assert_eq!(
        original_chirho.apply_chirho(
            IntegerOperationChirho::NegateChirho,
            Some(&shared_chirho),
            &mut budget_chirho
        ),
        Err(IntegerArithmeticErrorChirho::ArityChirho)
    );
    assert!(!budget_chirho.exhausted_chirho());
}

#[test]
fn a_single_result_cannot_grow_past_the_value_ceiling_chirho() {
    let largest_chirho = IntegerValueChirho(Arc::new(
        BigIntChirho::from(1u8) << (MAX_INTEGER_BITS_CHIRHO - 1),
    ));
    let two_chirho = integer_chirho("2");
    let mut budget_chirho = IntegerBudgetChirho::limited_chirho(u64::MAX, u64::MAX);
    assert_eq!(
        largest_chirho.apply_chirho(
            IntegerOperationChirho::MultiplyChirho,
            Some(&two_chirho),
            &mut budget_chirho
        ),
        Err(IntegerArithmeticErrorChirho::ResourceChirho)
    );
    assert!(budget_chirho.exhausted_chirho());
    assert_eq!(largest_chirho.0.bits(), MAX_INTEGER_BITS_CHIRHO);
}
