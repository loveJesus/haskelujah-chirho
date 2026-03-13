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

