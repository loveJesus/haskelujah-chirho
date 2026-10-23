<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ordering literal seeds Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (unit 530).

**The bug (RC5).** Bare `LT`, `EQ` and `GT` dispatch at Int. The trace is gpt_chirho #28162; the GO is #28187.
- Typing never binds them, so each one becomes a fresh type variable.
- Its Eq and Ord occurrences then default to Int evidence.

**The fix.** Seed them as mono `Ordering` beside `True` and `False`, in `builtins_chirho/constructors_chirho.rs`.

The branch `ordering-literal-seeds-chirho` starts from exact main bd690d83e289b4eb0a2153ff802b34db20367f38, whose crates tree is a48b0b73.

## Source

- [x] C2 (eedeb478), one commit:
  - seed LT/EQ/GT as `Ordering` in constructors_chirho.rs;
  - add tests/dictionary_evidence_chirho/ordering_literals_chirho.rs and its harness `mod` lines.
  - Rows fixed to GHC's answer: R1, R2, R5, R9 and T12. C2 also claimed T10 and T11. Gate run 1 proved that claim wrong, and C4 corrects it.
  - Rejected like GHC: T01 and T04, by `check`, with a type mismatch naming `Ordering`.
  - Unchanged:
    - exact: T05-T09, T13, T14, T16, R3, R4, R6, R7 and R8;
    - T17: the module's own constructors named LT/EQ/GT still override the seed;
    - still rejected: T02, T03, N01, N02 and N03.
- [x] C3, docs:
  - a "Builtin constructor seeds" section in language-features-chirho/dictionary-evidence-chirho.md, which the seed's code comment points to;
  - the RC5 sentences in that doc and in compiler-pipeline-chirho/builtin-type-environment-chirho.md, now stale, rewritten in the present tense;
  - this tasklist's ticks.
- [x] Checks without Cargo:
  - rustfmt, `diff --check` and gitleaks;
  - GHC 9.14.1 receipts for every program in the new test file;
  - path and directory limits.
  - Results:
    - All 3 Rust files are pinned-rustfmt no-ops, and `diff --check` is clean.
    - gitleaks finds nothing in the unit's commits.
    - GHC 9.14.1: all 28 extracted programs behave as recorded. The 21 accepted programs print their output with rc 0, and the 7 rejected ones fail with GHC-83865.
    - On the base CLI 2ddfca25, exactly the 9 targeted rows fail their new assertions, and the 19 controls already hold.
    - Limits: dictionary_evidence_chirho/ goes from 8 to 9 entries, and the new unit is 225 lines.
    - The driver's tests/ directory is already at 24 entries. That is older debt; this unit adds nothing there.
    - These checks ran only the base CLI. What the candidate would do was a prediction until gate run 1, which showed T10 and T11 still wrong.
- [x] C4, source only, after gate run 1 (gpt_chirho #28197 and #28198):
  - The bare-literal test asserts only R1, R2, R5, R9 and T12. Its 5 test names stay.
  - T10 and T11 stay in the file with GHC's answers, in an unasserted local table, so the oracle still extracts all 28 programs. No committed test pins their wrong behavior.
  - The seed section in dictionary-evidence-chirho.md now separates them:
    - T10 exposes the missing `Enum Ordering` instance;
    - T11 remains the recorded Ord method-model boundary.
  - Seven behaviors are corrected (R1, R2, R5, R9, T12, T01 and T04), but a run-level verdict moves for only six. T04's run already failed on the base, because its accepted program crashed. Its movement is in the `check` stream, from accepted to the mismatch.
  - No compiler code changes. The typing tree stays 53ddd1a3, so the frozen RC5 CLI db3d7bfa is the C4 compiler.

## Gate

The gate runs only under a builder handed by name. It is proposed with the source checkpoint.

- [x] Run 1, `rc5-ordering-14ac2e2f-chirho` on C3 (14ac2e2f): RED at focused, ledger 8880cce9.
  - Build was GREEN with 0 warnings; the frozen CLI is db3d7bfa.
  - `bare_ordering_literals_answer_as_ghc_chirho` failed at T10, and every later stage stayed unrun.
  - Diagnosis 884141af: `succ LT` is rejected naming the missing `Enum Ordering` instance, and `maximum [LT, GT, EQ]` crashes in the Int `<=` primop.
  - The run, its launcher 87a95826, the ledger and the diagnosis are preserved byte for byte.
- [x] Measured without Cargo, using the base CLI 2ddfca25 and db3d7bfa on the 28 programs: the candidate answers 26 of 28 as GHC does. Its two misses are T10 and T11.
- [ ] Build with 0 warnings. The focused suites differ from their baselines only by the new tests.
- [ ] A movement contract. Every other row, probe, matrix row and corpus verdict stays identical. The rows that move:
  - six run verdicts move to GHC's: R1, R2, R5, R9, T12 and T01;
  - T04's `check` moves to the mismatch;
  - T10 moves from a run crash to the named missing instance at `check`;
  - T11 holds.
- [ ] The mutants are caught: m1 (drop LT), m2 (seed them at Bool) and m3 (seed them at `forall a. a`). The existing 12 stay unchanged.
- [ ] Postflight passes.

## Outside this unit

- The unbound imported constructor `:|` (N04).
- Rejecting unbound constructors in general.
- Eq/Ord defaulting to Int without Num.
- T15, a user `data Ordering`.
