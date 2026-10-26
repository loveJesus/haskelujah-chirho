// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Haskell 2010 escape grammar (report section 2.6), shared by character and string
//! literals (row 561). Before it, a character body and a string body had separate decoders
//! that disagreed with GHC and with each other:
//! - `\&` was kept as two characters;
//! - `\0` before digits kept the digits;
//! - a caret control (`\^A`) was not decoded;
//! - an ASCII name was not matched by its longest prefix (`\SO1`, `\NULL`).
//!
//! One decoder serves both now. The empty escape `\&` and gaps belong to strings only.
//! workflow: compiler-pipeline-chirho/literal-escapes-chirho

use std::str::Chars;

/// The report's `ascii` names. A three-letter name is tried before a two-letter one.
const ASCII_NAMES_CHIRHO: [(&str, char); 34] = [
    ("NUL", '\u{0}'),
    ("SOH", '\u{1}'),
    ("STX", '\u{2}'),
    ("ETX", '\u{3}'),
    ("EOT", '\u{4}'),
    ("ENQ", '\u{5}'),
    ("ACK", '\u{6}'),
    ("BEL", '\u{7}'),
    ("BS", '\u{8}'),
    ("HT", '\u{9}'),
    ("LF", '\u{a}'),
    ("VT", '\u{b}'),
    ("FF", '\u{c}'),
    ("CR", '\u{d}'),
    ("SO", '\u{e}'),
    ("SI", '\u{f}'),
    ("DLE", '\u{10}'),
    ("DC1", '\u{11}'),
    ("DC2", '\u{12}'),
    ("DC3", '\u{13}'),
    ("DC4", '\u{14}'),
    ("NAK", '\u{15}'),
    ("SYN", '\u{16}'),
    ("ETB", '\u{17}'),
    ("CAN", '\u{18}'),
    ("EM", '\u{19}'),
    ("SUB", '\u{1a}'),
    ("ESC", '\u{1b}'),
    ("FS", '\u{1c}'),
    ("GS", '\u{1d}'),
    ("RS", '\u{1e}'),
    ("US", '\u{1f}'),
    ("SP", ' '),
    ("DEL", '\u{7f}'),
];

/// Consume the digits of one radix and give the code point they spell, or None when there
/// are none, or the value is no character.
fn digits_chirho(chars_chirho: &mut Chars<'_>, radix_chirho: u32) -> Option<char> {
    let mut value_chirho: u32 = 0;
    let mut count_chirho = 0_usize;
    while let Some(digit_chirho) = chars_chirho
        .as_str()
        .chars()
        .next()
        .and_then(|next_chirho| next_chirho.to_digit(radix_chirho))
    {
        chars_chirho.next();
        value_chirho = value_chirho
            .checked_mul(radix_chirho)?
            .checked_add(digit_chirho)?;
        count_chirho += 1;
    }
    if count_chirho == 0 {
        return None;
    }
    char::from_u32(value_chirho)
}

/// Decode the escape that follows a backslash, advancing past it, or None when no rule of the
/// grammar matches.
/// workflow: compiler-pipeline-chirho/literal-escapes-chirho
pub fn decode_escape_chirho(chars_chirho: &mut Chars<'_>) -> Option<char> {
    let rest_chirho = chars_chirho.as_str();
    for width_chirho in [3, 2] {
        let Some(prefix_chirho) = rest_chirho.get(..width_chirho) else {
            continue;
        };
        if let Some(&(_, value_chirho)) = ASCII_NAMES_CHIRHO
            .iter()
            .find(|(name_chirho, _)| *name_chirho == prefix_chirho)
        {
            chars_chirho.nth(width_chirho - 1);
            return Some(value_chirho);
        }
    }
    // A decimal escape takes every digit that follows, so `\012` is 12, not NUL and "12".
    if rest_chirho.starts_with(|first_chirho: char| first_chirho.is_ascii_digit()) {
        return digits_chirho(chars_chirho, 10);
    }
    match chars_chirho.next()? {
        'a' => Some('\u{7}'),
        'b' => Some('\u{8}'),
        'f' => Some('\u{c}'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        'v' => Some('\u{b}'),
        quoted_chirho @ ('\\' | '"' | '\'') => Some(quoted_chirho),
        // `\^@` is 0, `\^A` is 1, through `\^_`, which is 31.
        '^' => match chars_chirho.next()? {
            control_chirho @ '@'..='_' => char::from_u32(u32::from(control_chirho) - 64),
            _ => None,
        },
        'o' => digits_chirho(chars_chirho, 8),
        'x' => digits_chirho(chars_chirho, 16),
        _ => None,
    }
}

/// Decode a string literal's body, without its quotes: escapes, the empty escape `\&`, and
/// gaps (a backslash, whitespace, and a closing backslash). An escape that no rule matches
/// is kept as written.
/// workflow: compiler-pipeline-chirho/literal-escapes-chirho
pub fn unescape_string_body_chirho(body_chirho: &str) -> String {
    let mut out_chirho = String::with_capacity(body_chirho.len());
    let mut chars_chirho = body_chirho.chars();
    while let Some(char_chirho) = chars_chirho.next() {
        if char_chirho != '\\' {
            out_chirho.push(char_chirho);
            continue;
        }
        match chars_chirho.as_str().chars().next() {
            Some('&') => {
                chars_chirho.next();
            }
            Some(space_chirho) if space_chirho.is_whitespace() => {
                for gap_chirho in chars_chirho.by_ref() {
                    if gap_chirho == '\\' {
                        break;
                    }
                }
            }
            _ => {
                let mut escape_chirho = chars_chirho.clone();
                match decode_escape_chirho(&mut escape_chirho) {
                    Some(decoded_chirho) => {
                        out_chirho.push(decoded_chirho);
                        chars_chirho = escape_chirho;
                    }
                    None => out_chirho.push('\\'),
                }
            }
        }
    }
    out_chirho
}
