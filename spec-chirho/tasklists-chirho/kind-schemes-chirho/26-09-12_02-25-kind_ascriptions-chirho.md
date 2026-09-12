<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Kind ascriptions and promoted matching indices, row484

Isolated continuation from pusheddb60ff50, checkpoint tag
kind-ascription-indices-before-chirho. Main121d4f2c and canonical row484 remain
unchanged. Builder/DB lease retained; corpus CPU embargo is released.

## Brick 1: representation and acceptance

The CST retains `(_ :: ProxyChirho keyChirho)`, but lowering drops the kind and
TypeChirho cannot represent it. A new KindAnnotChirho node owns the annotated
type, written kind and span. Both children remain syntax through naming and
kind validation; only the type survives into type inference after its solved
indices have been recorded. A promoted head needs an explicit namespace in the
occurrence record; same-spelled type and promoted constructors are not one head.

The primary control is AscribedKeyChirho: an existential PackChirho contains a
ProxyChirho key, and a family extracts that key through the pattern annotation.
Two equality witnesses demand Int and Bool independently. GHC9.14.1 accepts;
the frozen3458 CLI rejects the missing matching variable and both witnesses.
The existing legal ClassifierCycle control remains enabled and must recover.
Unknown RHS variables and contradictory kinds/results must still reject.

The family clause's RHS must remain closed over REAL matching inputs. A bound
key belongs in the promoted constructor's invisible matching arguments, not in
the family head's unrelated hidden inputs. The reference model is GHC's
[existential promotion explanation](https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/exts/data_kinds.html#promoting-existential-data-constructors).

Placement: AST type node in ty_chirho.rs; shared parser implementation in the
existing lower_chirho/kind_annotations_chirho.rs; naming and signature visitors
visit both children; kind/elaboration and type conversion retain solved indices.
TH conversion preserves the node; runtime-only type-spine consumers transparently
unwrap it after checking. Audit wildcard consumers explicitly; a new enum variant
does not force wildcard arms to change. Keep work bounded by syntax/occurrence,
with no module-map clone per annotation. No new dependency or deferred data-family,
type-data or refined GADT-record declaration-shape decision is made here.

Confidence: high in producer loss and the fresh reference reduction, medium in
all consumer interactions. The pushed checkpoint bounds reversal. This is a
local/reversible choice within L.J.'s continue/fix-tests direction, not permission
to waive the corpus gate or edit its membership, public labels or site.

## Checklist

- [x] Full driver1774/1774 and frozen881/248 diagnostic recorded before this fork.
- [x] Independent read-only consumer audit; current producer loss checked directly.
- [x] Fresh GHC reference controls recorded; distinct-result, visible-quantifier and
  four additional producer controls demonstrated red before their repairs.
- [x] Retain both annotation children through parser/naming/TH and kind checking.
- [x] Record/materialize represented promoted constructor indices; keep clause closure checks.
- [x] Positive distinct-result execution, wrong result/kind/name and scope controls.
- [x] Recover ClassifierCycle without weakening its assertions.
- [ ] Inspect the remaining corpus regressions by reason after a frozen CLI diagnostic.
- [ ] Commit and push the ascription implementation and its reference records.
- [ ] Focused/full integration and typing gates; freeze, build explicit CLI,
  compare exact corpus sets before the final broad/two-pass landing gate.

The preceding import repair is fully recorded in26-09-12_01-47-import_contracts-chirho.md.
Its driver result does not mean this new implementation or all GHC tests pass.

## Measured implementation and recovery

Both TypeChirho and AstKindChirho retain ascriptions. The shared parser route
also covers bare alias RHS syntax and parenthesized instance/forall binders.
Four additional invalid annotation controls all passed incorrectly before those
producer/consumer repairs, then rejected; their four valid controls agree with
GHC9.14.1. Instance arguments are checked even when the historical extension
guard cannot establish the whole class-head contract. The guard still limits
that final class-head comparison; complete instance validation is not claimed.

Promoted head occurrences retain an explicit namespace. A GADT's promoted
scheme is inferred in its own checked forall scope. Anonymous source patterns
share one span-owned identity between kind checking and type conversion, which
recovers ClassifierCycle. Promoted list literals and explicit cons/nil uses now
carry the same solved element-kind indices; the first full integration exposed
two mismatches here (127/129), and the producer repair recovered both.

An ascription's visible quantifiers are not the provider's hidden indices.
Mono annotations hide subsequent @ slots; explicit poly annotations are checked
against rigid binders before fresh use instances are opened. Two new controls
failed before this repair (4/6), then all six passed. An independent read-only
audit found no concrete green-path identity/occurrence defect in this helper;
it did identify that a failed polymorphic contract was still opened afterward.
The recovery path now reports that error and stops elaborating that occurrence
without publishing the failed contract. No independently reproduced cross-
occurrence contamination is claimed from that defensive change.

The broader pre-recovery-edit gates were integration133, canaries7, parser357,
naming136, typing371 and TH17, all green with zero ignored/filtered. Two parser
assertions had explicitly expected erased annotations; they now compare the
whole retained type and classifier, not only a peeled prefix. Workspace all-
target check passed. Clippy completed with471 warning-message lines in this
selected-crate invocation, including summary/duplicate notices: zero lint
warnings and repository-wide structural compliance are NOT claimed. The large
parser root lost more lines than it gained; new integration tests live in a
small subject file. Final post-recovery-edit results are recorded beside the
reference JSONLs before checkpointing.

Evidence: ascriptions-chirho/reference-before-chirho.jsonl has five fresh GHC
and frozen3458 CLI comparisons; spine-reference-chirho.jsonl has seven GHC
observations; nested-reference-chirho.jsonl has eight. Each carries exact source
and hash. Only the two executable positive fixture sources are claimed as the
same GHC-checked programs executed by all three backends. The controls do not
establish arbitrary higher-rank subsumption, local-signature classifier checking,
complete TH binder conversion, imported promoted schemes or a static converter
without elaboration.

## Resume state

db60ff50 is the pushed pre-fork checkpoint. This implementation checkpoint
records final integration133/canaries7 and frontend136/357/17/371, all green
with zero ignored/filtered, plus workspace all-target check and format check.
Commands and log hashes are in ascriptions-chirho/ascription-gates-chirho.json;
raw logs are in /private/tmp/haskelujah-ascription-chirho.EgTd8V.
No CLI relink or corpus run has occurred on this implementation. Retained CLI3458 has SHA256
03e59700eb6073db34a585a7b27582861077641bb4c6b7f60d047dce9ed0534e.
Next: commit by named paths, push and verify, explicit CLI
build/hash, then one diagnostic pass per axis comparing exact sets. Main and
canonical row484 remain unchanged; the nine previous corpus regressions are
not assumed recovered from the focused greens.
