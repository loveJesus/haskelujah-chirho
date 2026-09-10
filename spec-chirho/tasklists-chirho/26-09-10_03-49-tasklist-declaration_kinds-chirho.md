<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Data declaration kind signatures — 2026-09-10

L.J.'s overnight continuation; HASKELUJAH/gpt_chirho, canonical row 483,
start 2026-09-10T07:49:58.254Z. Local/reversible compiler work in isolated
gpt-kind-signatures-chirho, checkpoint tag kind-signatures-before-chirho at
cb939e35. The flat-list source remains frozen in its own workspace gate.
No build load was added while that gate ran; row482 has now landed at 0093dd40.
No public-label policy/deployment.

## Brick 1: decision and scope

Measured GHC 9.14.1/current CLI disagreement: an inline result-kind tail is
mistaken for the complete kind, discarding head parameters (T11811 and a
polymorphic constructor reduction). A complete standalone signature plus an
inline tail is legal; lowering silently chooses the tail and drops the complete
signature. A complete signature requiring a higher-kinded head parameter is
also wrongly accepted when the declaration explicitly annotates it Type.

Recommendation: retype the existing optional data/newtype kind-signature field
to distinguish an inline result kind from a complete standalone signature,
retaining both when both are written. Compose only result tails with head
parameter kinds. Check standalone head parameters and an optional inline tail
against the complete signature, with separate lexical kind identities joined
structurally, not by coincidental variable spelling. Naming visits both source
annotations in their actual scopes. This is not one of the deferred data-family,
type-data or refined-GADT-result representation decisions.

Confidence: high in the reproduced distinction, medium in integration until
the positive/negative controls and corpus gates run. Alternatives considered:
prepending parameters to every signature double-counts complete signatures;
retaining only one of two legal annotations loses a contract; a source-name
guard cannot recover lost information. Correction cost is a local AST field and
its finite producers/consumers on this branch; the checkpoint remains intact.

Placement: a small declaration-kind enum beside DeclChirho; related kind
declaration conversion under kind_chirho/declarations_chirho.rs and separate
tests. Something is wrong with the remaining oversized kind/parser roots:
extract the relevant unit rather than add another long arm. No new dependency
or broad warning/file-split sweep; existing size/lint debt remains open.

## Checklist

- [x] Isolated checkpoint, canonical row, current source and GHC reproductions.
- [x] Read-only consumer audit and explicit AST/parser/typing regression controls.
- [x] Preserve both annotations, separate scopes, compose/reconcile declaration kinds.
- [x] Focused parser/naming/typing and exact-output driver controls; genuine negatives.
- [x] Rebase onto verified flat-list landing, freeze, full workspace and two passes per axis.
- [ ] Named-path commits, branch/main pushes with readback, main CLI smokes, DB closure.

## Evidence and limits

Scratch /private/tmp/haskelujah-kind-signature-chirho.AQnLxk;
initial-probes-chirho.jsonl records all observed exits and diagnostics.
CombinedKindSignatureChirho: GHC accepts; ours E0300.
MismatchedHeadKindChirho: GHC-83865; ours accepts.
MismatchedCombinedKindChirho: both reject, but our current rejection is arity,
not proof of reconciliation. Existing InlineKindTailChirho and
CompleteKindSignatureChirho controls live in the preceding kind-scope scratch.

No claim of rigid kind skolems, complete kind schemes, arbitrary named kinds,
full polymorphic recursion or imported authoritative kind metadata. The separate
rank-N callback false acceptance and GHC-10107 warning-policy gap are not this unit.

Source-stage evidence: the independent producer/consumer audit confirms two AST
fields, three genuine syntax producers, two attachment arms and four read arms
funneled through shared naming/kind consumers. Six synthetic Some literals were
migrated as inline result annotations; None literals retain their meaning.
The payload is Result(Type) or Standalone { complete, optional result }, keeping
all three nonempty states without an empty carrier. Relevant helpers were
extracted from the oversized parser/kind roots; no broad split is claimed.

GHC 9.14.1 rejects StandaloneRemainderChirho and StandaloneEtaChirho with
GHC-83865: a complete signature does not implicitly fill missing head binders.
DroppedCompleteKindChirho is also GHC-83865, but frozen cb939e35 accepts it
because the inline Type tail replaces the complete Type -> Type signature.
DeclarationKindContractsChirho independently runs 42/7/11/13 under GHC without
warnings; its identical source is now a three-engine driver control, UNRUN here
until row482's workspace completes. Production changes and new tests are still
unbuilt at this source checkpoint.

First focused build caught two remaining parser namespace tests destructuring
the now-typed payload as Type directly. They now explicitly inspect the inline
result; their required-forall/operator assertions are unchanged. The retry
passed naming 134, parser 331, typing 341 with no warnings. The new cache-scope
unit was then narrowed to an unannotated head: pinning acceptance of a Type-
annotated head under `type Box :: k -> Type` would encode a known false accept.
Fresh GHC reports GHC-25897 (rigid k) for that annotation and accepts the
unannotated control; baseline cb939e35 wrongly accepts the annotated one.
Rigid standalone kind quantifiers remain a measured separate gap, not hidden by
this provenance/composition repair. The revised full typing suite passes 341/341.

Two read-only reviews found no remaining scoped semantic blocker. They prompted
two stronger controls: exact source-slice assertions for complete and inline
kind spans (the old composite fallback covered the whole declaration), and a
reciprocal inline-retention mismatch. Fresh GHC-83865 rejects
`type Box :: Type; data Box :: Type -> Type where MkBox :: Box Int` at the
declaration; the driver must likewise reject at declaration-signature
reconciliation, not merely because a constructor application later fails.
This complements the dropped-complete negative. The positive combined execution
alone is not proof both redundant annotations are retained.

Explicit pre-existing limits from the parser review: symbolic standalone kind
signatures, attachment to non-data/newtype declarations, duplicate signatures,
and orphan-signature diagnostics are not covered by this named data/newtype unit.
Legacy unit names saying "standalone" may represent inline `data T :: K`;
the new declaration tests, not those names, establish the top-level `type T :: K`
path. Driver, exact-span rerun and full corpus gates remain owed.

Final focused rerun: parser 332, naming 134, typing 341; driver typing integration
29 (including the new identical-source STG/LLVM/Cranelift 42/7/11/13 execution),
canaries 7; all zero failed/ignored/filtered and actual Cargo exit zero.
The exact-span tests cover both data/newtype producers and both written contracts.
Fresh explicit CLI A/B against the row482 main binary confirms T11811 flips to
acceptance; all five GHC-invalid declaration controls flip from wrong acceptance
to declaration-signature mismatch. T14048a stays rejected for Constraint.
CLI and reference source digests are retained in cli-ab-focused-chirho.jsonl;
the extra reciprocal reference is DroppedInlineKindChirho-ghc.log.

Rebased by fast-forwarding the worktree base from cb939e35 to the completed flat
landing 0093dd40, with owned source edits retained. Rustfmt check passes. Five-crate
(ast/parser/naming/typing/driver) all-target clippy exits zero with 687 warning
messages (duplicates included), none at changed primary spans or new modules.
This wider target set is not directly comparable to the previous 650 count.
The parser root is still 17298 lines and the kind root 3278; neither is compliant
with the size gate. New helpers/tests are 65/142 and 107/210 lines respectively.

Pre-full-gate prediction: T11811 is one confirmed accept candidate; the reject
controls are reduced examples, not a measured corpus gain estimate. Every full
set delta will be named and checked against its source/GHC reason. Full workspace,
four frozen corpus passes and main landing are still pending.

## First frozen gate: not landable; binder representation extension

Checkpoint 63caa4cf, pushed, CLI SHA256
30b270b6773c8f191ce8781f7cff03d7f211bc3af429379d062036150f4dedb1:
two identical accept passes give 879/938, gaining T11811/T20873 but losing
T22560a/T22762/T23514c. Two identical reject passes give 225/767: gains
T22560_fail_b/T22560_fail_c/VisFlag1/VisFlag1_ql, loss tcfail225. Zero
timeouts/unexpected exits; per-file reasons remain under review. These are trial
results, not superseding the main 880/222 artifacts. Full workspace is deferred
until the three false accept-axis regressions are repaired.

The regression reductions identify two erased distinctions: data/newtype `@`
head binders are currently retained as ordinary positional binders, while the
kind interpreter treats visible `forall a ->` as invisible `forall a.`. Correct
reconciliation exposes that wrong arity. Weakening reconciliation would restore
the old false acceptances in the five independently rejected controls.

Brick-1 extension, local checkpoint declaration-binders-before-chirho at
63caa4cf: retain head binder visibility in the AST and parser; keep all binders
in lexical maps, but use only visible parameters in kind arrows, constructor
result applications and derived instance heads. Interpret required-forall kinds
with their visible argument arrows. Audit consumers before editing; visibility
is not inferred/specified specificity. A shared binder API and small focused
helpers avoid new long arms in the already oversized roots. No extra dependency,
no change to deferred data-family/type-data/GADT shape decisions. Confidence is
high in the reductions, medium in integration; correction cost remains the
isolated owned branch and a finite consumer sweep. The five negatives, three
upstream regressions, exact-output controls and full corpus gates must all hold
before main is eligible to move.

Binder extension focused evidence: fresh GHC 9.14.1 accepts all three regression
files, both gains and T17705. Two same-source field/type controls execute under
GHC; two exact Int-to-Bool field mutations reject GHC-83865. Fresh current CLI
recovers all three regressions and keeps T14048a. The trial's extra VisFlag1,
VisFlag1_ql and duplicate-binder rejections disappear again; none were banked.
The older required-forall kind unit explicitly asserted erasure of a visible
parameter; it is corrected to the visible-arity contract, supported by the GHC
execution control. Term-type kind inference is deliberately unchanged.

Found on the way, NOT FIXED/NOT IGNORED: the strengthened Eq/Show execution probe
exposes an existing generic record deriving defect even after removing both
`@jChirho` binders (OrdinaryDataBindersChirho.hs in scratch). Eq evaluation fails
with missing STG binding `==` for CoreId 19. With equality calls removed, GHC prints
`MkBoxChirho {boxValueChirho = 42}` and `MkWrapperChirho {wrapperValueChirho = 7}`;
ours prints `MkBoxChirho 42` and `7`. The new binder control therefore separates
actual field read-back on three engines (GHC oracle 42/7) from derived-instance
typechecking; it does not substitute our formatting as an oracle or claim Eq/Show
execution. The failing full probe is retained in scratch for a subsequent
execution-correctness unit. No previously committed test is removed or waived.

Read-only consumer audit: naming retains all lexical binders; local constructor
and selector result applications share the visible-parameter helper; inline and
standalone deriving filter once at four entry boundaries, including the last
visible Functor parameter. Interfaces contain names/members only, and core uses
term fields only. Remaining fidelity limits are explicit: TH binder reification
has no visibility representation; HM schemes lack kind-dependent quantifier
metadata, so exact constructor/selector visible-type-application ordering is not
claimed; cross-module interfaces have no authoritative kind/visibility contract.
Parser wildcard/unreadable-kind matrix cases are recovery/representation fixtures,
not a claim that every such synthetic declaration is accepted by GHC.

Further explicit boundary control: GHC accepts an implicit kind quantifier
(`type Box :: k -> Type; data Box @j a`), and rejects
`type Box :: forall k -> Type -> Type; data Box @j a b` with GHC-57916.
The latter can have the same visible arrow count and is still wrongly accepted
here: full matching of invisible binders to specified forall binders is not
implemented by arrow reconciliation. Consequently T22560_fail_b's current
rejection is a generic declaration-kind arity mismatch, not a claim of complete
GHC-57916 support. tcfail225 needs rigid GHC-25897 kind checking; its previous
rejection came from misattributing its inline tail as the complete kind.

Binder-stage final focused gates: parser333, naming134, typing344, driver
typing-integration33 and canaries7; all actual Cargo exit zero, no failed,
ignored or filtered tests. The three-engine new controls execute field reads
42/7 and the required-kind parameter case19; the prior combined contract case
42/7/11/13 remains green. Fresh GHC reference records are in
binder-references-chirho.jsonl, with the additional field-read reduction rerun
under GHC separately. Main0093dd40 independently reproduces the ordinary Eq
failure, confirming it is not caused by this branch. Five-crate all-target
clippy exits zero with685 warning messages, no changed-primary-span warning;
the wider repo is not zero-warning or size-compliant. Source and oracle
checkpoint only: the new frozen full workspace and four corpus passes remain
owed, and main remains0093dd40.

## Final frozen gate and landing evidence

Source3db3b6a69e30d59b60224346b541c245cc68b3e0, pushed and remote-exact.
CLI SHA2569168f50b07a67d23a8ff45f4bb8f2a8690355b44e71ed9bd44dbf3a64d5ec703
stays unchanged through all four corpus passes and the full workspace gate.
Actual corpus runner exit0: accept882/938 (+T11811/+T20873, zero losses);
reject221/767 (+T22560_fail_b, -tcfail225/-ExplicitSpecificity8). Both repeats
are byte-identical; zero timeouts/unexpected exits. Exact current list hashes:
accept e56823120d250c62966e0befef42c336663a9d28ce0dd3f08d55f51a6094c078;
reject 4063e8964e4ffc621a9df426227d2d56d01ff3cbdc943b042f962c707aae693c.

The extra reject loss was investigated before landing. On main0093dd40,
ExplicitSpecificity8 errors at the T1/T2 applications on lines11/14, not at either
declaration. The valid T1-only reduction is rejected by main and accepted by GHC
and current. The invalid `forall {k} -> k -> Type` declaration WITHOUT uses is
already accepted by main/current; GHC9.14.1 rejects GHC-57342. Nine bounded,
source/binary-hashed observations are in scratch/specificity-loss-chirho.jsonl.
That rule is missing, not lost implementation; no guard preserves the false arity.

Fresh GHC also confirms tcfail225's GHC-25897 and T22560_fail_b's GHC-57916.
Six A/B/reference observations are in scratch/reject-delta-references-chirho.jsonl.
The same recursive type body passes with CUSKs and rejects with NoCUSKs under
GHC9.14.1 (CompleteKindRecursionChirho/IncompleteKindRecursionChirho in scratch).
This is a declaration-completeness/recursive-instantiation distinction, not merely
the standalone rigid-contract gap. Both need proper kind binding metadata; neither
is repaired by the current arrow reconciliation. T22560_fail_b's generic arity
rejection remains explicitly short of full invisible-binder matching support.

Unfiltered workspace actual Cargo exit0:3387 passed across75 targets; zero
failed/ignored/measured/filtered. Driver1773/1773 includes126 native round trips;
curated537/537 includes514 compared oracles and23 compile-only inputs. All514
source hashes match the prior complete GHC9.14.1 manifest, not514 new runs.
Workspace raw log SHA256
ce8c45abd24f2ffba344f0bd64f5e0b7aaafdaab6d62c3d5c3ed05947b244d12.
Five-crate clippy remains685 messages, zero primary spans on any owned source
change relative to0093dd40; no zero-warning/size-compliance claim.

Evidence supersede prepared; canonical DB closure and main-path rebuild/smokes
remain pending until their actual results. No percentage-policy choice or deploy.
