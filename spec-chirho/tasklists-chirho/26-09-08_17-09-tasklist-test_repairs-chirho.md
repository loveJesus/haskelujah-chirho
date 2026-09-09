<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Test repairs, 2026-09-08

L.J. directed: "please do a work as deep as needed to help our tests all pass".
Owner: HASKELUJAH/gpt_chirho. Starting commit: 033fc4df. Builder and progress DB lease
claimed in room message 21542; implementation in the existing gpt worktree on
gpt-test-repairs-chirho. This is reversible compiler/test repair, with no public deployment.

## Acceptance and placement

Repair the actual compiler/runtime behavior exercised by failing tests. Keep valid
assertions, establish GHC semantics before repairing a malformed input, and do not
increase a score by suppressing diagnostics or silently dropping syntax. Generated
programs must return the expected output and terminate. Preserve laziness and the
distinction between names, fields, constructor arguments, and quantified variables.

Large files are a current structural defect. New substantial behavior belongs in
focused modules beside the affected subsystem; move the relevant existing mechanism
when needed, and put new behavioral tests in subject-specific integration files.
Do not add another large mechanism to infer_chirho.rs or compile_chirho.rs.

The workspace suite is the first end-state gate. The two GHC measurement corpora
remain separate compatibility evidence, with exact sets and reasons for movement.
Final reporting must state any unresolved failures and environment limits plainly.

L.J.'s 2026-09-08 clarification: the GHC tests are the compatibility target. Runtime
and harness repairs support that objective; workspace greens are not corpus gains.

## Work

- [x] Read current instructions, prior handoff, live Git state, and lease state.
- [x] Reproduce the current workspace failures using freshly built test targets;
      isolate the known runaway and memory-heavy test during the initial inventory.
- [x] Fix reproduced frontend/runtime failures at their root, with focused execution proofs.
- [x] Fix reproduced generated-code failures and make runaway child execution fail with evidence.
- [x] Fix reproduced parser/package/fixture failures, including worktree portability.
- [ ] Run the complete workspace gate including previously failing/hanging tests;
      resolve new failures rather than counting skipped tests as passing.
- [ ] Run lint/format checks, inspect structure and changed workflows, and validate the
      two corpus axes when checker/parser changes reach the landing boundary.
- [ ] Commit owned paths, update the progress row with exact evidence, land the tested
      changes, and release the builder and DB lease.

## Evidence and decisions

- Starting tree is clean. No compiler build was running at claim time.
- The prior handoff is a triage guide, not fresh proof: it names four driver unit
  failures, two parser containers tests, parser stack overflow, two Cabal tests,
  one golden parse case, LLVM executable tests, Cranelift recursive IO, and the
  Cranelift custom-list-instance runaway. Each will be reproduced or retired with
  current evidence.
- Initial workspace inventory uses fresh 033fc4df test binaries, one test thread,
  RUST_MIN_STACK=16777216, initially excluding the documented native list runaway
  and in-process GHC should_compile bulk test. It ended with eleven red targets;
  four doctest failures reported rebuilt dependency metadata, so those are not
  established baseline compiler defects. Exclusions are not passes and both
  previously excluded targets remain final-gate work.
- Two driver inputs repaired with independent semantic evidence: the record-pattern
  test's Rust line continuations stripped the Haskell indentation (GHC rejects the
  flattened source); its intended program already prints 42. The nested visible
  type application needs a specified forall binder and nested list inputs for
  @[Int]. Both corrected tests pass without a checker change.
- Six LLVM assertions pinned unforced operand spelling. Native execution confirms
  the intended arithmetic, floor division/modulus, comparisons in both directions,
  negation and alias behavior. LLVM 42/42 pass after replacing those assertions.
- Native dictionary repair: remove name-based erasure of runtime evidence. A
  focused simplify_chirho/dictionaries_chirho.rs reduces only a structurally
  proven selector/constructor projection; unresolved dictionaries remain runtime
  parameters. Cranelift must enter a function thunk before interpreting it as a
  closure. The recursive custom-list program now terminates with 1:2:3:[].
  Core 130/130; backend Cranelift 41, LLVM 42, Wasm 12 pass on this mechanism.
- Native IO's nonzero return is the backend not recognizing the desugarer's unit
  constructor spelling $tuple0. Both native tag tables now recognize that spelling;
  its existing recursive-IO execution test will gate it.
- Test infrastructure decision: use the existing test-harness crate for strict
  native round trips and bounded process execution, not another helper bolted onto
  the 10k-line driver test file. Compile/link failures must fail a test; deadlines,
  pipe limits, stdin closure and process-group cleanup are independently tested.
- Baseline A/B with the strict native harness: 117 of 126 round trips pass at
  033fc4df, nine fail. Four previously reported greens were signals/link failure:
  Cranelift comprehensive (SIGSEGV), LLVM qsort500 (SIGBUS), LLVM string-case/readInt
  (SIGTRAP), LLVM recursive custom-list (link failure). Five exceed 15 seconds;
  only the already-isolated custom-list case is established infinite, not the
  other four. Backend unit counts are a different target and are not invalidated.
- Show evidence is compound in the METHOD-occurrence path, not in the reference
  and literal paths. Preserve parentheses in nested compound keys and construct
  recursive Show dictionaries with field precedence 11. Eight interpreter
  examples and nested Just on both native backends match GHC's expected output.
- Dictionary defaultability recursively inspects free variables of argument types,
  including tuples, lists and higher-order arguments. An initial extension to
  result-only variables overreached the checker's current defaulting policy:
  ordinary `main` bindings became unapplied dictionary functions, causing 244
  driver failures. That overreach was removed at the policy boundary, not guarded
  per test. Result-only monomorphism restriction remains a compatibility limitation.
  Focused tuple evidence, pattern guards and integer API results now pass.
- Native thunk state and payload now occupy separate words; memoization retains
  every result bit and entry chases the full indirection path. Direct tests cover
  negative integers, float bits, 151-step chains, single evaluation, and allocation
  during entry. This is distinct from STG Show formatting; no unmeasured causal
  connection is claimed.
- GC boundary: a root value of 42 followed by forced collection reproducibly
  SIGSEGVs in the old guessed-indirection path. Roots are values, not root-slot
  addresses; only allocation-registry membership permits following them. Zero
  pushes now balance their pops. RTS 46 unit tests pass, including these two
  controls and a retained-heap collection-work bound with cold retirement.
- LLVM primitive lowering is extracted into a focused module. Unknown primitives
  no longer silently become addition or zero; executable generation returns a
  diagnostic. IR preview remains non-executable and carries explicit failure
  stubs. Both CLI and Cabal executable generation consume the checked API.
- The intermediate native rerun reached 121/126, but RTS was rebuilt during that
  run: this is triage only, not a frozen-artifact landing gate. Three failures
  identified previously unsupported compare/error primitive paths; two were
  native crashes. A fresh frozen rerun remains owed.
- The subsequent frozen run was 122/126. Its four failures were Cranelift's
  comprehensive program (a direct thunk ignored callee saturation), LLVM's
  qsort500 (a captured closure was not rooted), LLVM's non-tail-recursive fold
  (conditional dictionary forward-reference identity), and the 100-million-step
  Euler program's 15-second deadline. The first three now pass focused exact-output
  tests. Conditional dictionary generation moved into a focused child module;
  forward references and definitions now reuse the same binder id. Full native
  remeasurement remains owed.
- The curated harness recognized only `EXPECT_OUTPUT`, while 500 files used
  `EXPECTED`. Fourteen run tests actually compared output; five hundred silently
  ignored their declared oracle. Both spellings and escaped output are now read;
  a missing oracle fails. Wrong-answer controls were demonstrated red on the old
  reader, then green on the repair. Significant spaces and literal backslashes
  are covered. No `EXPECT_OUTPUT` header was changed.
- First full curated measurement with actual oracles: **514/537**, 23 failures,
  on the worktree. This is not a compiler regression from 537: the old measuring
  instrument did not compare 500 answers. The two published typecheck axes are
  separate and have not been remeasured or superseded by this number.
- Every one of the 23 unchanged inputs was run under installed **GHC 9.14.1**
  before changing an oracle. Classification and repairs:

  | Files | Independent evidence | Action |
  | --- | --- | --- |
  | T088, T089, T145, T187, T211, T216, T222, T366, T383, T419, T428, T436, T438, T537 | GHC matches our output, not the header | Correct header to the observed GHC result; program unchanged |
  | T354 | GHC also prints four tokens / parse 0 / eval 0 | Repair the miniature parser to consume its lexer's TEnd, exclude that sentinel from the reported token count; retain the intended original oracle, reverified by GHC |
  | T108, T178 | GHC rejects ambiguous local enumFromTo versus Prelude | Explicitly hide Prelude.enumFromTo; GHC then gives 220, so correct the 120 header |
  | T379 | GHC rejects an unclosed implicit let block inside explicit do braces | Add explicit braces to the let; GHC then gives the original oracle 25164150 |
  | T149 | GHC matches 25164150; ours gave 24661650 | Fix backtick operator lookup: canonical names do not include the backticks; div/mod/quot/rem use precedence 7 |
  | T278, T424 | GHC matches original separator rendering; ours applies String Show to later numbers | Preserve recursive reference evidence until SCC generalization supplies its predicates |
  | T423, T426 | GHC matches originals; ours throws on a constructor tag | Same deferred recursive evidence, plus nested-call rewriting must honor the reference's proof instead of surrounding result-type guesses |

- The five genuine curated failures all pass focused execution now. Dictionary
  evidence tests are 9/9, including Bool/Double recursive rendering and tuple
  numeric evidence; curated-root tests are 3/3. Core 130, typing 315 (one existing
  ignored test), RTS 46 plus the native-thunk integration test passed on the
  then-current code. The complete curated rerun passed **537/537** in 558 seconds:
  all 514 compile-and-run oracles were actually compared. The old 537/537 compared
  only 14, so the identical headline describes materially different evidence.
- Native performance is distinct from correctness. Profiling the Euler test
  shows unoptimized Rust dictionary hashing/locking on immediate values, not
  stack growth (roughly 1 MiB RSS). Thunk entry now avoids an update lock when
  no thunk was entered, and empty allocation registries avoid hashing. Testing
  an optimized native RTS in debug builds keeps debug assertions and information.
  The unchanged 100-million-step workload returns 2333333316666668 in 17.92 seconds;
  its test has an explicit 60-second budget, while ordinary round trips keep 15.
- Frozen native round-trip gate is now **126/126**, 184.91 seconds, including the
  previously hung custom-list program, both recursive IO paths, and all four
  baseline signal/link false greens. No skipped round trip is included.
- Parser fixtures are now vendored with upstream licenses and provenance.
  The original lexer test used a fixed byte offset into host-dependent CPP output;
  it now locates the same unsafeCoerce# argument by its containing declaration.
  The IntSet preprocessing failure was host cpp interpreting Haskell RULES syntax
  as a C directive. Pinned GHC-preprocessed sources keep the original AST assertions
  and remove the untracked package-cache requirement. Parser verification without the
  cache symlink: **316 passed, zero failed, three existing ignored**, with the existing
  RUST_MIN_STACK=16777216 test-stack setting. The cache itself was not changed.
- The long-running baseline bulk-summary child was sampled before termination:
  **42.2 GiB physical footprint**, stack in LLVM compile_module_chirho appending
  lifted-function text. The sample establishes code-generation growth, not the
  previously assumed external-watchdog explanation. Only owned child PID 55160 was
  terminated; parent cargo 71982 was paused during the initial corpus passes and
  later completed. The input was subsequently isolated as T2045 (details below).
  The interrupted inventory was not a passing or deliberately ignored bulk test.
- Corpus passes started with explicit rebuilt CLI SHA-256
  81906a00d7c6220fdce29f15a85bc03ecd0ae3984a13f2aac6566203346f59d0.
  The runner preserves the published error[E/panic rejection rule but now separately
  fails unexpected nonzero exits instead of counting them as accepts. An exit-1
  control produces accepted=0, unexpected_exits=1 and makes the runner fail.

- Four completed corpus passes had byte-identical sets to the committed 877/938
  and 222/767 lists, zero timeouts/unexpected exits and unchanged CLI digest.
  This precedes the later shared-case/native repairs; a fresh final measurement
  remains owed and no artifact has been superseded yet.
- Cabal conditional selection now distinguishes a missing branch from the first
  available branch; the focused parser is extracted into a child module. Package
  tests: 92/92. Golden parse list-comprehension pattern expectation corrected: 1/1.
- MaybeT IO examples now bind IO results before printing, with independently
  verified GHC output `Just 15`. STG enters a returned thunk when a case demands
  its constructor, without forcing an unused payload. Three controls pass;
  general `MaybeT m` inference is not claimed solved.
- The bulk failure was reduced to T2045. Phase markers placed the first 1 GiB
  breach in desugaring after 3.31 seconds, before LLVM. Nested case alternatives
  repeatedly expanded every remaining suffix. A focused matches module now
  desugars each row once, in source order, with shared lazy failure continuations.
  T2045 full compilation completes in approximately 3.5 seconds; a subprocess
  gate requires completion within 90 seconds and 1 GiB. Synthetic 8/16/32-row
  cases bound Core size relative to source size. Bulk typecheck tracking is now
  frontend-only, as its claim requires: target 8/8, not all corpus inputs green.
- Shared native thunk preparation closes nonrecursive let RHSs and constructor
  fields over one packed environment argument, independent of capture count.
  Native printing demands strings at the C boundary. Cranelift tail-let lowering
  now shares ordinary-let initialization instead of evaluating terminal failure
  before selecting a pattern. Three scaling/pattern controls pass, including one
  exact-output program on STG, LLVM and Cranelift with an unused error field.
  Subsequent native gate: 125/126; string packing failed to demand character
  fields. That repair and demanded-field controls are being verified.
- Required package fixtures are consolidated under
  test-data-chirho/package-fixtures-chirho with upstream licenses and exact
  package-relative source paths. These are slices, not complete package builds.
  Parser and driver share the raw sources; parser-only CPP outputs stay pinned.
  With no cache symlink, all 19 previously missing-fixture driver tests plus three
  LLVM tests changed from incidental IR assertions to native execution pass:
  22/22, 134.42 seconds. Optional whole-package tests still need provisioning and
  must not be counted as package-compilation proof when their cache is absent.
- Two new lazy-field controls pass on all three execution engines: nine captured
  inputs produce 45 without demanding the other field; demanding that field fails
  with its own message. Together with three scaling tests this is 5/5, but only
  the nested-pattern test and the two field tests run all three engines. The two
  other tests measure bounded compilation and Core growth.
- Native linking now agrees on target/deployment metadata instead of emitting
  LLVM override or missing Mach-O platform warnings. The shared harness fails on
  build/link stderr as well as status. The five controls pass with this stricter
  instrument. Two raw-pointer APIs have explicit unsafe contracts and documented
  Rust call-site proofs; generated ABI signatures are unchanged.
- Full workspace gate started on the frozen implementation with
  RUST_MIN_STACK=16777216, three build jobs and one test thread. It includes the
  formerly hung native test and all bulk targets; no new skip filters. Lint and
  formatting still have broad pre-existing failures, including oversized files;
  these are not silently treated as green.
- The frozen workspace run has completed all 126 driver native round trips with
  exact outputs and zero failures, including the repaired string-packing case.
  Six structural tests are red: four LLVM checks name particular temporary
  registers, and two Core checks require an outer Case rather than a shared lazy
  continuation. Their replacement is behavioral, not a new pinned spelling:
  native output/file-content assertions in a small LLVM test module, plus the
  two pattern cases executed by the driver. GHC independently gives the pattern
  control outputs 2/1/2/1/0/42. The rest of the full gate is still running;
  source and binaries remain frozen until it finishes.
- The frozen driver-library run completed: **1766 passed, zero failed, zero ignored,
  zero filtered**, 3056.08 seconds. Canaries 7/7 and scaling 3/3 passed next.
  The Cranelift integration target found three failures: one demands erasure of
  Integral selectors, while merge-sort and middle-equation fallback genuinely print
  pointer-sized values instead of list elements. Their output oracles stay unchanged;
  the numeric print demand boundary is under investigation. The rest of the full
  workspace run remains active.
- The step-limit property currently accepts any nonempty error and any non-Int
  success. Its repair will require the exact Fibonacci result or the requested
  runtime step-limit diagnostic, preserving all 100 randomized cases.
- The frozen workspace gate finished with nine failures across LLVM, Core and
  Cranelift integration. Seven pinned incidental structure; two unchanged list
  oracles printed pointers. After replacing the seven assertions with execution
  proofs and demanding numeric operands in Cranelift's direct printing adapter,
  LLVM is 42/42, Core 128/128 and Cranelift integration 43/43.
- Former ignores now run: parser 319/319, typing 316/316, syntax doctest 1/1.
  The parser cases were a flattened Haskell literal, an assertion missing a
  parenthesized lambda, and unpreprocessed CPP input. Original semantic assertions
  survive; the last case consumes a licensed GHC 9.14.1 preprocessed fixture.
  GHC's upstream Safe Haskell/GND warning is recorded in fixture provenance.
- The additional IO reuse control is genuinely red: a local action used twice
  prints once in STG. A native probe also fails; no successful three-engine
  result is claimed. Execution currently memoizes effects as ordinary thunk
  results. Brick-one placement: a focused shared Core execution lowering models
  an action as a reusable function inside a constructor and its result as a
  separate lazy payload. Sequencing executes the function; WHNF alone does not.
  This change is isolated in this worktree and remains unverified/unlanded.
- The IO reuse probe now passes on all three engines. The new action
  representation exposed two further boundaries: STG case binders must share a
  per-invocation scrutinee rather than recompute it with stale registers, and
  LLVM's legacy missing-parameter alias logic must not redefine an already
  loaded capture. These are repaired at their respective boundaries; the broad
  regression run and non-execution/lazy-result controls are still owed.
- Checkpoint only: the branch preserves the accumulated repairs and provisional
  IO lowering. This is not a tested main-branch landing. Neither measurement
  artifact nor the canonical progress row has been marked complete.
- Recovery checkpoint `ef11029e` preserves that state on `gpt-test-repairs-chirho`.
  The follow-up action controls now distinguish action WHNF, repeated execution,
  unforced return/bind payloads and fresh input. All three sources pass STG, LLVM
  and Cranelift against independently checked GHC 9.14.1 outputs. The existing
  IO group is 38/38; scoped catch/try/finally/bracket/atomically operations execute
  the inner action rather than treating an action value as its result.
- IORef helper generation had two copies and treated readIORef as a pure result.
  One focused generator now sequences read then write. MaybeT helper generation
  retains `m (Maybe a)` and consumes the checker-proved Monad dictionary, including
  its Applicative superclass for pure. Five interpreter controls pass with IO,
  Maybe and list instances; general transformer inference remains out of scope.
- Lazy function arguments now use the same thunk-producing boundary as fields.
  The first broader native run was 124/126: Cranelift's numeric loop aborted with
  a thunk blackhole and LLVM's 100-million-step loop exceeded 2 GiB. A finite
  descending demand analysis proves parameters demanded by every terminating
  branch of a saturated call, preserving laziness elsewhere. Both unchanged
  loop oracles pass focused (Cranelift 1.64 seconds, LLVM 29.28 seconds). The new
  ignored/conditional/partial/recursive-argument control passes on all three
  engines against GHC's 42/0/7/0. The full native rerun remains final-gate work;
  this does not establish the cause of every possible
  native blackhole or solve the general runtime representation problem.
- The sieve regression was traced to a GC tombstone, not a constructor mismatch:
  allocation paths collected before publishing the fresh return/register value.
  Safepoints now explicitly include pending allocations. Four forced-collection
  controls went red before the fix and pass after it; the unchanged sieve prints
  25 again. The separate native GC contract has not been conflated with this STG
  heap-index representation.
- The strengthened step-limit property passes all 100 randomized cases, requiring
  the exact Fibonacci value or the requested step-limit diagnostic (91.98 seconds).
- All-target clippy exposed a pre-existing Cabal test tautology (`condition || true`).
  The real assertion then failed: custom-setup existed but its setup-depends list
  was dropped. A focused setup parser now retains names and version constraints,
  scoped separately from library dependencies; package library tests are 92/92.
  Workspace lint warnings, including Chirho enum-name style warnings and large-file
  structural debt, remain reported failures rather than a zero-warning claim.
- Follow-up checkpoint is ready for remote backup after focused controls. The
  complete workspace run, all native round trips and final two-pass-per-axis CLI
  measurements remain owed before main can receive a verified landing.
