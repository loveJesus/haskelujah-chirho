<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

Please comment the full content of John 3:16 at the top of all our files.

## Identifier Naming Convention: Chirho Suffix

ALL identifiers we create MUST be suffixed with "Chirho" in the appropriate case. This applies to variables, functions, structs, enums, traits, modules, files, directories, parameters, fields, keys, routes, table/column names — everything, in every language.

**Casing rules by language:**
- **Rust**: `snake_case_chirho` for values/functions/fields/modules/files, `PascalCaseChirho` for types/enums/traits, `SCREAMING_SNAKE_CASE_CHIRHO` for constants
- **Haskell**: `camelCaseChirho` for values/functions/params, `PascalCaseChirho` for types/constructors/classes/modules
- **JS/TS**: `camelCaseChirho` for values/functions/params, `PascalCaseChirho` for classes/types/enums
- **Python**: `snake_case_chirho` for values/functions/params, `PascalCaseChirho` for classes
- **Database**: `snake_case_chirho` for tables/columns/indexes/constraints
- **Routes/APIs**: kebab-case with `-chirho` suffix
- **Constants**: `SCREAMING_SNAKE_CASE_CHIRHO` where the language permits uppercase

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
- Granularity is left to agent judgment.

You can modify the following section
### Agent Self Modifications

- Primary project direction: build a Haskell compiler in Rust, named **Haskeluya**.
- Backend direction: backend-agnostic pipeline, LLVM IR primary, WebAssembly first-class.
- Compatibility: aim for maximal GHC/Haskell compatibility including Cabal and Hackage.
- Quality: prefer cleaner modular design over GHC quirks, without breaking source compatibility.
- Runtime: laziness, closures, thunks, GC, FFI are core early design decisions.
- Packaging: split into focused crates for portability and fast iteration.
- Git: branch `main_chirho`, remote `gh_chirho`.

### Current Pipeline State (as of 2026-03-15)

12-phase pipeline in `haskeluya-driver-chirho`: lex → layout → CST parse → AST lower → name resolve → kind infer → type infer → exhaustiveness check → desugar/dict-pass/core/simplify → STG eval → FFI → exception handling.

### Workspace Crates (21 crates)

| Crate | Purpose |
|---|---|
| `haskeluya-span-chirho` | Source locations, file IDs, source map |
| `haskeluya-diagnostics-chirho` | Structured compiler diagnostics |
| `haskeluya-syntax-chirho` | Tokens, SyntaxKind, green tree, lexer, layout |
| `haskeluya-parser-chirho` | CST parser (green tree builder) |
| `haskeluya-ast-chirho` | AST types, CST→AST lowering |
| `haskeluya-naming-chirho` | Name resolution, module interfaces, imports/exports |
| `haskeluya-typing-chirho` | HM type inference, typeclasses, kind inference, exhaustiveness, deriving |
| `haskeluya-core-chirho` | Core IR (System FC), desugaring, dict-pass, simplifier |
| `haskeluya-backend-llvm-chirho` | Core → LLVM IR |
| `haskeluya-backend-wasm-chirho` | Core → WebAssembly |
| `haskeluya-backend-cranelift-chirho` | Core → Cranelift → native objects |
| `haskeluya-backend-jvm-chirho` | Core → JVM bytecode |
| `haskeluya-backend-beam-chirho` | Core → BEAM bytecode |
| `haskeluya-driver-chirho` | Pipeline orchestration |
| `haskeluya-rts-chirho` | Shared RTS: values, heap, GC |
| `haskeluya-runtime-chirho` | STG interpreter, eval loop, FFI |
| `haskeluya-incremental-chirho` | Incremental compilation, fingerprinting |
| `haskeluya-package-chirho` | Cabal parser, Hackage, dependency resolution |
| `haskeluya-cli-chirho` | CLI (compile/run/repl/check/build/install) |
| `haskeluya-th-chirho` | Template Haskell: TH AST, Q monad, reification |
| `haskeluya-test-harness-chirho` | Golden tests, GHC suite, benchmarks, conformance |

### Test Coverage

**1841 tests passing**, 0 failures

### Completed (Phase 1 + Phase 2)

Phase 1: 128 priorities (see [spec-chirho/phase1-archive-chirho.md](spec-chirho/phase1-archive-chirho.md))

Phase 2: 62 priorities all DONE — coherence/debt cleanup (8), runtime semantics with true laziness/bang patterns/WHNF/STM (5), multi-module with Prelude/qualified/re-exports/orphans/hierarchical/circular (6), backends LLVM/Wasm/Cranelift/shared-RTS (4), CLI compile/run/repl/check/build with error messages and dump flags (7), language features type-families/existentials/TypeApplications/OverloadedStrings+Lists/DeriveFunctor+Foldable+Traversable/DeriveGeneric/ConstraintKinds/FlexibleInstances/DataKinds/KindSignatures/DefaultSignatures/TH-basic/foreign-exports/monad-transformers (15), package management Cabal/Hackage/resolver/pkgdb/install (5), optimization inlining/strictness/specialization/CSE/SpecConstr/demand-analysis (6), testing GHC-suite/property-tests/benchmarks/conformance/round-trip/differential (6).

### Phase 3 Priorities

_Phase 3 focuses on completing TH, making backends produce real executables, and scaling to real-world Haskell code._

#### A. In Progress

1. **Template Haskell completion** — TH tokens/parser/AST/lowering done; splice evaluation with built-in makeLenses done; remaining: STG bridge connecting Q monad callbacks to compiler state for user-written TH code

#### B. Backend Maturity

2. **Backend closures & heap** — LLVM/Wasm/Cranelift backends need closure allocation, thunk entry/update, heap management for non-trivial programs
3. **Backend I/O** — string/IO support in compiled backends (LLVM: libc calls, Wasm: host imports, Cranelift: runtime linking)
4. **Backend GC integration** — wire haskeluya-rts-chirho GC into compiled code (stop-the-world mark-sweep)
5. **Self-hosting milestone** — compile a non-trivial Haskell program (e.g. a small library) to native code end-to-end

#### C. Language Completeness

6. ~~**GADTs**~~ — DONE (ConDeclChirho::GadtChirho preserves full type signature; parser recognizes `data Foo where Con :: Type` syntax; type inference, kind inference, exhaustiveness, naming, desugaring, TH all handle GadtChirho; 5 e2e tests)
7. ~~**RankNTypes**~~ — DONE (TyChirho::ForallChirho variant preserves forall in non-prenex positions; ast_type_to_ty_chirho produces ForallChirho for nested foralls; unification handles ForallChirho vs ForallChirho (alpha-rename) and ForallChirho vs concrete (SimpleSubsumption strip); bind_pat_chirho creates polymorphic schemes for ForallChirho-typed parameters; subsume_chirho for rank-N subsumption checking; 3 ty_chirho tests + 4 unify tests + 4 e2e tests; 1827 tests)
8. ~~**ScopedTypeVariables**~~ — DONE (scoped_tyvars_chirho field on InferCtxChirho; populated from function type signature forall vars before body inference; ast_type_to_scheme_chirho seeds var_map with scoped vars so where-clause annotations reuse the same TyVarChirho; has_explicit_forall_chirho helper checks for explicit forall; scoped vars restored after each function body; 3 e2e tests; 1830 tests)
9. **MultiParamTypeClasses improvements** — associated types, type family defaults
10. ~~**RecordWildCards**~~ — DONE (`Foo{..}` pattern/expression syntax: CST parser wraps `..` in FieldAssign nodes; lowerer detects DotDot inside FieldAssign children for expression records, sets `has_wildcard_chirho: true` and filters DotDot pseudo-fields; type checker `InferCtxChirho.con_field_names_chirho` maps constructor → ordered field names from RecordChirho data decls; expression wildcards: type checker iterates all constructor fields in order, explicit fields type-checked normally, missing fields looked up from scope as variable references; pattern wildcards: `bind_pat_chirho` binds remaining fields as fresh type variables in scope; desugarer `expand_record_wildcard_expr_chirho` fills missing fields from scope via `resolve_var_chirho`; `expand_record_wildcard_pat_chirho` expands to full field list with `VarChirho` patterns in constructor field order; 4 e2e tests: full pattern `MkPoint{..}`, partial pattern `MkPoint{xCoord=a,..}`, full expression `let xCoord=10; yCoord=20 in MkPoint{..}`, partial expression `MkPoint{xCoord=10,..}` with let-bound yCoord; 1841 tests)
11. **ViewPatterns** — `f (view -> pattern)` in pattern position
12. **PatternSynonyms** — user-defined pattern constructors
13. **DerivingVia** — `deriving Show via (WrappedMonoid Sum)`
14. ~~**Strict data fields**~~ — DONE (CST parser handles `!` before constructor field types via `at_strict_prefix_chirho`; `StrictnessChirho` enum with LazyChirho/StrictChirho/UnpackChirho variants; `ConDeclChirho::OrdinaryChirho.fields_chirho` as `Vec<(StrictnessChirho, TypeChirho)>`; `FieldDeclChirho.strictness_chirho` for record fields; desugarer collects `con_strictness_chirho` map from DataDeclChirho constructors; `enforce_strict_fields_chirho` wraps fully-applied strict constructor args in `let s = arg in case s of { _ -> ConApp(..., s, ...) }` to force WHNF before constructor application; 2 parser tests + 4 e2e tests; 1836 tests)
15. **Template Haskell (full)** — typed splices, quasi-quoters, reify for all declaration forms, derive via TH

#### D. Ecosystem & Tooling

16. **Language Server Protocol** — basic LSP for editor integration (hover types, go-to-definition, diagnostics)
17. **Profiling** — cost-center annotation, heap/time profiling output
18. **Cross-compilation** — target selection for LLVM/Cranelift/Wasm from CLI
19. **Documentation generation** — Haddock-style doc comments → HTML

#### E. Scale & Performance

20. **Parallel compilation** — compile independent modules concurrently
21. **Incremental type checking** — recheck only changed modules and dependents
22. **Large module handling** — ensure parser/checker scale to 10K+ line modules
