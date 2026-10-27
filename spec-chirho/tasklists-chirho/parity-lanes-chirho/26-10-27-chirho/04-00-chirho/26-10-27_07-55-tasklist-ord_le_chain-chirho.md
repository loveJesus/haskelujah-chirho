<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# GHC's `<=` chain and `compare`'s default Chirho

## Origin and scope

Row 626.
- **Split from row 612:** gpt_chirho #30088 named it as the `<=`-only minimal-definition default.
- **Widened by #30139, on correction #30138:**
  - Row 626 owns GHC's full `<=` chain for `<`, `>` and `>=`, together with `compare`'s `==`/`<=` default.
  - The measured ordinary-`compare`/reversed-`<=` program is a control.
  - Concrete `max`/`min` stays with row 627. `sort` and `comparing` stay with row 633.
- **Branch:** `ord-le-chain-chirho`, from main `d99f8d2d` (row 612 landed as `0459f863` and `2abc1462`).

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **GHC.Classes' chain:**
  - `x < y = not (y <= x)`, `x > y = not (x <= y)`, `x >= y = y <= x`;
  - `x <= y` from `compare`;
  - `compare x y = if x == y then EQ else if x <= y then LT else GT`.
  - An ordinary `compare` beside a reversed `<=` prints `(False,True,True,LT)|(False,MixChirho 1)`.
- **Probes:** `tmp-chirho/le-only-default-chirho/probes-chirho`, with GHC's outputs and the `main612-chirho` and `wip1-chirho` reports.
  - The main reports come from row 612's tip, whose compiler source main `d99f8d2d` carries; only comments changed.
- **Main:**
  - **Silently wrong:** the mixed instance gives `(True,False,False,LT)` on every engine. `compare _ _ = LT` gives `((True,True,False,False),True,False)`, where GHC gives `((False,True,False,True),False,True)`.
  - **Loud:**
    - a `<=`-only instance fails with `missing method Ord.compare`, or with `Ord.<` inside `Ord a =>` code;
    - `compare` through an instance's own `==` fails the same way.
- **The candidate:** six programs move to GHC on all three engines:
  - the mixed instance's operators, written, polymorphic, in a section and first-class (its polymorphic `max` already answered through `<=` on main);
  - a `<=`-only instance directly;
  - a `<=`-only instance through `Ord a =>` code, `sortBy compare`, `maximum` and `minimum`;
  - first-class `(<)` at a `<=`-only key, row 610's old stub-key pin;
  - `compare _ _ = LT`;
  - `compare` through an own `==`.
- **Holds:** an instance that writes only `<` (since row 612).
- **Named failures:** an instance with neither `compare` nor `<=` keeps `missing method Ord.< for VChirho`, written polymorphically or first-class; GHC loops.
- **Kept:**
  - The measured mixed program's concrete `max` cell stays `MixChirho 2` (row 565's helper, row 627).
  - `sort` at user keys and `insert` at any non-Int type are Int-shaped (row 633).

## Placement

- **Dictionary pass:**
  - `instance_chirho/ordering_operators_chirho.rs`: the shared `ordering_answer_chirho` follows the chain through the key's `<=`. That is the instance's own `<=` (`<` and `>` apply `not`), else `compare` read by the `<=` or `>` Ordering test with the operands swapped where GHC swaps them.
  - `instance_chirho/ordering_defaults_chirho.rs`: `compare`'s default from the Eq superclass's real `==` (own, derived, or row 606's default from `/=`) and the instance's own `<=`.
- **Controls:** `tests/dictionary_evidence_chirho/ordering_chirho/ord_chain_chirho.rs` (`ordering_chirho/` has 12 entries). Row 610's stub-key pin now holds an instance with neither `compare` nor `<=`.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, with the row 610 and 612 passages that row 626 changes.
  - The file is 1056 lines, over the 1k aim. Its Ord sections could move to their own document in a later docs row.

## Checklist

- [x] Trace on the row 612 tip; ruling #30139.
- [x] Source: the chain in the shared answer, and `compare`'s default from real `==` and own `<=` bodies.
- [x] Controls: GHC-measured, byte for byte.
  - **First suite run** (1387 tests): 1386 passed. The one failure was an intended mover: row 610's stub-key pin, whose `<=`-only program now answers GHC's `[False]`, moved into the controls, and the pin took an instance with neither body.
- [x] Commit; replay onto the newest main; broad moved-shape scan.
  - The source `294fd482` sits on main `d99f8d2d`.
  - **Scan** (`scan626-chirho.log`): 3587/3587 passed in 750 s, with no mover beyond the intended one. That is row 612's 3585, plus this row's 2 controls.
    - It covered every lib, the parser's included, and the 20 driver suites, leaving out `ghc_bulk`, and skipped the same 18.
    - One harness test, `run_compile_fail_test_pass_chirho`, passed but was flagged leaky under the scan's load. It passed cleanly 3 more times alone.
- [x] **Reach** (`tools-chirho/reach_626_chirho.zsh`, log `reach626-r1-chirho.log`): every mutation killed by the chain controls.
  - **M1, the own `<=` route off:** the mixed instance answers from `compare`, `(True,False,False,LT)`.
  - **M2, the chain through `compare` off** (each operator's own test, unswapped): `compare _ _ = LT` gives `((True,True,False,False),True,False)`.
  - **M3, `compare`'s default off:** `missing method Ord.compare for VChirho`.
  - **M4, the default's `==` branch answering LT:** equal operands compare LT.
  - **M5, the `not` through an own `<=` off:** `(True,False,True,LT)`.
  - **M6, the swap through an own `<=` off:** `(True,True,False,LT)`.
  - Every restore was byte-identical, and the restored tree passes 14/14.
- [x] Judge main against the contract copy; warm gate; record.
  - **Main `d99f8d2d`** is GREEN against the contract copy first (73 movers, 9 holds). Its probes give main's measured answers (`baseline-d99f8d2d-chirho.log`).
  - **r1, GREEN on `294fd482` (base `d99f8d2d`) in 237.36 s:**
    - 1355/1355 focused tests;
    - no Clippy diagnostic on the 260 changed lines in 7 files;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
  - **The tip's probe reports** (`tip-294fd482-chirho`): 7 programs are GHC on all three engines (21 cells).
    - Every compiled Cranelift run carries the marker; the one unmarked log is `LInsertSchemeChirho`, which fails type-checking first.
- [ ] Land on main and close progress row 626 (gpt).
