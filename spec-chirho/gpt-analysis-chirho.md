<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# GPT Analysis Chirho

## 2026-03-26 Adjunctions Rep Analysis Chirho

### Stable State Chirho

- Head observed during this investigation: `7ea5030f`
- `stm-2.5.3.1` now builds cleanly: `Compiled 10 modules from package 'stm'`
- User-reported live target remains `adjunctions`

### Exact Reproducer Chirho

The full package build is still noisy because earlier transitive packages fail first:

- `bifunctors`: `Data.Bifunctor.TH`
- `profunctors`: downstream missing `Data.Bifunctor.*`
- `semigroupoids`: downstream missing `Data.Biapplicative` / `Data.Bifunctor.*`
- `free`: TH surface issues in `Control.Monad.Free.TH`

That means the cleanest reproducer is **not** the package build. The direct module check is:

```sh
target/debug/haskelujah check .haskelujah-packages-chirho/adjunctions-4.4.4/src/Data/Functor/Contravariant/Rep.hs
```

That currently fails with the real local symptom:

- `index Proxy _ = ()` reports `expected (), found Proxy` and the reverse mismatch
- `tabulate (fst . f)` / `tabulate (snd . f)` in the `Product` instance report `expected (,) , found Product`
- the `:*:` instance shows the same tuple-vs-product shape

### Relevant Source Facts Chirho

The source in
[Data.Functor.Contravariant.Rep.hs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/.haskelujah-packages-chirho/adjunctions-4.4.4/src/Data/Functor/Contravariant/Rep.hs)
contains the expected associated type equations:

- `type Rep Proxy = ()`
- `type Rep (Product f g) = (Rep f, Rep g)`
- `type Rep U1 = ()`
- `type Rep (f :*: g) = (Rep f, Rep g)`

So the intended reduction target is confirmed from source.

### What I Verified Chirho

I checked the current type-family plumbing in
[infer_chirho.rs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/crates/haskelujah-typing-chirho/src/infer_chirho.rs):

- `process_class_decl_chirho` registers associated families like `Rep` as open families
- `process_instance_decl_chirho` registers associated family instances from instance bodies
- `reduce_type_families_in_ty_chirho` reduces recursively and already has the newer-registration and qualified-head fallback logic from the earlier fix
- existing regression tests already cover `Rep (Product f g)` reduction in the synthetic associated-type path

I also confirmed an important structural risk:

- `ClassEnvChirho` is keyed only by bare class name
- `type_families_chirho` is keyed only by bare family name

That means `Data.Functor.Rep.Representable` and `Data.Functor.Contravariant.Rep.Representable` can collide later, since both define a bare class `Representable` with a bare associated family `Rep`.

### Most Important Finding Chirho

The direct module-local reproducer means the immediate bug is **not only** an imported-package collision.

Why:

- `target/debug/haskelujah check .../Data/Functor/Contravariant/Rep.hs` already fails on the single module
- that path does not need the transitive `bifunctors` / `profunctors` package graph to reproduce the `Proxy` and `Product` mismatches

So there are two layers:

1. A **local** bug in the `Contravariant.Rep` typing path
2. A **future/secondary** collision risk from bare-name class / family registration across `Data.Functor.Rep` and `Data.Functor.Contravariant.Rep`

### Likely Immediate Bug Chirho

The local symptom now looks like one of these two:

1. The lowered AST or stored class method scheme for `index :: f a -> a -> Rep f` is wrong before instance specialization
2. The instance-method expected-type specialization path warps that scheme before reduction

What I checked already:

- `infer_matches_against_expected_chirho` splits arrows left-to-right and did not look obviously reversed
- `SubstChirho::apply_ty_chirho` looked structurally normal
- `ast_type_to_ty_chirho` / `ast_type_to_scheme_chirho` looked structurally normal on inspection

What is still missing:

- a regression that inspects the lowered AST for `index :: f a -> a -> Rep f`
- a regression that inspects the specialized expected type for the `Proxy` instance before method-body checking

### Concrete Next Steps Chirho

The next session should do these in order:

1. Add a parser/lowering regression in
   [lower_chirho.rs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/crates/haskelujah-parser-chirho/src/lower_chirho.rs)
   for:
   - `class Representable f where`
   - `type Rep f`
   - `index :: f a -> a -> Rep f`
   - assert the lowered AST preserves `f a -> a -> Rep f`

2. Add a typing regression in
   [infer_chirho.rs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/crates/haskelujah-typing-chirho/src/infer_chirho.rs)
   that manually constructs:
   - the contravariant `Representable` class
   - the `Proxy` instance with `type Rep Proxy = ()`
   - the `Product` instance with `type Rep (Product f g) = (Rep f, Rep g)`
   - then asserts the instantiated expected type for `index` is `Proxy a -> a -> ()`
   - and the instantiated expected type for `tabulate` in `Product f g` sees `(Rep f, Rep g)`

3. If the lowering regression fails:
   - fix the frontend shape first

4. If the lowering regression passes but the typing regression fails:
   - fix `process_class_decl_chirho` / `instantiate_instance_method_expected_ty_chirho` / associated-family reduction in the specialization path

5. After the direct module check is green, rerun:
   - `target/debug/haskelujah check .haskelujah-packages-chirho/adjunctions-4.4.4/src/Data/Functor/Contravariant/Rep.hs`
   - `cargo test -p haskelujah-parser`
   - `cargo test -p haskelujah-typing`
   - `target/debug/haskelujah build .haskelujah-packages-chirho/adjunctions-4.4.4/`

### Commands Run Chirho

```sh
target/debug/haskelujah build .haskelujah-packages-chirho/stm-2.5.3.1/
target/debug/haskelujah build .haskelujah-packages-chirho/adjunctions-4.4.4/
target/debug/haskelujah check .haskelujah-packages-chirho/adjunctions-4.4.4/src/Data/Functor/Contravariant/Rep.hs
```

The single-module `check` command is the one that isolates the `()` vs `Proxy` issue cleanly.

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
