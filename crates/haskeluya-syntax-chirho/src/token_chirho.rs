// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Token kinds for Haskeluya's future Haskell lexer.
//!
//! The baseline coverage follows Chapter 2 of the Haskell 2010 Report:
//! lexical program structure, comments, identifiers/operators, literals, and
//! layout insertion. Haskeluya additionally splits out a few parser-facing
//! Haskell 2010 import keywords (`qualified`, `as`, and `hiding`) even though
//! they are grammar-level words rather than Chapter 2 reserved identifiers.
//! Doc comments are also represented explicitly as preserved trivia so the
//! lexer can remain lossless for tooling and documentation extraction.

/// Every token kind needed by a lossless Haskell 2010 lexer front-end.
///
/// The enum distinguishes:
///
/// - lexical name classes (`qvarid`, `qconid`, `qvarsym`, `qconsym`)
/// - reserved identifiers and parser-facing keywords
/// - literal categories from Chapter 2
/// - punctuation and reserved operators
/// - virtual layout tokens inserted per the layout rule
/// - preserved trivia needed for a lossless token stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKindChirho {
    /// An unqualified variable identifier (`varid`) that is not reserved.
    VarIdChirho,
    /// An unqualified constructor identifier (`conid`).
    ConIdChirho,
    /// A qualified variable identifier (`qvarid`).
    QualifiedVarIdChirho,
    /// A qualified constructor identifier (`qconid`).
    QualifiedConIdChirho,
    /// An unqualified variable operator symbol (`varsym`) that is not reserved.
    VarSymChirho,
    /// An unqualified constructor operator symbol (`consym`) that is not reserved.
    ConSymChirho,
    /// A qualified variable operator symbol (`qvarsym`).
    QualifiedVarSymChirho,
    /// A qualified constructor operator symbol (`qconsym`).
    QualifiedConSymChirho,

    /// The `module` reserved identifier.
    ModuleKeywordChirho,
    /// The `where` reserved identifier.
    WhereKeywordChirho,
    /// The `let` reserved identifier.
    LetKeywordChirho,
    /// The `in` reserved identifier.
    InKeywordChirho,
    /// The `do` reserved identifier.
    DoKeywordChirho,
    /// The `case` reserved identifier.
    CaseKeywordChirho,
    /// The `of` reserved identifier.
    OfKeywordChirho,
    /// The `if` reserved identifier.
    IfKeywordChirho,
    /// The `then` reserved identifier.
    ThenKeywordChirho,
    /// The `else` reserved identifier.
    ElseKeywordChirho,
    /// The `class` reserved identifier.
    ClassKeywordChirho,
    /// The `instance` reserved identifier.
    InstanceKeywordChirho,
    /// The `data` reserved identifier.
    DataKeywordChirho,
    /// The `type` reserved identifier.
    TypeKeywordChirho,
    /// The `newtype` reserved identifier.
    NewtypeKeywordChirho,
    /// The `deriving` reserved identifier.
    DerivingKeywordChirho,
    /// The `import` reserved identifier.
    ImportKeywordChirho,
    /// The `qualified` Haskell 2010 import keyword.
    QualifiedKeywordChirho,
    /// The `as` Haskell 2010 import keyword.
    AsKeywordChirho,
    /// The `hiding` Haskell 2010 import keyword.
    HidingKeywordChirho,
    /// The `foreign` reserved identifier.
    ForeignKeywordChirho,
    /// The `default` reserved identifier.
    DefaultKeywordChirho,
    /// The `infix` reserved identifier.
    InfixKeywordChirho,
    /// The `infixl` reserved identifier.
    InfixlKeywordChirho,
    /// The `infixr` reserved identifier.
    InfixrKeywordChirho,
    /// The `forall` keyword (ExplicitForAll/RankNTypes/ScopedTypeVariables).
    ForallKeywordChirho,
    /// The standalone `_` reserved identifier used as a wildcard.
    UnderscoreReservedIdChirho,

    /// An integer literal, including decimal, octal, and hexadecimal forms.
    IntegerLiteralChirho,
    /// A floating-point literal in decimal form.
    FloatLiteralChirho,
    /// A character literal.
    CharLiteralChirho,
    /// A string literal, including escaped characters and string gaps.
    StringLiteralChirho,

    /// The `(` punctuation token.
    LeftParenChirho,
    /// The `)` punctuation token.
    RightParenChirho,
    /// The `,` punctuation token.
    CommaChirho,
    /// The explicit `;` punctuation token.
    SemicolonChirho,
    /// The `[` punctuation token.
    LeftBracketChirho,
    /// The `]` punctuation token.
    RightBracketChirho,
    /// The `` ` `` punctuation token for infix identifier use.
    BacktickChirho,
    /// The explicit `{` punctuation token.
    LeftBraceChirho,
    /// The explicit `}` punctuation token.
    RightBraceChirho,

    /// The `..` reserved operator.
    DotDotChirho,
    /// The `:` reserved operator.
    ColonChirho,
    /// The `::` reserved operator.
    DoubleColonChirho,
    /// The `=` reserved operator.
    EqualsChirho,
    /// The `\` reserved operator.
    BackslashChirho,
    /// The `|` reserved operator.
    PipeChirho,
    /// The `<-` reserved operator.
    LeftArrowChirho,
    /// The `->` reserved operator.
    RightArrowChirho,
    /// The `@` reserved operator.
    AtSignChirho,
    /// The `~` reserved operator.
    TildeChirho,
    /// The `=>` reserved operator.
    DoubleArrowChirho,
    /// The `'` tick for DataKinds promoted constructors/types.
    TickChirho,

    /// A virtual `{` inserted by the layout rule.
    VirtualLeftBraceChirho,
    /// A virtual `}` inserted by the layout rule.
    VirtualRightBraceChirho,
    /// A virtual `;` inserted by the layout rule.
    VirtualSemicolonChirho,

    /// Whitespace preserved as trivia.
    WhitespaceTriviaChirho,
    /// A `--` line comment preserved as trivia.
    LineCommentTriviaChirho,
    /// A `{- -}` nested block comment preserved as trivia.
    BlockCommentTriviaChirho,
    /// A documentation comment preserved as trivia for lossless tooling.
    DocCommentTriviaChirho,
    /// A `{-# ... #-}` pragma (LANGUAGE, OPTIONS, etc.)
    PragmaChirho,

    // -- Template Haskell tokens --
    /// The `$` splice operator (followed by identifier or parenthesized expr).
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
    /// Opening `[e|` for expression quotation (explicit form).
    ThOpenExpExplicitQuoteChirho,
    /// Opening `[||` for typed expression quotation.
    ThOpenTypedExpQuoteChirho,
    /// Closing `||]` for typed expression quotation.
    ThCloseTypedQuoteChirho,
}
