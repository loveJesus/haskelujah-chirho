<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ordering operators through a body-backed compare Chirho

Row 551 is the first brick of the Ord lane. It was claimed in #28918, ruled in #28919, and is bounded so that it does not change the class model. It started from main_chirho `72a338e8` and was moved onto `cebd8f9b` (row 550) before its first commit.

The desugarer turns infix and section `<`, `<=`, `>` and `>=` into Int primops before any type is known. Row 547 gave Double its float primops. Every other type met the Int primop as a heap value or a C string:

- **the interpreter refused** them (`LtIntChirho: expected Int#`);
- **the native backends compared pointers or tags.**

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

The probes put the greater value first: `"b"`/`"a"`, `[1, 3]`/`[1, 2]`, `GreenChirho`/`RedChirho`, `PairChirho 1 3`/`PairChirho 1 2`, and `GT`/`LT`.

| type | interpreter at base | LLVM at base | Cranelift at base | all three now |
|---|---|---|---|---|
| String `<`, `>=` | refused | WRONG | WRONG (C-string pointer order) | GHC's |
| `[Int]` | refused | WRONG | right by accident | GHC's |
| derived record | refused | WRONG | right by accident | GHC's |
| derived enum, Ordering | refused | right (tag order) | right | GHC's |
| `compare` on each | GHC's | GHC's | GHC's | unchanged |

- The base cells were measured with the hook reverted on this tree.
- `Maybe Int` and `(Int, Int)` have no body-backed compare row, so they do not move.
  - The interpreter still refuses them.
  - Natively, tuple `<` and `>=` are still WRONG on LLVM, compared by pointer. That is a native boundary for brick 2, recorded here and not fixed.

## Placement

- **The Ordering tests** are in `dict_chirho/prelude_chirho/ordering_tests_chirho.rs`.
  - It generates `$ordering_lt_chirho`, `$ordering_le_chirho`, `$ordering_gt_chirho` and `$ordering_ge_chirho`, each `\o -> case o of { C -> v; _ -> not v }`.
  - `<` accepts LT, `<=` rejects GT, `>` accepts GT and `>=` rejects LT. Those are GHC's `Ord` class defaults.
  - One table (`ORDERING_TESTS_CHIRHO`) maps each primop to its test, for both the generator and the route.
- **The route** is in `dict_chirho/rewrite_chirho/compare_route_chirho.rs`. It is called first from the primop arm of `rewrite_method_refs_with_locals_chirho`, after the operands are rewritten. Row 547's Double step follows.
  - The route takes an operand's concrete key. Int, Char, Bool and Double keep their primops.
  - Otherwise `lookup_dispatch_body_name_id_chirho("$prim_Ord_compare_<key>")` must find a body-backed row, not a generated missing-method stub. This is the resolver compare dispatch trusts, so no bare `compare` placeholder is minted.
  - When it finds one, the primop becomes `test (compare a b)`. `compare` is evaluated once, because the test scrutinizes its argument once.
- **The controls** are in `tests/dictionary_evidence_chirho/ordering_chirho/compare_route_chirho.rs`, measured on GHC as written:
  - a 23-line program runs on all three engines: String, `[Int]` with a prefix case, derived enum and record, Ordering, both sections, and unchanged Int, Char, Bool and Double lines;
  - the interpreter keeps failing on `Maybe`, tuple, `max` and prefix `(<)` (GHC: False, False, b, False);
  - structurally, `main = print ("b" >= "a")` mentions the `[Char]` compare row exactly once and no `>=#`.

## Boundaries

- **Ruled out of this brick (#28919):** `Maybe` and tuple compare (brick 2), `max`, `min`, `maximum` and `minimum` (the next stacked Ord-aggregate brick, reusing this resolver), prefix `(<)`, and the class model.
- **Overlap with row 552** (claude2_chirho, sum and product): both rows hook the method-reference walker. Each uses a separate child module and one narrow hook. Whichever lands second rebases and reruns its focused gate.
- **The gate** is warm, and its negative cases are ordinary tests. The candidate CLI runs once over row 535's saved contract.

## Checklist

- [x] Measure `compare`, `<` and `>=` across eight types, interpreter first, against GHC.
- [x] Find the trusted resolver and confirm which keys have body-backed compare rows (`[Char]`, `[Int]`, derived types, `Ordering`) and which do not (`Maybe Int`, `(Int, Int)`).
- [x] Generate the four Ordering tests and route through them.
- [x] Move onto `cebd8f9b` and resolve the one module-list conflict with row 550.
- [x] Add the controls. Their GHC outputs equal the constants as written, and the dictionary-evidence suite is 81/81.
- [x] Reach check: with only the hook reverted, the route and once controls fail and the boundary control passes. Restored with a fresh mtime and verified to recompile core, all three pass.
- [x] Record the route in the dictionary-evidence workflow.
- [x] Commit (`2835e3d1`), run the warm gate, and record it.
  - Gate r1 on `cebd8f9b` is GREEN in 207.09 s.
  - 681/681 focused tests passed, the three row 551 controls included.
  - Clippy: none of 456 diagnostic locations fell on the 279 changed lines.
  - The parallel contract pass took 23.58 s.
  - Row 535's judge gave 126/126, at 73 movers and 9 holds. Its 26 cell changes are exactly the landed rows' moves, so no contract program moved for row 551.
  - The gate's build step was a no-op. The binaries are current because the reach check's restore recompiled core after the last source edit. The route controls pass only with the hook compiled in.
