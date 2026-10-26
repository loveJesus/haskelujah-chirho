<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Derived Show precedence Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (row 533).

**The bug.** Derived Show builds `show` by concatenation (`"C" ++ " " ++ show a1 ++ ...`).
- Nested constructors, negative fields and records are wrong on main.
- Contexts at precedence 11 fall to a runtime renderer that parenthesizes any text containing a space and renders operator constructors prefix.
- Trace and disposition: gpt_chirho #28285, #28294, #28295. The receipts are in tmp-chirho/derived-show-chirho/: 55 probes, every one confirmed by GHC 9.14.1.

**The fix.** Derived Show becomes a precedence-aware Core renderer on the existing Show shape machinery, following GHC's derived rules:
- prefix: `showParen (d > 10)`, fields at 11;
- record: `showParen (d > 10)`, fields at 0, labels in declaration order;
- infix: a binary symbolic constructor uses `showParen (d > p)` with both operands at p+1, where p is the module's declared precedence or 9.

The Show class keeps exposing only `show`. `$prim_Show_showsPrec_<key>` is an internal, body-backed helper, and `$prim_Show_show_<key>` is its wrapper at precedence 0.

**Held gain it unblocks.** The patterns plus fundep replay (row 531, de92912a). It carries +10 should_fail rejections and the GHC-agreeing tc231 correction, which cannot gate while G2, K06f and N1 are newly WRONG.

The branch `derived-show-precedence-chirho` starts from exact main 6ea3de750d6e6a657f6635fb14e0ed07a0040c70. Its two report commits above 9944ed4c touch only the report, and the crates tree is ad3f3661.

## Source

- [x] C1 (542d98b2): this plan.
- [x] C2 (e9873607): the source and its tests, 12 paths.
  - **Typing:**
    - deriving_chirho/show_chirho.rs (326 lines) takes derive_show_chirho, unchanged, out of mod.rs, which goes from 3,503 to 3,419 lines.
    - `derived_show_metadata_chirho` returns `DerivedShowChirho` for each `data` type whose deriving clause names Show: visible parameters, plus each constructor's name, form and field types, all in declaration order.
    - The form is prefix, a record with its labels, or infix. Infix applies only to a binary symbolic constructor, at the module's declared precedence or 9.
    - A GADT constructor, or a field type outside the key grammar, leaves the type out.
  - **Driver:** lib.rs builds the metadata beside newtype_info and passes it to `dict_pass_module_full_with_evidence_chirho`, the single dictionary-pass call, which every path reaches.
  - **Core:**
    - The `DerivedChirho` shape (shape_chirho.rs) is admitted at any precedence by `derived_show_key_chirho` when every field of every constructor parses at its precedence.
    - Recursion counts as rendering, and final answers are kept (C5 replaces this memo; see below):
      - a `false` is always final, since an assumption only ever answers `true`;
      - a `true` that rested on a key still being checked further out is provisional and is not kept, so a key inside a recursive cycle can be checked again after the outer key is final;
      - acyclic fan-out is checked once per key. (C2's commit message says each key is checked once; C4 corrects that.)
    - derived_chirho.rs (473 lines) generates `$prim_Show_showsPrec_<key> d x`, one recursive helper per concrete key.
    - `rewrite_derived_show_rows_chirho` points each renderable monomorphic type's public row `$prim_Show_show_<T>` at the helper at 0. That covers evidenced print, `show` occurrences and the instance dictionary.
    - The Show class layout, selectors and runtime are unchanged.
  - **Tests:**
    - tests/dictionary_evidence_chirho/derived_show_chirho.rs: 3 tests covering the 8 movers, C04 and the 10 derived holds.
    - show_chirho/derived_tests_chirho.rs: 7 Core tests in C2, 9 after C4:
      - infix thresholds and operands at p + 1;
      - prefix fields at 11 and record fields at 0;
      - fail-closed;
      - parameter substitution;
      - the NonEmpty seam at infixr 5, with no negative-number parentheses at 6;
      - the public row;
      - a 40-level acyclic fan-out graph, checked once per key;
      - (C4) a mutual recursion with a missing renderer (AChirho needs BChirho, which needs AChirho and an unshowable FooChirho): both stay unrenderable, and the recursion state clears;
      - (C4) a renderable mutual recursion, admitted from either side, with the state cleared. The self-recursive positive is the infix test's EChirho.
    - deriving_chirho/show_tests_chirho.rs: 4 metadata tests.
- [x] Checks without Cargo:
  - every touched Rust file is a pinned-rustfmt no-op, and `diff --check` and gitleaks are clean;
  - the new files are all under 500 lines, and the directories hold 4, 4 and 10 entries.
  - GHC 9.14.1 confirms all 19 extracted programs. Main's frozen 77586815 is WRONG on 9 (the 8 movers and C04) and EXACT on the 10 holds, matching the test partition.
  - Receipts: tmp-chirho/derived-show-chirho/oracle-unit-main-r2-chirho/. The earlier oracle-unit-main-chirho/ is an invalid partial run: extraction failed before any program ran.
- [x] Docs (C3): a Derived Show section in print-show-evidence-chirho.md, and these ticks.

- [x] C4, source only, after gpt_chirho's source review (#28303):
  - The comments and this tasklist state the memo's exact behavior.
  - Two mutual-recursion tests are added, making 16 new tests in all.
  - C04 is the ninth claimed mover. A monomorphic derived type reached through a dictionary answers as GHC does, because the instance dictionary's `show` is the public row #28295 asked to keep.
  - The trace (tmp-chirho/derived-show-chirho/TRACE-chirho.md) is kept as measured. It listed C04 outside the claim; the public-row placement refined that diagnosis.
  - The boundary is now a derived type with parameters reached through a dictionary.
- [x] Replay onto main 9a820c64 after row 532 landed (#28320). C1-C4 became 89a7f150, 214d14f6, 27d4cf39 and 66d76d6f. The union proof is GREEN 14/14 (tmp-chirho/derived-show-chirho/replay-9a820c64-chirho/).
- [x] C5 (286a57b3), source only. A read-only desk-check of the never-compiled unit found two defects in the support check, and gpt_chirho gave GO in #28328:
  - **D1, a compiler crash on valid Haskell.** A non-regular datatype such as `data N a = Nest (N [a]) | Flat a` met a new key at every level. The parser's depth guard restarted on every call, so the check recursed until the stack overflowed. This happened even for a monomorphic type that only holds such a field. Main compiles these programs.
  - **D2, compile time.** A provisional `true` was never reused within its outermost check, so a cycle with fan-out was checked 2^n times.
  - **The fix.**
    - The check fails closed past the shared depth bound (64) and for a key over 512 bytes.
    - A `false` that depends on the depth bound is not kept.
    - A `true` carries the deepest nesting at which it held.
    - Provisional trues are reused within the outermost check and promoted only when it renders.
    - The parenthesized branch reuses parenthesize_chirho.
  - **Three Core tests:**
    - a nested datatype in both shapes;
    - the depth boundary in either call order, the depth-limit mutant's control;
    - a 40-type fan-out cycle, the promotion mutant's control.
  - That makes 19 new tests in all: core 12, typing 4, driver 3.
- [x] C6: these ticks, and the workflow's bounded semantics.
- [x] C7: the Gate ticks and the gate runs record, after the v6 gate (#28365).

## Gate

The shape is gpt_chirho #28303, amended by:
- #28328: the in-gate base build, bounded failure, typing_integration, row 532's guards and the M9 and M10 mutants;
- #28337: a strict cross-form Clippy bijection;
- #28342: the reviewed corpus movers;
- #28345: the live execution sources pinned.

It runs only under a builder handed by name, on the final tip over main 9a820c64, with the base built in the gate at exact 9a820c64. Each item below is ticked from the GREEN v6 run, `ds533-derived-show-467c81fc-v6-chirho` (see Gate runs).
- [x] The 9 movers become GHC-exact: S01, S03, S04, S06, S11, S18, C05, D03, and the monomorphic C04.
  - v6: the oracle moved exactly these 9 from WRONG to GHC-exact, and the candidate answers all 19 programs as GHC does. The trace moved the same 9.
- [x] A parameterized derived type reached through a dictionary is measured as a calibration probe. If it moves, the gate stops RED for review.
  - v6: K01 held, LOUD on both sides.
- [x] The 14 measured exact rows and every other non-boundary trace row hold.
  - v6: of the 56 trace probes, every row except the 9 movers kept its base verdict and detail.
  - The base measured EXACT 14, LOUD 28 and WRONG 14; the candidate measured EXACT 23, LOUD 28 and WRONG 5.
- [x] Bounded failure, end to end. Each of these terminates within the gate's timeout and stays on the base route:
  - `N` at a concrete type (X01);
  - the growing `Seq` family (X02);
  - an unprinted monomorphic `M` that holds `N Int` (X03).

  A stack overflow, signal, timeout, panic or missing result is RED.
  - v6: both CLIs ran all three to completion, with no timeout, signal, panic or non-start.
  - The candidate's exit code, stdout and stderr equal the base's byte for byte, and GHC matches every expectation.
- [x] The native backends still print derived values: the LLVM and Cranelift derived-enum and field-constructor round trips now run the helper.
  - v6: exactly the six named round trips ran and passed on both builds, with LLVM rc 0 and Cranelift rc 0.
  - They are the LLVM print of a derived enum and of a field constructor, and the Cranelift print and show of each.
- [x] typing_integration is identical to the base. Row 532's GREEN receipts and counts are pinned guards.
  - v6: typing_integration's 40 (test, outcome) pairs equal the base's exactly.
  - Row 532's run (e1ca05f2) and launcher (2409adf3) are pinned sets, and the 6 suites row 532 also ran pass on the base at exactly its recorded counts.
- [x] Mutants are caught on prefix, record and infix precedence, on fixity, on the depth bound (M9) and on provisional promotion (M10).
  - v6: each of the 10 mutants terminated, built and was caught by its named control. M4 (public row skipped) was killed by C04 among others.
  - The restore reproduced the baseline, and the 3 mutated files hash to their tip blobs.
- [x] The corpus moves exactly as reviewed in #28342. This replaces "the corpus does not move".
  - Membership and replay are byte-identical across sides: accept 938 lines, reject 767 lines, replay 295 lines.
  - Execution goes from 28/35 to 30/35.
  - The fixed set is exactly run and native GeneratedLiteralsChirho, whose source is pinned at 8c10ad0c.
  - The candidate's compile, run and native are rc 0, with stdout (92 bytes) and runtime stderr (empty) GHC-exact.
  - The base prints 82 bytes on run and native, and its verdict is FAIL.
  - Nothing is lost, and every other predicate is identical.
  - v6: exactly so, with base 28/35, candidate 30/35, fixed [('native', 'GeneratedLiteralsChirho'), ('run', 'GeneratedLiteralsChirho')], lost none and changed none.

## Gate runs

The receipts are in the Claude worktree's tmp-chirho/derived-show-chirho/gate-chirho/runs-chirho/, with one run directory and one launcher directory per tag. Every directory digest is C-locale, `find <root> -type f -exec shasum -a 256 {} + | LC_ALL=C sort | shasum -a 256`, run from runs-chirho/.

### RED: `ds533-derived-show-467c81fc-chirho` (tooling v3), tip 467c81fc

- The builder was handed by name in #28333. The build started at 16:54:31 EDT. Build, base, focused and native exited 0, and lint exited 1 at 17:11:41.
- Lint went RED on Clippy's rendering form, not on a diagnostic.
  - The base, built cold in the gate's new worktree, printed every diagnostic in short form.
  - The warm tip replayed 4 core, 87 driver and 78 CLI diagnostics from Cargo's cache in long form.
  - So the per-form multisets differed:
    - core: short 130 tip vs 134 base, long 4 vs 0;
    - driver: 375 vs 462 and 87 vs 0;
    - the CLI: 353 vs 431 and 78 vs 0.
  - Typing matched, at 56 and 0.
  - Each long-only diagnostic paired one to one with a short-only one at the same location. None was in the unit's crates.
- Frozen candidate CLI 65c421c5851397890a04f491831e9ffe3e974dfec79f8d700f7961717b13824a, frozen base f5ce4b40d42ec6657b7fc7879e8eb1978eeb58cdfa6fa580597e364e03bec8ff, gate script 1fd0156cb54431d213938faa9a48d3feaf6f73a9b659458b4e8d322ea588ea6b.
- The receipts are kept byte-for-byte and are pinned as immutable sets from v4 on:
  - run 50e7807ff3f9f67c0c70b345b527443442ab3102ac78faabe22e0961576d634f (355 files);
  - launcher bf247371b713689b76f3e382fee760eb471eacf4157ec3133b9ab506687756a6 (6 files);
  - ledger a926bb286862051012ef6dc37c2c34c95a8ac162587a716de21f4735b1184915;
  - exit codes 589f2155f2a53f9a4b59bdea52142c3d2eb4c98df5e07a657854cf753f6c0424.
- Remedy (#28337): v4's strict cross-form bijection judge.
  - A short and a long diagnostic pair only at the same coordinate and severity, off the 14 unit paths, on a byte-identical source line, with the headline equal or followed by exactly ": ".
  - Anything unparsed or unmatched, or a changed multiplicity, is RED.

### RED: `ds533-derived-show-467c81fc-v4-chirho` (tooling v4), tip 467c81fc

- The builder was handed by name in #28339. The build started at 17:37:51 EDT. Build through mutations exited 0, and corpus, the last stage, exited 1 at 18:33:34. The postflight inside corpus was GREEN.
- Corpus went RED on a movement that was GHC-correct but not yet reviewed.
  - The reused dict-split judge requires "fixed none, lost none", and the unit fixed GeneratedLiteralsChirho on run and native.
  - Its nested tree with `LeafChirho (-2)` now prints GHC's 92 bytes, where the base printed a flat 82.
  - Execution went from 28/35 to 30/35, and nothing was lost.
- Frozen candidate CLI 65c421c5851397890a04f491831e9ffe3e974dfec79f8d700f7961717b13824a: run 1's binary, since this build compiled nothing. Frozen base f5ce4b40d42ec6657b7fc7879e8eb1978eeb58cdfa6fa580597e364e03bec8ff, gate script 1d4d15c43cb37e6dc0c766913a19463669451d89743cd57a0c86738478880e9b.
- The receipts are kept byte-for-byte and are pinned as immutable sets in v5 and v6:
  - run bc0c770e0f314042b6ad4439369dd4d15645776d683185fd07da21d73c96ca29 (25323 files);
  - launcher 35a159233d6a3369a2f2ce02013198fc1e381dc7f372ffff6d687472859cb59b (11 files);
  - ledger 7962f1202de435ed983453f62ac618ec7051d2bdcabb0c881788fbc0f18db3fd;
  - exit codes 1285b1c5e2fb7514b1c679a3d1731325df5998d37e512f6c89e876ad9c5f7e71;
  - corpus judgment 559a7795326dc1fbfd13bf3c7e3545e5a174ffed59d4e568ec84055336dccfc5.
- Remedy:
  - gpt_chirho accepted the gain as reviewed, not waived (#28342). v5's `corpus-movers` judge pins exactly the two movers and checks the raw files on both sides.
  - #28345 held v5 because the live execution sources were not pinned, and v6 pins them. The v5 tag was never launched.

### GREEN: `ds533-derived-show-467c81fc-v6-chirho` (tooling v6), tip 467c81fc

- The builder was handed by name in #28346, and the build started at 19:24:40 EDT.
- All 10 stages exited 0: build 19:25:14, base 19:31:53, focused 19:38:53, native 19:38:59, lint 19:39:25, oracle 19:43:17, trace 19:45:11, guards 19:46:46, mutations 19:50:44 and corpus 20:21:19 EDT.
- The postflight inside corpus was GREEN, and all 25 ledger rows are GREEN.
- Hashes:
  - ledger a4c88845c0a700cf9757d2156342b316329f6070d42b148d78e9b0d93f49c184, 25 rows, all GREEN;
  - exit codes 249954e49b46ce87864954e0d9ad384098136612dffb9e08d1722c7ccbe75a20, 10 rows, all 0;
  - C-locale run d587c9c554427b0af40c98536826937ded1372974850f58cf3a9be733ec183cb (26181 files);
  - C-locale launcher 1464c5ffd876991c7c1ed70de52cc030e7330c327c3862b2e22b0c3f5d0f9dc9 (11 files);
  - frozen candidate CLI b19f30b844e6d9249975b15e02e9054e6ab7fd089917233ca23a8a37a31dec98;
  - frozen base CLI f5ce4b40d42ec6657b7fc7879e8eb1978eeb58cdfa6fa580597e364e03bec8ff;
  - gate script 4bcb8e61d04d23e96e707886d725c4049f04f07b5288608b693ffe67b44e50cf.
- Preflight covered the 6 commits and 14 unit paths from 9a820c64; under crates/, only typing, core and driver changed. GHC re-derived 19/19, and the judge selftest passed 99.
- Pins: 13 tools, 15 C-locale immutable sets and 2 held refs.
  - The sets include execution-sources, the live literal-checkpoint-chirho/sources, at 63751e609945567208b78d742a9f801c67de4937d50a4c5658b8510028b1b233.
  - They also include the v3 and v4 RED runs and launchers.
  - The held refs are row 531's replay, patterns-fundep-rc5-replay-chirho at de92912a, and row 532's import-scope-join-amp-main-chirho at 000d5c45.
- Build and base: rc 0 with no warnings on both.
  - The base ran 8 suites, native and 4 Clippy logs.
  - The 6 suites row 532 also ran pass at exactly its recorded counts.
- Focused, each as its contract requires:

  | suite | passed | contract |
  |---|---|---|
  | core-lib | 160 | the base's 148 and the 12 new tests |
  | typing-lib | 358 | the base's 354 and the 4 new tests |
  | dictionary_evidence | 64 | the base's 61 and the 3 new tests |
  | show_precedence | 2 | the base's set |
  | instance_obligations | 28 | the base's set |
  | io_actions | 9 | the base's set |
  | driver-filtered | 609 (1244 filtered) | the base's set |
  | typing_integration | 40 | the base's set |

- Native: the six named round trips ran and passed on both builds.
- Lint, through the strict bijection:
  - core 130 same-form and 4 cross-form pairs; driver 375 and 87; the CLI 353 and 78; typing 56 and 0;
  - nothing unmatched or unparsed, no error, and nothing on an inserted line or in a new file;
  - gitleaks scanned 6 commits and found no leaks.
- Oracle:
  - the 9 movers;
  - 10 holds, EXACT on both sides: S02, S05, S07, S08, S10, S12, S14, S15, S16 and S17;
  - so 19/19 as GHC;
  - the five guard oracles are identical at their pinned counts: list_orders 15, string_lists 9, higher_order_occurrences 34, generic_heads 19 and ordering_literals 28.
- Trace: 56 probes, with exactly the 9 movers. The calibration K01 held, and the three bounded-failure probes completed byte-identical to the base.
- Guards: all 37 matrix rows identical with 24 requirements MET, and the 12 RC5 probes identical.
- Mutations, caught 10 of 10, with the restore at the baseline:
  - M1 prefix fields at 10;
  - M2 record fields at 11;
  - M3 prefix threshold 11;
  - M4 public row skipped;
  - M5 derived only at 0;
  - M6 infix operands at p;
  - M7 infix threshold 10;
  - M8 fixity ignored;
  - M9 depth limit removed;
  - M10 promotion skipped.
- Corpus: both CLIs completed with byte-identical passes. The movement is exactly the reviewed contract: base 28/35, candidate 30/35, run and native GeneratedLiteralsChirho fixed, nothing lost.
- Postflight:
  - both frozen CLIs unchanged;
  - the tree clean at 467c81fc, with the mutated files at their tip blobs;
  - the gate script, 13 tools, 15 sets and 2 held refs at their pins;
  - no cargo, rustc, Clippy or CLI process alive.

  An independent read-only check after the run agreed.
- Not claimed: reproducible dev-profile bytes.
  - Runs 1 and 2 froze candidate CLI 65c421c5; run 2's build compiled nothing.
  - Run 2's mutation restore then rebuilt core, the three backends, the driver and the test harness from the restored tip sources. Run 3's build recompiled 7 crates and froze b19f30b8, at the same tip and toolchain.
  - Behavior was gated on b19f30b8.
- Nothing was written to main or to the progress DB. Landing and DB row 533 are gpt_chirho's.

## Outside this unit

- The runtime Show precedence methods, meaning showsPrec, shows and showParen as functions plus the class defaults (B rows and S13).
- C01, a hand-written show-only instance at nested precedence.
- A derived type with parameters reached through a dictionary. The monomorphic C04 is inside the unit.
- S09, newtype erasure.
- Standalone deriving.
- Hand-written instances.
- Declaration forms: GADT, prefix symbolic (I07) and backtick (I08, G1).
- NonEmpty metadata, which unit 517 supplies.
- T10 and T11.
