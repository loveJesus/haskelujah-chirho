<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Import scope for join and & Chirho

L.J.'s 2026-10-25 direction is GHC test passing. Agy's #28250 ranking and #28275 trace found that the builtin seed gives `join` and `&` bare names in every module, so a program that never imports `Control.Monad` or `Data.Function` still checks. GHC 9.14.1 reports GHC-88464 for T12406 and T12589. Claude checked the same facts independently (#28273).

Owner: claude2_chirho writes source-only (gpt_chirho #28284, progress unit 532). The reviewed checkpoint 7d478b30 on `import-scope-join-amp-chirho` came from exact main 9944ed4c. Its replay onto main 6ea3de75, on `import-scope-join-amp-main-chirho`, has a byte-identical unit diff (#28298). gpt_chirho owns the builder, main and the DB. No Cargo runs before the reviewed source checkpoint.

## Contract

- `surface_chirho/part_12_chirho.rs` no longer binds bare `join` or `&`.
- It binds each scheme, unchanged, under a module-qualified key for every builtin interface that exports the name. `Control.Monad`, `GHC.Base`, `Data.Function` and `Control.Lens` carry the function's own scheme. The semigroupoids group keeps the approximation its imports reached before.
  - `join` for `Control.Monad`, `GHC.Base` and the five semigroupoids modules (`Data.Functor.Apply`, `Data.Functor.Bind`, `Data.Functor.Bind.Class`, `Data.Functor.Alt`, `Data.Functor.Plus`);
  - `&` for `Data.Function` and `Control.Lens`, plus the 16 other `Control.Lens.*` names under which the builtin lens interface pushes its export set. Those 16 keep the scheme their imports reached before, as preservation. The exporters numbered 18 at unit 532, and the inventory reads them from the interfaces.
- Driver Phase 3.9 already gives an import the module-qualified scheme first, for open imports, import lists, hiding, qualified imports and aliases. Every import therefore keeps the scheme it reaches today, and a module that imports neither name no longer sees it.
- A module's own `join` or `&` keeps shadowing as before, at top level and locally.

## Claim

At check time, T12406 and T12589 are rejected for the unbound name, as GHC rejects them. No other corpus row moves.

The claim also includes a type-precision movement outside the corpus, as required evidence (#28298). A qualified import (`CM.join`, `DF.&`) found no module key before, so it typed the name as an accept-anything placeholder. The real module keys replace that placeholder with the imported function's scheme. The misuses J22 and A21 therefore move from accepted to rejected, as GHC rejects them (GHC-83865). Their unqualified twins were already rejected.

## Boundaries, named and not claimed

- Imported `join` and `&` have no runtime implementation. An imported use checks and then fails at run with "missing STG binding". That is identical before and after this unit, so it is a separate follow-up.
- An unimported fully qualified `Control.Monad.join` or `Data.Function.&` still checks, where GHC reports GHC-76037. The type environment seeds every canonical key, and the fix is a separate qualifier-scope unit.
- Semigroupoids' `join` is its `Bind` method. Its modules keep the Monad scheme they reached through the old bare fallback; that is preserved, not claimed GHC-exact.
- The 16 `Control.Lens.*` aliases are synthetic interface approximations; which of them really export `(&)` in GHC was not measured. Their keys are preservation and inventory evidence only, with no GHC oracle row and no movement claim (gpt_chirho #28311).
- `guard` and the other non-Prelude bare seeds are the same defect class. No should_fail row measures them, so they stay outside this claim.

## Controls

Every program is GHC 9.14.1-measured and embedded in `tests/typing_integration_chirho/import_scope_chirho.rs`, with a program GHC rejects written before its file name.

- Rejected as unbound, the 8 programs that use the bare name without importing it: no import; an import list without it; hiding it; a qualified-only import with a bare use. Each of these is covered for both names, plus the prefix form `(&)`.
- Accepted at check, the 7 imports: `Control.Monad` open, explicit and qualified; `GHC.Base (join)`; `Data.Function` open, explicit and qualified.
- Accepted and run with GHC's answer: a local `let join`, a top-level `join`, and a top-level `(&)`.
- Rejected as a type mismatch, the 5 misused imports: `join` at Int through `Control.Monad`, `GHC.Base` and a qualified `CM`; `&` with a Bool through `Data.Function` and a qualified `DF`. These are the discriminators for "imports keep the exact scheme", which a placeholder would accept.
- Inventory: there is no bare key, and the four named modules carry the function's own scheme. Every builtin interface that exports a name has a key holding the preserved scheme. That set is read from the interfaces, not frozen as a list, so it detects drift.
- Recorded and unasserted: the unimported fully qualified forms, Q01 and Q02.

## Checklist

- [x] Trace the seed, Phase 3.9 and every interface that exports either name (agy #28275, claude #28273, claude2 #28278).
- [x] Measure the 20 controls and the four corpus files on GHC 9.14.1 and frozen main 2ddfca25.
- [x] Write the controls and the module-qualified seed; rustfmt, diff-check, gitleaks, and GHC receipts from the committed file (GREEN 25/25).
- [x] Remeasure the before-state on the post-RC5 base CLI 77586815, over the 25 programs and the 4 corpus files. 14 programs GHC rejects are accepted: the 12 that must move, plus Q01 and Q02. 11 are accepted as GHC accepts them, and 4 are rejected as GHC rejects them.
- [x] Post the exact source checkpoint and the proposed gate, and take the builder only by name (#28297, #28307; handed by name in #28308).
- [x] Gate `g532-import-scope-e60cfd07-lcc-chirho` went RED at focused `typing_integration`, 2026-10-25 15:18 EDT. The exporter inventory named `Control.Lens.Type`: 16 lens aliases exported `&` and had no key. Its run (0af84475) and launcher (c47d0271) receipts are kept byte-for-byte as immutable sets (#28310, #28311).
- [x] Bind `&` under all 18 current exporters (4463d8b7), and name the lens aliases as preservation. Mutant m6 restores the e60cfd07 two-key list, and m7 drops one synthetic key; the inventory must name the exporter each time.
- [x] Gate `g532-import-scope-5284fd33-chirho`: GREEN through build, focused, lint, the oracle, the mutants, and a corpus pair moving exactly T12406 and T12589, with replay, execution and postflight (#28316, accepted in #28317). The gate record follows.

## Gate record

### RED: `g532-import-scope-e60cfd07-lcc-chirho`, tip e60cfd07

- The builder was handed by name in #28308.
- Build was GREEN, with frozen CLI 562972cff75a6acb0b7e4cae9251108f42cda639afe8e630c7c903f939bd6ef8, and six focused suites were GREEN.
- `typing_integration` went RED with rc 101, 39 passed and 1 failed. The exporter inventory reported "`Control.Lens.Type` exports `&`, so its key keeps the preserved scheme". Lint, the oracle, the mutants and the corpus did not run.
- Cause: the builtin lens interface pushes one export set, which spells the operator `"(&)"`, under 17 Control.Lens names, but only Data.Function and Control.Lens had keys. The three source enumerations had searched for `"&"`.
- Both receipts are kept byte-for-byte as immutable C-locale sets: run 0af844757834929dacd0daf059671db7b14b716b724fe06d6e7400bfb1138ff8 and launcher c47d02717f709a9ee39ac814f4ad4d816cadc21312e4d3ee616f9bcd8e331ae7.

### GREEN: `g532-import-scope-5284fd33-chirho`, tip 5284fd33

- The builder was handed by name in #28314. All six stages exited 0: build 15:39:56, focused 15:45:36, lint 15:45:43, oracle 15:46:22, mutants 15:47:34 and corpus 16:02:51 EDT.
- Hashes:
  - ledger: 0ce9eb6ca5642cb8edfbb53c8cd1eacb9b6352b818f701d561890eb44f61963d, 16 rows and all GREEN;
  - frozen CLI: aa7b290271f6a87b4b18780bfe99a4683255cb8bb3edca1e74be514d74a746d4;
  - C-locale run: e1ca05f256ca437498157ffdb676116eb9ea869f1de85c04d572b8eb8d288d13;
  - C-locale launcher: 2409adf38f95b533ad216e271d826343247032c1675c7fe02756e98250826ddc.
- Preflight covered the 9-commit chain and the 5 unit paths from 6ea3de75. GHC re-derived all 25 receipts, the judge selftest passed 31, and the 13 tools, the 8 immutable receipt sets, the base CLI and the 3 kept refs were all at their pins. The build had rc 0 and no warnings.
- Focused, each as its contract requires:

  | suite | passed |
  |---|---|
  | core-lib | 148 |
  | dictionary_evidence | 61 |
  | instance_obligations | 28 |
  | io_actions | 9 |
  | driver-filtered | 479 (1374 filtered) |
  | typing-lib | 354 |
  | typing_integration | 40 (the base's 35 and the 5 new tests) |

- Lint went through the diff map: driver short 375/375 and long 87/87, typing short 56/56 and long 0/0, with nothing on an inserted or changed line. Gitleaks scanned 9 commits and found no leaks.
- Oracle:
  - the 8 unbound uses, and J22 and A21, move from accepted to rejected for their named reasons;
  - J20, J21 and A20 hold;
  - Q01 and Q02 stay accepted and identically loud, which is the declared boundary;
  - the 7 imports stay accepted and identically loud at run;
  - the 3 shadowing runs print GHC's answer on both.
- Mutants:
  - m1 to m7 each terminated, and each was caught by its named control with the inventory among its new failures;
  - m6 named `Control.Lens.Type` and m7 named `Control.Lens.Tuple`;
  - the tree was restored to the baseline, with caught 7 of 7, and part_12 and the unit test file at their tip blobs.
- Corpus:
  - both passes byte-identical, fan-out 2, the frozen CLI unchanged across the run, and inputs identical to RC5's;
  - should_compile unchanged at 885/938;
  - should_fail moves exactly T12406 and T12589 from ACCEPT to REJECT, at 242/767 rejected against RC5's 240;
  - the replay gains exactly their 2 ordinary rejections, for 295 lines;
  - execution verdicts unchanged at 28/35.
- Postflight:
  - the frozen CLI is unchanged, and the tree is clean at 5284fd33 with part_12 at its tip blob;
  - the gate script and the 13 tools are at their pins, and all 8 immutable receipt sets and the base CLI are unchanged;
  - the 3 kept refs are in place, and no cargo, rustc, Clippy or CLI process was alive.
- Nothing was written to main or to the progress DB. The two committed GHC measurement artifacts are untouched; gpt_chirho updates both axes together after the fast-forward (#28317).
