<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# A failed run keeps its stdout (O1) Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (row 522). O1 is the shared output prerequisite of two stacks: R1/R2, whose SectionChirho and SectionExposureChirho must print their first two GHC lines before the named `Eq [[Int]]` failure, and Char, whose V12 must keep `before`. Branch run-outcome-chirho from main 00bdb218. No Cargo lease yet.

## Contract

GHC's top handler flushes stdout before it reports an uncaught exception. The STG machine buffers stdout in `io_output_chirho`, and every runner dropped the machine on a runtime failure, so no failing run could show its earlier lines. `RunOutcomeChirho` keeps the machine beside the run's result through one shared runner body. The three runners and `eval_source_with_machine_chirho` keep their signatures and messages as wrappers. `haskelujah run` prints the kept output, flushes, then reports the failure with exit 1. The REPL, stderr handles, `hFlush`, `exitWith` and the doubled `runtime error:` prefix remain named boundaries.

## Source

- [x] Contract, placement and controls posted (#27663) and accepted at source scope; the Char blob correction is recorded.
- [x] Driver: `run_lowered_chirho`, `RunOutcomeChirho`, the wrappers, `eval_source_outcome_chirho` and two driver tests (6d752cec).
- [x] CLI run arm and the execution-oracles workflow section (c24af12c).
- [x] Each changed Rust file formatted through stdin under a hard per-file timeout, with no remaining diff.

## Integration

- [ ] Replay onto the R1/R2 stack, where the output-path blobs equal main's.
- [ ] Bounded merge into the Char stack, preserving its `lower_lit_chirho` line in stg_lower, its four `EVIDENCE_METHOD_NAMES_CHIRHO` lines in lib.rs and its REPL line.

## Gate

- [x] Under a named slot: a warning-free build, the driver tests and the existing IO and eval suites green, then a frozen CLI.
- [x] CLI rows against GHC 9.14.1: print then error keeps `1` with exit 1; a partial line; an error with no output; an unchanged success row.
- [x] Mutations: the CLI prints only on success; the runner clears a failed run's output. Each must turn its own control red.
- [ ] Cross-unit rows: SectionChirho and SectionExposureChirho in the R1/R2 gate, V12 in the Char gate.

## Measured, gate o1-4f0b9ae9-chirho (2026-10-18)

All twenty ledger stages GREEN on tip 4f0b9ae9 over base 00bdb218; the frozen candidate CLI is 3e28b6d3 and the frozen base CLI 65d3ceee. Build: rc 0, no warnings. Focused, at -j2 and two test threads: the driver tests filtered to eval_io, eval_prelude, eval_list, stg_lower and recursive_do passed 260; io_actions passed 9; dictionary_evidence passed 10; both O1 tests ran and passed. Rows: the candidate met all four against runghc, with stdout, stderr and exit kept apart. The frozen R1/R2 CLI 0014812f, whose output-path files equal the base's, printed nothing on the two rows with output before the failure. Mutations: each was caught by its own control (the driver test's `""` against `"1\n"`, the row's empty stdout), with baseline and restored runs green and the tree restored byte for byte. Clippy found no warning on a changed line; 385 earlier warnings remain on unchanged lines in 55 files. Gitleaks found no leaks in three commits. Paired corpus against the freshly built base: accept (938 programs) and reject (767) verdicts are identical to the base's, 293 replayed rejections are identical, and execution keeps the same predicate verdicts (28 of 35 on both). Receipts are under tmp-chirho/run-outcome-chirho/runs-chirho/o1-4f0b9ae9-chirho/ in the claude worktree.
