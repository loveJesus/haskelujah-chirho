<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 572: Maybe and tuple Ord compare rows

Base: `a593ce12`, source main `356da5ea` plus the row 572 progress reservation only.

- [x] Reserve row 572 and announce the disjoint source lane.
- [x] Measure GHC/interpreter/LLVM/Cranelift behavior for bounded Maybe and tuple comparisons.
- [x] Trace how structured Eq/Show rows are generated and where Ord.compare stays a missing row.
- [x] Implement body-backed compare rows only where every component comparison is available.
- [x] Add all-engine controls, named residuals and a mechanism mutation.
- [x] Update the dictionary-evidence workflow.
- [x] Run the warm focused gate, changed-line Clippy, rustfmt and diff-check. Run gitleaks on the committed range before landing.
- [ ] Rebase onto exact main, rerun the controls, land, and close only row 572.

Scope starts with `Maybe` and 2-tuples, the two concrete keys named as residuals by rows 551 and
565. Tuple arity and nested structured keys will expand only where the existing instance evidence
and a bounded body generator already establish every component comparison. Row 570 Double power,
Rows 560/561/557 String representation and rendering, the Ord class seed, prefix `(<)`, and
polymorphic dictionary wrappers are outside this row.

## Measured transition

GHC 9.14.1 prints `GT` for `compare (Just 2) (Just 1)`, `GT` for
`compare (1,3) (1,2)`, and chooses `Just 3` and `(2,'a')` through `maximum` and
`minimum`. At the base, direct `compare` failed by name on all three engines; infix ordering
crashed the interpreter and silently compared native heap words; extrema were partly wrong.
The source control now gives GHC's six-line answer on the interpreter, LLVM and Cranelift. It
also covers a triple, `Maybe [Int]`, and `(String,[Int])`, proving recursive composition from
existing rows rather than a finite table of special cases.

The checker originally published only `Maybe` and `(,)` at method occurrences. Row 572 retains
the complete concrete `Ord` key, as `Show` already did. Direct infix operators no longer carry a
method occurrence after desugaring, so a read-only Core prepass collects only the concrete
operand keys the landed compare route itself will accept; generation still refuses any missing
or stub component body.

## Gate record

The warm focused gate completed in 66.43 seconds: `haskelujah` built without compiler warnings,
core passed 174/174, typing passed 363/363, and dictionary evidence passed 101/101. Clippy
completed for the core, typing and driver library/test targets with no diagnostic on a changed
line; rustfmt and `git diff --check` were clean. Removing only the structured-row generation hook
made the all-engine control fail at its first interpreter observation with `missing STG binding
compare for CoreId 3`; restoring the hook byte for byte restored the exact result.
