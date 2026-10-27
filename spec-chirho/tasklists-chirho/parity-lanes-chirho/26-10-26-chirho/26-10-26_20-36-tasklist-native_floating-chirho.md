<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native unary Floating primops through the RTS's libm symbols Chirho

## Origin and scope

Row 575: I claimed it in #29272 and gpt assigned it in #29274.
- It covers `sqrt`, `exp`, `log`, `sin`, `cos`, `tan`, `asin`, `acos` and `atan`, through shared RTS libm calls on LLVM and Cranelift.
- The interpreter is unchanged.
- `pi`, the hyperbolics, `logBase`, `atan2` and the other missing Prelude bindings stay outside.
- The branch starts from main `75feb22f`.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes** are G1-G16 in `tmp-chirho/native-floating-chirho/probes-chirho`.
- **Class A**, exact in the interpreter and refused by both natives: `sqrt#`, `exp#`, `log#`, `tan#`, `asin#`, `acos#` and `atan#`, plus native `sin#` and `cos#`.
  - Now G1-G3, G6-G9 and the top-level-variable probe G15 are exact on all three engines, NaN and ±Infinity included.
  - `sin 1` and `cos 1` are exact in the control.
- **Outside scope:**
  - `sin pi` and `cos pi` (G4 and G5) fail on `pi`;
  - `pi`, `sinh`, `cosh`, `tanh`, `asinh`, `atanh`, `logBase` and `atan2` are missing bindings everywhere.

## Placement

- **RTS.** `float_math_chirho.rs` holds one macro that generates:
  - `extern "C" fn haskelujah_float_<op>_chirho(bits) -> bits`, which calls the f64 libm method;
  - `FLOAT_MATH_PRIMOPS_CHIRHO`;
  - `float_math_symbol_chirho`.
- **Unit table:** `ffi_chirho/arithmetic_tests_chirho.rs` compares each finite answer with GHC's printed digits. Several are π fractions, which Clippy's `approx_constant` would flag as literals.
- **LLVM.** One declaration loop over the table, and one unary branch calling the symbol the lookup names.
- **Cranelift.** `float_math_imports_chirho` imports every symbol from the table. One guarded arm lowers through `float_math_chirho`.
- **Control:** `unary_floating_primops_answer_as_ghc_everywhere_chirho` in `tests/dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs`. It is a 10-line program, measured on GHC as written and run on all three engines.
- **Workflow:** `testing-chirho/native-fail-loud-chirho.md`.

## Checklist

- [x] Measure G1-G16 on GHC and main's three engines; claim with the three-class split (#29272); gpt assigned row 575 (#29274).
- [x] Write the RTS macro, symbols, table and lookup; the LLVM and Cranelift calls; the unit table.
- [x] Probes in scope exact natively; the control measured on GHC as written passes on all three engines.
- [x] Reach mutation: the shared lookup answers None.
  - The control fails with LLVM refusing exactly the nine primops.
  - The power tests hold.
  - The restore was byte-identical, the RTS recompiled, and 6/6 pass.
- [x] Clippy screen on every touched crate with `--tests`: clean.
- [x] Commit, rebase onto current main, judge main against the contract, run the warm gate, and record it.
  - **Main `76ed56e4`,** after row 574, was GREEN against the contract copy.
  - **r1 was GREEN on `9b5dd960` (base `76ed56e4`) in 276.62 s:**
    - 883/883 focused tests;
    - no Clippy diagnostic on the 187 changed lines in 8 files;
    - the judge at 126/126.
- [ ] Land on main and close progress row 575.
