<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Literal escapes as GHC lexes them Chirho

Row 561 (gpt_chirho #29012, #29096), root 4 of row 557's trace. Character and string literals
decoded their escapes in two places that had drifted apart:
- `parse_haskell_char_body_chirho` in the AST crate;
- `unescape_string_chirho` in the parser, with its own copy of the ASCII-name table.

Against GHC, 7 of 10 measured lines differed:
- `'\''` decoded to U+FFFD, because the token's quotes were trimmed with `trim_matches`;
- `\&` was kept;
- `\0` before digits kept the digits;
- caret controls (`\^A`) were not decoded;
- names were not matched by their longest prefix (`\NULL`, `\SO1`).

## Contract

- One decoder, the report's section 2.6, serves both literal kinds; the parser's copy is
  removed. The empty escape `\&` and gaps are string-only.
- A character literal strips exactly one quote at each end.
- All three engines show GHC's bytes for the measured control; GHC's own lexing of 23 string
  and 19 character bodies is the table test.
- Holds:
  - surrogate code points decode to None, as before;
  - natively, `\NUL` in a string literal ends it (row 560);
  - an invalid escape keeps its old handling in a string, and gives U+FFFD in a character
    literal, where GHC rejects both at lex time.

## Checklist

- [x] Measure GHC on every affected form, and today's decoding beside it (7 of 10 lines differ).
- [x] Add the shared grammar in `lit_chirho/escape_chirho.rs`; the character decoder delegates to
  it; the parser's duplicate decoder and its four helpers (164 lines) are gone.
- [x] Fix the character token's quote trim.
- [x] Generate the table test from runghc's output: ast 18/18, parser 395/395.
- [x] All-engine control generated from GHC's output; byte-identical on all three engines.
- [ ] Rebase, corpus attribution, battery transitions, reach mutations, the focused gate, and post.
