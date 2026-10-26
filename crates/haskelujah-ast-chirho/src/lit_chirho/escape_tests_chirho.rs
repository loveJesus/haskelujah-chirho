// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The escape grammar against GHC's own lexer (row 561). Each body is written once; the
//! code points are runghc 9.14.1's `map fromEnum` of it as a string literal, or `fromEnum`
//! of it as a character literal, generated from `EscapeTableChirho.hs` in the row's receipts.

use super::{parse_haskell_char_body_chirho, unescape_string_body_chirho};

/// (a string literal's body as written, the code points GHC lexes from it).
const STRING_CASES_CHIRHO: &[(&str, &[u32])] = &[
    (r"\1234\&5", &[1234, 53]),
    (r"\SO\&H", &[14, 72]),
    (r"a\&b\&", &[97, 98]),
    (r"\012x\0\01\00", &[12, 120, 0, 1, 0]),
    (r"\NULL\SOHx\SO1", &[0, 76, 1, 120, 14, 49]),
    (r"\^A\^Z\^_\^@x", &[1, 26, 31, 0, 120]),
    (r#"it\'s \"q\""#, &[105, 116, 39, 115, 32, 34, 113, 34]),
    (r"ab\   \cd", &[97, 98, 99, 100]),
    (r"\x7f\o177\127", &[127, 127, 127]),
    (r"\a\b\f\n\r\t\v", &[7, 8, 12, 10, 13, 9, 11]),
    (r"\SP\DEL\ESC\EM", &[32, 127, 27, 25]),
    (r"\^\\^]\^^", &[28, 29, 30]),
    (r"\x10FFFF", &[1114111]),
    (r"\o0\1\&", &[0, 1]),
    (r"\&", &[]),
    (r"", &[]),
    (r"\\", &[92]),
    (r#"\""#, &[34]),
    (r"\DC1\DC4\NAK", &[17, 20, 21]),
    (r"\BSx\HTy", &[8, 120, 9, 121]),
    (r"\STX\ETX\EOT\ENQ\ACK\BEL", &[2, 3, 4, 5, 6, 7]),
    (r"\LF\VT\FF\CR\SI\DLE", &[10, 11, 12, 13, 15, 16]),
    (
        r"\DC2\DC3\SYN\ETB\CAN\SUB\FS\GS\RS\US",
        &[18, 19, 22, 23, 24, 26, 28, 29, 30, 31],
    ),
];

/// (a character literal's body as written, the code point GHC lexes from it).
const CHAR_CASES_CHIRHO: &[(&str, u32)] = &[
    (r"\'", 39),
    (r#"""#, 34),
    (r"\\", 92),
    (r"\n", 10),
    (r"\DEL", 127),
    (r"\SOH", 1),
    (r"\SO", 14),
    (r"\^A", 1),
    (r"\^@", 0),
    (r"\^[", 27),
    (r"\^\", 28),
    (r"\x41", 65),
    (r"\o101", 65),
    (r"\65", 65),
    (r"\SP", 32),
    (r"a", 97),
    (r"\1114111", 1114111),
    (r"\t", 9),
    (r"\^_", 31),
];

#[test]
fn each_string_body_decodes_as_ghc_lexes_it_chirho() {
    for (body_chirho, expected_chirho) in STRING_CASES_CHIRHO {
        let decoded_chirho: Vec<u32> = unescape_string_body_chirho(body_chirho)
            .chars()
            .map(u32::from)
            .collect();
        assert_eq!(decoded_chirho, *expected_chirho, "{body_chirho}");
    }
}

#[test]
fn each_char_body_decodes_as_ghc_lexes_it_chirho() {
    for (body_chirho, expected_chirho) in CHAR_CASES_CHIRHO {
        assert_eq!(
            parse_haskell_char_body_chirho(body_chirho).map(u32::from),
            Some(*expected_chirho),
            "{body_chirho}"
        );
    }
}
