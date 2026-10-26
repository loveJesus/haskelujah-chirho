<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Report Prelude Folds Chirho

Row 548 supplies the Haskell report's missing `scanl1`, `foldl1`, and `until`
Core bindings. The checker already typed the names, but all three execution engines
previously stopped on a missing binding.

## Contract and boundary

- Contract rows U01, U02, U03, and U05 must print GHC 9.14.1's exact answers in the
  interpreter, LLVM, and Cranelift.
- An empty `foldl1` fails with `Prelude.foldl1: empty list` on every engine.
- The definitions reuse the existing generated `scanl` and `foldl` bindings rather
  than duplicating their list recursion.
- U04 `realToFrac` remains a named missing binding because its Real and Fractional
  dictionaries are a separate unit.
- Native whole-`Double` rendering still omits `.0`; row 548 records but does not hide
  that separate formatting residual.

## Checklist

- [x] Generate `scanl1`, `foldl1`, and recursive `until` in a bounded Prelude child
      module.
- [x] Keep `realToFrac` as the fail-closed unbound-name control.
- [x] Measure ordinary, constant, left-associative, immediate-stop, and empty-list
      behavior against GHC.
- [x] Run the original 27-second warm focused gate.
- [x] Re-run core 168/168 and native error helpers 32/32 on the combined row 546/548
      main tree.
- [x] Land as `dc5b383f` and close progress row 548 GREEN.

## Landing record

Claude2 prepared and gated the source at `bfb0c1cb` on `e9192768`. It was cherry-picked
without conflict after row 546 as `dc5b383f`; the exact-current-main verification also
kept row 546 runtime 57/57 and fractional-literal integration 3/3. Diff-check and
gitleaks are clean.
