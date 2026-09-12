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

## Hidden-input implementation checkpoint

Checkpoint tag family-matching-indices-before-chirho points atde8fa2ed. The
chosen isolated implementation is typed clauses carrying separate kind/type
inputs, plus solved family-occurrence indices. Registration, imported clause
transport, reduction and equality deferral all retain visibility. One shared
matcher binds hidden and ordinary patterns together; source-kind authority for
imports and kind-level hidden-row reduction remain outside this result.

- [x] FamilyKindSelectionChirho: GHC9.14.1 executes49; new driver control shown
  red before implementation and green afterward. Reversed open-row order and
  implicit kind applications also execute49; wrong constructor rejects.
- [x] ListKindFamilyRhsChirho: a written `[k] -> k -> Type` annotation was erased
  wholesale, allowing Maybe at the wrong kind. Preserve it as the existing
  list-constructor application. The positive/negative driver control was shown
  red before that conversion and green afterward; GHC independently agrees.
- [x] Existing flexible-kind application control recovered. Annotation scopes
  now reuse a known implicit name's lexical classifier rather than giving its
  second occurrence a fresh kind-of-kind. No assertion changed in that test.
- [x] RecursiveFamilyKindsChirho: GHC executes49; driver control shown red then
  green. Captured equation keys now follow the substituted published identity,
  not the stale pre-rigidification key. Conflicting captures emit an error.
- [x] Fresh explicit CLI accepts T16502b and T25597 again; T21583 stays accepted.
  Remaining focused failures: CoerceToVDQ,T12381,T17067,T13879,T14010,T22560c,T26358.
- [x] Remove all temporary family-input trace instrumentation; explicit CLI build
  warning-free, SHA256a29b95bb00153a8d0c95137f6e2d39c1e2c048ccf5d887f017fcf4c6319d7dec.
- [x] Parser356/356, typing369/369, canaries7/7, zero ignored/filtered;
  workspace all-target check passes. The parser's old `[Symbol]` test pinned an
  absent binder annotation: replace that assertion with the complete list kind,
  retaining its separate no-declaration-kind assertion. No scanner guard removed.
- [ ] Integration113/114, zero ignored/filtered: only the existing legal
  ClassifierCycleChirho annotation control is red. It remains enabled unchanged.
  Driver--lib is running separately; full workspace tests remain owed.
- [x] Freeze/push5280f1ff, remote-exact; one diagnostic pass per axis completed.
- [ ] Type-pattern ascriptions/promoted constructor indices, existing classifier
  control, kind-level open rows and all accept regressions remain open.

The prior corpus880/247 does not describe this hidden-input checkpoint. The legacy
usize::MAX reducer wrapper is removed now that the type consumer calls a bounded
single-row matcher; this is not the cause of the earlier T15552a abort. That abort
was independently reduced to classifier recursion and fixed in the22-20 leaf.
No main merge, canonical DB update, denominator/label edit or site deployment.

## Hidden-input diagnostic and broader gate,2026-09-12 00:06EDT

Source5280f1ff stayed clean throughout. Driver--lib completed1760/1772, zero
ignored/filtered,885.49s. All twelve exact failures also failed on a freshly built
parentde8fa2ed (0/12,1760 filtered in that focused A/B). That establishes only
that5280 did not introduce these failures; all twelve remain row484 landing
blockers. The earlier full green at45c4a108 is a bracket to preserve, not a reason
to call these inherited. The failures concern imported/re-exported aliases,
associated-family schemes and two real parsec source slices. Raw final diagnostics
are hidden-family-{parent-,}driver-lib-chirho.log under the scratch directory.

After restoring5280, explicit cargo build -p haskelujah relinked a different CLI.
The old-hash provenance guard stopped BEFORE the first corpus file. No failed
measurement was counted. Nine reference controls were rerun on the rebuilt CLI:
all agree with GHC9.14.1, including five exact executions. Their sources, hashes,
exits and output are committed in family-equations-chirho/hidden-inputs-chirho.jsonl.
The corpus then used SHA256
`78b0c5eb63303b62c4a2b455c6bc9efb86335a781c58b7a3b77efcbe576b021c`,
unchanged before/after each axis. Diagnostic completed2026-09-12 00:05:53EDT;
both axes have zero timeouts/unexpected exits. Evidence directories are
diagnostic-hidden-family-inputs-{accept,reject}-chirho under the same scratch root.

Accept880/938, reject248/767. Accept880->880 hides three recovered files and
three new failures: T16188/T16502b/T25597 recover; ControlMonadClassesState,
T18129,T20356 newly fail. Against main: eight gains (PolytypeDecomp,RuleEqs,
SplitWD,T14451,T16188,T20922,T26256a,tc124), ten regressions (CoerceToVDQ,
T12381,T13879,T14010,ControlMonadClassesState,T17067,T18129,T20356,T22560c,
T26358). These are verdict movements, not ten independently validated features.
T16188 still contains unrepresented data instances; acceptance is not execution
or singleton-refinement proof. Main remains882/221; no final gate or landing.

Reject delta from247: +ContextStack2,+VisFlag2,-T11347. ContextStack2 is a FALSE
REJECTION on an equation-closure error at8:26: the file itself says it succeeds
with the post2016 approach, and fresh GHC9.14.1 accepts it. It has no committed
stderr oracle. VisFlag2's closure error at14:23 is not GHC's required/invisible
forall-kind mismatch; both its committed stderr and fresh GHC9.14.1 name that
visibility contract. Neither gain is bankable. T11347's old E0204 at19:18 is not
the deriving representation-coercion error at6:41 (GHC9.14.1 GHC-10283; older
committed stderr GHC-25897). The lost verdict was not proof of that deriving rule.
Membership and all public labels are unchanged.

The simple no-PolyKinds hypothesis for the import failures was refuted by source:
the compiler defaults to GHC2021, which enables PolyKinds. Do not default away
valid hidden binders to repair an imported contract. Next reduce the new three
failures and alias/equation closure evidence before a new representation sweep.
Observed additional lead: tuple kind inference returns Type when its common
element classifier is unresolved, without constraining that classifier; retained
family occurrences then expose the unsolved classifier as an unbound input.
This is a hypothesis to prove with positive/negative controls, not a finished fix.

## Known Prelude classifier repair

T20356's new alias-closure failure was reduced to `Rekind Eq`. A temporary
trace established that Eq's missing kind binding supplied an unconstrained
classifier, not a binder of the alias. The trace is removed. The environment
already seeded higher-kinded classes but omitted thirteen ordinary Prelude
classes. Register Eq/Ord/Show/Read/Bounded/Enum/Num/Real/Integral/Fractional/
Floating/RealFrac/RealFloat with their actual `Type -> Constraint` kinds through
the existing binding path. These are normal entries; local classes can shadow
them. No family-name special case or imported-kind default was added.

- [x] GHC9.14.1 independently checked all thirteen class contracts and the local
  higher-kinded Eq shadow. The valid hidden-family source executes42; its
  higher-kinded misuse of Prelude Eq rejects with GHC-83865.
- [x] The two new positive/negative controls were shown0/2 before this fix and
  2/2 afterward; a third control preserves local class shadowing.
- [x] Explicit CLI accepts T20356 again and the thirteen-contract source.
- [x] Typing369/369, canaries7/7 and workspace all-target check pass.
- [ ] Integration116/117, zero ignored/filtered: the same ClassifierCycle kind-
  annotation control remains red and enabled. This does not clear the twelve
  driver failures or any unmeasured corpus regression.
- [ ] Freeze/push the owned fix, retain fresh reference observations, and measure
  exact corpus sets before reporting a new diagnostic. The5280 figure is stale
  for this change. Run the twelve driver failures against main explicitly;
  parentde8fa2ed alone was not an inheritance bracket.
