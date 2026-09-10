<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Flat type syntax — 2026-09-10

Continued overnight direction from L.J.; HASKELUJAH/gpt_chirho owns this
isolated branch, gpt-flat-types-chirho, based on frozen kind-scope checkpoint
3683dae8. Canonical progress row 482 started 2026-09-10T06:59:34.746Z.
The preceding kind-scope workspace gate continues in its separate worktree;
no source edits or extra heavy build load enter that gate.

## Brick 1: representation, placement, and proof

Reproduced with GHC 9.14.1 and the frozen CLI: a record field `[] Char` is
valid and reads back `kept` under GHC, but our checker rejects E0300. Bare
`[]` in a record field is invalid (GHC-83865) but our checker accepts it.
The flat-token path fabricates a list application with a placeholder element
for an empty pair of brackets. It must retain the existing AST distinction:
the constructor `[]` versus an applied list `[a]`. No new AST shape or
kind-checker exception is needed.

Something is wrong with the oversized lowering root. Extract its contiguous
flat-type reconstruction helpers to lower_chirho/flat_types_chirho.rs; keep
focused tests in flat_type_tests_chirho.rs. This is a routine, reversible
placement improvement, not the deferred broad parser split. New code must
stay bounded and use existing type representations. The remaining oversized
root is still quality debt, not silently counted as compliant.

Preserve nested delimiter ownership: the old list scanner stops at the first
closing bracket, even for nested lists. Before changing that path, establish
controls where a type after the inner closing bracket must remain outside it.
Do not claim full flat forall/type-operator semantics from this list repair.

## Acceptance

- [x] Reproduce valid applied and invalid unsaturated list controls with GHC/current CLI.
- [x] Capture parser AST controls and exact-output execution controls, including nesting.
- [x] Extract helpers and repair list constructor/application and bracket ownership.
- [x] Focused parser, typing, driver, and canary gates; check T14761c for actual reachability.
- [ ] Rebase onto the preceding verified landing, freeze, and run both full upstream axes twice.
- [ ] Complete meaningful workspace/execution gates, named-path commits, push/read-back,
      main-path smoke checks, and canonical DB closure.

## Prepared controls and audit

GHC 9.14.1 independently runs the three execution sources without warnings:
prefix-list record readback `kept`; nested list-of-functions readback `42`;
GADT record combining both `kept/42`. The old CLI rejects the first two.
Ordinary nested lists `[[Char]]` are a passing control before the repair.
After the preceding workspace gate completed, the repaired full parser suite
passed 328/328 with zero ignored or filtered tests. The five new parser controls
were each demonstrated red with the old scanner restored, then the repaired
scanner was restored and the full suite rerun. Driver typing integration passes
27/27 (including the three new identical-source STG/LLVM/Cranelift executions
and ordinary/GADT unsaturated/overapplied negative controls), canaries 7/7.

The independent review caught a defect in the initial AST-shape formatter:
both Con("[]") and List(Var("")) printed `[]`, so it could hide the exact
fabrication being repaired. Variables now render with an explicit `var(...)`
tag. The old placeholder therefore differs from a real list constructor.
This test-instrument repair is included in the demonstrated five baseline reds.

Full typing passes 335/335. T14761c now checks under the explicitly rebuilt CLI;
the previous main CLI still reports E0300 at its `[] Char` field, and fresh GHC
9.14.1 accepts the unchanged upstream file with its own -Werror option. This
removes a spurious kind error, not a claim that every UNPACK/StrictData warning
contract is implemented. Full-corpus movement remains unmeasured at this point.
Three clippy findings in the extracted existing helper were corrected without
changing its forall behavior; unrelated lint/size debt is not waived.

## Other measured candidate, not implemented

The frozen compiler also mishandles `deriving newtype`: GHC's numeric example
prints `7`, ours lacks Num CounterChirho; GHC's paired stock/newtype Show
example prints `StockChirho 7` then `7`, ours reports a bogus Show arity error.
The parser tests a reserved Newtype token as if it were a VarId, and the AST
deriving lists retain only class names. A token-only fix would still discard
the strategy and risk the wrong Show behavior. A coherent deriving-strategy
representation/consumer repair remains separate work, not a claimed gain here.

## Additional reductions, separate from this repair

- Inline result-kind tails and attached complete standalone signatures share
  kind_sig_chirho today. A polymorphic constructor in
  `data TestChirho (aChirho :: kChirho) (bChirho :: kChirho) :: kChirho -> Type`
  has type `TestChirho aChirho bChirho aChirho`; specializing it at main prints
  42 under GHC 9.14.1, while the frozen CLI rejects E0300. The complete standalone
  signature control works in both. Do not prepend head parameters indiscriminately:
  the AST needs to distinguish a result-kind tail from a complete signature.
  An initial concrete-constructor reduction was GHC-invalid under its default
  language; unchanged T11811, which explicitly selects Haskell2010, is GHC-valid.
- `data HolderChirho = HolderChirho { identityChirho :: forall aChirho. aChirho -> aChirho }`
  followed by `badChirho = HolderChirho (\_ -> True)` is rejected by GHC-25897
  but accepted by frozen 3683dae8. The equivalent positional constructor is also
  wrongly accepted. This is not isolated to the flat parser, so the callback-checking
  cause remains unproven. The genuine identity callback prints 42/True in both engines.
  The list repair must not claim to repair that broader rank-N constructor boundary.
