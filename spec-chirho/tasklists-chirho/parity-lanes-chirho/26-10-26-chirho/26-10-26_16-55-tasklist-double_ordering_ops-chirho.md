<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Double ordering at the desugarer's Int primops Chirho

Row 547 was claimed provisionally in #28835 and scoped in #28867. It started from main_chirho `e9192768`, where rows 543 and 545 are closed. It rebased cleanly onto `b202a58a` (rows 546 and 548) and onto `ddb64b43` (row 549).

The desugarer turns infix and section `<`, `<=`, `>` and `>=` into Int's `<#`, `<=#`, `>#` and `>=#` before any type is known (`builtin_primop_name_chirho`, `build_section_body_chirho`). A prefix minus on an operand that is not a float literal becomes Int's `negate#` the same way.

At Double, both backends went wrong:

- **The interpreter refused** every such comparison (`primop LtIntChirho: expected Int#, got (-1.5##, -2.5##)`).
- **The native backends compared IEEE bits as signed integers.** That orders positive Doubles correctly, negative Doubles backwards, and puts NaN and -0.0 wherever their bits fall.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

The control program has 21 lines and prints only Bools.

- **Its lines cover:**
  - negative literals, a negative top-level binding and two Double parameters;
  - NaN on each side of all four operators;
  - -0.0 against 0.0;
  - both section forms;
  - Int and Char ordering, which keep their Int primops.
- **At e9192768:**
  - the interpreter fails on the first line;
  - LLVM and Cranelift each print the wrong Bool on 13 of the 21 lines (1, 3-7, 10-12, 14, 16-18), counted by position.
- **On the candidate,** all 21 lines are GHC's on the interpreter, LLVM and Cranelift.
- **Two lines were added after claude2's #28895.** `print ((-1) / 2 :: Double)` failed with `missing STG binding /` on all three engines; GHC prints `-0.5`. `print (compare ((-1) / 0 :: Double) 0)` gives GHC's `LT`. Both are now GHC's on all three engines, so the control has 23 lines.

## Placement

- **The rule** is `dict_chirho/rewrite_chirho/double_primops_chirho.rs`.
  - It holds the float counterpart of each blind Int primop: `<.#`, `<=.#`, `>.#`, `>=.#` and `negateFloat#`.
  - It applies when an operand keys as Double through `infer_type_key_for_rewrite_chirho`: binder types, type applications, and literal records (row 543).
  - It is called from the primop arm of `rewrite_method_refs_with_locals_chirho`.
- **The type key of `negate#`** is its operand's key, in `type_keys_chirho.rs`, and `Int` only when the operand has none. A negated literal at Double therefore keys `/` at Fractional Double.
- **The new primops** are `<=.#` and `>=.#`, beside `<.#` and `>.#`:
  - the interpreter's `LeFloatChirho` and `GeFloatChirho` (IEEE `<=` and `>=`), with their STG names;
  - LLVM `fcmp ole` and `oge`;
  - Cranelift's ordered `LessThanOrEqual` and `GreaterThanOrEqual`.
  - A NaN on either side gives False, as GHC's `<=##` and `>=##`.
- **Classifiers:**
  - the dictionary pass's Bool result list, the LLVM show classifier and Cranelift's two Bool lists name the float comparisons;
  - LLVM's show classifier had none of them before.
- **The control** is `tests/dictionary_evidence_chirho/double_primops_chirho.rs`. It runs through the interpreter and both native backends.
  - Its native assertion is now shared with row 543's controls as `assert_native_chirho` in `dictionary_evidence_chirho.rs`.

## Boundaries

- **Int, Char and Bool ordering are unchanged.** The control's last three lines pin that.
- **Ordering of Strings, tuples and derived types is a separate lane.** The seeded `Ord` class declares only `compare` (typing `class_chirho.rs:481`). So `"b" < "a"`, tuple `<`, derived `<`, `max` beyond Int, tuple `compare` and prefix `(<)` all still fail in the interpreter (#28867).
- **No change** to the desugarer, type checking, or `^` at Double (`2 ^ 3 :: Double` still reaches Int's `^#`).
- **The gate** is warm, and its negative cases are ordinary tests. The candidate CLI runs once over row 535's saved contract, as the landed rows moved it.

## Checklist

- [x] Measure E1-E6 and the 21-line control on GHC and on the three engines at the base, by position.
- [x] Add `<=.#` and `>=.#` to the interpreter, LLVM and Cranelift, and name the float comparisons in the Bool and show classifiers.
- [x] Retarget the blind Int ordering and negation primops at a Double operand in the dictionary pass.
- [x] Add the all-engine control. It was measured on GHC as written; its constant equals GHC's output.
- [x] Show the control reaches the rule: with only the hook reverted it fails; restored byte-identical, it passes. The dictionary-evidence suite is 77/77.
- [x] Record the rule in the dictionary-evidence workflow and the new primops in the native workflow.
- [x] Key `negate#` by its operand. With only that reverted, the control fails with `missing STG binding /`; restored byte-identical, the dictionary-evidence suite is 78/78.
- [x] Gate r1 on `b202a58a` (tip `d5fc7a62`, before the `negate#` key) was GREEN in 168.86 s.
  - 673/673 focused tests passed.
  - Clippy: none of 456 diagnostic locations fell on the 152 changed lines.
  - The parallel contract pass took 22.95 s.
  - Row 535's judge gave 126/126. Its accounting is now derived from row 535's contract and the copy, at 73 movers and 9 holds. Checked on row 543's r5 run, the derivation reproduced 71 and 11.
  - The judge lists 26 cell changes, which are exactly the landed rows' moves (538-541, 544, 545 and 548). No contract cell moved for row 547.
- [ ] Commit the `negate#` key, run gate r2 on `ddb64b43`, and record it.
