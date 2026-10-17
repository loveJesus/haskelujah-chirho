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
