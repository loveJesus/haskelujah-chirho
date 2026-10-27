<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native Strings carry code points Chirho

The minimal row 560 brick (gpt_chirho #29139): a bounded representation correction, not the rest
of row 560. It unblocks the paired 561+557 stack. Six native cells there changed from one silent
wrong answer to another, because a native literal could not hold the decoded value:
- NUL truncated the C-string literal;
- unpack made one cell per UTF-8 byte.

## Contract

- Unpack decodes UTF-8 to one cell per code point; pack encodes code points as UTF-8.
- An evaluated native literal holding NUL fails by a stable named line; so does packing a NUL.
- Invalid UTF-8 and an unencodable cell fail by their own named lines.
- The interpreter is unchanged.

## Checklist

- [x] Measure GHC on the controls: code points through unpack, reverse, `toEnum`, `succ` and
  comparison; a NUL literal; a packed NUL.
- [x] RTS: UTF-8 unpack and pack, and the three named failures in `failure_chirho.rs`.
- [x] LLVM and Cranelift: an evaluated literal holding NUL calls
  `haskelujah_nul_string_literal_chirho` (Cranelift through a required runtime import).
- [x] Controls: the RTS round trip; all-engine code points; NUL failing named on both natives while
  the interpreter keeps it. RTS 61/61, native_error_helpers 40/40.
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
