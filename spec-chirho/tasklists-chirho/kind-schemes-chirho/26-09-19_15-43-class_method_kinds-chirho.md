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
- [x] Run focused parser/naming/typing/driver, canaries and the live package
 control; compare complete corpus membership at the meaningful gate boundary.
- [x] Retain source/hash/result evidence, limits and workflow; commit only owned
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

Implementation resumed from ff75ea3f, checkpoint tag
`row484-gnd-kind-seam-before-chirho`. Main's independent context/duplicate lane
has landed; this work remains in the isolated kind worktree, with no canonical
DB write. Preserve deriving applications as TypeChirho, using the shared type
lowerer and lexical declaration scope. Split the kind driver into declaration
preparation and instance/finalization, with a mutable frontend entry point that
inserts GND between them. Keep the existing immutable kind-check entry point
for callers that do not request deriving; neither path clones or rebuilds a
kind checker. Pure representation eta reduction belongs in a deriving child;
the kind-context adapter owns checked classifiers and diagnostics.

This is a local reversible compiler change. Existing deriving strategies and
runtime coercion are not being declared complete. Exact GHC-calibrated verdicts
and generated-head/context behavior, not names or corpus exceptions, gate it.

Keep the GND repair inside one kind-inference context: checked/closed declaration
kinds, then elaborate GND using the retained written class application, then
ordinary instance checking, then finalization and ordinal-keyed publication.
Generating after KindResult would omit the hidden-argument transport and
associated-default checking. Do not recreate a second kind checker from env.
Claude2 independently verified this dependency at #23909. My earlier concern
that current Generic deriving emits associated equations was incorrect: its
emitted instances have empty associated-equation lists. Limit the phase move to
GND for change scope, not on that false premise.

- [x] Replace name-only deriving payloads with full represented applications;
  check their written arguments under the declaration's lexical scope.
- [x] Put GND elaboration in a focused child module. Consume checked residual
  class kind and newtype kind; preserve all written class arguments in both
  the generated instance head and its representation constraint.
- [x] Eta-reduce only trailing newtype parameters that the representation can
  actually remove, keeping generated instances inside the ordinary instance
  pass and declaration-ordinal transport.
- [x] Gate G1/G2/G4/G6/G8 positives and the distinct G3 unary, G5 eta and G7
  wrong-kind negatives by their actual diagnostics, then unchanged T3955.
- [x] Complete current-revision focused suites, imported-class and live-package
  controls, immutable CLI replay and lint accounting after the structural split.
- [x] Compare complete corpus membership before a landing decision. No new
  runtime-coercion claim without execution.

The first eight-case draft exposed two repair defects, both fixed before the
14-case gate: generated target nodes reused the written deriving class span,
aliasing hidden-argument records; and the newtype kind was opened rigidly instead
of instantiated for its derived use. The eight controls plus arrow/list/tuple,
free-prefix and two-kind cases now pass, as does the unchanged T3955 source.
Reference predictions were recorded before the five extra GHC 9.14.1 probes;
all five held. Imported qualified class arguments have a further positive and
wrong-kind reference pair, awaiting the full current candidate gate.

Claude2's read-only review #23992 verified closure -> GND -> instance checking
-> finalization, isolated binder identity, and no instance publication after an
error. This is a source review, not independent candidate execution. Existing
deriving strategy, coercion and runtime dictionary limitations remain separate.
The former 3,635-line deriving root was split into stock, via, syntax-builder,
higher-kinded and test children. Dispatch now shares one stock-class classification
across inline data, newtype, standalone and late-GND ownership; the root is 338
lines. The parser root remains separate pre-existing structural debt, not clean.

### Live-package regression found before checkpoint

The first recorded gate passed naming138, parser374, typing409, TH18, canaries7
and all460 typing integration tests. The LIVE constraints package test then failed:
GND incorrectly claimed Generic1 in transformers and tagged, and the builtin Ix
kind was missing. Dependent mtl/root missing-name errors were downstream, not
independent regressions. The failed receipt is retained; those green subsets
never established package acceptance.

Generic1 is stock-owned but its generator is still unimplemented. The unified
dispatch emits an explicit unsupported-stock warning, rather than manufacturing
a GND representation constraint or claiming Generic1 works. Ix receives its known
Type -> Constraint seed; local declarations still shadow it. Four predictions
were recorded before GHC9.14.1 reference checks: package-shaped Generic1, ordinary
Ix deriving and a local higher-kinded Ix accept; Ix Maybe rejects for kind arity.
All four candidate controls pass along with the other16 matching newtype tests.
This restores strategy ownership, not complete deriving strategies, Generic1
generation, role/coercion proof or runtime method execution.

Post-split/post-ownership broad gates, live package, a frozen CLI and corpus
membership remain owed. No landing, main/DB change or public measurement update.

The first retry advanced through the original Generic1/Ix failures but exposed
Data in transformers/Data.Functor.Constant (62.79s, one live test failed).
The stock ownership set now includes Data, Typeable and Lift. Their early pass's
existing metadata-only support is preserved, not promoted to a new claim of
generated Data/Lift methods. The fifth GHC reference accepts the combined stock
declarations with a phantom parameter. Revision2 passes naming138, parser374,
typing409, TH18, canaries7 and ALL465 typing integration tests (zero ignored or
filtered). The LIVE constraints package then passes: one test, 1775 filtered,
89.51s. Frozen CLI5d5976ec belongs to that revision only. The remaining new
Chirho-enum naming lint is removed by storing the selected generator directly
in the dispatch entry, rather than disabling the lint. Final-revision gates and
the frozen candidate are being regenerated; the older lint debt remains.

Final revision3 completes those gates on unchanged source: naming138, parser374,
TH18, typing409, canaries7 and typing integration465, all zero failed, ignored
or filtered. The LIVE package passes again (1 run, 1775 filtered, 92.78s).
Frozen CLIbc186a6d41d756ee18cf91b7b5303c8e1cf0d04b3b57fe1474209de7880a191a
agrees with all20 retained GHC frontend cases and accepts all9 original corpus
losses, T3955 included; no timeout and binary hashes stable before/after. The
six negative diagnostics are the intended unary-constraint, eta-reduction or
kind-arity errors, not missing metadata. Clippy reports433 distinct warnings,
zero on changed lines; this is existing debt, NOT a clean lint result.

Replay stopped before the imported pair when the retained provider text failed
its recorded hash. The original scratch source matched the recorded SHA256;
the retained copy had one extra LF. The copy is corrected and the discrepancy
recorded on both reference rows; the complete29-case replay then passed. No
compiler change followed from that evidence-copy correction. One diagnostic
corpus pass per axis is queued after Claude's driver suite, not a landing gate.

### First GND corpus diagnostic and standard monad follow-up

That diagnostic is complete on frozen bc186a6d: accept889/938, reject268/767,
zero initial/unresolved timeouts, runtime panics, unexpected exits or output
limits. Source and binary hashes remained unchanged. Against parent650be538,
accept gains are the three class-default boot files and T18036b; NEW LOSS
T12734 is not waived. All9 first-candidate accept losses recovered. The sole
new reject is T6001; its real Int/Integer instance-signature defect is caught,
but the message inverts expected/actual, anchors the binding and omits the
generality rule. Record ADJACENT, not matching (independent review #24081).
T15712/T26137 wrong-reason rejections are removed. Versus current main there
remain older row484 differences plus main's independently landed duplicate
instance work; the total difference is not this unit's capability gain.
Attribution against the separately retained pre-GND CLI83444193 confirms it
already rejected T6001 with the identical diagnostic. The +1 is since the older
parent650be538, from the earlier InstanceSigs work, not a new GND capability.
That same immutable replay rejects T18036b and accepts T12734, locating those
two accept-side movements in the current GND continuation.

T12734 reports missing unary classifiers for MonadIO, MonadFix and MonadTrans.
The central builtin kind seed contains none of them; GHC9.14.1 reports
MonadIO/MonadFix :: (Type -> Type) -> Constraint and
MonadTrans :: ((Type -> Type) -> Type -> Type) -> Constraint. Six predictions
held in reference probes: valid deriving and well-kinded contexts accept,
three wrong-kind arguments reject, and local first-order same-spelled classes
shadow the standard contracts. Frozen bc186a6d rejects the valid deriving and
falsely accepts all three wrong-kind arguments. The repair adds these central
contracts, not a GND-name workaround or a relaxed unary check. Five new driver
tests include the unchanged T12734 and that six-case matrix. Code is written;
its own tests/build/replay remain pending while Claude holds #24080's corpus
slot. Reference sources and both compilers' outputs are retained beside the
other GND controls.

## Storage interruption receipt

### Reboot recovery, 2026-09-20

The host reboot erased /private/tmp, including unarchived GND gate/replay/corpus
receipts and frozen binaries. Revision4's source and the three retained reference
JSONL files survived. Its prior running verification is incomplete, not green:
only the delivered library-stage results are known; no final receipt exists.
Re-run the same source with persistent, compressed logs in the GND evidence
directory and the immutable CLI under this worktree's ignored tmp-chirho, not
the operating system temporary directory. The older figures above remain
historical reports, not reopenable evidence or a current acceptance gate.
No source edits, main changes or canonical DB writes are required by recovery.

The repeated gate on unchanged revision4 now passes naming138, parser374,
TH18, typing409, canaries7 and typing integration470 (zero failed, ignored or
filtered). The LIVE constraints package passes separately: 1 run, 1775 filtered,
68.87s, with its cabal prerequisite hashed before and after. No compiler warning
lines; clippy433 distinct warnings remain, zero primary spans on changed lines.
Frozen CLI6aab9ed64d2fa159b09350054ef0a4f4adfb033f5dbc31cda27deb98e48b88bb
matches all26 retained reference verdicts and all10 selected corpus recoveries,
including T12734. Hashes stable; no replay timeout, panic or unexpected exit.
Persistent receipts and runners are under gnd-chirho/checkpoint-chirho and
gnd-chirho/tools-chirho. Complete corpus membership waits for Claude's announced
quiet boundary, not a guessed completion time.

A separate reference check resolves a misleading next-repair premise: unchanged
T5481 exits1 under GHC9.14.1 with GHC-76037 for b at6:16 and a at8:16, matching
our scope rejection. It remains a loss under the frozen should_compile inventory,
but making it accept would contradict the measured reference. No corpus movement,
denominator change or waiver is authorized here. The paired T17067 source DOES
pass GHC and remains a real candidate defect: data-family applications are nominal
in family equation patterns, unlike type-family applications. Both commands,
sources and diagnostics are retained in checkpoint-chirho/next-reference-chirho.jsonl.
The checked all.T directive at line360 also says compile_fail for T5481; its
file hash and exact directive are in checkpoint-chirho/oracle-note-chirho.json.

The structural gate then caught kind_chirho at16 entries after adding two phase
files. Move the three declaration-phase files (groups, module driver, GND adapter)
under phases_chirho using path-selected modules, preserving their logical Rust
scope and byte-identical contents. The kind directory is now14 entries and its
phase child3; deriving has7 focused children, all below1500 lines. This is a new
source layout and gets its own final-gates-chirho receipt and frozen binary;
the preceding passing run remains retained rather than relabeled as this one.

The final-layout receipt completes at14:43:51 EDT: naming138, parser374, TH18,
typing409, canaries7 and typing integration470, all zero failed/ignored/filtered.
The LIVE package passes separately (1 run,1775 filtered,83.54s). Formatting
passes, no compiler warning lines; clippy433 distinct warnings persist, none
with a primary span on a changed line. Frozen CLI
88c17197835dd4ccd6afc1b998f426a9ff5c284f05ebfddda0d84e5e9872063b
then matches all36 retained checks with stable hashes and no timeout, panic or
unexpected exit. All nine negative diagnostics were read: the intended unary,
eta-reduction or kind-arity errors, not a metadata bail-out. Final receipts are
final-gates-chirho, final-lint-chirho.json and final-replay-chirho.jsonl. The
complete diagnostic corpus pair still waits for the agreed explicit release.

That release was #24224. The final diagnostic pair (#24225 START / #24226 DONE)
now completes on88c17197: accept890/938, reject268/767, zero initial/unresolved
timeouts, runtime panics, unexpected exits or output limits. Every source and
binary hash is unchanged. Persistent observations and exact membership comparison
are under checkpoint-chirho/corpus-chirho. One pass per axis, not a landing gate.

All1705 current source hashes match the retained parent650be538 and first
candidate5c145678 observations. Against that parent: four accept gains
(ClassDefaultInHsBoot, A2, A3 and T18036b), no accept losses; one reject gain
(T6001), no reject losses. T6001 remains ADJACENT and predates this GND unit.
Against the first candidate: all9 accept regressions recover plus T18036b;
T15712/T26137's wrong-reason rejections disappear, not their missing GHC rules.
T12734 is recovered. Against main c74db426, the full-branch differences remain
16 accept gains/11 losses and59 reject gains/19 losses; they include older
row484 work and main's independent duplicate-instance lane. No overall
no-regression claim or waiver follows. Ready for an isolated owned checkpoint,
not main landing; canonical row484 and published measurements stay unchanged.

The isolated source/evidence checkpoint is committed and pushed as
f1c99aa687855eed8144ca5b47dbaeba38b5665e on gpt-kind-schemes-chirho;
ls-remote returned that exact SHA after the push. Main is still clean at
c74db426f9464b9e4e7a820713f0366a89938aaa. No canonical DB write, public
measurement change, main merge or runtime-coercion claim. The existing package
cache symlink remains untracked and was not staged. This checklist closes the
isolated verification checkpoint, not row484's outstanding main-line differences.

Read-only next-unit trace for T17067: the family lowerer recognizes both data
and type keywords but stores neither in TypeFamilyDeclChirho; the AST has no
form field. Declaration grouping consequently puts both in kind_family_names,
and exported/imported KindHeadShapeChirho also labels both FamilyChirho. The
equation-pattern guard then rejects either. Preserve the declaration form at
the producer and in checked import contracts before changing the guard; do not
special-case T17067 or make all family patterns legal. The next bounded controls
need local and imported data-family positives, a type-family negative and local
shadowing. No implementation or new capability is claimed by this trace.

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
