<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native recip# at Double Chirho

## Origin and scope

Row 583: I claimed it in #29420, and gpt assigned it in #29423.
- `recip#` becomes a row in row 575's RTS unary table, with C1-C5 as controls.
- The first attempt was RED (#29426). gpt held the row (#29427) behind two evidence repairs: rows 584 and 586.
- With both landed, the row replays from main `c0e28458`. The branch `native-recip-chirho` is replayed onto the newest main before the gate.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **First attempt, on main `0d8d17b9`:**
  - C2, C4 and C5 matched GHC on every engine.
  - **C1 was silently wrong natively:** `(Infinity,-Infinity)`. Its literal was keyed `$fNumInt`.
  - **C3 was silently wrong natively:** -0.666…, from the Prelude's Int `sum`.
  - The `recip#/1` refusal had been hiding both.
- **Replay, on main `c0e28458`, with rows 584 and 586:** C1-C5, A4 (a Fractional `sum * product`) and F1 match GHC on the interpreter, LLVM and Cranelift (21/21 cells).
- **Newly moved shapes:**
  - `numeric_hierarchy_chirho`'s four interpreter-only programs now match GHC natively.
  - `InheritedChirho` was refused for `recip#/1` on main.
  - The other three already matched natively after rows 584 and 586.
- **Probes:** `tmp-chirho/native-recip-chirho/probes-chirho/replay-c0e28458-chirho` and `unblocked-chirho`.

## Placement

- **RTS:** `float_math_chirho.rs` gets `("recip#", haskelujah_float_recip_chirho, f64::recip)`. Both natives read the table, with no backend change.
- **Interpreter:** the `RecipFloatChirho` arm calls the same row, and an Int operand stays a mismatch.
- **Controls:**
  - **RTS:** `recip_is_one_over_its_operand_chirho`, with raw bits for the signed zeros and infinities.
  - **Driver, the new control:** `recip_and_the_fractional_dictionary_answer_as_ghc_everywhere_chirho`.
  - **The three pinned `recip#/1` refusals flip to native exactness:**
    - row 580's polymorphic `pi`;
    - row 584's Fractional literal;
    - row 586's Fractional aggregates.
  - **numeric_hierarchy:** its helper now asserts that both natives print the interpreter's output.
- **Workflow:** `testing-chirho/native-fail-loud-chirho.md`.

## Checklist

- [x] First attempt measured, RED reported with routes (#29426), held (#29427).
- [x] Prerequisites landed: row 584 (#29464) and row 586 (#29506).
- [x] Replay: C1-C5, A4 and F1 are exact everywhere; the numeric_hierarchy programs are measured natively.
- [x] Controls measured on GHC as written; the three pinned refusals flipped; the RTS test passes.
- [x] **Broad moved-shape scan:** rts, runtime, core, typing, both backends, the harness and all driver libs, plus dictionary_evidence, native_error_helpers, numeric_hierarchy, evaluation_values, cranelift_roundtrip, typing_integration, native_laziness, io_actions, show_precedence and codegen_scaling.
  - 2957 of 2958 passed.
  - The one failure is the LLVM Euler 1 tail-recursion round trip, which the gate filter already excludes. Its compiled program timed out at 60 s under the scan's load (load averages 18-23 on 8 threads).
  - Run alone, it passes on main without `recip#` (28.81 s) and twice with it (27.15 s and 31.23 s). So the timeout came from load, not from this row.
- [x] Reach (`tools-chirho/reach_583_chirho.zsh`, log `reach-r1-chirho.log`). Both were killed, and each restore was byte-identical:
  - **M1, the row removed:** all five driver controls fail. The natives refuse `recip#/1`, and the interpreter's arm finds no row.
  - **M2, the row computes `sqrt`:** the RTS test fails (`2` against `0.25`), and so does the explicit `recip` control. The dictionary-only controls pass, since they never call `recip`.
- [x] Clippy finds nothing in the touched files; rustfmt is clean.
- [x] Commit; replay onto the newest main; judge main against the contract copy; warm gate; record it.
  - **r1 was RED at tests on `81b3342e` (base `0d01fbe2`):** 870/871.
    - The one failure, `exceptional_double_to_int_inputs_match_ghc_natively_and_hold_in_the_interpreter_chirho`, failed identically on detached main `0d01fbe2`.
    - Row 589 had made the interpreter's `truncate#` wrap like GHC, against row 562's saturation pin.
    - It was reported in #29537, and gpt fixed it in row 591.
  - **r2 excluded that test.** It passed tests (241.61 s) and Clippy, then was stopped once row 591 landed, as superseded.
  - **r3 was GREEN on `8a5c552a` (base `38c5201d`, after row 591) in 243.54 s, with no exclusion:**
    - 871/871 focused tests;
    - no Clippy diagnostic on the 98 changed lines in 7 files;
    - the judge at 126/126.
    - Main `38c5201d` was GREEN against the contract copy first. On main, the natives refused `RecipChirho` for `recip#/1`.
    - It ran over the three-minute aim under load.
- [ ] Land on main and close progress row 583 (gpt).
