<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Family equation occurrence consumers, row484 continuation

Own isolated worktree; parent5d526afb, main121d4f2c unchanged. No canonical DB,
public measurement, membership, percentage policy or deployment change.

## Measured contract

The OpenFamilyRhsChirho source declares a poly-kinded Box, defines an open Base
equation returning Box Maybe, constructs Base Int Bool, and reads True back.
GHC9.14.1 executes True. The3c56cf99 CLI rejects it as Box @t6 versus bare Box.
Changing its signature to Base Int Int gives GHC's Bool-versus-Int rejection.
The driver regression test was shown0/1 before code changes; it exercises both
open and closed forms, exact output and the contradictory ordinary type argument.

## Implementation and gate

- [x] Share equation-local classifier checking between closed rows and top-level
  open rows of locally declared families, after declaration kinds are published.
- [x] Reuse solved nominal occurrence conversion for stored equations. An explicit
  equation policy prevents eager family reduction and pattern-wildcard warnings.
- [x] Require result variables to belong to converted matching inputs; no fresh
  RHS parameter or nominal stand-in is invented when a source binder is absent.
- [x] Driver control1/1 now green; both sources execute True and reject Int/Bool.
- [x] Explicit CLI build; actual T21583 now accepted.
- [x] Typing368/368; canaries7/7, zero ignored/filtered.
- [x] Workspace all-target cargo check passes.
- [ ] Driver integration:110/111. The existing legal ClassifierCycleChirho
  control is newly red; do not weaken or ignore it. No full-workspace test gate.
- [x] Freeze/push1b742b11, remote-exact, and run one diagnostic pass per corpus
  axis. Accept880/938, reject247/767; zero timeouts or unexpected exits.
- [ ] Preserve type-pattern kind ascriptions and their matching binder provenance,
  then repair all regressions before any main landing.

## Newly exposed missing representation

The legal row is EntryOfValKey ('EntryOfVal (_ :: Elem (k,v) kvs)) = k.
lower_chirho.rs's KindAnnotTypeChirho and ParenTypeChirho paths retain only the
type left of ::. The AST TypeChirho has no ascription variant. The old static
SynonymTypeConverter turned an RHS variable missing from its parameter list into
ConChirho("k"). That was not a valid equation or evidence of annotation support.
The new converter reports an unbound matching input instead, exposing the gap.
The cycle proof itself still terminates; this is a different missing consumer.

Next representation decision: a real type-kind ascription node, containing both
the annotated type and its kind plus their spans, with naming and kind checking
visiting both. Type-level family matching additionally needs the bound identities
from those annotations in its actual matching inputs (including promoted
constructor indices); merely storing a tag or adding RHS variables cannot supply
that contract. This is separate from the deferred data-family/type-data/refined
GADT-result declaration decisions. Do not begin that sweep in a frozen gate.

This is a checkpoint with a known regression, not a completed feature or a claim
that the GHC corpora pass. Diagnostic outcomes and reference records follow here.

## Completed diagnostic and reference audit

The explicit CLI SHA256 was
`b6463bd60a2c50320bc5c6c8f92d1cb51588cbaab11a5a72aa9e4b430999502c`,
unchanged across both passes. Source clean at1b742b11 before/after each axis;
the last pass ended2026-09-11 23:00:19EDT. This is not the two-pass final gate.
Evidence directories under `/private/tmp/haskelujah-visible-kind-app-chirho.X3Kq8n/`
are `diagnostic-family-occurrences-{accept,reject}-chirho/` (summaries, exact
sets, per-file exits/logs, timeout and unexpected-exit lists). CPU embargo released;
SLOT/DB still ours. Main121d4f2c and published counts remain882/221.

Against3c56cf99, T21583 recovers; CoerceToVDQ, T12381, T16502b, T17067 and
T25597 newly fail. Main-relative gains remain PolytypeDecomp, RuleEqs, SplitWD,
T14451, T20922, T26256a and tc124. Main-relative regressions are CoerceToVDQ,
T12381, T13879, T14010, T16502b, T17067, T22560c, T25597 and T26358.
Seven gains and nine losses, not a landable net count.

New-failure reductions: CoerceToVDQ loses the binder in `(_ (arg :: k))`;
T16502b and T25597 need hidden family matching inputs, absent from stored rows;
T12381 needs open-kind-family reduction for `G Int = Bool`; T17067's data-family
heads are classified as prohibited type-family equation applications. None is
fixed by suppressing the equation closure check. Data-family shape authority
remains separate and no new declaration shape was adopted here.

Reject gains T15552a, T15793 and T17301, losses none. Comparing their candidate
logs to their committed GHC stderr: T15552a rejects on erased annotation binder k
at17:56, not GHC-22979's nested family use at26:9. T15793 rejects an unbound RHS
matching input at18:11, not GHC-45474's oversaturated visible kind argument at18:3.
T17301 rejects erased annotation binder ty at26:5, not GHC-16220's existential
ambiguity in Forget at22:3. All three are unearned verdict gains, not new rules.
These are reason comparisons to committed stderr, not new full GHC runs.

`test-data-chirho/kind-oracles-chirho/family-equations-chirho/equation-occurrences-chirho.jsonl`
retains seven bounded GHC9.14.1/candidate observations with exact sources and
hashes: six verdict agreements, one disagreement. Two are execution controls
(open/closed Box source, both True). Two contradictory Int/Bool sources reject
on both; T21583 checks on both (GHC emits missing-method warnings). The legal
ClassifierCycle source checks on GHC and fails here; its wrong-kind mutation
rejects on both. Do not describe this record as all green or all execution.

## Next shared consumer decision

The next isolated step will represent hidden family inputs in both stored
equation matching spines and family occurrences, not only nominal RHS uses.
Use kind elaboration's solved row identities; never fabricate extra RHS binders.
Preserve the distinction between ordinary and invisible applications through
reduction, deferral and injectivity. Visible injectivity claims must not silently
become claims about a new hidden argument. No new dependency is needed.

A fresh source reduction defines two nullary-visible open Pick equations,
selected by kind (`Type` gives Maybe; `Bool` gives FlagChirho). GHC9.14.1 runs
it to49; the1b742b11 CLI reports unreduced Pick @Type/Pick @Bool. This is the
first exact-output control for the next consumer, not an achieved fix. Confidence
is high in the missing input, medium in all consumers; reversal remains the
pushed checkpoint. Kind-ascription/promoted-constructor indices still require
their own real representation after this family-spine step; their existing red
control stays enabled. Open-kind reduction and data-family classification also
remain explicit gates rather than incidental accept/reject totals.
