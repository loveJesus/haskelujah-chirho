<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Qualified do tasklist

Owner: `gpt_chirho`

## Brick 1

- [x] Reproduce deterministic `T21206.hs` failure and pin the current diagnostic.
- [x] Preserve an optional module qualifier on `do` expressions through layout, CST parsing,
      AST lowering, and every AST consumer.
- [x] Keep ordinary `do` and `RecursiveDo` behavior unchanged.

## Semantics

- [x] Resolve self-qualified `>>=`, `>>`, and `fail` methods from the recorded module qualifier.
- [x] Bridge qualified Prelude aliases through the current bare-export linker.
- [x] Execute a custom qualified-do runtime case whose result proves the qualified bind path ran.
- [x] Reject or loudly report unresolved qualified methods rather than silently using Prelude.
- [ ] Follow up in the linker: distinguish module-qualified definitions so custom imported
      QualifiedDo operator modules can link without bare-name collisions.

## Gates

- [x] Add focused layout/parser/lowering, typing, Core, and driver regressions.
- [x] Make GHC `T21206.hs` pass.
- [x] Run bounded ordinary-do and RecursiveDo regressions.
- [x] Run bounded parser/typing/Core suites, owned-file formatting, and `git diff --check`.
- [x] Commit explicit owned paths, push `main_chirho`, and release the builder.

## Gate record

- QualifiedDo parser/lowering focused gates: 5/5.
- RecursiveDo parser gates: 7/7.
- QualifiedDo driver gates: 5/5.
- Ordinary-do and IO-do focused driver gates: 2/2.
- Core: 128/128; typing: 259 passed, 1 ignored; AST: 5/5; TH: 15/15.
- GHC `T21206.hs`: accepted by a freshly built debug CLI.
- GHC `T17594f.hs`: advanced through QualifiedDo and stopped at a separate required-type-
  application mismatch on line 28; it is not claimed as passing.
- Broad parser library run excluding proptests: 278 passed, 3 ignored, 2 known external-CPP/
  MagicHash fixture failures unrelated to this slice.
- `rustfmt` is clean for the touched QualifiedDo regions. Two pre-existing drift regions remain
  elsewhere in the large CST/lowerer files and were deliberately not rewritten in this landing.
