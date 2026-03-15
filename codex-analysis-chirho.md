<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Rhasky Chirho Codebase Analysis

## Current Snapshot

- Date checked: 2026-03-15
- Workspace size: 19 crates
- Rust size under `crates/`: about 99,905 lines
- Static test count: 1,530 `#[test]` cases
- Property-test presence: active `proptest!` suites in parser, core, and driver
- Verification: `cargo test --workspace --quiet` passed on this review
- Worktree status during review: active development branch with unrelated local changes in parser, syntax, naming, typing, and AST files

## Executive Summary

The codebase is in materially better shape than the previous analysis reflected. The workspace is green, the PRD is much closer to the actual implementation, the driver now has a shared frontend pipeline, property tests exist, and the project is no longer just "broad but drifting". It is broad and increasingly coherent.

The project still reads as `M1` rather than "finished M1", but that is now because of hardening and completeness, not because the core compiler is missing. You already have a real end-to-end compiler/evaluator stack:

- lossless CST parsing with layout
- CST-to-AST lowering
- implicit Prelude handling
- name resolution and module interfaces
- kind inference
- HM type inference with typeclasses and deriving
- exhaustiveness checking
- Core desugaring, dictionary passing, and simplification
- STG-style evaluation runtime
- LLVM IR and Wasm emission
- Cabal / package / incremental support crates

## What Is Better Than Before

### 1. The workspace is green again

The most important outdated finding is gone: `cargo test --workspace --quiet` now passes.

That changes the project posture substantially. It means the current codebase is once again safe to describe as actively evolving but integrated, rather than "interesting subsystems with a broken workspace".

### 2. The driver architecture is more coherent

`crates/rhasky-driver-chirho/src/lib.rs:83-167` now has a shared `run_frontend_chirho` path that centralizes:

- CST parse
- AST lowering
- implicit Prelude import injection
- deriving
- name resolution
- orphan-instance warnings
- kind inference
- type inference
- exhaustiveness checking

That is a meaningful architectural improvement. It reduces phase drift and gives the project a clearer compiler entry boundary.

### 3. `check` now uses the real frontend

`crates/rhasky-driver-chirho/src/lib.rs:219-259` now runs `run_frontend_chirho` and extracts the module name from the real AST, instead of the older header-only path.

This is not full backend compilation, but it is no longer a fake "check". It is a genuine frontend pass with warnings, which is the right direction.

### 4. The PRD is much closer to reality

`spec-chirho/prd-chirho.json:3-10` now says `M1` is in progress, and `spec-chirho/prd-chirho.json:235-330` reflects the actual workspace crates instead of the older `hir/namer/typecheck/package-db` split.

This was one of the biggest process problems before. It is much better now.

### 5. Property tests are now real, not just aspirational

There are active property tests in:

- `crates/rhasky-parser-chirho/src/proptest_chirho.rs`
- `crates/rhasky-core-chirho/src/proptest_chirho.rs`
- `crates/rhasky-driver-chirho/src/tests_chirho/proptest_chirho.rs`

That is important because it moves the project from "lots of example tests" toward actual invariant checking.

## Current Strengths

### Foundation crates are solid

`rhasky-span-chirho`, `rhasky-diagnostics-chirho`, `rhasky-syntax-chirho`, and `rhasky-test-harness-chirho` remain the cleanest part of the repo. They are focused, reusable, and aligned with the intended long-term architecture.

### The front-end is now genuinely substantial

The parser, lowerer, naming, and typing layers are no longer toy implementations. The compiler supports enough real Haskell surface area that the remaining questions are about semantics, robustness, and packaging, not whether the pipeline exists.

### The runtime and evaluator are real

`rhasky-runtime-chirho` plus `rhasky-driver-chirho/src/stg_lower_chirho.rs` give the project an actual execution story. This is no longer just a parser/typechecker repo.

### Package and incremental work are valuable

`rhasky-package-chirho` and `rhasky-incremental-chirho` are still among the more disciplined parts of the project. They give the repo a credible path toward real-world compilation rather than isolated source-file demos.

## Current Highest-Priority Issues

### 1. Malformed-input parser robustness is still not where it needs to be

The strongest current correctness signal is in the parser proptest file itself:

- `crates/rhasky-parser-chirho/src/proptest_chirho.rs:9-13`
- `crates/rhasky-parser-chirho/src/proptest_chirho.rs:54-60`

The tests explicitly say arbitrary-input parsing currently uses `catch_unwind` because the parser can still panic on malformed input due to checkpoint stack depth mismatches in the green builder.

That means:

- the parser is good on valid and semi-structured inputs
- malformed-input recovery still has a real internal panic bug
- this matters for CLI robustness, fuzzing, editor/LSP scenarios, and future package-ecosystem ingestion

This is the highest-priority technical debt now that the workspace is green.

### 2. Non-fatal warnings are collected but not consistently surfaced

`run_frontend_chirho` collects deriving, exhaustiveness, and orphan-instance warnings:

- `crates/rhasky-driver-chirho/src/lib.rs:102-161`

`check_source_file_chirho` returns those warnings in `CheckSummaryChirho`:

- `crates/rhasky-driver-chirho/src/lib.rs:250-259`

But `compile_source_chirho` and `compile_modules_chirho` currently discard them when destructuring `FrontendResultChirho`:

- `crates/rhasky-driver-chirho/src/lib.rs:422-428`
- `crates/rhasky-driver-chirho/src/lib.rs:511-515`

This is no longer a phase-ordering problem. It is now a diagnostics plumbing problem.

Impact:

- front-end warning infrastructure exists
- backend consumers do not receive it consistently
- tools using `compile_source_chirho` can miss meaningful diagnostics

### 3. Incremental compilation exists, but full compile-result reuse is not finished

`rhasky-incremental-chirho` is real, and the driver uses session-based compilation ordering. But the integration is still incomplete:

- `crates/rhasky-driver-chirho/src/lib.rs:966-967`

The driver still has a TODO to cache serialized `CompileResultChirho` in the artifact store so cache-hit paths can skip recompilation entirely.

That means the architecture is present, but the high-value optimization loop is not closed yet.

### 4. `check` is now real frontend checking, but backend reporting is still stub-based

`check_source_file_chirho` now runs the real frontend, which is good. But it still reports backend output via lightweight stubs:

- `crates/rhasky-driver-chirho/src/lib.rs:246-248`

This is fine if the command is documented honestly. It becomes confusing only if the README or CLI messaging suggests `check` validates backend lowering too.

### 5. Experimental backends should be described as experimental

The workspace now includes:

- `rhasky-backend-cranelift-chirho`
- `rhasky-backend-jvm-chirho`
- `rhasky-backend-beam-chirho`

and the PRD marks them as scaffold/in-progress:

- `spec-chirho/prd-chirho.json:300-337`

This is healthy, but only if the docs say the same thing. The README should distinguish:

- currently-used backends and runtime paths
- experimental backends present in the workspace for future work

### 6. The largest modules are still large enough to be architectural risks

The top files are now roughly:

- `crates/rhasky-core-chirho/src/dict_chirho/prelude_chirho.rs` — 13,832 lines
- `crates/rhasky-typing-chirho/src/infer_chirho.rs` — 8,532 lines
- `crates/rhasky-driver-chirho/src/tests_chirho/eval_basic_chirho.rs` — 6,054 lines
- `crates/rhasky-parser-chirho/src/lower_chirho.rs` — 5,975 lines
- `crates/rhasky-core-chirho/src/desugar_chirho.rs` — 4,790 lines
- `crates/rhasky-runtime-chirho/src/eval_chirho.rs` — 4,494 lines

This is better than having one giant driver/test monolith, and some splitting has clearly already happened. But these files are still large enough that local reasoning, refactoring safety, and review quality will remain hard unless they keep being decomposed.

## Architectural Assessment By Area

### Frontend

The frontend has become the clearest success story in the repo. The phases are explicit, the driver shares them, the parser/lowering/naming/typing path is real, and the PRD now reflects that.

The main remaining frontend weakness is malformed-input hardening, not lack of language support.

### Middle End

Core desugaring, dictionary passing, simplification, specialization metadata, and inline annotations make the middle end much richer than the earlier README suggested.

This is now a serious compiler middle layer. The main challenge is maintainability, especially in the Core prelude generation and inference-heavy logic.

### Runtime and Evaluation

The STG-style runtime is one of the most impressive parts of the codebase. It already supports a substantial execution story, including closures, PAPs, GC, I/O capture, FFI, and many builtins.

The remaining risk is semantic completeness and keeping runtime/compiler assumptions aligned as more language surface area and more backends arrive.

### Tooling and Process

The project process is healthier than before because:

- the workspace is green
- the PRD is closer to reality
- the progress database is active
- tests are abundant
- property tests now exist

The next process improvement should be to keep README, PRD, and actual compiler entrypoints in sync so that the docs stop lagging behind the code.

## Progress Against Milestones

### M0

This looks complete in both code and PRD terms.

### M1

`M1` being "in-progress" still makes sense, but it is an advanced in-progress, not an early one.

The project already satisfies most of the spirit of the current M1 acceptance list:

- lexer/layout/CST/AST stack
- module loading and multi-module support
- HM typing, kind inference, typeclasses, deriving
- exhaustiveness
- Core IR and simplification
- STG evaluation path
- LLVM / Wasm emission
- incremental-session groundwork
- large test corpus

What still keeps `M1` from feeling done is mainly:

- malformed-input panic hardening
- better warning/reporting plumbing
- clearer status for experimental backends
- finishing the artifact-cache reuse story

## Recommended Next Moves

1. Remove the parser panic path.
   - Make arbitrary malformed input produce diagnostics, not `catch_unwind`-guarded panics.
2. Carry warnings through `compile_source_chirho` and `compile_modules_chirho`.
   - The collection work is already done; the API just does not expose it consistently.
3. Finish artifact-store reuse in the driver.
   - This is the next high-leverage systems improvement.
4. Keep splitting the largest files.
   - Especially Core prelude generation, type inference helpers, parser lowering, and huge driver test files.
5. Document backend maturity accurately.
   - LLVM/Wasm/STG evaluation are current execution paths; Cranelift/JVM/BEAM are experimental.

## Bottom Line

Rhasky Chirho is now a real compiler project with a green workspace, a coherent shared frontend, active property tests, and an execution story that goes well beyond parsing and typing.

The previous analysis was right that coherence mattered. The current analysis is that coherence has improved enough that the remaining work is now sharper and more valuable: robustness, warning propagation, cache integration, and continued modularization.
