<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Haskelujah Phase 2 Archive — Completed Priorities

**Phase 2 span**: 2026-03-14 → 2026-03-15
**Final state**: 1801 tests passing, 20 crates

---

## Summary

62 priorities all DONE across 8 areas:

- **Coherence & Debt (8)**: unified driver pipeline, stale parser cleanup, export list lowering, token model fix, file splitting, warning cleanup, spec reconciliation, John 3:16 compliance
- **Runtime Semantics (5)**: true lazy evaluation, lazy I/O, bang patterns, WHNF semantics (seq/$!/$!!/deepseq/force/evaluate/NFData), STM (newTVar/readTVar/writeTVar/atomically/retry/orElse)
- **Multi-Module (6)**: automatic Prelude import, qualified module syntax, module re-exports, orphan instance detection, hierarchical module compilation with dependency graph, circular module imports via Tarjan SCC + .hs-boot
- **Backends (4)**: LLVM backend revival (native executables via clang), WebAssembly backend revival, Cranelift backend expansion, shared RTS library extraction
- **CLI & DX (7)**: `rhasky compile`, `rhasky run`, `rhasky repl` (with :type/:info/:load/:let), `rhasky check`, `rhasky build` (Cabal-aware), Elm-style error messages with did-you-mean, `--dump-core/--dump-stg/--dump-llvm`
- **Language Features (15)**: type families, existentials, TypeApplications, OverloadedStrings, OverloadedLists, DeriveFunctor/Foldable/Traversable, DeriveGeneric, ConstraintKinds, FlexibleInstances/Contexts, DataKinds, KindSignatures, DefaultSignatures, TH basic (AST+Q+reify+convert), foreign exports, monad transformers (StateT/ReaderT/WriterT/ExceptT/MaybeT)
- **Package Management (5)**: full Cabal parsing (conditionals/flags/stanzas), Hackage download+extraction, dependency resolver with backtracking, package database, `rhasky install`
- **Optimization (6)**: INLINE/NOINLINE/INLINABLE pragmas, strictness analysis + worker/wrapper, SPECIALIZE pragmas, CSE, constructor specialization (SpecConstr), demand analysis + dead arg elimination
- **Testing (6)**: GHC test suite (20 curated + 55% bulk pass rate), property-based testing (19 proptest tests), nofib benchmarks (10/10 correct), Haskell 2010 conformance tracker, backend round-trip smoke tests, differential testing vs GHC (25/25 agreement)
