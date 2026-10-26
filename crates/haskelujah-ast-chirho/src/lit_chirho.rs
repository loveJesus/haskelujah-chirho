// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Literal values

use std::fmt;

use haskelujah_span_chirho::SpanChirho;

use crate::provenance_chirho::{OccurrenceRoleChirho, OriginIdChirho};

mod escape_chirho;
#[cfg(test)]
mod escape_tests_chirho;
pub use escape_chirho::{decode_escape_chirho, unescape_string_body_chirho};

/// A literal value in source code: its value, its span, and, once a producer
/// has stamped it, its origin. A literal is an occurrence: an overloaded
/// literal is a use of `fromInteger`, `fromRational` or `fromString`, and a
/// literal pattern is a comparison. The origin is that use's identity for
/// evidence; equality ignores it, as a name's does.
/// workflow: language-features-chirho/dictionary-evidence-chirho
#[derive(Clone)]
pub enum LitChirho {
    /// Integer literal, its source span and its origin.
    IntChirho(i64, SpanChirho, Option<OriginIdChirho>),
    /// Floating-point literal, its source span and its origin.
    FloatChirho(f64, SpanChirho, Option<OriginIdChirho>),
    /// Character literal, its source span and its origin.
    CharChirho(char, SpanChirho, Option<OriginIdChirho>),
    /// String literal contents, its source span and its origin.
    StringChirho(String, SpanChirho, Option<OriginIdChirho>),
}

impl LitChirho {
    /// Return the source span covering this literal.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::IntChirho(_, s_chirho, _) => *s_chirho,
            Self::FloatChirho(_, s_chirho, _) => *s_chirho,
            Self::CharChirho(_, s_chirho, _) => *s_chirho,
            Self::StringChirho(_, s_chirho, _) => *s_chirho,
        }
    }

    /// The producer-minted origin of this literal occurrence, if it has one.
    pub fn origin_chirho(&self) -> Option<OriginIdChirho> {
        match self {
            Self::IntChirho(_, _, o_chirho)
            | Self::FloatChirho(_, _, o_chirho)
            | Self::CharChirho(_, _, o_chirho)
            | Self::StringChirho(_, _, o_chirho) => *o_chirho,
        }
    }

    /// Give this literal occurrence `origin_chirho`, or take its origin away.
    pub fn set_origin_chirho(&mut self, origin_chirho: Option<OriginIdChirho>) {
        match self {
            Self::IntChirho(_, _, o_chirho)
            | Self::FloatChirho(_, _, o_chirho)
            | Self::CharChirho(_, _, o_chirho)
            | Self::StringChirho(_, _, o_chirho) => *o_chirho = origin_chirho,
        }
    }

    /// The same literal as an occurrence with a producer-minted origin.
    pub fn with_origin_chirho(mut self, origin_chirho: OriginIdChirho) -> Self {
        self.set_origin_chirho(Some(origin_chirho));
        self
    }

    /// The method use an overloaded literal of this kind makes in an
    /// expression. A character literal is never overloaded.
    pub fn role_chirho(&self) -> Option<OccurrenceRoleChirho> {
        match self {
            Self::IntChirho(..) => Some(OccurrenceRoleChirho::IntegerLiteral),
            Self::FloatChirho(..) => Some(OccurrenceRoleChirho::FractionalLiteral),
            Self::StringChirho(..) => Some(OccurrenceRoleChirho::StringLiteral),
            Self::CharChirho(..) => None,
        }
    }
}

impl PartialEq for LitChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        match (self, other_chirho) {
            (Self::IntChirho(a_chirho, sa_chirho, _), Self::IntChirho(b_chirho, sb_chirho, _)) => {
                a_chirho == b_chirho && sa_chirho == sb_chirho
            }
            (
                Self::FloatChirho(a_chirho, sa_chirho, _),
                Self::FloatChirho(b_chirho, sb_chirho, _),
            ) => a_chirho == b_chirho && sa_chirho == sb_chirho,
            (
                Self::CharChirho(a_chirho, sa_chirho, _),
                Self::CharChirho(b_chirho, sb_chirho, _),
            ) => a_chirho == b_chirho && sa_chirho == sb_chirho,
            (
                Self::StringChirho(a_chirho, sa_chirho, _),
                Self::StringChirho(b_chirho, sb_chirho, _),
            ) => a_chirho == b_chirho && sa_chirho == sb_chirho,
            _ => false,
        }
    }
}

impl fmt::Debug for LitChirho {
    /// Renders as the derived form did, and names the origin only when there is
    /// one, so output that never involved an origin is unchanged.
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name_chirho, value_chirho, span_chirho, origin_chirho): (
            &str,
            &dyn fmt::Debug,
            &SpanChirho,
            &Option<OriginIdChirho>,
        ) = match self {
            Self::IntChirho(v_chirho, s_chirho, o_chirho) => {
                ("IntChirho", v_chirho, s_chirho, o_chirho)
            }
            Self::FloatChirho(v_chirho, s_chirho, o_chirho) => {
                ("FloatChirho", v_chirho, s_chirho, o_chirho)
            }
            Self::CharChirho(v_chirho, s_chirho, o_chirho) => {
                ("CharChirho", v_chirho, s_chirho, o_chirho)
            }
            Self::StringChirho(v_chirho, s_chirho, o_chirho) => {
                ("StringChirho", v_chirho, s_chirho, o_chirho)
            }
        };
        let mut tuple_chirho = f_chirho.debug_tuple(name_chirho);
        tuple_chirho.field(value_chirho).field(span_chirho);
        if let Some(origin_chirho) = origin_chirho {
            tuple_chirho.field(origin_chirho);
        }
        tuple_chirho.finish()
    }
}

/// Parse the body of a Haskell character literal, excluding its quote marks.
///
/// workflow: language-features-chirho/type-level-character-families-chirho
pub fn parse_haskell_char_body_chirho(body_chirho: &str) -> Option<char> {
    let Some(rest_chirho) = body_chirho.strip_prefix('\\') else {
        return body_chirho.chars().next();
    };
    // One escape, through the grammar string literals share (row 561), and nothing after it.
    let mut chars_chirho = rest_chirho.chars();
    let value_chirho = decode_escape_chirho(&mut chars_chirho)?;
    chars_chirho.as_str().is_empty().then_some(value_chirho)
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
