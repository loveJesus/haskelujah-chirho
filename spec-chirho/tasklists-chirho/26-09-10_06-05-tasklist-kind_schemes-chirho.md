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

Main stays at121d4f2c,882/938 accept and221/767 reject. Row484 and the builder/
canonical DB lease remain open. The isolated branch's latest pushed checkpoint
is15104f0f. Its seventh diagnostic is886/230, one full pass per axis with no
timeouts/unexpected exits, clean HEAD and unchanged CLI digest. Against main:
accept+7/-3, reject+15/-6. T22141f alone recovers from the sixth diagnostic;
T12045a/T13643/T14010 remain accept regressions, so main is held. Next: measured
family-kind reductions before choosing the shared elaboration repair.

Post-diagnostic review reproduced a class-method implicit-kind name leak, absent
from the corpus. The correction is focused-green (typing356, integration68,
canaries7); its fresh CLI digest is recorded at the tail. Do not attribute the
seventh diagnostic to this newer code. The family sources are not modified yet.

Multiplicity correction: parser346,typing356,naming136,integration67,canaries7
green, zero ignored/filtered, actual cargo exits0. Twenty GHC9.14.1/candidate
pairs have19 verdict agreements and the named existing warning-only linear-use
mismatch. Fresh CLI SHA256
`89805de43947c7b30c455eb131de4f4fab846a78cb4688ce9e85fb95fdbc7cc8`.
The scoped clippy run exits0 with existing warnings (driver34), none in the new
arrow module or changed kind child modules. The checkpoint and full diagnostic
are complete; full workspace/two-pass landing gates remain owed.

### Historical prototype baseline

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

### Dependent kind terms checkpoint, not a landing

Tag `dependent-kind-terms-before-chirho` names 6f107c9c. Kind terms,
substitution and equality now live in `kind_chirho/terms_chirho.rs`, with nominal
constructors, applications and dependent functions distinct from inference
metavariables. `type_to_kind` preserves a term; it no longer asks the environment
for the kind OF that term. A required application substitutes its actual term.
Bound positions use de Bruijn indices, separate from globally fresh inference
ids. Two tests exposed flaws in the first id-based prototype: a nested binder
was substituted through, and a local binder escaped into a free meta. Both are
now rejected by scoped substitution/equality; abstraction, argument lifting,
alpha-equivalence and defaulting have explicit controls. No nominal-name exception.

The old unit test expecting `Indexed Maybe` and `Indexed Int` to work when
`Indexed :: Family k -> Type` was itself wrong. The unchanged equivalent source
is GHC 9.14.1-rejected at both applications for GHC-83865. Its assertion now
requires those kind errors; a separately GHC-accepted `Indexed a -> Indexed a`
companion guards valid use. This is not an oracle inferred from our output.
Typing 356 passes with zero exclusions. Driver integration 48/49, canaries 7/7:
the remaining integration error is the concrete TYPE tail in
`signature_elaboration_metavariables_are_not_written_kind_contracts_chirho`.
Do not waive it; runtime-representation interpretation is the next unit.

Explicit CLI SHA-256
`c56160825b8d3a2311677aff78926d156b887fd507c29cb7615eb34230228262`
accepts unchanged CoerceToVDQ. Eight source-hashed GHC/candidate pairs agree on
seven verdicts (dependent and nested-dependent positives, Bool/Int negative,
matching/mismatching family-kind controls, promoted positive/negative). The
runtime-representation positive remains GHC-accepted/candidate-rejected.
Evidence: `dependent-scopes-chirho.jsonl`, `term-scopes-red-chirho.log`,
`term-scopes-typing-chirho.log`, `term-integration-chirho.log` under
`/private/tmp/haskelujah-kind-terms-chirho.oZJ9Ka/`. No new full-corpus figure.

Remaining contracts: AstKind still conflates nominal constructors with variables
and collapses application; runtime TYPE terms and primitive consumers are not
complete. Kind-family application equality/normalization and implicit-kind
equation indexing are also unimplemented. The T14010 reducer has zero visible
patterns in both valid instances, but no hidden-kind key, so it chooses the
newest row. Read-only review confirms the key must survive method schemes and
later instance specialization; a span-to-final-choice table is insufficient.
Implicitly quantified kind patterns and explicit forall uniformity are different
contracts. Do not restore parser loss or presume family application injectivity.

Fourth-pass reject review, source/log inspection rather than a new execution:
T11356/T11563/T4875/tcfail225 are close reason matches; T15799/T23734/tcfail209
are not, and T16502 lacks a reference stderr. T16512a still needs a fresh-main
bracket. T23162b's old wrong reason is measured above. T23162d has no stderr and
its own `all.T` expects successful compilation: this directory-based reject
loss is not lost semantic capability. The frozen denominator is unchanged.

### Nominal/runtime continuation — brick 1

699e6f4a is pushed and remote-exact. Next isolated fork distinguishes a source
nominal kind from a lexical kind variable in AstKind, with naming/dependency/TH
consumers updated together. A named constructor is not implicitly quantified.
Extract the existing kind-annotation lowering into a focused child before
changing it. Runtime kind contracts belong in one kind child, not in per-file
guards: TYPE classifies a runtime representation, a function/field can consume
an appropriate TYPE result, and a boxed-list element still requires Type.
Complete data signatures may determine the runtime result representation but
must not invent missing head arguments. Positive and deliberately wrong
representation controls are required; no complete UnliftedDatatypes/native
execution claim. Confidence is medium on propagation, high on the measured
term/classifier distinction. Local tag `nominal-runtime-kinds-before-chirho`
bounds correction. Main, published axes and the label-policy choice stay held.

### Nominal/runtime checkpoint — focused results, full diagnostic next

- [x] Extract kind-annotation lowering; retain nominal names, qualifiers, source
  spans and applications through naming/dependencies/kind conversion/TH reification.
- [x] Check TYPE representation arguments; admit unlifted runtime value kinds
  without allowing primitive elements in boxed lists or inventing missing head arguments.
- [x] Retain literal Nat/Symbol/Char kinds and promoted-list element kinds;
  update corresponding builtin producer contracts rather than weaken consumers.
- [x] Preserve kind-validation scope lifetime; the existing scope test found a
  temporary classifier binding escaping, and is green after fixing that lifetime.
- [x] Parser344, naming136, typing356, TH16: no failures/ignored/filtered, cargo0.
  Driver typing integration52 and canaries7 likewise green. The prior TYPE-tail
  failure and the two newly exposed Nat-literal failures are repaired, not waived.
- [x] Sixteen source-hashed candidate/GHC9.14.1 pairs agree in both directions;
  includes wrong TYPE argument, wrong nominal kind, missing named kinds in head
  and forall annotations, boxed Int# list, and a missing-head-argument control.
- [x] Full corpus diagnostic completed below; per-file reject attribution remains owed.

Frozen explicit CLI SHA256 is
`b21a2a254304287ee033557628f80475734e8701cbc638773b42c9d79a312497`.
Evidence under `/private/tmp/haskelujah-kind-terms-chirho.oZJ9Ka/`:
`nominal-runtime-final-pairs-chirho.jsonl`, `nominal-runtime-crates-final-chirho.log`,
`nominal-runtime-final-integration-chirho.log`, and the earlier nine-file focused
surface `nominal-runtime-nine-surface-second-chirho.jsonl`. Eight of the prior
nine losses now pass focused checks: CoerceToVDQ/T10432 and all six nominal/runtime
cases. T17817b needed the genuine GHC.Types UnliftedType export; its final CLI
check is in `t17817b-final-chirho.log`. T14010 remains the hidden kind-indexed
family-reducer defect described above. These focused results do not establish
the absence of new corpus regressions. Main remains121d4f2c (882/221), row484 open.

Two old test contracts changed with the representation: a parser assertion
requiring an applied kind to be discarded now requires the retained application;
builtin literal-family expectations now state Nat/Symbol/Char rather than Type.
New driver controls independently checked by GHC constrain the resulting behavior.
Incomplete grammar for list/promoted binder annotations, imported authoritative
kind metadata, local Type/Constraint alias shadowing, kind-family equality,
representation-polymorphic calling rules and native unlifted execution are not
claimed. Existing oversized roots/directory and lint debt are not declared fixed.

### Fifth full diagnostic — a615988f, still not landable

The checkpoint is pushed and remote-exact. One full pass per axis completed
with wrapper0, zero timeouts/unexpected exits, and unchanged clean HEAD/CLI hash:
883/938 accept,232/767 reject. Relative to main882/221, accept gains remain
PolytypeDecomp/RuleEqs/SplitWD/T14451/T15079/T20922/tc124; losses are now
T12045a/T13142/T13643/T14010/T15428/tc167. Eight previous regressions recovered
and five new ones appeared. A higher total does not satisfy the no-loss gate.

Reject +17/-6 (reason attribution still owed, none banked): gained
T11356/T11563/T15799/T15801/T16502/T16821/T22645/T23734/T24553/T4875/T7368a/
T9634/UnliftedNewtypesInfinite/VisFlag1/VisFlag1_ql/tcfail209/tcfail225;
lost ExplicitSpecificity3/T12803/T16512a/T23162b/T23162d/T5853.
Evidence: `diagnostic-nominal-chirho.log` and the two diagnostic-nominal axis
directories under the same scratch root. DONE posted after actual completion.

Next small consumer unit: preserve qualified-import identity for K.Type,
give prefix (->) its runtime-representation-polymorphic kind, and reconcile
arrow-kind syntax with the same constructor-application term shape. Each needs
an independently GHC-checked positive and negative; no weakening a rigid or
nominal head. Remaining family cases require retained visible kind applications,
kind-family normalization/injectivity and hidden family keys, not file guards.

### Consumer-boundary correction — focused, not a new full measurement

- [x] T13142, T15428 and tc167 recover on the explicitly rebuilt CLI. T12045a,
  T13643 and T14010 remain failing on the family boundaries described above.
- [x] Qualified builtin kinds resolve through declared aliases. Prefix (->)
  accepts primitive domains; flexible application kinds can unify with arrows
  without permitting a nominal Maybe head to become a function kind.
- [x] A new GHC control exposed inferred RuntimeRep defaulting, not a valid
  unboxed identity: `type A = (->) Int#; identity :: A Int#` is rejected by GHC.
  Preserve that negative, default unnamed representation/levity holes at group
  publication, and retain written TYPE r parameters. Explicit fully applied
  `(->) Int# Int#` and the partial alias used with lifted Int both remain valid.
- [x] Twenty-seven source-hashed GHC9.14.1/candidate pairs now agree; no child
  timeout or unexplained exit. Typing356, driver integration55 and canaries7
  pass with zero ignored/filtered. These are focused checks, not corpus gates.

CLI SHA256 `a185d3a2a898daa3ed0a190e622f987a48b5c5eec7f45ea511241963ac6a51eb`;
same scratch root, `consumer-defaulting-after-chirho.jsonl`,
`consumer-boundaries-six-chirho.jsonl`, `consumer-boundaries-typing-chirho.log`,
`consumer-boundaries-integration-chirho.log`. The earlier before/after records
retain the initially invalid primitive-arrow probe and the defaulting mismatch;
they were not overwritten. No revised full-axis number is claimed from six checks.

Next representation vertical: kind-family applications must retain supplied
kind arguments and support equality/reduction without treating family heads as
injective constructors. T12045a needs explicit @ arguments and closed reduction;
T13643 needs the declared injectivity contract retained, not assumed; T14010
additionally needs hidden kind indices to survive into value-type reduction.
These are compiler representations, not six filename-specific conditions.
Main and its 882/221 artifacts remain untouched; row484 and the builder lease
stay open. No site deployment or label-policy choice is inferred.

Scoped clippy (`typing`, `driver`, lib targets) completed successfully, but
existing warnings remain, including driver35. Its log reports none in the
changed kind child modules; this does not claim a warning-free workspace.

### Reviewed boundary fork — 0d089454 pushed, main still held

The checkpoint is remote-exact at 0d089454. Read-only review supplied three
hypotheses; fresh paired controls confirm the dependent declaration-head loss
and TYPE Many wrong acceptance. The proposed instance control with explicit
PolyKinds/FlexibleInstances does not reach the old instance check, so its
positive already passes and its invalid Int instance also passes. Removing
those flags without changing the default edition reaches the old check and
reproduces the false rejection of a valid runtime-polymorphic instance.
Evidence: `review-extended-boundaries-before-chirho.jsonl` in the same scratch root.

Recommendation before family elaboration: preserve dependent head binder
identities and reconcile a complete kind one binder at a time; seed the finite
runtime constructors' actual classifier contracts; use scoped, instantiated
kind equality at the existing local-instance boundary. Do not broaden the
unsupported-instance-syntax guard by extension name in this unit. Confidence
is high in the reproduced controls, medium in the integration. Tag
`kind-consumer-audit-before-chirho` bounds this local reversible fork.
The existing large root and environment-copying instance checker are structural
debt: extract the touched instance consumer rather than add more there; group
its existing child tests if needed to keep the directory bounded. No new dependency.

### Reviewed boundary results — focused repairs, no new corpus claim

- [x] A parser regression was demonstrated red before repair: the standalone
  lowerer discarded a nested forall binder's `::`, losing its annotation.
  Consume only the declaration delimiter; the nested annotation now survives.
- [x] Complete dependent data/newtype heads consume their contract one rigid
  binder at a time. A valid dependent head is accepted; specializing its second
  binder to Type is rejected, matching GHC9.14.1.
- [x] Finite RuntimeRep constructors have their actual classifiers. TYPE Many,
  a Bool in TupleRep, and swapped VecRep arguments reject; their positive
  counterparts accept. Tuple/vector controls now live in driver integration,
  not only in scratch. This is not a native unboxed/vector execution claim.
- [x] Extract the local-instance consumer from the large kind root. Each use
  instantiates its class kind and unifies in a journaled scope; no whole-env
  cloning or old-kind-shape equality. The existing extension guard is unchanged.
- [x] Parser345 and typing356 pass with the documented 16MiB Rust test-thread
  stack; driver integration60 and canaries7 pass. Zero ignored/filtered,
  actual cargo exits0. The first parser attempt without the stack setting
  aborted at the nested-parentheses property and is retained as a failed run.
- [x] Thirty-nine source-hashed GHC/candidate pairs completed:38 agree and one
  pre-existing mismatch remains. WrongRuntimeKindInstanceChirho is wrongly
  accepted with explicit PolyKinds/FlexibleInstances because the old guard
  bypasses that head; removing those flags reaches the repaired check. This is
  not a 39/39 claim or complete instance-kind validation.
- [x] Freeze this checkpoint and run a fresh full diagnostic; main stays held.

Evidence under the same scratch root: `standalone-binder-before-chirho.log`,
`review-boundaries-crates-stack-chirho.log`,
`review-boundaries-permanent-integration-chirho.log`,
`review-boundaries-final-pairs-chirho.jsonl`, `review-boundaries-clippy-chirho.log`.
Scoped clippy exits0 with existing warnings (driver35), none reported in the
changed kind child modules. No warning-free or file-size-compliant workspace
claim: the existing large parser/kind roots remain debt. Dependent abstraction
is scoped to the declaration, but repeated suffix traversal is not claimed
linear for arbitrarily long telescopes. That growth boundary remains a follow-up.

### Sixth full diagnostic — 5fcb76a4, still held

Checkpoint pushed and remote-exact. One full pass per axis, wrapper0,
zero timeouts/unexpected exits, clean HEAD and explicit CLI digest unchanged:
885/938 accept,230/767 reject. CLI SHA256
`fbb312dfcb5fccea40374e28f2773c660486f582f589aaf94923e2ac1f700861`.
The 39 reference pairs were also rerun on this digest (same38 agreements and
one known instance-guard mismatch). Evidence: `review-boundaries-frozen-pairs-chirho.jsonl`,
`diagnostic-consumer-chirho.log` and the diagnostic-consumer axis directories.

Against main882/221, accept+7/-4: the seven gains remain unchanged; T13142,
T15428 and tc167 recover. Remaining losses T12045a/T13643/T14010 plus the new
T22141f. Reject+15/-6: the fifth diagnostic's gained list loses VisFlag1 and
VisFlag1_ql; all six lost rejections remain. These verdicts are diagnostic,
not banked capabilities; per-file rejection-reason arbitration remains owed.

T22141f reports Multiplicity as a function argument at `%'One`/`%'Many`.
Source inspection finds the multiplicity scanner accepts one unquoted token
but not a promotion tick, then recursively parses the remainder as a type.
Next bounded repair: reduce both spellings against GHC, preserve the arrow
annotation in its own slot, and prove the following declaration still survives.
Do not remove the classifier contract that exposed the parser defect.

The reduction confirms three valid spellings rejected (quoted, parenthesized,
and family-applied multiplicity), plus two old wrong accepts (%2 and a Bool-kind
multiplicity variable). Unquoted One/Many without DataKinds is a separate
pre-existing wrong accept; the plain probe is retained as a negative, not
relabeled from our output. The new parser boundary test is demonstrated red.

Placement decision: wrap the multiplicity atom in its own CST child, reuse the
normal atomic type grammar, and retain nontrivial annotations as an explicit
AST type expression. Extract arrow lowering into a focused lower child; naming,
dependency collection, lexical quantification and kind checking visit that
expression. Keep fixed One/Many conversion in one helper used by both type
converters. This is syntax/classifier fidelity, not complete linear-use checking,
multiplicity-family reduction, or TH reification. Checkpoint tag
`multiplicity-syntax-before-chirho` at5fcb76a4 bounds the reversible correction.

### Multiplicity boundary correction — focused implementation

- [x] Dedicated CST multiplicity child reuses the atomic type grammar. Arrow
  lowering moved to `lower_chirho/type_arrows_chirho.rs`; argument, annotation
  and result retain their separate roles. The previously red parser test now
  retains the quoted/parenthesized annotation's exact source slice and the next
  declaration. Only bare `%1` is numeric sugar; `%2`/`%(1)` require classification.
- [x] Replace the unused name-only MultVar carrier with an explicit AST type
  expression. Naming, signature-binder collection, dependency collection and
  kind substitution visit it. Arrow classification requires Multiplicity.
- [x] GHC controls exposed two adjacent authority boundaries before freezing:
  GHC.Types' interface omitted One/Many members, while seeded builtin constructor
  kinds could outrank local promoted constructors. Repair the member inventory
  and preserve separate ordinary/promoted kind lookup. A type alias named One
  may mean Many; never infer the fixed-linear bit from that alias's spelling.
  A nullary local enum constructor has its owner's classifier; missing metadata
  for other constructor shapes remains opaque, not a builtin inherited by name.
- [x] T22141f checks successfully on the explicit candidate CLI. This is one
  focused recovery, not yet a new corpus count.
- [x] Final reference pairs and crate/integration gates:20 pairs,19 agreements,
  parser346/typing356/naming136/integration67/canaries7, no exclusions and actual
  exits0. The two new identity sources also produce GHC's independently measured
  42 through STG,LLVM and Cranelift. Final CLI SHA256 is recorded above.
- [x] Scoped clippy exits0 with existing warnings (driver34), none in the new
  arrow module or changed kind child modules; not a warning-free workspace claim.
- [x] Commit/push only the isolated checkpoint, then compare a full diagnostic.

The existing linear-use checker emits warnings, not rejecting diagnostics.
The reference control that duplicates a quoted-One argument still disagrees
with GHC. Full multiplicity variables/family reduction in internal types,
unquoted-alias elaboration and TH reification remain outside this correction.
The AST now retains those expressions for their actual kind/scope checks; this
does not make the two-valued internal multiplicity model complete.

Evidence is under `/private/tmp/haskelujah-multiplicity-chirho.vhiDs4`:
`before-chirho.jsonl`, `parser-before-detailed-chirho.log`,
`after-chirho.jsonl`, `namespace-after-chirho.jsonl`, final pair/gate logs.
Earlier stages are retained as observations, not overwritten by the repair.

### Seventh full diagnostic — 15104f0f, still held

Checkpoint15104f0f62977861e6fdf7b064587826ed712db7 is pushed and verified by
`ls-remote`; main121d4f2c remains clean and remote-exact. The wrapper actually
exited0. One full pass per axis gives886/938 accept and230/767 reject, with
zero timeouts/unexpected exits, a clean frozen HEAD, and the recorded CLI digest
unchanged before and after each axis. This is diagnostic, not the two-pass
landing gate. Logs and complete sets are in the multiplicity scratch root's
`diagnostic-chirho.log` and `diagnostic-{accept,reject}-chirho/` directories.

Set comparison against the sixth diagnostic: T22141f is the sole recovered
accept, no new accept failures, and the rejected set is byte-identical. Against
main the seven accept gains remain PolytypeDecomp, RuleEqs, SplitWD, T14451,
T15079, T20922 and tc124; the three losses are T12045a, T13643 and T14010.
Reject movement remains+15/-6 as listed in the sixth diagnostic, with per-file
reason arbitration owed before any capability is banked.

### Independent review — class-method scope correction

The review supplied three falsifiable probes. All were executed under GHC9.14.1
and the explicit15104f0f CLI before any fix. A local non-nullary constructor
used as a multiplicity and a wrong local where-signature multiplicity both
remain wrong accepts. These are measured missing promoted metadata/local kind
checking, not demonstrated newly introduced regressions.

The third probe is a real candidate wrong rejection: one class method's free
`multiplicityChirho` was reused by the next method. GHC accepts both method
orders, and merely renaming the second method's variable made the candidate
accept. The new permanent driver control first failed with E0300 on that exact
application, then passed after adding one signature-local kind scope. Shared
class-head constraints still accumulate: a class parameter used at incompatible
kinds across two methods remains rejected by both GHC and the candidate.

Scope entry uses the existing environment journal and the current fresh-id
boundary. Cleanup retains outer identities and undoes local bindings, never
clones the module environment or resets its substitution. Explicit binder
shadow restoration remains owned by the existing binder helper.

- [x] Demonstrate the new driver test red, then green on the repair.
- [x] Six GHC/candidate pairs before/after: both method orders recover, renamed
  control stays accepted, shared-class contradiction stays rejected. The two
  independently named unsupported cases remain wrong accepts (4/6 agreements).
- [x] Explicit CLI build; typing356, integration68, canaries7 pass with actual
  cargo exits0 and no ignored/filtered tests. CLI SHA256:
  `8687e146d81b770d96049ee48427144916afda40771b26c0ca9977638d5b5ef9`.
- [ ] Commit/push the correction and compare the next frozen full diagnostic.

Evidence: multiplicity scratch root's `audit-chirho/six-{before,after}-chirho.jsonl`
and `method-{red,after,typing,integration,build}-chirho.log`.
Scoped clippy exits0 but still reports52 typing warnings (same count as the prior
multiplicity checkpoint); none points at the changed scope helper or method loop.
This is not the project's required zero-warning state.

Family-kind preparation is measured, not implemented: eleven paired reductions
under `/private/tmp/haskelujah-family-kinds-chirho.iAIit3` distinguish closed
forward reduction, injective improvement, non-injective ambiguity, explicit kind
application, implicit kind-pattern dispatch in both equation orders, overlapping
equations, and explicit-uniform versus implicit-matchable result quantifiers.
The negative controls demonstrate additional existing gaps; no family capability
is claimed from the current table. Checkpoint tag`family-kinds-before-chirho`
atdfff0b4d records the state before this preparation and subsequent scope review.
