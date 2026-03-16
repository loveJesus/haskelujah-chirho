<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Compatibility Chirho

## Compatibility Contract

The project aims for maximal practical compatibility with Haskell as it is actually used, not only with a classroom subset. The compatibility target has four layers:

1. source language compatibility
2. package metadata and build compatibility
3. runtime and foreign-function compatibility
4. tooling and workflow compatibility

## Source Language Compatibility

The baseline target should include the standard language plus the real-world features needed by common packages. The implementation should prioritize:

- modules, imports, exports, and qualified names
- algebraic data types, records, pattern matching, and typeclasses
- kinds, higher-kinded types, and generalized deriving paths needed by ecosystem code
- enough extension coverage to compile important libraries instead of stopping at toy examples

Extension support should be prioritized by measured ecosystem need, not by novelty.

## Package Compatibility

The package system needs first-class support for:

- Cabal package description parsing
- package flags and conditional sections
- local package databases
- Hackage index retrieval and storage
- reproducible build plans
- interface metadata that downstream packages can consume

The compiler architecture should assume real dependency graphs, not a single-package world.

## Compatibility Strategy

- prefer compatibility shims over source-breaking divergence
- if a historical compiler quirk is relied on by important packages, preserve it behind a clearly named compatibility mode when possible
- keep deviations explicit, testable, and documented
- maintain a compatibility corpus of real packages and track failures by cause

## Runtime Compatibility

Compatibility is not only parser-deep. The runtime must eventually account for:

- laziness semantics
- closure and thunk behavior
- exception semantics
- foreign-function boundaries
- linking and package initialization behavior

The project should not claim ecosystem compatibility until runtime behavior, not just frontend acceptance, supports that claim.

## Tooling Compatibility

The system should eventually expose enough structure for:

- editor integration
- REPL inspection commands
- package build graph introspection
- deterministic diagnostics across machine environments

## Improvement Policy

The compiler should be better without being needlessly different:

- better error messages are good
- cleaner crate boundaries are good
- stronger invariants are good
- silently incompatible semantics are not good

## Extension Priority Tiers

Extension support should be staged by measured ecosystem need:

### Tier 1 — Target: Milestone 2

Extensions needed by nearly every non-trivial Haskell package:

OverloadedStrings, ScopedTypeVariables, FlexibleInstances, FlexibleContexts, MultiParamTypeClasses, DeriveGeneric, DeriveFunctor, DeriveTraversable, DeriveFoldable, GeneralizedNewtypeDeriving, TypeApplications, LambdaCase, TupleSections, BangPatterns, RecordWildCards, NamedFieldPuns, StandaloneDeriving, DerivingStrategies, DerivingVia, InstanceSigs, KindSignatures, ExplicitForAll.

### Tier 2 — Target: Milestones 3–5

Extensions needed by important libraries (aeson, lens, servant, mtl, transformers):

GADTs, TypeFamilies, DataKinds, RankNTypes, FunctionalDependencies, ExistentialQuantification, ConstraintKinds, TypeOperators, MultiWayIf, PatternSynonyms, ViewPatterns, OverloadedLists, DefaultSignatures, DeriveAnyClass, QuantifiedConstraints, RoleAnnotations.

### Tier 3 — Target: Milestone 8+

Advanced extensions deferred without shame:

TemplateHaskell, QuasiQuotes, UnboxedTuples, UnboxedSums, MagicHash, LinearTypes, ImpredicativeTypes, TypeInType, Backpack, OverloadedRecordDot, OverloadedRecordUpdate, MonadComprehensions, RebindableSyntax, CApiFFI.

Extensions move tiers only when a concrete ecosystem need is demonstrated, not by novelty or request.

## Base Library Strategy

The project must ship its own base library (`haskelujah-base-chirho`) because GHC's `base` is tightly coupled to GHC-specific primitive operations (MagicHash, unboxed types, GHC.Prim) that are impractical to replicate in early milestones.

### Initial Scope

- Prelude (standard functions, type classes, basic IO)
- Data.List, Data.Maybe, Data.Either, Data.Tuple
- Data.Char, Data.String
- Data.Int, Data.Word (fixed-width integers)
- System.IO (basic file IO)
- Control.Monad (core monad operations)
- Data.IORef (mutable references)

### Expansion Strategy

Add modules as package compatibility demands. When a real package fails to compile because a base module is missing, that module is prioritized. Track coverage gaps explicitly in the compatibility corpus.

### Primitive Operations

Define a small set of compiler-known primitive operations (arithmetic, IO, array access, etc.) that the base library calls into. These primops are the interface between Haskell code and the Haskelujah runtime. They should be documented, versioned, and stable enough that base library code does not break across compiler updates.

## C Foreign Function Interface Strategy

Many important Hackage packages depend on C libraries. FFI support is not optional for ecosystem compatibility.

### Haskell Side

Parse `foreign import` and `foreign export` declarations uniformly regardless of backend. Support the `ccall` and `capi` calling conventions as specified by the Haskell FFI addendum.

### LLVM Path

Standard C calling convention via LLVM IR `declare`/`call` instructions. Link against system C libraries using the platform linker. Header locations and library paths resolved through package metadata (`.cabal` `extra-lib-dirs`, `includes`) or environment variables.

### WebAssembly Path

WASI imports for system calls (filesystem, clock, random). For non-WASI FFI, C libraries must be compiled to `wasm32-wasi` or shimmed with JavaScript/host imports. This is an inherent limitation of the Wasm sandbox model.

### Known Hard Cases

- Packages with `c-sources` in `.cabal`: require compiling C to the target backend
- Platform-specific FFI (`Win32`, `POSIX`): need conditional compilation support in the package system
- Callbacks from C to Haskell: require stable pointers and the ability to create Haskell closures callable from C
- `CApiFFI` extension: deferred to Tier 3 but architecturally supported

## Template Haskell Acknowledgment

Template Haskell is used by many of the most popular Hackage packages: `aeson`, `lens`, `persistent`, `servant`, `yesod`, `optics`, and others. It is deferred to Milestone 8+ but the architecture must not preclude it.

### Architectural Implication

TH requires the compiler to execute Haskell code at compile time. This means:

- the runtime must be available during compilation (the driver crate must be able to call into the runtime crate)
- compiled TH splices must be able to inspect and generate AST structures
- cross-compilation with TH requires either a host-target split or a Wasm-based evaluator

### Tracking

TH-dependent packages should be tracked separately in the compatibility corpus. When TH is eventually supported, the compatibility gap should narrow significantly.

