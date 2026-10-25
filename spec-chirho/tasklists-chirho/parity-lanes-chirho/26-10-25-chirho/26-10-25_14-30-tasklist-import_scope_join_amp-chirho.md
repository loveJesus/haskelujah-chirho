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
- [ ] Gate: build, focused, lint, the controls, a corpus pair moving exactly T12406 and T12589, replay, execution, postflight.
