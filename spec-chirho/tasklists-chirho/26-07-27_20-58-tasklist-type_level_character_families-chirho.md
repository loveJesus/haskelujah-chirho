<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Type-level character families tasklist

Owner: `gpt_chirho` -> transferred to `claude2_chirho` at #9270 (Codex stopped for credits);
finished, verified, and landed by claude2_chirho.

## Brick 1

- [x] Reproduce `T14934a.hs`: `CharToNat '\1'` remains stuck against `1`.
- [x] Reproduce `T19535.hs`: `CharToNat 'a'` and `NatToChar 120` remain stuck in equality
      evidence.
- [x] Confirm parsing, naming, kind inference, visible type application, and equality evidence
      already reach the family reducer.

## Implementation

- [x] Centralize Haskell character escape parsing/rendering for term and type literals.
- [x] Register `CharToNat` and `NatToChar` as exported unary TypeLits families.
- [x] Reduce concrete character-to-codepoint and valid codepoint-to-character applications.
- [x] Leave variable and invalid-codepoint applications stuck rather than fabricating evidence.
- [x] Pin both GHC source files end to end and add focused reducer/metadata tests.

## Gates

- [x] Keep the latest VTA surface (`Vta1`, `Vta2`, `T17594f`, `T12734a`) green.
- [x] Run focused AST/parser/naming/kind/typing/driver tests and bounded full crate suites.
- [x] Run a warning-free `-j2` CLI build, formatting checks, and `git diff --check`.
- [x] Commit explicit owned paths, push `main_chirho`, release the builder, and request
      deterministic two-axis corpus remeasurement.

## Verification at landing (claude2_chirho, 2026-07-27 ~21:15)

- T14934a + T19535: PASS on fresh debug binary (previously stuck baselines resolved).
- VTA sentinels Vta1/Vta2/T17594f/T12734a: all PASS.
- Focused driver filter `type_level_character_families_chirho`: 2/2.
- Bounded suites: ast/naming/typing/core 485 passed / 1 ignored; parser deterministic
  surface 282 passed with ONLY the two disclosed pre-existing fixture reds (preprocessed
  primitive lexer assertion, containers-intset preprocessed lowering) and the known
  nested-parens proptest stack overflow excluded — all pre-dating this slice (#9210).
- CLI build warning-free; cargo fmt --all --check clean.
- Diff reviewed by the landing agent (shared char parse/render helpers as single source
  of truth; stuck-not-fabricated for invalid codepoints; workflow comments in place).
