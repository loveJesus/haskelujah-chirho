<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Rank-N expected-type propagation tasklist

Owner: `gpt_chirho`

## Brick 1

- [x] Reproduce `Vta2.hs`: both `if` and `case` infer one monomorphic type for repeated uses
      of a rank-N lambda parameter, yielding `expected Int, found Bool`.
- [x] Reproduce the `Vta1.hs` regression: `f = g True` checks `True` against `F t` before
      using the expected `Char` result to establish `t ~ Char` and reduce `F Char ~ Bool`.
- [x] Identify the gap: expected result types are not pushed through `if` branches or `case`
      alternatives, so their lambda parameters are initially bound monomorphically.
- [x] Verify that once a lambda receives its expected rank-N parameter type,
      `bind_pat_chirho` already turns the outer forall into a fresh-per-use scheme; no new
      application-time instantiation path is needed.
- [x] Pin expected-type propagation through both control-flow forms before changing behavior.

## Implementation

- [x] Permit equations to consume fewer term arguments than their signatures and check the RHS
      against the residual function type.
- [x] Use a known application result type to specialize shared variables before checking an
      argument whose expected type may require type-family reduction.
- [x] Check both `if` branches against their expected type after checking the condition as
      `Bool`.
- [x] Check every `case` alternative RHS against the expected result type while preserving
      pattern and local-where scopes.
- [x] Limit the new branch propagation to polymorphic expected types so ordinary monomorphic
      and GADT inference retain their existing behavior.
- [x] Keep visible type application, required type arguments, GADT leniency, and ground
      condition diagnostics unchanged.

## Gates

- [x] Make `Vta1.hs` and `Vta2.hs` check successfully and keep `T17594f.hs`/`T12734a.hs`
      green on a fresh CLI binary.
- [x] Pass the two focused inference tests and the three focused driver extension tests.
- [x] Pass typing `264 passed, 1 ignored`, Core `128 passed`, and the deterministic parser
      surface `289 passed, 3 ignored, 3 known filtered`.
- [x] Run a warning-free CLI build.
- [x] Run formatting checks and `git diff --check`.
- [x] Commit explicit owned paths and push `main_chirho`.
- [x] Release the builder and request corpus
      remeasurement because two tracked GHC files flip.
