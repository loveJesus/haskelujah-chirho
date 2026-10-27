<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Char and String show as GHC spells it Chirho

Row 557 (gpt_chirho #28974, #29012) gives `show` for `Char` and `String` GHC's `showLitChar`
spelling on the interpreter, LLVM and Cranelift. One renderer in the runtime crate serves every
engine. It is renderer-only. The trace (#29005) found four roots; this row owns root 1.

## Roots

1. **Renderer (this row):** seven sites quoted without escaping. Natively the Char renderer was
   Rust's `{:?}`, right only by coincidence for `\n`, `\\` and `\'`.
2. **Native `++#` (row 560):** the RTS append reads a list cell as a NUL-terminated buffer, so a
   runtime-built String reads as `""`.
3. **Literal tails and unpacked bytes (row 560):** a cons onto a literal stops at the literal
   natively and in the interpreter's `++#`; unpack makes one cell per UTF-8 byte.
4. **Literal decoding (row 561):** `'\''` decodes to U+FFFD; `\&` is kept as two characters;
   `\0` before digits keeps the digits.

## Contract

- Every engine renders through `show_char_chirho` / `show_string_chirho`.
- The `[Char]` Show row is `showStr# s`; both backends lower `showStr#` to
  `haskelujah_show_string_chirho`. The renderer reads every character, so a runtime-built String
  shows exactly natively (m04, m16-m18 are this row's gains, #29012).
- Each engine reads the String exactly or fails named:
  - natively, a literal tail refuses with `show: no exact native reading of this String` (row 560
    owns it); a cell above ASCII reads exactly once row 560's unpack carries code points;
  - in the interpreter, a literal tail reads exactly (m05 is a gain), and a cell holding no
    character fails `primop ShowStrChirho: expected Char`.
- Holds: `putStrLn`, literals, native `++#` (m01), and every native String outside `show`.

## Landing condition

#29012: a changed wrong answer is RED. A baseline build of main af9dcac6 against the candidate,
100 programs on all three engines, gives:
- 68 gains;
- 13 wrong answers that now fail named;
- 0 regressions from exact;
- 14 RED cells, from two roots that other rows own.

The RED cells:
- **Root 4, 10 cells: m08, m13 and m14 on all three engines, and s29 in the interpreter.** Escaping
  now shows the mis-decoded value (`'\65533'`, `"\1234\\&5"`). **Row 561 must land first.**
- **Native unpack, 4 cells: n22 and n23 on both natives.** They moved from silently wrong to a
  SIGBUS (rc 138), with no named report.
  - `haskelujah_unpack_string_chirho` links each new cell through an unrooted local, so a
    collection in mid-unpack frees the part already built.
  - It is pre-existing and needs no show: on main, natively, `length "<2000-character literal>"`
    prints 15.
  - Row 557 makes it reachable for strings built at run time: `length (show s)` now unpacks a real
    string, where it used to unpack `""`.
  - At length 1000 the answer changes from 2 to 20. **This fix must land first too.**

Moved and attributed:
- s21 is exact here since row 563 fixed `lines`.
- n15 (`toEnum` at Char gives an Int in the interpreter; row 520 owns Char `Enum`) now fails named
  instead of printing another wrong answer.

## Checklist

- [x] Trace the three wrong-answer classes to four roots, measured on GHC and source-cited (#29005).
- [x] Add the shared renderer, checked against GHC's own table: 170 Chars, 24 Strings, code points 0-130.
- [x] Native: the Char entry uses the renderer. `haskelujah_show_string_chirho` reads a buffer as
  UTF-8 and walks a list strictly with the cell reader that maps already used, now shared as
  `native_lists_chirho.rs`.
- [x] LLVM and Cranelift lower `showStr#` to it (Cranelift as a required runtime import).
- [x] Interpreter: five quoting sites use the renderer; a strict heap `[Char]` reader for `showStr#`.
- [x] Core: the `[Char]` row becomes `showStr#`.
- [x] Three-engine controls with GHC 9.14.1's output, and the native and interpreter refusals.
- [x] Battery on main 241a62e8, then 100 programs on af9dcac6, a baseline build against the
  candidate each time; every moved cell is attributed above.
- [x] Row 535's 41 contract programs on all three engines: 123 of 123 cells hold.
- [x] Reach mutations: 5 of 5 killed by named assertions, every file restored byte-identical.
- [x] Gate r1 GREEN in 145 s on 9ccba4cd:
  - RTS 61, runtime 57, core 170, LLVM 38, Cranelift 42;
  - seven driver suites 195, and the driver library's show/print/string/char filter 173;
  - Clippy: none of 627 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
- [x] Row 566 landed (unpack and the collector). Replayed as one stack on current main with row
  560's minimal brick and row 561 (#29139), since each alone changes cells another fixes. The
  above-ASCII refusal is dropped; its control is now all-engine exact.
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
