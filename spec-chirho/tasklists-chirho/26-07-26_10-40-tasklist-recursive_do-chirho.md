*For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)*

# RecursiveDo slice — claude2_chirho

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
- Structural note: rec desugaring goes in a NEW module — `lower_chirho.rs` is 780K and
  `desugar_chirho.rs` is also large; per itty-bitty-scrollbars we do not grow either.

## Checklist

- [x] Diagnosis + mechanism pinned in source
- [ ] Brick-1 architecture: new module home for rec lowering (proposal:
      `crates/haskelujah-parser-chirho/src/rec_stmt_chirho.rs` for parse/AST +
      `crates/haskelujah-core-chirho/src/rec_desugar_chirho.rs` for the mfix knot);
      RecursiveDo added to extension plumbing (pattern: follow how RankNTypes is tracked)
- [ ] Lexer: `rec` contextual keyword + `mdo` keyword ONLY under RecursiveDo
      (fixes the mdo-as-varid latent bug as a regression test)
- [ ] Layout: `rec` opens a layout context (statement group like `do`/`of`/`let`)
- [ ] CST/AST: RecStmt node carrying the inner statement group
- [ ] Desugar: rec group → lazy-tuple mfix knot
      (`(xs, ys) <- mfix (\ ~(xs, ys) -> do { ...; return (xs, ys) })`);
      whole-group knot first, GHC-style minimal segmentation later if corpus needs it
- [ ] Typing: MonadFix class (`mfix :: (a -> m a) -> m a`) in class env; instances IO,
      Maybe, [] minimum; superclass Monad
- [ ] STG eval: `mfix`/`fixIO` knot-tying via result thunk (runtime laziness + refs exist)
- [ ] Tests: driver eval test (repro yields `[1,2,1,2]`-shaped knot output), mdo forward-ref
      eval test, mdo-as-varid-without-ext regression, ghc-tests T4404 recheck
- [ ] Gates before any claim: fresh debug build (mtime vs git log), cumulative full driver
      run, cargo fmt --check clean, zero warnings
- [ ] Land per-cluster with room announcement; log step in progress DB (single-writer window)

## Builder discipline

Implementation starts only after claude_chirho posts builder-free (their validity_chirho.rs
slice owns the toolchain per soundness-first priority). Until then: read-only prep only.
