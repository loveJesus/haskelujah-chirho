<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Nested forall lexical scope — 2026-09-09

L.J. directed continue after the infix/binder landing. Baseline `f3a513d0`;
owner HASKELUJAH/gpt_chirho, isolated `gpt-forall-scope-chirho` branch.
Canonical progress row 480 is open in main only. Local reversible compiler work;
no deployment, AST representation redesign or unrelated lint sweep.

## Design and acceptance

The main AST-to-type converter currently inserts forall-bound variables into a
shared name map without restoring the enclosing bindings. Extract conversion
from the 28,417-line inference root into a focused child module. A lexical scope
must introduce fresh local binder identities, convert its body, then restore
only the previous entries for those binders (reverse order), removing entries
that were local-only. Newly discovered free variables must survive scope exit.
Keep explicit binder order, scoped/skolem treatment and required versus invisible
forall representation intact. The consumer audit found that the old detached
predicate conversion depended on leaked result-forall binders; result-spine
constraints now pair with the converted type and temporarily reopen its exact IDs.
Signature preparation also needs a focused sibling: leading foralls and contexts
must be processed in lexical order, and RHS scope export must be distinct from
scheme quantification. Only the first syntactically outermost forall scopes the
definition. Both extraction homes are under `infer_chirho/`; no new dependency.

An independent read-only consumer audit runs alongside the implementation.
Use semantic regressions: repeated Int/Bool use, outer names before/after a
shadow, free variables first encountered inside the binder, and enclosing
ScopedTypeVariables. Negative controls must fail for the relevant type error.
No corpus gain is predicted from source resemblance alone.

## Checklist

- [x] Verify source/lease state and create an isolated branch and canonical row.
- [x] Reproduce the baseline, independently verify controls with installed GHC.
- [x] Add failing regression controls and audit conversion consumers.
- [x] Extract and repair scoped conversion with bounded per-binder state.
- [x] Run focused semantic tests and relevant existing suites; update workflow.
- [ ] Freeze/push source; full workspace, explicit CLI build, two complete passes
      per upstream axis with exact file deltas and stable binary provenance.
- [ ] Supersede evidence, fast-forward main, run in-place smoke checks, close
      row 480, commit/push named paths and release builder/DB leases.

## Evidence and scope limits

Transient probes and logs: `/private/tmp/haskelujah-forall-scope-chirho.AI6njz`.
The prior lane's two-use `a -> (forall a. a -> a) -> a` case is the starting
reduction; it is rechecked here rather than treated as a current measurement.
Existing 535 clippy warning messages and oversized-file debt are not waived.

## Focused evidence

- Explicit baseline CLI build at f3a513d0, SHA256
  `bcb77a970d37d1d34894162925ad2b7951ace8930d190878af50e31141bb2baa`:
  the Int/Bool two-use shadowing control rejects its second use (E0200).
  Three new converter scope tests fail against the original implementation.
- GHC 9.14.1 independently executed four exact integration sources: invisible
  shadowing/free-first-inside/enclosing scope; required shadowing; parenthesized
  quantification/local shadowing/consecutive groups; constrained result foralls.
  All outputs match the repaired STG, LLVM and Cranelift executions. It rejects
  the parenthesized/later-group RHS-scope controls and the Bool escape control
  under GHC-25897; our corresponding diagnostics are tested, not just error presence.
- Focused integration suite: 18/18. Scope unit controls: 7/7. Broad gates pending.
- Canaries 7, dictionary evidence 10, given equalities 10 and rigid variables 14
  all pass. The targeted typing/driver all-target clippy invocation exits zero
  but emits 558 warning messages across its targets/dependencies; none has a
  span in the new conversion modules or edited integration test. This invocation
  is not the same scope as the preceding lane's 535-message workspace tally.
- Two conversion modules (225 and 549 lines at this checkpoint) remove a net
  588 lines from the 28,417-line inference root. Remaining structural debt is open.
- Full type-synonym RHS alpha-renaming and nested qualified-type representation
  are distinct pre-existing limits, not claimed by this repair.
- Public-label truncation versus nearest rounding is awaiting L.J.'s decision;
  the peer messages are advisory. No reporting-policy change or deployment here.
