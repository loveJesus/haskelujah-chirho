<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native nullary globals stay lazy Chirho

Row 588 (gpt_chirho #29491). Natively, a reference to a top-level nullary binding or builtin in
a lazy position was evaluated where it stood. Measured on main against runghc 9.14.1:
- `length [undefined, undefined]` failed "Prelude.undefined";
- a user error CAF passed as an argument failed "boom";
- `case "ab" : undefined of (w : _) -> w` failed;
- a self-recursive CAF in an unused list field crashed SIGBUS.
The interpreter was exact on all of them.

## Trace (#29494) and ruling (#29497, #29524)

- The shared enabler is `prepare_native_thunks_chirho`. It closes lazy arguments, fields and lets,
  but `needs_thunk_chirho` treats every Var as already lazy. It is unchanged, by ruling.
- The rule lives in each backend's lazy boundary, `compile_lazy_value_chirho` and
  `lower_lazy_value_chirho`. Both fell back to the eager expression lowering, which calls a
  nullary global for its value.
- The repair is that each boundary suspends a nullary global as a thunk over its function's
  address, entered by the arity-0 trampoline both backends already emit. Strict positions keep
  the direct call.
- Checkpoint 2 (#29523): Cranelift's `lower_constructor_app_chirho` carried an inline copy of
  the boundary. Its fields now call `lower_lazy_value_chirho`, as its arguments already did.

## Measurement correction (#29523, #29524)

- My runners asked for Cranelift with `--backend cranelift`, which the CLI ignores. Every
  "cranelift" battery cell from rows 576, 579 and this row's first run was LLVM.
- The runners now pass `--cranelift`, and every compiled cell must carry `compiled (cranelift)`.
  The audit counted 1,256 marked cells, 0 unmarked and 116 compile refusals.
- The row 576 and row 579 tasklists carry their real-Cranelift reruns.

## Checklist

- [x] Trace and checkpoint (#29494); ruling (1), the symmetric backend boundary (#29497).
- [x] LLVM hook in `compile_lazy_value_chirho`, with the thunk allocation factored as
  `emit_thunk_chirho`.
- [x] Cranelift hook in `lower_lazy_value_chirho`, with `alloc_thunk_from_fvs_chirho` factored
  and the `undefined_ref_chirho` accessor. Its constructor fields route through the boundary
  (#29524).
- [x] The ten probes are GHC-exact on the interpreter, LLVM and real Cranelift.
- [x] Control `nullary_globals_stay_lazy_in_lazy_positions_everywhere_chirho`, generated from
  runghc's output.
- [x] Reach mutations, each hook off in turn:
  - LLVM: the control fails on LLVM, "Prelude.undefined";
  - Cranelift: it fails on Cranelift, the same way.
  Both files were restored byte-identical.
- [x] Real-Cranelift battery, main 0d01fbe2 against f64247a6:
  - the ten probes: 12 GAIN-EXACT and 18 hold-exact;
  - the row 576 battery and the row 535 contract hold;
  - the row 579 sets: 6 GAIN-EXACT, its undefined-tail probes on both natives;
  - one named failure moved, k02 on Cranelift, as below;
  - no signal or timeout.
- [x] stdin IO probes (`getLine` let-bound, in lists, run twice): base and candidate identical on
  every engine.
- [x] The row 576 and row 579 real-Cranelift reruns, with their tasklists corrected.
- [x] Gate r1 was RED, on two items fixed in this commit:
  - the LLVM unit test `compile_undefined_argument_resolves_to_runtime_abort_chirho` asserted the
    eager call this row removes. It is now
    `compile_undefined_argument_suspends_its_runtime_abort_chirho`: `undefined` still resolves to
    the runtime abort, and is suspended over its address, never called where it stands;
  - a collapsible_if, now a let-chain.
- [x] Gate r2 GREEN on a712387c (main 8318aad6) in 233 s:
  - build with 0 warnings; LLVM 38, Cranelift 42;
  - eight driver suites 215, and the driver library filter 28;
  - Clippy: none of 535 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
  The crates interdiff from the measured f64247a6 is only those two fixes (git range-diff).

## Cost and residuals

- Each lazy reference to a nullary global allocates one thunk. A CAF is not shared or cached
  natively: every reference recomputes it. That stays outside this row.
- Cranelift alone: `length (words (concat (replicate N "ab ")))` is exact through N = 1,000.
  At N = 2,000 it fails named, on main too, with a message that varies by run (`<<loop>>`,
  "no matching alternative", "not valid UTF-8"). That is consistent with a rooting defect on long
  lists. LLVM is exact.
