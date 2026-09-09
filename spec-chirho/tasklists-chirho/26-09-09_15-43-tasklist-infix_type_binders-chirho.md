<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Infix type binders — 2026-09-09

L.J. approved continuing with the confirmed infix type-variable defect and visible
type-argument ordering. Owner HASKELUJAH/gpt_chirho; baseline `8fb62c60`, isolated
`gpt-infix-type-binders-chirho` branch. Local, reversible compiler repair; no deployment.

## Placement and acceptance

Preserve type-variable identity when an infix type is lowered to ordinary type
application. Verify that signature quantification follows source binder order,
not the traversal order of the rewritten prefix application. Preserve explicit
forall order, seeded class variables, nested forall scopes and kind dependencies.
The oversized parser/inference roots get small hooks; substantial mechanisms and
tests belong in focused modules. A read-only review covers the typing paths while
the lead owns edits/builds. No AST redesign for data-family/type-data/GADT records
is included in this brick.

Required evidence: AST namespace controls, positive and negative typechecking,
an independently specified observable execution using visible applications, the
complete workspace gate, and two full passes per upstream corpus axis on a freshly
built hashed CLI. Every verdict delta must be explained; a right verdict from an
unrelated diagnostic is not a demonstrated repair. Existing clippy/structural debt
remains explicit and must not be represented as a zero-warning gate.

## Checklist

- [x] Verify main/worktree state, prior gate evidence and current failing candidates.
- [x] Reproduce namespace and binder-order controls on the baseline; verify GHC behavior.
- [x] Extract focused type-operator lowering; share the application constructor with the flat symbol path.
- [x] Repair source-order binder inventory after the reduced control proved the defect.
- [x] Resolve the independently reduced type-fixity defect with linear chain reduction.
- [x] Run focused positive/negative and execution controls; update the workflow.
- [ ] Run full workspace and both corpus axes twice; retain exact sets and provenance.
- [ ] Commit/push owned paths, land tested source, close canonical progress row,
      and release builder/DB ownership.

## Evidence

- Current main CLI (explicitly rebuilt in the preceding lane, digest verified)
  rejects `T23764.hs` at `op` and `tc156.hs` at `b`, both E0101. Source inspection
  confirms that InfixTypeChirho admits VarId but constructs ConChirho unconditionally.
  The flat-child helper recognizes symbols only, so its constructor namespace is
  correct for its current grammar; it now shares the binary application builder.
- Namespace-only repair: all three AST controls turn red-to-green, including qualified
  constructors. T23764 then fails Int versus `(,)`, confirming the second boundary.
  GHC 9.14.1 prints `42/7/11/True` for the infix/context/explicit/prefix control;
  after scheme-order repair, our interpreter and both native engines match.
- tc156 then exposed type precedence: `(Int :*: Bool) :+: Char` passes while the
  unparenthesized form fails. The parser grouping control goes red on that defect;
  linear type-chain reduction uses the existing fixity table, without changing
  expression lowering or the data-head scanner. tc156 now typechecks.
- tc192 now reaches the separate unsupported arrow-notation `proc` path; it remains
  a failure, not a claimed third gain. No corpus totals measured on this lane yet.
- Read-only independent review found no blocker in scheme ordering. The existing
  quantification set, class-variable prefix and scoped/skolem filters are preserved.
  Full kind-dependency ordering and AST-converter same-name forall shadowing are
  explicit boundaries in the workflow, not repairs claimed here.

## Found on the way (reproduced on main's baseline CLI)

- A polymorphic custom `keepChirho @Int @Bool` class method typechecks but execution
  reports missing STG binding `keepChirho`; GHC prints True. The new scope control
  asserts only typechecking and says so. Separate dispatch repair remains open.
- Symbolic value constructor `:+:` reports a missing STG binding even with an
  explicitly parenthesized type on baseline. The precedence execution control uses
  named value constructors with the same infix type constructors; GHC prints
  `3/True/'c'`, and baseline still rejects it specifically for type precedence.

## Pre-freeze results

- Parser: four namespace/fixity controls pass (three namespace controls and the
  fixity grouping control were each observed red before the corresponding fix).
- Typing: five signature-order/scope controls pass. Driver typing integration:
  12/12; two same-source GHC-checked execution controls pass on STG, LLVM and
  Cranelift. Canaries: 7/7. Formatting and diff whitespace checks pass.
- Full workspace, corpus totals and final clippy attribution remain pending.
- Detailed transient logs/probes:
  `/private/tmp/haskelujah-type-binders-chirho.rGrXvC`.
