<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Rhasky Chirho Codebase Analysis

## Snapshot

- Workspace size: 19 crates, about 79,235 lines of Rust under `crates/`.
- Test surface: about 1,104 `#[test]` cases. Test volume is strongest in `rhasky-driver-chirho` (589), `rhasky-typing-chirho` (142), and `rhasky-core-chirho` / `rhasky-parser-chirho` / `rhasky-runtime-chirho` (65 each).
- Property testing: I found no active `proptest!` or `quickcheck!` usage.
- Current repo state: dirty working tree. `git status --short` shows modified `Cargo.toml`, `Cargo.lock`, `spec-chirho/progress-chirho.sqlite`, plus untracked backend crates for BEAM, Cranelift, and JVM.
- Verification run:
  - `cargo test --workspace --quiet`
  - targeted `cargo test -p rhasky-driver-chirho let_expression_infers_chirho -- --exact --nocapture`

## Overall Assessment

This is no longer a bootstrap compiler skeleton. The codebase already has a real multi-phase pipeline, a lossless green CST, non-trivial Haskell surface support, a substantial HM/typeclass implementation, a Core language, an STG-style runtime/evaluator, package and incremental subsystems, and broad test coverage.

The strongest pattern is ambition plus breadth. The weakest pattern is drift: the code, tests, workspace manifest, and PRD are moving at different speeds. That drift is now large enough to create correctness risk, broken workspace builds, and duplicated phase logic.

## Highest-Priority Findings

### 1. The workspace is not green

`cargo test --workspace --quiet` currently fails.

- `crates/rhasky-backend-beam-chirho/src/beam_module_chirho:153`, `:169`, `:181`
  - `CoreModuleChirho` initializers are missing the now-required `name_chirho` field.
- `crates/rhasky-backend-jvm-chirho/src/class_chirho:96`, `:112`
  - same schema drift.
- `crates/rhasky-backend-cranelift-chirho/src/codegen_chirho:110-111`
  - the crate uses `iconst` and `return_` without importing Cranelift's `InstBuilder` trait, so the library itself does not compile.
- `crates/rhasky-backend-cranelift-chirho/src/codegen_chirho:149`, `:163`, `:183`
  - more `CoreModuleChirho` initializers missing `name_chirho`.

Impact:

- The repo cannot currently claim workspace-level correctness.
- Backend additions are landing in the workspace before they are integration-safe.

Recommendation:

- Either fix these crates immediately or gate them behind features / remove them from default workspace verification until they compile and have passing smoke tests.

### 2. `check` is not actually a compiler check

There are two materially different front doors in the driver:

- `crates/rhasky-driver-chirho/src/lib.rs:64-86`
  - `check_source_file_chirho` only calls `parse_source_file_chirho`, extracts a module name, and emits legacy LLVM/Wasm stubs.
- `crates/rhasky-driver-chirho/src/lib.rs:206-270`
  - `compile_source_chirho` runs the real pipeline: CST parse, lower, deriving, name resolution, kind inference, type inference, exhaustiveness, Core lowering, dictionary pass, simplification, and backend generation.

That means the CLI `check` / `plan` path is not validating the same thing the compiler pipeline validates.

Impact:

- User-facing commands can report success on code that would fail in the real parser, typechecker, or backend path.
- Bugs found through `compile_source_chirho` are invisible to the `check` path.

Recommendation:

- Make `check_source_file_chirho` a thin wrapper around the real phase pipeline and downgrade the old module-header-only path to an internal helper or remove it entirely.

### 3. The public parser entry point in `rhasky-parser-chirho` is stale and semantically weak

`crates/rhasky-parser-chirho/src/lib.rs:26-128` still exposes a string-based module-header parser that:

- only skips blank lines and `--` comments (`:41-47`)
- does not handle block comments, pragmas, or other leading trivia
- accepts header text by `starts_with("module ")` and `strip_suffix("where")` (`:86-97`)

That parser will mis-handle valid Haskell with leading pragmas/comments, and it also accepts malformed text such as a header ending in `where` without requiring token boundaries.

Impact:

- The crate's top-level API no longer matches the real parser capability.
- Anything calling `parse_source_file_chirho` is getting a partial and misleading result.

Recommendation:

- Replace this API with the real CST parser or rename it to something honest like a header scanner and stop using it as a compiler entry point.

### 4. The PRD and the workspace no longer describe the same system

The manifest and the PRD have drifted:

- `Cargo.toml` lists actual crates like `rhasky-naming-chirho`, `rhasky-typing-chirho`, `rhasky-package-chirho`, `rhasky-incremental-chirho`, and the three new backend crates.
- `spec-chirho/prd-chirho.json:271-345` still describes older crate names such as `rhasky-hir-chirho`, `rhasky-namer-chirho`, `rhasky-types-chirho`, `rhasky-typecheck-chirho`, and `rhasky-package-db-chirho`.
- `spec-chirho/prd-chirho.json:8` still says `"current_milestone_chirho": "M0"` while `spec-chirho/prd-chirho.json:379-394` says `M0` is complete and `M1` is in progress.

Impact:

- The spec is no longer reliable as an architecture map or milestone tracker.
- Contributors can make "correct" decisions against the PRD and still diverge from the real codebase.

Recommendation:

- Reconcile the PRD against the actual workspace before adding more crates or milestones. The spec should describe the implementation that exists now, not the one from several design iterations ago.

### 5. Phase sequencing is duplicated in the driver, and warning handling is inconsistent

The driver repeats large chunks of pipeline logic in multiple paths. One visible consequence:

- `crates/rhasky-driver-chirho/src/lib.rs:223-224`
  - deriving warnings are computed and immediately discarded.
- `crates/rhasky-driver-chirho/src/lib.rs:247-254`
  - exhaustiveness warnings are noted as non-fatal but not unified into returned diagnostics.
- `crates/rhasky-driver-chirho/src/lib.rs:345-347`
  - the package/multi-module path uses `infer_module_with_imports_chirho`.
- `crates/rhasky-driver-chirho/src/lib.rs:242`
  - the single-module path uses `infer_module_chirho`.

Inference from structure:

- The project already knows it needs an import-aware typing path, but the single-source path still uses the simpler API.
- Duplicated orchestration is likely to keep drifting in exactly this way.

Recommendation:

- Move phase orchestration into one reusable pipeline builder or coordinator struct that always returns one unified diagnostic collection and takes configuration for imports/execution mode.

### 6. The token model is duplicated and currently inconsistent

There are two token enums:

- `crates/rhasky-parser-chirho/src/lexer_chirho.rs`
  - `RawTokenKindChirho`
- `crates/rhasky-syntax-chirho/src/token_chirho.rs`
  - `TokenKindChirho`

The mapper in `crates/rhasky-parser-chirho/src/cst_parser_chirho.rs:33-66` includes this line:

- `RawTokenKindChirho::QualifiedIdChirho => TokenKindChirho::QualifiedConIdChirho`

But the syntax layer also defines `QualifiedVarIdChirho`.

Inference from code structure:

- Qualified value identifiers appear to be collapsed too early in the raw lexer and then reclassified as constructor-qualified tokens in the CST mapping.
- Even if the current parser mostly survives this, it is the wrong long-term abstraction boundary.

Recommendation:

- Either lex qualified identifiers into value-vs-constructor forms up front, or make the syntax layer preserve the ambiguity until lowering/name resolution.

### 7. Export-list semantics are still absent from lowering

`crates/rhasky-parser-chirho/src/lower_chirho.rs:203-205` still emits:

- `exports_chirho: None // TODO: lower export list`

Impact:

- Interface construction, package visibility, and import/export correctness are incomplete.
- This blocks meaningful progress toward real Cabal/Hackage compatibility even if parsing and typing improve.

Recommendation:

- Treat export list lowering as a near-term requirement, not a late parser TODO.

## Architectural Analysis By Area

### Foundation Crates

`rhasky-span-chirho`, `rhasky-diagnostics-chirho`, `rhasky-syntax-chirho`, and `rhasky-test-harness-chirho` are the cleanest part of the repo. They are focused, portable, and broadly aligned with the intended architecture. The span/diagnostic model is strong enough to support CLI, IDE, and REPL rendering later.

The green-tree layer is especially valuable. That was the right investment. It gives you a stable lossless surface for parser and tooling work.

### Parser / CST / AST Boundary

The real parser is substantially ahead of what the crate-level docs imply. The CST parser, layout logic, and lowerer cover a meaningful Haskell subset and already have golden tests. The problem is not lack of functionality; it is split authority.

Right now there is a "real parser stack" and a "legacy header parser". That split should end. The lowerer is also getting too large for safe maintenance at 4,800 lines, and its responsibilities include feature lowering, AST shaping, and pragma extraction all in one place.

### Naming / Typing

This is ambitious and already useful. The typing crate has real breadth: kind inference, type inference, typeclasses, deriving, and exhaustiveness. That is a strong foundation.

The main risk here is monolithic complexity. `infer_chirho.rs` is 6,471 lines and `class_chirho.rs` is 1,891. That makes localized reasoning hard, especially for subtle solver behavior. The presence of both `infer_module_chirho` and `infer_module_with_imports_chirho` is good design, but the driver needs to stop using them inconsistently.

### Core / Runtime / Driver

This is where capability and risk are both highest.

- `crates/rhasky-core-chirho/src/dict_chirho.rs` is 16,567 lines.
- `crates/rhasky-driver-chirho/src/lib.rs` is 12,471 lines.
- `crates/rhasky-runtime-chirho/src/eval_chirho.rs` is 2,986 lines.

Those file sizes are no longer a style issue; they are an architectural issue. The driver is acting as:

- compiler orchestrator
- package loader
- builtin module provider
- evaluator harness
- integration test container
- partial STG layer host

That concentration will slow every future change, especially when backend and runtime semantics keep evolving.

### Package / Incremental

These crates are comparatively disciplined. `rhasky-package-chirho` and `rhasky-incremental-chirho` have focused APIs and reasonable internal boundaries. They look more maintainable than the parser/core/driver center.

The main issue is integration: the implementation has outgrown the PRD, and the package / incremental story is further along than the spec says.

### Backends

LLVM and Wasm are usable as early-stage emitters, but their current implementations are still subset code generators from Core, not the eventual laziness-preserving backend story described in the PRD.

BEAM, JVM, and Cranelift are not yet workspace-stable. They read as promising scaffolds rather than production compiler targets. They should be managed that way.

## Quality, Testing, and Compliance

### Test Posture

The quantity of tests is good. The distribution is less ideal.

- Driver tests dominate the tree, which is useful for integration confidence but can hide subsystem regressions inside one giant file.
- There are currently no active property-test suites even though the project explicitly wants them.
- The most important missing classes are:
  - parser/layout malformed-input properties
  - simplifier semantic-preservation checks
  - dictionary-pass invariants
  - runtime evaluator step/heap invariants
  - backend round-trip smoke tests that execute emitted artifacts where possible
  - differential tests against GHC for syntax/typechecker edge cases

### Warning Debt

The warning profile is now meaningful, not cosmetic.

- runtime: unreachable pattern and unused label
- parser: dead helper plus unused variables
- core: dead code and many unused variables in large transformation files
- driver: unused imports, unused variables, and naming-style warnings

The codebase would benefit from treating warnings as backlog items rather than normal background noise.

### Naming / Header Compliance

Automated scans found three files missing the John 3:16 header near the top:

- `CLAUDE.md`
- `crates/rhasky-incremental-chirho/Cargo.toml`
- `spec-chirho/prd-chirho.json`

I also found naming drift in places that are meant to follow the repo convention:

- Rust test names in `crates/rhasky-driver-chirho/src/lib.rs:9158`, `:9218`, `:9369`, `:12457` use `mapM` / `forM` mixed-case forms and trigger `non_snake_case` warnings.
- Embedded Haskell test snippets in `crates/rhasky-driver-chirho/src/lib.rs:10647` and `:12198` define `IntList` and `Age` without the required `Chirho` suffix.

Most other apparent scan hits were trait methods like `fmt`, `default`, and `from`, which are dictated by Rust traits and should be treated as unavoidable exceptions.

## Strengths Worth Preserving

- The crate split is directionally correct.
- The span + diagnostic foundation is strong.
- The green CST is the right long-term parser architecture.
- The typing and runtime work are already beyond "toy compiler" level.
- Package and incremental crates are relatively clean and reusable.
- The codebase already has enough test mass to support real refactoring if the workspace is kept green.

## Recommended Sequence

1. Restore a green workspace.
   - Fix or gate BEAM/JVM/Cranelift immediately.
2. Collapse to one real driver pipeline.
   - `check`, `compile`, package compilation, and script mode should share one orchestrator.
3. Remove or quarantine the legacy header parser.
   - The parser crate should expose the real parser stack, not a bootstrap remnant.
4. Reconcile the PRD with the actual workspace.
   - Crate names, milestones, and current status need one source of truth.
5. Split the biggest files.
   - Start with `driver/lib.rs`, `core/dict_chirho.rs`, `typing/infer_chirho.rs`, and `parser/lower_chirho.rs`.
6. Add property and differential tests where invariants matter most.
   - Parser/layout, simplifier, dictionary pass, runtime evaluator.
7. Finish export/import semantics before claiming serious package compatibility progress.

## Bottom Line

The project already contains the core of a real Haskell compiler in Rust, not just a scaffold. The current limiting factor is no longer lack of features. It is system coherence: green builds, one authoritative pipeline, one authoritative spec, and smaller phase-local modules with explicit invariants.

If those are fixed, the existing breadth becomes a major asset. If they are not, the current rate of accretion will keep converting working subsystems into a hard-to-maintain monolith.
