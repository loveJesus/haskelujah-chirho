<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ordering literal seeds Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (unit 530).

**The bug (RC5).** Bare `LT`, `EQ` and `GT` dispatch at Int. The trace is gpt_chirho #28162; the GO is #28187.
- Typing never binds them, so each one becomes a fresh type variable.
- Its Eq and Ord occurrences then default to Int evidence.

**The fix.** Seed them as mono `Ordering` beside `True` and `False`, in `builtins_chirho/constructors_chirho.rs`.

The branch `ordering-literal-seeds-chirho` starts from exact main bd690d83e289b4eb0a2153ff802b34db20367f38, whose crates tree is a48b0b73.

After gate run 1, C1-C4 were replayed onto main 0ec9f8f5 (gpt_chirho #28249). That main is a DB-only child of bd690d83, opening row 531.
- The replay is `ordering-literal-seeds-replay-chirho`: C1 27635b4b, C2 a260cc5e, C3 4a7625d7, C4 4d443803.
- It differs from 655b590a only by main's DB commit. Its patch-ids, trees and messages are identical (replay proof 22/22).

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
- [x] C4 (655b590a, replayed as 4d443803), source only, after gate run 1 (gpt_chirho #28197 and #28198):
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
- [x] Run 2, `rc5-ordering-4d443803-chirho` on the replayed C4 (4d443803), 2026-10-25 13:15-14:13: GREEN, all 11 stages and the postflight.
  - Ledger f814ed31 (27 lines, none RED), launcher 835e4440, frozen CLI 77586815.
  - That CLI is not byte-identical to run 1's db3d7bfa, though C4 changed no compiler source. Its candidate streams match the db3d7bfa prediction on every measured row, apart from T11's heap numbers.
- [x] Build with 0 warnings. The focused suites differ from their baselines only by the new tests.
  - dictionary_evidence has 61: r2's 56 plus the 5 new tests.
  - core-lib (148), instance_obligations (28), io_actions (9), the filtered driver suite (479) and typing-lib (354) equal r2.
- [x] A movement contract. Every other row, probe, matrix row and corpus verdict stays identical. The rows that move:
  - six run verdicts move to GHC's: R1, R2, R5, R9, T12 and T01;
  - T04's `check` moves to the mismatch;
  - T10 moves from a run crash to the named missing instance at `check`;
  - T11 holds.
  - Measured:
    - the oracle candidate answers 26 of 28 as GHC does;
    - the matrix moves exactly D02, E02 and OrderingCompareRows to EXACT, with 24 requirements met;
    - the corpus equals r2's (28/35, none fixed, none lost);
    - lint matches r2 in both diagnostic forms, and gitleaks finds nothing in the 4 commits.
- [x] The mutants are caught: m1 (drop LT), m2 (seed them at Bool) and m3 (seed them at `forall a. a`). The existing 12 stay unchanged.
  - m1 and m3 fail the bare-literal test and the Int rejection test.
  - m2 fails both of those and the annotated test.
  - The existing sets catch 7, 3 and 2, as in r2. Their only new failures are among the 5 new tests.
- [x] Postflight passes: the frozen CLI, the tree, the 17 tools and the 21 immutable receipt sets are unchanged, and no compiler or CLI process is alive.

**Directory digests.** Run from gate-chirho/, a digest is `find <dir> -type f -exec shasum -a 256 {} + | LC_ALL=C sort | shasum -a 256`.
- Run 2's directory (24,667 files) is 57d3c449b1c16354c81d88e2aab45a4caaefac10e771505cec4f28e336d7967a.
- When files share contents, the sort locale orders them by path. That changes the digest:
  - the bd883e3f reported in #28280 is run 2's digest under en_US.UTF-8;
  - the gate's own pins were taken in that locale, so run 1's directory is f40fd8ed there and 409fc34e under `LC_ALL=C`.
- Future digest helpers pin `LC_ALL=C` (gpt_chirho #28282).

## Outside this unit

- The unbound imported constructor `:|` (N04).
- Rejecting unbound constructors in general.
- Eq/Ord defaulting to Int without Num.
- T15, a user `data Ordering`.
- `Enum`, `Bounded` and `Read` for `Ordering` (T10).
- The Ord method model behind `maximum`, `minimum` and `<=` at non-scalar types (T11).
