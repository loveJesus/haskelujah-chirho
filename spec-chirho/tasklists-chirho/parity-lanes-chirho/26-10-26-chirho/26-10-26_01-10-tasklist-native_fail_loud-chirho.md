<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native fail-loud (row 535) Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB.

The scope is P1 from the Show-methods trace: gpt_chirho #28373, #28380, #28382 and #28411. Row 535 lands before the row 531 and unit 517 baseline is frozen, so every later gate starts from deterministic failure behavior.

**The defect.** Where the interpreter names a failure, native executables currently either fail silently or die by a signal. Measured on the frozen main CLI b19f30b8 and GHC 9.14.1:
- **Unbound names.** A name that type-checks but has no body (`showsPrec`, `scanl1`, `foldl1`, `until`, `realToFrac`) is aliased to an unused enclosing parameter. LLVM and Cranelift then call through it and die by SIGSEGV with no message. A parameterless CAF fails differently: LLVM refuses with a raw clang error, and Cranelift prints nothing and exits 0.
- **Integer formatting.** The show and print fallback, `showInt#`, formats a tagged heap pointer as a decimal, a different odd number on every run.
- **Missing methods.** The missing-method stub uses the plain `error` primop, which Cranelift does not lower: it prints an empty line or `0` and exits 0. LLVM names it, then aborts with SIGABRT.
- **Unknown primops.** Cranelift lowers them to 0, which prints `LT` for every `compare`, `0` for IORef, and `NaN` or `[]` elsewhere. It also lowers `^#` to its left operand. LLVM refuses them at compile time.
- **Case defaults.** A case with no matching alternative traps: SIGTRAP on LLVM, SIGILL on Cranelift.
- **`abort()`.** Every named native failure ends in `abort()`, which is SIGABRT. LLVM's user `error` writes its message to stdout, and its `undefined` writes nothing.

**Contract (#28382).** At every reached R1-R5 site owned by this unit, a native executable does one of two things:
- it is refused at compile time, naming `name/arity`, and the compile command exits 1 (R4);
- or it flushes stdout, writes exactly one stderr line, and exits 1, with no abort and no signal. The line is `haskelujah: error: <message>` for a program failure, or `haskelujah: internal error: <message>` for a violated runtime or compiler invariant.

It never aliases an enclosing parameter, never substitutes 0, and never formats a heap pointer as an integer.

**Routes:**
- R1: an unbound reference fails, named, when entered. The parameter aliasing is removed from both backends.
- R2: the runtime's integer formatters refuse a live boxed value.
- R3: one error route for the `error` and `error#` primops, user `error`, `errorWithoutStackTrace`, `undefined`, pattern-match failure, missing-method stubs and Cranelift `throw#`.
- R4: unknown primops are refused at compile time on both backends. Cranelift refuses `^#` explicitly.
- R5: a compiler-emitted default with no matching alternative fails, named.

**Owned by this unit.** The two native backends, the runtime support library, the CLI's reporting of refusals, and a new driver test file.

**Outside this unit:**
- native Data.Map, IORef, `fromIntegral`, `compare*` and `^#` implementations (they stay refused, now on both backends);
- Show dictionaries and exact rendering (the F lane, after 517);
- unit 517;
- K01's root cause, and its Cranelift SIGBUS from upstream type confusion;
- S09;
- DerivedEqOrd's wrong values and RecursiveDo's dominance error;
- the Cranelift string/list mismatch;
- letrec zero initialization;
- arbitrary memory faults;
- any interpreter change.

## Source

- [x] C1 (7a701d64): this plan.
- [x] Re-baseline the 36-row inventory before any source edit: the 16 Show rows and 20 non-Show probes, plus the four wildcard holds.
  - It ran on a hash-verified copy of row 534's frozen CLI 15e1b77b, which is main's crates tree 0cfd246e, against GHC 9.14.1, in interpreter, LLVM and Cranelift mode.
  - Correction (#28416, #28417): this re-baseline did invoke Cargo, though it rebuilt nothing. C2 and #28414 had said it ran with no Cargo.
    - Every native `compile` runs `cargo build -p haskelujah-rts --quiet` in the tree the CLI was built in (cli `main.rs:871-885`). 15e1b77b was built in agy's worktree, and b19f30b8 in mine.
    - Each native compile, and the later IR probes on 15e1b77b, therefore ran an RTS freshness check in those trees.
    - Both checks were no-ops. agy's `libhaskelujah_rts.a` still dates from Oct 25 22:46:13 (18,950,288 bytes) and mine from Sep 18 11:21:44 (18,975,952 bytes), so neither RTS was rebuilt.
    - After the C3 runtime edits, b19f30b8 is retired for native compiles, because it would rebuild the edited RTS. The gate's oracle runs this check under the builder lease.
  - Row 534's measured delta against b19f30b8 is exactly two rows: P04 (`M.toList`) and P05 (`M.size`) go from WRONG to EXACT in the interpreter.
  - Every native result of the 40 programs is unchanged, and the other 38 programs are identical in every mode.
  - On main, the native classes are:
    - every unbound name, including the U05 CAF on Cranelift, dies by SIGSEGV. U05 on LLVM fails to compile, with clang's undefined-value error;
    - the five pointer-format rows are WRONG on both backends;
    - the missing-method rows (B06, B09, M01, M02) are SIGABRT on LLVM and WRONG on Cranelift;
    - E01 and E04 are SIGABRT on both backends; E03 is SIGABRT on LLVM and WRONG-ACCEPT on Cranelift;
    - E05 is SIGSEGV on both;
    - A01 and E02 are SIGTRAP on LLVM and SIGILL on Cranelift;
    - P01-P06 and compare are REFUSED on LLVM and WRONG on Cranelift;
    - the four wildcard holds are EXACT on both.
  - Correction to the earlier inventory: U05 on Cranelift is SIGSEGV. The earlier "rc 0, empty output" reading took its exit code from the wrong pipeline.
  - The receipt is tmp-chirho/native-fail-loud-chirho/rebaseline-a5f7d483-chirho/: classes table sha 77c110ec, C-locale directory digest e8bd889b, 1,313 files.
- [x] C3, the runtime error route (written; unbuilt until the builder handoff):
  - `ffi_chirho/failure_chirho.rs` holds the one route: Rust stdout and C stdio flushed, one `haskelujah: error:` or `haskelujah: internal error:` line, exit 1;
  - `error`, `undefined`, the new `no_match`, `uncaught_exception` and unbound-thunk entries use it, and so do the allocator, main-thread and thunk invariants, which were `abort()`s. A blackholed thunk reports `<<loop>>`;
  - the unbound thunk is one per name, rooted for the program's lifetime, so a loop that mentions the name allocates once;
  - R2's check sits in `show_int`, `print_int` and the `[Int]` heads. `show_bool_list` formats no integer, so it is not an R2 site;
  - `tests/native_failure_chirho.rs` observes each case from a child process: exit 1, the exact stderr line, and the report after buffered C output on a merged stream.
- [x] R1 on both backends (LLVM C4 c28988f4; Cranelift C5). Written, unbuilt.
  - LLVM records each function's unbound free ids. A reference to one becomes the runtime's failing thunk for its name, so an untaken path still runs, and a parameterless CAF now compiles.
  - Cranelift's pre-scan embeds the name of every unbound free id beside the string literals. A reference to one becomes the same thunk.
  - A Cranelift miss outside that set keeps its 0 placeholder: a bound local that lowering did not bind. That residual is outside this unit, like letrec zero initialization.
  - The parameter alias and the helpers only it used are gone from both backends.
- [x] R4 on both backends, with Cranelift's `^#` refused.
  - LLVM is unchanged. Every executable path (CLI, harness, driver) already returns the refusal as `Err`. The panicking wrapper is test-only, and the IR preview never builds an executable.
  - Cranelift records every primitive that reaches `lower_primop`'s default arm and refuses the executable with the sorted list, in LLVM's wording. `^#`'s left-operand arm is removed, so it reaches that arm.
  - Names whose calls Cranelift implements at the call site are exempt (`putStrLn#`, `returnIO#` and the rest). The IO-action pass gives their adapter bindings bodies of the same primitive, and those bodies never run in place of the call-site lowering.
  - Desk check: among primitives LLVM lowers, the newly refused ones are only `compare#` and `compareChar#`. No Cranelift round trip or shared native suite reaches a refused family.
- [x] R3 on both backends, with `errorWithoutStackTrace` (C4, C5).
  - LLVM: `error`, `error#` and `errorWithoutStackTrace` force the message and raise it on the program route. `undefined` calls the runtime.
  - Cranelift: the plain `error` primitive joins `error#`, `errorWithoutStackTrace` joins the call-site `error` arm, and `throw#` reports `uncaught exception`.
  - `undefined` fails where it is referenced on both backends, as LLVM did before.
- [x] R2: the runtime's checked formatting (C3), and the LLVM `%ld` sites routed through it (C4): Int `show`, list heads that default to Int, and a pure `main`'s result.
- [x] R5 on both backends.
  - LLVM's four defaults call the runtime's no-match entry. Since C8, so does a case with no alternatives, which used to return its scrutinee.
  - Cranelift calls it before each trap and for a case with no alternatives.
- [x] C8, from the read-only desk check. That check was a code read without Cargo, so it could not see Clippy lints, and its earlier "no clippy problem" overclaimed: gate r2 found one (see Gate runs). Its findings:
  - LLVM's zero-alternative case now fails named, as Cranelift's does;
  - `errorWithoutStackTrace` joins Cranelift's call-site exemption list, matching its call-site arm;
  - the thunk allocator's last `expect` (a panic, so an abort, inside `extern "C"`) uses the internal route.
  - Reported, not changed:
    - Cranelift's call-site `show` and default-Int paths pass unforced values to the R2 check, so a lazily bound Int reports the R2 line. Forcing there would turn WRONG output EXACT, which is beyond this unit's movement.
    - `undefined` is strict on both backends.
- [x] Controls (C6), one per route per backend, with exact stderr, exit 1 and no signal. Written, unbuilt.
  - Placement: they live in `tests/native_error_helpers_chirho.rs`, the existing native error suite, not in a new file. `tests/` already has 25 entries against a cap of 15.
  - Routes and probes:
    - R1: U01 and U05, plus a hold where an unbound name on an untaken path runs.
    - R2: C01.
    - R3: E01, E03, E05 and M01.
    - R4: P01 on both backends and P06 on Cranelift, each with its exact refusal text.
    - R5: A01.
  - The RTS cases (C3) and the LLVM unit tests (C4) control the same routes below the driver.
  - Tests that change on purpose:
    - LLVM's two alias unit tests (C4);
    - two more LLVM unit tests found at implementation, which asserted `snprintf` for Int `show` and `printf` for a pure `main` and now assert the checked runtime formatters (C4);
    - `native_error_helpers`' failure helper, which accepted a signal and now requires exit 1 with the exact line (C6).
- [x] C7: workflow `spec-chirho/workflows-chirho/testing-chirho/native-fail-loud-chirho.md`, cited by every new module.
- [x] C10 (a14baa14), after gate r1 (#28421, #28422).
  - Cranelift lowers `compare#` and `compareChar#` in `lower_chirho/ordering_chirho.rs`, with LLVM's semantics: signed less gives LT 0, equal gives EQ 1, anything else gives GT 2, using `con_tag_for_chirho`'s tags. `compareFloat#` and `compareStr#` stay refused.
  - The new end-to-end control `int_and_char_compare_answer_as_ghc_on_cranelift_chirho` checks that `compare (2 :: Int) 1` and `compare 'b' 'a'` print GHC's `GT` and `GT` on Cranelift.
  - Unit counts: 16 paths, 11 commits and 24 driver controls.
- [x] C12 (c7015a2f), after gate r2 (#28426, #28427).
  - It collapses the call-site `error` arm's nested `if` into an edition-2024 let chain.
  - Its `collapsible_if` predates row 535 (a5f7d483 `lower_chirho.rs:1402`), but C5 changed that condition, so the gate counts it as the unit's.
  - Behavior is unchanged, and every other pre-existing warning on an unchanged line stays.
  - Unit counts with C13: 16 paths, 13 commits and 24 driver controls.
- [ ] Checks without Cargo:
  - pinned rustfmt;
  - `diff --check`;
  - gitleaks;
  - file and directory sizes;
  - a read-only desk-check for compile errors.

## Gate

The gate runs only under a builder handed by name, after source and tooling review.

- [ ] Every R1-R5 control is GHC-reference-checked and produces its named failure, or its refusal, on both backends.
- [ ] Holds:
  - the four wildcard programs stay GHC-exact natively;
  - the 11 PASS native corpus programs hold;
  - the native round trips hold: 88 LLVM, 85 Cranelift and the shared native suites.
- [ ] The execution corpus stays at 30/35. Only SectionsChirho's native detail moves, from a pointer to the named R2 error.
- [ ] Membership, replay, the run predicates and the reject predicates are byte-identical.
- [ ] Each mutant is caught by its own control, with exact isolated failure sets. The mutants cover:
  - aliasing and zero, on each backend;
  - pointer formatting;
  - the missing `error` arm;
  - stdout plus abort;
  - the runtime's abort;
  - the `unreachable` and trap defaults;
  - `^#`;
  - `errorWithoutStackTrace`;
  - the internal prefix;
  - M16 (#28422): the Int/Char Ordering back to the pre-535 constant 0. Its isolated killer is the new Cranelift semantic control. M04 stays scoped to the unknown-primitive zero fallback.

### Gate runs

**r1, RED** (tag `nfl535-native-fail-loud-c431f218-r1-chirho`, run once at 02:51:09 EDT on 2026-10-26 under the builder gpt_chirho handed over in #28419). The tag is never resumed.
- Ledger:
  - preflight GREEN at c431f218 (crates bb012aff);
  - build GREEN with rc 0 and no warnings. Frozen CLI `c1f1feb6281a129f0ef26d2c91b640592ca9a36e0f63a27f00c98ef50c6e74bc`; RTS lib `7ef80b4536171d6287e75643a8d3c5e8d0bd81f168ca66d079e582f7597f1b56`;
  - focused GREEN for rts-lib 47, rts-native_failure 10, rts-native_thunks 1, llvm-lib 38, cranelift-lib 42, native_error_helpers 23, native_laziness 5 and codegen_scaling 5;
  - focused-cranelift_roundtrip RED: rc 101, 32 passed and 11 failed. Exit codes were build 0 and focused 1.
  - Not run: the rest of focused, then lint, oracle, corpus and mutants (with postflight).
- Each of the 11 failures is `Cranelift does not support primitive(s): compare#/2`, the complete refusal list. The failing tests:
  - `cranelift_integral_operations_use_their_runtime_evidence_chirho`;
  - the `cranelift_round_trip_*_chirho` tests applyop_divide, applyop_modulo, deep_tail_recursion, euler1_tail_recursion, guard_where_binding, merge_sort_preserves_all_elements, non_tail_recursive_list_fold, otherwise_guard, user_range_large and where_div_mod_binding.
- Root cause, which corrects #28416 2d:
  - these programs never call `compare`. A reachable Prelude `Ord` dictionary compiles its method body, `compare#` for Int (`instance_chirho.rs:111-114`);
  - Cranelift used to lower that body to a silent 0, and LLVM lowers it itself;
  - I had scanned test sources for `compare` instead of treating every primitive LLVM lowers as reachable.
- After the run, the tree was clean at the tip, no process was alive, and every unit file matched its tip blob.
- Receipts are kept byte-exact as two sets, each digested over its sorted file hashes:
  - the run directory: 3,526 files, `47f0f1a5211f3f8a735534e6d710d1a33ec00e8780b639946551ffe4876af938`;
  - the launcher directory: 3 files, `252e928e1e002cb5f8a3a8fe62c5e23175a559ca5981ee052f9f80506a48a44d`.
- Disposition (#28422): C10, a 41st contract row, and M16.
  - The new row is `IntCharCompareChirho`, with its GHC output re-derived without Cargo and its provenance in `contract-rows-chirho/`.
  - The contract now has 41 programs and 82 native cells: 64 moving and 18 held.
  - Compare2's Cranelift refusal is now exactly `compareFloat#/2, compareStr#/2`.

**r2, RED** (tag `nfl535-native-fail-loud-73f4a656-r2-chirho`, run once at 03:15:56 EDT on 2026-10-26 under the builder gpt_chirho handed over in #28424). The tag is never resumed.
- Ledger:
  - preflight GREEN at 73f4a656 (crates b0e55df0);
  - build GREEN with rc 0 and no warnings. Frozen CLI `54b9067cc5433896e637f57fbc7cb632898d42e34ae95b22a36e61d30eb6350c`; RTS lib `7ef80b4536171d6287e75643a8d3c5e8d0bd81f168ca66d079e582f7597f1b56`, unchanged from r1 because C10 and C11 leave the RTS alone;
  - all 13 focused suites GREEN, so C10 cleared r1's failure:
    - rts-lib 47, rts-native_failure 10, rts-native_thunks 1;
    - llvm-lib 38, cranelift-lib 42;
    - native_error_helpers 24, native_laziness 5, codegen_scaling 5, cranelift_roundtrip 43;
    - driver-native 127 (1,726 filtered out), show_precedence 2, io_actions 9, typing_integration 40;
  - lint RED; gitleaks GREEN, 11 commits and no leaks. Exit codes were build 0, focused 0 and lint 1.
  - Not run: oracle, corpus and mutants (with postflight).
- The one real lint is `clippy::collapsible_if` at `lower_chirho.rs:1430`, the only diagnostic on a changed or inserted line or in a new file. It predates row 535 (a5f7d483 `lower_chirho.rs:1402`, the same `if`), but C5 changed its condition. C12 fixes it.
- A rendering artifact, not code:
  - The tip's Clippy ran in the worktree's warm target directory. For 7 crates it did not re-check (ast, diagnostics, incremental, naming, package, syntax and th), cargo replayed output cached in the long form on 2026-09-18 and 2026-10-12.
  - The base ran in a fresh target directory and printed the short form.
  - The judge left those 86 diagnostics unparsed, so the unchanged lines counted 337 against 423 (the driver 374 against 460).
  - Each of the 86 matches a base diagnostic one to one by primary path, line and message, and no tip diagnostic is new.
- Both causes repeat traps this fleet had already met: a touched condition's old warning (2026-10-23) and warm-tip replay (row 533). The desk-check overclaim is recorded under C8 above.
- After the run, the tree was clean at the tip, no process was alive, and the frozen CLI and RTS lib were unchanged.
- Receipts are kept byte-exact as two sets, each digested over its sorted file hashes:
  - the run directory: 4,920 files, `b6bec34a0659db48500be8a33857922c6b00f2827600eeda43ca127693595246`;
  - the launcher directory: 4 files, `ebd788c04b51cc1f1823052bfaebb3d74fddf605e4117eb88e7d28e816da9373`.
- Disposition (#28427): C12, this record (C13), and fresh Clippy targets.
  - The lint stage gives the tip its own fresh run-local target directory, `tip-target-chirho`, separate from the base's and the worktree's. Both sides start cold and print the requested short form, and the stage fails fast if either target already exists.
  - The parser, the changed-line rule and the unchanged-line multiset rule do not change.
  - The tooling proof runs the literal lint stage on a recording stub. One negative restores the warm route, and two pre-create a target.
  - The contract (41 programs, 64 movers and 18 holds) and all 16 mutants are unchanged.

**r3, RED** (tag `nfl535-native-fail-loud-f5d896da-r3-chirho`, run once at 03:43:10 EDT on 2026-10-26 under the builder gpt_chirho handed over in #28429). The tag is never resumed.
- Ledger:
  - preflight GREEN at f5d896da (crates 0a9f3727);
  - build GREEN with rc 0 and no warnings. Frozen CLI `a216bbf800f92db8d728f29bba2ae68058666ceca5ba4677e7c36f9a5472dbec`; RTS lib `7ef80b4536171d6287e75643a8d3c5e8d0bd81f168ca66d079e582f7597f1b56`;
  - all 13 focused suites GREEN, with r2's counts;
  - lint GREEN, with both sides linted cold in fresh run-local targets, so C12 and the fresh targets held. gitleaks GREEN, 13 commits and no leaks;
  - oracle RED: 119 PASS and 7 FAIL in 126 checks, all seven on Cranelift. Exit codes were build 0, focused 0, lint 0 and oracle 1.
  - Not run: corpus and mutants (with postflight).
- Six Cranelift REFUSED cells (Compare2, P01, P03, P04, P05 and P06) were refused with exactly the contract's primitive lists.
  - But the CLI printed them through its pre-existing wrapper: `cranelift compilation failed: Cranelift does not support primitive(s): …` (`haskelujah-cli-chirho/src/main.rs:816`, outside this unit).
  - The contract had assumed the bare line, which only the in-process driver controls print.
- K01's Cranelift cell was held as BASE (SIGBUS, rc 138). It now compiles, exits 1 with empty stdout, and prints exactly R2's line, `haskelujah: error: show: no native Show instance for a boxed value`.
  - The likely reason, an inference and not an established cause: its `show` path reaches the checked runtime formatter before the base's fault.
- After the run, the tree was clean at the tip, no process was alive, and the frozen CLI and RTS lib were unchanged.
- Receipts are kept byte-exact as two sets, each digested over its sorted file hashes:
  - the run directory: 6,831 files, `2459027a95aa5127f69aa6244b1a3bd209deca703e64e47af364176ab68d67e5`;
  - the launcher directory: 5 files, `9e5edc8ec2a414cf050f3f43211000a95de9a7c8849e21e4785506bc0b854811`.
- Disposition (#28432): both oracle corrections, with no source change and no lldb.
  - A Cranelift REFUSED cell now requires the CLI's exact wrapped line, and LLVM's stays bare and exact. Selftest negatives cover a missing, changed or LLVM-style Cranelift prefix, and the wrapper on LLVM.
  - K01's Cranelift cell becomes `NAMED:show: no native Show instance for a boxed value`, with negatives for its exit code, stdout and line.
  - The contract is now 41 programs and 82 native cells: 65 moving and 17 held (58 NAMED, 6 REFUSED, 10 EXACT and 8 BASE).
  - The updated judge is GREEN on r3's own oracle receipts (126 of 126), and its base view is RED on exactly the 65 movers.
  - C14 is this record plus the workflow's wrapper note: 16 paths and 14 commits. The crates tree stays 0a9f3727, and the 16 mutants and the baseline of 38 are unchanged.

**r4, RED at mutants only** (tag `nfl535-native-fail-loud-de348c32-r4-chirho`, run once from 04:04:44 to 04:36:25 EDT on 2026-10-26 under the builder gpt_chirho handed over in #28434). The tag is never resumed.
- Ledger:
  - preflight GREEN at de348c32 (crates 0a9f3727): 26 judge checks, 16 anchors, 9 tools and 10 immutable sets;
  - build GREEN with rc 0 and no warnings. Frozen CLI `a216bbf800f92db8d728f29bba2ae68058666ceca5ba4677e7c36f9a5472dbec`, the same as r3's; RTS lib `7ef80b4536171d6287e75643a8d3c5e8d0bd81f168ca66d079e582f7597f1b56` at build;
  - all 13 focused suites GREEN, with r2's counts;
  - lint GREEN, cold on both sides; gitleaks GREEN, 14 commits and no leaks;
  - oracle GREEN: all 41 programs and 126 checks, 65 moving and 17 held, each exactly as the contract says;
  - corpus GREEN: accept, reject and replay identical to row 534 r4, execution verdicts identical at 30/35, and SectionsChirho's native run moved from a pointer to the named R2 failure;
  - mutants RED; postflight GREEN, with the frozen CLI unchanged, the tree clean at de348c32 with all 8 mutated files at their tip blobs, the gate, tools and immutable sets at their pins, and nothing alive.
  - Exit codes were build 0, focused 0, lint 0, oracle 0, corpus 0 and mutants 1.
- The mutation partition:
  - baseline VALID with 38 passed; restored BASELINE with 38 passed; caught 16 of 16;
  - new-failure counts: M01 4, M02 2, M03 2, M04 2, M05 3, M06 1, M07 1, M08 3, M09 9, and 1 each for M10 through M16;
  - every set matched its prediction except M02's.
- M02 (the Cranelift parameter alias) killed `an_unbound_call_fails_named_on_cranelift_chirho`, as predicted, and also `undefined_fails_named_on_cranelift_chirho`, which ran to rc 0 printing `0`.
  - M02 re-creates base's `bind_missing_param_aliases_chirho`, which C5 removed. That alias also binds `undefined` to a parameter, so the R3 route never runs.
  - Base did the same on the same program: E03's 15e1b77b Cranelift run was rc 0 with stdout `0`.
  - The second kill is genuine evidence, so M02 is not narrowed.
- The RTS archive is not part of the evidence, by design.
  - `target/debug/libhaskelujah_rts.a` was `7ef80b45` at build and `5c2ba764` after the mutants stage. It was rebuilt from tip source in M16's run, after M15's RTS edit was restored, with the same size.
  - Rebuilt dev archives are not claimed reproducible, so no archive-hash invariant is pinned (#28437).
  - The landing invariant stays explicit: the 8 mutated source files restored to their tip blobs, and the restored 38/38 baseline.
- Receipts are kept byte-exact as two sets, each digested over its sorted file hashes:
  - the run directory: 21,175 files, `3b1103be6e9aaa239921f1d114c563bdf7bd8ea4c1ee9dcc2a3e232082c94766`;
  - the launcher directory: 7 files, `eea2798ddcd720af8942739a46d980d70dfc35cb2e071b2908dcdfc5cbda0e61`.
- Disposition (#28437):
  - M02's predicted set becomes exactly those two tests. Its control and signal reason stay.
  - Re-judged with the corrected table, r4's preserved mutations log is GREEN (19 of 19).
  - The judge's selftest judges that measured log against the table (GREEN) and against the old one-failure prediction (RED on M02 alone).
  - C15 is this record: 16 paths and 15 commits, with the crates tree still 0a9f3727. The contract (41 programs, 65 movers and 17 holds) and the other 15 mutant definitions are unchanged.
  - r5 runs the whole gate once more, from build through mutants and postflight.
