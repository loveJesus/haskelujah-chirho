// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Literal values

use haskelujah_span_chirho::SpanChirho;

/// A literal value in source code.
#[derive(Debug, Clone, PartialEq)]
pub enum LitChirho {
    /// Integer literal and its source span.
    IntChirho(i64, SpanChirho),
    /// Floating-point literal and its source span.
    FloatChirho(f64, SpanChirho),
    /// Character literal and its source span.
    CharChirho(char, SpanChirho),
    /// String literal contents and its source span.
    StringChirho(String, SpanChirho),
}

impl LitChirho {
    /// Return the source span covering this literal.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::IntChirho(_, s_chirho) => *s_chirho,
            Self::FloatChirho(_, s_chirho) => *s_chirho,
            Self::CharChirho(_, s_chirho) => *s_chirho,
            Self::StringChirho(_, s_chirho) => *s_chirho,
        }
    }
}

fn named_ascii_escape_chirho(name_chirho: &str) -> Option<char> {
    match name_chirho {
        "NUL" => Some('\0'),
        "SOH" => Some('\u{0001}'),
        "STX" => Some('\u{0002}'),
        "ETX" => Some('\u{0003}'),
        "EOT" => Some('\u{0004}'),
        "ENQ" => Some('\u{0005}'),
        "ACK" => Some('\u{0006}'),
        "BEL" => Some('\u{0007}'),
        "BS" => Some('\u{0008}'),
        "HT" => Some('\t'),
        "LF" => Some('\n'),
        "VT" => Some('\u{000B}'),
        "FF" => Some('\u{000C}'),
        "CR" => Some('\r'),
        "SO" => Some('\u{000E}'),
        "SI" => Some('\u{000F}'),
        "DLE" => Some('\u{0010}'),
        "DC1" => Some('\u{0011}'),
        "DC2" => Some('\u{0012}'),
        "DC3" => Some('\u{0013}'),
        "DC4" => Some('\u{0014}'),
        "NAK" => Some('\u{0015}'),
        "SYN" => Some('\u{0016}'),
        "ETB" => Some('\u{0017}'),
        "CAN" => Some('\u{0018}'),
        "EM" => Some('\u{0019}'),
        "SUB" => Some('\u{001A}'),
        "ESC" => Some('\u{001B}'),
        "FS" => Some('\u{001C}'),
        "GS" => Some('\u{001D}'),
        "RS" => Some('\u{001E}'),
        "US" => Some('\u{001F}'),
        "SP" => Some(' '),
        "DEL" => Some('\u{007F}'),
        _ => None,
    }
}

/// Parse the body of a Haskell character literal, excluding its quote marks.
///
/// workflow: language-features-chirho/type-level-character-families-chirho
pub fn parse_haskell_char_body_chirho(body_chirho: &str) -> Option<char> {
    let Some(rest_chirho) = body_chirho.strip_prefix('\\') else {
        return body_chirho.chars().next();
    };

    match rest_chirho.chars().next()? {
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        '"' => Some('"'),
        '0' if rest_chirho.len() == 1 => Some('\0'),
        'a' => Some('\x07'),
        'b' => Some('\x08'),
        'f' => Some('\x0C'),
        'v' => Some('\x0B'),
        'o' => u32::from_str_radix(&rest_chirho[1..], 8)
            .ok()
            .and_then(char::from_u32),
        'x' => u32::from_str_radix(&rest_chirho[1..], 16)
            .ok()
            .and_then(char::from_u32),
        digit_chirho if digit_chirho.is_ascii_digit() => {
            rest_chirho.parse::<u32>().ok().and_then(char::from_u32)
        }
        upper_chirho if upper_chirho.is_ascii_uppercase() => named_ascii_escape_chirho(rest_chirho),
        _ => body_chirho.chars().nth(1),
    }
}

/// Render a character-literal body in a stable Haskell-compatible form.
///
/// workflow: language-features-chirho/type-level-character-families-chirho
pub fn render_haskell_char_body_chirho(value_chirho: char) -> String {
    match value_chirho {
        '\0' => "\\0".to_string(),
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\x07' => "\\a".to_string(),
        '\x08' => "\\b".to_string(),
        '\x0C' => "\\f".to_string(),
        '\x0B' => "\\v".to_string(),
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '"' => "\\\"".to_string(),
        control_chirho if control_chirho.is_control() => {
            format!("\\x{:X}", control_chirho as u32)
        }
        ordinary_chirho => ordinary_chirho.to_string(),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::{parse_haskell_char_body_chirho, render_haskell_char_body_chirho};

    #[test]
    fn parses_haskell_character_escape_forms_chirho() {
        assert_eq!(parse_haskell_char_body_chirho("a"), Some('a'));
        assert_eq!(parse_haskell_char_body_chirho("\\97"), Some('a'));
        assert_eq!(parse_haskell_char_body_chirho("\\o141"), Some('a'));
        assert_eq!(parse_haskell_char_body_chirho("\\x61"), Some('a'));
        assert_eq!(parse_haskell_char_body_chirho("\\NUL"), Some('\0'));
        assert_eq!(parse_haskell_char_body_chirho("\\n"), Some('\n'));
    }

    #[test]
    fn rendered_haskell_character_bodies_round_trip_chirho() {
        for value_chirho in ['a', '\'', '\\', '\n', '\u{0001}', 'λ'] {
            let rendered_chirho = render_haskell_char_body_chirho(value_chirho);
            assert_eq!(
                parse_haskell_char_body_chirho(&rendered_chirho),
                Some(value_chirho)
            );
        }
    }
}
