<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Prior Art Chirho

## Why Study Prior Art

Every alternative Haskell implementation has encountered the same fundamental tensions: laziness versus performance, compatibility versus cleanliness, and ecosystem breadth versus implementation depth. Studying these projects avoids repeating known mistakes and grounds design decisions in evidence.

## Eta

Haskell compiler targeting the JVM. Achieved significant GHC source compatibility by forking GHC's frontend.

**Lesson:** Runtime performance on non-native platforms requires dedicated effort beyond just getting code to compile. Reusing GHC's frontend accelerates compatibility but creates tight coupling that makes independent evolution difficult. Rhasky should build its own frontend for long-term independence.

## GRIN (Graph Reduction Intermediate Notation)

Whole-program optimizer framework for lazy functional languages with aggressive optimization through heap points-to analysis.

**Lesson:** Whole-program optimization yields excellent code quality but does not compose well with separate compilation or incremental builds. Rhasky should use per-module optimization with limited cross-module inlining, reserving whole-program analysis for optional link-time optimization.

## UHC (Utrecht Haskell Compiler)

Multi-backend Haskell compiler (JavaScript, JVM, LLVM) with a modular attribute-grammar-based architecture.

**Lesson:** Multi-backend modularity is achievable and valuable. However, ecosystem adoption requires package compatibility, not just language compatibility. UHC's attribute grammar approach was academically interesting but proved harder to maintain than traditional pass-based architectures. Rhasky should prefer explicit pass-based transformations over grammar-driven approaches.

## PureScript

Strict, explicitly-typed functional language with Haskell-like syntax, compiling to JavaScript.

**Lesson:** Diverging from Haskell semantics (strict evaluation, row polymorphism instead of typeclasses, different module system) gains implementation simplicity and can succeed in a specific niche. But it loses access to the Haskell ecosystem entirely. Rhasky should preserve Haskell semantics even when the lazy evaluation model or typeclass system is inconvenient to implement, because ecosystem compatibility is a core goal.

## Hugs

Haskell interpreter focused on REPL interaction and teaching use cases.

**Lesson:** An interpreter-first approach delivers an excellent interactive experience with fast startup and immediate feedback. But it limits the performance ceiling and makes compiled-code interop difficult. Rhasky should support interpretation for scripting and REPL while keeping ahead-of-time compilation as the primary execution path, sharing the same runtime-oriented IR for both.

## jhc

Whole-program Haskell compiler with a novel region-based memory management strategy instead of traditional garbage collection.

**Lesson:** Alternative GC strategies like region inference can work for lazy languages and eliminate GC pauses, but they require deep commitment to a fundamentally different memory model and limit interoperability with C libraries that expect traditional heap semantics. Rhasky should start with proven copying GC and consider alternative strategies only if specific workloads demand them.

## rust-analyzer

IDE-focused Rust compiler frontend with red-green syntax trees, salsa-style incremental computation, and demand-driven analysis.

**Lesson:** Red-green lossless syntax trees and incremental query frameworks are proven patterns for IDE-quality tooling that also works well for batch compilation. The key insight is that investing in lossless syntax representation early pays dividends across formatting, refactoring, REPL, and error recovery. Rhasky should adopt similar CST patterns for its parser output.

## Futhark

Purely functional language compiling to GPU code (OpenCL, CUDA, multicore C).

**Lesson:** Targeting specific backends excellently is better than targeting many backends poorly. Futhark achieves competitive GPU performance by deeply understanding its target. Rhasky should ensure each backend (LLVM, Wasm) receives focused attention rather than treating them as interchangeable code emission targets.
