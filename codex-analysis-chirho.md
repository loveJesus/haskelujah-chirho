<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. (John 3:16) -->

# Rhasky Chirho Codebase Analysis

## Current Snapshot

- Date checked: 2026-03-15
- Milestone posture in PRD: `M1` is `in-progress`; `M0` is effectively complete
- Workspace crates: 19
- Rust source files under `crates/`: 102
- Rust line count under `crates/`: 103,066
- Static `#[test]` count: 1,565
- Progress log rows in `spec-chirho/progress-chirho.sqlite`: 216
- Verification: `cargo test --workspace --quiet` passed on this review
- Worktree during review: active local changes in naming, runtime, and typing plus untracked `.claude/`

This analysis reflects the current working tree, not a pristine branch tip. That matters because the repo is moving fast and some conclusions are about current integration quality, not just committed design intent.

## Executive Summary

This is now a real compiler project, not a promising skeleton.

The front end is substantial, the middle end is credible, the runtime is real, the workspace is green, and the documentation state is materially better than it was in the older analysis. The main constraint is no longer "does the pipeline exist?" It does. The constraint is now systems quality: panic hardening, warning propagation, cache reuse, semantic completeness, and decomposition of very large modules.

The project reads as late `M1`, with pieces of later milestones already present:

- lossless syntax and layout handling
- CST parsing and AST lowering
- multi-module name resolution and interfaces
- kind inference and HM type inference
- typeclass support and deriving
- exhaustiveness checking
- Core lowering, dictionary passing, and simplification
- STG-style evaluation with GC and FFI
- LLVM and Wasm backend paths
- package and incremental infrastructure

That is a strong base. The risk is not lack of ambition or lack of implementation. The risk is that the project is now large enough that integration discipline matters more than raw feature growth.

## Evidence Checked

The current analysis was refreshed from the local repository state using:

- `git status --short`
- `cargo test --workspace --quiet`
- `sqlite3 spec-chirho/progress-chirho.sqlite 'select count(*) from steps_taken_chirho;'`
- direct inspection of:
  - `spec-chirho/prd-chirho.json`
  - `crates/rhasky-driver-chirho/src/lib.rs`
  - `crates/rhasky-parser-chirho/src/proptest_chirho.rs`

## What Is Clearly Better Now

### 1. The workspace is healthy

The most important current fact is simple: the workspace test run passes.

That changes the overall posture of the project. A compiler codebase with this many moving parts is only really usable if the workspace stays green. Right now it does.

### 2. The driver has a real shared frontend entrypoint

`rhasky-driver-chirho` now uses `run_frontend_chirho` as a common phase boundary for:

- parsing
- AST lowering
- implicit Prelude insertion
- deriving
- name resolution
- orphan-instance checking
- kind inference
- type inference
- exhaustiveness checking

That is the right architectural shape. It reduces drift between commands and makes the compiler easier to reason about phase by phase.

### 3. The PRD is much closer to the real workspace

Earlier documentation drift was a serious problem. It is much better now.

The PRD tracks the actual 19-crate workspace, recognizes the current milestone as `M1`, and no longer describes an obviously outdated architecture. It is still aspirational in places, but it is no longer detached from the codebase.

### 4. Property testing is present in meaningful places

There are active `proptest!` suites in parser, core, and driver layers. That matters because the repo had already accumulated a large example-based test corpus; adding invariant-style testing is what starts to make a compiler robust instead of merely broad.

### 5. Runtime and evaluation work are not superficial anymore

The runtime is no longer a placeholder execution story. It includes:

- STG-style evaluation
- closure and application mechanics
- GC
- FFI machinery
- exception/catch support
- a large driver-level end-to-end eval test corpus

That gives the project a real semantic core, not just a syntax-and-typing prototype.

## Current Strengths By Area

### Syntax, spans, diagnostics, and harnesses

The foundational crates still look like the cleanest part of the workspace. They are relatively focused and their boundaries make sense. This is valuable because compilers tend to rot first at the edges where core representations become muddy; these crates are mostly resisting that.

### Frontend pipeline

The frontend is now legitimately substantial. The parser, lowerer, naming layer, and typing layer are all large because they do real work, not because they are empty abstractions.

The strongest signal here is breadth plus integration:

- layout-sensitive syntax is handled
- module interfaces exist
- imports and exports are not toy-only
- kind inference is wired before type inference
- exhaustiveness is wired after type inference

That is coherent compiler architecture.

### Middle end

The Core pipeline is one of the repo's strongest signs of seriousness. Desugaring, dictionary passing, simplification, and backend lowering are no longer wishlist items. There is already enough shape here for optimization and backend work to be worth doing.

### Runtime and driver

The driver and runtime together give the repo a real end-to-end identity. This is not merely a compiler that can dump IR. It can parse, typecheck, lower, and execute a significant subset of Haskell-like programs today.

### Tooling and project process

The progress database exists and is being used. The test corpus is large. The PRD is much more aligned than before. These are all positive signs that the project is being operated as an engineering effort rather than a pile of experiments.

## Current Highest-Priority Issues

### 1. Parser malformed-input robustness still has a known panic hole

This remains the sharpest correctness concern.

`crates/rhasky-parser-chirho/src/proptest_chirho.rs` still explicitly documents and uses `catch_unwind` around arbitrary malformed input because the parser can panic on certain bad token streams due to builder/checkpoint mismatches.

That means the project currently has two parser quality levels:

- strong enough for normal and test-driven inputs
- not yet hardened enough for hostile or highly malformed inputs

For a compiler, editor integration, or package-ingestion workflow, that gap matters a lot. A malformed program should produce diagnostics, not require panic containment in tests.

### 2. Warning plumbing is still incomplete in the main compile APIs

`run_frontend_chirho` correctly collects non-fatal warnings from deriving, exhaustiveness, and orphan-instance checks. That is good.

But `compile_source_chirho`, `compile_modules_chirho`, and additional internal call sites still destructure `FrontendResultChirho` using `warnings_chirho: _warnings_chirho`, which means warnings are gathered and then discarded on common compile paths.

That is no longer a design gap. It is an integration gap.

Impact:

- warnings exist
- the frontend already computes them
- some consumers never see them

This is exactly the kind of issue that makes a compiler feel less mature than it is internally.

### 3. Incremental compilation architecture exists, but the last high-value reuse step is still pending

The driver still contains the TODO to cache serialized `CompileResultChirho` in the artifact store.

That means the project has:

- dependency and fingerprinting infrastructure
- artifact-store concepts
- session-aware orchestration

but not the final closed loop where cache hits fully bypass repeated compile work. This is one of the highest-leverage remaining engineering tasks because it converts architectural groundwork into visible iteration speed.

### 4. `check` is much better, but backend previewing is still stub-shaped

The `check` path now uses the real shared frontend, which is the correct move. But it still derives LLVM/Wasm preview information through stub helpers rather than the full backend pipeline.

That is acceptable if described honestly. It becomes a problem only if tooling or documentation starts implying full backend validation from a cheap frontend-oriented check command.

### 5. The largest modules are still large enough to hurt maintainability

Current outliers include:

- `rhasky-core-chirho/src/dict_chirho/prelude_chirho.rs` at 14,042 lines
- `rhasky-typing-chirho/src/infer_chirho.rs` at 8,945 lines
- `rhasky-driver-chirho/src/tests_chirho/eval_basic_chirho.rs` at 6,054 lines
- `rhasky-parser-chirho/src/lower_chirho.rs` at 5,976 lines
- `rhasky-runtime-chirho/src/eval_chirho.rs` at 4,902 lines
- `rhasky-core-chirho/src/desugar_chirho.rs` at 4,790 lines

Large files are not automatically bad. In a compiler, though, they correlate strongly with:

- harder local reasoning
- slower review quality
- accidental invariant leakage
- duplicated helper logic
- fear of refactoring

The codebase is now mature enough that continuing to split these files is not cleanup vanity; it is risk control.

### 6. Backend maturity needs continued honesty

LLVM, Wasm, and the runtime evaluator are real current paths.

Cranelift, JVM, and BEAM are valuable directions, but they should continue to be framed as experimental or scaffold-level until they are proven under the same end-to-end standards as the main pipeline. The codebase is healthier when docs, tests, and user expectations are strict about that distinction.

## Architectural Read

### The project is increasingly phase-correct

The strongest architectural improvement is that phase boundaries are becoming real rather than rhetorical. The driver now reflects the intended compiler story instead of bypassing it. This matters more than any single feature.

The pipeline is understandable:

1. syntax and layout
2. CST parse
3. AST lower
4. naming
5. kinds
6. types
7. exhaustiveness
8. Core transforms
9. backend or runtime path

That is the right shape for a backend-agnostic Haskell compiler in Rust.

### The middle end is where long-term leverage now lives

The project already has enough frontend breadth that adding more syntax alone is no longer the highest leverage use of time. The biggest long-term leverage points now sit in:

- Core invariants
- optimization boundaries
- dictionary-passing correctness
- runtime/compiler agreement
- backend contract definition

That is normal for a compiler at this stage.

### The runtime/compiler contract is now important enough to formalize more aggressively

Because the runtime is real, the repo is now exposed to a class of bugs that toy compilers do not face:

- subtle forcing mismatches
- constructor layout assumptions
- FFI type agreement bugs
- exception propagation edge cases
- GC root tracking regressions

The solution is not "slow down runtime work". The solution is to make runtime contracts more explicit and test those invariants at the boundary between Core/STG lowering and execution.

## Testing Assessment

The testing story is strong in volume and increasingly credible in shape.

Current positives:

- very large example-based test corpus
- parser goldens
- driver end-to-end tests
- runtime coverage
- package and incremental tests
- property tests in parser, core, and driver
- green workspace run

What still needs attention:

- malformed-input parser tests still rely on panic containment
- very large test files should be split for maintainability just like large production files
- runtime semantic invariants deserve more property-style testing where feasible
- warning propagation should be tested end-to-end once exposed through compile APIs

The project no longer has a "missing tests" problem in the generic sense. It has a "target the remaining high-risk invariants more precisely" problem.

## Milestone Assessment

### M0

M0 looks done in practical terms.

### M1

`M1` being `in-progress` still makes sense, but it is clearly an advanced `in-progress`, not an early-stage milestone.

The codebase already satisfies most of the spirit of what a substantial `M1` should mean:

- concrete parsing and lowering pipeline
- module-aware naming
- kind and type inference
- typeclass and deriving infrastructure
- Core lowering and simplification
- executable runtime path
- backend output paths
- test harness and large regression suite

What keeps `M1` from feeling finished is mostly hardening and coherence:

- eliminate malformed-input panic paths
- surface warnings consistently
- finish meaningful incremental-result reuse
- keep docs honest about backend maturity
- keep splitting the biggest files

## Concrete Recommendations

1. Remove the parser panic escape hatch.
   Turn the `catch_unwind` proptest accommodation into a failing invariant and fix the underlying builder/checkpoint mismatch.

2. Carry warnings all the way through the compile APIs.
   The collection logic already exists. The missing work is result-shape plumbing and tests.

3. Finish artifact-store compile-result reuse.
   This is one of the highest-payoff engineering tasks left because it converts infrastructure into faster iteration.

4. Keep decomposing the giant files.
   Prioritize `infer_chirho.rs`, `lower_chirho.rs`, `eval_chirho.rs`, and the huge test monoliths.

5. Add more contract tests around runtime boundaries.
   Especially GC roots, exception unwinding, constructor layout, and forced-vs-lazy behavior.

6. Keep documentation synchronized with the actual execution paths.
   The repo is good enough now that stale docs will mislead people more than missing docs.

## Bottom Line

Rhasky Chirho is in a much stronger state than an older snapshot would suggest.

The workspace is green. The pipeline is real. The architecture is increasingly coherent. The codebase already contains enough compiler, runtime, and tooling substance to justify calling it a serious under-development Haskell compiler in Rust.

The remaining work is not basic existence work. It is the harder and more valuable work: hardening invariants, tightening interfaces, improving reuse, and keeping a fast-moving compiler coherent as it grows.
