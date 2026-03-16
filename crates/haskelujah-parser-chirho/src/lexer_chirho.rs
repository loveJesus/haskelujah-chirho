// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Lexer for Haskell source code
//!
//! Produces a stream of tokens from raw source text. Handles:
//! - All Haskell 2010 lexical constructs
//! - Nested block comments (`{- ... {- ... -} ... -}`)
//! - String gaps (`"hello \    \world"`)
//! - Numeric literals (decimal, hex `0x`, octal `0o`)
//! - Character escapes
//! - The layout rule (virtual `{`, `}`, `;` insertion)
//!
//! The lexer is implemented as an iterator that yields `RawTokenChirho` values.
//! Layout rule processing happens as a separate pass over the raw token stream.

use haskelujah_span_chirho::{ByteOffsetChirho, FileIdChirho, SpanChirho};

// ---------------------------------------------------------------------------
// RawTokenKindChirho — token kinds before layout rule processing
// ---------------------------------------------------------------------------

/// Token kind produced by the raw lexer (before the layout rule inserts
/// virtual braces and semicolons).
///
/// This is a simplified token kind enum. Once the full `TokenKindChirho` from
/// `haskelujah-syntax-chirho` is wired in, this will be replaced or mapped to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawTokenKindChirho {
    // -- Keywords --
    CaseChirho,
    ClassChirho,
    DataChirho,
    DefaultChirho,
    DerivingChirho,
    DoChirho,
    ElseChirho,
    ForeignChirho,
    IfChirho,
    ImportChirho,
    InChirho,
    InfixChirho,
    InfixlChirho,
    InfixrChirho,
    InstanceChirho,
    LetChirho,
    ModuleChirho,
    NewtypeChirho,
    OfChirho,
    ThenChirho,
    TypeChirho,
    WhereChirho,
    ForallChirho,

    // -- Identifiers and operators --
    /// A lowercase identifier (variable or function name).
    VarIdChirho,
    /// An uppercase identifier (constructor, type, module name).
    ConIdChirho,
    /// A symbolic operator (e.g. `+`, `>>=`, `.:`)
    VarSymChirho,
    /// A constructor operator (e.g. `:`, `:+:`)
    ConSymChirho,
    /// A qualified identifier (e.g. `Data.List.sort`)
    QualifiedIdChirho,

    // -- Literals --
    IntLitChirho,
    FloatLitChirho,
    CharLitChirho,
    StringLitChirho,

    // -- Punctuation --
    LeftParenChirho,
    RightParenChirho,
    LeftBracketChirho,
    RightBracketChirho,
    LeftBraceChirho,
    RightBraceChirho,
    CommaChirho,
    SemicolonChirho,
    BacktickChirho,

    // -- Special symbols --
    DotDotChirho,       // ..
    ColonColonChirho,   // ::
    EqualsChirho,       // =
    BackslashChirho,    // \ (lambda)
    PipeChirho,         // |
    LeftArrowChirho,    // <-
    RightArrowChirho,   // ->
    LinearArrowChirho,  // ⊸  (LinearTypes)
    FatArrowChirho,     // =>
    AtChirho,           // @
    TildeChirho,        // ~
    UnderscoreChirho,   // _ (wildcard)

    // -- Trivia --
    WhitespaceChirho,
    LineCommentChirho,
    BlockCommentChirho,
    DocCommentChirho,
    /// `{-# ... #-}` pragma (LANGUAGE, OPTIONS, etc.)
    PragmaChirho,

    // -- Layout tokens (inserted by layout rule pass) --
    VirtualLeftBraceChirho,
    VirtualRightBraceChirho,
    VirtualSemicolonChirho,

    /// A `'` tick used for DataKinds promoted constructors (`'True`, `'Just`, `'[]`).
    TickChirho,

    // -- Template Haskell --
    /// The `$` splice operator.
    ThSpliceChirho,
    /// The `$$` typed splice operator.
    ThTypedSpliceChirho,
    /// Opening `[|` for expression quotation.
    ThOpenExpQuoteChirho,
    /// Closing `|]` for quotation.
    ThCloseQuoteChirho,
    /// Opening `[d|` for declaration quotation.
    ThOpenDecQuoteChirho,
    /// Opening `[t|` for type quotation.
    ThOpenTypeQuoteChirho,
    /// Opening `[p|` for pattern quotation.
    ThOpenPatQuoteChirho,
    /// Opening `[e|` for expression quotation (explicit).
    ThOpenExpExplicitQuoteChirho,
    /// Opening `[||` for typed expression quotation.
    ThOpenTypedExpQuoteChirho,
    /// Closing `||]` for typed expression quotation.
    ThCloseTypedQuoteChirho,

    // -- Special --
    EofChirho,
    ErrorChirho,
}

impl RawTokenKindChirho {
    /// Whether this token introduces a layout context (requires virtual braces).
    pub fn is_layout_keyword_chirho(self) -> bool {
        matches!(
            self,
            Self::WhereChirho | Self::LetChirho | Self::DoChirho | Self::OfChirho
        )
    }

    /// Whether this token is trivia (whitespace or comment).
    pub fn is_trivia_chirho(self) -> bool {
        matches!(
            self,
            Self::WhitespaceChirho
                | Self::LineCommentChirho
                | Self::BlockCommentChirho
                | Self::DocCommentChirho
                | Self::PragmaChirho
        )
    }

    /// Whether this is a keyword.
    pub fn is_keyword_chirho(self) -> bool {
        matches!(
            self,
            Self::CaseChirho
                | Self::ClassChirho
                | Self::DataChirho
                | Self::DefaultChirho
                | Self::DerivingChirho
                | Self::DoChirho
                | Self::ElseChirho
                | Self::ForeignChirho
                | Self::IfChirho
                | Self::ImportChirho
                | Self::InChirho
                | Self::InfixChirho
                | Self::InfixlChirho
                | Self::InfixrChirho
                | Self::InstanceChirho
                | Self::LetChirho
                | Self::ModuleChirho
                | Self::NewtypeChirho
                | Self::OfChirho
                | Self::ThenChirho
                | Self::TypeChirho
                | Self::WhereChirho
                | Self::ForallChirho
        )
    }
}

// ---------------------------------------------------------------------------
// RawTokenChirho — a token with its span
// ---------------------------------------------------------------------------

/// A raw token produced by the lexer, before layout processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawTokenChirho {
    pub kind_chirho: RawTokenKindChirho,
    pub span_chirho: SpanChirho,
}

// ---------------------------------------------------------------------------
// LexerChirho — the raw lexer
// ---------------------------------------------------------------------------

/// Raw lexer that scans Haskell source text into tokens.
///
/// Does NOT handle the layout rule — that is a separate pass over the
/// raw token stream.
pub struct LexerChirho<'src> {
    source_chirho: &'src str,
    bytes_chirho: &'src [u8],
    pos_chirho: usize,
    file_id_chirho: FileIdChirho,
}

impl<'src> LexerChirho<'src> {
    /// Create a new lexer for the given source text.
    pub fn new_chirho(source_chirho: &'src str, file_id_chirho: FileIdChirho) -> Self {
        Self {
            source_chirho,
            bytes_chirho: source_chirho.as_bytes(),
            pos_chirho: 0,
            file_id_chirho,
        }
    }

    /// Lex the entire source into a vec of tokens (including trivia).
    pub fn lex_all_chirho(&mut self) -> Vec<RawTokenChirho> {
        let mut tokens_chirho = Vec::new();
        loop {
            let token_chirho = self.next_token_chirho();
            let is_eof_chirho = token_chirho.kind_chirho == RawTokenKindChirho::EofChirho;
            tokens_chirho.push(token_chirho);
            if is_eof_chirho {
                break;
            }
        }
        tokens_chirho
    }

    /// Get the next token.
    pub fn next_token_chirho(&mut self) -> RawTokenChirho {
        if self.pos_chirho >= self.bytes_chirho.len() {
            return self.make_token_chirho(RawTokenKindChirho::EofChirho, self.pos_chirho);
        }

        let start_chirho = self.pos_chirho;
        let byte_chirho = self.bytes_chirho[self.pos_chirho];

        match byte_chirho {
            // Whitespace
            b' ' | b'\t' | b'\r' | b'\n' => self.lex_whitespace_chirho(start_chirho),

            // Line comment or operator starting with -
            b'-' => {
                if self.peek_at_chirho(1) == Some(b'-')
                    && self.peek_at_chirho(2).is_none_or(|b_chirho| !is_symbol_char_chirho(b_chirho))
                {
                    self.lex_line_comment_chirho(start_chirho)
                } else if self.peek_at_chirho(1) == Some(b'>') && !self.has_more_symbol_at_chirho(2)
                {
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::RightArrowChirho, start_chirho)
                } else {
                    self.lex_operator_chirho(start_chirho)
                }
            }

            // Block comment
            b'{' => {
                if self.peek_at_chirho(1) == Some(b'-') {
                    self.lex_block_comment_chirho(start_chirho)
                } else {
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::LeftBraceChirho, start_chirho)
                }
            }

            // Punctuation
            b'(' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::LeftParenChirho, start_chirho)
            }
            b')' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::RightParenChirho, start_chirho)
            }
            b'[' => {
                // Template Haskell quotation brackets
                if self.peek_at_chirho(1) == Some(b'|') && self.peek_at_chirho(2) == Some(b'|') {
                    // [|| — typed expression quote
                    self.pos_chirho += 3;
                    self.make_token_chirho(RawTokenKindChirho::ThOpenTypedExpQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1) == Some(b'|') {
                    // [| — expression quote
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::ThOpenExpQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1) == Some(b'd') && self.peek_at_chirho(2) == Some(b'|') {
                    // [d| — declaration quote
                    self.pos_chirho += 3;
                    self.make_token_chirho(RawTokenKindChirho::ThOpenDecQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1) == Some(b't') && self.peek_at_chirho(2) == Some(b'|') {
                    // [t| — type quote
                    self.pos_chirho += 3;
                    self.make_token_chirho(RawTokenKindChirho::ThOpenTypeQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1) == Some(b'p') && self.peek_at_chirho(2) == Some(b'|') {
                    // [p| — pattern quote
                    self.pos_chirho += 3;
                    self.make_token_chirho(RawTokenKindChirho::ThOpenPatQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1) == Some(b'e') && self.peek_at_chirho(2) == Some(b'|') {
                    // [e| — explicit expression quote
                    self.pos_chirho += 3;
                    self.make_token_chirho(RawTokenKindChirho::ThOpenExpExplicitQuoteChirho, start_chirho)
                } else {
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::LeftBracketChirho, start_chirho)
                }
            }
            b']' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::RightBracketChirho, start_chirho)
            }
            b'}' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::RightBraceChirho, start_chirho)
            }
            b',' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::CommaChirho, start_chirho)
            }
            b';' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::SemicolonChirho, start_chirho)
            }
            b'`' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::BacktickChirho, start_chirho)
            }

            // String literal
            b'"' => self.lex_string_chirho(start_chirho),

            // Character literal
            b'\'' => self.lex_char_chirho(start_chirho),

            // Numeric literal
            b'0'..=b'9' => self.lex_number_chirho(start_chirho),

            // Identifiers and keywords
            b'a'..=b'z' | b'_' => self.lex_lower_ident_chirho(start_chirho),
            b'A'..=b'Z' => self.lex_upper_ident_chirho(start_chirho),

            // Operators
            b':' => {
                if self.peek_at_chirho(1) == Some(b':') && !self.has_more_symbol_at_chirho(2) {
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::ColonColonChirho, start_chirho)
                } else {
                    self.lex_operator_chirho(start_chirho)
                }
            }
            b'.' => {
                if self.peek_at_chirho(1) == Some(b'.') && !self.has_more_symbol_at_chirho(2) {
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::DotDotChirho, start_chirho)
                } else if self.peek_at_chirho(1).is_some_and(|b_chirho| is_symbol_char_chirho(b_chirho)) {
                    self.lex_operator_chirho(start_chirho)
                } else {
                    // Standalone dot — could be a qualified name separator or operator
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::VarSymChirho, start_chirho)
                }
            }
            b'=' => {
                if self.peek_at_chirho(1) == Some(b'>') && !self.has_more_symbol_at_chirho(2) {
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::FatArrowChirho, start_chirho)
                } else if self.peek_at_chirho(1).is_some_and(|b_chirho| is_symbol_char_chirho(b_chirho)) {
                    self.lex_operator_chirho(start_chirho)
                } else {
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::EqualsChirho, start_chirho)
                }
            }
            b'\\' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::BackslashChirho, start_chirho)
            }
            b'|' => {
                // Template Haskell closing quotes
                if self.peek_at_chirho(1) == Some(b'|') && self.peek_at_chirho(2) == Some(b']') {
                    // ||] — typed expression quote close
                    self.pos_chirho += 3;
                    self.make_token_chirho(RawTokenKindChirho::ThCloseTypedQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1) == Some(b']') {
                    // |] — quote close
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::ThCloseQuoteChirho, start_chirho)
                } else if self.peek_at_chirho(1).is_some_and(|b_chirho| is_symbol_char_chirho(b_chirho)) {
                    self.lex_operator_chirho(start_chirho)
                } else {
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::PipeChirho, start_chirho)
                }
            }
            b'<' => {
                if self.peek_at_chirho(1) == Some(b'-') && !self.has_more_symbol_at_chirho(2) {
                    self.pos_chirho += 2;
                    self.make_token_chirho(RawTokenKindChirho::LeftArrowChirho, start_chirho)
                } else {
                    self.lex_operator_chirho(start_chirho)
                }
            }
            b'@' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::AtChirho, start_chirho)
            }
            b'~' => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::TildeChirho, start_chirho)
            }

            // Template Haskell splice: $ or $$
            // $ is a TH splice only when immediately followed by an identifier or '('
            // $$ is a typed splice only when immediately followed by an identifier or '('
            // Otherwise $ is a normal operator (e.g. `f $ x`)
            b'$' => {
                let next_chirho = self.peek_at_chirho(1);
                if next_chirho == Some(b'$') {
                    let after_chirho = self.peek_at_chirho(2);
                    if after_chirho.is_some_and(|b_chirho| b_chirho.is_ascii_alphabetic() || b_chirho == b'_' || b_chirho == b'(') {
                        // $$(name) or $$name — typed splice
                        self.pos_chirho += 2;
                        self.make_token_chirho(RawTokenKindChirho::ThTypedSpliceChirho, start_chirho)
                    } else {
                        self.lex_operator_chirho(start_chirho)
                    }
                } else if next_chirho.is_some_and(|b_chirho| b_chirho.is_ascii_alphabetic() || b_chirho == b'_' || b_chirho == b'(') {
                    // $(expr) or $name — splice
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::ThSpliceChirho, start_chirho)
                } else {
                    // Standalone $ operator or part of longer operator ($>, etc.)
                    self.lex_operator_chirho(start_chirho)
                }
            }

            // Any other symbol character
            _ if is_symbol_char_chirho(byte_chirho) => self.lex_operator_chirho(start_chirho),

            // Non-ASCII bytes — could be Unicode identifiers, BOM, etc.
            _ if byte_chirho > 0x7F => {
                // UnicodeSyntax: recognize Unicode alternatives for Haskell punctuation
                let rest_chirho_pre = self.source_chirho.get(self.pos_chirho..);
                if let Some(rest_pre_chirho) = rest_chirho_pre {
                    if let Some(ch_pre_chirho) = rest_pre_chirho.chars().next() {
                        let (kind_opt_chirho, len_chirho) = match ch_pre_chirho {
                            '→' => (Some(RawTokenKindChirho::RightArrowChirho), ch_pre_chirho.len_utf8()),
                            '←' => (Some(RawTokenKindChirho::LeftArrowChirho), ch_pre_chirho.len_utf8()),
                            '∷' => (Some(RawTokenKindChirho::ColonColonChirho), ch_pre_chirho.len_utf8()),
                            '⇒' => (Some(RawTokenKindChirho::FatArrowChirho), ch_pre_chirho.len_utf8()),
                            '∀' => (Some(RawTokenKindChirho::ForallChirho), ch_pre_chirho.len_utf8()),
                            'λ' => (Some(RawTokenKindChirho::BackslashChirho), ch_pre_chirho.len_utf8()),
                            '⊸' => (Some(RawTokenKindChirho::LinearArrowChirho), ch_pre_chirho.len_utf8()),
                            _ => (None, 0),
                        };
                        if let Some(kind_chirho) = kind_opt_chirho {
                            self.pos_chirho += len_chirho;
                            return self.make_token_chirho(kind_chirho, start_chirho);
                        }
                    }
                }

                // Decode the UTF-8 character to advance by the right number of bytes
                let rest_chirho = match self.source_chirho.get(self.pos_chirho..) {
                    Some(s_chirho) => s_chirho,
                    None => {
                        self.pos_chirho += 1;
                        return self.make_token_chirho(RawTokenKindChirho::ErrorChirho, start_chirho);
                    }
                };
                if let Some(ch_chirho) = rest_chirho.chars().next() {
                    if ch_chirho.is_alphabetic() || ch_chirho == '\u{FEFF}' {
                        // Unicode identifier or BOM — treat as ident start
                        self.pos_chirho += ch_chirho.len_utf8();
                        // Continue eating ident chars
                        while self.pos_chirho < self.bytes_chirho.len()
                            && is_ident_char_chirho(self.bytes_chirho[self.pos_chirho])
                        {
                            self.pos_chirho += 1;
                        }
                        // Determine if it's lower or upper
                        if ch_chirho.is_uppercase() {
                            self.make_token_chirho(RawTokenKindChirho::ConIdChirho, start_chirho)
                        } else {
                            self.make_token_chirho(RawTokenKindChirho::VarIdChirho, start_chirho)
                        }
                    } else if ch_chirho.is_whitespace() {
                        // Unicode whitespace (e.g. non-breaking space \u{A0})
                        self.pos_chirho += ch_chirho.len_utf8();
                        while self.pos_chirho < self.source_chirho.len() {
                            let rest2_chirho = &self.source_chirho[self.pos_chirho..];
                            match rest2_chirho.chars().next() {
                                Some(c_chirho) if c_chirho.is_whitespace() && c_chirho != '\n' => {
                                    self.pos_chirho += c_chirho.len_utf8();
                                }
                                _ => break,
                            }
                        }
                        self.make_token_chirho(RawTokenKindChirho::WhitespaceChirho, start_chirho)
                    } else {
                        // Unicode symbol/punctuation — treat as operator or error
                        self.pos_chirho += ch_chirho.len_utf8();
                        self.make_token_chirho(RawTokenKindChirho::VarSymChirho, start_chirho)
                    }
                } else {
                    // Invalid UTF-8 — advance one byte
                    self.pos_chirho += 1;
                    self.make_token_chirho(RawTokenKindChirho::ErrorChirho, start_chirho)
                }
            }

            // Unknown ASCII / error
            _ => {
                self.pos_chirho += 1;
                self.make_token_chirho(RawTokenKindChirho::ErrorChirho, start_chirho)
            }
        }
    }

    // -- Helper methods --

    fn peek_at_chirho(&self, offset_chirho: usize) -> Option<u8> {
        self.bytes_chirho.get(self.pos_chirho + offset_chirho).copied()
    }

    fn has_more_symbol_at_chirho(&self, offset_chirho: usize) -> bool {
        self.peek_at_chirho(offset_chirho)
            .is_some_and(|b_chirho| is_symbol_char_chirho(b_chirho))
    }

    /// MagicHash: consume one or more trailing `#` characters.
    /// In GHC, `MagicHash` allows `#` at the end of identifiers and literals
    /// (e.g. `Int#`, `foo#`, `42#`, `"hello"#`, `3.14##`).
    /// We always accept this in the lexer since `#` is syntactically unambiguous
    /// at these positions — it cannot appear in standard Haskell here.
    fn consume_magic_hash_chirho(&mut self) {
        while self.pos_chirho < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho] == b'#'
        {
            self.pos_chirho += 1;
        }
    }

    fn make_token_chirho(
        &self,
        kind_chirho: RawTokenKindChirho,
        start_chirho: usize,
    ) -> RawTokenChirho {
        RawTokenChirho {
            kind_chirho,
            span_chirho: SpanChirho::new_chirho(
                self.file_id_chirho,
                ByteOffsetChirho::from_usize_chirho(start_chirho),
                ByteOffsetChirho::from_usize_chirho(self.pos_chirho),
            ),
        }
    }

    fn lex_whitespace_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        while self.pos_chirho < self.bytes_chirho.len()
            && matches!(
                self.bytes_chirho[self.pos_chirho],
                b' ' | b'\t' | b'\r' | b'\n'
            )
        {
            self.pos_chirho += 1;
        }
        self.make_token_chirho(RawTokenKindChirho::WhitespaceChirho, start_chirho)
    }

    fn lex_line_comment_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        // Check for haddock: --|, --^, -- |, -- ^
        let after_dashes_chirho = self.pos_chirho + 2;
        let is_doc_chirho = if after_dashes_chirho < self.bytes_chirho.len() {
            let c_chirho = self.bytes_chirho[after_dashes_chirho];
            if c_chirho == b'|' || c_chirho == b'^' {
                true
            } else if c_chirho == b' ' && after_dashes_chirho + 1 < self.bytes_chirho.len() {
                let c2_chirho = self.bytes_chirho[after_dashes_chirho + 1];
                c2_chirho == b'|' || c2_chirho == b'^'
            } else {
                false
            }
        } else {
            false
        };

        while self.pos_chirho < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho] != b'\n'
        {
            self.pos_chirho += 1;
        }
        let kind_chirho = if is_doc_chirho {
            RawTokenKindChirho::DocCommentChirho
        } else {
            RawTokenKindChirho::LineCommentChirho
        };
        self.make_token_chirho(kind_chirho, start_chirho)
    }

    fn lex_block_comment_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        // Check for pragma: {-#
        let is_pragma_chirho = self.pos_chirho + 2 < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho + 2] == b'#';
        // Check for haddock: {-| or {-^
        let is_doc_chirho = !is_pragma_chirho
            && self.pos_chirho + 2 < self.bytes_chirho.len()
            && (self.bytes_chirho[self.pos_chirho + 2] == b'|'
                || self.bytes_chirho[self.pos_chirho + 2] == b'^');

        self.pos_chirho += 2; // skip {-
        if is_pragma_chirho {
            self.pos_chirho += 1; // skip #
        }
        let mut depth_chirho: u32 = 1;

        while self.pos_chirho < self.bytes_chirho.len() && depth_chirho > 0 {
            if is_pragma_chirho
                && self.bytes_chirho[self.pos_chirho] == b'#'
                && self.peek_at_chirho(1) == Some(b'-')
                && self.peek_at_chirho(2) == Some(b'}')
            {
                // Pragma close: #-}
                self.pos_chirho += 3;
                depth_chirho = 0;
            } else if self.bytes_chirho[self.pos_chirho] == b'{'
                && self.peek_at_chirho(1) == Some(b'-')
            {
                depth_chirho += 1;
                self.pos_chirho += 2;
            } else if self.bytes_chirho[self.pos_chirho] == b'-'
                && self.peek_at_chirho(1) == Some(b'}')
            {
                depth_chirho -= 1;
                self.pos_chirho += 2;
            } else {
                self.pos_chirho += 1;
            }
        }

        let kind_chirho = if is_pragma_chirho {
            RawTokenKindChirho::PragmaChirho
        } else if is_doc_chirho {
            RawTokenKindChirho::DocCommentChirho
        } else {
            RawTokenKindChirho::BlockCommentChirho
        };
        self.make_token_chirho(kind_chirho, start_chirho)
    }

    fn lex_string_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        self.pos_chirho += 1; // skip opening "

        while self.pos_chirho < self.bytes_chirho.len() {
            match self.bytes_chirho[self.pos_chirho] {
                b'"' => {
                    self.pos_chirho += 1;
                    // MagicHash: consume trailing # (e.g. "hello"#)
                    self.consume_magic_hash_chirho();
                    return self.make_token_chirho(
                        RawTokenKindChirho::StringLitChirho,
                        start_chirho,
                    );
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
                            self.pos_chirho += 1; // skip escaped char
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

    fn lex_char_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        // DataKinds: if `'` is followed by an uppercase letter (promoted
        // constructor like 'True, 'Just) or `[` (promoted list like '[Int]),
        // emit a Tick token and let the parser handle it.
        // BUT: `'A'` is a char literal, not a promoted constructor — check that
        // the uppercase letter is not followed immediately by a closing `'`.
        if let Some(next_chirho) = self.peek_at_chirho(1) {
            if next_chirho == b'[' || next_chirho == b'(' {
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
            if self.pos_chirho < self.bytes_chirho.len() {
                self.pos_chirho += 1; // skip escaped char
            }
        } else {
            self.pos_chirho += 1; // skip the character
        }

        if self.pos_chirho < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho] == b'\''
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

    fn lex_number_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        // Check for hex (0x), octal (0o), or binary (0b) prefix
        if self.bytes_chirho[self.pos_chirho] == b'0' && self.pos_chirho + 1 < self.bytes_chirho.len() {
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
                        && self.peek_at_chirho(1).is_some_and(|b_chirho| b_chirho.is_ascii_hexdigit())
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
                    return self.make_token_chirho(
                        RawTokenKindChirho::IntLitChirho,
                        start_chirho,
                    );
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
                    return self.make_token_chirho(
                        RawTokenKindChirho::IntLitChirho,
                        start_chirho,
                    );
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
            && self.peek_at_chirho(1).is_some_and(|b_chirho| b_chirho.is_ascii_digit())
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

    fn lex_lower_ident_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        while self.pos_chirho < self.bytes_chirho.len()
            && is_ident_char_chirho(self.bytes_chirho[self.pos_chirho])
        {
            self.pos_chirho += 1;
        }

        // MagicHash: consume trailing # (e.g. foo#, bar##)
        self.consume_magic_hash_chirho();

        let text_chirho = self.source_chirho
            .get(start_chirho..self.pos_chirho)
            .unwrap_or("");

        // Check for _ (wildcard) as a special case
        if text_chirho == "_" {
            return self.make_token_chirho(RawTokenKindChirho::UnderscoreChirho, start_chirho);
        }

        let kind_chirho = keyword_kind_chirho(text_chirho)
            .unwrap_or(RawTokenKindChirho::VarIdChirho);
        self.make_token_chirho(kind_chirho, start_chirho)
    }

    fn lex_upper_ident_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
        while self.pos_chirho < self.bytes_chirho.len()
            && is_ident_char_chirho(self.bytes_chirho[self.pos_chirho])
        {
            self.pos_chirho += 1;
        }

        // Check for qualified name: Foo.Bar.baz or Foo.Bar.+
        if self.pos_chirho < self.bytes_chirho.len()
            && self.bytes_chirho[self.pos_chirho] == b'.'
            && self.peek_at_chirho(1).is_some_and(|b_chirho| {
                b_chirho.is_ascii_alphanumeric() || b_chirho == b'_' || is_symbol_char_chirho(b_chirho)
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
                    return self.make_token_chirho(
                        RawTokenKindChirho::QualifiedIdChirho,
                        start_chirho,
                    );
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
            return self.make_token_chirho(
                RawTokenKindChirho::QualifiedIdChirho,
                start_chirho,
            );
        }

        // MagicHash: consume trailing # (e.g. Int#, MutableArray##)
        self.consume_magic_hash_chirho();
        self.make_token_chirho(RawTokenKindChirho::ConIdChirho, start_chirho)
    }

    fn lex_operator_chirho(&mut self, start_chirho: usize) -> RawTokenChirho {
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

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Whether a byte is a valid Haskell symbol character.
fn is_symbol_char_chirho(byte_chirho: u8) -> bool {
    matches!(
        byte_chirho,
        b'!' | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'*'
            | b'+'
            | b'.'
            | b'/'
            | b'<'
            | b'='
            | b'>'
            | b'?'
            | b'@'
            | b'\\'
            | b'^'
            | b'|'
            | b'-'
            | b'~'
            | b':'
    )
}

/// Whether a byte can appear in an identifier (after the first character).
fn is_ident_char_chirho(byte_chirho: u8) -> bool {
    byte_chirho.is_ascii_alphanumeric()
        || byte_chirho == b'_'
        || byte_chirho == b'\''
        || byte_chirho > 0x7F // Non-ASCII bytes (part of multi-byte UTF-8 chars)
}

/// Look up a keyword kind from text, returning `None` for identifiers.
fn keyword_kind_chirho(text_chirho: &str) -> Option<RawTokenKindChirho> {
    match text_chirho {
        "case" => Some(RawTokenKindChirho::CaseChirho),
        "class" => Some(RawTokenKindChirho::ClassChirho),
        "data" => Some(RawTokenKindChirho::DataChirho),
        "default" => Some(RawTokenKindChirho::DefaultChirho),
        "deriving" => Some(RawTokenKindChirho::DerivingChirho),
        "do" => Some(RawTokenKindChirho::DoChirho),
        "else" => Some(RawTokenKindChirho::ElseChirho),
        "foreign" => Some(RawTokenKindChirho::ForeignChirho),
        "if" => Some(RawTokenKindChirho::IfChirho),
        "import" => Some(RawTokenKindChirho::ImportChirho),
        "in" => Some(RawTokenKindChirho::InChirho),
        "infix" => Some(RawTokenKindChirho::InfixChirho),
        "infixl" => Some(RawTokenKindChirho::InfixlChirho),
        "infixr" => Some(RawTokenKindChirho::InfixrChirho),
        "instance" => Some(RawTokenKindChirho::InstanceChirho),
        "let" => Some(RawTokenKindChirho::LetChirho),
        "module" => Some(RawTokenKindChirho::ModuleChirho),
        "newtype" => Some(RawTokenKindChirho::NewtypeChirho),
        "of" => Some(RawTokenKindChirho::OfChirho),
        "then" => Some(RawTokenKindChirho::ThenChirho),
        "type" => Some(RawTokenKindChirho::TypeChirho),
        "where" => Some(RawTokenKindChirho::WhereChirho),
        "forall" => Some(RawTokenKindChirho::ForallChirho),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn lex_chirho(source_chirho: &str) -> Vec<RawTokenChirho> {
        let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
        lexer_chirho.lex_all_chirho()
    }

    fn non_trivia_kinds_chirho(source_chirho: &str) -> Vec<RawTokenKindChirho> {
        lex_chirho(source_chirho)
            .into_iter()
            .filter(|t_chirho| !t_chirho.kind_chirho.is_trivia_chirho())
            .map(|t_chirho| t_chirho.kind_chirho)
            .collect()
    }

    #[test]
    fn lex_module_header_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("module Main where");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ModuleChirho,
                RawTokenKindChirho::ConIdChirho,
                RawTokenKindChirho::WhereChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_integer_literals_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("42 0xFF 0o77 0b1010");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_float_literals_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("3.14 1.0e10 2.5E-3");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::FloatLitChirho,
                RawTokenKindChirho::FloatLitChirho,
                RawTokenKindChirho::FloatLitChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_numeric_underscores_chirho() {
        // NumericUnderscores: underscores in decimal, hex, octal, binary, float
        let kinds_chirho = non_trivia_kinds_chirho("1_000_000 0xFF_FF 0o7_7 0b10_10 3.14_15");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::IntLitChirho,
                RawTokenKindChirho::FloatLitChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_string_with_gap_chirho() {
        let tokens_chirho = lex_chirho("\"hello \\    \\world\"");
        let string_tokens_chirho: Vec<_> = tokens_chirho
            .iter()
            .filter(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::StringLitChirho)
            .collect();
        assert_eq!(string_tokens_chirho.len(), 1);

        let text_chirho = &"\"hello \\    \\world\""
            [string_tokens_chirho[0].span_chirho.start_chirho().as_usize_chirho()
                ..string_tokens_chirho[0].span_chirho.end_chirho().as_usize_chirho()];
        assert_eq!(text_chirho, "\"hello \\    \\world\"");
    }

    #[test]
    fn lex_nested_block_comment_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("{- outer {- inner -} still outer -} x");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_operators_and_special_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho(":: -> <- => .. = | \\ @ ~");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ColonColonChirho,
                RawTokenKindChirho::RightArrowChirho,
                RawTokenKindChirho::LeftArrowChirho,
                RawTokenKindChirho::FatArrowChirho,
                RawTokenKindChirho::DotDotChirho,
                RawTokenKindChirho::EqualsChirho,
                RawTokenKindChirho::PipeChirho,
                RawTokenKindChirho::BackslashChirho,
                RawTokenKindChirho::AtChirho,
                RawTokenKindChirho::TildeChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_keywords_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("if then else case of let in do where");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::IfChirho,
                RawTokenKindChirho::ThenChirho,
                RawTokenKindChirho::ElseChirho,
                RawTokenKindChirho::CaseChirho,
                RawTokenKindChirho::OfChirho,
                RawTokenKindChirho::LetChirho,
                RawTokenKindChirho::InChirho,
                RawTokenKindChirho::DoChirho,
                RawTokenKindChirho::WhereChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_char_literals_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("'a' '\\n' '\\\\' '0'");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::CharLitChirho,
                RawTokenKindChirho::CharLitChirho,
                RawTokenKindChirho::CharLitChirho,
                RawTokenKindChirho::CharLitChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_constructor_operator_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho(":+: :*:");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ConSymChirho,
                RawTokenKindChirho::ConSymChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_punctuation_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("( ) [ ] { } , ;");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::LeftParenChirho,
                RawTokenKindChirho::RightParenChirho,
                RawTokenKindChirho::LeftBracketChirho,
                RawTokenKindChirho::RightBracketChirho,
                RawTokenKindChirho::LeftBraceChirho,
                RawTokenKindChirho::RightBraceChirho,
                RawTokenKindChirho::CommaChirho,
                RawTokenKindChirho::SemicolonChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_doc_comment_chirho() {
        let tokens_chirho = lex_chirho("-- | doc comment\n-- regular comment");
        let doc_count_chirho = tokens_chirho
            .iter()
            .filter(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::DocCommentChirho)
            .count();
        let line_count_chirho = tokens_chirho
            .iter()
            .filter(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::LineCommentChirho)
            .count();
        assert_eq!(doc_count_chirho, 1);
        assert_eq!(line_count_chirho, 1);
    }

    #[test]
    fn lex_wildcard_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("_ _x");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::UnderscoreChirho,
                RawTokenKindChirho::VarIdChirho, // _x is an identifier, not wildcard
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_type_signature_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("main :: IO ()");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::ColonColonChirho,
                RawTokenKindChirho::ConIdChirho,
                RawTokenKindChirho::LeftParenChirho,
                RawTokenKindChirho::RightParenChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    // ── DataKinds lexer tests ──────────────────────────────────────
    #[test]
    fn lex_promoted_constructor_chirho() {
        // 'True should be Tick + ConId, not a char literal
        let kinds_chirho = non_trivia_kinds_chirho("'True");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::TickChirho,
                RawTokenKindChirho::ConIdChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_char_vs_promoted_chirho() {
        // 'A' is a char literal (uppercase but followed by closing ')
        let kinds_chirho = non_trivia_kinds_chirho("'A'");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::CharLitChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_promoted_list_chirho() {
        // '[Int, Bool] should start with Tick + [
        let kinds_chirho = non_trivia_kinds_chirho("'[");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::TickChirho,
                RawTokenKindChirho::LeftBracketChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_promoted_nothing_chirho() {
        // 'Nothing is a promoted constructor
        let kinds_chirho = non_trivia_kinds_chirho("'Nothing");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::TickChirho,
                RawTokenKindChirho::ConIdChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn spans_are_correct_chirho() {
        let source_chirho = "module Main where";
        let tokens_chirho = lex_chirho(source_chirho);
        let module_token_chirho = &tokens_chirho[0];
        assert_eq!(module_token_chirho.kind_chirho, RawTokenKindChirho::ModuleChirho);
        let text_chirho = module_token_chirho
            .span_chirho
            .text_chirho(source_chirho)
            .unwrap();
        assert_eq!(text_chirho, "module");
    }

    #[test]
    fn lex_th_splice_chirho() {
        // $foo should be ThSplice + VarId
        let kinds_chirho = non_trivia_kinds_chirho("$foo");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ThSpliceChirho,
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_th_splice_parens_chirho() {
        // $(expr) should be ThSplice + ( ...
        let kinds_chirho = non_trivia_kinds_chirho("$(foo)");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ThSpliceChirho,
                RawTokenKindChirho::LeftParenChirho,
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::RightParenChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_dollar_operator_chirho() {
        // f $ x — dollar is an operator, not a splice
        let kinds_chirho = non_trivia_kinds_chirho("f $ x");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::VarSymChirho,
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_th_typed_splice_chirho() {
        // $$foo should be ThTypedSplice + VarId
        let kinds_chirho = non_trivia_kinds_chirho("$$foo");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ThTypedSpliceChirho,
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_th_expression_quote_chirho() {
        // [| expr |] should produce open/close quote tokens
        let kinds_chirho = non_trivia_kinds_chirho("[| foo |]");
        assert_eq!(
            kinds_chirho,
            vec![
                RawTokenKindChirho::ThOpenExpQuoteChirho,
                RawTokenKindChirho::VarIdChirho,
                RawTokenKindChirho::ThCloseQuoteChirho,
                RawTokenKindChirho::EofChirho,
            ]
        );
    }

    #[test]
    fn lex_th_dec_quote_chirho() {
        // [d| ... |]
        let kinds_chirho = non_trivia_kinds_chirho("[d| x = 1 |]");
        assert_eq!(kinds_chirho[0], RawTokenKindChirho::ThOpenDecQuoteChirho);
        assert_eq!(kinds_chirho[kinds_chirho.len() - 2], RawTokenKindChirho::ThCloseQuoteChirho);
    }

    #[test]
    fn lex_th_type_quote_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("[t| Int -> Bool |]");
        assert_eq!(kinds_chirho[0], RawTokenKindChirho::ThOpenTypeQuoteChirho);
    }

    #[test]
    fn lex_th_pat_quote_chirho() {
        let kinds_chirho = non_trivia_kinds_chirho("[p| (x, y) |]");
        assert_eq!(kinds_chirho[0], RawTokenKindChirho::ThOpenPatQuoteChirho);
    }

    #[test]
    fn lex_normal_list_unaffected_chirho() {
        // [1, 2, 3] should not be affected by TH quote lexing
        let kinds_chirho = non_trivia_kinds_chirho("[1, 2, 3]");
        assert_eq!(kinds_chirho[0], RawTokenKindChirho::LeftBracketChirho);
    }
}
