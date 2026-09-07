<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Dictionary evidence — the checker tells the dictionary pass what it proved (2026-09-06)

Directive: L.J. — "get our GHC tests to all pass, see if there are deeper better thought out ways". Lane owner: claude_chirho (HASKELUJAH). Slot: claude2 holds it for A2b; this lane drafts without building and takes the slot at SLOT-FREE.

## Why (measured at dd6d694a and unchanged at fe815aaf)
The dictionary pass (`crates/haskelujah-core-chirho/src/dict_chirho/`) decides instances by looking at Core syntax — sibling arguments, literal markers, constructor names, binder types — and treats a binding's ground scheme predicates as proof. Where that guess is wrong or absent the program is wrong at run time, not at compile time:
- 48 of 537 curated GHC programs die with a missing `fromInteger`/`+`/`-`/`*`/`read` binding (signed `main`, literal under a user function; `spec-chirho/bug-dict-pass-literal-key-from-sibling-chirho.md`).
- `f :: Num a => a -> a; f x = x + 1; print (f 2.5)` hits the Int primop (the `+` inside `f` is dispatched to `Num Int` because a rigid-typed occurrence is defaulted to Int and the literal marker says Int).
- `g :: Fractional a => a -> a; print (g 5)` prints `$PAP g 5`: no dictionary is passed at the call site.
- `both :: (Describe a, Describe b) => ...; both True (3 :: Int)` fails with a wrong-tag case: only the first predicate's evidence survives the join (`join_occurrence_evidence_chirho` keeps one `(class, key)` per occurrence).
- Five driver `--lib` tests red at HEAD are in this class (`eval_show_nested_just`, `maybe_t_bind_success`, `type_app_nested_type`, `prelude_list_expanded`, `frontend_record_pattern_field_types_follow_constructor_layout`) — to be confirmed one by one at brick 3.
The rigid-variable lane made the leak visible (trap 17) and restored the old contract; this lane replaces the contract.

## Design (the classical translation, driven by the checker's evidence)
The checker already instantiates every constrained reference and solves its predicates. It records, per reference, the solved instance of each predicate, and per literal the solved `Num`/`Fractional`/`IsString` instance; the dictionary pass consumes those records and stops guessing for anything that has one. Keys are source spans (unique per token; generated code with dummy spans falls back to today's paths), not per-name ordinals, so inference order never has to match desugar order.

An evidence record is one of: a concrete instance head key (`Int`, `[]`, `Maybe`, ...), or "the enclosing binding's own dictionary parameter for class C" (the occurrence's type is a rigid variable of the signature, or a variable the binding generalizes). The pass builds `$sel_C_m dict` / `$prim_C_m_Key` from the first and `$sel_C_m $dC` from the second; a dictionary-parameterised function reference gets one dictionary argument per predicate, in the scheme's predicate order.

Placement (the decision to surface at brick 1): three crates, one seam each.
- typing: `infer_chirho.rs` capture sites (both `Var` arms) and the literal arm gain span-keyed captures; a new child module `infer_chirho/evidence_chirho.rs` owns the record type, the capture helpers and the finalize (moved out of the big file, not added to it). `InferResultChirho` gains `evidence_chirho: HashMap<SpanChirho, Vec<EvidenceChirho>>`.
- core: `desugar_chirho.rs` records `occurrence id -> span` for literals and for every reference to a name the driver marks as constrained (the opted-in occurrence-name set becomes standard methods ∪ names whose scheme carries predicates); `dict_chirho/rewrite_chirho.rs` consumes evidence at method occurrences and at dictionary-parameter call sites, before any heuristic.
- driver: `lib.rs` join becomes span-first, ordinal-fallback.

## Bricks (each gated: `cargo test -p haskelujah-typing`, `cargo test -p haskelujah-driver --lib` (skip the Cranelift runaway), the driver integration suites, the curated suite count, then both corpus axes two passes at the landing)
- [x] 1. Tasklist + this design surfaced to L.J. (this file)
- [x] 2. Literal evidence (worktree, 2026-09-06; the literal wrapper keeps the selector form — the prim row broke Cranelift's `Box 42`): span-keyed `Num`/`Fractional`/`IsString` records; desugar span table for literal occurrences; join; pass consumes for `fromInteger`/`fromRational`/`fromString` occurrences. Target: the 48 curated failures and the bug note's program.
- [x] 3. Rigid occurrences record "own parameter" evidence, indexed by the signature predicate (`$own:k`): `both :: (Describe a, Describe b)` printed `True/3` while the key was class-only, because the pass keeps one parameter per class name. Generalized variables get the class-only form through reference evidence instead of defaulting to Int; the pass dispatches those through the binding's dictionary parameter. Target: `f :: Num a => a -> a` at Double.
- [x] 4. Constrained user functions (top-level, class methods excluded — they stay on the method mechanism; occurrence ids minted at the variable arm; the higher-order-argument rewriter hands an evidenced reference to the variable arm; every table keyed by a binding id sees through an occurrence id — the dictionary-parameter lookups AND the module-level call-site parameter scan, whose miss cost `T338_variance` and the sieve test their `*`/`==` dictionaries): per-occurrence ids for their references; all predicates per occurrence in order; dictionary arguments at call sites from evidence. Target: `g 5`, `both True 3`.
- [~] 5. The five driver reds, triaged: `prelude_list_expanded` — fixed by brick 2. `eval_show_nested_just` — not dispatch: `show (Just (Just 42))` prints `Just Just 42` on the untouched commit and here alike, the runtime `Show Maybe` row does not parenthesise a constructor argument (`showsPrec` precedence; runtime crate). Not this class, left red with their root causes named: `maybe_t_bind_success` (an argument-less `comp = bindMaybeT ...` is IO-defaulted at generalization instead of staying monomorphic for its use site to fix, the monomorphism-restriction shape; checker), `type_app_nested_type` (`myLength @[Int]` on an inferred type — GHC rejects a visible type application on inferred variables, the test pins a non-GHC verdict), `frontend_record_pattern_field_types_follow_constructor_layout` (a record pattern's field types are taken from the wrong constructor when two constructors share a field name; checker).
- [ ] 6. Retire the heuristics the evidence makes redundant only where a gate proves it (design-evidence-threading P4 list); keep a loud residual for occurrences without evidence.
- [x] 7. Landing (948395b0, 2026-09-06): both axes two passes byte-identical and identical to the committed lists (874 of 938, 205 of 767 — this lane moves what runs, not what type-checks); artifact headers superseded; DB row 470; SLOT-FREE posted.

## Found on the way (not fixed by this lane unless a brick says so)
- Own-parameter evidence reaches only the methods in the pass's standard occurrence list; a user class method at a rigid variable (`describe x` inside `both :: (Describe a, Describe b) => ...`) is dispatched by the call-site parameter keys the pass infers, which is right here but is still a guess. Giving every class method a span-keyed occurrence needs the method paths that key on canonical ids to see through occurrence ids first (minting them as references broke `desc`/`mempty`/`mconcat`).
- A probe that "runs" is not a probe that is right: `both True 3` ran and printed `True/3`, which is the Int instance applied to a Bool. Every interpreter-level test in this lane asserts the exact output GHC produces.
- (done in brick 8) `T039_fib_acc`: local `where` bindings generalized over a class.
- `is_defaultable_pred_chirho` (dict pass) does not look inside tuple or list argument types, so `addPair (x, y) = x + y` is monomorphised to `Int` instead of taking a `Num` dictionary (`T229_curry_uncurry`); a call at Double would be wrong silently.

## Boundaries
- Not touching: the parser and kind layer (claude2, A2b), the naming crate (gpt), the big-file split (needs L.J.).
- No heuristic is widened to pass a test; a program without evidence keeps today's path and today's verdict.
