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
- [x] Freeze/push source; full workspace, explicit CLI build, two complete passes
      per upstream axis with exact file deltas and stable binary provenance.
- [ ] Supersede evidence, fast-forward main, run in-place smoke checks, close
      row 480, commit/push named paths and release builder/DB leases.

## Evidence and scope limits

Transient probes and logs: `/private/tmp/haskelujah-forall-scope-chirho.AI6njz`.
The prior lane's two-use `a -> (forall a. a -> a) -> a` case is the starting
reduction; it is rechecked here rather than treated as a current measurement.
Existing clippy diagnostics and oversized-file debt are not waived; the raw-output
comparison is recorded below rather than treating formatting duplicates as new defects.

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
- Focused integration suite: 18/18. Scope unit controls: 7/7. Full typing: 328/328.
- Canaries 7, dictionary evidence 10, given equalities 10 and rigid variables 14
  all pass. The targeted typing/driver all-target clippy invocation exits zero
  but emits 558 warning messages across its targets/dependencies; none has a
  span in the new conversion modules or edited integration test. This invocation
  is not the same scope as the preceding lane's 535-message workspace tally.
- Two conversion modules (225 and 550 lines at the source checkpoint) remove a net
  588 lines from the 28,417-line inference root. Remaining structural debt is open.
- Full type-synonym RHS alpha-renaming and nested qualified-type representation
  are distinct pre-existing limits, not claimed by this repair.
- Public-label truncation versus nearest rounding is awaiting L.J.'s decision;
  the peer messages are advisory. No reporting-policy change or deployment here.

## Frozen gates — 2026-09-10 UTC

- Source `dace042b98e12ea3c8a885f17fa1e9c320b1481d` committed and pushed to
  `gh_chirho/gpt-forall-scope-chirho`, with the remote hash independently read back.
- Explicit warning-free CLI build; SHA256
  `9d449ed5521d4798a5fa626ad08233409d023ecc393f2afc423a5fe363a19c32`.
  HEAD and digest held before/after all four P4/15-second corpus passes, with
  isolated 60-second timeout reruns and a pure-shell diagnostic detector.
- Accept 879/938: zero gained/lost, baseline failing list byte-identical.
  Reject 223/767: gained GivenForallLoop.hs, zero lost. Both repeated pass pairs
  agree byte-for-byte; zero unresolved timeouts or unexpected exits.
- GivenForallLoop's own .stderr and a fresh GHC 9.14.1 invocation report
  GHC-25897: the outer `a` and `b` are distinct rigid variables despite the inner
  `forall b`. The baseline CLI accepts; the repaired CLI reports that rigid
  mismatch (E0200). No source/oracle was changed to obtain this matching-reason gain.
- Full workspace cargo exit 0: **3344 passed, zero failed/ignored/filtered**,
  75 completed targets. Driver library 1773, including the existing 126 native
  round trips; bulk 8; curated 537/537 with 514 compared execution oracles and
  23 compile-only inputs. All 514 source hashes still match the earlier complete
  GHC 9.14.1 manifest; this is not a fresh 514-program GHC rerun.
- The result-summary parser initially rejected a next-target stderr heading
  arriving before the previous stdout summary. Ordered target/result pairing now
  requires each libtest-announced count to agree; a mutated count is proven red.
  The raw completed log is unchanged, SHA256
  `b51621d1b0746614570b2e25ae9f7920fab8329b34a8ea5214c83be0edc1e11d`.
- `cargo fmt --all --check` and `git diff --check` pass. Workspace compiler warnings:
  zero. Workspace all-target clippy exits 0 but emits 636 warning-output lines.
  The independent read-only audit normalizes both this and the prior 535-line
  log to 493 primary diagnostics at 492 locations; 101 repeated parser renderings
  explain the difference. Root-inference locations shift with the 588-line
  extraction. No new conversion/integration-file diagnostic is identified.

## Measured next defect, not folded into this freeze

The separate kind converters still retain an inner forall's environment entry.
Both the baseline and `dace042b` CLIs reject the following with E0300, while
GHC 9.14.1 executes it warning-free and prints `42/7`:

```haskell
{-# LANGUAGE RankNTypes #-}
module Main where
retainChirho :: fChirho Int -> (forall fChirho. fChirho -> fChirho) -> fChirho Int
retainChirho valueChirho _ = valueChirho
retainLaterChirho :: (forall fChirho. fChirho -> fChirho) -> fChirho Int -> fChirho Int
retainLaterChirho _ valueChirho = valueChirho
main :: IO ()
main = do
  print (sum (retainChirho [42] id))
  print (sum (retainLaterChirho id [7]))
```

`kind_chirho.rs` has separate `infer_type_kind_chirho` and `type_to_kind_chirho`
paths, not the surface/signature map repaired here. This is the next measured
scope defect, not a claimed corpus gain or a reason to modify this frozen source.
