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
  3. Every B-run program prints GHC's output on the interpreter, with Eq, Ord, Show and Num coming from real `GHC.Internal.Classes`, `GHC.Internal.Base`, `GHC.Internal.Show` and `GHC.Internal.Num` source.
- **Drop rule:** L.J. sets the token budget for the slice on 2026-11-12. If the bucket is not passed inside it, stop, report the measured state and keep `legacy` as it is.

## Steps

- [x] GHC 9.14.1 source and test-suite archives retained, hash-verified and unpacked (2026-11-10)
- [ ] Bucket membership computed from the per-test table; list frozen with the legacy baseline for B-check and B-run
- [ ] S0: crates `haskelujah-hir-chirho` and `haskelujah-tc-chirho`; `--pipeline next`; Core gets its own type carrier or an adapter
- [ ] S0: interner, metas with levels, unifier, constraint types, solver for equalities, dictionaries and implications
- [ ] S1: Haskell 2010 expressions, patterns, bindings, generalization, data, newtypes, synonyms, classes, instances, defaulting, literals; Core out with explicit dictionaries
- [ ] W0: unboxed values, primop calls and MagicHash in the interpreter; primop table generated from `primops.txt.pp`
- [ ] L0: real `GHC.Internal.Types`, `GHC.Internal.Classes`, `GHC.Internal.CString`, then as much of `GHC.Internal.Base`, `GHC.Internal.Show`, `GHC.Internal.Num` as B-run needs; every temporary patch recorded in the patch series
- [ ] Bucket measured; result reported to L.J.; continue or drop

## Solo continuation, L.J. 2026-11-10

L.J. directed gpt_chirho to inspect Claude's tmux handoff and continue alone after reporting Claude's token exhaustion. GPT takes the first implementation lane now; the earlier November 12 worker date does not block this continuation. The full B-check/B-run acceptance and drop rule above remain unchanged. No token budget was added by the agent.

Unit 740 begins from main 4348c3d9 in the isolated next-pipeline-s0-chirho worktree. The latest private legacy cefd5490 and its retained CLI/data remain frozen as the stronger comparison baseline. Nothing in that private compiler delta is silently merged into main.

Brick 1 placement: new haskelujah-hir-chirho owns compiler-independent names, term shapes and surface types; new haskelujah-tc-chirho owns the interned types, levels/metas, constraints and elaboration. A new driver next module owns the parser/HIR/checker/Core boundary and the only compatibility adapter into today's Core type data. It never calls legacy inference, the dictionary pass, or the handwritten interface catalogue. The CLI's explicit --pipeline next selects this path; default legacy behavior is retained. Unsupported next features refuse rather than delegating to legacy. No third-party dependency is introduced for the first brick.

- [ ] Freeze the literal B-check/B-run membership, profile, original hashes, stdin, and retained legacy outcomes before production authoring. Keep unknown action/flag cases explicit.
- [ ] Deliver a running S0 vertical: parsed Haskell through HIR, owned type/constraint solving and explicit Core into the existing interpreter, with a selectable CLI check path and real semantic failure controls. This milestone does not claim the full first slice or real-library runtime gate.
- [ ] Extend S1 and the real-library/primop boundary toward the unchanged full bucket; do not substitute handwritten standard classes or smaller benchmark arguments.
- [ ] Measure, secret-scan and push coherent owned progress. Preserve failures and mark only the actual completed scope; retain a resumable handoff.
