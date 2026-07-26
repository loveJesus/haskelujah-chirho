<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# GHC should_compile burn-down — toward drop-in parity

Goal context: Haskelujah as a drop-in replacement for GHC. Current measured
typecheck parity: **850/938 (90.6%)** on `ghc-tests-chirho/typecheck-chirho/should_compile`.
This tasklist burns down the 88 remaining failures in root-cause clusters.

## Ground rules
- Acceptance runs the CUMULATIVE surface (watch-127 / full driver), never the target list alone.
- No hack/cheat passes: fix the compiler, do not loosen the corpus or special-case a filename.
- Only commit files this lane owns; leave cross-lane dirty files untouched.
- Every published number comes from a timed, regenerated artifact.

## Root-cause clusters (measured 2026-07-26, fresh release bin)

Verified: 0 of the 88 pass now, 0 time out — the 850/938 figure is accurate.

| # | Cluster | Files | Root cause |
|---|---------|-------|-----------|
| A | Stuck type-family equalities | ~18 | `unify` hard-fails on `F <metavar> ~ T` instead of deferring |
| B | Kind errors, `expected * / found k -> k'` | ~27 | shared SIGNATURE, cause only partly identified (see below) |
| C | Unbound variable | 13 | mixed: `proc`/arrows, `do` forms, `..` in pattern bindings, `@` type patterns |
| D | Long tail | ~30 | impredicativity, improvement, injectivity, TypeError |

### Cluster A evidence (minimal repro, both written this session)
- `F Int` (arg known) reduces -> **passes**.
- `F <metavar>` (arg unresolved) -> `error[E0200]: expected Bool, found (F t12)`.

GHC defers the equality (flattening: stuck family app becomes a fresh meta var plus a
deferred equality) and generalizes over it. We hard-fail. Single hook point:
`infer_chirho.rs:1509`, the one fall-through to structural `unify_chirho`.
Narrow the deferral to heads registered in `type_families_chirho` AND genuinely stuck,
so fully-applied wrong equations still get rejected.

### Cluster B — signature shared, cause NOT yet fully identified
13 files are literally `expected *, found k -> k'` (plus the mirror). The obvious guess
(poly-kinded constructors not instantiated) was tested and is WRONG: plain `PolyKinds`
+ `data Proxy (a :: k)` + `Proxy Maybe` all check clean today.

What IS confirmed is a narrower cause, reproduced in 6 lines — a GADT record whose
existential appears only in the fields, never in the result type:

```haskell
{-# LANGUAGE GADTs #-}
data T where
  MkT :: { f :: a -> Int, x :: a } -> T
foo t = t { f = length, x = "hi" }
-- error[E0300]: kind mismatch in kind application: expected `*`, found `k1 -> k2`
```

The kind checker appears to treat the existential `a` as a parameter of `T`. That covers
T3632 and the other GADT-record files; the remaining cluster-B files still need
individual triage before anyone claims one fix covers 27.

## MEASURED: the compatibility claim is ONE-SIDED, and the other side is bad

We publish should_compile (850/938 = 90.6%) but had never measured **should_fail**.
Measured this session (artifact: `spec-chirho/ghc-should-fail-measurement-chirho.txt`):

**should_fail: 141 / 767 correctly rejected = 18.4%. We ACCEPT 626 programs GHC rejects.**

A drop-in GHC must accept what GHC accepts AND reject what GHC rejects. Quoting only
should_compile overstates compatibility — a checker that accepted everything would score
100% on that axis. Verified genuine (not a harness artifact): malformed source still
errors, `f :: Int -> Bool; f x = x` is still rejected, and 4 hand-sampled accepted files
are real GHC type errors (fundep violation, rigid skolem, superclass loop, newtype ctx).
141 is an UPPER bound — it counts any emitted error, not a correct one.

### This inverts the burn-down priority
Cluster A (deferring stuck family equalities) makes the checker MORE permissive. Landing
it would raise should_compile and push should_fail further down, in a checker that is
already unsound on 81.6% of GHC's rejection corpus. **Do not land Cluster A as a
standalone "compat win."** Soundness is now the bigger gap and should lead.
Every typechecker change must re-measure BOTH corpora.

### Soundness triage — the 626 misses, clustered by GHC's OWN expected error
The corpus ships 710 `.stderr` files (GHC's expected diagnosis), so the misses can be
clustered by what GHC actually says. First-match priority, 60 of the 626 have no `.stderr`:

| Files | GHC says | What we are missing |
|-------|----------|---------------------|
| 118 | `Couldn't match type` | core unification rigor |
| 70 | `Illegal … / not allowed` | well-formedness + validity checks |
| 56 | kind errors | kind checking rigor |
| 54 | `No instance for` | instance resolution not enforced |
| 42 | `Could not deduce` | missing-constraint checking |
| 26 | rigid type variable / escape | skolem escape not detected |
| 11 | `Ambiguous type` | ambiguity check |
| 10 | `Not in scope` | scoping |
| 7 | overlapping instances | overlap check |
| 169 | long tail | many distinct validity checks, 2-3 files each |

The long tail is not one bug — it is a list of checks GHC performs and we never wrote:
newtype constructor with a context, type-synonym arity, family injectivity violation,
"not a visible associated type", family parameter-count mismatch, conflicting class
definitions, 65-tuple limit, `main` not defined in `Main`, and so on.

**Why this cluster is attractive regardless of the strategy call:** validity checks only
ADD rejections for programs GHC also rejects, so they raise should_fail without putting
should_compile at risk — the opposite risk profile from Cluster A.

### Also found (cheap, unrelated): RecursiveDo is entirely unimplemented
A 6-line `rec`-in-do program fails with `unbound variable: ``` (empty name). Not a
RecordWildCards bug as the failing test T4404 suggests — plain `rec` alone reproduces it.

## Steps
- [x] Verify binary currency before measuring (found `target/release` 4 months stale; rebuilt)
- [x] Re-cluster the 88 failures by root cause with the current binary
- [x] Rank clusters by (files recovered) / (blast radius)
- [x] Establish the missing should_fail baseline BEFORE touching the typechecker
- [ ] Cluster A: defer stuck type-family equalities
- [ ] Cluster B: kind polymorphism instantiation
- [ ] Re-measure BOTH corpora; regenerate artifacts; update README/site only if numbers move
- [ ] Full driver sweep to certify zero regressions before landing
