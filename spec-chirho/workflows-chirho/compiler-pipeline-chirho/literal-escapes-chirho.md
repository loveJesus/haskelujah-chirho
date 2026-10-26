<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Workflow: literal escapes

Row 561 (gpt_chirho #29012, #29096). Character and string literals decode their escapes through
one grammar, the Haskell 2010 report's section 2.6, in `haskelujah-ast-chirho`'s
`lit_chirho/escape_chirho.rs`. Every literal, in an expression or a pattern, is lowered by the
parser's `lower_lit_chirho`.

```mermaid
flowchart TD
    token_le_chirho[literal token text] --> kind_le_chirho{kind}
    kind_le_chirho -->|character| quote_le_chirho["strip exactly one quote at each end: '\\'' keeps its escaped quote"]
    quote_le_chirho --> char_le_chirho["parse_haskell_char_body_chirho: one escape and nothing after it, or None"]
    kind_le_chirho -->|string| string_le_chirho["unescape_string_body_chirho"]
    string_le_chirho --> amp_le_chirho["\& (the empty escape): nothing"]
    string_le_chirho --> gap_le_chirho["backslash, whitespace, backslash (a gap): nothing"]
    string_le_chirho --> escape_le_chirho
    char_le_chirho --> escape_le_chirho["decode_escape_chirho"]
    escape_le_chirho --> names_le_chirho["ASCII names, the longest first: \SOH, not \SO then H; \NUL then L"]
    escape_le_chirho --> letters_le_chirho["\a \b \f \n \r \t \v \\ \" \'"]
    escape_le_chirho --> caret_le_chirho["\^@ through \^_ : 0 through 31"]
    escape_le_chirho --> numbers_le_chirho["decimal, \o octal, \x hex: every digit that follows (\012 is 12)"]
```

- **Strings only:** `\&` and gaps. A character literal accepts exactly one escape.
- **Invalid escapes:**
  - In a string, an escape no rule matches is kept as written; GHC rejects it at lex time.
  - In a character literal, an escape no rule matches gives None, and the parser falls back to U+FFFD.
  - Neither is a GHC-exact claim.
- **Boundaries:**
  - a surrogate code point (`'\55296'`) has no Rust `char`, so it decodes to None, as before;
  - natively, a string literal is a NUL-terminated buffer, so a `\NUL` inside one ends it (row 560).
- **Tests:**
  - GHC's own lexing of 23 string bodies and 19 character bodies: `lit_chirho/escape_tests_chirho.rs`;
  - all three engines: `literal_escapes_decode_as_ghc_lexes_them_everywhere_chirho`.
