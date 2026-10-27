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
- [x] Corpus: none of the 1,705 typecheck files uses an affected form (the selector proven on a
  probe file); only 4 character literals there carry any escape, all valid.
- [x] Reach mutations: 5 of 5 killed by named assertions, every file restored byte-identical.
- [x] Landed as one stack with rows 560 (minimal), 561 and 557 (#29139): each alone changed cells
  another fixes. A fresh build of main a593ce12 against the stack on all three engines:
  - 115 one-line programs: 116 gains, 10 named failures, 0 RED, 0 regressions from exact;
  - 47 programs (the controls, escapes, the long literal, row 535's 41 contract programs):
    6 gains, 6 named failures, 0 RED; all 123 contract cells hold;
  - every failing cell is named: the NUL line or the literal-tail refusal; none by signal.
- [x] Stack gate GREEN in 287 s:
  - RTS 64, runtime 58, core 171, ast 18, typing 362, parser 395, LLVM 38, Cranelift 42;
  - nine driver suites 225, and the driver library's show/print/string/char/literal/escape filter 212;
  - Clippy: none of 732 warnings on a changed line; rustfmt, diff-check and gitleaks clean.

## Landing record

The four-commit stack replayed onto row 570 at `22497167`. Its only content conflict was the
expected union of the independent named-failure constants in `failure_chirho.rs`. The first
default-stack run stopped abnormally in the pre-existing long-literal control; that same control
passed with the project's recorded `RUST_MIN_STACK=16777216`. The complete overlap gate then
passed in 139.63 seconds: RTS 65/65, runtime 58/58, LLVM 38/38, Cranelift 42/42,
native-error helpers 45/45, and dictionary evidence 103/103. Gitleaks found no leak in the replayed
four-commit range. The stack landed at `2abce64b`; progress closes in the following commit.
