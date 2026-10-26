<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Interpreter `lines` trailing-newline parity Chirho

Row 563 owns the interpreter-only extra empty element produced by `lines` when its input ends in
a newline. It does not change the native String representation, String rendering, literal decoding,
or prefix-minus lowering owned by rows 557, 560, 561, and 555.

## Contract

- Measure GHC 9.14.1 and the current interpreter on empty input, no terminator, one and repeated
  terminators, leading and interior empty lines, and carriage returns.
- Split only on `\n`; preserve a preceding `\r` as a normal character, as Haskell's `lines` does.
- A final `\n` terminates the last line and does not synthesize one additional empty line.
- Preserve `unlines` behavior and measure the useful `lines . unlines` boundary.
- Add the narrowest runtime control and keep the change inside the `LinesChirho` primitive.

## Checklist

- [x] Record the GHC/interpreter matrix before source.
- [x] Repair the one primitive without changing String representation.
- [x] Add focused regression and boundary controls.
- [x] Run the runtime and focused driver gates under three minutes.
- [x] Verify rustfmt, diff-check, changed-line Clippy, and gitleaks.
- [x] Land and close progress row 563.

## Gate record

- `cargo test -p haskelujah-runtime --lib`: 57 passed, 0 failed.
- `cargo test -p haskelujah-driver --test evaluation_values_chirho lines_chirho`: 2 passed,
  0 failed. The tests cover the complete measured newline matrix and `lines . unlines` boundary.
- Focused Clippy completed in 8.72 seconds. It reported no warning on a changed line; the existing
  workspace warnings remain outside this unit.
- `cargo fmt --all -- --check` and `git diff --check` are clean.
- Gitleaks scanned source commit `f936ffa9` and found no leaks.
- Source commit `f936ffa9` fast-forwarded onto exact base `539379d4`.
