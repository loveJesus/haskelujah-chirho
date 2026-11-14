// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn conversion_uses_low_bits_and_preserves_the_integer_chirho() {
    for (text_chirho, expected_chirho) in [
        ("0", 0),
        ("18446744073709551621", 5),
        ("-9223372036854775809", i64::MAX),
        ("340282366920938463463374607431768211457", 1),
        ("-340282366920938463463374607431768211457", -1),
    ] {
        let value_chirho =
            IntegerValueChirho::from_literal_chirho(text_chirho.trim_start_matches('-')).unwrap();
        let value_chirho = if text_chirho.starts_with('-') {
            value_chirho
                .apply_chirho(
                    crate::integer_arithmetic_chirho::IntegerOperationChirho::NegateChirho,
                    None,
                    &mut IntegerBudgetChirho::default(),
                )
                .unwrap()
        } else {
            value_chirho
        };
        assert_eq!(
            value_chirho
                .wrapping_int_chirho(&mut IntegerBudgetChirho::default())
                .unwrap(),
            expected_chirho
        );
        assert_eq!(value_chirho.to_string(), text_chirho);
    }
}

#[test]
fn int_and_integer_cannot_replenish_the_same_owner_allowance_chirho() {
    let integer_chirho = IntegerValueChirho::from_literal_chirho("1").unwrap();
    for mut budget_chirho in [
        IntegerBudgetChirho::limited_chirho(6, 100),
        IntegerBudgetChirho::limited_chirho(100, 2),
    ] {
        assert_eq!(
            integer_chirho
                .wrapping_int_chirho(&mut budget_chirho)
                .unwrap(),
            1
        );
        assert_eq!(
            IntOperationChirho::AddChirho
                .apply_chirho(i64::MAX, Some(1), &mut budget_chirho)
                .unwrap(),
            IntOutcomeChirho::IntChirho(i64::MIN)
        );
        assert_eq!(
            integer_chirho.apply_chirho(
                crate::integer_arithmetic_chirho::IntegerOperationChirho::SignumChirho,
                None,
                &mut budget_chirho
            ),
            Err(IntegerArithmeticErrorChirho::ResourceChirho)
        );
        assert_eq!(
            IntOperationChirho::NegateChirho.apply_chirho(0, None, &mut budget_chirho),
            Err(IntegerArithmeticErrorChirho::ResourceChirho)
        );
        assert!(budget_chirho.exhausted_chirho());
        assert_eq!(integer_chirho.to_string(), "1");
    }
}

#[test]
fn invalid_operand_counts_do_not_spend_the_arithmetic_allowance_chirho() {
    let mut budget_chirho = IntegerBudgetChirho::limited_chirho(3, 1);
    for (operation_chirho, right_chirho) in [
        (IntOperationChirho::AddChirho, None),
        (IntOperationChirho::NegateChirho, Some(0)),
        (IntOperationChirho::FromIntegerChirho, None),
    ] {
        assert_eq!(
            operation_chirho.apply_chirho(0, right_chirho, &mut budget_chirho),
            Err(IntegerArithmeticErrorChirho::ArityChirho)
        );
    }
    assert_eq!(
        IntOperationChirho::NegateChirho
            .apply_chirho(i64::MIN, None, &mut budget_chirho)
            .unwrap(),
        IntOutcomeChirho::IntChirho(i64::MIN)
    );
    assert!(!budget_chirho.exhausted_chirho());
}
