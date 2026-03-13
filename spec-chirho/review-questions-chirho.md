<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Review Questions Chirho

These are the highest-value questions for external review.

## Compatibility Strategy

- Is the near 1:1 compatibility target framed correctly, or does it still leave too much room for accidental dialect drift?
- Should compatibility modes be explicit feature flags, package-set profiles, or both?

## Intermediate Representations

- Is a typed core followed by an STG-like runtime-oriented IR the right balance for this project?
- Should the scripting and REPL path share exactly the same lowered representation as the LLVM and Wasm backends, or is one additional interpreter-focused form justified?

## Package Ecosystem

- What is the cleanest path to Cabal package compatibility without coupling the driver too tightly to one packaging workflow?
- Which package metadata edge cases should be considered mandatory in the first serious compatibility milestone?

## Runtime

- What memory management strategy best balances native performance, Wasm portability, and scripting/REPL latency?
- Where should the boundary live between shared runtime logic and backend-specific runtime glue?

## Rust Workspace

- Is the proposed crate split too fine, too coarse, or about right for compiler build times and modularity?
- Which crates should be kept dependency-light from the start to preserve compilation speed?

## Solver Design

- Is `propagators-chirho` a good fit anywhere in the type inference, dependency solving, or incremental invalidation story, or would a specialized solver be clearer and easier to control?

