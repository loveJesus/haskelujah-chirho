<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: the dictionary pass keys a literal's `fromInteger` off a sibling argument

**Found:** 2026-09-06, while partitioning the rigid-type-variables lane's workspace failures.
**Status:** open, PRE-EXISTING at `dd6d694a` (reproduced with the untouched HEAD binary).
**Severity:** a valid program dies at run time with `missing STG binding \`fromInteger\``.
**Scale:** 48 of the 537 programs in the curated GHC suite fail this way at `dd6d694a`
(`cargo test -p haskelujah-test-harness --test ghc_curated_chirho`: 489/537, the assertion
expects 537). `T035_either` is the plain case: `main :: IO ()` with
`print (fromRight 0 (Right 42))` — the signed `main` has no scheme predicates, and the literal's
key is guessed from the sibling `Right 42`. Every failure in that list is a missing
`fromInteger`/`+`/`-`/`*`/`read` binding or a literal applied as a function.

## Reproduction

```haskell
module Main where
xs :: String
xs = take 5 "hello world"
main = putStrLn xs
```

`haskelujah run` fails with `missing STG binding \`fromInteger\` for CoreId 3`. The same body
without the signature works, and so does `take 5 [1,2,3]` with or without a signature.

## Mechanism

`rewrite_method_refs_with_locals_chirho` (`crates/haskelujah-core-chirho/src/dict_chirho/rewrite_chirho.rs`)
resolves the literal's `fromInteger` through `collect_method_app_chirho` with a type key taken
from the application's arguments: for `take (fromInteger 5) "hello world"` the key becomes
`Char` (the list's element), `Num Char` has no instance, and the last fallback
(`fallback_dict_for_class_chirho`) only fires when the enclosing binding's scheme carries a
`Num` predicate. An unsigned binding keeps its ground `Num Int` wanted in its scheme, so the
fallback rescues it; a signed binding's scheme is the signature (`String`, no predicates), so
the reference stays bare.

## Where to fix

The literal's own type is known to the type checker; the dictionary pass should receive it as
evidence (the evidence-threading records in `InferResultChirho::method_occurrences_chirho` cover
explicit method references, not literals) instead of guessing from siblings. Until then, do not
"fix" this by widening the fallback: an ambient default dictionary is not proof (see the comment
above `fallback_dict_for_class_chirho`).
