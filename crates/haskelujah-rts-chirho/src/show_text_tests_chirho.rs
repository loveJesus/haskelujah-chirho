// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's own `show` for every code point up to 160, the surrogate and top boundaries,
//! and the `String` cases where `showLitString` escapes a quote or inserts `\&` (row 557).
//! The tables are generated from runghc 9.14.1's output for this program:
//!
//! ```haskell
//! module Main where
//!
//! main :: IO ()
//! main = do
//!   mapM_ (\c -> putStrLn ("C\t" ++ show (fromEnum c) ++ "\t" ++ show c)) charsChirho
//!   mapM_ (\s -> putStrLn ("S\t" ++ show (map fromEnum s) ++ "\t" ++ show s)) stringsChirho
//!   putStrLn ("R\t" ++ show (map toEnum [0 .. 130 :: Int] :: String))
//!   where
//!     charsChirho = map toEnum ([0 .. 160] ++ [173, 233, 255, 256, 1234, 55296, 57343, 65533, 1114111]) :: [Char]
//!     stringsChirho =
//!       [ "", "plain", "a\"b", "it's", "\\", "tab\there"
//!       , "\1234" ++ "5", "\1234" ++ "x", "\1234\1235", "\200\"", "\200" ++ "0123"
//!       , "\SO" ++ "H", "\SO" ++ "h", "\SOH", "\SO\SO" ++ "H", "\DEL" ++ "1", "\0" ++ "1"
//!       , "\128\129", "\233x", "x\233", "\n\r\t\v\f\a\b", "'\"'", "\55296" ++ "7", "9\1234" ]
//! ```

use crate::{show_char_chirho, show_string_chirho};

/// (code point, GHC's `show` of that `Char`).
const CHAR_CASES_CHIRHO: &[(u32, &str)] = &[
    (0, r"'\NUL'"),
    (1, r"'\SOH'"),
    (2, r"'\STX'"),
    (3, r"'\ETX'"),
    (4, r"'\EOT'"),
    (5, r"'\ENQ'"),
    (6, r"'\ACK'"),
    (7, r"'\a'"),
    (8, r"'\b'"),
    (9, r"'\t'"),
    (10, r"'\n'"),
    (11, r"'\v'"),
    (12, r"'\f'"),
    (13, r"'\r'"),
    (14, r"'\SO'"),
    (15, r"'\SI'"),
    (16, r"'\DLE'"),
    (17, r"'\DC1'"),
    (18, r"'\DC2'"),
    (19, r"'\DC3'"),
    (20, r"'\DC4'"),
    (21, r"'\NAK'"),
    (22, r"'\SYN'"),
    (23, r"'\ETB'"),
    (24, r"'\CAN'"),
    (25, r"'\EM'"),
    (26, r"'\SUB'"),
    (27, r"'\ESC'"),
    (28, r"'\FS'"),
    (29, r"'\GS'"),
    (30, r"'\RS'"),
    (31, r"'\US'"),
    (32, r"' '"),
    (33, r"'!'"),
    (34, r#"'"'"#),
    (35, r"'#'"),
    (36, r"'$'"),
    (37, r"'%'"),
    (38, r"'&'"),
    (39, r"'\''"),
    (40, r"'('"),
    (41, r"')'"),
    (42, r"'*'"),
    (43, r"'+'"),
    (44, r"','"),
    (45, r"'-'"),
    (46, r"'.'"),
    (47, r"'/'"),
    (48, r"'0'"),
    (49, r"'1'"),
    (50, r"'2'"),
    (51, r"'3'"),
    (52, r"'4'"),
    (53, r"'5'"),
    (54, r"'6'"),
    (55, r"'7'"),
    (56, r"'8'"),
    (57, r"'9'"),
    (58, r"':'"),
    (59, r"';'"),
    (60, r"'<'"),
    (61, r"'='"),
    (62, r"'>'"),
    (63, r"'?'"),
    (64, r"'@'"),
    (65, r"'A'"),
    (66, r"'B'"),
    (67, r"'C'"),
    (68, r"'D'"),
    (69, r"'E'"),
    (70, r"'F'"),
    (71, r"'G'"),
    (72, r"'H'"),
    (73, r"'I'"),
    (74, r"'J'"),
    (75, r"'K'"),
    (76, r"'L'"),
    (77, r"'M'"),
    (78, r"'N'"),
    (79, r"'O'"),
    (80, r"'P'"),
    (81, r"'Q'"),
    (82, r"'R'"),
    (83, r"'S'"),
    (84, r"'T'"),
    (85, r"'U'"),
    (86, r"'V'"),
    (87, r"'W'"),
    (88, r"'X'"),
    (89, r"'Y'"),
    (90, r"'Z'"),
    (91, r"'['"),
    (92, r"'\\'"),
    (93, r"']'"),
    (94, r"'^'"),
    (95, r"'_'"),
    (96, r"'`'"),
    (97, r"'a'"),
    (98, r"'b'"),
    (99, r"'c'"),
    (100, r"'d'"),
    (101, r"'e'"),
    (102, r"'f'"),
    (103, r"'g'"),
    (104, r"'h'"),
    (105, r"'i'"),
    (106, r"'j'"),
    (107, r"'k'"),
    (108, r"'l'"),
    (109, r"'m'"),
    (110, r"'n'"),
    (111, r"'o'"),
    (112, r"'p'"),
    (113, r"'q'"),
    (114, r"'r'"),
    (115, r"'s'"),
    (116, r"'t'"),
    (117, r"'u'"),
    (118, r"'v'"),
    (119, r"'w'"),
    (120, r"'x'"),
    (121, r"'y'"),
    (122, r"'z'"),
    (123, r"'{'"),
    (124, r"'|'"),
    (125, r"'}'"),
    (126, r"'~'"),
    (127, r"'\DEL'"),
    (128, r"'\128'"),
    (129, r"'\129'"),
    (130, r"'\130'"),
    (131, r"'\131'"),
    (132, r"'\132'"),
    (133, r"'\133'"),
    (134, r"'\134'"),
    (135, r"'\135'"),
    (136, r"'\136'"),
    (137, r"'\137'"),
    (138, r"'\138'"),
    (139, r"'\139'"),
    (140, r"'\140'"),
    (141, r"'\141'"),
    (142, r"'\142'"),
    (143, r"'\143'"),
    (144, r"'\144'"),
    (145, r"'\145'"),
    (146, r"'\146'"),
    (147, r"'\147'"),
    (148, r"'\148'"),
    (149, r"'\149'"),
    (150, r"'\150'"),
    (151, r"'\151'"),
    (152, r"'\152'"),
    (153, r"'\153'"),
    (154, r"'\154'"),
    (155, r"'\155'"),
    (156, r"'\156'"),
    (157, r"'\157'"),
    (158, r"'\158'"),
    (159, r"'\159'"),
    (160, r"'\160'"),
    (173, r"'\173'"),
    (233, r"'\233'"),
    (255, r"'\255'"),
    (256, r"'\256'"),
    (1234, r"'\1234'"),
    (55296, r"'\55296'"),
    (57343, r"'\57343'"),
    (65533, r"'\65533'"),
    (1114111, r"'\1114111'"),
];

/// (code points, GHC's `show` of that `String`).
const STRING_CASES_CHIRHO: &[(&[u32], &str)] = &[
    (&[], r#""""#),
    (&[112, 108, 97, 105, 110], r#""plain""#),
    (&[97, 34, 98], r#""a\"b""#),
    (&[105, 116, 39, 115], r#""it's""#),
    (&[92], r#""\\""#),
    (&[116, 97, 98, 9, 104, 101, 114, 101], r#""tab\there""#),
    (&[1234, 53], r#""\1234\&5""#),
    (&[1234, 120], r#""\1234x""#),
    (&[1234, 1235], r#""\1234\1235""#),
    (&[200, 34], r#""\200\"""#),
    (&[200, 48, 49, 50, 51], r#""\200\&0123""#),
    (&[14, 72], r#""\SO\&H""#),
    (&[14, 104], r#""\SOh""#),
    (&[1], r#""\SOH""#),
    (&[14, 14, 72], r#""\SO\SO\&H""#),
    (&[127, 49], r#""\DEL1""#),
    (&[0, 49], r#""\NUL1""#),
    (&[128, 129], r#""\128\129""#),
    (&[233, 120], r#""\233x""#),
    (&[120, 233], r#""x\233""#),
    (&[10, 13, 9, 11, 12, 7, 8], r#""\n\r\t\v\f\a\b""#),
    (&[39, 34, 39], r#""'\"'""#),
    (&[55296, 55], r#""\55296\&7""#),
    (&[57, 1234], r#""9\1234""#),
];

/// GHC's `show` of the `String` holding code points 0 through 130 in order.
const FIRST_131_SHOWN_CHIRHO: &str = r##""\NUL\SOH\STX\ETX\EOT\ENQ\ACK\a\b\t\n\v\f\r\SO\SI\DLE\DC1\DC2\DC3\DC4\NAK\SYN\ETB\CAN\EM\SUB\ESC\FS\GS\RS\US !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\DEL\128\129\130""##;

#[test]
fn each_char_shows_as_ghc_shows_it_chirho() {
    for (code_point_chirho, expected_chirho) in CHAR_CASES_CHIRHO {
        assert_eq!(
            show_char_chirho(*code_point_chirho),
            *expected_chirho,
            "code point {code_point_chirho}"
        );
    }
}

#[test]
fn each_string_shows_as_ghc_shows_it_chirho() {
    for (code_points_chirho, expected_chirho) in STRING_CASES_CHIRHO {
        assert_eq!(
            show_string_chirho(code_points_chirho.iter().copied()),
            *expected_chirho,
            "code points {code_points_chirho:?}"
        );
    }
    assert_eq!(show_string_chirho(0..=130), FIRST_131_SHOWN_CHIRHO);
}
