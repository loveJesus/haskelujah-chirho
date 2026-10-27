<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Hyperbolic Floating methods and logBase at Double Chirho

## Origin and scope

Row 578: I claimed it after row 575 landed, and gpt assigned it in #29302.
- It covers `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh` and `logBase` at Double, on every engine, over row 575's RTS table.
- `pi` and `atan2` stay separate (proposed as X2).
- The branch `floating-hyperbolic-chirho` starts from main `5463c8b4`. It is replayed onto `0d4a5b8e` (row 577) before the gate.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Before:** the checker typed all seven, but no Core bound them, so every engine failed with a missing binding.
- **GHC equals libm** on every probe value. Probes are G10-G18 in `tmp-chirho/floating-hyperbolic-chirho/probes-chirho`.
- **Rust's std is not libm** for `asinh`, `acosh` and `atanh`. At 1.8, 1.2 and -0.9 std gives 1.3504407402749723, 0.6223625037147785 and -1.4722194895832208 (rustc 1.93.0). GHC prints libm's 1.3504407402749725, 0.6223625037147786 and -1.4722194895832204.
- **`sinh 710` is finite** (1.1169973830808555e308). `sinh 711` overflows to Infinity.
- **Found while probing, a program's own top-level names:** the generated Prelude alias took the id of a top-level binding of the same name and replaced it. Probes S1-S5 are in `probes-chirho/shadow-sources-chirho`.
  - S4 hides the Prelude's `sin` and defines its own. It printed the Prelude's answer on every engine, before this row.
  - S1 and S3 do the same with `logBase` and `sinh`. This row's new aliases would have overridden them too.
  - Local shadows (S2 `where`, S5 `let`) were already right.

## Placement

- **RTS.** Six more rows in `float_math_chirho.rs`'s macro table. `asinh`, `acosh` and `atanh` bind libm's own symbols through `unsafe extern "C"`. `apply_float_math_chirho` applies the same function for the interpreter.
- **Interpreter.** Six `PrimOpKindChirho` variants and their STG names. All fifteen unary arms call `apply_float_math_chirho` through one helper.
- **Natives.** No backend change: LLVM and Cranelift read the table.
- **Prelude.** The six names join `generate_floating_prelude_chirho`'s list. `logBase` is generated in the new child module `dict_chirho/prelude_chirho/floating_methods_chirho.rs` as `\x y -> log# y /.# log# x`.
- **The alias rule.** The existing `push_prelude_alias_chirho` now yields to a name in `body_backed_names_chirho`, the program's own top-level bindings. The Floating aliases now go through it. Its other caller binds qualified `Data.Map.*` names, which no top-level binding can share.
- **Controls:**
  - RTS: `hyperbolic_floating_primops_answer_as_ghc_chirho`.
  - Driver: `hyperbolic_methods_and_log_base_answer_as_ghc_everywhere_chirho` and `a_programs_own_floating_names_stay_its_own_everywhere_chirho`. Each is measured on GHC as written and runs on all three engines.
- **Workflow:** `testing-chirho/native-fail-loud-chirho.md`.

## Checklist

- [x] Measure on GHC and claim. gpt assigned row 578 (#29302).
- [x] Write the RTS rows with libm's inverse hyperbolics, the interpreter variants and helper, the STG names, the Prelude list and `logBase`.
- [x] Probes G10-G18: 18/18 cells equal GHC on the interpreter, LLVM and Cranelift.
- [x] Shadow probes S1-S5: the alias rule makes 15/15 cells equal GHC.
- [x] Controls are byte-equal to the GHC-measured sources and outputs. RTS 5/5 and driver `double_primops` 13/13 pass.
- [x] Reach mutations (`tools-chirho/reach_578_chirho.zsh`, log `reach-r1-chirho.log`). Each was killed, and each restore was byte-identical:
  - **M1:** std's inverse hyperbolics. The RTS table and the all-engine control fail.
  - **M2:** no alias guard. The own-names control prints `(10.0,0.8414709848078965)`.
  - **M3:** `logBase` operands swapped. The control's logBase lines fail.
  - **M1b** on the restructured RTS test: killed.
- [x] Clippy screen with `--tests` on every touched crate: nothing on the changed lines. rustfmt is clean.
- [x] Commit, rebase, judge main against the contract copy, run the warm gate, and record it.
  - **On `0d4a5b8e`:** main was GREEN against the contract copy. r1 was GREEN on `f2ed419f` in 238.38 s: 888/888 focused tests, no Clippy diagnostic on the 297 changed lines in 8 files, and the judge at 126/126.
  - **Row 576 landed (main `8650a568`).** The branch replayed cleanly as `613b103f`, and main `8650a568` was GREEN against the contract copy.
  - **r2 was GREEN on `613b103f` (base `8650a568`) in 209.33 s:** 891/891 focused tests, no Clippy diagnostic on the 297 changed lines in 8 files, and the judge at 126/126 (73 movers, 9 holds).
- [x] Check the alias override's reach: hidden and redefined `not` and `abs` were already right on every engine, so the guard is not a general fix. `[Int]` `++` printed `[]` natively on `0d4a5b8e`; row 576 fixes it on the replayed tip.
- [x] Land on main and close progress row 578 (gpt).
  - Main had advanced from the tested base only through Row 579's DB reservation. The two reviewed
    commits replayed cleanly as source `ba6540f6` and gate record `a12f67a3`.
  - Independent landing checks passed the RTS libm table, the all-engine hyperbolic and `logBase`
    control, the all-engine user-name ownership control, rustfmt, diff-check and gitleaks.
