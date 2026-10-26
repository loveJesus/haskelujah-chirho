<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Interpreter Show Float Specials Chirho

Row 546 owns the interpreter rendering of non-finite `Double` values through
`ShowFloatChirho`.

## Contract and boundary

- GHC 9.14.1 prints `NaN`, `Infinity`, and `-Infinity` for `0 / 0`, `1 / 0`, and
  `(-1) / 0` at `Double`.
- The interpreter currently appends `.0` to Rust's non-finite spellings, producing
  `NaN.0`, `inf.0`, and `-inf.0`.
- `ShowFloatChirho` will spell those three values exactly as GHC does before the
  existing finite-number rendering path.
- Ordinary finite values keep their current output, including `0.0`, `-0.0`, and
  `1.5`.
- Native infinity output remains outside this interpreter row: LLVM and Cranelift
  currently print `inf` and `-inf`. The existing native `NaN` control stays exact.
- GHC-style exponent selection for large and small finite values remains a separate
  unit: this row does not change the current fixed-decimal output for `1.0e20` or
  `1.0e-20`.

## Checklist

- [x] Reproduce and bound the three non-finite mismatches.
- [x] Add a unit-level rendering control for all three non-finite values and ordinary
      finite regressions.
- [x] Turn the landed row 543 `NaN` native-only control into an all-engine control and
      add both infinities as interpreter controls.
- [x] Record the runtime rendering decision in the Show workflow.
- [x] Run a warm focused gate within the 2–5 minute bar.
- [x] Rebase onto current `main_chirho`, land, and close progress row 546 GREEN.

## Source checkpoint

- `show_float_chirho` handles the three non-finite spellings before the existing finite
  path. Its unit control also pins signed zero, an ordinary fraction, and the current
  fixed-decimal large and small finite boundaries.
- GHC 9.14.1 prints the three expected spellings from the committed expressions.
- The end-to-end control measures each division separately because a combined three-call
  program exposes a separate occurrence-evidence defect and still loses `/` at runtime.
- Native `NaN` remains exact. LLVM currently prints positive infinity as `inf`, so native
  infinity spelling is recorded outside this interpreter-only row rather than hidden.
- Warm focused proof: runtime library 57/57 and the three row 543/546 fractional-literal
  tests 3/3. The complete focused sequence, including both Clippy invocations, took 21.1
  seconds. Clippy reports no diagnostic in either changed Rust file; its warnings are on
  pre-existing lines elsewhere.
- Landed by fast-forward on exact main at `ffb8466b`; progress row 546 is GREEN.
