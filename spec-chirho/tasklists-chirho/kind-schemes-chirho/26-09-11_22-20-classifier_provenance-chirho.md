<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Classifier recursion and source provenance, row484 continuation

The active root tasklist retains the unit's authority and historical evidence.
This focused leaf keeps that1476-line history below the1500-line cap rather
than extending it into another monolith. Local/reversible; own worktree only.
Main121d4f2c,882/938 and221/767, remains unchanged. No DB/public artifact edit.

## Diagnostic that required the repair

Frozen5917c4fd, CLI117bb4d1, produced881/938 accepts, no timeouts. It recovered
T12928/tc160 but newly lost T11723/T13142/tc269 compared with9720465c. Main-relative
gains: PolytypeDecomp, RuleEqs, SplitWD, T14451, T20922, T26256a, tc124.
Main-relative regressions: T11723, T13142, T13879, T14010, T21583, T22560c,
T26358, tc269. Reject accounting was245 rejected,521 accepted,one abort134 on
T15552a. The runner failed;245 is not a valid completed reject measurement.

Logs and per-file sets: /private/tmp/haskelujah-visible-kind-app-chirho.X3Kq8n/diagnostic-synonym-identities-chirho.log and its accept/reject directories.

## Root causes and implementation

- T15552a: instrumented stack/term traces repeat exactly EntryOfValKey
  (T15552.EntryOfVal e) through classifier -> consume argument -> unify -> check
  solved classifiers. This is recursive proof construction, not evidence of
  recursive family reduction. A scoped cache reserves a result variable before
  checking arguments and unifies that result on completion. Re-entry shares the
  proof obligation rather than opening fresh copies forever. Retire the cache
  at the outer substitution-check boundary. Depth128/work16384 exhaustion emits
  its own diagnostic. Unknown heads remain unproved; all temporary traces removed.
- T11723: flat syntax erased the tick on '[], fabricating the list TYPE constructor.
  Preserve promoted constructor/list syntax; reuse depth-aware comma splitting for
  tuples and promoted lists. AstKind still cannot hold these annotations, so this
  removes a fabricated check, not the existing unrepresented-kind limitation.
- T13142: qualified K.Type became Star, then unqualified Type, resolving the local
  type called Type. Qualified Type/Constraint retain the nominal name and span.
- tc269: the synonym binder identity was anonymous in its own contract but named k
  in another declaration. Converting by the global source spelling allocated a
  different free variable. Bind both the local parameter name and the exact kind
  identity; solved occurrences prefer the identity. No extra parameter is invented
  and the unclosed-body diagnostic remains in force. Higher-rank kind subsumption
  remains incomplete; this is not a claim that tc269's entire feature works.

## Checks and remaining work

- [x] Verify ownership, interrupted trace build and main cleanliness.
- [x] Show both new parser controls failing on their actual missing properties.
- [x] Remove the crash with a scoped recursive classifier proof.
- [x] Positive cycle with UndecidableInstances accepts; supplying Bool instead of
  EntryOfVal rejects for a kind mismatch, both under GHC9.14.1 and candidate.
- [x] Assert the resource-bound diagnostic itself and distinguish changed local
  classifier authority after the previous proof cache has retired.
- [x] T11723, T13142 and tc269 pass the explicit rebuilt CLI.
- [x] Parser356, typing368, integration110, canaries7; zero ignored/filtered.
- [x] Workspace all-target cargo check and explicit CLI build pass.
- [x] Forty bounded GHC/candidate reference observations agree; nine compare exact
  execution stdout. Sources, diagnostics and binary hash remain in reference JSONL.
- [x] Final CLI SHA2562b9d7f0ead9afcd2cc67b903e8f6f8495117f5bb54f8fd47bc73c7d1dc285fc6.
- [x] Private diagnostic classifier7/7: abnormal exits take precedence over any
  preceding error text; a panic/abort cannot satisfy the rejection count. Timeout,
  ordinary error, success, missing diagnostic and a benign panic filename discriminate.
- [ ] Freeze/commit/push explicit owned paths and run both complete diagnostic axes.
- [ ] Compare exact sets and record every delta; no main landing while any new
  should_compile failure remains. Final two-pass/full-workspace gate still owed.

Clippy exited0 but reports existing warning debt; this is NOT warning-free.
No warning was emitted in the modified classifier/provenance helper files.
The unchanged original T15552a now accepts: its GHC-22979 nested-family validity
rule is unimplemented. Crash recovery does not earn reject-axis capability.
The generic reducer's usize::MAX wrapper is a separate boundedness concern;
Claude2's candidate mechanism was not the recursion shown by the measured trace.
