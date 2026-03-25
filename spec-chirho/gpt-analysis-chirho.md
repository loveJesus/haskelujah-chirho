<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# GPT Analysis Chirho

## Stable State

- Version: `v0.1.10`
- Passing Hackage packages: `28`
- Curated tests: `537/537`
- Typing tests: `188`
- Parser tests: `183`

## Current Containers Analysis

User-reported live blocker:

- package: `containers-0.8`
- module: `Data.Sequence.Internal`
- error shape from package build: `type mismatch: expected FingerTree, found (-> t)` at offset `65862`
- file location for the offset: line `1890`, column `15`

Relevant source slice around the offset in
[Data.Sequence.Internal.hs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/.haskelujah-packages-chirho/containers-0.8/src/Data/Sequence/Internal.hs):

```hs
{-# SPECIALIZE consTree' :: Elem a -> FingerTree (Elem a) -> FingerTree (Elem a) #-}
{-# SPECIALIZE consTree' :: Node a -> FingerTree (Node a) -> FingerTree (Node a) #-}
consTree'        :: Sized a => a -> FingerTree a -> FingerTree a
consTree' a EmptyT       = Single a
```

### What I verified

Standalone file checking is still blocked earlier by sibling-module discovery, so it does **not** currently reach the `FingerTree` mismatch:

- `Utils.Containers.Internal.Prelude`
- `Utils.Containers.Internal.State`
- `Utils.Containers.Internal.StrictPair`

That means the `consTree'` / `FingerTree` mismatch is only visible in package-build mode after more of the package graph is compiled.

### Likely Root Cause

This does **not** look like a plain top-level forward-reference miss in `infer_chirho.rs`:

- top-level `FunBindChirho` names are already pre-bound before SCC inference
- `consTree'` itself is a normal top-level function binding
- the suspicious source location is exactly where the `SPECIALIZE` pragmas sit immediately above the function signature

Most likely explanations, in descending order:

1. `SPECIALIZE` pragma handling is still polluting the typing path for the following binding in package-build mode.
2. Package-build/module-resolution state changes the local environment before `consTree'` is inferred, so the error only appears once sibling modules are available.
3. There is a second-order ordering issue involving the source graph, but not a simple “later function isn’t pre-bound” bug.

## Current Free / Profunctors Analysis

### free

Standalone
[Control.Applicative.Trans.Free.hs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/.haskelujah-packages-chirho/free-5.2/src/Control/Applicative/Trans/Free.hs)
now `check`s cleanly.

Full package build currently stops later on transitive `stm` primops:

- `mkWeak#`
- raw `#` tokenization around that primop lane

So `free` is no longer blocked by the earlier `ApF` frontend/type mismatch on this head.

### profunctors

`profunctors` is currently blocked earlier by transitive `bifunctors` Template Haskell surface, not by `Data.Profunctor.Unsafe` itself:

- missing `union`
- missing `AppE`
- missing helper names like `mkSimpleLam`, `mkSimpleTupleCase`, `foldDataConArgs`

## Recommended Next Step

For `containers`, do **not** attack the `FingerTree` mismatch blind from standalone check mode. First make package-build diagnostics reproducible at the exact `consTree'` site with sibling modules available, then test one narrow change:

1. make sure package-build reaches `Data.Sequence.Internal`
2. temporarily ignore or skip `SPECIALIZE` pragmas during typing for the immediately following binding
3. re-run:
   - `target/debug/haskelujah build .haskelujah-packages-chirho/containers-0.8/`
   - `target/debug/haskelujah build .haskelujah-packages-chirho/transformers-0.6.3.0/`
   - `target/debug/haskelujah build .haskelujah-packages-chirho/mtl-2.3.2/`
   - `target/debug/haskelujah build .haskelujah-packages-chirho/parsec-3.1.18.0/`

That is the safest next lane from the current stable `28`-package state.
