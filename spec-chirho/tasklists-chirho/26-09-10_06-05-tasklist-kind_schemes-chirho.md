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

Main stays at121d4f2c,882/938 accept and221/767 reject. Row484 is unfinished;
SLOT and sole canonical DB writer remain ours. Runtime identity was verified
as HASKELUJAH/gpt_chirho@HASKELUJAH:3:%154; no competing broker lease appeared.
No DB write, main merge, public artifact change or deployment in this continuation.

Latest corpus diagnostic43833550: accept881/938, nine main-relative regressions
and eight gains; reject248/767, zero timeouts or unexpected exits. One pass per
axis, not a final gate. Source checkpoint is pushed and remote-exact.
Typed family clauses retain hidden inputs, published identities and list kinds.
Prior driver--lib1760/1772: twelve failed on parent/current but PASS fresh main.
Thirteen known Prelude class kinds restored; T20356 and its positive/negative/
shadowing controls recover. The following isolated repair enforces ConstraintKinds
on solved alias kinds, not use-site accidents. Raw pragma directives now retain
edition provenance; selected defaults precede explicit flags in a shared lexer/
lowerer normalizer. Seventeen GHC9.14.1 verdict controls agree; parser3/3 new
controls, parser356,typing369,naming136,canaries7 and workspace check pass.
Integration118/119 retains the annotation red. The license/edition diagnostic
holds the accept set byte-identical and gains only tcfail209 for GHC-75844.
Nineteen frozen reference observations agree. The next imported kind/closed-alias
repair recovers those twelve driver tests in a focused run, plus two transport
controls (14/14,1760 filtered). Typing371, new import controls4/4 and canaries7
pass. Full integration121/122 retains the annotation red; full driver and a new
corpus diagnostic remain owed. Checked templates/closed aliases travel beside
naming's interface, preserving fresh IDs and private dependency origins.
Current implementation/gates: kind-schemes-chirho/26-09-12_01-47-import_contracts-chirho.md.
Detail/reference evidence and next hidden-family-input decision:
kind-schemes-chirho/26-09-11_22-55-family_occurrences-chirho.md. The earlier classifier
repair and accidental UnliftedNewtypes rejection audit are in the22-20 leaf.

Visible-application checkpoint3bbcd49c is committed, pushed and remote-exact.
Its recursive/local nominal follow-up f5c4eedb is also pushed and remote-exact;
T12045a recovers, but twelve other accept files regress against b362f326. Kind-index
capture is keyed to the final quantified identities, including monomorphic SCC
uses; local expression signatures instantiate known nominal slots. Family/synonym/
imported index consumers and complete local classifier checking remain unfinished.

The new local-synonym repair is frozen and pushed after typing366, driver integration104
and canaries7, zero ignored/filtered. Twenty-seven bounded GHC9.14.1/candidate
observations agree, six in execution mode, with sources and binary hashes retained
beside the visible-application fixtures. The diagnostic confirms recovery of
DeepSubsumption02, GivenTypeSynonym, LocalGivenEqs and tc151, but newly loses
T12928 and tc160. Reduce those two before the next family-equation consumer repair.
The synonym-identity/classifier follow-up recovered T12928/tc160 but introduced
the5917c4fd failures above. Do not infer landability from its focused greens.
Full workspace tests, final two-pass gate and DB closure remain owed.

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
- [x] Commit/push the correction and compare the next frozen full diagnostic.

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

### Eighth diagnostic and usage-limit handoff — c838387e

The frozen wrapper actually exited0. Accept886/938 and reject230/767, with
zero timeouts/unexpected exits and SHA256
`8687e146d81b770d96049ee48427144916afda40771b26c0ca9977638d5b5ef9`
unchanged before and after each axis. Four explicit `diff -q` comparisons
(accepted and rejected lists on both axes) against the seventh diagnostic
produce no differences. This is one full pass per axis on c838387e, not a
two-pass landing gate on that commit. Git status is clean; `ls-remote` matches
both the candidate c838387ef2b3ab1b70b15781320e10c15c2984ba and main
121d4f2c33e28b629428bd7ac7da17a4462491cd before this docs-only handoff.

Evidence: `/private/tmp/haskelujah-multiplicity-chirho.vhiDs4/diagnostic-method-chirho.log`
and `diagnostic-method-{accept,reject}-chirho/` in the same root. The 15 gained
rejects still require per-file reason arbitration; the delegated reader hit its
usage limit and returned no usable review. Do not count that attempted review
as completed evidence. The three accept regressions remain T12045a, T13643 and
T14010; the seven gains are PolytypeDecomp, RuleEqs, SplitWD, T14451, T15079,
T20922 and tc124. No published artifact, label policy, main source or DB changes.

#### Durable continuation packet

- project_identity_chirho: HASKELUJAH
- work_item_id_chirho: progress-chirho/484
- measured_state_chirho: c838387e pushed, eighth diagnostic886/230, three accept regressions.
- blocker_chirho: autonomous goal usageLimited; review agent explicitly exhausted its provider allowance.
- authority_class_chirho: delegated local/reversible compiler work; no deployment authority.
- recommendation_chirho: preserve this checkpoint, release leases, resume the family vertical when execution allowance is available.
- confidence_chirho: high in frozen counts and set equality; family design remains unimplemented.
- alternatives_chirho: another authorized worker may take the documented isolated branch after reacquiring leases; main remains usable.
- reversibility_chirho: main untouched; code checkpoint and family-kinds-before-chirho tag retained.
- correction_cost_chirho: resume/reverify Git, broker ownership and fresh binary before further work; no rollback is needed.
- evidence_references_chirho: diagnostic-method logs/sets and eleven family GHC9.14.1/candidate pairs named above.
- checkpoint_ref_chirho: gh_chirho/gpt-kind-schemes-chirho at c838387e plus this docs-only handoff.
- safe_work_while_waiting_chirho: read-only review of the 15 reject gains; no automatic builds, DB writes or provider retries.
- human_action_if_required_chirho: restore execution allowance or explicitly route the continuation to an available authorized worker.
- human_completion_evidence_chirho: successful authorized execution and freshly reacquired SLOT/DB, not a liveness ping.
- completion_failure_input_chirho: any one of the three previously accepted files still rejected forbids landing; any timeout/unexpected exit invalidates a gate.

Proposed next brick (not implemented): preserve closed-family result binder,
optional result kind and injectivity annotation in the AST, validate equation
contracts before trusting inverse improvement, and share ordered equation
matching/reduction policy between kind and type consumers. Begin with the
ForwardKindFamily/InjectiveKindFamily controls and non-injective/invalid-
injectivity negatives. Hidden kind-pattern dispatch (T14010) and explicit kind
application (T12045a) require retained semantic arguments, not per-file guards.
Record that placement decision before editing; the prior read-only design review
does not substitute for a compiling, behaviorally verified implementation.

### Family vertical placement decision

Recommendation: start with local closed-family normalization and validated
injectivity, while preserving the head's actual result binder/kind/annotation
and whether the declaration is closed. Retype the AST result slot so its current
kind consumers must be revisited. Extract CST family parsing to its own child
module; do not add another large arm to the oversized parser root. Share the
ordered matching policy between Ty and Kind through a small term adapter under
typing/types_chirho, rather than implement two family solvers. Group the four
existing kind test files before adding the family consumer to that directory.

Confidence: medium in the complete integration, high in the eleven independently
measured reductions. Alternative of reducing only declaration syntax loses
instantiation-dependent reduction; treating a stuck family as a fresh kind loses
soundness. Inverse improvement must use validated injectivity only. Unsupported
hidden kind arguments remain explicit follow-on work, not invented arguments.
Checkpoint family-vertical-before-chirho at ea72bb12 bounds this reversible fork.

- [x] Demonstrate forward reduction, injective improvement and invalid annotation controls red.
- [x] Retain family result metadata and equation form through parser/naming consumers.
- [x] Share ordered equation matching and bound kind-family output growth without treating stuck applications as nominally injective.
- [x] Validate represented first-order injectivity before improvement; preserve the non-injective negative.
- [x] Re-run the eleven reference pairs and UnliftedNewtypesDifficultUnification control.
- [x] Focused crate/integration tests, then frozen diagnostic before any landing claim.

### Family foundation — measured before the next diagnostic

The three new driver controls actually failed0/3 before implementation, then
passed3/3: closed forward reduction, validated injective improvement and an
invalid annotation rejected without uses. A subsequent full integration run
found a real regression in the existing non-injective repeated-use control;
GHC9.14.1 accepts its exact source. Fresh occurrence matching fixes it without
cancelling a family on written/rigid variables. The explicit leading forall
also stays quantified in an otherwise inferred data head, rather than being
specialized by its first GADT constructor.

Claude2's read-only review supplied a real numeric/named variable-key collision.
The adapter now uses an enum, with a direct reduction control. Its two forall
probes are both illegal GHC equations (GHC-91510), not missing substitution
contracts; both were candidate wrong accepts and are now diagnosed. A separate
probe found a real unsound annotation: `Erase a = Bool` was nominally retained
on an injective RHS. Synonyms now expand before validity/validation; the erasing
case rejects and the independently GHC-accepted `Maybe a` counterpart passes.

The normalizer charges expanded output nodes before copying a duplicating RHS.
The growth control reaches a specific work-limit result rather than allocating
until the OS kills it. This is a local kind-family bound, not a claim that the
whole type checker or synonym expansion is bounded. Opaque/nested-family terms
and verification-budget exhaustion never certify injectivity.

Fresh focused results: parser348, naming136, typing362, integration73, canaries7,
all actual cargo exits0 with zero ignored/filtered tests. Seven focused parser
family tests also retain the result binder/kind/dependency and distinguish
closed-empty from open families. The explicit CLI build has no compiler warning.
Final CLI SHA256: `a0e1025f98374c25824a1397b1df53f5b71a9283d94312c20591fd8ebf474318`.

Seventeen paired GHC9.14.1/CLI reductions have11 verdict agreements and6 named
unfinished contracts. The original11 now agree on forward/injective/invalid-
injectivity/non-injective/implicit-matchable cases; explicit kind application,
hidden pattern dispatch in both orders, invalid explicit kind application,
explicit-uniform open-family checking and overlapping open equations remain
unfinished. The six additional review/alias/shared-use sources all agree.
Focused corpus: T13643 recovers, T12045a/T14010 remain red, and the already-green
UnliftedNewtypesDifficultUnification remains green. No full-count prediction is
banked from those four checks.

Scoped clippy exits0 with146 warning messages across dependencies: diagnostics3,
syntax1, parser88, typing54. Two are new enum-postfix style warnings that conflict
with the mandatory Chirho naming rule; no suppression or gratuitous public API
was added to hide them. The new collapsible-if and const-mutation warnings were
fixed. This is not zero-warning project completion; existing oversized roots
and structural debt also remain. Family CST parsing was extracted and four kind
test files grouped before adding focused consumers; new implementation files
remain below400 lines.

Evidence: `/private/tmp/haskelujah-family-kinds-chirho.iAIit3/` contains driver-
before/after logs, crate/integration gates, clippy JSONL and the11-pair
`foundation-after-chirho.jsonl`. `/private/tmp/haskelujah-family-review-chirho.09U6QK/`
contains the six additional sources, the measured erasing-alias wrong accept,
and final pairs. Main/DB/published artifacts remain unchanged. Next: freeze this
foundation and compare complete sets before adding explicit/hidden kind inputs.

### Ninth diagnostic: family foundation, not landable

Frozen58795e47, explicit CLI SHA256a0e1025f98374c25824a1397b1df53f5b71a9283d94312c20591fd8ebf474318:
accept877/938 and reject235/767. Both complete runs exit0 with zero timeouts
or unexpected exits. Clean Git HEAD and binary hash held before/after each
axis; the outer runner emitted COMPLETE and exited0. This is one diagnostic
per axis, not the final two-pass landing gate. Embargo released in22099;
SLOT/DB retained, no main/artifact/DB writes.

Compared with eighth886/230: accept+T13643; -CoerceToVDQ, T10776, T13248,
T16995, T17594f, T20241, T22560d, T22762, T26358,
type_in_type_hole_fits. Reject+CustomTypeErrors05, T10836, T11623, T12430,
T23162c, T6018failclosed; -T16821. The latter changes are not yet certified
as correct-reason capabilities. Main stays unchanged.

Evidence: `/private/tmp/haskelujah-family-kinds-chirho.iAIit3/diagnostic-foundation-chirho.log`
and its `diagnostic-foundation-{accept,reject}-chirho/` sibling directories.
Reduce equation-local scope and wildcard classification first; the displayed
errors also include nested injectivity, invisible binders, dependent kinds
and closed-family apartness. Do not collapse those into a single hypothesis.

### Family follow-up: shared producer repairs and reference arbitration

Equation rows now hide declaration-head names in a journaled scope. Their
variables are fresh even when spellings match a header. Anonymous patterns get
independent kind classifiers and independent stored type variables, not Type
or a fabricated constructor. Open/closed/associated rows share one converter;
the relevant 168-line converter moved out of the oversized inference root.
Family head lowering retains bare and annotated invisible binders with the
shared binder parser, but only visible binders contribute ordinary arity.
An initial draft lost the first binder of an infix family; the full parser
gate caught it (348/349), and the existing assertion was kept while fixing it.

Family normalization can erase a genuinely unused dependent binder, shifting
surviving outer indices rather than dropping their scope. A negative preserves
a live dependency. StarIsType is unqualified syntax, independent of qualified
Nat multiplication and disabled by NoStarIsType. Two unchanged driver tests
exposed that classifier bug. Three other driver failures were invalid inputs:
local TYPE was used as the built-in (GHC-83865 once DataKinds is enabled), and
two percent-Many tests omitted its import. Corrected inputs preserve intended
results; the TYPE test moves to integration with a local-shadow negative.

Final explicit CLI SHA256:
`a5818688a41dc5739d6cb34d0c3e82cf3df435fe6d09955a06ddf73080ef3336`.
Eighteen fresh GHC9.14.1/candidate checks agree on verdicts, with source hashes
and diagnostics in `test-data-chirho/kind-oracles-chirho/family-followup-chirho.jsonl`.
These are typecheck observations, not eighteen execution claims. Current
parser349/naming136/typing363/integration79/canaries7 pass with zero exclusions;
full driver1772 passes (1104.48s, three test threads, actual Cargo0).
That count moves one test from lib to integration, not to an ignore/filter.
Format check passes. All-target scoped clippy exits0
with559 warning messages (417 distinct JSON diagnostics), not zero warnings.

Seven focused recoveries: T10776, T16995, T17594f, T20241, T22560d, T22762 and
type_in_type_hole_fits. CoerceToVDQ/T13248/T26358 still fail; T13643 stays green,
T12045a/T14010 remain red. An executed AST/kind dump shows CoerceToVDQ's data
family already becomes TypeFamilyDeclChirho, but its full standalone signature
is not attached. Do not confuse that missing head contract with an inline
result kind or infer a dropped declaration from an unrelated scanner branch.

The fifteen upstream expect_broken candidates were independently run unchanged
under bounded normal GHC9.14.1 fno-code: thirteen reject, T10770b/T14761c pass.
`upstream-expectations-chirho.jsonl` records hashes/diagnostics. These markers
name known GHC issues, not invalid Haskell. T14761c is broken only in named
coverage/profiling/optimized ways; InstanceGivenOverlap2 uses compile_fail.
No denominator, membership, label policy, main or DB changes follow from this
audit. These repairs are still an isolated checkpoint, not a final landing.

### Complete family-head contract, reversible next brick

45c4a108 is pushed and remote-exact; local tag family-head-contract-before-chirho
bounds this next change. Reuse the existing complete/result declaration-kind
shape under the general name DeclKindSigChirho, and retype the family's kind
slot to carry it. A complete family kind is not its result annotation and
does not determine reduction arity. Naming visits the signature independently
of header binders; dependency scheduling retains both contracts. Extract the
existing complete-head binder reconciliation for data and families to share,
while keeping data-only return-kind rules and ordinary family inference distinct.
Confidence medium; reversal is the isolated checkpoint. No new dependency,
data-family-instance representation, higher-rank family reduction or publication
authority is implied. Prove source retention, a dependent data-family head,
contradictory result kinds and missing type names before another full diagnostic.

The two new driver tests were red0/2 before this repair and green2/2 afterward.
Further controls preserve both written spans, distinguish standalone forall
scope from header scope, allow a zero-argument family returning Maybe, and allow
a Constraint-valued family while rejecting a contradictory inline arrow kind.
Seven fresh GHC9.14.1/CLI verdict pairs agree; the four rejections also name the
same contracts/locations. Evidence: kind-oracles-chirho/family-head-contracts-chirho.jsonl.
Parser350, naming136, typing363, integration82 and canaries7 pass, zero exclusions.
Explicit CLI SHA25634dea8604503cc7c4cef3e819e8f1f0f49270ae29f506ea84a777a67e59bd42f.
CoerceToVDQ checks successfully on that CLI; this is not a full-corpus count.
Clippy on parser/naming/typing all-targets exits0 with596 warning messages,
417 distinct JSON diagnostics; no zero-warning claim. The prior full driver1772
belongs to45c4a108, before this head repair. Freeze and full diagnostic are next;
main, published artifacts, the rounding decision and canonical DB remain held.

### Tenth diagnostic, complete and still not landable

Frozen2311cd9f and CLI34dea8604503cc7c4cef3e819e8f1f0f49270ae29f506ea84a777a67e59bd42f:
accept883/938, reject238/767, actual outer runner0/COMPLETE; both axes have
zero timeouts/unexpected exits, clean HEAD and unchanged binary before/after.
Versus ninth: +CoerceToVDQ/T10776/T16995/T17594f/T20241/T22560d/T22762/
type_in_type_hole_fits; -T12919/T14366. The full set confirms the eight
recoveries but exposes two equation-consumer gaps the focused controls missed.
T12919 reports an unreduced VC classifier; T14366 cannot consume a required
dependent argument. These need measured reductions, not signature removal.

Reject+ExplicitSpecificity3/T12803/T15552a/T18640a/T18640b/T5853;
-CustomTypeErrors05/T22645/T24553. Reasons are under separate read-only review;
six new errors are not six banked capabilities. Main-relative accept+7/-6:
T12045a/T12919/T13248/T14010/T14366/T26358 remain regressions.
Evidence: /private/tmp/haskelujah-family-head-chirho.mGuVQD/diagnostic-head-chirho.log
and diagnostic-{accept,reject}-chirho/ in that root. The embargo was released
in22109; SLOT/DB retained. This is not a two-pass landing gate or main change.

### Equation consumers and promotion follow-up

- [x] Three reduced valid sources fail on2311cd9f and pass the repaired candidate.
- [x] Preserve actual scheme quantifiers per row and consume required arguments by term.
- [x] Retain promoted GADT result indices and share explicit-promotion type identities.
- [x] Arbitrate positive and negative controls with GHC before fixing the expectation.
- [x] Parser350/naming136/typing363, integration85 and canaries7; zero exclusions.
- [x] Freeze, commit/push owned paths and run the next complete diagnostic.

The three reductions exposed more than the initially suspected consumer: GADT
promotion had no result-index contract, and stored equations and signature
conversion disagreed about the promotion prefix. The shared telescope consumer,
actual scheme instantiation and checked GADT classifier repair those producers.
Homogeneous equality's imported Refl classifier expresses reflexivity; the same
spelling on a local GADT keeps its own contract. No data-family instance or
unsupported context is fabricated. A duplicate homogeneous-equality seed in an
intermediate draft was removed; its existing binding now lives with the promoted
classifier. Two existing digit-grouping warnings in that touched block were fixed.

One important refuted prototype: rigidifying all equation variables before the
RHS rejects valid Haskell. GHC accepts both Pick x = Int under forall k. k -> k
and Cast _ _ Refl x = Int, inferring indices. The restriction was removed and
those controls now prevent its return. Explicit Bool indices reject Int. Any
visible-only reduction must prove hidden inputs unconstrained after the whole
row; only wholly variable patterns qualify, with no unbound RHS variables.
This is a bounded uniformity proof, not complete hidden-index reduction.

The indexed fixture originally reused WitnessChirho as its constructor and type
name without a quote. GHC rejected it; it was repaired to OnlyWitnessChirho
before being used as a positive oracle. All nine final pairs agree in verdict;
the two negatives report the same kind contract, not necessarily identical
diagnostic ordering. Sources, hashes and diagnostics are retained in
test-data-chirho/kind-oracles-chirho/family-equation-contracts-chirho.jsonl.
Full driver1772 still belongs to45c4a108, not this follow-up. No execution count,
full workspace result, published count, DB closure or main merge follows here.
Final explicit CLI73e3e94341fbc57fc5feb82a46ca1b7565d3f6bdc2dcba6f2086fda6666217aa;
nine reference pairs rerun on that binary, actual runner0. Final typing363,
integration85 and canaries7 rerun, actual Cargo0 and zero exclusions. Format and
diff checks pass. Scoped all-target clippy exits0 with590 warning messages,
414 distinct JSON diagnostics, not zero warnings; three prior digit-grouping
diagnostics disappear through the touched seed cleanup. T12919/T14366 recover
in fresh focused checks; T12045a/T13248/T14010/T26358 remain red.

### Independent ninth-to-tenth reject audit (Claude2,22111)

The first-error comparison finds matching contracts only for T18640a/T18640b.
T15552a has the right verdict for adjacent nested-family validity; T12803 and
T5853 have unrelated kind errors; ExplicitSpecificity3 is rejected by GHC at
parse time but by the candidate later in kind checking. Thus six count gains
are not six capabilities. Lost CustomTypeErrors05/T22645/T24553 rejected on
different lines and rules from GHC; do not restore those accidental reasons.
This is a source/log audit, not proof that every later diagnostic agrees or
that GHC's every rejection establishes invalid Haskell. Original evidence:
/private/tmp/haskelujah-family-head-chirho.mGuVQD/reject-reason-audit-claude2-chirho.md.

### Eleventh diagnostic and dependent family-head follow-up

Frozen77b11701 completed884/938 accept and236/767 reject, one complete pass per
axis, actual runner0/COMPLETE, zero timeouts/unexpected exits. Clean HEAD and
CLI73e3e94341fbc57fc5feb82a46ca1b7565d3f6bdc2dcba6f2086fda6666217aa stayed
unchanged throughout. T12919/T14366 recover; SplitWD loses its earlier gain.
Against main there are six gains and four regressions: T12045a/T13248/T14010/
T26358 remain red. This is not a landable checkpoint or final two-pass gate.
Reject loses T15552a/T18640b versus tenth, gains none. The former had an adjacent
wrong reason; the latter had a matching contract, so investigate that loss rather
than calling both accidental. Main-relative reject+18/-3 still needs complete
reason accounting. Main's reject artifact lists wrong accepts, so compare its
complement against the measured rejected set, not the list directly.
Evidence: /private/tmp/haskelujah-family-head-chirho.mGuVQD/diagnostic-equations-chirho.log
and diagnostic-equations-{accept,reject}-chirho/. Embargo released in22113;
SLOT/DB retained, no main/artifact/DB changes.

Next bounded implementation: an inline family result can refer to its visible
head binder, as in data family F (k :: Type) :: k. Reuse data's existing binder
identity/telescope construction rather than give families an unrelated arrow
builder. The current family path registers classifiers without term identities
and always constructs non-dependent arrows. Prove both result-dependent uses
and a contradictory higher-rank equation against GHC before changing it. Check
SplitWD's cross-row classifier sharing separately; no assumption that this same
repair fixes it. Explicit @ retention and the four main regressions remain open.

The dependent-head repair now shares prepare/compose with data declarations;
its two controls were red before the fix. T18640b now reports the matching
GHC-83865 contract at14:10 (family equation result), and ordinary family uses
cannot select a result kind independently of the supplied argument. The first
ordinary-use control used a parenthesized kind annotation, which lowering
drops entirely. Its false acceptance remains recorded, not fixed or waived.
The retained control demands the same kind through Maybe/FlagBox application,
so its negative reaches the family classifier instead of the missing syntax.

SplitWD reduced to a parser classification defect: changing only its infix
family into prefix Append makes it pass; changing only its constructor spelling
does not. GHC accepts all three reductions. The dispatcher recognized only a
single name token before ::, so type (+++) :: ... became an alias and the family
lost its independent complete scheme. Dispatch moved out of the oversized CST
root into its existing family child; bounded name lookahead handles identifiers
and parenthesized operators without scanning a binder annotation or later line.
The new parser contract test was red before, green after; the indexed-family
driver control and all unchanged siblings pass. No assertion was relaxed.

Current gates: parser351 plus its three property and three golden tests,
naming136, typing363, integration88 and canaries7, all actual Cargo0 and zero
ignored/filtered. Explicit CLI build0, SHA256
4933651da2f53bbb2fa298501730104c9b0c1341b478760687f4623a6b745f50.
Seventeen reference contract pairs agree; the separate InlineFamilyWrongChirho
observation remains GHC1/candidate0. Source hashes, both diagnostics and the
scope distinction are in kind-oracles-chirho/family-equations-chirho/
family-head-followup-chirho.jsonl. The three previous equation fixtures and
their evidence moved together into that child, keeping the parent at11 entries
and the child at5, not growing an ungrouped fifteenth sibling. All consumer
includes were updated before the full focused gates. Clippy exits0 with590
warning messages/414 distinct diagnostics, unchanged, not zero warnings.
Full driver1772 still belongs to45c4a108; full workspace and final two-pass
landing gates remain owed. Freeze and next full diagnostic are next.

### Twelfth diagnostic and composed-injectivity decision

fdcd3655 is committed, pushed and remote-exact. The twelfth diagnostic completed
885/938 accept and236/767 reject with actual runner0/COMPLETE, no timeouts or
unexpected exits, clean source and SHA256
4933651da2f53bbb2fa298501730104c9b0c1341b478760687f4623a6b745f50 unchanged.
Accept+SplitWD/T26256a,-T15079 versus eleventh; reject+T18640b,-T15801.
The unchanged reject total hides a different set. T18640b's result-dependency
contract is recovered. T15801 formerly failed in its line23 superclass; GHC's
GHC-18872 is about nominal roles in the line52 instance, not that old reason.
T15079 now exposes a higher-rank local classifier: c at forall i. i -> Type
is used at two distinct kinds. Its separate complete operator signature now
survives, so previously merged kinds no longer hide that missing binder contract.
This is still open, not a claim that the whole file was right previously.
Evidence: /private/tmp/haskelujah-family-head-chirho.mGuVQD/diagnostic-head-followup-chirho.log
and diagnostic-head-followup-{accept,reject}-chirho/. Embargo released22115;
SLOT/DB retained. Main and its four-regression gate stay unchanged.

Next reversible decision: prove a single covering family equation by composing
already validated injective argument positions. Existing blanket family-headed
RHS rejection blocks T13248's Bar(Foo x) even though Foo and Bar have independently
validated dependencies. Place the bounded proof in the existing shared family
validator; no AST/dependency direction change or recursive speculative validation.
Only registered, arity-matched proofs are eligible. Missing proofs, a non-injective
inner family, uncovered patterns, and multi-row family-result interactions remain
unproved/rejected, never permission to project an argument. Confidence medium;
fdcd3655 bounds reversal. Test the actual proof as well as declaration acceptance.
GHC9.14.1's blanket family-headed ban is an explicitly recorded difference; a
constructor-wrapped equivalent and negative erasure controls arbitrate the
composition property independently. No corpus gain or hidden-argument support
is claimed before measurement. T12045a/T14010/T26358 and T15079 remain separate.

The covering-composition driver control was red before the implementation and
green afterward. An initial exact-filter invocation ran zero tests; it was not
counted. The corrected fully qualified invocation ran one and failed at the
family-headed RHS, then passed after the proof. Expanding the negative to
Either () (Wrap (Erase x)) revealed a second gap: tracing showed rows=[] because
family-term conversion did not represent the unit tuple. Both signature terms
and family terms now share tuple constructor composition. The negative then
rejects for injectivity instead of satisfying a test by not being checked.
The final control also checks ordinary tuple wrapping and the wrong concrete
result type. Neither wrapping nor erasure is inferred from an error-count total.

Two generic proof tests check every required dependency link, arity, an empty
injectivity set, and budget exhaustion returning unproved. Full typing365,
driver integration89 and canaries7 pass with zero ignored/filtered; the expanded
tuple control also passes. The explicit CLI build is current, SHA256
5ad934381d01b3a56ff704d6e4f6d86ce52f89a22036ad050bd634c8e6fa1ff2.
Eight bounded GHC9.14.1/candidate observations are stored in
kind-oracles-chirho/family-equations-chirho/composed-injectivity-chirho.jsonl:
six agree in verdict and two intentionally differ (T13248 and the reduced
family-headed composition). The latter are not described as GHC-accepted.
GHC accepts both constructor-wrapped forms and rejects their erasing mutations.
The wrong-result unwrapped reference still stops at GHC's family-head ban;
only our diagnostic reaches that wrong result, as the record states.
Scoped typing all-target clippy exits0 with107 messages/56 distinct diagnostics,
none on the changed family/kind files; this is not a zero-warning claim.
T13248 passes in the freshly rebuilt CLI. No updated corpus count is claimed
until this checkpoint's frozen diagnostic; main and the final landing gates
remain held. General nested inverse solving, hidden inputs and the remaining
accept regressions are not fixed by proving this dependency composition.

### Thirteenth diagnostic — composed proof, not a landing

b362f326d6997c49a876af1afcb98c21a11755cc is committed, pushed and remote-exact.
Its frozen one-pass-per-axis diagnostic ran 2026-09-10 23:42:28 through23:46:32
EDT: accept886/938, reject236/767, actual runner0/COMPLETE, zero timeouts and
unexpected exits. Clean HEAD and SHA256
5ad934381d01b3a56ff704d6e4f6d86ce52f89a22036ad050bd634c8e6fa1ff2 were asserted
before and after both axes. Versus twelfth, T13248 is the only accept change
and it recovers; the reject membership is unchanged, not just its count.

Against main121d4f2c, accept gains are PolytypeDecomp/RuleEqs/SplitWD/T14451/
T20922/T26256a/tc124. The three regressions are T12045a/T14010/T26358. The
baseline set was recomputed from the committed 56-file list and checked to
contain882 accepted files before comparison. Reject gains by counting rule:
T10836/T11356/T11563/T11623/T12430/T15799/T16502/T18640a/T18640b/T23162c/
T23734/T4875/T6018failclosed/T7368a/T9634/UnliftedNewtypesInfinite/tcfail209/
tcfail225. Losses: T16512a/T23162b/T23162d. The reject baseline is the complement
of the committed546 wrong accepts, checked to contain221 files. These18 gains
are not a claim of18 independently matching GHC reasons; the reason audit and
accidental-verdict distinctions above remain mandatory before any landing.

Evidence: /private/tmp/haskelujah-family-head-chirho.mGuVQD/diagnostic-composition-chirho.log,
diagnostic-composition-{accept,reject}-chirho/ and diagnostic-composition-chirho.ts.
Broker22116/22117 bracketed START/DONE. The machine-load embargo is released;
SLOT/DB remain with this lane and canonical row484 stays open. Main, published
artifacts, denominators and label policy are unchanged. Full workspace and the
final two-pass landing gate remain owed.

Next source boundary verified by reading, not implemented: lower_type_chirho's
AppType arm explicitly drops an @ token and the following type node. A proper
repair must preserve the explicit application and its span, resolve its type
names, consume the intended invisible kind binder, and keep hidden family/type
arguments coherent in consumers. Merely retaining it as an ordinary argument
or fixing only the parser is not that repair. T15079 separately needs its
higher-rank local kind classifier; T26358 requires separately scoped equation
variables and correlated apartness, not name-based identity conflation. None
of these mechanisms is claimed fixed by this diagnostic checkpoint.

### Visible kind application — representation decision

2026-09-11, direct L.J. continuation at549fc904, clean owned worktree and main,
no intervening broker lease claim. Row484/SLOT/DB remain owned by GPT.
Recommendation: represent type-level @ with a dedicated KindAppChirho AST
shape; name/scope visitors must traverse both operands, and kind inference
must consume ordered invisible binders without confusing them with ordinary
arguments. Bindings must retain specified/inferred status and classifier
dependencies, including explicitly quantified names absent from the body.
Hidden arguments must not disappear before family/type equality consumers;
retaining syntax alone is not completion. Existing public inferred-type and
constructor builders need the same elaboration contract rather than unrelated
arity/name heuristics. No dependency or main/API release is authorized here.

This is isolated, reversible compiler work, confidence medium. Alternative of
turning @ into ordinary App changes arity and is incorrect; retaining only a
span annotation preserves the wrong representation. Checkpoint tag
visible-kind-applications-before-chirho at549fc904 bounds reversal. Extract
application lowering from the oversized root into its existing flat-type
module, and keep new signature machinery in focused typing children. Prove
positive/negative @ selection, source ordering, inferred-binder skipping,
phantom kind distinction and an executed control against GHC before any
updated corpus or broad feature claim. Main's three-regression gate remains.

### Visible application checkpoint — retained arguments, incomplete consumers

- [x] Dedicated Type/AstKind KindApp, source spans and specified/inferred binders;
  mixed-spine lowering and visitors retain @ rather than an ordinary argument.
- [x] Ordered source schemes retain phantom binders and classifier dependencies.
  Provisional family schemes leave unselected classifier holes shared across rows.
- [x] Solved local nominal indices cross the driver/type-inference boundary in
  KindElaboration and InferInputs; constructor results and signature equality
  retain them. New machinery lives in focused children; oversized roots shrink.
- [x] Complex forall annotations reach the ordinary CST type grammar. A binder
  whitelist previously dropped the binder entirely; the new parser control was
  demonstrated red before this repair. Missing/wrong @ kinds are diagnosed.
- [x] Wildcard @ arguments are inferred, never fabricated as Type. Eight focused
  controls pass; the original five were demonstrated red on the old implementation.
- [x] Fresh GHC9.14.1/candidate evidence: reference-chirho.jsonl beside the seven
  visible-application fixtures contains one metadata and14 observation records,
  all agreeing in verdict. Both execution pairs require42 plus LF; the same two
  sources also pass exact-output STG/LLVM/Cranelift integration. CLI SHA256 is
  029835e8f04ce6314676c7b66583a5898f624871cc4464612d317cae1a3eb42b.
- [x] Post-edit gates: naming136, parser353, typing365, core128, TH16,
  driver integration97, canaries7; actual cargo0, zero ignored/filtered. Two
  prior parser tests explicitly asserted invisible-argument erasure; their
  expectations now require the distinct retained application, not erasure.
- [ ] Complete recursive nominal, synonym/family equation and imported-index
  consumers before any feature/compatibility completion claim. T12045a now
  reaches E0200 on line35: FreeCat versus FreeCat @t60, after its @kind checks
  recover. T14010/T26358 remain red. This is not a landed corpus gain.
- [ ] Frozen corpus diagnostic, no-new-accept-regression gate, full workspace,
  final two-pass measurement, DB closure and main landing are still owed.

Access resumed normally in the same process; no chmod, remount or security
bypass occurred. The TCC explanation remains a hypothesis, not a diagnosed
cause. Main121d4f2c and its published artifacts/denominators/labels are untouched.
Row484 remains open with SLOT/DB reserved; this is an isolated checkpoint.
Evidence scratch: /private/tmp/haskelujah-visible-kind-app-chirho.X3Kq8n/.
Six-crate all-target clippy exits0 but reports684 warning messages/365 distinct
diagnostics. This is not warning-free. The relocated legacy eight-argument
wrapper retains its existing too-many-arguments warning; new elaboration and
mixed-spine scheme functions have no diagnostic. No warning suppression added.

### Recursive and expression-local nominal follow-up

3bbcd49c was pushed and ls-remote verified before continuing. A GHC-executed
recursive/mutually-recursive fixture prints42/7; its new integration control
was red on3bbcd49c (missing indices) and green after capture was delayed until
the final scheme is known. Pending occurrences distinguish already quantified
arguments from group identities generalized only at publication; neither is
replaced by a fresh unrelated application or erased to get equality.

The full integration gate then found an existing inline-declaration execution
control red: its expression signature had never been visited by the module
kind pass and consequently contained a bare nominal head. The type consumer
now instantiates the KNOWN local head's quantified slots there, respecting @
specificity, rather than inventing an imported contract. A new local-expression
control prints42, while its @Type versus @Bool mutation rejects under both
GHC9.14.1 and ours. It first exposed a second boundary mismatch (Bool versus
GHC.Types.Bool); solved kind terms now use ordinary imported-type normalization.
Complete classifier checking of local annotations is explicitly not claimed.

Post-follow-up typing365, integration99 and canaries7 pass without exclusions.
The reference JSONL now has1 metadata+17 observations, all verdict-agreeing;
four execution pairs agree exactly, and the two added executable sources pass
STG/LLVM/Cranelift. SHA256 d1f4f2acb3d414a704f51d555d408669878262989cf8d2390ba377cd05eb1e77
identifies the explicit CLI. T12045a passes there; T14010/T26358 remain open.
No corpus movement is banked until the following frozen diagnostic. Main and
row484 closure remain held, with all unfinished consumers recorded in workflow.

### Visible-application frozen diagnostic and consumer repairs

f5c4eedb, CLI SHA256 d1f4f2acb3d414a704f51d555d408669878262989cf8d2390ba377cd05eb1e77,
completed one pass per axis:875 accept,234 reject, zero timeout/unexpected exits,
938/767 denominators verified, clean source/CLI stable before and after both.
Main unchanged at882/221; no artifact, label, DB or membership changes.

Main-relative accept regressions: DeepSubsumption02, GivenTypeSynonym, LocalGivenEqs,
T13879, T13951, T14010, T15942, T18986a, T21583, T22560c, T23501b, T26358, tc151.
Main-relative accept gains: PolytypeDecomp, RuleEqs, SplitWD, T14451, T26256a, tc124.
Against b362f326: T12045a recovers; the new failures are those thirteen excluding
T14010/T26358, plus T20922 (already red on main). Counts alone hide that trade.

Reject versus b362f326: gains T12045b/T15474; losses ExplicitSpecificity3/T12803/
T17563/T5853. These are verdict deltas only; no matching-reason claim without audit.
Previous main-relative reject qualifications remain applicable, not erased by234.

- [ ] Restore consumers of nominal indices without dropping the retained arguments:
  synonym/equation RHS conversion, constructor refinement detection and equalities.
- [ ] Preserve higher-rank kind binders and associated-family binder contracts.
- [ ] Reduce the remaining dependent/family-kind failures with positive/negative controls.
- [ ] Repeat frozen diagnostic, then the full landing gate only after no new accept loss.

Evidence: /private/tmp/haskelujah-visible-kind-app-chirho.X3Kq8n/diagnostic-visible-applications-chirho.log
and its accept/reject directories. Both frozen checkpoints are pushed; CPU embargo
ended at CORPUS-PASSES-DONE22171, SLOT/DB retained. No corpus names enter compiler rules.

### Indexed GADT refinements and quantified kind binders

Two added source controls were red on f5c4eedb, while GHC9.14.1 accepted both.
Constructor-result refinement stopped at KindApp instead of reaching the nominal
head; it now traverses both application forms without erasing either index.
The cast control prints42 on STG/LLVM/Cranelift, and its non-equality mutation
rejects. A higher-rank annotation was lost even before binding: AstKind had no
forall form. Explicit invisible/required forall kinds now retain source scope;
naming, dependency discovery and TH visit them. A forall-annotated variable owns
a scheme instantiated per use, capturing outer identities rather than generalizing
them. Alias expansion also preserves a leading quantified kind contract.

Focused fresh CLI recoveries versus f5c4eedb: T13951,T20922,T15942,T18986a,T23501b.
T13879 reaches a later E0200 on HRefl versus r; it is NOT recovered. T22560c still
lacks an associated-family specified-binder contract. No new full count claimed.

Gates: naming136, parser353, typing365, TH16, integration101, canaries7, all zero
ignored/filtered and actual cargo0; workspace all-target check0. The reference
artifact holds1 metadata+21 bounded observations, all GHC/candidate verdicts
agree, including five exact execution pairs. CLI SHA256
f8e5edefd37c353044f57c9309158b35a07f311fd71117b090e4499a0725b4e8.
No full workspace test, final two-pass gate or main landing is claimed. The
remaining work includes stored synonym/equation indices, associated families,
T14010 and T26358. Checkpoint before that producer repair; row484 stays open.

Frozen01f3be2e diagnostic completed:880/938 accept,235/767 reject, no timeout or
unexpected exit, stable source/CLI. Exactly the five focused recoveries held;
no new accept failure versus f5c4eedb. Main-relative regressions: DeepSubsumption02,
GivenTypeSynonym, LocalGivenEqs, T13879, T14010, T21583, T22560c, T26358, tc151.
Reject gains only T16946, none lost: NOT matching-reason capability. GHC-71451
rejects escaping skolems at generalization; ours reports a kind mismatch on the
same signature. Retain this distinction even though the counted verdict agrees.

Next reversible step: local synonyms need explicit hidden parameter contracts
and RHS conversion using the same solved indices as ordinary signatures, not a
static converter that emits unindexed nominal heads. Keep ordinary/invisible
parameter spines separate and substitute simultaneously. Extract synonym storage/
expansion from the oversized inference root into a focused child; no new dependency,
no indexed/unindexed equality bypass. Checkpoint tag synonym-kind-indices-before-chirho
is01f3be2e. Imported/family equations require their own complete contracts afterward.

### Local synonym kind-index contracts

Local synonym bodies now use the signature converter and close their own ordinary/
invisible parameter identities. Expansion consumes the two spines separately and
substitutes simultaneously; caller variables and linear-arrow multiplicity survive.
Constraint-alias occurrences follow the same recorded-span path. The new focused
synonym module removes the old expansion and substitution blocks from the oversized
inference root. A body with unclosed variables errors instead of becoming shared state.

The first source control was red on01f3be2e and now prints42/7/11 on STG, LLVM and
Cranelift, matching independently executed GHC9.14.1. Its phantom-kind mutation
rejects in both. Three producer defects surfaced while closing the RHS: an annotation
classifier was computed but not required to be Type; a parenthesis wrapper looked up
the wrong occurrence span; NoPolyKinds defaulting rewrote the body but left the old
identities in the binder list and pending applications. All three producers are
repaired, not bypassed at equality. Positive/negative defaulting and classifier
controls agree with GHC; a substitution control preserves caller names and linearity.

Frozen-candidate gates: typing366, integration104, canaries7, zero ignored/filtered;
workspace all-target check0 and explicit CLI build0. Reference JSONL has1 metadata
plus27 observations (six exact execution pairs), all verdicts agree. CLI SHA256
4fedbbec8ef54cd806aa6f0233789c066c4d8687eddedd235fa8167b0349cf84.
Typing all-target clippy exits0 with103 warning messages/54 distinct diagnostics;
the new synonym module, elaboration module and argument consumer have none. This
is not warning-free or a full-workspace test gate. Focused recovery is four files;
T21583 still exposes the separate family-equation RHS index gap. Freeze/push before
measuring both axes. Main, public artifacts and row484 closure remain untouched.

Frozen9720465c diagnostic completed:882/938 accept,244/767 reject, one pass each;
zero timeouts/unexpected exits, clean source and CLI4fedbbec stable before/after
both axes. Compared with01f3be2e, the four named recoveries held but T12928 and
tc160 newly failed. T12928 hits the new unclosed-synonym-body error; tc160 loses
alpha-renaming/quantification behavior in a nested rank-n synonym. These are
regressions to repair, not grounds to remove the closure check or weaken the test.

Main-relative gains: PolytypeDecomp, RuleEqs, SplitWD, T14451, T20922, T26256a,
tc124. Main-relative regressions: T12928, T13879, T14010, T21583, T22560c,
T26358, tc160. The total equals main while membership differs by seven each way.
Reject gains versus01f3be2e: ExplicitSpecificity3, T12803, T12966, T17563,
T24553, T5853, UnliftedNewtypesFamilyKindFail1, VisFlag1, VisFlag1_ql; none lost.
Their reasons are NOT yet audited and244 is a verdict count, not a capability claim.
The runner, full sets and per-file logs are under
`/private/tmp/haskelujah-visible-kind-app-chirho.X3Kq8n/diagnostic-synonym-contracts-{accept,reject}-chirho/`;
the runner log also records exact HEAD9720465cd21a95ddcccf172bccd00cd2be4c0474
and CLI hash above. CPU embargo released in broker22185; row484 remains open.

### Synonym identities and inferred kind classifiers

tc160 reused a stored forall identity at nested source uses. Source expansion
now freshens template binders lexically; normalization stays idempotent. Its
GHC-accepted reduction prints42 on all three engines. A Bool-specializing mutation
then exposed flexible stripping of an expected result forall: equations and
lambdas now open those binders rigidly. Both negative forms reject for type mismatch.

T12928 had inferred nominal kind arguments whose classifiers were not constrained.
Opened scheme parameters now retain their classifiers; only newly solved equality
entries are checked against authoritative known term contracts. The dependent
synonym control prints42 on all three engines. Arrow representation binders and
promoted GADT schemes needed their real classifiers, not generic Type defaults.
Refl now separates its inferred kind from its specified value parameter.

The reflexivity negative initially passed because flat head lowering DROPPED its
`True :~: False` annotation. Measured AST trace: kind_annotation=None. The original
positive/negative source is unchanged; the bounded binder now uses shared flat
type lowering. Its helper moved out of the oversized lowering root, replacing
the smaller duplicate kind parser. Operator, operands, spans and following
signature are pinned as the parser contract. Unsupported list/promoted AstKind
forms remain named limitations, not claimed support. Temporary traces removed.

Gates: parser354, typing367, integration107, canaries7, zero ignored/filtered;
workspace all-target check0, explicit CLI build0. Reference JSONL now1 metadata
plus34 observations, including9 exact execution pairs, all GHC9.14.1/candidate
verdicts agree. CLI SHA256 117bb4d1af6e671fa0f485d0b5c655f75759fdadb99d591b2f739afc08eff1e5.
Typing clippy0 still reports103 warning messages including duplicates; it is NOT
warning-free. Actual focused T12928/tc160 recover; the other five main-relative
regressions remain. Freeze/push, then measure exact corpus sets before proceeding
to family-equation consumers. Main, public artifacts and canonical row484 unchanged.
