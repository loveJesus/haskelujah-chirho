<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Lazy Core text functions in the generated Prelude Chirho

Row 579 (gpt_chirho #29361) takes option (b) of #29356. The generated `words`, `lines`, `unwords`,
`unlines` and String `intercalate` were strict text primops that read only a literal. A String
built at run time read as "" or lost its run-time elements, and an infinite String did too. They
become lazy Core, and the primop handlers stay.

## Decisions at brick 1, measured on main 1641a772

- Home: `dict_chirho/prelude_chirho/report_chirho/`, per #29378 (the entry cap).
  - `mod.rs` holds the Core builders the report modules share.
  - `report_folds_chirho.rs` is moved there.
  - `report_text_chirho.rs` is new.
  - `prelude_chirho.rs` keeps only `mod report_chirho;`, one generator call, and the deletion
    of the five blocks.
- Split with `takeWhile` and `dropWhile`, the report's meaning of `break`. The generated `span`
  forces its recursion: `take 3 (fst (span (const True) (repeat 'a')))` exceeds the interpreter's
  step limit. `takeWhile` and `dropWhile` are productive on all three engines.
- `words` tests GHC 9.14.1's `isSpace` code points directly, measured over every Char: 9-13, 32,
  160, 5760, 8192-8202, 8239, 8287 and 12288.
  - The interpreter's `isSpace#` is Rust's White_Space, which also counts U+0085, U+2028 and
    U+2029.
  - Neither native backend has `isSpace#` (`LLVM does not support primitive(s): isSpace#/1`).
- The joins use the generated lazy `append`, never the String `++#`. Natively, `++#` packs both
  operands: `take 5 ("ab" ++ repeat 'c')` gets no answer natively.
- Only the five generated blocks named the five primops. The STG table maps names only.

## Checklist

- [x] `report_chirho/` with the five lazy definitions and GHC's space table. Core builds with 0
  warnings.
- [x] Structural control: each of the five is generated once, and no text primop is in its body.
  The space ranges expand to GHC's measured list.
- [x] Checkpoint #29413 measured 10 RED cells from 3 roots. The ruling (#29415) put all three
  in this row:
  - Native `putStr` reads its String through the normalizer `putStrLn` uses (6 cells: h04,
    u06 and r11 on both natives).
  - The per-character space test drops its negation. The remaining long-literal moves to the
    named 100000-step limit are accepted as the shared Core budget (3 cells).
  - `isSpace` is a guarded alias of the same generated test `words` uses (w10).
- [x] Transition table r2: a fresh build of main 5993e2d3 against the candidate, all three
  engines.
  - 54 one-line probes: 114 GAIN-EXACT and 40 hold-exact. 4 WRONG cells moved to named
    failures: k01 and k02 to the step limit in the interpreter, and p04's NUL to its named line
    natively. 4 named native failures moved to "Prelude.undefined", from the native
    cons-tail strictness that predates this row.
  - 17 programs: 37 GAIN-EXACT and 7 hold-exact. 3 accepted budget cells: words of 1,000, and
    lines and words of 2,000. 4 cells moved from a base stack-overflow abort or a native
    refusal to a named failure.
  - The row 576 battery: 15 GAIN-EXACT and every other cell held, including all 123 row-535
    contract cells.
  - No candidate timeout, signal or panic anywhere.
- [x] Descriptive thresholds, bisected: words of a literal exact through 846 words (2,538 chars);
  lines through 1,958 lines (5,874 chars). The base is exact past both.
- [x] All-engine controls generated from runghc output, byte-identical to the measured
  programs: `report_text_functions_are_lazy_and_exact_everywhere_chirho`,
  `put_str_reads_every_string_representation_everywhere_chirho`, and putStr in the NUL test.
- [x] Transition table r3 on main 530858f8, after the rulings: identical to r2.
- [x] Gate GREEN on 268516d7 (main 0ac0e6df) in 217 s, patch id c659ba58472d1371:
  - build with 0 warnings; core 180, RTS 73, runtime 58;
  - 30 full repeats of the RTS binary, 0 failed;
  - LLVM 38, Cranelift 42;
  - seven driver suites 200, and the driver library filter 200;
  - Clippy: none of 563 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
- [x] Replayed onto e820a7e0 with the same patch id. Build has 0 warnings; the structural tests 2,
  native_error_helpers and evaluation_values 66; diff-check and gitleaks clean.
