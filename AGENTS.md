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
9. **Desugar → Dict Pass → Core → Simplify** — `rhasky-core-chirho` System FC-style Core IR with dictionary-passing transform
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

**1273 tests passing**, 0 failures, 3 ignored (2 Cranelift, 1 doctest)

For detailed Phase 1 test breakdown by category, see [spec-chirho/phase1-archive-chirho.md](spec-chirho/phase1-archive-chirho.md).

### Phase 2 Priorities

_Phase 1 completed 128 priorities (see [spec-chirho/phase1-archive-chirho.md](spec-chirho/phase1-archive-chirho.md)). Phase 2 focuses on making the compiler practical: true laziness, real multi-module compilation, backend code generation, CLI usability, and Hackage compatibility._

#### A. Runtime Semantics — Laziness & Evaluation Model

1. **True lazy evaluation** — replace strict-by-default STG evaluation with proper lazy thunk semantics; `take 5 [1..]` and infinite list idioms must work; update thunk entry, blackholing, and GC root scanning
2. **Lazy I/O** — `interact`, `getContents`, `hGetContents` return lazy strings; must integrate with GC and exception handling
3. **Bang patterns and strict fields** — `!` annotations in data declarations and function arguments force evaluation at binding time; `{-# UNPACK #-}` pragma for strict fields
4. **Weak head normal form semantics** — ensure `seq`, `deepseq`, `evaluate`, `($!)` have correct WHNF forcing behavior; `NFData` type class
5. **STM (Software Transactional Memory)** — TVar, atomically, retry, orElse; conflict detection and rollback

#### B. Multi-Module System & Imports

6. **Automatic Prelude import** — every module implicitly imports Prelude unless `{-# LANGUAGE NoImplicitPrelude #-}` or explicit `import Prelude` is present
7. **Qualified module syntax** — `Data.Map.insert`, `Data.Set.member` as qualified function calls in user source
8. **Module re-exports** — `module Data.Map (module Data.Map.Internal)` re-export syntax
9. **Orphan instance detection** — warn on orphan instances; support `{-# OPTIONS_GHC -fno-warn-orphans #-}`
10. **Hierarchical module compilation** — compile multi-file Haskell projects with proper dependency ordering; `.hi` interface file generation and consumption
11. **Circular module imports** — handle mutual module dependencies via `.hs-boot` files or a fixpoint approach

#### C. Code Generation Backends

12. **LLVM backend revival** — make `rhasky-backend-llvm-chirho` produce runnable executables; STG closure layout in LLVM IR; entry code, info tables, stack management; link with a minimal RTS
13. **WebAssembly backend revival** — make `rhasky-backend-wasm-chirho` produce runnable `.wasm` modules; memory management, function tables, linear memory GC
14. **Cranelift backend expansion** — extend `rhasky-backend-cranelift-chirho` beyond scaffold to compile non-trivial programs; leverage Cranelift's fast compilation for JIT and debug builds
15. **Shared RTS library** — factor runtime support (GC, thunk entry, stack management, exception frames) into a linkable RTS shared across LLVM/Cranelift/WASM backends

#### D. CLI & Developer Experience

16. **`rhasky compile`** — compile `.hs` file(s) to object code via selected backend (LLVM default, `--backend=wasm/cranelift/jvm/beam`)
17. **`rhasky run`** — compile and execute a Haskell source file in one step (via STG interpreter by default, or native via `--native`)
18. **`rhasky repl`** — interactive REPL with expression evaluation, `:type`, `:info`, `:load`, `:reload` commands
19. **`rhasky check`** — type-check without code generation (fast feedback loop)
20. **`rhasky build`** — build a Cabal project (parse `.cabal`, resolve dependencies, compile modules in dependency order)
21. **Error messages** — structured diagnostics with source spans, suggestions, and color output; follow Rust/Elm error message style
22. **`--dump-core`/`--dump-stg`/`--dump-llvm`** — debug flags to print intermediate representations

#### E. Language Features — Remaining GHC Haskell

23. **Type families** — open and closed type families (`type family F a where ...`); type instance declarations; associated type families in classes
24. **ExistentialQuantification** — `data Showable = forall a. Show a => MkShowable a`
25. **TypeApplications** — `read @Int "42"`, `show @Bool True`
26. **OverloadedStrings** — `IsString` type class; string literals desugar to `fromString`
27. **OverloadedLists** — `IsList` type class; list literals desugar to `fromList`
28. **DeriveFunctor/DeriveFoldable/DeriveTraversable** — auto-derive Functor/Foldable/Traversable
29. **DeriveGeneric** — `Generic` type class and `GHC.Generics` representation types
30. **ConstraintKinds** — constraints as first-class kinds
31. **FlexibleInstances/FlexibleContexts** — relax Haskell 98 instance/context restrictions
32. **DataKinds** — promote data constructors to type-level
33. **KindSignatures** — explicit kind annotations on type variables
34. **DefaultSignatures** — default method implementations using superclass constraints
35. **Template Haskell (basic)** — quasi-quotation, reify, splicing for compile-time metaprogramming
36. **Foreign exports** — `foreign export ccall` for Haskell functions callable from C/JS
37. **Monad transformers** — StateT, ReaderT, WriterT, ExceptT, MaybeT evaluation through STG machine

#### F. Package Management & Hackage

38. **Cabal file parsing (full)** — complete `.cabal` spec: conditionals, flags, common stanzas, source-repository, custom setup
39. **Hackage package download** — fetch `.tar.gz` from Hackage, unpack, parse `.cabal`
40. **Dependency resolution** — solve version constraints across transitive dependency graph; conflict resolution; use `rhasky-package-chirho` resolver
41. **Package database** — installed package registry; track compiled modules and their interface files
42. **cabal-install compatibility** — `rhasky install` fetches and builds packages from Hackage

#### G. Optimization

43. **Inlining** — `INLINE`/`NOINLINE` pragmas; automatic small-function inlining in Core simplifier
44. **Strictness analysis** — worker/wrapper transform; unboxing strict arguments
45. **Specialization** — `SPECIALIZE` pragma; monomorphize polymorphic functions at known types
46. **Common subexpression elimination** — CSE pass on Core
47. **Constructor specialization** — SpecConstr-style optimization for recursive functions
48. **Demand analysis** — absence analysis, usage analysis for dead argument elimination

#### H. Testing & Conformance

49. **GHC test suite integration** — pull and run relevant GHC test cases; track pass rate
50. **Property-based testing** — add proptest/quickcheck-style tests for parser, type checker, evaluator
51. **Benchmark suite** — nofib-style benchmarks for runtime performance tracking
52. **Haskell Report conformance tracker** — systematic coverage of Haskell 2010 Report sections
