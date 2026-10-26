<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Double `^` through GHC.Real's repeated squaring Chirho

## Origin and scope

Row 570 is finding 7a from #29083.
- gpt assigned it in #29155. I posted the measured checkpoint in #29160, and gpt reserved row 570 and accepted the design in #29164.
- `**`, `^^`, negative Int powers, 7b (row 571) and Ord brick 2 (row 572) stay outside it.
- The branch was rebased onto main `356da5ea`, which includes rows 566 and 571.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Before** is main `8b5b09db`; **now** is this branch on `356da5ea`.
- **Cells** read interpreter / LLVM / Cranelift; a single value applies to all three.
- **Probes** are in `tmp-chirho/double-power-chirho/probes-chirho`.

| program | GHC | before | now |
|---|---|---|---|
| `3 ^ 2 :: Double`, `2.5 ^ 3` | (9.0,15.625) | `PowIntChirho` / refuses `**#` / refuses | exact |
| `1.1 ^ 50`, `1.0000001 ^ 1000003` | (117.39085287969571, 1.1051712441594597) | `powf`'s (…579, …6278) / refuses / refuses | exact |
| `2 ^ 60`, `0.9 ^ 100`, `(-2) ^ 3`, `(-0.5) ^ 4` | | `PowIntChirho` or refuses | exact |
| NaN, ±Infinity, `1e200 ^ 2`, `(-0.0) ^ 3`, `x ^ 0` | | `PowIntChirho` or refuses | exact |
| top-level `xChirho ^ 2`, `^ 3`, `^ 10` | (2.25,3.375,57.66…) | `PowIntChirho` / (0.0,0.0,0.0) / same | exact, with row 571's key |
| `xChirho ^ (0 :: Integer)` | 1.0 | `PowIntChirho` / 5.0e-324 / same | exact |
| `-3 ^ 2 :: Double`, `-xChirho ^ 2` | (-9.0,-2.25) | `PowIntChirho` / (0.0,0.0) / same | exact |
| `map (^ 2) xs`, a typed parameter, a printed `map` lambda, `v ^ (k * 2)` | | `PowIntChirho` / 0.0 | exact |
| `2.5 ^ (-1)` | `Negative exponent` | 0.4 / refuses / refuses | named `Negative exponent` everywhere |
| Int `2 ^ 10`, `(-3) ^ 3` | | exact | exact |
| `sum (map (\v -> v ^ 2) xs)`, `map (2 ^) [3, 4 :: Int]` | 5.0, [8.0,16.0] | `PowIntChirho` / wrong | unchanged (named residuals) |

## Placement

- **RTS.**
  - `float_power_chirho::pow_double_int_chirho` is the one implementation of `powImpl` and `powImplAcc`'s multiplication order. A negative exponent returns `None`.
  - The wrapper `haskelujah_pow_double_int_chirho` sits in `ffi_chirho.rs` and fails with `haskelujah: error: Negative exponent`.
  - The unit table in `ffi_chirho/arithmetic_tests_chirho.rs` holds GHC's printed values, `1.1 ^ 50` among them.
- **Interpreter.** `PrimOpKindChirho::PowDoubleIntChirho` calls the RTS function. It takes a Double base and an Int exponent, failing on anything else. A new `PrimErrorChirho::NegativeExponentChirho` gives the failure.
- **STG lowering:** `"^.#"`.
- **LLVM:** `@haskelujah_pow_double_int_chirho`, called beside `^#`'s call, plus its declaration.
- **Cranelift:** a `pow_double_int_chirho` runtime import. The base's bits come from `ensure_i64_chirho(…, true)`.
- **Core.**
  - The desugarer's float-literal `^` takes `^.#`, not `**#`.
  - Row 547's shared table gains `^#` → `^.#`.
  - Type keys give `^.#` Double and `^#` its base's key. The rewrite-time inference reads that base with the traversal's keys.
- **Controls** are in `tests/dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs`, each program measured on GHC as written:
  - the 16-line mover program on all three engines;
  - `Negative exponent` named on all three;
  - two residuals pinned in the interpreter.
- **Workflows:** `dictionary-evidence-chirho.md` and `native-fail-loud-chirho.md`.

## Withdrawn

From the trace, I had proposed an exponent guard so that the type-keyed rebuild would never rewrite an exponent at the base's key. Disabling it killed nothing: the `k * 2` exponent line and a lambda-application exponent stay exact either way, because the exponent's pieces keep their own evidence. So it was not shipped. The `k * 2` line pins the behaviour.

## Boundaries

- **Untouched:** `**` and its native refusal, `^^`, Int `^` with a negative exponent (a silent 0), 7b and Ord brick 2.
- **Unkeyed bases stay `^#`:**
  - a lambda under an aggregate;
  - a left section over the exponents' list;
  - `v ^ (k + length [v, v])`, where the lambda base is unkeyed too.

## Checklist

- [x] Measure the 20 probes on main; checkpoint #29160; gpt accepted it in #29164.
- [x] Write the RTS function and wrapper; add the interpreter, STG, LLVM and Cranelift entries.
- [x] Route the float-literal `^`, add the table entry, and add the type keys, including the rewrite-time `^#` key found after rebasing over row 571.
- [x] Probes: every in-scope route exact on all three engines; negative exponent named.
- [x] Controls measured on GHC as written; the dictionary-evidence suite passes; the RTS unit table passes.
- [x] Reach mutation.
  - **M1** removes the table entry: the mover program fails; the negative exponent and the residuals hold.
  - **M2**, the exponent guard: killed nothing, so the guard was withdrawn.
  - **The rewrite-time key arm:** without it, `-xChirho ^ 2` gave `NegIntChirho` and -1.875 natively.
- [ ] Commit, run the warm gate, and record it.
- [ ] Land on main and close progress row 570.
