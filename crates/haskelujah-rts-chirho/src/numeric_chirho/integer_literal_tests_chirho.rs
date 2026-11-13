// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn exact_integer_values_exceed_machine_width_and_normalize_radices_chirho() {
    let decimal_chirho =
        IntegerValueChirho::from_literal_chirho("340282366920938463463374607431768211457").unwrap();
    let hex_chirho =
        IntegerValueChirho::from_literal_chirho("0x1_00000000000000000000000000000001").unwrap();
    assert_eq!(decimal_chirho, hex_chirho);
    assert_eq!(
        decimal_chirho.to_string(),
        "340282366920938463463374607431768211457"
    );
    assert_eq!(
        IntegerValueChirho::from_literal_chirho("0b1_00000000_00000000_00000000_00000001")
            .unwrap()
            .to_string(),
        "4294967297"
    );
}

#[test]
fn malformed_and_over_budget_inputs_never_become_zero_chirho() {
    for text_chirho in [
        "",
        "0x",
        "0b2",
        "12#",
        "-7",
        "+7",
        "not-a-number",
        "1_",
        "0_x1",
        "_1",
    ] {
        assert_eq!(
            IntegerValueChirho::from_literal_chirho(text_chirho),
            Err(IntegerLiteralErrorChirho::MalformedChirho)
        );
    }
    assert_eq!(
        IntegerValueChirho::from_literal_chirho(&"1".repeat(MAX_INTEGER_LITERAL_BYTES_CHIRHO + 1)),
        Err(IntegerLiteralErrorChirho::ResourceChirho)
    );
}
