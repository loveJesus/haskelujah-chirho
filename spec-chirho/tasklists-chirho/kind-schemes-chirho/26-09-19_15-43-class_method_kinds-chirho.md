<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Class-method hidden kinds, row484

Resume isolated gpt-kind-schemes-chirho at 30b83956 under L.J.'s continue-dev
instruction. Main 6db522ad, peer branches, canonical DB and public measurements
remain untouched. Local pre-beta compiler work is reversible; no landing is implied.

## Placement and hypothesis

The retained eight GHC 9.14.1 graphs show four valid methods failing when their
class parameter has an inferred or explicitly polymorphic kind. An explicit
Type-kinded parameter and a value signature using the same family equation pass.
Trace the checked class-parameter classifier and method's hidden family input
through registration and instantiation. Do not erase hidden inputs, relax
family matching or infer a kind from a corpus name.

The first implementation choice belongs at the existing typing declaration /
method-scheme boundary, using the kind checker's represented evidence. Keep
new logic in a focused sibling module; do not grow the oversized infer root.
Decide exact metadata ownership from the trace before adding fields. Preserve
ordinary method variables, class-parameter order, source ownership and bounded
work per declaration/use.

## Checklist

- [x] Replay retained reference graphs on the immutable current CLI and add
 driver controls that demonstrate the valid failures and genuine contradictions.
- [x] Trace classifier/term identities and settle the smallest shared repair.
- [x] Implement without dropping or fabricating checked kind evidence.
- [ ] Run focused parser/naming/typing/driver, canaries and the live package
 control; compare complete corpus membership at the meaningful gate boundary.
- [ ] Retain source/hash/result evidence, limits and workflow; commit only owned
 paths, push and verify the exact remote tip.

## Acceptance and limits

Local and imported class methods must agree with GHC, with and without defaults,
at more than one parameter kind. A wrong associated result and an ordinary
method-polymorphism violation must still reject for their real reason. Test the
main frontend, not a temporary tree shape. T14441 is independent until evidence
joins it. Passing selected files never establishes the remaining corpus count.

## Resume state, 2026-09-19 16:17 EDT

The channel is the existing kind elaboration: class heads now publish ordered
checked kind binders separately from type-application heads. ClassDecl retains
their corresponding typing variables; methods use the same lexical seed map as
ordinary class parameters. Each checked instance occurrence supplies its hidden
arguments before the method scheme is specialized. Remaining method universals
are skolems, not flexible variables chosen by an implementation body.

The eight retained source graphs are tested through BOTH the file-check graph
and compile_modules. The latter previously failed to transport ClassEnv, so an
imported contradictory implementation could pass because its class was absent.
It now forwards the checked class environment like the file-based entry point.
This is a checking fix, not proof of runtime dictionaries or full import privacy.

Revision A (before method-universal rigidity) passed typing 407, core 128 and
typing integration 443; frozen CLI dffbc899494101b993f73d39728d20d90f29b8654a8dd3af763e11e688ffe6f5.
That CLI still accepted the measured invalid method-parametric control. Revision B
adds rigidity and passed four focused matching tests. Its fresh broad gates pass
typing 407, core 128 and typing integration 444, with zero ignored/filtered tests.
The fresh CLI is 5c145678e45ee3996fd22ce3b078c65e2986e4612b102c72da28e95a0b2ee495;
all twelve calibrated scope cases agree with GHC, with hashes stable before/after.
Never attribute revision A's broad gate to revision B.

Peer K2 and K6 negative predictions rejected in GHC for ambiguity rather than
the intended contract. K2b replaces K2; K6 is retained as refuted evidence but
excluded from the behavioral table. Twelve scope controls include the actual
parametricity contradiction and three family-method cases used at two kinds.
Three new dependent-class-head probes are measured separately before deciding
their scope; the parent accepts a head whose supplied kind and promoted argument
disagree, so that is not presumed fixed by the method channel.
Current CLI also accepts that bad dependent head; valid two-kind instances and
the genuine bad method result retain their reference verdicts. The new method
channel is not a claim of complete dependent instance-head checking.

Evidence lives beside the retained graphs in associated-methods-chirho/method-scopes-chirho.
Claude2's bounded read-only review #23794 found no capture/identity defect in
the method helper, confirmed the lexical ordering and noted its maintenance
dependence. The workflow now states that ordering explicitly.

No fresh corpus claim, canonical DB write, main merge, public-label change or
deployment. Broad current-revision gates, corpus membership, clippy and the live
constraints package remain explicit obligations before any landing decision.

## First complete diagnostic and repair, 2026-09-19

Frozen revision B 5c145678 is NOT landable: accept 880/938 vs parent650be538
886/938, with three gains and nine losses by name. Reject 269/767 vs267/767;
both gains are wrong reasons, not new capabilities. All four axes had zero
initial/unresolved timeouts, runtime panics and unexpected exits. The full
observations are retained in method-scopes-chirho/repair-chirho/first-corpus-chirho.tar.gz;
the adjacent README and corpus-delta list every moved file and reason boundary.
No main, public artifact or canonical DB change.

Five losses first occur when method universals become rigid; four already fail
the preceding channel revision. New InstanceSigs controls exposed signatures
discarded by lowering, so consuming signatures alone was insufficient. Retain
the written signatures, check their generality against the class obligation,
then check each body under its own scoped binders. Source-binder provenance must
travel with imported ClassDecls; synthetic inference holes are not universals.
This follow-up is dirty and unproved until its own gates run. Generated instance
kind evidence must also stop using source spans as occurrence identity.

The first scope follow-up passes all four focused integration tests, including
17 calibrated scope/signature cases and the eight source graphs through both
entry points. Frozen CLI90e9160e recovers T11552, T13142, T21323 and T5676.
T14010 plus the four earlier channel regressions remain. T15712 now accepts;
its wrong-reason rejection was not a capability. T26137 still hits missing
metadata. The next isolated change keys class-instance kind evidence by the
post-deriving declaration ordinal (both phases consume that same list), not
source spans shared by generated declarations. It has not been gated yet.

## Representation follow-up, 2026-09-19 17:06 EDT

Frozen CLI 1b1304668f9e2e879cd267ea7f6a7967afb5c7ff14c6821994471772b0c8d6eb
recovers eight of the nine accept losses in the bounded eleven-file replay.
T3955 alone still fails. Both wrong-reason reject gains now accept; neither was
a capability. Six calibrated representation controls agree with GHC, with
binary hashes stable before/after and no timeouts. The original local-Type
negative failed GHC at an invalid import; its corrected version rejects at the
intended nominal mismatch and replaces it in the behavioral table. Sources,
diagnostics and hashes are under repair-chirho/representation-chirho.

The checked instance channel now uses the shared post-deriving declaration
ordinal and class name, not source spans. T18129's raw AST contained ordinary
Con [] because lowering skipped the promotion quote; that quote now reaches the
shared type parser. T14010's raw family inputs compared hidden Type to a TYPE r
row; canonical builtin Type representation repairs matching without weakening
rigid method variables. These changes need their own current-revision gates.

Next brick placement: retain full deriving class applications in the AST/lowerer,
move new GND elaboration into a focused child of deriving_chirho, and use checked
class/newtype kinds to choose eta reduction. The oversized deriving root must
not grow another name-based workaround. T3955 currently loses the written a in
MonadReader a and generates a fully applied T a x instead of T a; no skipped
kind check, guessed class arity or corpus-name exception is an acceptable fix.
This next brick is not implemented by the eight-file recovery checkpoint.

## Primitive identity follow-up, 2026-09-19

Claude2's static review identified that local-name collection omitted associated
families. A GHC-calibrated reduction made the consequence executable: 1b130466
accepts Proxy Type -> Proxy Bool = id when the module declares a catch-all
associated TYPE family returning Bool. GHC rejects Type versus Bool. The
explicitly qualified local TYPE application must still reduce and accept.

The repair counts associated heads as local type declarations and honors exact
builtin nominal identities before family/synonym suffix lookup. Import-name
ownership moved into a 130-line driver child module, shrinking the root. Both
new driver controls pass. Current source also passes core128, naming138,
parser372, typing407 (zero ignored/filtered) and the live constraints package
test (1 passed, 1775 filtered, 83.70s). The package symlink and cabal file were
present, so this was not its missing-cache early return. Full typing integration
and a new immutable CLI replay follow; pre-shadow-fix gates are not attributed
to this source. T3955 remains open, and the measured G7 wrong-kind deriving
false accept is part of the same next brick, not waived by these checks.

The subsequent verification is complete for this diagnostic checkpoint:
typing407 and full typing integration445 pass, zero ignored/filtered, after
regrouping the checker arguments to remove a newly introduced lint warning.
Final frozen CLI8344419381552688b1af2dd8eaf2220a3b20f17d312379443963799f7dfb5278
repeats all eight recoveries and eight representation verdicts, zero timeouts,
hashes stable before/after. The exact logs and source hashes are under
representation-chirho/shadowing-chirho/checkpoint-chirho. Clippy still reports
387 warnings, none with a primary span on a changed line; lint is NOT clean.

An extra qualified-alias negative remains a measured pre-existing gap on both
parent650be538 and candidate40b2154b: a local type synonym Type = Bool can capture
KindChirho.Type through bare-name fallback. Its GHC mismatch and paired local
alias positive are retained separately, not counted among the passing controls.
No full corpus result, main landing, canonical DB or public-label change follows
from this checkpoint.

### Next-brick phase decision

Keep the GND repair inside one kind-inference context: checked/closed declaration
kinds, then elaborate GND using the retained written class application, then
ordinary instance checking, then finalization and ordinal-keyed publication.
Generating after KindResult would omit the hidden-argument transport and
associated-default checking. Do not recreate a second kind checker from env.
Claude2 independently verified this dependency at #23909. My earlier concern
that current Generic deriving emits associated equations was incorrect: its
emitted instances have empty associated-equation lists. Limit the phase move to
GND for change scope, not on that false premise.

- [ ] Replace name-only deriving payloads with full represented applications;
  check their written arguments under the declaration's lexical scope.
- [ ] Put GND elaboration in a focused child module. Consume checked residual
  class kind and newtype kind; preserve all written class arguments in both
  the generated instance head and its representation constraint.
- [ ] Eta-reduce only trailing newtype parameters that the representation can
  actually remove, keeping generated instances inside the ordinary instance
  pass and declaration-ordinal transport.
- [ ] Gate G1/G2/G4/G6/G8 positives and the distinct G3 unary, G5 eta and G7
  wrong-kind negatives by their actual diagnostics, then T3955 and complete
  corpus membership. No new runtime-coercion claim without execution.

## Storage interruption receipt

L.J.'s request to free disk space paused new builds. Owner-verified regenerable
outputs were removed with cargo clean using each exact cache directory as its
target; no source, worktree, branch, database, retained binary or scratch evidence
was removed. Allocated sizes before deletion, in KiB:

- haskelujah-gpt-chirho/target/debug/incremental: 6,201,068.
- haskelujah-gpt-flat-types-chirho/target/debug/incremental: 1,437,656.
- haskelujah-claude2-chirho/target: 2,312,760 (explicit owner clearance #23752).

All paths are under /Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho.
Total allocated footprint: 9,951,484 KiB, about 9.49 GiB. All three were verified
absent afterward; other target contents and Git status were preserved. A plain
rm attempt was denied and removed nothing; only the scoped Cargo cleans executed.
uv prune encountered an active lock and was cancelled without force or deletion.
AICEO owns the separate internal user-cache cleanup. Concurrent df changes are
not attributed to this lane; final receipts were posted as #23774 and #23775.
