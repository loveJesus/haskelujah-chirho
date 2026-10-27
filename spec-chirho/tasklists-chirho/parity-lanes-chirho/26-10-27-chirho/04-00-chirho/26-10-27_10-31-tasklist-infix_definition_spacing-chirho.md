<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# A `!` or `-` after a pattern: GHC's whitespace rule Chirho

## Origin and scope

Row 617, mechanism 1 of the row 617 trace (#30213).
- **Released** by gpt_chirho in #30258, after row 640 split the parser.
- **Scope ruling #30275:** the `bindings_chirho.rs` whitespace predicate only. The checkpoint was #30273.
- **Branch:** `infix-definition-spacing-chirho`, from main `dd03ca05`.
- **Lane:** `tmp-chirho/infix-definition-spacing-chirho/`.

## The bug

After a name head or a constructor's arguments, every `!` and `-` started an argument pattern. So:
- `x ! y = ...` defined `x`, and `!` stayed unbound;
- `a - b = ...` defined `a`;
- an instance's `V a - V b = ...` left `-` undefined: `missing method Num.- for V`.

GHC's rule is about spacing. A prefix occurrence (`f !x`, `g -1`) is a pattern, and every other occurrence is the operator.

## The rule (`crates/haskelujah-parser-chirho/src/cst_parser_chirho/bindings_chirho.rs`)

- **`is_prefix_occurrence_chirho`** ports `Lexer.x`'s `precededByClosingToken` and `followedByOpeningToken` over the source text, as `closes_token_chirho` and `opens_token_chirho`. A comment's `-}` closes nothing, and its `{-` opens nothing.
- **`starts_prefix_pattern_operator_chirho`** accepts exactly two forms:
  - a prefix `!`;
  - a prefix `-` before a number, the approved lenient NegativeLiterals reading.
- **Both decisions use it:**
  - `starts_infix_fun_bind_chirho`, for a name head;
  - `can_start_fun_arg_pat_idx_chirho`, for constructor arguments and lambda parameters.

## Measurements (GHC 9.14.1 -O0; main `dd03ca05` and the change on the interpreter, LLVM and Cranelift)

- **Probes:** `probes-chirho/sources-chirho` holds 23, and `probes-chirho/added-chirho` holds 3 more.
- **12 movers to GHC on all three engines.**
  - On main they were compile errors or `missing method Num.-`. `SLetMinusChirho` printed -1 silently.
  - `!` (5): loose, tight, suffix, a constructor head, and `where`-bound.
  - `-` (7): the trace's DHiddenMinusSig and DHiddenMinus, tight, `a -b`, two instance methods, and `let`.
- **8 holds stay GHC:** the trace's four infix shapes; `f !x !y`; `\ !x !y`; `case … of Just !x`; and the prefix `(-) a b` form.
- **The current boundary, unchanged and named by #30275:**
  - **Row 644:** the NegativeLiterals cells (`HNegLitArgChirho` and `HNegLitConChirho`) lose their sign.
  - **Row 645:** Num's `-` and `negate` defaults (`XNumMinusDefaultChirho`, and `SInstanceMinusConChirho`'s `negate`).
  - **Row 646:** a newtype's negative field in derived Show (`XNewtypeShowNegChirho`).
  - **The lenient reading:** `XNoNegLitMinusChirho` stays loud.
- **The corpus** (`corpus-chirho/`, by the committed verdict rule): the CST changes in 10 of the 3,864 tracked `.hs` files.
  - REJECT to ACCEPT: proposal-229d, read029, T13600a and T13600b. Each matches GHC `-fno-code`.
  - should_run `church.hs` gives its expected `74`.
  - The published axes do not move.

## Checklist

- [x] Measure GHC 9.14.1 and main on all three engines before source.
- [x] Checkpoint #30273; ruling #30275.
- [x] The rule in `bindings_chirho.rs`. Parser crate 400/400.
- [x] **Controls.**
  - Parser unit tests: `cst_parser_chirho/tests_chirho/infix_definition_spacing_tests_chirho.rs`, 13 shapes with GHC's `proposal-229e` comment cases, plus the lambda and case bangs.
  - All-engine controls: `typing_integration_chirho/infix_definition_spacing_chirho.rs`, 20 programs byte for byte as measured. Generated from the probe files.
- [x] The workflow doc: `pattern-matching-chirho.md`, "Operator or argument pattern".
- [x] **Reach** (`tools-chirho/reach_617_chirho.zsh`). Every mutation was restored byte-identical (SHA-256), and the restored tree passes the unit tests and the controls.
  - **M1, the old rule** (every `!` and `-` after a pattern starts an argument pattern): the bang and minus movers fail, first at `SLooseBangChirho` and `DHiddenMinusSigChirho`. The holds pass. The unit table reads `x ! y = x` as defining `x`.
  - **M2, constructor arguments and lambda parameters lose the gate:** each list's first failure is its constructor-head mover, `SBangConLeftChirho` and `SInstanceConMinusChirho`. Every name-head mover before them passes; each test stops at its first failure. In the unit table, `Just a !{-comment-}x` becomes no function binding.
  - **M3a, a comment's `-}` closes a token:** `f{-comment-}!x` defines `!`. **M3b, a comment's `{-` opens one:** `Just a !{-comment-}x` becomes no function binding. The all-engine programs hold no comments, so the unit table and GHC's `proposal-229e` guard these.
  - **M4, a prefix `-` before anything starts a pattern:** the minus list first fails at `SPrefixMinusVarSigChirho` (`a -b`), after the loose and tight programs pass. Every bang mover passes. The unit table reads `a -b` as defining `a`.
  - **Receipts:** `reach617-r1-chirho` (M1, M2, M4); `reach617-r2-m3-chirho` (M3 together); `reach617-r3-m3ab-chirho` (each half alone).
  - **r1's M3 run is void:** zsh `$"..."` quoting left its anchors unexpanded, so it ran the unmutated tree. A mutation that does not apply now stops the script.
- [x] **Commit and rebase.** Commit `13cf17e1`; replayed as `df7fb105` on main `0d0eac63`, which row 629 reached meanwhile. They share no parser or control file.
- [x] **Re-measured on the replay** (`probes-chirho/{main-0d0eac63,tip-df7fb105}-chirho`, both probe sets, all three engines):
  - the 12 movers reach GHC, and the 8 holds keep it;
  - four boundary cells are byte-identical on main and the tip;
  - `SInstanceMinusConChirho` and `XNoNegLitMinusChirho` stay loud. Only their errors change: the first now stops at `negate`, and the second loses the unbound `-`.
- [x] **The corpus on the replay** (`corpus-chirho/verdicts-{main-0d0eac63,tip-df7fb105}-chirho.tsv`):
  - main's parser crate is identical to `dd03ca05`'s;
  - the tip's full CST dump equals the measured one, and changes 10 files with no panic;
  - the same four REJECT to ACCEPT moves;
  - `church.hs` prints 74 on the tip, and `missing method Num.- for Ch` on main.
- [x] **The broad scan** (`scan617-chirho.log`, on `df7fb105`): 3601/3601 passed in 915 s, with 18 skipped and a clean tree before and after. It is row 640's 3,589 tests, plus row 629's 6 and this row's 6, with none missing.
- [x] **Judge and gate.**
  - **Main `0d0eac63`** is GREEN against the contract copy first (73 movers, 9 holds; `baseline-0d0eac63-chirho.log`). Its CLI is frozen for the probe sets.
  - **r1, GREEN on `df7fb105` (base `0d0eac63`, 1 commit) in 228.67 s** (`gate617-r1-chirho/`):
    - 1395/1395 focused tests, with this row's 6 and the 32 lazy-pattern tests;
    - no Clippy diagnostic on the 316 changed lines in 5 files;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit), and a build with no warning.
- [ ] Land on main and close progress row 617 (gpt). Then row 641.
