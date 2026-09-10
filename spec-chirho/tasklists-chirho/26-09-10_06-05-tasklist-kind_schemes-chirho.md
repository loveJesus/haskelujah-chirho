<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Kind binding and declaration checking — 2026-09-10

L.J.: continue working while I sleep. HASKELUJAH/gpt_chirho retains the builder
and canonical DB lease after row483's verified landing at 8bc16dad. This unit
is local/reversible, isolated on gpt-kind-schemes-chirho in the existing owned
kind-signatures worktree; main's fresh row483 CLI remains the A/B reference.
No percentage policy decision, site deployment or deferred declaration-shape work.

## Brick 1

Two measured wrong accepts share missing kind-binding metadata, but have
different checking rules. A standalone `type Box :: k -> Type` must not be
specialized by `data Box (a :: Type)`. The recursive tcfail225 body is accepted
by GHC 9.14.1 with CUSKs and rejected with NoCUSKs. The latter is a recursion/
declaration-completeness distinction, not the standalone subsumption rule.

The current environment stores bare kinds and treats every free variable as
universally quantified at every constructor use. Its unifier treats all kind
variables as flexible. Recommendation: explicit monomorphic/poly-kinded
bindings, quantifier-aware instantiation and substitution, and scoped rigid
checking of written contracts. Data/newtype checking should own one lifecycle:
head, complete/incomplete recursive binding, constructor fields, publication.

Place the binding machinery and its tests in focused kind child modules, and
centralize the duplicated data/newtype field checker there. Something is wrong
with the 3284-line kind root; extract this relevant unit rather than add another
large arm. No new dependency. Existing broad size/lint debt remains a failure,
not waived by this repair. Work must scale with the binder/contract, not clone
the growing environment for each local scope.

Confidence: high in the two reductions, medium in the integration. Alternative
of making every kind variable rigid rejects ordinary inference; making every
recursive use monomorphic rejects valid complete signatures. Neither is a fix.
Checkpoint kind-schemes-before-chirho at 8bc16dad bounds reversal. Required
dependent-kind binders, implicit versus specified quantifier metadata and
unsupported promoted kinds may require a finer representation before rigidity
is safe; reductions decide rather than per-corpus guards.

## Checklist

- [x] Re-read project/goal/Git; prior landing pushed and remote-exact.
- [x] Record baseline/reference controls and audit completeness/binder consumers.
- [ ] Implement explicit binding/checking lifecycle with focused positive/negative tests.
- [ ] Run parser/naming/typing/driver controls and behavioral execution canaries.
- [ ] Freeze and run full workspace plus two complete passes on both corpus axes.
- [ ] Name every delta by reason; commit/push owned paths, main-path smokes and DB closure.

## Acceptance and limits

Controls must distinguish a rigid standalone contract from an unannotated valid
head, and CUSKs/NoCUSKs from ordinary monomorphic inference. Preserve alpha-
renamed kind scopes, invisible head binders, required-kind arguments, GADT
constructor-local variables, existing quantified-constraint class uses and
GHC-55233's declaration/binder distinction. Scheme instantiation must preserve
sharing internally and separate different uses, without substituting its bound
variables or allowing a local rigid identity into an unrelated declaration.

No claimed corpus gain estimate yet. Upstream baseline is 882/938 accept and
221/767 reject; full workspace3387, zero exclusions. The prior kind-control
source/oracle reductions and three-engine executions remain mandatory controls.
Full kind-dependent types, all class/family/synonym CUSKs, UnliftedNewtypes,
cross-module kind authority and generic record Eq/Show are not implied by a
green focused test. Any unsupported representation exposed by rigidity stays
named, never bypassed by a source-name or corpus-file condition.

## Resume state

Unlanded prototype: explicit Mono/Poly kind bindings, quantified substitution,
rigid written-kind checking, journaled constructor scopes and shared data/newtype
checking. The first explicit CLI build succeeded; fresh GHC 9.14.1/main/prototype
controls show three real wrong accepts rejected (standalone specialization,
rigid annotation used as Type, NoCUSKs recursive specialization), with seven
valid controls held. Scratch evidence is in
`/private/tmp/haskelujah-kind-schemes-chirho.h1N3tE/`; it is not a frozen gate.

Checkpoint 7706b150 is pushed on the isolated branch. Its first typing run was
349 passed / 1 failed: a class referencing later local `Tagged` published its
kind too early. Dependency scheduling below repairs that failure without
weakening the assertion or substituting quantified variables.

### Dependency-order decision checkpoint

Recommendation: infer dependency SCCs, keep incomplete members monomorphic until
the whole group has been checked, and publish only then. Complete written kinds
must be available to recursive uses independently of declaration order. Type
signatures are checked after the kind declarations they consume. This replaces
source-order publication; it does not add a forward-name exception.

Reuse the existing adjacency-to-SCC algorithm from value inference in one common
dependency module. Keep source/value dependency collection separate. Typing's
src directory is already at 15 entries; group the existing type representation
and substitution files under `types_chirho/` with their public module paths
preserved, so the common graph module does not grow a 16th root entry. These are
pure moves; no type/substitution semantics change is intended. Kind scheduling
and dependency collection live in focused kind child modules, not the large
root. Confidence: high that early publication is the defect, medium on complete
signature recursion and declaration-graph integration. Reversal is the isolated
prototype checkpoint; no main source changes, new dependencies, or public API
renames. Independent read-only review is checking group/scope requirements.

Canonical row484 remains open; only main writes the DB. Full corpus/workspace,
fresh final CLI and landing remain owed. The prior main row483 gates are not
evidence for this prototype.

### Diagnostic reduction, not a landing

The first full diagnostic (CLI SHA-256
`077a0a95f0ba79e315afa8c3f9e5ee41c52093d224d431bd230fb07da7823154`)
found 863/938 accept and 230/767 reject, one pass per axis, no timeouts or
unexpected exits. Nineteen baseline-accepted files failed and none was gained.
The reject net +9 is not yet classified by each file's own GHC reason; it is
not nine banked capabilities. Main and published artifacts remain unchanged.
Evidence: `/private/tmp/haskelujah-kind-schemes-chirho.h1N3tE/diagnostic-first-chirho.log`.

The reduction repaired three roots, without file-name guards:

- Superclass and quantified variable-headed constraints now reach class-kind
  inference before publication. The flat context lowerer previously dropped
  every segment containing `forall` or `=>`, despite an existing quantified
  constraint AST. A focused context module reuses the full type parser and
  constraint conversion. Leading `forall` now scopes over its whole body,
  including arrows and premises; binder annotations retain their structure.
- Effective defaults are GHC2021 (PolyKinds, not CUSKs), with ordered explicit
  edition/extension overrides. Legacy NoPolyKinds CUSKs must contribute body
  constraints before publication; standalone signatures still break inference
  cycles. Eight reduced GHC 9.14.1/candidate pairs agree on those boundaries.
- Written kind variables in an incomplete inference SCC may alias one another,
  but may not specialize to concrete or arrow kinds. Retain original variable
  identities, check the group, validate that contract, then rigidify and publish.
  The independently rejected NoCUSKs concrete-specialization control remains red.

The latest focused recheck of the original nineteen, on CLI SHA-256
`7237bcd21b42ad0b3f15a95294a01885418a77672909d47b427866005e73fdcf`,
recovers ten: T14735, ControlMonadPrimitive, T16609, T18920, T20732, T22383,
T22560b, T26020, T7196 and tc201. Nine remain: CoerceToVDQ, T10432, T17021a,
T17817b, T18891, T20187b, T21951b, T22141f and TcTypeNatSimple.
Evidence: `/private/tmp/haskelujah-kind-groups-chirho.EaepkD/19-after-chirho.jsonl`.
This is not a new full-corpus count, and recovery does not prove associated
family or class standalone-kind contracts are represented.

Parser336 and typing352 passed with zero failures, ignores or filters
(actual Cargo0, `crates-final-chirho.log` in that same directory). The first
driver run was 40/41 plus canaries7/7: the new helper required literal wording
`kind mismatch` although the invalid instance correctly returned E0300 with an
instance-specific message. The helper now checks that machine-readable code;
the rerun passed integration41 and canaries7 with zero exclusions (Cargo0,
`integration-code-chirho.log`). The group controls also exercised exhaustive
three-vertex dependency graphs and 100,000-vertex chain/cycle bounds.

Open representation debts exposed by the remaining nine include named kinds
being confused with free kind variables, promoted constructor uses sharing a
monomorphic kind, and visible dependent binders/TYPE terms losing dependency.
Do not erase those distinctions to regain the count. A separate negative
`LimitChirho Maybe` control is accepted here but GHC-83865 rejected: existing
Type/Constraint compatibility is not a proof that a variable predicate returns
Constraint. That wrong accept is recorded, not hidden by the recovered positive.
Original reference controls, another full diagnostic, reject-reason attribution
and the frozen landing gates remain owed.

### Second diagnostic and boundary repairs

Checkpoint 71ec7bd7's full diagnostic, CLI SHA-256
`3f2084d283fdd3eab331b65a908e652585005a525e1d4450de722951ca2434fb`,
also accepted 863/938, but it was not the same set: twelve first-pass failures
recovered and twelve new failures appeared. Relative to main: +T15079/tc124,
-21 accepted files. Reject239/767 was net +18, still requiring per-file reason
attribution. Both axes completed with zero timeouts/unexpected exits. Evidence:
`/private/tmp/haskelujah-kind-groups-chirho.EaepkD/diagnostic-reduction-chirho.log`.
An unchanged count is not convergence; compare membership on the next full pass.

The next reduction repairs four measured boundaries:

- Class, family and alias heads reuse whole-binder consumption. An unreadable
  bracketed/application annotation must not turn its inner `::` into a result
  kind or its variable names into extra parameters. The family AST control
  demonstrated two binders instead of four before this repair. Bare predicate
  variables also survive as `c`, not fabricated `? c`, and bind their kind on
  first constraint use. The four-argument family positive and three-argument
  GHC-83865 negative now both discriminate correctly.
- Inline result-kind annotations permit implicit kind variables alongside a
  forall; standalone signatures retain forall-or-nothing. The former naming
  test pinned the wrong rule. Its revised control was red before the fix;
  an independently GHC-rejected standalone counterpart remains rejected.
- Record construction and update feed known field types into the existing
  expected-type checker before inferring lambdas. GHC 9.14.1 and the STG test
  print `(3,'x')` then `7`; a Bool-in-the-Int-result mutation remains E0200.
  The exact-output test demonstrated red on the previous checker.
- Inline kind elaboration records source-variable identities explicitly.
  Temporary application-result metas, including one from a declaration with
  no written kind variable at all, are not written contracts. Resolve captured
  identities after annotation elaboration, then constrain later body inference;
  original head-annotation contracts are unchanged. This does not implement
  nominal/dependent kinds or claim TYPE is fully represented.

Fresh CLI SHA-256
`387ee27500b4f0516db114467438fb93c7729824580c6710d8825a4f840822b8`
recovers all twelve new losses plus T18891 in the 35-file focused recheck.
Eight original main-relative losses remain there: CoerceToVDQ, T10432, T17021a,
T17817b, T20187b, T21951b, T22141f, TcTypeNatSimple. T3632 stays accepted;
HardRecordUpdate still rejects. This is not a full-corpus result.
The original sixteen GHC/candidate pairs and nine new boundary pairs agree
on this binary (50 independently run verdict observations). Evidence:
`boundaries-contracts-after-chirho.jsonl`, `old-controls-contracts-chirho.jsonl`
in the group scratch directory; new paired reductions in
`/private/tmp/haskelujah-kind-boundaries-chirho.PpyQZq/after-contracts-chirho.jsonl`
and `/private/tmp/haskelujah-kind-contracts-chirho.pRvDuM/after-chirho.jsonl`.

Actual Cargo0: parser338, naming134, typing352; integration43, record_fields14,
canaries7, zero ignores/measured/filtered. One parser invocation omitted the
documented RUST_MIN_STACK and aborted on nested parens; the complete rerun uses
16777216 and is green. These focused gates do not cover the full regression
surface: a third complete diagnostic and final workspace gates remain owed.
Main, artifact lists and row484 closure are untouched.

### Third diagnostic: regressions of existing paths, not convergence

Checkpoint e028c1a3, CLI `387ee27500b4f0516db114467438fb93c7729824580c6710d8825a4f840822b8`,
completed one diagnostic pass per axis: accept859/938, reject234/767, zero
timeouts/unexpected exits and stable binary hash. Compared with the second
diagnostic: fourteen accept recoveries, eighteen new failures. Compared with
main: +T14451/T15079/tc124, -26. The lower total is not landable. Evidence:
`/private/tmp/haskelujah-kind-groups-chirho.EaepkD/diagnostic-boundary-chirho.log`.

Sixteen new failures are implicit-parameter labels reaching type-name lookup
after the bare-predicate parser repair. The measured 26-file `?label` surface
has sixteen new failures, nine still accepted and one already-failing file,
tc218; it is not ten passing controls. Constraint labels now use the same
evidence-versus-type namespace distinction as variable uses. Retained argument
types are still visited. Two naming controls were demonstrated red first.
The first draft also used an unseeded Int environment; the reduced rerun removed
that unrelated error before validating the repair. This is not complete
ImplicitParams support: lowering still drops the type in `?x :: Type`, and
the existing value rule supplies fresh types rather than faithful evidence.
A MissingPayloadChirho probe is wrongly accepted by freshly explicitly built
main121d4f2c (CLI77cc6d4f...) and the repaired candidate, but GHC-76037 rejected.
A following MissingResultChirho canary is independently diagnosed by both.

T10808 exposed a different defect in the shared type unifier. Its deferral rule
ran only when structural unification failed, so successful `G a ~ G b`
decomposition incorrectly equated a and b even for a noninjective family.
Deferral now precedes such decomposition; assigning the entire application to
a metavariable remains allowed if its occurs check passes. No record-specific
exception or rollback of rank-N expected checking. The new exact-output
GHC9.14.1/STG control prints True/2; changing the updated Bool field to Char
still rejects. The positive test was demonstrated red before repair.

Candidate CLI `c984eeebd5d8558f2b02c52735b0a7ce7e10229f0965cf7a04e6bfff9b59873a`:
naming136, typing352, record_fields16, typing_integration43, canaries7 all pass
with zero exclusions and actual Cargo0. A fresh 29-file recheck recovers the
sixteen implicit-parameter failures and T10808; tc218 and T25597 remain red.
The prior 35-file recheck preserves the earlier recoveries and eight known
kind-representation losses. This is not a fourth full-corpus measurement.
Evidence: `/private/tmp/haskelujah-kind-third-reduction-chirho.7sP4Re/`.

The second diagnostic's reject239 decomposes into +19/-1 against main. A
read-only oracle review classified five close reason matches, thirteen wrong
reasons and one missing .stderr; these are not nineteen banked capabilities.
The lost T23162b was then bracketed with fresh main/candidate CLIs: main rejects
the valid foo signature at line19 for E0300, not GHC's line27/31 conflicts.
A declaration-plus-foo reduction is accepted by GHC and candidate, rejected by
main. Thus the old rejection was accidental; user-family injectivity checking
remains unimplemented and must not be simulated by restoring its kind error.

Next bounded producer repair: T25597's closed-family equation `FuncU sem '[] r`
loses the bracketed pattern in a separate manual LHS scanner. The repaired
three-argument head exposes that pre-existing two-pattern equation. GHC9.14.1
accepts the unchanged corpus input. Something is wrong with duplicated pattern
scanners in the oversized lowerer: extract family lowering to a focused sibling
and share atomic type parsing for equation/instance heads, preserving complete
brackets, promoted forms and source spans. No new AST or dependency. Prove the
producer shape and behavioral selection with positive/negative controls before
another corpus pass. The current owned checkpoint bounds reversal; main stays
unchanged, and the nominal/dependent-kind design remains separate.

### Family producer repair, still an isolated checkpoint

The two new parser controls were demonstrated red at the previous scanner:
an empty promoted-list pattern vanished (one argument instead of two), and a
nested-list pattern became three arguments instead of two. Reuse is at the
CST grammar rather than another flat-token reconstruction: both open instances
and closed equations call the normal type parser. The extracted family lowerer
consumes the complete application, and the two manual scanners are removed.
Malformed tokens cannot leave the equation parser looping without progress.

Two adjacent representation losses were exposed by independent reductions:
closed-family registration used declaration parameter names rather than the
equation's local variables, and the existing open-instance collector skipped
variables inside promoted lists. A shared focused registration module now owns
both paths and their pattern-variable collection. Infix lowering also retains
the promotion tick (the cons-pattern producer test was demonstrated red first).
One older parser assertion explicitly allowed an ordinary colon constructor
while calling it promoted; it now requires the promoted AST shape, retaining
the checks on both arguments. No reducer arity guard or source-name exception.

GHC9.14.1 and candidate agree on ten source-hashed reductions (seven positives,
three negatives). The combined positive was independently executed by GHC and
then by STG/LLVM/Cranelift: 7, True, 11, (13,True), Just 17, 19, True, 'c'.
Bool/Char and Int/Bool mutations reject for GHC-83865/E0200. T25597 now checks;
replacing only its f signature with Bool rejects in both GHC and candidate.
The first nested-open control omitted its required [[Type]] annotation and was
GHC-invalid; it was corrected before inclusion in the semantic test. A parser-
only CLI attempt timed out on T25597; the final bounded check completed after
the registration and parser-progress repairs. No timing result is banked as a
verdict, and the initial prototype was not reported as fixed.

CLI SHA-256 `7e8ef43fc7528d8d91b0b830c7351713ab807c0fee9e112ebe48ab1cc07e150c`:
the 29-file recheck has only pre-existing tc218 red; all eighteen third-pass
new failures recover. The prior 35-file recheck retains the same eight known
kind-representation losses plus pre-existing HardRecordUpdate. Actual Cargo0:
parser343, naming136, typing352, integration45, record_fields16, canaries7;
zero failures/ignored/measured/filtered in those complete selected targets.
Evidence: `/private/tmp/haskelujah-family-patterns-chirho.4iuhJr/` (reference
pairs, source reductions, before/after logs and set rechecks). This is still
not a fourth full-corpus result or a main landing. Row484 remains open.
The extraction's six inherited clippy findings were repaired. The final scoped
clippy invocation exits0 but still reports parser90, typing53 and four dependency
warnings; none points into the two extracted modules or the changed operator
module. This is not a zero-warning project claim.

### Fourth diagnostic: recovery with one newly exposed family-pattern loss

Checkpoint4391c43f, frozen CLI SHA-256
`3fb5aa79864d676c5ee10df2d71f5fcce0373440919e00dd80c2cbca6f2c4a77`,
completed one full diagnostic per axis with actual wrapper exit0, zero timeouts
or unexpected exits, and unchanged SHA before/after: accept880/938, reject226/767.
Against the third diagnostic: 22 accept recoveries and one new rejection,
T14010. Against main: seven recoveries (PolytypeDecomp, RuleEqs, SplitWD, T14451,
T15079, T20922, tc124), nine new failures (the previous eight kind-representation
losses plus T14010). This is convergence in this pass, not a landable result.
Focused greens had covered the eighteen regressions, not this additional surface.

Reject movement against main is +8/-3, not eight proved capabilities: new
rejections T11356, T11563, T15799, T16502, T23734, T4875, tcfail209, tcfail225;
lost rejections T16512a, T23162b, T23162d. Reasons remain to be checked against
each input's reference stderr; T23162b's old accidental rejection is already
bracketed above. Compare the reject artifact's wrongly-accepted list with the
current accepted list, not with the rejected list: the two artifact bodies have
opposite polarity. Evidence and immutable per-file logs:
`/private/tmp/haskelujah-family-patterns-chirho.4iuhJr/diagnostic-family-chirho.log`
and its `diagnostic-family-accept-chirho` / `diagnostic-family-reject-chirho`
directories. Main121d4f2c and its882/221 artifacts remain untouched; row484 open.

### Promoted occurrence namespace and the next kind-term fork

T10432's reduced source is accepted by GHC9.14.1 but fails on candidate4391c43f:
two promoted Wrap occurrences share one monomorphic kind variable, accidentally
equating independently rigid ka/kb. A focused kind test demonstrated that failure
after repairing two test-construction compile errors. Promoted contracts now have
a separate environment map; known schemes instantiate, absent metadata remains
independent per occurrence. No same-spelled type constructor kind is reused.
Typing353 passes with zero exclusions. Explicit CLI29fa4d1e accepts the unchanged
T10432 and the GHC-confirmed reduction; changing its result to Int still rejects
for GHC-83865/E0200. Full promotion remains unimplemented: the AST drops some
existential constructor annotations, so this repair cannot claim to validate
every promoted use. Evidence: `/private/tmp/haskelujah-kind-terms-chirho.oZJ9Ka/`.
The focused driver regression also passes (one run, 45 filtered intentionally);
its initial compilation required `.err().expect(...)` because the success
bundle is not Debug. This is not a new complete integration/workspace gate.

The T14010 diagnostic is not a tuple-parsing error: two kind-indexed instances
of the same zero-visible-argument family now register correctly, but the reducer
chooses the newest RHS without their erased kind distinction (ArrPair versus
arrow). Do not restore dropped parsing or add an operator-name exception.

Next reversible architecture choice: preserve a kind term separately from the
kind that classifies it. Start with a scoped dependent-function binder, then
nominal/applied/runtime-representation terms and their actual consumers. A
required forall must substitute the supplied kind TERM into its result; an
ordinary arrow cannot do that. GHC accepts the new Shape/Consumer reduction,
while the current CLI rejects it for a rigid-variable/Type mismatch. A separate
runtime-kind reduction is also GHC-valid and currently rejected. Alternatives
(freshening complete contracts or equating all nominal kinds with Type) would
hide invalid programs and are rejected. Confidence is high in the representation
distinction, not in a predicted corpus gain. A checkpoint/tag bounds correction.

Something is wrong with the oversized kind root: move its kind terms,
substitution and unification into one focused child before extending them.
Keep conversion, lexical scope, scheme publication and declaration lifecycle
separate. No new dependency or public deployment; unknown-import metadata is
still a boundary rather than an authoritative contract. Focused positive/negative
GHC controls precede the next full diagnostic; main stays unchanged.
