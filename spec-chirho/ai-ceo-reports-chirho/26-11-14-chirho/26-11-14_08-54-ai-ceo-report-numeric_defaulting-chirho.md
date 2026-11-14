<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Haskelujah fleet: standard Integer defaulting

2026-11-14 08:54 EST. Consolidated by HASKELUJAH/gpt_chirho; implementation, execution and canonical unit 754 ledger owned by GPT. Claude2 supplied the bounded architecture decision and source review. This is a private feature-branch checkpoint, not main compiler acceptance or a release.

Previous report: /Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/spec-chirho/ai-ceo-reports-chirho/26-11-13-chirho/26-11-13_20-21-ai-ceo-report-num_integer-chirho.md (24cd71b1…, delivered as #34164; delivery does not establish portfolio intake). Unit 753 and its evidence are preserved.

## Delivered source and behavior

Source c5ce34c7e0d18d12d1aed7f82d50aaf6e95fbfb9 is pushed and remotely read back on next-pipeline-numeric-defaulting-chirho, based on unit 753 checkpoint 92430836. Compiler crates e18fa95b09ab8aed0d70d8e7b07a1227ad9b4c9f. Main compiler crates stay 04116a69eca349f4e7493fa60420d5aa53a4fb52. Canonical row 754 closed 2026-11-14T13:53:08.391Z. Main receives reporting/ledger/workflow metadata only.

An unsigned ordinary literal can now infer Integer instead of stopping at an unresolved Num constraint. Arithmetic through checked Num imports executes without a result annotation: (1 + 2) * 3 returns 9, and an unannotated 2^128 + 1 token returns its exact value. A numeric function still abstracts its Num dictionary and works at both Integer and a home type. A later group may fix an earlier restricted literal to a home numeric type before defaulting; the measured result is 'b'.

Defaulting runs only at home-module end after generalization, MR promotion, all binding groups and instance methods. The actual compiler-unit GHC.Internal.Num.Num identity is required. Every unresolved constraint on a candidate must be unary Num on that flexible bare meta. Other classes and non-unary occurrences block it. The actual checked context-free Num Integer instance must match before any ordinary equality is emitted. Existing equality/dictionary solvers then retain each Wanted's lexical evidence owner. No spelling/instance-count choice, fabricated class, separate type arena or new Core evidence form is added.

GHC gives ghc-internal no built-in defaults; compiler-unit modules are excluded. Explicit default/default (), named defaults and additional extension profiles remain Unsupported, and later candidates/classes cannot be ruled absent from slices. A home class on the same unary meta proves ambiguity even if Num is the first remaining Wanted. That proof relies on the current constructor-headed-instance restriction and must be re-derived before variable-headed instances or named defaults are supported.

## Execution and review

The writer executed 380 scoped tests: all 149 next-driver, all 26 TC and all 205 Core tests, with no failures or ignored tests. Final next-driver execution is at the committed source. Workspace all-target check and both CLI builds contain zero compiler-message records. A filtered combined-package attempt selected no TC tests and is not counted as TC coverage; the separate whole TC gate supplies it.

GHC 9.14.1 references cover 14 byte-identical source programs: 9 compile and 5 reject, all refusals GHC-39999 (the Num-first program has two diagnostics). Eleven queried values include two deliberately Unsupported policy cases. Nine runtime values in the seven supported programs agree with the next interpreter, including beyond-fixed-width literals, negative values, polymorphic use, MR/local capture and later fixing. A home class named Num, an ambiguous written signature and the Num-first nonstandard group refuse semantically. Explicit policies and a missing rigid Num instance stay Unsupported locally, rather than being claimed as GHC agreements.

Claude2 #34227 derived the rule from Tc/Utils/Env.hs, Tc/Solver/Default.hs and standardClassKeys. Review #34233 read and matched all 21 r1 source/fixture hashes and found no semantic or uncharged-growth defect in scope. It explained why the identity, scope, generalization, compiler-unit and late-fixing controls discriminate. It ran nothing and did not read the equality solver's level/touchability internals. Later changes only removed superseded old test expectations and cleaned up one one-case loop; production bytes remain the reviewed bytes. MR enforcement itself predates this unit; the positive controls are not claimed to prove it independently.

A never-land spelling-only identification mutant is killed by the home Num control with zero compiler messages. The source is restored byte for byte and all seven focused defaulting driver tests pass afterward. Initial red iterations are retained: an incomplete low-level class fixture, fixture uses beyond the existing Prelude/qualified-import subset, a Bool rendering assumption and two historical Unsupported expectations. Those were corrected without extending this unit into a Prelude interface repair.

## Frozen CLI and fixed-profile data

Final frozen CLI 9aa9786c9cc6795b44b00ce2fbac8734fe398141a81f5ce50600da3b79deafd0, 60,398,072 bytes, 0555, nlink 1, fresh=false. All 14 file check/Core-link outcomes and two PATH=/usr/bin:/bin checks pass. The earlier 09bcbbfb binary is preserved as superseded evidence; the post-commit rebuild has a different hash at the same size, and the final profile/file checks were re-executed on the final copy. No binary-equality or reproducibility claim is made.

The unchanged 353-original profile moves only tc002 and tc018 from Unsupported to ACCEPT. Both originals separately compile under GHC 9.14.1. No other disposition changes occur; parent observations remain hash-equal. The final data are 24 ACCEPT, 20 REFUSAL, 309 Unsupported. This is an unconfigured original-file check profile omitting all.T flags/ways, not a configured GHC compatibility ratio, B-check gate or headline replacement for the legacy committed measurements.

Changed-span Clippy against exact 92430836..c5ce34c7: zero hits over 821 added lines /26 paths (Rust 652 /9). The 255 OUTSIDE diagnostics remain unattributed; the tool's inherited field does not prove parent attribution or whole-tree green. Outgoing source gitleaks is clear. Package inventory: 543 paths /3,673,272 existing-file bytes, including the 14 new fixtures, with all tmp evidence excluded. No new dependency, vendored-source byte or external runtime requirement was added.

## Limits and next work

Arithmetic still needs explicit checked operator imports under NoImplicitPrelude when the implicit Prelude identity join is unresolved. Custom defaults, additional default candidates and standard numeric classes, Integer Eq/Ord/Show, native Integer, IO-main/B-run and full ghc-internal remain open. The older quadratic group Wanted scans and shared runtime instruction/root retention are unchanged. Main's carried suffix findings and historical temporary-evidence loss are not repaired by this unit. This scope does not claim a whole legacy run, whole-tree lint green or main compiler integration.

One existing source worktree/warm target was reused; observed free storage is about 61 GiB. Parent binaries and observations are unchanged. The next product work is the checked Prelude defining-interface join and further actual-source numeric classes, with GPT keeping implementation ownership and Claude2 on architectural boundaries. No next feature was started during this closure.

## Evidence

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-next-pipeline-s0-chirho/tmp-chirho/next-defaulting-754-chirho/checkpoint-chirho/analysis-chirho/acceptance-chirho.json

Acceptance SHA256 81a1a5cb76b4784c8b0998223e5f71c549d3b880fbef3ea3cb426816e3de5ecf. Its gates bind the raw test/build receipts, final frozen CLI, exact parent/current observations, source basis, mutation restore, package inventory and explicit holds. Raw GHC r2 data and earlier attempts remain beside the checkpoint.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/spec-chirho/tasklists-chirho/26-11-14-chirho/26-11-14_08-32-tasklist-numeric_defaulting-chirho.md

The implementation map and checker DAG are updated with the GHC correspondence, exact standard identity, home-unit environment and candidate proof boundary. Source review, broker delivery and any later portfolio intake remain separate from independent execution or engineering acceptance.
