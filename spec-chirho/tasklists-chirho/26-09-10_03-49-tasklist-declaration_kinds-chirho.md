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
- [ ] Rebase onto verified flat-list landing, freeze, full workspace and two passes per axis.
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
