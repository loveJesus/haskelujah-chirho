<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: pattern-synonym signature with an empty `() =>` context invents a `Num ()` wanted

**Status:** open. Found 2026-07-31 while landing the certainly-unsolvable-constraint check.
**Severity:** real false positive — we reject a valid program.

**Not caused by the constraint check.** Verified directly: the reproduction below is
rejected with the identical `no instance for `Num ()`` diagnostic by a binary built from
`8e9030b5` with the check's commit reverted. It reaches the pre-existing
`!entails_chirho(...)` error path in `check_deferred_preds_chirho`, so no gate in the new
check suppresses it and none should be added for it — gating `PatternSynonyms` was tried,
did not fix this program, and only cost a genuine reject-axis win. Fix the lowering.

## Reproduction

Valid Haskell that we wrongly diagnose (with the gate removed):

```haskell
{-# LANGUAGE PatternSynonyms #-}
module Q1 where
pattern Q :: () => Int -> Maybe Int
pattern Q x = Just x
```

→ `error[E0204]: no instance for `Num ()`` pointing at the `()` in the signature.

Minimal contrast set, all run through `./target/debug/haskelujah check`:

| Program | Result |
|---|---|
| `pattern Q :: Int -> Maybe Int` (no context) | accepted (correct) |
| `pattern Q :: () => Int -> Maybe Int` | **`Num ()` invented** |
| `pattern Q :: (Eq a) => a -> Maybe a` | accepted (correct) |
| `f :: () => Int -> Int` (ordinary binding) | accepted (correct) |
| `g :: ()` / `h :: ((), Int)` (unit as a type) | accepted (correct) |

So the defect is specific to an **empty tuple in the constraint position of a
pattern-synonym signature** — ordinary bindings with `() =>` are fine, and `()` as an
ordinary type is fine.

## Corpus evidence

`ghc-tests-chirho/typecheck-chirho/should_fail/PatSynExistential.hs` is rejected by us with
`no instance for `Num ()`` at 5:14. GHC rejects it too, but for an unrelated reason
(`[GHC-33973]`: the result type of the signature mentions existential `x`). Right verdict,
wrong reason — it was counted as a win by the reject-axis counting rule until the gate
removed it.

## Where to look

`lower_pat_syn_decl_chirho` (`crates/haskelujah-parser-chirho/src/lower_chirho.rs:4699`)
walks tokens looking for `=` / `<-` and never handles the `::` signature form: for
`pattern Q :: …` no separator is seen, so the type node is skipped and the declaration is
built without its signature. The spurious `Num` wanted is created downstream of that
mis-lowering — the empty-tuple constraint ends up in a term position where numeric-literal
defaulting attaches `Num`. Confirm with the contrast table above before changing anything;
the fix belongs in signature lowering, not in the constraint checker.

## Acceptance for the fix

1. All five contrast programs behave as the table says (the `() =>` pattern-synonym
   signature is accepted).
2. Full should_compile pass stays a subset of the known failing set, and the reject count
   does not drop. `PatSynExistential` must still be rejected afterwards — but verify it is
   then rejected for the *right* reason (existential in the result type, `[GHC-33973]`),
   not for `Num ()`.
