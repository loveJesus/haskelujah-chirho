<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

Please comment the full content of John 3:16 at the top of all our files.

## Identifier Naming Convention: Chirho Suffix

ALL identifiers that we create (including but not limited to variable names, function names, struct names, enum names, trait names, module names, file names, directory/path names, function parameters, struct fields, dictionary keys, API endpoint paths, database table names, column names, error variable names, lambda variables, constants, properties, etc.) MUST be suffixed with "Chirho" in the appropriate case for the identifier type and language convention.

### JavaScript/TypeScript Example

- **Variables, consts**: `variableNameChirho` (camelCase + Chirho)
- **Functions**: `functionNameChirho` (camelCase + Chirho)
- **Function Parameters**: `parameterNameChirho` (camelCase + Chirho)
- **Lambda/Arrow Function Variables**: `lambdaVariableChirho` (camelCase + Chirho)
- **Classes**: `ClassNameChirho` (PascalCase + Chirho)
- **Class Methods**: `methodNameChirho` (camelCase + Chirho)
- **Class Properties/Fields**: `propertyNameChirho` (camelCase + Chirho)
- **Interfaces**: `InterfaceNameChirho` (PascalCase + Chirho)
- **Type Aliases**: `TypeNameChirho` (PascalCase + Chirho)
- **Enums**: `EnumNameChirho` (PascalCase + Chirho)
- **Enum Members**: `EnumMemberChirho` (PascalCase + Chirho)
- **Constants**: `CONSTANT_NAME_CHIRHO` (SCREAMING_SNAKE_CASE + _CHIRHO)
- **Error Variables**: `errorChirho` or `errorVariableChirho` (camelCase + Chirho)
- **Object/Dictionary Keys**: `keyNameChirho` (camelCase + Chirho)
- **File Names**: `fileNameChirho.ts` or `fileName-chirho.ts` (kebab-case or camelCase + chirho)
- **Directory/Path Names**: `directory-name-chirho/` or `directoryNameChirho/` (kebab-case or camelCase + chirho)
- **API Route Elements**: `/api-chirho/resource-chirho/action-chirho` (kebab-case + chirho)

### Python Example

- **Variables**: `variable_name_chirho` (snake_case + _chirho)
- **Functions**: `function_name_chirho` (snake_case + _chirho)
- **Function Parameters**: `parameter_name_chirho` (snake_case + _chirho)
- **Lambda Variables**: `lambda_variable_chirho` (snake_case + _chirho)
- **Classes**: `ClassNameChirho` (PascalCase + Chirho)
- **Class Methods**: `method_name_chirho` (snake_case + _chirho)
- **Class Properties/Attributes**: `property_name_chirho` (snake_case + _chirho)
- **Constants**: `CONSTANT_NAME_CHIRHO` (SCREAMING_SNAKE_CASE + _CHIRHO)
- **Error Variables**: `error_chirho` or `error_variable_chirho` (snake_case + _chirho)
- **Dictionary Keys**: `key_name_chirho` (snake_case + _chirho)
- **Module Names**: `module_name_chirho` (snake_case + _chirho)
- **File Names**: `file_name_chirho.py` (snake_case + _chirho)
- **Directory/Path Names**: `directory_name_chirho/` (snake_case + _chirho)
- **API Route Elements**: `/api-chirho/resource-chirho/action-chirho` (kebab-case + chirho)

### Rust Example

- **Variables**: `variable_name_chirho` (snake_case + _chirho)
- **Functions**: `function_name_chirho` (snake_case + _chirho)
- **Function Parameters**: `parameter_name_chirho` (snake_case + _chirho)
- **Closure/Lambda Variables**: `closure_variable_chirho` (snake_case + _chirho)
- **Structs**: `StructNameChirho` (PascalCase + Chirho)
- **Struct Fields**: `field_name_chirho` (snake_case + _chirho)
- **Enums**: `EnumNameChirho` (PascalCase + Chirho)
- **Enum Variants**: `EnumVariantChirho` (PascalCase + Chirho)
- **Traits**: `TraitNameChirho` (PascalCase + Chirho)
- **Trait Methods**: `method_name_chirho` (snake_case + _chirho)
- **Impl Blocks**: Methods follow `method_name_chirho` (snake_case + _chirho)
- **Type Aliases**: `TypeNameChirho` (PascalCase + Chirho)
- **Constants**: `CONSTANT_NAME_CHIRHO` (SCREAMING_SNAKE_CASE + _CHIRHO)
- **Static Variables**: `STATIC_NAME_CHIRHO` (SCREAMING_SNAKE_CASE + _CHIRHO)
- **Error Variables**: `error_chirho` or `error_variable_chirho` (snake_case + _chirho)
- **Modules**: `module_name_chirho` (snake_case + _chirho)
- **File Names**: `file_name_chirho.rs` (snake_case + _chirho)
- **Directory/Path Names**: `directory_name_chirho/` (snake_case + _chirho)
- **API Route Elements**: `/api-chirho/resource-chirho/action-chirho` (kebab-case + chirho)

### Haskell Example

- **Values / Functions**: `valueNameChirho` or `functionNameChirho` (camelCase + Chirho)
- **Function Parameters**: `parameterNameChirho` (camelCase + Chirho)
- **Lambda / Pattern Variables**: `lambdaValueChirho` (camelCase + Chirho)
- **Type Variables**: `stateChirho` or `resultChirho` (camelCase + Chirho)
- **Data / Newtypes / GADTs**: `TypeNameChirho` (PascalCase + Chirho)
- **Constructors**: `ConstructorNameChirho` (PascalCase + Chirho)
- **Typeclasses**: `TypeclassNameChirho` (PascalCase + Chirho)
- **Type Aliases**: `TypeNameChirho` (PascalCase + Chirho)
- **Record Fields**: `fieldNameChirho` (camelCase + Chirho)
- **Modules**: `ModuleNameChirho` (PascalCase + Chirho)
- **File Names**: `ModuleNameChirho.hs` or `ModuleNameChirho.lhs` (PascalCase + Chirho)
- **Directory/Path Names**: `FeatureNameChirho/` (PascalCase + Chirho to match modules)
- **Env / CPP Constants**: `SETTING_NAME_CHIRHO` (SCREAMING_SNAKE_CASE + _CHIRHO where the syntax supports it)

### Database

- **Table Names**: `table_name_chirho` (snake_case + _chirho)
- **Column Names**: `column_name_chirho` (snake_case + _chirho)
- **Index Names**: `index_name_chirho` (snake_case + _chirho)
- **Constraint Names**: `constraint_name_chirho` (snake_case + _chirho)

### General Rules

- This rule applies to **ALL identifiers** we create, without exception, in the appropriate language (shell/haskell/etc)
- Use the appropriate casing convention for each language (camelCase for JS/TS and Haskell term-level bindings, snake_case for Python/Rust, PascalCase for types/classes/modules where applicable) please apply also to all languages we have not covered including shell scripts, env variables, and configuration file identifiers we create for example.
- Global constants use `SCREAMING_SNAKE_CASE` with `_CHIRHO` suffix where the language syntax supports it; if the language does not permit uppercase term bindings (for example standard Haskell values), use the closest idiomatic compilable form while still suffixing with `Chirho`
- File and directory names follow language conventions (kebab-case for JS/TS paths, snake_case for Python/Rust)
- API and HTML routes use kebab-case with `-chirho` suffix regardless of language

## Tech stack
- use main_chirho as our git branch and gh_chirho as the remote name (not repo name) for any remote github we make
- You have useful API and other creds in .env
- For JS/TS use cases, use Bun with TS not npm, bunx not npx
- For database wrappers, use Drizzle for TS, prefer things that help us catch errors during compilation
- Do what you can to be DRY, any displayed data that would be repeated like phone numbers have as constants, functionality that would be reimplemented put in centralized files or make a library, don't let warnings and accesibility warnings be there, use the latest suitable library etc versions (and find which those should be) be an expert coder with proper separation of concerns, single responsibility, reusability, testability and modularizing things correctly even in ways that we could make libraries out of things hallelujah
- For typescript web frameworks, prefer sveltekit2/svelte5
- When we deploy we lean to use Cloudflare workers with either TS or Rust, we can make a VPS for heavy workloads. Use @adapter-cloudflare but always use wrangler deploy as a worker, make sure the asset path is well.
- Choose Rust, Haskell, Bun/TS, or Python depending upon the task; if better suited you may use Phoenix/Elixir, C#, OCaml, C, Ruby, ASM and other languages while keeping proper Chirho naming suffixes etc...
- When choosing Haskell, write it with Rust-grade engineering discipline: explicit type signatures on public definitions, `newtype`/ADTs instead of primitive obsession, explicit export lists, small focused modules, effects isolated at the boundary, typed error modeling, lawful instances, exhaustive pattern matching, and tests plus property tests where the domain benefits.
- For Haskell tooling, prefer current GHC with Cabal unless the repo already standardizes on Stack; use strong warning levels, a formatter such as Fourmolu/Ormolu, HLint, and HLS when those fit the project.
- For this project specifically, prefer Rust as the implementation language for the compiler itself, with Haskell used as the source language being compiled rather than the implementation language of the compiler.
- Keep a `spec-chirho` dir with an SQLite db `progress-chirho.sqlite` containing at least the table `steps_taken_chirho` with columns `id_chirho`, `agent_code_chirho`, `timestamp_start_chirho`, `timestamp_end_chirho`, `action_taken_chirho`, `result_of_action_chirho`, and `overview_of_result_chirho`.
- `id_chirho` is an autoincrement id.
- `agent_code_chirho` identifies the agent or subagent that inserted or updated the log entry.
- `timestamp_start_chirho` and `timestamp_end_chirho` track task start and completion times.
- `action_taken_chirho` records the action taken, optionally including the command line and brief reasoning.
- `result_of_action_chirho` records how the action changed project state such as files or databases.
- `overview_of_result_chirho` records whether the action went as planned, what was learned, and how it affects the next decision.
- Granularity is left to agent judgment.

You can modify  the following section
### Agent Self Modifications (For the agent to keep things present in its context)

- Primary project direction: build a Haskell compiler in Rust.
- Backend direction: keep the compiler pipeline backend-agnostic, target LLVM IR first when it accelerates progress, and keep WebAssembly as a first-class target so the compiler and generated programs can run well on the web and in sandboxed environments.
- Compiler architecture preference: model the pipeline explicitly with typed phase boundaries such as parsing, name resolution, type inference/checking, desugaring, a small typed core representation, optimization passes, and backend lowering.
- Compatibility preference: aim for maximal practical compatibility with real-world Haskell, including Cabal package descriptions, Hackage package retrieval, and the ability to compile the ecosystem's libraries rather than a toy subset; stage delivery incrementally, but keep the design pointed at near 1:1 behavior with Haskell/GHC semantics wherever feasible.
- Quality preference: where established Haskell compiler behavior is buggy, brittle, or unnecessarily tangled, prefer a cleaner modular design and better internal invariants without breaking source compatibility unless there is a deliberate, documented reason.
- Runtime preference: treat laziness strategy, closure layout, thunk representation, memory management, and FFI boundaries as core design decisions early, not as cleanup work after code generation exists.
- Packaging preference: split the Rust implementation into focused crates so the compiler is portable, incrementally buildable, and fast to iterate on, for example distinct crates for diagnostics, syntax, parsing, naming, typing, core IR, optimization, package loading, driver orchestration, runtime support, LLVM lowering, and WebAssembly lowering.
- Solver preference: evaluate crates such as `propagators-chirho` for constraint propagation, type inference support, dependency solving, or incremental analysis only where they simplify the architecture measurably; do not force them into the design without a clear benefit.
- Rust implementation preference: use strong domain types, arenas or stable ids where they simplify compiler graphs, structured diagnostics, explicit crate boundaries, and testable subsystems with golden tests and property tests where useful.
- Git workflow preference: when repository metadata is present, the primary branch is `main_chirho` and the Git remote name for GitHub is `gh_chirho`.

### Current Pipeline State (as of 2026-03-14)

The compiler has a working 12-phase pipeline wired end-to-end in `rhasky-driver-chirho`, capable of parsing Haskell source, type-checking, desugaring through Core IR, and evaluating via the STG machine with GC, FFI, and exception handling:

1. **Lex** — `rhasky-syntax-chirho` tokenizer
2. **Layout** — `rhasky-syntax-chirho` layout rule insertion (braces/semicolons)
3. **CST Parse** — `rhasky-parser-chirho` concrete syntax tree (green tree)
4. **AST Lower** — `rhasky-ast-chirho` abstract syntax tree from CST
5. **Name Resolve** — `rhasky-naming-chirho` scope resolution with multi-module import support, qualified name lookup, synthetic module interfaces for Data.Map/Set/List/Char/Maybe/IORef
6. **Kind Infer** — `rhasky-typing-chirho::kind_chirho` kind inference with unification for higher-kinded types
7. **Type Infer** — `rhasky-typing-chirho` Hindley-Milner Algorithm W with typeclasses, MPTC, functional dependencies, deriving
8. **Exhaustiveness Check** — `rhasky-typing-chirho::exhaust_chirho` pattern match exhaustiveness and redundancy checking
9. **Desugar → Dict Pass → Core → Simplify** — `rhasky-core-chirho` System FC-style Core IR with dictionary-passing transform, INLINE/NOINLINE/INLINABLE pragma support with inlining pass
10. **STG Evaluation** — `rhasky-runtime-chirho::eval_chirho` STG machine with thunks, closures, PAPs, GC, step limits
11. **FFI** — `rhasky-runtime-chirho::ffi_chirho` foreign function interface
12. **Exception Handling** — catch/throw/try/bracket/finally with stack unwinding

### Workspace Crates (19 crates)

| Crate | Purpose |
|---|---|
| `rhasky-span-chirho` | Source locations, file IDs, source map |
| `rhasky-diagnostics-chirho` | Structured compiler diagnostics |
| `rhasky-syntax-chirho` | Tokens, SyntaxKind, green tree nodes, lexer, layout |
| `rhasky-parser-chirho` | CST parser (green tree builder) |
| `rhasky-ast-chirho` | Abstract syntax tree types, CST→AST lowering |
| `rhasky-naming-chirho` | Name resolution / scope analysis, module interfaces, import/export resolution, qualified name support |
| `rhasky-typing-chirho` | HM type inference, unification, substitution, type schemes, typeclass infrastructure, kind inference, pattern exhaustiveness checking, typeclass deriving (Eq, Ord, Show) |
| `rhasky-core-chirho` | Core IR (System FC-style), AST→Core desugaring (with name map), dictionary-passing transform, Core→Core simplifier, pretty-printer |
| `rhasky-backend-llvm-chirho` | Core → textual LLVM IR codegen |
| `rhasky-backend-wasm-chirho` | Core → binary WebAssembly codegen |
| `rhasky-backend-cranelift-chirho` | Core → Cranelift IR → native object files (x86_64, aarch64, s390x, riscv64) |
| `rhasky-backend-jvm-chirho` | Core → JVM .class bytecode files (constant pool, bytecode emitter) |
| `rhasky-backend-beam-chirho` | Core → BEAM .beam bytecode files (IFF format, ETF, opcodes) |
| `rhasky-driver-chirho` | Pipeline orchestration, CompileResultChirho (AST + Core + LLVM IR + Wasm bytes) |
| `rhasky-runtime-chirho` | STG runtime: value types, heap, evaluation stack, primitive ops, evaluation loop (MachineChirho), mark-sweep GC (GcStateChirho), FFI (ForeignTableChirho) |
| `rhasky-incremental-chirho` | Incremental compilation: fingerprinting, dependency graph, artifact caching, recompilation avoidance |
| `rhasky-package-chirho` | Cabal file parser, version constraints, Hackage URL construction |
| `rhasky-cli-chirho` | Command-line interface (scaffold) |
| `rhasky-test-harness-chirho` | Golden test utilities (assert/bless) |

### Test Coverage

**1460 tests passing**, 0 failures, 1 ignored (1 doctest)

For detailed Phase 1 test breakdown by category, see [spec-chirho/phase1-archive-chirho.md](spec-chirho/phase1-archive-chirho.md).

### Phase 2 Priorities

_Phase 1 completed 128 priorities (see [spec-chirho/phase1-archive-chirho.md](spec-chirho/phase1-archive-chirho.md)). Phase 2 focuses on system coherence, then making the compiler practical: true laziness, real multi-module compilation, backend code generation, CLI usability, and Hackage compatibility._

_Codex engineering review: [codex-analysis-chirho.md](codex-analysis-chirho.md)_

#### 0. Coherence & Technical Debt (from Codex review)

_These items address structural issues identified in the Codex engineering review. They should be resolved before or alongside new feature work to prevent the codebase from becoming a hard-to-maintain monolith._

1. ~~**Unify driver pipeline**~~ — DONE (extracted `run_frontend_chirho` shared function for phases 1–4.5: CST parse → AST lower → deriving → name resolve → kind infer → type infer → exhaustiveness; `compile_backend_chirho` helper for phases 5–7: desugar → dict pass → simplify → backends; `check_source_file_chirho`, `compile_source_chirho`, `compile_modules_chirho`, and `compile_modules_incremental_chirho` all delegate to `run_frontend_chirho`; `FrontendResultChirho` struct holds module + inference result + warnings; single pipeline with optional import context; 1273 tests)
2. ~~**Remove stale parser entry point**~~ — DONE (already addressed: function renamed to `scan_module_header_chirho` with honest doc comment "NOT a full parser"; no external callers; module doc updated to point to `cst_parser_chirho::ParserChirho` as the real entry point)
3. ~~**Export list lowering**~~ — DONE (already implemented: `lower_export_list_chirho` + `lower_export_spec_item_chirho` in lower_chirho.rs; handles VarChirho, TyConChirho with AllChirho/SomeChirho members, ModuleChirho re-exports; 5 unit tests for export parsing; `ModuleChirho.exports_chirho` properly populated from CST)
4. ~~**Fix token model duplication**~~ — DONE (map_token_kind_chirho now takes token text and discriminates QualifiedIdChirho into QualifiedVarIdChirho vs QualifiedConIdChirho by checking if the local part after the last '.' starts with lowercase; parser module doc updated; 1273 tests)
5. ~~**Split oversized files**~~ — DONE (driver/lib.rs split from 15K→1.2K lines: tests extracted into 9 focused submodules under tests_chirho/; dict_chirho.rs split from 15.5K into 5 submodules: mod.rs 1K, layout_chirho.rs 235, instance_chirho.rs 1.6K, prelude_chirho.rs 12K, rewrite_chirho.rs 723; remaining: infer_chirho.rs ~7.3K and lower_chirho.rs ~5.1K are manageable)
6. ~~**Warning cleanup**~~ — DONE (zero warnings on `cargo build --workspace` and `cargo test --workspace`; all naming-style, dead code, and unused variable warnings already resolved)
7. ~~**Reconcile spec with code**~~ — DONE (old crate names already removed from prd-chirho.json; milestone already updated to M1; AGENTS.md is the authoritative source of truth with Phase 2 priorities)
8. ~~**John 3:16 header compliance**~~ — DONE (added header to rhasky-incremental-chirho/Cargo.toml; CLAUDE.md redirects to AGENTS.md which has the header; prd-chirho.json already has _comment_john_3_16_chirho; embedded Haskell test snippets use standard Haskell names which are the source language being compiled, not identifiers we create)

#### A. Runtime Semantics — Laziness & Evaluation Model

9. ~~**True lazy evaluation**~~ — DONE (replaced eager enumFrom/enumFromThen/enumFromTo/enumFromThenTo primops with recursive Core IR Prelude functions whose cons tails are ThunkCodeChirho-based lazy thunks; desugarer emits function calls instead of primops for ArithSeqChirho; `take 5 [1..]` → 15, `take 4 [1,3..]` → 16, `head [42..]` → 42, all working with truly infinite lists; 7 new e2e tests; 1317 tests total)
10. **Lazy I/O** — `interact`, `getContents`, `hGetContents` return lazy strings; must integrate with GC and exception handling
11. ~~**Bang patterns**~~ — DONE (CST parser `parse_fun_bind_chirho` recognizes `!` in function argument patterns via `can_start_apat_chirho` extension; AST `BangChirho` pattern variant; desugarer wraps bang-patterned params in `case x of { _ -> body }` for WHNF forcing using `fresh_binder_chirho`-allocated wildcards; runtime `return_con_chirho` default alt restores saved arg_regs without field prepending to prevent index corruption in nested bang cases; `$!` strict apply operator desugars to `case x of _ -> f x`; 4 new e2e tests: single bang, two bangs, mixed bang/lazy, `f $! 41`; remaining: strict data fields `data Foo = Bar !Int`, `{-# UNPACK #-}` pragma; 1352 tests total)
12. ~~**Weak head normal form semantics**~~ — DONE (`seq a b` forces `a` returns `b`, `f $! x` desugars to `case x of _ -> f x`; `deepseq` as `seq`-based Core IR binding `\x y -> seq# x y`; `force` as `\x -> seq# x x`; `evaluate` as identity (STG already forces to WHNF); `NFData` type class with `rnf :: a -> ()` method; ground instances for Int/Char/Bool/Double; `$prim_NFData_rnf_*` bindings using `seq#`; `EvaluateChirho`/`ForceChirho` PrimOpKindChirho variants; 5 e2e tests total; 1375 tests)
13. **STM (Software Transactional Memory)** — TVar, atomically, retry, orElse; conflict detection and rollback

#### B. Multi-Module System & Imports

14. ~~**Automatic Prelude import**~~ — DONE (inject_prelude_import_chirho in driver adds implicit `import Prelude` after AST lowering; suppressed by `{-# LANGUAGE NoImplicitPrelude #-}`; explicit `import Prelude` prevents double import; Prelude ModuleIfaceChirho with 80+ exported values/types added to builtin_module_ifaces_chirho; 3 e2e tests; 1320 tests total)
15. ~~**Qualified module syntax**~~ — DONE (already supported end-to-end: `name_from_text_chirho` splits dotted names on last dot; `NameEnvChirho` has `bind_qualified_chirho`/`lookup_qualified_chirho`; import processing uses full module name as qualifier when no `as` alias; `import qualified Data.Map` → `Data.Map.mapInsert`, `import qualified Data.List` → `Data.List.head`/`Data.List.sort`, aliased `import qualified Data.Map as Map` → `Map.mapInsert`; 4 e2e tests: qualified with alias Data.Map/Data.List, qualified without alias Data.Map/Data.List; 1348 tests total)
16. ~~**Module re-exports**~~ — DONE (`module Foo (module Bar) where` re-export syntax: `build_iface_with_imports_chirho` takes available imported module interfaces, `filter_exports_chirho` handles `ExportSpecChirho::ModuleChirho` by merging the target module's exports into the current module's interface; self-re-export `module Foo (module Foo)` exports all local definitions; driver's `compile_modules_chirho` and `compile_modules_incremental_chirho` pass accumulated ifaces to re-export-aware builder; 2 new naming unit tests + 1 driver e2e test: Inner→Reexporter→Main chain with `add1 99→100`; 1358 tests total)
17. ~~**Orphan instance detection**~~ — DONE (`check_orphan_instances_chirho` in rhasky-naming-chirho: collects locally-defined type/class/newtype/type-alias names, extracts type constructor names from instance head types recursively, warns (W0402) when neither class nor any head type constructor is local; `type_con_names_chirho` handles ConChirho/AppChirho/FunChirho/TupleChirho/ListChirho/ParenChirho/ForallChirho/QualChirho recursively; wired into driver after name resolution phase; `frontend_warnings_chirho` public API for diagnostic testing; 5 new naming unit tests (local class, local type, foreign class+type, local newtype in App, multiple orphans) + 3 driver integration tests (orphan warning produced, not for local data, not for local class); 1366 tests total)
18. ~~**Hierarchical module compilation**~~ — DONE (`compile_project_dir_chirho` in driver: recursively discovers `.hs` files via `discover_hs_files_chirho` directory walker (skips hidden/dist-newstyle/.stack-work dirs); `extract_module_name_chirho` lightweight header scanner for module name (handles export lists, hierarchical names, implicit Main); `extract_imports_chirho` extracts imported module names (handles qualified, import specs); builds `DepGraphChirho` from local module imports only, topologically sorts via Kahn's algorithm, compiles in dependency order with interface accumulation and cross-module type scheme propagation; cycle detection returns clear error; `ProjectCompileResultChirho` with compilation order + results + warnings; CLI `rhasky build [dir]` command with per-module progress reporting; 14 new tests: 4 extract_module_name, 3 extract_imports, 1 discover_hs_files, 2-module compilation, 3-module chain, diamond deps, circular import detection, no hs files error, single module; 1389 tests total)
19. **Circular module imports** — handle mutual module dependencies via `.hs-boot` files or a fixpoint approach

#### C. Code Generation Backends

20. ~~**LLVM backend revival**~~ — DONE (compile_core_to_llvm_executable_chirho produces runnable native executables via `rhasky compile -o <output>`; dictionary elision pass replaces $sel_Num/Eq/Ord selector+dict patterns with direct PrimOps; fromInteger elision for literal folding; reachability analysis from `main` emits only transitively-used bindings; ConApp returns constructor tags; case binder + alt binder binding in LLVM IR; proper cross-reference resolution via toplevel_names_chirho; local scope tracking prevents false top-level calls; CLI `-o`/`--output` flag writes `.ll` then invokes `clang -O2`; tested: `main = 42` → 42, `f x y = x + y; main = f 10 32` → 42, `fib 10` → 55, `fact 12` → 479001600; 3 LLVM executable unit tests + 3 driver integration tests; remaining: closures/heap allocation, string/IO, thunks needed for full Prelude support in native code; 1326 tests total)
21. ~~**WebAssembly backend revival**~~ — DONE (rewrote `rhasky-backend-wasm-chirho` codegen: proper function calls via `call` instruction with name→func_idx map, let bindings via `local.set`/`local.get` with `EmitCtxChirho` local allocation, case expressions with `if`/`else` chains for literal/constructor/default dispatch, `local.tee` for case binder binding, `compile_core_to_wasm_executable_chirho` with shared dict elision via `elide_dicts_and_filter_chirho`, CLI `--wasm -o` flag for `.wasm` output, constructor tags as i64, float literals as f64 bit patterns; 6 new backend unit tests + 3 driver integration tests; remaining: closures/heap in linear memory, I/O host imports, full constructor field access; 1335 tests total)
22. ~~**Cranelift backend expansion**~~ — DONE (fixed two-pass declare/define architecture with pre-imported FuncRefs via `declare_func_in_func` for direct function calls; `func_ref_map_chirho` in `LowerCtxChirho` replaces broken `func_decl_map_chirho` approach; `compile_core_to_object_executable_chirho` with shared dict elision via `elide_dicts_and_filter_chirho`; `flatten_apps_chirho` for multi-arg call detection; CLI `--cranelift -o` flag produces native object → links via `cc`; 6 new backend unit tests (function call, single-arg call, dict-elided executable, multi-arg call, recursive function, constructor app) + 3 driver integration tests (constant, arithmetic with call, recursive fibonacci); remaining: closures/heap allocation, I/O via runtime linking, thunks; 1346 tests total)
23. **Shared RTS library** — factor runtime support (GC, thunk entry, stack management, exception frames) into a linkable RTS shared across LLVM/Cranelift/WASM backends

#### D. CLI & Developer Experience

24. ~~**`rhasky compile`**~~ — DONE (CLI command reads `.hs` file, runs full pipeline, reports module name, core bindings count, LLVM IR and WASM output sizes)
25. ~~**`rhasky run`**~~ — DONE (CLI command reads `.hs` file, evaluates via STG interpreter, prints IO output to stdout; works with infinite lists, IO, typeclasses, all Phase 1+2 features)
26. ~~**`rhasky repl`**~~ — DONE (interactive REPL loop with expression evaluation via STG machine, IO action execution with output capture, user-friendly value display; commands: `:type <expr>` shows inferred type via run_frontend_chirho, `:info <name>` looks up type in env, `:load <file>` loads declarations and type environment, `:reload` re-loads last file, `:let <decl>` accumulates definitions, `:clear` resets, `:{`/`:}` for multi-line input, `:quit` exits; smart declaration vs expression detection via `has_toplevel_equals_chirho` that skips `=` inside strings/parens/`==`; fallback `print()` wrapping on eval failure; 3 new e2e tests: REPL-style expression, IO with show, let binding; 1355 tests total)
27. ~~**`rhasky check`**~~ — DONE (CLI command reads `.hs` file, runs frontend pipeline through type-checking, reports module/mode/diagnostics without full code generation; already wired as `check`/`plan`/`script` subcommands)
28. ~~**`rhasky build`**~~ — DONE (CLI `rhasky build [dir]` command: auto-detects `.cabal` files via `find_cabal_file_chirho` and uses `compile_cabal_project_chirho` with package index and dependency resolution; falls back to `compile_project_dir_chirho` for bare directories; reports package name, module count, and compilation order; 2 new tests: cabal-based project compilation, hidden directory skipping; 1391 tests total)
29. ~~**Error messages**~~ — DONE (render_chirho.rs in rhasky-diagnostics-chirho: Rust/Elm-style diagnostic renderer with source code snippets, underline annotations `^^^`, ANSI color support, file:line:col location arrows, secondary label rendering, notes and fix suggestions, bundle summary line; RenderConfigChirho with color_chirho and context_lines_chirho; AnsiChirho helper with severity-colored output (red errors, yellow warnings, cyan info, green hints); render_diagnostic_chirho/render_bundle_chirho public API; render_diagnostics_chirho convenience in driver; CLI check/compile commands use renderer with terminal color detection via IsTerminal; 6 new tests: error with span, warning with note, secondary label, bundle summary, dummy span, primary label message; 1372 tests total)
30. ~~**`--dump-core`/`--dump-stg`/`--dump-llvm`**~~ — DONE (CLI flags print Core IR via pretty_module_chirho, STG code table entries via Debug, and LLVM IR text to stderr; work with both `run` and `compile` subcommands; proper flag parsing separated from positional args; help text with usage examples)

#### E. Language Features — Remaining GHC Haskell

31. **Type families** — open and closed type families (`type family F a where ...`); type instance declarations; associated type families in classes
32. ~~**ExistentialQuantification**~~ — DONE (parser `parse_con_decl_chirho` recognizes `forall` keyword before constructor declarations, skips type variables until dot, handles optional class context before `=>`; CST→AST lowerer `lower_con_decl_chirho` detects `ForallKeywordChirho` at start of ConDecl, scans past `DoubleArrowChirho` or `VarSymChirho` (dot) to find the actual constructor name; works with and without context; existential values can be constructed and pattern-matched; 3 new tests: existential with context parses, existential without context parses, existential constructor eval `MkBox 42 → 42`; 1394 tests total)
33. ~~**TypeApplications** — `read @Int "42"`, `show @Bool True`~~ — DONE (CST parser recognizes `@Type` after expressions via `parse_fexp_chirho`; `TypeAppExprChirho` CST node; AST `TypeAppChirho` variant; CST→AST lowering extracts expression and type children; type inference passes through to inner expression; desugarer erases type application; 4 new e2e tests: `id @Int 42`, `show @Bool True`, `f @Int 41`, `apply @Int (\n -> n+8) 34`; 1405 tests total)
34. ~~**OverloadedStrings** — `IsString` type class; string literals desugar to `fromString`~~ — DONE
35. ~~**OverloadedLists** — `IsList` type class; list literals desugar to `fromList`~~ — DONE (IsList class declaration with `fromList :: [a] -> l` and `toList :: l -> [a]` methods in class_chirho.rs; `IsList [a]` identity instance; type signatures in infer_chirho.rs; `$prim_IsList_fromList_[t9037]`/`$prim_IsList_toList_[t9037]` builtin bindings with `id#` primop; dict pass handles IsList defaulting; `IdChirho` primop in runtime for identity function; top-level user binding shadowing prevents dict rewriting of user-defined `toList`/`fromList`; fromList/toList added to Prelude exports; 2 new tests; 1396 tests total)
36. ~~**DeriveFunctor/DeriveFoldable/DeriveTraversable** — auto-derive Functor/Foldable/Traversable~~ — DONE (derive_functor_chirho generates `fmap` with field-level type analysis: direct var → `f x`, nested var → `fmap f x`, no var → passthrough; derive_foldable_chirho generates `foldMap` combining with `<>`; derive_traversable_chirho generates `traverse` with `<$>`/`<*>` applicative style; `type_is_var_chirho` for span-insensitive type comparison; Foldable/Traversable class declarations in class_chirho.rs; `fmap` now polymorphic `Functor f => (a -> b) -> f a -> f b`; foldMap/traverse type signatures in prelude; 5 unit tests + 2 e2e tests (simple Box fmap, multi-field Tagged fmap); 1412 tests total)
37. **DeriveGeneric** — `Generic` type class and `GHC.Generics` representation types
38. **ConstraintKinds** — constraints as first-class kinds
39. ~~**FlexibleInstances/FlexibleContexts** — relax Haskell 98 instance/context restrictions~~ — DONE (no Haskell 98 restrictions enforced; all instance heads and contexts already flexible)
40. **DataKinds** — promote data constructors to type-level
41. ~~**KindSignatures**~~ — DONE (AST `TyVarChirho` struct with `name_chirho: NameChirho` + `kind_annotation_chirho: Option<AstKindChirho>`; `AstKindChirho` enum with `StarChirho` and `ArrowChirho`; `TyVarChirho` implements `Deref<Target=NameChirho>` and `From<NameChirho>` for minimal disruption; `type_vars_chirho` changed from `Vec<NameChirho>` to `Vec<TyVarChirho>` in DataDeclChirho, NewtypeDeclChirho, TypeAliasDeclChirho, ClassDeclChirho, ForallChirho; parser lowerer recognizes `(varId :: kind)` pattern in flat token stream with `try_parse_kind_annotated_tyvar_chirho`; kind parser handles `*`, `Type`, `* -> *`, nested `(* -> *) -> *`; kind inference uses annotations as constraints instead of fresh variables via `ast_kind_to_kind_chirho` converter; 5 parser unit tests + 5 driver e2e tests; 1437 tests total)
42. ~~**DefaultSignatures**~~ — DONE (`default methodName :: ConstrainedType` syntax parsed in class bodies via `try_extract_default_sig_chirho`; detects `default VarId :: ...` pattern in DefaultDeclChirho CST nodes, distinguishing from `default (Int, Double)` declarations; stores raw type text on `ClassMethodChirho.default_sig_chirho: Option<String>`; class body where-clause scanning handles both direct DefaultDecl children and nested WhereClause children; default method implementations already work end-to-end with instance method fallback; 2 parser unit tests (default sig present, absent) + 3 e2e driver tests (basic eval with default, override eval, AST verification); 1460 tests total)
43. **Template Haskell (basic)** — quasi-quotation, reify, splicing for compile-time metaprogramming
44. **Foreign exports** — `foreign export ccall` for Haskell functions callable from C/JS
45. ~~**Monad transformers** — StateT, ReaderT, WriterT, ExceptT, MaybeT evaluation through STG machine~~ — DONE (StateT monad operations: `get` retrieves state, `put` sets state, `modify` applies function to state, `bindStateT`/`returnStateT` for monadic composition, `evalState`/`execState`/`runState`/`runStateT` for running computations; MaybeT constructor/destructor (`MaybeT`/`runMaybeT`); all operations generated as Core IR prelude bindings with proper tuple construction/destruction; type signatures in infer_chirho.rs; names exported from Prelude; 5 new e2e tests: get+evalState, put+execState, modify×3, bind+return counter, bind+return with arithmetic; remaining: ReaderT/WriterT/ExceptT operations, MonadTrans lift instances; 1401 tests total)

#### F. Package Management & Hackage

46. **Cabal file parsing (full)** — complete `.cabal` spec: conditionals, flags, common stanzas, source-repository, custom setup
47. **Hackage package download** — fetch `.tar.gz` from Hackage, unpack, parse `.cabal`
48. **Dependency resolution** — solve version constraints across transitive dependency graph; conflict resolution; use `rhasky-package-chirho` resolver
49. **Package database** — installed package registry; track compiled modules and their interface files
50. **cabal-install compatibility** — `rhasky install` fetches and builds packages from Hackage

#### G. Optimization

51. ~~**Inlining**~~ — DONE (`{-# INLINE f #-}`, `{-# NOINLINE f #-}`, `{-# INLINABLE f #-}` pragmas parsed from source, stored on `ModuleChirho.inline_pragmas_chirho`, propagated to `CoreBindingChirho.inline_chirho` during desugaring; `InlineAnnotationChirho` enum (Always/Never/Inlinable/None); Core simplifier inlining pass: INLINE always inlines regardless of size, NOINLINE never inlines, INLINABLE inlines small non-recursive bindings (threshold=10 AST nodes), no-annotation auto-inlines trivial expressions (Var/Lit only); `expr_size_chirho` AST node counter; `build_inline_env_chirho` + `inline_expr_chirho` with proper shadow handling; 15 new tests: 8 simplifier unit tests, 1 pragma parsing test, 4 e2e tests, 2 size tests; 1427 tests total)
52. **Strictness analysis** — worker/wrapper transform; unboxing strict arguments
53. ~~**Specialization**~~ — DONE (`{-# SPECIALIZE f :: Type #-}` and `{-# SPECIALISE f :: Type #-}` pragmas parsed from source via `extract_specialize_pragmas_chirho`; stored on `ModuleChirho.specialize_pragmas_chirho` and propagated to `CoreModuleChirho.specialize_pragmas_chirho`; `specialize_bindings_chirho` Core-to-Core pass in simplifier Phase 4 clones binding RHS for each specialization, creates `$spec_f_N` named bindings marked `InlineAnnotationChirho::AlwaysChirho` for aggressive optimization; handles multiple specializations per function and nonexistent targets gracefully; 4 parser unit tests (basic, British spelling, multiple, absent) + 4 simplifier unit tests (creates copy, multiple types, nonexistent, preserves RHS) + 4 e2e driver tests (basic eval, Core binding created, SPECIALISE spelling, multiple specs); 1455 tests total)
54. ~~**Common subexpression elimination**~~ — DONE (two-level CSE pass: top-level binding deduplication via `cse_top_level_chirho` identifies non-recursive bindings with identical non-trivial RHS and redirects duplicates to canonical binding; intra-expression CSE via `cse_expr_chirho` deduplicates identical RHS within `let` blocks and rewrites body references; respects INLINE/INLINABLE annotations — never deduplicates annotated bindings; recursive bindings skipped; `apply_cse_redirects_chirho` rewrites variable references throughout expression tree; integrated as Phase 3 in simplify_module_chirho iteration loop; 6 new unit tests: top-level duplicate/no-dup/recursive-skip, let-binding dup/different-rhs/redirect-in-body; 1443 tests total)
55. **Constructor specialization** — SpecConstr-style optimization for recursive functions
56. **Demand analysis** — absence analysis, usage analysis for dead argument elimination

#### H. Testing & Conformance

57. **GHC test suite integration** — pull and run relevant GHC test cases; track pass rate
58. **Property-based testing** — add proptest/quickcheck-style tests for parser, type checker, evaluator; focus on parser/layout malformed-input properties, simplifier semantic-preservation, dictionary-pass invariants, runtime evaluator step/heap invariants
59. **Benchmark suite** — nofib-style benchmarks for runtime performance tracking
60. **Haskell Report conformance tracker** — systematic coverage of Haskell 2010 Report sections
61. **Backend round-trip smoke tests** — execute emitted LLVM/Wasm/Cranelift artifacts where possible and compare output to STG interpreter
62. **Differential testing against GHC** — syntax and typechecker edge case comparison
