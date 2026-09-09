<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Infix type binders — 2026-09-09

L.J. approved continuing with the confirmed infix type-variable defect and visible
type-argument ordering. Owner HASKELUJAH/gpt_chirho; baseline `8fb62c60`, isolated
`gpt-infix-type-binders-chirho` branch. Local, reversible compiler repair; no deployment.

## Placement and acceptance

Preserve type-variable identity when an infix type is lowered to ordinary type
application. Verify that signature quantification follows source binder order,
not the traversal order of the rewritten prefix application. Preserve explicit
forall order, seeded class variables and nested-scope exclusions in the inventory.
General kind-dependency ordering and converter shadowing are explicit boundaries.
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
- [x] Commit/push source checkpoint 18b7d1c3 by explicit owned paths; verify remote tip.
- [x] Run full workspace and both corpus axes twice; retain exact sets and provenance.
- [ ] Land tested source/evidence, close canonical progress row,
      and release builder/DB ownership.

## Evidence

- Baseline main CLI (explicitly rebuilt in the preceding lane, digest verified)
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
- tc192 now reaches the separate unsupported arrow-notation `proc`/`x` path; it remains
  a failure, not a claimed third gain. The July handoff already maps it as the sole
  `arrows_proc_notation_chirho` member: potential +1 when that feature lands, not an
  infix failure left unfixed. Full corpus results are recorded below.
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
- Same-name nested forall conversion has a two-use reduction:
  `preserveChirho :: a -> (forall a. a -> a) -> a`, returning its first argument.
  GHC prints `42/True` when it is called at Int and Bool; both baseline and this
  frozen CLI reject the second use as Int versus Bool. One use alone passed.
  The converter overwrites an outer name-map entry at the nested forall; a repair
  must restore bound names without discarding newly discovered free names. This
  is a queued root fix, not part of the ordering inventory's scope test.

## Pre-freeze results

- Parser: four namespace/fixity controls pass (three namespace controls and the
  fixity grouping control were each observed red before the corresponding fix).
- Typing: five signature-order/scope controls pass. Driver typing integration:
  12/12; two same-source GHC-checked execution controls pass on STG, LLVM and
  Cranelift. Canaries: 7/7. Formatting and diff whitespace checks pass.
- Detailed transient logs/probes:
  `/private/tmp/haskelujah-type-binders-chirho.rGrXvC`.

## Frozen gate results — 18b7d1c309a6297c04170658cc1a1f1a59aa3d4a

- Full workspace: **3331 passed, zero failed/ignored/filtered**, cargo exit 0,
  75 targets, `RUST_MIN_STACK=16777216`, `-j 3 -- --test-threads=4`.
  Driver 1773/1773, including 126 native round trips; curated 537/537 including
  514 compared execution oracles and 23 compile-only inputs. All 514 source hashes
  still match the preceding lane's GHC 9.14.1 reference; no claim of rerunning all
  514 GHC executions in this lane. The new controls were independently run under GHC.
- Explicit CLI build warning-free. SHA-256:
  `bcb77a970d37d1d34894162925ad2b7951ace8930d190878af50e31141bb2baa`.
  Owner wrapper asserted HEAD, clean tracked state and binary digest before/after
  each pass. Both axes ran twice with pure-shell detection, P4/15s and serial 60s
  timeout reruns: zero unresolved timeouts or unexpected exits.
- should_compile **877 -> 879 of 938**: gained T23764/tc156; zero losses.
  should_fail **222 of 767 held**: nothing gained or lost. Both complete verdict
  sets byte-identical across their two passes. No corpus input/oracle edits.
- Workspace log SHA-256:
  `70f21001d171804bd72a2a1b6a72a7037b5498baf9905638e323ff580fac4fc9`.
  The committed 75-target JSONL and execution measurement workflow retain scope
  and provenance; both upstream artifacts retain exact lists and paired labels.
- All-target clippy exits 0 but emits **535 warning messages** including repeats
  and summaries; no diagnostic locations in the four new child modules or extended
  typing integration file. Existing root-file warnings and structural debt remain;
  this is not the project's zero-warning gate. Formatting and whitespace checks pass.
- Read-only review found no blocker in either binder ordering or type-chain
  grouping. Imported fixity metadata, invalid mixed-fixity diagnostics and general
  traversal stack safety are not claimed. New child modules are 145–242 lines;
  their directories have 2 and 6 entries, and the oversized roots both shrink.
