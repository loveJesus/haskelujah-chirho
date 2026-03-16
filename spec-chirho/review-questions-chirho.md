<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Review Questions Chirho

These are the highest-value questions for external review, with current resolutions.

## Compatibility Strategy

- Is the near 1:1 compatibility target framed correctly, or does it still leave too much room for accidental dialect drift?
- Should compatibility modes be explicit feature flags, package-set profiles, or both?

**Resolution:** The compatibility target is framed correctly. Compatibility modes should be both feature flags and profiles: individual flags for specific GHC quirks (e.g. `--compat-relaxed-import-warnings`), and named profiles that bundle flags (e.g. `--compat-profile ghc-9.8`). This lets users opt into specific behaviors and lets the project ship curated compatibility sets. A `--strict` mode that rejects all non-Haskell-Report code is also valuable for catching portability issues.

## Intermediate Representations

- Is a typed core followed by an STG-like runtime-oriented IR the right balance for this project?
- Should the scripting and REPL path share exactly the same lowered representation as the LLVM and Wasm backends, or is one additional interpreter-focused form justified?

**Resolution:** Yes, typed core + STG is the right balance. This is proven by GHC, Eta, and UHC. The scripting/REPL path should start by interpreting STG directly. One additional bytecode form is justified only if profiling reveals that tree-walking STG interpretation is too slow for interactive use. Do not introduce the bytecode form preemptively.

## Package Ecosystem

- What is the cleanest path to Cabal package compatibility without coupling the driver too tightly to one packaging workflow?
- Which package metadata edge cases should be considered mandatory in the first serious compatibility milestone?

**Resolution:** The `haskelujah-package-db-chirho` crate should parse Cabal files independently of the driver, exposing a package metadata API that the driver, script runner, and REPL all consume. This keeps packaging logic reusable. For the first compatibility milestone (M2), mandatory edge cases are: conditional sections (`if flag(...)` / `if os(...)` / `if impl(ghc)`), common stanzas, multiple `library` sections, and `build-depends` version ranges. Package flags with defaults should work. `custom-setup` and `configure`-style builds can be deferred.

## Runtime

- What memory management strategy best balances native performance, Wasm portability, and scripting/REPL latency?
- Where should the boundary live between shared runtime logic and backend-specific runtime glue?

**Resolution:** Copying generational GC for native (proven by GHC for lazy languages with high allocation rates). Linear-memory GC for Wasm (more portable than the evolving Wasm GC proposal). The GC interface should be abstracted behind a trait in `haskelujah-runtime-chirho` so that both backends implement the same allocation/collection API with different underlying strategies. Shared runtime logic includes: thunk evaluation protocol, exception handling, module initialization, and blackhole detection. Backend-specific glue includes: memory layout, stack management, and calling conventions.

## Rust Workspace

- Is the proposed crate split too fine, too coarse, or about right for compiler build times and modularity?
- Which crates should be kept dependency-light from the start to preserve compilation speed?

**Resolution:** The split is about right for the target architecture. Some crates (`haskelujah-simplify-chirho`, `haskelujah-stg-chirho`) should start as modules within their parent crate and split out when they grow large enough to justify the boundary. Crates that must stay dependency-light: `haskelujah-span-chirho` (zero external deps), `haskelujah-syntax-chirho` (only span), `haskelujah-diagnostics-chirho` (only span). The `haskelujah-parser-chirho` crate should avoid pulling in heavy dependencies that would slow down the most frequently recompiled code during development.

## Solver Design

- Is `propagators-chirho` a good fit anywhere in the type inference, dependency solving, or incremental invalidation story, or would a specialized solver be clearer and easier to control?

**Resolution:** A specialized solver is preferred for type inference (OutsideIn(X)-inspired) and dependency solving (version-constraint SAT). `propagators-chirho` may be worth evaluating for incremental invalidation in the REPL (M7), where propagation-style updates could efficiently recheck only affected declarations when a module is reloaded. Do not introduce it earlier unless a concrete prototype demonstrates measurable benefit over a simpler approach.
