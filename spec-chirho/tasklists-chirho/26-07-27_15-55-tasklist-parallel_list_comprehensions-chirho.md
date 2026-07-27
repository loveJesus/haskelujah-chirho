<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Parallel list comprehensions tasklist

Owner: `gpt_chirho`

## Brick 1

- [x] Reproduce deterministic `T11982a.hs` failure: the parser stops at the second top-level
      `|`, so the second branch binder is absent from the lowered AST.
- [x] Represent regular and parallel qualifier groups without flattening zip semantics into
      sequential list-comprehension semantics.
- [x] Preserve one source of truth for qualifier parsing and lowering.

## Semantics

- [x] Infer each parallel branch in an isolated lexical scope, then expose all branch result
      bindings to the comprehension body.
- [x] Desugar parallel branches to lockstep zip semantics rather than a Cartesian product.
- [x] Keep ordinary list comprehensions behaviorally unchanged.

## Gates

- [x] Add parser/lowering, typing, and Core desugaring regressions.
- [x] Make GHC `T11982a.hs` pass.
- [x] Execute a runtime case that distinguishes zip from Cartesian semantics.
- [x] Run bounded parser/typing/Core suites, owned-file formatting, and `git diff --check`.
- [ ] Commit explicit owned paths, push `main_chirho`, and release the builder.

## Gate notes

- The exact lowerer regression, the `T11982a.hs` CLI check, 15 ordinary/parallel
  list-comprehension driver tests, 260 typing tests, and 127 Core tests pass.
- The broad parser suite reaches this slice's regression successfully, then remains red on an
  external CPP fixture failure, an unrelated MagicHash lexer pin, and the existing nested-parens
  proptest stack overflow. The isolated `containers` test fails in CPP before parsing
  (`Internal.hs:564: invalid preprocessing directive`).
- Rustfmt reports only pre-existing drift in older parser tests/comments outside the changed
  hunks; all new hunks and `git diff --check` are clean.
