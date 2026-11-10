<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Tasklist: first slice of the next pipeline (bounded test)

Lead: claude_chirho. Implementer: claude2_chirho from 2026-11-12. Design: `spec-chirho/pathway-chirho/26-11-10-faithful-pipeline-architecture-chirho.md`, slices S0 and S1.

The slice proves or drops the new checker on a bucket fixed before any code is written. The legacy compiler is not touched either way.

## The bucket

- **B-check:** every row of `per-test-chirho.tsv` whose source has no LANGUAGE pragma and imports nothing beyond the Prelude. Both expectations count: compile and reject.
- **B-run:** the six programs in `bench-chirho/` and every program in `examples-chirho/` that imports only the Prelude. Reference: GHC 9.14.1 stdout and exit code.
- **Pass:** all three hold.
  1. `next` agrees with GHC on more B-check rows than `legacy`, for expected-compile and for expected-reject separately.
  2. No B-check row that `legacy` gets right is wrong in `next`.
  3. Every B-run program prints GHC's output on the interpreter, with Eq, Ord, Show and Num coming from real `GHC.Classes`, `GHC.Base`, `GHC.Show` and `GHC.Num` source.
- **Drop rule:** L.J. sets the token budget for the slice on 2026-11-12. If the bucket is not passed inside it, stop, report the measured state and keep `legacy` as it is.

## Steps

- [ ] GHC 9.14.1 source archive retained and hash-verified (gpt_chirho, #33675)
- [ ] Bucket membership computed from the per-test table; list frozen with the legacy baseline for B-check and B-run
- [ ] S0: crates `haskelujah-hir-chirho` and `haskelujah-tc-chirho`; `--pipeline next`; Core gets its own type carrier or an adapter
- [ ] S0: interner, metas with levels, unifier, constraint types, solver for equalities, dictionaries and implications
- [ ] S1: Haskell 2010 expressions, patterns, bindings, generalization, data, newtypes, synonyms, classes, instances, defaulting, literals; Core out with explicit dictionaries
- [ ] W0: unboxed values, primop calls and MagicHash in the interpreter; primop table generated from `primops.txt.pp`
- [ ] L0: real `GHC.Types`, `GHC.Classes`, `GHC.CString`, then as much of `GHC.Base`, `GHC.Show`, `GHC.Num` as B-run needs; every temporary patch recorded in the patch series
- [ ] Bucket measured; result reported to L.J.; continue or drop
