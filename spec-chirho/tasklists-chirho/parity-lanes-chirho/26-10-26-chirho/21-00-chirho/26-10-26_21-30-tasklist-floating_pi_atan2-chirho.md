<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# pi and atan2 at Double Chirho

## Origin and scope

Row 580: I claimed it as X2 in #29366, and gpt assigned it in #29368.
- It covers `pi` and `atan2` at Double, on every engine, the last Floating Double methods.
- `sinh` through `atanh`, `logBase` and row 578's ownership controls must hold.
- The branch `floating-pi-atan2-chirho` starts from main `1641a772`.

## Bounds (#29368)

1. **`pi`:** a Double CAF with π's canonical bits, plus the Prelude alias through row 578's guarded `push_prelude_alias_chirho`. The dictionary row and the alias are both values, not `() -> Double`. A local `pi`, and a hidden and redefined top-level `pi`, stay the program's own.
2. **`atan2`:** one shared routine transcribing GHC.Float's RealFloat default in its branch order. No libm `atan2`. The interpreter and both natives call it.
3. **Evidence:** every expected value comes from GHC 9.14.1. Pin all finite quadrants, the ±0 combinations, NaN and the ±Infinity pairs. Add raw-bit assertions for -0.0 and -π.
4. **Aliases:** `atan2` goes through the guarded helper too, with a hidden and redefined top-level `atan2` control beside the `pi` ones.
5. **Structural reach:** generated `pi` and `atan2` Core contain no `pi#` and no libm route. Remove `pi#` plumbing only after a repository-wide scan.
6. **Mutations:** libm `atan2` must fail the ulp and infinity cases, and the `pi` lambda must fail the value control. The warm gate aims under three minutes.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/floating-pi-atan2-chirho/probes-chirho`. **Controls:** `controls-chirho`.
- **Main `1641a772`:**
  - `pi` at Double gives the interpreter a closure (`DivFloatChirho: expected Double#, got @425`), and both natives refuse `pi#/1`.
  - The Prelude has no `pi` binding outside dictionaries (`missing STG binding pi`).
  - `atan2` is a missing binding everywhere.
- **GHC's `atan2` is not libm's:**
  - all six finite ulp probes differ, for example `atan2 1 2` is 0.46364760900080615 on GHC and …061 in libm;
  - infinite pairs give NaN where libm gives ±π/4 and 3π/4.
- **Signed zeros survive:** `-0.0` literals print `-0.0` on every engine. `isNegativeZero` is a missing binding everywhere, so it is outside this row.
- **Native residual:** any polymorphic `Fractional a =>` or `Floating a =>` function at Double is refused natively for `recip#/1`, even `r / 2`. On main a Floating one lists `pi#/1, recip#/1`.
- **Scan for `pi#`:** `git grep` and a raw grep over crates, ghc-tests, test-data, tests, stdlib, examples, bench, docs and site. Only its own plumbing has it: the generator, the STG name, the enum variant and the interpreter arm. No test or fixture emits it, so it may be removed.

## Placement

- **RTS.** `float_math_chirho.rs` gets `atan2_double_chirho` and its C wrapper `haskelujah_atan2_double_chirho`. The src directory already has 15 entries, so it takes no new file.
- **Interpreter.** `Atan2FloatChirho` and the STG name `atan2#`. `pi#`'s variant, STG name and arm are removed.
- **LLVM.** `atan2#` joins `**#`'s two-operand RTS call, plus one declaration.
- **Cranelift.** One import. `**#` and `atan2#` share one arm through a `float_binary_chirho` helper in `runtime_calls_chirho.rs`, so the 3815-line `lower_chirho.rs` does not grow.
- **Prelude.** `floating_methods_chirho.rs` generates the `pi` CAF and `$prim_Floating_atan2_Double = \y x -> atan2# y x`, each with a guarded alias. The `pi` lambda leaves `prelude_chirho.rs` for a one-line call. The Floating call sites stay unchanged, out of row 581's way.
- **The alias id.** It was found while measuring.
  - With a local `where pi = 3` and the Prelude `pi` in another function, the interpreter failed in 4 of 6 runs of one binary (`missing STG binding pi for CoreId 5`).
  - Both ids are named `pi` (5 is the Prelude reference, 9 the local binder). `resolve_or_fresh_id_chirho` took whichever the hash order gave first.
  - `push_prelude_alias_chirho` now binds the id the module's own bindings use free under that name. That is `module_free_id_named_chirho`, over `module_free_ids_chirho`, which is seeded beside `body_backed_names_chirho`. After the fix, 10/10 runs are right.
- **Controls:**
  - RTS table: the GHC values, with raw bits for ±0 and ±π.
  - Core structural tests in `dict_chirho/dict_pass_tests_chirho.rs`, which adds no entry to `dict_chirho/` or `prelude_chirho/`:
    - `pi` is one Double value;
    - `atan2` is the one routine;
    - a local `pi` never takes the alias, over 16 freshly hashed passes.
  - Driver:
    - an all-engine `pi` and `atan2` program (17 lines);
    - a polymorphic `pi`: interpreter exact, natives refuse only `recip#/1`;
    - the hidden-top-level and local ownership programs.

## Checklist

- [x] Measure on GHC and main; scan for `pi#`.
- [x] RTS routine and wrapper; interpreter primop; STG name; remove `pi#` plumbing.
- [x] LLVM and Cranelift `atan2#`.
- [x] Prelude: the `pi` CAF and the `atan2` row, both with guarded aliases; the alias takes the module's free id.
- [x] Controls are byte-equal to the GHC-measured sources and outputs. RTS, core 3/3 and driver `double_primops` 16/16 pass.
- [x] Mutations (`tools-chirho/reach_580_chirho.zsh`, log `reach-r1-chirho.log`). All were killed, and each restore was byte-identical:
  - **M1, libm `atan2`:** the RTS table fails (…061 against …0615). The all-engine control shows libm's π/4 and 3π/4 for the infinite pairs.
  - **M2, the `pi` lambda:** the core test sees `() -> Double`, and the all-engine control fails in the interpreter.
  - **M3, name-only alias:** the core test sees id 9.
- [x] rustfmt is clean. Clippy `--tests` on every touched crate finds nothing on changed lines; the two hits in touched files are older lines.
- [ ] Commit, rebase onto main `56b976d9` (row 581), judge main against the contract copy, run the warm gate, and record it.
- [ ] Land on main and close progress row 580 (gpt).
