<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 571: negated top-level Double bindings

Base: `49ef0320`, whose only change beyond source main `0d516dd3` is the progress reservations for rows 570 and 571.

- [x] Reserve row 571 and announce the disjoint source lane.
- [x] Measure the bounded GHC/interpreter/LLVM/Cranelift matrix before editing source.
- [x] Trace where a typed top-level Double binding loses its key at `negate#`.
- [x] Implement the smallest shared evidence repair without touching row 570's Double-power route.
- [x] Add GHC-measured all-engine controls and kill the module-key omission mutation.
- [x] Update the relevant workflow documentation.
- [ ] Run the warm focused gate, changed-line Clippy, rustfmt, diff-check and gitleaks.
- [ ] Rebase onto exact main, rerun the controls, land, and close only row 571.

Scope is the 7b residual recorded after row 555: a negation whose operand is a top-level binding
with a concrete `Double` signature. Direct Double literals, negation rebuilt under an already
known type key, Double power, parser precedence, list/String representation and generic
polymorphic negation are outside this row unless measurement proves the same concrete root.

## Measurement and root

GHC 9.14.1 prints `-1.5` for `xChirho :: Double; xChirho = 1.5; print (-xChirho)`.
Before this row the interpreter stopped at `NegIntChirho`, while LLVM and Cranelift printed the
silent wrong answer `-3.0`. A computed top-level `3 / 2` had the same transition; `-x + 1`
printed `-2.0` natively instead of GHC's `-0.5`. Typed parameters and locals, a binding whose
definition is already negated, named `negate`, and an Int binding were exact before editing.

The type environment retained the concrete `Double` scheme, but the top-level Core binder still
carried `t0`. The dictionary pass seeded type keys for typed lambda/local binders and inferred
module call arguments only. It now adds concrete, monomorphic, unconstrained, zero-argument
module values to that same key map. Row 547's existing `negate#` to `negateFloat#` table then
performs the only operation rewrite.

The mechanism mutation restored the old call-argument-only module map. The Core control failed
with `negate#` still present, and the all-engine control failed immediately at the interpreter's
`NegIntChirho`. Restoring the module-value key made both controls pass again.
