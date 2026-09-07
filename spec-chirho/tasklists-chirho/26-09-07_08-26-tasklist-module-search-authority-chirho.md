<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Module-search authority repair

Objective: restore the real CLI-in-place behavior broken by the type-namespace lane without weakening namespace accuracy or special-casing `GHC.Base`/the curated fixture.

## Brick 1 — reproduce and choose the authority boundary

- [x] Reproduce `T001_basic_types.hs` at landed `048de337` from the repository root and verify the isolated-file control.
- [x] Bracket against the built `665ec6ef` baseline when needed; distinguish candidate discovery from interface/export consumption.
- [x] Trace every consumer that treats a recursively discovered module as authoritative, not only the failing `GHC.Base` path.
- [x] Record the architecture placement before editing: move the search subsystem out of the 7,070-line driver root into `module_search_chirho.rs`; admit a recursive candidate only when its declared module exactly matches its path below the active source root, and keep interface names unique before any first/last-match consumer sees them.

## Brick 2 — behavioral fix

- [x] Add focused regressions whose assertions depend on a nested unrelated fixture not shadowing the canonical interface and on a genuine root-level `Prelude.hs` replacing the seeded fallback.
- [x] Preserve legitimate sibling-module and package-project discovery, including hierarchical Cabal ordering tests.
- [x] Implement the smallest root mechanism at the candidate-authority boundary; do not add a filename, module-name, or corpus guard.
- [x] Update the module/interface workflow and comment participating functions.

## Brick 3 — proof and landing

- [x] Run focused module-search, interface, and naming tests, then a warning-free explicit CLI build.
- [ ] Run the real CLI-in-place T001 check, isolated-directory control, curated 537 harness, and relevant driver suites.
- [ ] Run both GHC corpus axes twice against the landed 877/222 baseline; explain every movement before touching artifacts.
- [ ] Update canonical artifacts only if their exact lists move, write one progress row as the sole DB writer, commit named owned paths with machine-readable authority trailers, fast-forward `main_chirho`, push, and release SLOT/DB.

## Starting evidence

- `665ec6ef`: in-place T001 accepted.
- `55348594` and `048de337`: in-place T001 reports E0101 for `Int`; a copy in an empty directory is accepted.
- Reduction corrected the initial attribution: the minimal shadow is `ghc-tests-chirho/parser-chirho/should_compile/T17045/Prelude.hs`, whose narrow `module Prelude` replaces the seed. That fixture alone reproduces all five E0101s; `module-chirho/base01` alone does not. The earlier `665ec6ef`/`55348594` bracket remains valid.
- The typecheck corpus lives under a different root and the in-process curated harness resolves differently, so both published corpus gates and the 537 harness missed the CLI regression.
- The successful hierarchical parse path appended every interface without checking either the declared name against the path-derived candidate name or the declared name against interfaces already admitted. The sibling and panic-recovery paths already de-duplicate by declared name, exposing the success path as the inconsistent branch.
- Import/name/type consumers intentionally select an admitted interface by module name (several use newest-first; export extraction has older first-match sites). Enforcing unique names at admission removes that order dependence without changing resolver precedence for callers that deliberately provide successive interfaces.
- Probe trap: a bare relative `check T001.hs` supplies an empty parent path and does not exercise discovery; `check ./T001.hs` does. The regression test uses an absolute scratch path with a nonempty parent.

## Verification evidence so far

- Focused authority tests: 9 passed, including the nested `Prelude` rejection and root-level `Prelude` positive control.
- Existing sibling/hierarchical/Cabal ordering tests passed; naming crate: 132 passed.
- Explicit `cargo build -p haskelujah` was warning-free; both in-place and isolated CLI checks accepted T001.
- Driver `--lib` with the known Cranelift runaway skipped: 1760 passed; the four documented baseline reds remained. One unrelated CPP test collided with another test's shared scratch path under parallel execution and passed when rerun alone. A first worktree run also exposed the documented missing-package-cache trap; after linking the existing cache, all 19 package-backed tests passed.
- Driver-only clippy still reports 35 pre-existing warnings in `splice_chirho.rs`, `stg_lower_chirho.rs`, and the remaining driver root; the new focused module reports none.
