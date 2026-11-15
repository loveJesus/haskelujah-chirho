// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;

#[test]
fn signed_decimal_and_precedence_preserve_every_boundary_chirho() {
    let mut budget_chirho = IntegerBudgetChirho::default();
    for (precedence_chirho, number_chirho, expected_chirho) in [
        (6, -5, "-5"),
        (7, -5, "(-5)"),
        (11, 5, "5"),
        (11, i64::MIN, "(-9223372036854775808)"),
        (0, i64::MIN, "-9223372036854775808"),
    ] {
        assert_eq!(
            show_int_chirho(precedence_chirho, number_chirho, &mut budget_chirho).unwrap(),
            expected_chirho
        );
    }
    let huge_chirho = IntegerValueChirho::from_literal_chirho("12345678901234567890123")
        .unwrap()
        .apply_chirho(
            crate::integer_arithmetic_chirho::IntegerOperationChirho::NegateChirho,
            None,
            &mut budget_chirho,
        )
        .unwrap();
    assert_eq!(
        huge_chirho
            .show_decimal_chirho(7, &mut budget_chirho)
            .unwrap(),
        "(-12345678901234567890123)"
    );
    assert_eq!(
        huge_chirho
            .show_decimal_chirho(6, &mut budget_chirho)
            .unwrap(),
        "-12345678901234567890123"
    );
    assert_eq!(
        IntegerValueChirho::from_literal_chirho("0")
            .unwrap()
            .show_decimal_chirho(11, &mut budget_chirho)
            .unwrap(),
        "0"
    );
}

#[test]
fn conversion_and_expansion_share_one_latched_allowance_chirho() {
    let value_chirho = IntegerValueChirho::from_literal_chirho("18446744073709551616").unwrap();
    for (work_chirho, output_chirho) in [(1, 1000), (10000, 1)] {
        let mut budget_chirho = IntegerBudgetChirho::limited_chirho(work_chirho, output_chirho);
        assert_eq!(
            value_chirho.show_decimal_chirho(0, &mut budget_chirho),
            Err(IntegerArithmeticErrorChirho::ResourceChirho)
        );
        assert!(budget_chirho.exhausted_chirho());
        assert_eq!(
            show_int_chirho(0, 0, &mut budget_chirho),
            Err(IntegerArithmeticErrorChirho::ResourceChirho)
        );
        assert_eq!(value_chirho.to_string(), "18446744073709551616");
    }
    let mut budget_chirho = IntegerBudgetChirho::limited_chirho(10000, 100);
    let mut rendered_chirho = 0;
    while show_int_chirho(0, 1, &mut budget_chirho).is_ok() {
        rendered_chirho += 1;
    }
    assert!(rendered_chirho > 0 && rendered_chirho < 10);
    assert!(budget_chirho.exhausted_chirho());
}

#[test]
fn formatting_keys_are_exact_and_never_library_spellings_chirho() {
    for operation_chirho in ShowNumberOperationChirho::ALL_CHIRHO {
        assert_eq!(
            ShowNumberOperationChirho::from_primitive_chirho(operation_chirho.primitive_chirho()),
            Some(operation_chirho)
        );
    }
    assert_eq!(
        ShowNumberOperationChirho::from_primitive_chirho("showSignedInt"),
        None
    );
    assert_eq!(
        ShowNumberOperationChirho::from_primitive_chirho("showsPrec"),
        None
    );
}

#[test]
fn an_oversized_decimal_declines_before_creating_text_chirho() {
    // This compact binary value would need more than 64 KiB of decimal text.
    let value_chirho = IntegerValueChirho(std::sync::Arc::new(
        num_bigint::BigInt::from(1_u8) << 220_000_usize,
    ));
    let mut budget_chirho = IntegerBudgetChirho::limited_chirho(u64::MAX, u64::MAX);
    assert_eq!(
        value_chirho.show_decimal_chirho(0, &mut budget_chirho),
        Err(IntegerArithmeticErrorChirho::ResourceChirho)
    );
    assert!(budget_chirho.exhausted_chirho());
}
