// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared symbol identity and maximal munch.
//! Workflow: language-features-chirho/flat-type-syntax-chirho.md.
use super::{LexerChirho, RawTokenChirho, RawTokenKindChirho, unicode_ident_continue_len_chirho};
use unicode_general_category_chirho::{
    GeneralCategory as GeneralCategoryChirho, get_general_category as get_general_category_chirho,
};

/// ASCII symbols plus GHC's non-ASCII symbol categories (Lexer.Interface.adjustChar).
/// Brackets and quotation punctuation are graphic, not operators, in GHC9.14.1.
/// Letters, marks, digits, spaces and control characters do not become operators.
pub(crate) fn is_symbol_char_chirho(character_chirho: char) -> bool {
    if character_chirho.is_ascii() {
        return matches!(
            character_chirho,
            '!' | '#'
                | '$'
                | '%'
                | '&'
                | '*'
                | '+'
                | '.'
                | '/'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '\\'
                | '^'
                | '|'
                | '-'
                | '~'
                | ':'
        );
    }
    matches!(
        get_general_category_chirho(character_chirho),
        GeneralCategoryChirho::MathSymbol
            | GeneralCategoryChirho::CurrencySymbol
            | GeneralCategoryChirho::ModifierSymbol
            | GeneralCategoryChirho::OtherSymbol
            | GeneralCategoryChirho::ConnectorPunctuation
            | GeneralCategoryChirho::DashPunctuation
            | GeneralCategoryChirho::OtherPunctuation
    )
}

impl LexerChirho<'_> {
    pub(super) fn has_more_symbol_at_chirho(&self, offset_chirho: usize) -> bool {
        self.source_chirho
            .get(self.pos_chirho + offset_chirho..)
            .and_then(|rest_chirho| rest_chirho.chars().next())
            .is_some_and(is_symbol_char_chirho)
    }

    /// A dash run is a comment only when its next character is not a symbol.
    pub(super) fn starts_line_comment_chirho(&self) -> bool {
        if self.peek_at_chirho(1) != Some(b'-') {
            return false;
        }
        let mut offset_chirho = 2;
        while self.peek_at_chirho(offset_chirho) == Some(b'-') {
            offset_chirho += 1;
        }
        !self.has_more_symbol_at_chirho(offset_chirho)
    }

    pub(super) fn starts_unboxed_paren_open_chirho(&self) -> bool {
        if self.peek_at_chirho(1) != Some(b'#') {
            return false;
        }
        // Preserve names such as (#.) and (#⊗), but keep unboxed sum/tuple syntax.
        if self.has_more_symbol_at_chirho(2) && !matches!(self.peek_at_chirho(2), Some(b'#' | b'|'))
        {
            let end_chirho = self.symbol_end_chirho(self.pos_chirho + 2);
            if self.bytes_chirho.get(end_chirho) == Some(&b')') {
                return false;
            }
        }
        true
    }

    /// Callers enter at a token start or after checked ASCII delimiters, never
    /// at a recovery byte inside a scalar. Keep that precondition explicit.
    fn symbol_end_chirho(&self, start_chirho: usize) -> usize {
        debug_assert!(self.source_chirho.is_char_boundary(start_chirho));
        let mut end_chirho = start_chirho;
        for character_chirho in self.source_chirho[start_chirho..].chars() {
            if !is_symbol_char_chirho(character_chirho) {
                break;
            }
            end_chirho += character_chirho.len_utf8();
        }
        end_chirho
    }

    fn consume_ident_tail_chirho(&mut self) {
        loop {
            let length_chirho =
                unicode_ident_continue_len_chirho(self.source_chirho, self.pos_chirho);
            if length_chirho == 0 {
                break;
            }
            self.pos_chirho += length_chirho;
        }
    }

    pub(super) fn lex_upper_ident_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        self.consume_ident_tail_chirho();
        let mut qualified_chirho = false;
        while self.peek_at_chirho(0) == Some(b'.') {
            let next_chirho = self.source_chirho[self.pos_chirho + 1..].chars().next();
            if !next_chirho.is_some_and(|character_chirho| {
                character_chirho.is_alphanumeric()
                    || character_chirho == '_'
                    || is_symbol_char_chirho(character_chirho)
            }) {
                break;
            }
            qualified_chirho = true;
            self.pos_chirho += 1;
            if self.has_more_symbol_at_chirho(0) {
                self.pos_chirho = self.symbol_end_chirho(self.pos_chirho);
                return self.make_token_chirho(RawTokenKindChirho::QualifiedIdChirho, start_chirho);
            }
            self.consume_ident_tail_chirho();
        }
        self.consume_magic_hash_chirho();
        self.make_token_chirho(
            if qualified_chirho {
                RawTokenKindChirho::QualifiedIdChirho
            } else {
                RawTokenKindChirho::ConIdChirho
            },
            start_chirho,
        )
    }

    pub(super) fn lex_operator_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        self.pos_chirho = self.symbol_end_chirho(self.pos_chirho);
        // Reserved punctuation is recognized AFTER maximal munch. A reserved
        // prefix inside ->⊗ or →⊗ must not discard the operator's second operand.
        let text_chirho = &self.source_chirho[start_chirho..self.pos_chirho];
        let kind_chirho = match text_chirho {
            "->" | "→" => RawTokenKindChirho::RightArrowChirho,
            "<-" | "←" => RawTokenKindChirho::LeftArrowChirho,
            "::" | "∷" => RawTokenKindChirho::ColonColonChirho,
            "=>" | "⇒" => RawTokenKindChirho::FatArrowChirho,
            "∀" => RawTokenKindChirho::ForallChirho,
            "⊸" => RawTokenKindChirho::LinearArrowChirho,
            ".." => RawTokenKindChirho::DotDotChirho,
            "=" => RawTokenKindChirho::EqualsChirho,
            "\\" => RawTokenKindChirho::BackslashChirho,
            "|" => RawTokenKindChirho::PipeChirho,
            "@" => RawTokenKindChirho::AtChirho,
            "~" => RawTokenKindChirho::TildeChirho,
            _ if text_chirho.starts_with(':') => RawTokenKindChirho::ConSymChirho,
            _ => RawTokenKindChirho::VarSymChirho,
        };
        self.make_token_chirho(kind_chirho, start_chirho)
    }
}
