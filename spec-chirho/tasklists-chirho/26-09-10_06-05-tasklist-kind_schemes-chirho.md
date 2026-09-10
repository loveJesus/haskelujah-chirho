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
