<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Sieve local-worker dispatch tasklist

Owner: `gpt_chirho`

## Brick 1

- [x] Claim the unowned defect in `haskelujah-chirho`.
- [x] Preserve the measured boundary: guard values are correct; boxed-Boolean case dispatch
      inside a recursive local worker is wrong; ReturnIO forcing is independently falsified.
- [x] Architecture placement remains undecided until a trace identifies the first divergence.
      Candidate layers are STG closure capture/lowering and runtime case entry; no force-harder
      workaround is acceptable.

## Trace

- [x] Reproduce the broken hand-written `where go` sieve and a comparison-free
      cross-invocation recursive stream on a fresh interpreter test binary.
- [x] Add temporary env-gated tracing for recursive closure allocation and placeholder patching.
- [x] Prove that two invocations patch the same static placeholder to different captured closures.
- [x] Remove all temporary trace code before landing.

## Fix

- [x] Add focused failing regressions for self-recursive, mutually recursive, and sieve workers.
- [x] Allocate captured recursive groups atomically per invocation in STG/runtime.
- [x] Restore both arg-index and value environments around the new group-body lowering path;
      keep unrelated lambda/non-recursive cleanup out of this root fix.

## Gates

- [x] Focused runtime/STG/driver regressions pass.
- [x] Bounded driver `eval_` suite passes with resource caps.
- [x] Fresh CLI reproducer returns `[2,3,5,7,11,13]`.
- [x] Owned files pass Rust 2024 formatting and `git diff --check`.
- [x] Commit explicit owned paths, push `main_chirho`, and announce builder release.
