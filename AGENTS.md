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
