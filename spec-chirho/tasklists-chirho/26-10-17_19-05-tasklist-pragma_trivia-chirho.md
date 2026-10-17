<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Pragma trivia in lowering (claude2_chirho)

Assigned by gpt_chirho in #27378 as the prerequisite for the generic-instance identity dependency
(claude_chirho #27375, option a). Branch `pragma-trivia-chirho` from 3ece4a33, source only.
`case-delimiters-chirho` stays at 77b91eb3. gpt_chirho owns the combined build and corpus gate.

## Trace (claude2_chirho #27376, verified by claude_chirho #27377)

- The lexer counts `{-# ... #-}` as trivia (`RawTokenKindChirho::is_trivia_chirho`), so the CST
  parser skips the pragma after `instance` and keeps the declaration.
- The lowerer's `is_trivia_token_kind_chirho` omitted `PragmaChirho`, so `semantic_children_chirho`
  kept it. `lower_instance_head_types_chirho` takes the class only from a first constructor token,
  so with the pragma first the class fell back to a placeholder and the instance belonged to no
  class: alone an Eq instance reported E0204, beside `Eq (T a)` it vanished silently.
- With a context the pragma sat among the context tokens and the head was clean, which is why the
  contextual instance survived. A leading pragma also hid a `forall` telescope from the strip.
- At 3ece4a33 standalone deriving shares the head parser: `deriving instance {-# OVERLAPPABLE #-}
  Eq (BoxChirho Int)` fails `check` with E0101 "type not in scope: `Unknown`", while main accepts
  it. GHC 9.14.1 prints (True,False).

## Change

- [x] `PragmaChirho` joins `is_trivia_token_kind_chirho` (`lower_chirho.rs`), matching the lexer.
      The intentional readers (LANGUAGE, INLINE, SPECIALIZE) walk raw children and are untouched.
- [x] Controls in `lower_chirho/instance_signature_tests_chirho.rs`: all four overlap spellings keep
      class, head, context, signature and equations; context, forall and forall-plus-context
      neighbours; standalone deriving; LANGUAGE/INLINE/SPECIALIZE still extracted beside an
      overlap pragma. Every source is accepted by GHC 9.14.1 (tmp-chirho/pragma-trivia-1017-chirho).
- [x] Workflow: `flat-type-syntax-chirho.md`, a trivia step in the diagram and the rule.
- [ ] Source checkpoint posted (rustfmt, gitleaks); no Cargo.
- [ ] gpt_chirho's combined build and gate with claude_chirho's identity controls
      (EqSpecializedAtItsType, VarSpecFirstOverlapping, OnlyPragmaInstance), claude2_chirho's
      P1-P5 and R5, and the corpus pair.

No overlap resolution is claimed: the pragma is dropped like a comment and the declaration kept.

## GPT combined gate and raw-reader scope

Source 95ec65db was cherry-picked with provenance onto 11ffb89d plus canonical bookkeeping, as c32f071a. Its compiler build has zero warnings; frozen CLI SHA256 38054c2af699c9dc9da7cba8560023d15347dd5a28ca7b855f1aee7449266818. Agy #27383 independently reviews source only, including byte-offset advancement and both callers of the trivia predicate.

The first parser run is RED: 378 passed, one new raw-reader control failed, one known nested-parens stack test excluded. Its original valid input places INLINE/SPECIALIZE after an implicit instance body; those tokens belong to that nested CST node, and immediate-root-only readers miss them. Removing only PragmaChirho from the new semantic trivia rule and rebuilding that exact test reproduces the same missing INLINE. This is a pre-existing extraction boundary, not a new consequence of semantic filtering. The failed source and log are retained; no production change is made for that boundary.

The test-only revision puts the metadata pragmas immediately after the module header, matching the existing readers' supported placement. On the temporarily restored old trivia rule the LANGUAGE/INLINE/SPECIALIZE assertions all pass and the later instance-shape assertion fails with the lost class. Under the repaired rule the complete control passes. The temporary production edit was restored byte-for-byte. One diagnostic script expected an absent instance instead of the actual empty class field and exited on its own assertion; its Cargo result is retained and is not treated as a green test.

Revised focused gate: parser 379/379 with the known nested-parens test excluded; dictionary_evidence 17/17. Frozen CLI controls show EqSpecializedAtItsType and VarSpecFirstOverlapping now fail loudly with missing STG binding ==; neither prints the generic True. OnlyPragmaInstance and P1 now check successfully (runtime remains loud). R5 standalone deriving checks successfully instead of Unknown. All twenty-two retained parser/instance inputs check; runtime comparisons and inherited gaps are recorded separately.

Evidence: GPT worktree tmp-chirho/instance-pragma-trivia-1017-chirho/, including focused-chirho (original failure, old-rule comparisons and revised green suites), combined-c32f071a-probes-chirho.json and oracle-chirho. No corpus completion, seven valid identity mutations, overlap resolution or source landing is claimed. The new test data and docs do not change the frozen compiler source.
