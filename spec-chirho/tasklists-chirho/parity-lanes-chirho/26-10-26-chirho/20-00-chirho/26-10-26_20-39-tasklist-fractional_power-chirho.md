<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 577: Fractional power

Base: `76ed56e4`, exact main with the row 577 progress reservation.

- [x] Reserve row 577 and announce the disjoint lane.
- [x] Measure GHC 9.14.1 and the interpreter, LLVM and Cranelift before source.
- [x] Trace the missing `(^^)` binding, scheme, evidence and existing power primitives.
- [x] Implement the smallest evidence-backed Prelude route without touching row 575 backends.
- [x] Add GHC-measured all-engine controls and pin named boundaries.
- [x] Kill a focused reach mutation and restore source byte-for-byte.
- [x] Update the dictionary-evidence workflow.
- [x] Run the warm focused gate, changed-line Clippy, rustfmt and diff-check.
- [ ] Replay onto exact main, rerun actual behavior, land, and close only row 577.

Scope is `(^^) :: (Fractional a, Integral b) => a -> b -> a`, first measured at `Double` with
`Int` and `Integer` exponents. Row 575's unary native Floating primitives, `(**)`, `(^)`, String
representation, and unrelated missing Prelude bindings remain outside this row.

## Evidence before source

- GHC 9.14.1 prints `8.0`, `0.125`, `1.0`, `Infinity`, `-0.125` and
  `117.39085287969571` for `2 ^^ 3`, `2 ^^ (-3)`, `0 ^^ 0`, `0 ^^ (-1)`,
  `(-2) ^^ (-3)` and `1.1 ^^ 50` at `Double`.
- Both section directions are measured: `map (2 ^^) [-2,0,3]` prints
  `[0.25,1.0,8.0]`, and `map (^^ 2) [1.5,2.5]` prints `[2.25,6.25]`.
- A top-level `Double` base with a top-level `Integer` exponent `-2` prints
  `0.4444444444444444`.
- At the base, the interpreter failed every case with `missing STG binding ^^`; LLVM and
  Cranelift failed with `missing native binding ^^`.

## Root and implementation

- The checker already has the exact two-predicate scheme and the environment's constrained-name
  scan includes `^^`; no operator-specific allowlist is required.
- Fallback infix and section operators also resolved the shared canonical ID directly. They now
  mint a reference occurrence with the operator span and provenance. Primop-lowered operators
  keep their existing path.
- A `Fractional Double, Integral Int` occurrence becomes a body-backed generated helper applied
  to the body-backed Double division row. Other pairs, open evidence and dangling or missing
  bodies remain on the named-failure path.
- The helper uses row 570's `^.#`, then divides `1.0` by the result for a negative exponent. This
  retains GHC.Real's multiplication order and avoids inventing a native `recip#` lowering.
- An applied occurrence rewrites only its base under the proved `Double` key. This is what makes
  a negative base use `negateFloat#`; the exponent retains its separate Integral evidence.
- Focused controls are GREEN on the interpreter, LLVM and Cranelift, and the body-backed refusal
  unit is GREEN.

## Reach mutation

- Requiring the impossible key `Fractional DoubleMutationChirho` at the exact-pair guard makes
  the all-engine control fail immediately with `missing STG binding ^^`; exit 101.
- Restoring only that guard returns the source file to SHA-256
  `9b140dcb3e13827cba8c1edacc2ed9144515b22a8571b24583b9701c969b0a2c`, and the same control
  passes on the interpreter, LLVM and Cranelift.
- A proposed insertion of `^^` into the driver's constrained-name set was removed before the
  mutation: deleting it did not change behavior, proving the normal environment scan already
  carries the binding and the special case was redundant.

## Warm gate on the source tree

- 331 focused tests pass: Core `175/175`, dictionary evidence `106/106`, method-occurrence
  evidence `7/7`, and typing integration `43/43`.
- The all-engine control passes inside the full dictionary-evidence suite; the body-backed refusal
  unit passes inside the full Core suite.
- Clippy reports 456 pre-existing primary warning locations across the two packages' dependency
  and test graph, with `0` on a Row 577 added line. The stricter `-D warnings` run stops first in
  untouched diagnostics, RTS and AST files, so no broader warning-free claim is made.
- `cargo fmt --all -- --check` and `git diff --check` are clean. Gitleaks follows the committed
  range, then the exact-main replay and actual all-engine rerun remain.
