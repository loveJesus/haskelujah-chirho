// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing token scanners; the parent keeps cursor state and dispatch.

use super::{
    LexerChirho, RawTokenChirho, RawTokenKindChirho, is_ident_char_chirho, is_symbol_char_chirho,
    keyword_kind_chirho, unicode_ident_continue_len_chirho,
};

impl LexerChirho<'_> {
    pub(super) fn lex_string_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        self.pos_chirho += 1; // skip opening "

        while self.pos_chirho < self.bytes_chirho.len() {
            match self.bytes_chirho[self.pos_chirho] {
                b'"' => {
                    self.pos_chirho += 1;
                    // MagicHash: consume trailing # (e.g. "hello"#)
                    self.consume_magic_hash_chirho();
                    return self
                        .make_token_chirho(RawTokenKindChirho::StringLitChirho, start_chirho);
                }
                b'\\' => {
                    self.pos_chirho += 1;
                    if self.pos_chirho < self.bytes_chirho.len() {
                        if self.bytes_chirho[self.pos_chirho].is_ascii_whitespace() {
                            // String gap: skip whitespace until next backslash
                            while self.pos_chirho < self.bytes_chirho.len()
                                && self.bytes_chirho[self.pos_chirho].is_ascii_whitespace()
                            {
                                self.pos_chirho += 1;
                            }
                            if self.pos_chirho < self.bytes_chirho.len()
                                && self.bytes_chirho[self.pos_chirho] == b'\\'
                            {
                                self.pos_chirho += 1; // skip closing backslash
                            }
                        } else {
                            self.consume_escape_sequence_chirho();
                        }
                    }
                }
                _ => {
                    self.pos_chirho += 1;
                }
            }
        }

        // Unterminated string
        self.make_token_chirho(RawTokenKindChirho::ErrorChirho, start_chirho)
    }

    fn consume_escape_sequence_chirho(&mut self) {
        if self.pos_chirho >= self.bytes_chirho.len() {
            return;
        }

        match self.bytes_chirho[self.pos_chirho] {
            b'o' => {
                self.pos_chirho += 1;
                while self.pos_chirho < self.bytes_chirho.len()
                    && matches!(self.bytes_chirho[self.pos_chirho], b'0'..=b'7')
                {
                    self.pos_chirho += 1;
                }
            }
            b'x' => {
                self.pos_chirho += 1;
                while self.pos_chirho < self.bytes_chirho.len()
                    && self.bytes_chirho[self.pos_chirho].is_ascii_hexdigit()
                {
                    self.pos_chirho += 1;
                }
            }
            b'^' => {
                self.pos_chirho += 1;
                if self.pos_chirho < self.bytes_chirho.len() {
                    self.pos_chirho += 1;
                }
            }
            b'0'..=b'9' => {
                while self.pos_chirho < self.bytes_chirho.len()
                    && self.bytes_chirho[self.pos_chirho].is_ascii_digit()
                {
                    self.pos_chirho += 1;
                }
            }
            b'A'..=b'Z' => {
                while self.pos_chirho < self.bytes_chirho.len()
                    && (self.bytes_chirho[self.pos_chirho].is_ascii_uppercase()
                        || self.bytes_chirho[self.pos_chirho].is_ascii_digit())
                {
                    self.pos_chirho += 1;
                }
            }
            _ => {
                self.pos_chirho += 1;
            }
        }
    }

    pub(super) fn lex_char_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        // Template Haskell quoted names: 'foo, ''Bar, '(+)
        if let Some(next_chirho) = self.peek_at_chirho(1) {
            if next_chirho == b'\'' {
                if let Some(after_second_chirho) = self.peek_at_chirho(2) {
                    if after_second_chirho == b'('
                        || after_second_chirho == b'['
                        || after_second_chirho == b'_'
                        || after_second_chirho.is_ascii_alphabetic()
                    {
                        self.pos_chirho += 1;
                        return self
                            .make_token_chirho(RawTokenKindChirho::TickChirho, start_chirho);
                    }
                }
            }
            if next_chirho == b'_' || next_chirho.is_ascii_lowercase() {
                let mut lookahead_chirho = self.pos_chirho + 1;
                while lookahead_chirho < self.bytes_chirho.len()
                    && (self.bytes_chirho[lookahead_chirho].is_ascii_alphanumeric()
                        || self.bytes_chirho[lookahead_chirho] == b'_')
                {
                    lookahead_chirho += 1;
                }
                if self.bytes_chirho.get(lookahead_chirho) != Some(&b'\'') {
                    self.pos_chirho += 1;
                    return self.make_token_chirho(RawTokenKindChirho::TickChirho, start_chirho);
                }
            }
        }

        // DataKinds: if `'` is followed by an uppercase letter (promoted
        // constructor like 'True, 'Just), `[` (promoted list like '[Int]),
        // `(` (promoted tuple), or `:` (promoted constructor operator like
        // `':`), emit a Tick token and let the parser handle it. BUT: `'A'`,
        // `'('`, `'['`, and `':'` are char literals when followed immediately
        // by a closing `'`.
        if let Some(next_chirho) = self.peek_at_chirho(1) {
            let is_single_char_literal_chirho = self.peek_at_chirho(2) == Some(b'\'');
            if (next_chirho == b'[' || next_chirho == b'(' || next_chirho == b':')
                && !is_single_char_literal_chirho
            {
                self.pos_chirho += 1; // consume just the tick
                return self.make_token_chirho(RawTokenKindChirho::TickChirho, start_chirho);
            }
            if next_chirho.is_ascii_uppercase() {
                // Check if this is a single-char literal like 'A' (char at offset+2 is `'`)
                // vs a promoted constructor like 'True (no closing `'` after the ident)
                let is_char_lit_chirho = self.peek_at_chirho(2) == Some(b'\'');
                if !is_char_lit_chirho {
                    self.pos_chirho += 1; // consume just the tick
                    return self.make_token_chirho(RawTokenKindChirho::TickChirho, start_chirho);
                }
            }
        }

        self.pos_chirho += 1; // skip opening '

        if self.pos_chirho >= self.bytes_chirho.len() {
            return self.make_token_chirho(RawTokenKindChirho::ErrorChirho, start_chirho);
        }

        if self.bytes_chirho[self.pos_chirho] == b'\\' {
            self.pos_chirho += 1; // skip backslash
            self.consume_escape_sequence_chirho();
        } else {
            self.pos_chirho += 1; // skip the character
        }

        if self.pos_chirho < self.bytes_chirho.len() && self.bytes_chirho[self.pos_chirho] == b'\''
        {
            self.pos_chirho += 1;
            // MagicHash: consume trailing # (e.g. 'a'#)
            self.consume_magic_hash_chirho();
            self.make_token_chirho(RawTokenKindChirho::CharLitChirho, start_chirho)
        } else {
            // Not a char literal — might be a tick used for promoted types
            // or name-priming. For now, emit what we have.
            self.make_token_chirho(RawTokenKindChirho::CharLitChirho, start_chirho)
        }
    }

    pub(super) fn lex_number_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        // Check for hex (0x), octal (0o), or binary (0b) prefix
        if self.bytes_chirho[self.pos_chirho] == b'0'
            && self.pos_chirho + 1 < self.bytes_chirho.len()
        {
            match self.bytes_chirho[self.pos_chirho + 1] {
                b'x' | b'X' => {
                    self.pos_chirho += 2;
                    while self.pos_chirho < self.bytes_chirho.len()
                        && (self.bytes_chirho[self.pos_chirho].is_ascii_hexdigit()
                            || self.bytes_chirho[self.pos_chirho] == b'_')
                    {
                        self.pos_chirho += 1;
                    }
                    // HexFloatLiterals: 0xHH.HHpEE or 0xHHpEE
                    let mut is_hex_float_chirho = false;
                    if self.pos_chirho < self.bytes_chirho.len()
                        && self.bytes_chirho[self.pos_chirho] == b'.'
                        && self
                            .peek_at_chirho(1)
                            .is_some_and(|b_chirho| b_chirho.is_ascii_hexdigit())
                    {
                        is_hex_float_chirho = true;
                        self.pos_chirho += 1; // skip .
                        while self.pos_chirho < self.bytes_chirho.len()
                            && (self.bytes_chirho[self.pos_chirho].is_ascii_hexdigit()
                                || self.bytes_chirho[self.pos_chirho] == b'_')
                        {
                            self.pos_chirho += 1;
                        }
                    }
                    // Exponent part: p or P followed by optional sign and decimal digits
                    if self.pos_chirho < self.bytes_chirho.len()
                        && matches!(self.bytes_chirho[self.pos_chirho], b'p' | b'P')
                    {
                        is_hex_float_chirho = true;
                        self.pos_chirho += 1;
                        if self.pos_chirho < self.bytes_chirho.len()
                            && matches!(self.bytes_chirho[self.pos_chirho], b'+' | b'-')
                        {
                            self.pos_chirho += 1;
                        }
                        while self.pos_chirho < self.bytes_chirho.len()
                            && (self.bytes_chirho[self.pos_chirho].is_ascii_digit()
                                || self.bytes_chirho[self.pos_chirho] == b'_')
                        {
                            self.pos_chirho += 1;
                        }
                    }
                    // MagicHash: consume trailing # on hex literals
                    self.consume_magic_hash_chirho();
                    return self.make_token_chirho(
                        if is_hex_float_chirho {
                            RawTokenKindChirho::FloatLitChirho
                        } else {
                            RawTokenKindChirho::IntLitChirho
                        },
                        start_chirho,
                    );
                }
                b'o' | b'O' => {
                    self.pos_chirho += 2;
                    while self.pos_chirho < self.bytes_chirho.len()
                        && matches!(self.bytes_chirho[self.pos_chirho], b'0'..=b'7' | b'_')
                    {
                        self.pos_chirho += 1;
                    }
                    // MagicHash: consume trailing # on octal literals
                    self.consume_magic_hash_chirho();
                    return self.make_token_chirho(RawTokenKindChirho::IntLitChirho, start_chirho);
                }
                b'b' | b'B' => {
                    self.pos_chirho += 2;
                    while self.pos_chirho < self.bytes_chirho.len()
                        && matches!(self.bytes_chirho[self.pos_chirho], b'0' | b'1' | b'_')
                    {
                        self.pos_chirho += 1;
                    }
                    // MagicHash: consume trailing # on binary literals
                    self.consume_magic_hash_chirho();
                    return self.make_token_chirho(RawTokenKindChirho::IntLitChirho, start_chirho);
                }
                _ => {}
            }
        }

        // Decimal digits (with NumericUnderscores support)
        while self.pos_chirho < self.bytes_chirho.len()
            && (self.bytes_chirho[self.pos_chirho].is_ascii_digit()
                || self.bytes_chirho[self.pos_chirho] == b'_')
        {
            self.pos_chirho += 1;
        }

        // Check for float (decimal point followed by digits)
        let mut is_float_chirho = false;
        if self.pos_chirho < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho] == b'.'
            && self
                .peek_at_chirho(1)
                .is_some_and(|b_chirho| b_chirho.is_ascii_digit())
        {
            is_float_chirho = true;
            self.pos_chirho += 1; // skip .
            while self.pos_chirho < self.bytes_chirho.len()
                && (self.bytes_chirho[self.pos_chirho].is_ascii_digit()
                    || self.bytes_chirho[self.pos_chirho] == b'_')
            {
                self.pos_chirho += 1;
            }
        }

        // Check for exponent
        if self.pos_chirho < self.bytes_chirho.len()
            && matches!(self.bytes_chirho[self.pos_chirho], b'e' | b'E')
        {
            is_float_chirho = true;
            self.pos_chirho += 1;
            if self.pos_chirho < self.bytes_chirho.len()
                && matches!(self.bytes_chirho[self.pos_chirho], b'+' | b'-')
            {
                self.pos_chirho += 1;
            }
            while self.pos_chirho < self.bytes_chirho.len()
                && (self.bytes_chirho[self.pos_chirho].is_ascii_digit()
                    || self.bytes_chirho[self.pos_chirho] == b'_')
            {
                self.pos_chirho += 1;
            }
        }

        let kind_chirho = if is_float_chirho {
            RawTokenKindChirho::FloatLitChirho
        } else {
            RawTokenKindChirho::IntLitChirho
        };
        // MagicHash: consume trailing # (e.g. 42#, 3.14##)
        self.consume_magic_hash_chirho();
        self.make_token_chirho(kind_chirho, start_chirho)
    }

    pub(super) fn lex_lower_ident_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        while self.pos_chirho < self.bytes_chirho.len() {
            let b_chirho = self.bytes_chirho[self.pos_chirho];
            if b_chirho.is_ascii_alphanumeric() || b_chirho == b'_' || b_chirho == b'\'' {
                self.pos_chirho += 1;
            } else if b_chirho > 0x7F {
                // Non-ASCII: decode to check if it's an identifier char (not whitespace)
                let len_chirho =
                    unicode_ident_continue_len_chirho(self.source_chirho, self.pos_chirho);
                if len_chirho > 0 {
                    self.pos_chirho += len_chirho;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // MagicHash: consume trailing # (e.g. foo#, bar##)
        self.consume_magic_hash_chirho();

        let text_chirho = self
            .source_chirho
            .get(start_chirho..self.pos_chirho)
            .unwrap_or("");

        // Check for _ (wildcard) as a special case
        if text_chirho == "_" {
            return self.make_token_chirho(RawTokenKindChirho::UnderscoreChirho, start_chirho);
        }

        let kind_chirho =
            keyword_kind_chirho(text_chirho).unwrap_or(RawTokenKindChirho::VarIdChirho);
        self.make_token_chirho(kind_chirho, start_chirho)
    }

    pub(super) fn lex_upper_ident_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        while self.pos_chirho < self.bytes_chirho.len() {
            let b_chirho = self.bytes_chirho[self.pos_chirho];
            if b_chirho.is_ascii_alphanumeric() || b_chirho == b'_' || b_chirho == b'\'' {
                self.pos_chirho += 1;
            } else if b_chirho > 0x7F {
                let len_chirho =
                    unicode_ident_continue_len_chirho(self.source_chirho, self.pos_chirho);
                if len_chirho > 0 {
                    self.pos_chirho += len_chirho;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Check for qualified name: Foo.Bar.baz or Foo.Bar.+
        if self.pos_chirho < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho] == b'.'
            && self.peek_at_chirho(1).is_some_and(|b_chirho| {
                b_chirho.is_ascii_alphanumeric()
                    || b_chirho == b'_'
                    || is_symbol_char_chirho(b_chirho)
            })
        {
            // Consume the rest of the qualified name
            while self.pos_chirho < self.bytes_chirho.len()
                && self.bytes_chirho[self.pos_chirho] == b'.'
            {
                self.pos_chirho += 1; // skip .
                if self.pos_chirho < self.bytes_chirho.len()
                    && is_symbol_char_chirho(self.bytes_chirho[self.pos_chirho])
                {
                    // Qualified operator
                    while self.pos_chirho < self.bytes_chirho.len()
                        && is_symbol_char_chirho(self.bytes_chirho[self.pos_chirho])
                    {
                        self.pos_chirho += 1;
                    }
                    return self
                        .make_token_chirho(RawTokenKindChirho::QualifiedIdChirho, start_chirho);
                }
                // Qualified identifier continuation
                while self.pos_chirho < self.bytes_chirho.len()
                    && is_ident_char_chirho(self.bytes_chirho[self.pos_chirho])
                {
                    self.pos_chirho += 1;
                }
            }
            // MagicHash: consume trailing # on qualified names
            self.consume_magic_hash_chirho();
            return self.make_token_chirho(RawTokenKindChirho::QualifiedIdChirho, start_chirho);
        }

        // MagicHash: consume trailing # (e.g. Int#, MutableArray##)
        self.consume_magic_hash_chirho();
        self.make_token_chirho(RawTokenKindChirho::ConIdChirho, start_chirho)
    }

    pub(super) fn lex_operator_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        let starts_with_colon_chirho = self.bytes_chirho[self.pos_chirho] == b':';

        while self.pos_chirho < self.bytes_chirho.len()
            && is_symbol_char_chirho(self.bytes_chirho[self.pos_chirho])
        {
            self.pos_chirho += 1;
        }

        let kind_chirho = if starts_with_colon_chirho {
            RawTokenKindChirho::ConSymChirho
        } else {
            RawTokenKindChirho::VarSymChirho
        };
        self.make_token_chirho(kind_chirho, start_chirho)
    }
}
