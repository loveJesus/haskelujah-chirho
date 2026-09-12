<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Local open-family rows in kinds, row484

Resume boundary: pushed9801f3e8, local tag open-kind-rows-before-chirho.
Main121d4f2c, canonical row484 and public artifacts remain unchanged. Last
diagnostic eb809e70:882/938 accept,249/767 reject; thirteen main-relative
accept regressions remain. SLOT/DB retained; no corpus CPU embargo now.

## Brick 1: ownership and representation

The open equation checker currently discards the checked row. Dependency groups
also omit its references, so F's result kind can consume G before G's instance
exists. T12381 reduces this to two local families and two equations. T11348
additionally needs an implicit kind input matched before the visible parameter.

Recommendation: attach each direct local open row and its syntax dependencies
to its existing family owner before grouping; publish checked, closed rows
after the owner's kind contract is finalized, before later consumers. Rows of
imported or associated families need separate authority and stay outside this
registration step. A recursive group that cannot establish the row's kinds is
not permission to publish speculative equations.

Open rows match hidden and visible inputs in one substitution. Kind terms must
carry solved occurrence indices; an explicit @ fills the corresponding slot,
not a second application. Closed-family visible-only rows keep their existing
proof that omitted hidden arguments impose no conditions. Preserve that proof
until closed indexed rows are represented; never apply an open-row policy to
closed source ordering or trust open-family inverse improvement.

Open overlap is not closed first-match order. Add an explicitly validated
unordered equation collection beside the shared FamilyTerm matcher. Validate
pairwise compatibility using possibly-infinite LHS unification and compare RHS
terms under that graph WITHOUT unifying the RHS into agreement. Variables are
row-local. Opaque terms/resource exhaustion mean unproved, not apart or valid.
Work is bounded per collection, including expanded substitutions; no global
map clone per use. New helpers live in existing kind and types family modules,
with small subject test files. No dependency or AST declaration-shape change.

Reference: [GHC9.14.1 family compatibility](https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/type_families.html#compatibility-and-apartness-of-type-family-equations).
Nine fresh reference controls are retained in the scratch open-rows-reference
JSONL: six legal cases fail the frozen a426 compiler; conflicting finite and
infinite overlaps wrongly pass it; a free RHS name already rejects. Published
counts are diagnostic baselines, not acceptance criteria to game.

This is pre-beta/local/reversible compiler work under L.J.'s continue direction.
Confidence high on the dropped row/dependency, medium on hidden-input consumer
coverage. Alternative was deferring all open reduction, which leaves both
measured regressions. The isolated checkpoint bounds correction cost; no human
authority or public methodology decision is required to implement and test it.

## Checklist

- [x] Owned checkpoint pushed; source/CLI identities and main verified.
- [x] Independent GHC9.14.1 references for forward/reverse order, implicit/explicit
  hidden inputs, compatible/incompatible overlap and free RHS names.
- [x] Install integration controls and demonstrate the intended failures.
- [x] Attach direct local rows/dependencies to owners; publish after checking.
- [x] Retain one hidden-input spine through kind term interpretation/reduction.
- [x] Validate open compatibility and reduce unordered rows with a bounded shared matcher.
- [x] Execute an independently checked witness on STG/LLVM/Cranelift; preserve negatives.
- [x] Focused and crate/integration gates on the reviewed implementation.
- [ ] Commit/push the isolated implementation and freeze its CLI for the diagnostic.
- [ ] One frozen diagnostic per axis, exact set/reason comparison versus eb809e70/main.
- [ ] Final whole-workspace and two-pass landing gates; no main merge with regressions.

## Resume state

Implementation complete in the owned worktree; checkpoint/frozen diagnostic next.
Scratch evidence is under /private/tmp/haskelujah-ascription-chirho.EgTd8V.
Typing379/379 and integration185/185 plus canaries7/7 are green with zero ignored
or filtered. The exact hidden-row witness prints42 on STG/LLVM/Cranelift and
GHC9.14.1. These are not a full driver-lib or workspace gate: the last1774 full
driver green belongs to a426 and remains explicitly labelled as such.

## Measured iterations and limits

- The initial nine integration controls ran1/9 on the old implementation: six
  valid inputs falsely rejected, two conflicting row sets falsely accepted,
  and the free-RHS-name negative already rejected. New controls first reached9/9.
- The broader175 integration run exposed two regressions: retaining an irrelevant
  closed-family hidden index on a nested RHS broke closure/injectivity validation,
  and recursive Plus references no longer shared their row representation.
  The fix applies an already-published closed erasure proof at both producer and
  use. All29 family controls recovered; no negative assertion was weakened.
- GHC rejects an infinite LHS overlap even when both RHSs are Int. The initial
  generic validator accepted it. A new source control went red on that validator;
  compatibility now requires a finite substitution after complete rational
  unification. A clash in another input still proves apartness. RHS equality is
  read-only and may compare identical family-headed terms structurally.
- Validation, matching-input preflight, expanded substitution and kind-spine
  materialization share bounded work. The matcher preflight charges complete
  trees and enforces depth128; it is not one debit per recursive comparison.
  Exhausted normalization returns None, not a copy of the residual tree.
- Fresh focused CLI: T12381 recovers. T11348 still fails because TrivialFamily's
  hidden input remains an unsolved kind metavariable in ProblemType. This is not
  permission for open-family inverse improvement. The other eleven prior
  main-relative regressions also remain; a full diagnostic is still owed.
- ExplicitRowInputChirho is valid and runs42 with GHC, but our CLI reports E0101
  on a fabricated question-mark family name. collect_app_class_chirho cannot
  represent KindApp on an INSTANCE LHS. Explicit arguments at USE sites work;
  that is a different producer and is not claimed to repair the instance head.
- The first focused-run script failed before any compiler observation because
  Bun's hash API needs a typed view, not ArrayBuffer. Corrected the script and
  the nested T16234/ControlMonadClassesState path, then reran all21 observations.
- Reference JSONLs preserve21 independent GHC sources, including the unsupported
  explicit-instance-head source. No oracle is invented from our output. Public
  artifacts, main, canonical row484 and corpus membership remain untouched.
- Clippy exits0 but emits461 warning-message lines including summaries and
  duplicates. The new ShapeChirho enum triggers the postfix lint under the
  required Chirho convention; it is not suppressed. Two other new suggestions
  were repaired. This checkpoint is not lint-clean or a whole-workspace pass.
- A further GHC -ddump-tc reduction localizes T11348: TrivialFamily itself has
  Type -> Type, not the polymorphic kind our open head invented. GHC accepts
  the inferred and explicitly Type-kinded alias, rejects the Bool-kinded alias
  and a second Bool-kinded instance. That is an open-head inference policy,
  not permission to infer inverse equalities from an open row. Its repair is
  the next separate checkpoint; no code for that policy is included here.
- Independent review found a hidden-input validity gap: GHC-73138 rejects an
  irreducible family in an invisible equation input while the candidate accepted
  it. GHC accepts both its reducible counterpart and a fixed family classifier.
  Publication now normalizes and validates actual hidden matching inputs, not
  every source classifier. A one-row collection independently refuses a family
  LHS; pairwise validation alone cannot prove that row legal. Three integration
  controls and a generic negative retain the measured distinction.
- Review also found tuple child enumeration allocated before work accounting.
  FamilyChildrenChirho is a constant-work borrowed view, with fanout checked
  before enqueuing/importing children. This closes that width bypass without
  claiming every opaque type consumer or the whole compiler is budgeted.
- The first hidden-input test invocation failed to compile because expect_err
  requires Debug on the success type. A let-else test fixes the test harness;
  the semantic red is independently measured on the hashed pre-repair CLI,
  not inferred from that Rust compile error. The width guard initially left
  unused fuel on exhaustion; the existing output-budget control caught it and
  the established zero-fuel exhaustion contract is restored.
- Second independent read-only review found both reported issues closed and
  no further concrete fault within those changes. Current final gates repeat
  typing379, integration185 and canaries7; all have zero ignored/filtered.
