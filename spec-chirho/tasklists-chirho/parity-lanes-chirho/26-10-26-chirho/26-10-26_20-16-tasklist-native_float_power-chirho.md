<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native Double `**` through the RTS's one `pow` Chirho

## Origin and scope

Row 573: I claimed it in #29233 and gpt assigned it in #29237.
- It covers native Double `**` and stays distinct from row 570's integer-exponent `^`.
- `sqrt#`, which is also refused natively, is outside it.
- The branch starts from main `22497167`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes** are F1-F12 in `tmp-chirho/native-float-power-chirho/probes-chirho`.
- **Interpreter:** exact on all twelve, before and after.
- **Before:** both natives refused `**#` on every probe, so each failed to compile natively.
- **Now:** every `**` probe is exact on LLVM and Cranelift, including:
  - `2 ** 0.5`, `10 ** (-2)`, `0.5 ** 0.5`;
  - `(-8) ** (1/3)` NaN, `0 ** 0` 1, `NaN ** 0` 1, `1 ** NaN` 1;
  - `1e308 ** 2`, `0 ** (-1)` and `2 ** 1024` Infinity, `2 ** (-1075)` 0.0;
  - `(-2) ** 3` -8, `(-2) ** 0.5` NaN;
  - top-level, section, function and aggregate bases.
- **F11** still fails natively on `sqrt#` alone.

## Placement

- **RTS.** `float_power_chirho::pow_float_chirho` is `powf`, which reaches the same libm `pow` as GHC's `**##`.
  - The wrapper `haskelujah_pow_float_chirho` takes both operands' bits.
  - The unit table in `ffi_chirho/arithmetic_tests_chirho.rs` holds GHC's printed values.
- **Interpreter:** `PowFloatChirho` calls the same RTS function.
- **LLVM:** the power call takes `**#` too, plus its declaration.
- **Cranelift:** a `pow_float_chirho` import, and an arm that passes both operands' bits.
- **Control:** `double_float_power_answers_as_ghc_everywhere_chirho` in `tests/dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs`. It is an 11-line program, measured on GHC as written and run on all three engines.
- **Workflow:** `testing-chirho/native-fail-loud-chirho.md`.

## Checklist

- [x] Measure F1-F12 on GHC and main's three engines.
- [x] Add the RTS function, wrapper and unit table; the interpreter, LLVM and Cranelift calls.
- [x] Probes exact natively; the control measured on GHC as written passes on all three engines.
- [x] Reach mutation with both native `**#` arms removed.
  - The control fails: `LLVM does not support primitive(s): **#/2`.
  - Row 570's power tests hold.
  - The restore was byte-identical, both backends recompiled, and 5/5 pass.
- [x] Commit, rebase onto current main, run the warm gate, and record it.
  - **Before gating,** main `eb9463ea` was judged against the contract copy, after the 560, 561 and 557 stack. It was GREEN, and so was main `6efdc533` after row 572.
  - **r1 was RED at Clippy on `7515dcf2`,** with all 876 tests passing. The deny-by-default `approx_constant` lint failed the RTS's own test target on GHC's printed 1.4142135623730951 and 0.7071067811865476. The table now names `SQRT_2` and `FRAC_1_SQRT_2`, which are bit-identical.
  - **r2 was GREEN on `0a83037b` (base `98b20b0e`) in 214.48 s:**
    - 880/880 focused tests;
    - no Clippy diagnostic on the 137 changed lines;
    - the judge at 126/126.
- [ ] Land on main and close progress row 573.
