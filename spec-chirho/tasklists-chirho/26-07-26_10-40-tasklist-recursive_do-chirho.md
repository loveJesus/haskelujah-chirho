*For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)*

# RecursiveDo slice — gpt_chirho

Lane claimed in room `haskelujah-chirho` (msgs #7720/#7727); soundness-neutral under L.J.'s
soundness-first ruling (adds acceptance of GHC-accepted programs; rejection behavior untouched).

## Diagnosis (done, 2026-07-26)

Repro: `rec` bind-group in a do-block (scratchpad `rec-repro-chirho.hs`) → 4 diagnostics:
2× `unbound variable: ``' (EMPTY name, SYNTHETIC 0..0 span) + 2× `ys` unbound at real spans.

Mechanism, pinned in current source:

- `"rec"` appears in ZERO `.rs` files — the compiler has no knowledge of the keyword.
  Layout never opens a context at `rec`, so the block's statements collapse into one
  mangled statement; lowering's synthetic-span fallback then emits empty-name placeholder
  vars (`lower_chirho.rs:16046` area) → the `` `` `` diagnostics.
- `mdo` IS lexed but as an unconditional alias of plain `do`
  (`crates/haskelujah-parser-chirho/src/lexer_chirho.rs:1333`). Two bugs in one:
  (a) mdo-blocks get NO recursive scoping — any forward reference errors as unbound;
  (b) `mdo` should be a keyword ONLY under RecursiveDo — vanilla Haskell allows it as a
  plain varid, so we currently REJECT valid Haskell (a should_compile-class latent bug).
- `mfix`/`MonadFix` are name-seeded only (`crates/haskelujah-naming-chirho/src/iface_chirho.rs:2020-2022`);
  no class decl, no instances, no evaluator support.
- Diagnosis ran on a scratchpad COPY of the 3-week-stale July 5 debug binary (stale-binary
  trap acknowledged): acceptable for locating mechanism in current source since no
  intervening commit touched rec/mdo/mfix paths; ALL behavioral claims re-verified on a
  fresh build before landing.
- Structural note: rec desugaring goes in a NEW module — `lower_chirho.rs` is 17,305 lines
  and `desugar_chirho.rs` 6,861 (vs the 1.5k-line rule); per itty-bitty-scrollbars we do
  not grow either. (Sizes in LINES per room convention — bytes/lines side by side caused a
  45x misread once already.)

## Checklist

- [x] Diagnosis + mechanism pinned in source
- [x] Brick-1 architecture: new module home for rec lowering:
      `crates/haskelujah-parser-chirho/src/rec_stmt_chirho.rs` for CST-to-AST transformation +
      `crates/haskelujah-core-chirho/src/rec_desugar_chirho.rs` for genuinely lazy
      tuple projections around the mfix knot. Keep RecStmt in the CST only; lower it to
      ordinary AST do statements before typing so downstream phases do not gain a
      RecursiveDo-specific variant. RecursiveDo is classified between raw lexing and layout
      from actual pragma tokens, using the same pragma parser as CST lowering.
- [x] Lexer: `rec` contextual keyword + `mdo` keyword ONLY under RecursiveDo
      (fixes the mdo-as-varid latent bug as a regression test)
- [x] Layout: `rec` opens a layout context (statement group like `do`/`of`/`let`)
- [x] CST: RecStmt node carrying the inner statement group; AST transformation remains next
- [x] Desugar: rec group → lazy-tuple mfix knot
      (`(xs, ys) <- mfix (\ ~(xs, ys) -> do { ...; return (xs, ys) })`);
      Core lowering represents every tuple field as a delayed selector thunk. `mdo` blocks
      with no forward/self dependency remain sequential; dependency-bearing prefixes use
      the knot.
- [x] Typing: MonadFix class (`mfix :: (a -> m a) -> m a`) in class env; instances IO,
      Maybe, [] minimum; superclass Monad
- [x] STG eval: generated IO/Maybe/list `mfix` bodies tie Core letrec knots; ReturnIO preserves
      payload thunks, and the driver forces only the final observable result
- [x] Tests: driver eval test (repro yields `[1,2,1,2]`-shaped knot output), mdo forward-ref
      eval test, mdo-as-varid-without-ext regression, ghc-tests T4404 recheck
- [x] Gates before final semantic claim: fresh debug build (mtime vs git log), focused suites
      during iteration, bounded eval suite and direct CLI checks at the landing boundary,
      owned new/modified modules rustfmt-check clean, zero new warnings; full driver run only
      under the shared-machine resource guard.
      Verified: RecursiveDo driver 4/4, runtime 52/52, typing 257/257 (+1 ignored), Core
      125/125, bounded driver eval 990/990, T4404 accepts, and fresh CLI mdo prints `42`.
      The historical 70-case scratch probe was no longer present, so no probe result is
      claimed. Full parser remains 279 pass / 2 unrelated fixture failures / 3 ignored:
      Primitive.ByteArray token-position drift and external containers CPP input.
- [x] Land the completed semantic slice in one explicit-path commit and announce it in the room
- [ ] Log the step in progress DB after the existing shared dirty writer releases it

## Insertion points (mapped read-only, 2026-07-26 ~10:40)

- Extension plumbing is STRING-based, not an enum: pragma walk extracts names during CST
  lowering (`crates/haskelujah-parser-chirho/src/lower_chirho.rs` ~:160-181), known-extension
  whitelist near the end of the file (add "RecursiveDo").
- The pragma keyword match is now CASE-INSENSITIVE (`eq_ignore_ascii_case("LANGUAGE")`,
  ~`:181`, claude_chirho's fix 2026-07-26 — mixed-case `{-# Language ... #-}` previously
  dropped EVERY declared extension). CONSTRAINT CORRECTED (2026-07-27, per claude_chirho's
  #8590 flag — my original wording was impossible as written): the pre-scan cannot CALL
  `extract_pragma_extensions_chirho`, which walks a CST that does not yet exist at pre-scan
  time. The binding form of the constraint is the INTENT: extract the shared
  "pragma inner text → extension names" logic into ONE function, and have BOTH the CST walk
  and the raw-source pre-scan call it — one source of truth, semantics cannot fork.
  (Slice now gpt's per the #8590 lane board; a ~90-line starter with 9 tests sits in
  claude_chirho's scratchpad as pragma_scan_chirho.rs.proposal, gpt's to adopt or bin.)
- ARCHITECTURAL FINDING (brick 1): extensions previously became visible only AT LOWERING, but
  `rec`/`mdo` keyword-ness is a LEXER decision and `rec`-opens-a-block is a LAYOUT decision —
  both run before lowering. This ordering gap is precisely why `mdo` was hardcoded as an
  unconditional keyword. Landed fix: the raw lexer keeps both words as VarId, then a pass over
  actual `PragmaChirho` tokens calls the shared pragma parser and reclassifies them before
  layout. This is safer than scanning raw source text: strings and ordinary comments cannot
  false-enable RecursiveDo.
- Lexer keyword site: `crates/haskelujah-parser-chirho/src/lexer_chirho.rs:1333`
  (`"do" | "mdo" => DoChirho`) — split; `rec` contextual, `mdo` gated on RecursiveDo.
- Layout: keyword-class match ~`layout_chirho.rs:441` (Of/Where class), after-keyword state
  flags ~`:520-530`, do-position logic `:848` — `rec` joins the block-opening class.
- CST: `parse_do_stmt_chirho` at `cst_parser_chirho.rs:3404` gains the RecStmt arm;
  `parse_do_expr_chirho` at `:3355` is the mdo entry.
- Naming seeds to upgrade: `crates/haskelujah-naming-chirho/src/iface_chirho.rs:2020-2022`
  (mfix value + MonadFix class placeholder → real class scheme).
- Shared-tree caution: claude_chirho's uncommitted `validity_chirho.rs` also consumes the
  extension string set — the pre-scan must feed the SAME set, not fork a second source of
  truth, and must not disturb their read path.

## Builder discipline

gpt_chirho owns the RecursiveDo builder lane from room message #8609. Every cargo invocation
uses `CARGO_BUILD_JOBS=2`, targeted tests run sequentially, and no full driver/proptest sweep
runs without an explicit shared-machine landing boundary.
