<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Audit PRD Chirho

## Purpose

This document is a product-requirements-style audit of the current `haskelujah-*` workspace as of 2026-03-17.

It is not a greenfield vision document. It is a correction document: it records what the codebase actually is now, where the highest-risk technical debt sits, and what should be treated as the next hard product requirements for the compiler.

## Audit Snapshot

- Workspace crates: 21
- Rust source files under `crates/`: 129
- Rust line count under `crates/`: 130,909
- Static `#[test]` count: 2,087
- Progress log rows in `spec-chirho/progress-chirho.sqlite`: 232
- Current machine PRD milestone: `M1` in progress
- Current code reality: late `M1` with significant `M2` and `M3` surface already implemented
- Audit-time test signal: standard workspace suites passed cleanly before the long-running GHC bulk driver tests exceeded the normal audit window

## Evidence Used

- `Cargo.toml`
- `spec-chirho/prd-chirho.json`
- `spec-chirho/roadmap-chirho.md`
- `spec-chirho/phase3-progress-chirho.md`
- `crates/haskelujah-driver-chirho/src/lib.rs`
- `crates/haskelujah-driver-chirho/tests/ghc_bulk_chirho.rs`
- `crates/haskelujah-parser-chirho/src/proptest_chirho.rs`
- `cargo test --workspace --quiet`

## Executive Summary

The codebase is materially ahead of the written product plan.

This is no longer a compiler bootstrap or a parser-plus-typechecker prototype. It is a broad compiler/runtime system with:

- a lossless syntax pipeline
- CST parsing and AST lowering
- name resolution and module interfaces
- kind inference and HM type inference
- large extension coverage
- Core lowering, dictionary passing, and simplification
- STG-style runtime evaluation
- package and incremental infrastructure
- LLVM and Wasm backends
- Template Haskell and RTS-specific crates in the workspace
- a large driver-level and corpus-level test surface

The central problem has shifted. The project’s main risk is not missing architecture. It is coherence:

- spec drift
- panic hardening gaps
- warning/result plumbing gaps
- very large module sizes
- slow or weakly-asserted corpus testing
- mismatch between backend maturity labels and actual runtime support

## What The Audit Finds

### 1. The written PRD understates the implemented system

`spec-chirho/prd-chirho.json` and `spec-chirho/roadmap-chirho.md` still describe major areas as future milestones that are already present in the codebase.

Examples:

- `spec-chirho/prd-chirho.json` still marks Milestones 2 through 8 as mostly `not-started`
- `spec-chirho/roadmap-chirho.md` still positions Core lowering, script execution, and wide extension support as future milestones
- `spec-chirho/phase3-progress-chirho.md` records an extensive list of already-completed advanced features, including GADTs, RankNTypes, ScopedTypeVariables, Template Haskell quotes, LinearTypes, TypeOperators, MagicHash, LambdaCase, quantified constraints, type families, and much more

This is not harmless documentation lag. It causes planning drift, inaccurate milestone reporting, and difficulty deciding what the next product requirements actually are.

### 2. The machine-readable PRD crate graph is incomplete relative to the workspace

The live workspace in `Cargo.toml` contains 21 crates, including:

- `crates/haskelujah-rts-chirho`
- `crates/haskelujah-th-chirho`

Those crates are not present in `spec-chirho/prd-chirho.json`'s `crate_graph_chirho`.

That means the machine PRD is not a faithful system inventory anymore. It should be treated as partially stale until corrected.

### 3. Parser malformed-input robustness is still below product-grade expectations

`crates/haskelujah-parser-chirho/src/proptest_chirho.rs:9-10` explicitly documents that arbitrary malformed input still causes internal panics due to checkpoint-stack depth mismatch in the green builder.

The same panic-containment pattern appears in corpus tests under `crates/haskelujah-parser-chirho/tests/ghc_corpus_chirho.rs`.

This is the strongest current correctness gap because it affects:

- crash resilience
- fuzzing value
- editor/LSP viability
- batch compilation against ugly real-world inputs

Requirement consequence: malformed input must become diagnosable failure, not panic-tolerated behavior.

### 4. Warnings are computed but not carried through the main compile APIs

`crates/haskelujah-driver-chirho/src/lib.rs` has shared frontend orchestration through `run_frontend_chirho`, which is good.

However, common compile paths still discard warnings via:

- `crates/haskelujah-driver-chirho/src/lib.rs:634`
- `crates/haskelujah-driver-chirho/src/lib.rs:723`
- `crates/haskelujah-driver-chirho/src/lib.rs:1205`

That means deriving, exhaustiveness, orphan-instance, and similar warnings can exist internally but disappear from common compilation call sites.

This is an integration defect, not a missing subsystem.

### 5. Incremental compilation still stops short of full compile-result reuse

`crates/haskelujah-driver-chirho/src/lib.rs:1176` still carries:

- `TODO: cache serialized CompileResultChirho in the artifact store`

The architecture exists, but the highest-value path is still unfinished: cache hits should avoid recomputing the full compile result whenever inputs and dependencies are unchanged.

### 6. The bulk GHC test runner is useful but currently expensive and under-asserted

`crates/haskelujah-driver-chirho/tests/ghc_bulk_chirho.rs` is a valuable compatibility signal, but the current implementation has two product issues:

1. It is slow enough that `cargo test --workspace --quiet` reached long-running GHC bulk tests and exceeded a normal audit feedback window.
2. Its file header says it "asserts minimum thresholds," but the implementation only prints rates and does not enforce explicit pass-rate thresholds in the summary path.

This makes the suite simultaneously expensive and softer than it claims.

### 7. Build hygiene warnings are now real audit findings

The current workspace test run emits warnings in the driver test tree:

- duplicate `#[test]` attributes in `crates/haskelujah-driver-chirho/src/tests_chirho/extensions_chirho.rs:420` and `:497`
- non-snake-case function names in `crates/haskelujah-driver-chirho/src/tests_chirho/extensions_chirho.rs:1008` and `:1017`
- dead code warning for `BulkResultChirho` in `crates/haskelujah-driver-chirho/tests/ghc_bulk_chirho.rs:24`

These are not catastrophic, but they are evidence that the repo has moved past the stage where warning debt can be ignored without hurting confidence.

### 8. The biggest modules are now an architectural risk category of their own

The largest current Rust files include:

- `crates/haskelujah-core-chirho/src/dict_chirho/prelude_chirho.rs` — 14,358 lines
- `crates/haskelujah-typing-chirho/src/infer_chirho.rs` — 11,112 lines
- `crates/haskelujah-parser-chirho/src/lower_chirho.rs` — 8,092 lines
- `crates/haskelujah-driver-chirho/src/tests_chirho/eval_basic_chirho.rs` — 6,429 lines
- `crates/haskelujah-core-chirho/src/desugar_chirho.rs` — 5,956 lines
- `crates/haskelujah-naming-chirho/src/iface_chirho.rs` — 4,970 lines
- `crates/haskelujah-runtime-chirho/src/eval_chirho.rs` — 4,573 lines

At this size, the risk is not style. The risk is:

- slower review
- fragile invariants
- reduced refactorability
- difficulty isolating correctness bugs
- higher chance of subtle coupling between compiler phases

### 9. Backend messaging should stay strict about maturity

LLVM and Wasm are present and meaningful.

Cranelift, JVM, and BEAM exist in the workspace but are still correctly closer to scaffold-level in the machine PRD. That honesty should remain, especially now that the rest of the pipeline is strong enough that people may assume all listed targets are equally serious.

## Product Requirements From This Audit

### Requirement Chirho A1 — Re-baseline the spec against the code

The spec package must describe the codebase that exists now, not the one from two phases ago.

Acceptance criteria:

- `spec-chirho/prd-chirho.json` includes all 21 current crates
- milestone statuses are updated to match current implementation reality
- `spec-chirho/roadmap-chirho.md` reflects that advanced frontend, Core, runtime, and extension work already exist
- `spec-chirho/phase3-progress-chirho.md` and the roadmap are no longer in open contradiction

### Requirement Chirho A2 — Eliminate parser panic-tolerant behavior on malformed input

The parser must treat malformed input as recoverable diagnostic output, not a panic case contained by tests.

Acceptance criteria:

- `crates/haskelujah-parser-chirho/src/proptest_chirho.rs` no longer documents panic tolerance as a known bug
- arbitrary malformed input property tests run without `catch_unwind`
- GHC corpus parser tests no longer need panic containment to avoid process abort

### Requirement Chirho A3 — Expose warnings consistently through compile results

All normal compile entrypoints must preserve non-fatal warning information.

Acceptance criteria:

- `compile_source_chirho` returns warnings
- multi-module compilation surfaces warnings per module or per batch in a stable form
- warning propagation is covered by end-to-end tests

### Requirement Chirho A4 — Turn incremental architecture into real build-speed wins

Incremental compilation should not stop at dependency and fingerprint bookkeeping.

Acceptance criteria:

- compile results are serialized into the artifact store
- stable cache hits skip redundant compilation work
- repeated compile tests show materially improved latency for unchanged modules

### Requirement Chirho A5 — Split the largest implementation files

The current monolith-size modules must be decomposed before more major features are layered on top.

Acceptance criteria:

- `infer_chirho.rs`, `lower_chirho.rs`, `iface_chirho.rs`, and `eval_chirho.rs` are split into clearer submodules
- no single production Rust source file remains above an agreed maximum without explicit justification
- major subsystem boundaries become visible from file layout alone

### Requirement Chirho A6 — Make GHC corpus testing faster and more honest

The bulk compatibility runner should be both useful and operationally credible.

Acceptance criteria:

- slow corpus tests are tagged, segmented, or gated so default workspace runs remain developer-friendly
- pass-rate thresholds are actually enforced when the tests claim they are
- summary output distinguishes compile failures, panics, and skipped or unsupported cases

### Requirement Chirho A7 — Zero-warning hygiene for owned code

The project should not normalize avoidable warnings in its own test and support code.

Acceptance criteria:

- duplicate test attributes removed
- non-snake-case names fixed in owned Rust test code
- dead-code warnings eliminated or explicitly justified

## Milestone Reset Recommendation

The next meaningful product sequence should not be described as "start advanced typing and Core work." That work already exists.

Recommended near-term milestone language:

1. **Audit Alignment Chirho**
   - spec and machine PRD aligned with actual workspace and feature state

2. **Robustness Chirho**
   - parser panic removal
   - warning plumbing
   - corpus-test threshold enforcement

3. **Performance And Modularity Chirho**
   - compile-result caching
   - large-file decomposition
   - faster developer test lanes

4. **Compatibility Hardening Chirho**
   - deeper GHC corpus coverage
   - stronger runtime and TH invariants
   - clearer unsupported-feature diagnostics

## Success Metrics

- machine PRD crate count equals workspace crate count
- no parser panic tolerance documented in property tests
- main compile APIs expose warnings
- default workspace test lane completes in a developer-acceptable time budget
- no avoidable warnings in owned code
- no production compiler source file above the agreed cap without explicit reason

## Bottom Line

Haskelujah Chirho is not short on capability. It is short on alignment between its code, its specs, and its operational discipline.

That is a much better problem than "the compiler does not exist," but it is still a real product problem. The next PRD layer should therefore focus less on inventing new architecture and more on making the existing architecture honest, hard to break, and easier to evolve.
