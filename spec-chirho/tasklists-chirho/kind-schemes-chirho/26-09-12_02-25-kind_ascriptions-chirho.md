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
- [x] Commit and push the ascription implementation and its reference records (250b22fb, remote exact).
- [x] Focused/full integration and typing gates; freeze, build explicit CLI,
  and compare exact corpus sets in one diagnostic per axis.
- [ ] Reduce new corpus failures; final broad/two-pass landing gate remains blocked.

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

## Frozen diagnostic at250b22fb

Explicit CLI build at250b22fbaeb62933c41ac57a609014dabf9e4f8b, SHA256
de6d497cf168c8156f709761e2e1a0762d83421f38823c8658e0565affc82db6.
One pass per axis with pure-shell output classification, P4/15s and serial
timeout reruns. Source clean and CLI hash identical before/after both passes.
Accept869/938; reject252/767; zero timeouts or unexpected exits. This is a
diagnostic, NOT the final two-pass gate or a public measurement supersession.

Versus compiler3458, accept gains T15079 and loses thirteen:
LevPolyResult, T11348, T12734, T12734a, T12850, T15772, T16204a, T16204b,
T18185, T19682, T23543, T26737, tc184. All nine previous main-relative
regressions remain, so versus main121d4f2c this is nine gains and22 losses.
The narrow greens did not cover this surface and cannot waive these failures.

Reject versus3458 gains T14904a, T14904b, T24470a, T3540 and tcfail215,
and loses T15552a. These are verdict movements, not yet audited matching-reason
capability. Full failing-verdict sets, deltas and list hashes are retained in
ascriptions-chirho/ascription-diagnostic-chirho.jsonl. Published counts, labels,
membership and canonical row484 are unchanged.

## Resume state

db60ff50 is the pushed pre-fork checkpoint. This implementation checkpoint
records final integration133/canaries7 and frontend136/357/17/371, all green
with zero ignored/filtered, plus workspace all-target check and format check.
Commands and log hashes are in ascriptions-chirho/ascription-gates-chirho.json;
raw logs are in /private/tmp/haskelujah-ascription-chirho.EgTd8V.
CLI250b22fb and its frozen diagnostic are recorded above. Next: reduce the
thirteen new failures by source/diagnostic and inspect T15079's recovery and
reject movements; do not weaken ascription or closure checks to recover totals.
Main and canonical row484 remain unchanged. SLOT/DB lease retained, corpus
CPU embargo released after both passes.

## Producer reconciliation after250b22fb: bounded next step

The new shared forall-binder route exposed a mismatch between the two syntax
representations: TypeChirho can express promoted constructors/lists, literals
and tuples, while the narrower AstKind converter returns None and drops the
ENTIRE binder. LevPolyResult is now E0101 on `a`, whose written classifier
contains 'BoxedRep. Preserve this syntax using an AstKindChirho::TypeSyntaxChirho
payload owning the existing TypeChirho subtree, not a second set of tuple/list/
literal nodes and not a fabricated kind variable. Naming, dependency discovery,
kind inference and TH reification must delegate that payload to their existing
type-syntax visitors in the same change. The old simple forms remain compatible.
This is reuse of the already checked kind-expression grammar, not an opaque
unrepresented marker or permission to ignore an unknown classifier.

This local/reversible representation completion is within the ongoing ascription
unit, with pushed6b56f3d0 as the checkpoint. Confidence is high on the measured
producer loss; its complete reach still requires focused controls and a corpus
diagnostic. tc184 separately shows a constructor context being mistaken for a
field; retain the context boundary rather than relaxing field kind checking.
Neither change introduces the pending data-family/GADT-record declaration shapes.

The first context gate showed the earlier diagnosis was incomplete: the CST
itself ends the constructor before the context arrow when forall is absent,
leaving a nameless Ordinary node. Move constructor/record CST parsing into
cst_parser_chirho/constructors_chirho.rs, with one delimiter-aware context
lookahead shared by explicit-forall and no-forall paths. Stop at declaration
boundaries and consume only the located outer arrow, not a nested constraint.
This removes code from the oversized parser root. Runtime evidence storage for
ordinary/record constructor contexts remains unrepresented and is not claimed;
the behavioral control measures surviving names, actual fields and read-back.

## Producer repair results and limits

TypeSyntaxChirho now preserves promoted constructors/lists, tuples and literal
classifiers without dropping their annotated binder. Naming, dependencies,
free-variable scope, kind conversion and TH reification consume the same payload.
The checked term interpreter distinguishes promoted nil/cons from the list type
constructor and no longer fabricates Type for an unmatched shape.

The constructor CST and lowerer now agree on an optional context before the
actual constructor. A GHC-verified source constructs two such values, reads
their real fields, and prints42/7 on STG, LLVM and Cranelift. Unknown implicit-
parameter payload names and unsaturated/unlifted payload classifiers reject.
This is not full constructor-context evidence storage: Ordinary/Record still
cannot retain that evidence and the runtime control does not claim otherwise.

Instance-argument and superclass token slices now use the shared flat grammar.
The old duplicate dropped the promotion tick in Stack '[] and literal arguments
in CmpSymbol/CmpNat applications; fresh paired controls reproduced both losses.
The common path retains tuple arity/application identity and source spans.
One old parser assertion required an incidental extra Paren around (,,); it now
peels parentheses while still checking the same constructor arity and argument.
No acceptance or canary assertion was weakened.

Retaining the binder exposed a separate source-authority defect in T12850:
GHC.Types did not export TYPE in our interface inventory. The inventory now
does so; fresh explicit/qualified controls accept while omitted/hidden imports
still reject. This is not a global wired-in naming exemption.

Twenty-two fresh GHC9.14.1 observations are retained in
ascriptions-chirho/producers-chirho, including exact source hashes and baseline
diagnostics. The mixed implicit-context control uses fChirho _ =1: the earlier
undefined body was rejected by GHC for impredicative instantiation and was not
a valid positive oracle. The separate coerce control is rejected by GHC-18872;
our accepting it does not prove implicit-parameter role/evidence correctness.

Current producer gates: naming136, parser359, TH17, typing371, integration141
and canaries7, all green with zero ignored/filtered. These do not establish full
driver or corpus health. First focused CLI4afc8474 recovered LevPolyResult,
T12734, T18185, T26737 and tc184 from the22 known main-relative failures;
after the interface repair CLI7bb16c44 also accepts T12850. T12734a now reaches
a type-level promoted-cons index mismatch, rather than the former producer
kind error. The other failures remain live. This is focused evidence, not an
inferred full-corpus count. Next: freeze this owned checkpoint, explicitly build,
run one diagnostic per axis, and compare exact sets before the next reduction.
Main121d4f2c, canonical row484, corpus membership and public labels stay unchanged.

## Frozen producer diagnostic at351bdbb3

Implementation351bdbb36202ebe5b8efa04dcf4cd410b3c33617 is pushed, remote exact.
Local tag kind-producer-reconciliation-before-chirho points to6b56f3d0.
The post-commit explicit build retains SHA256
7bb16c445516e1f32e522c120bbcea7de2aa705d7acd3c0d5f7af9a1f0ad4bab.
One diagnostic per axis, P4/15s with isolated timeout reruns, clean source and
unchanged binary before/after: accept875/938 and reject252/767, zero timeouts or
unexpected exits. This is not the final two-pass gate or artifact supersession.

Versus250b22fb, accept gains eight: LevPolyResult,T10856,T12734,T12850,T18185,
T18831,T26737,tc184; loses T26256a and T7903. Thus six net recoveries but two
newly failing files, not a clean repair. The two new diagnostics both involve
the function constructor (->) on the shared instance-token route. Main-relative
membership is now ten gains and17 losses; no failure is waived.

Reject total252 is unchanged but its membership is not: +T18714,-T12102.
Both GHC stderr files demand GHC-01259, illegal constraints in a kind. Our new
T18714 error is instead an escaping polymorphic-ascription binder, so it is not
credited as implementing that contract. T12102 now passes; the actual illegal-
kind-constraint rule remains unimplemented. Exact sets/deltas/hashes are in
producers-chirho/diagnostic-chirho.jsonl. Broad driver results from3458 do not
cover351bdbb3. CPU embargo released; SLOT/DB remains mine while the instance
grammar and remaining list occurrence/equation-index failures are reduced.

The next measured reduction isolates both new failures: a class instance for
(->) and a partially applied, ascribed (->) both pass GHC9.14.1 and fail351bdbb3.
The shared atom grammar must own the parenthesized constructor boundary. Retain
the outer instance group's delimiters until that grammar sees them; recognize
the single arrow inside a parenthesized atom, not a bare arrow as a valid type.
This completes the shared grammar rather than restoring the duplicate lowerer.

Fresh GHC9.14.1 controls now distinguish two additional missing contracts.
Promoted-list literals at expression-level @ applications fail while identical
signature literals carry the solved index; empty, singleton, explicit-cons and
two-element forms all fail351 and execute42 under GHC. Instantiate the existing
promoted-head contract once for an unvisited literal, sharing its one index
across every cons and nil. Do not invent a head for an unknown import or claim
that this supplies the missing expression-wide kind-checking traversal.

For family RHS constraints, DirectNil and Reflexive controls pass both compilers,
but xs ~ '[] fails351's closure check and xs ~ 'True wrongly passes it. GHC accepts
the former and rejects the latter for operand kinds. The builtin homogeneous
equality contract is absent: seed (~) with forall k. k -> k -> Constraint in the
existing environment, so its operands share the same classifier. Keep the RHS
closure rule unchanged; an orphaned nil index is not a new matching input.

Parser360 is green after the arrow repair, but its first exact execution control
exposed a missing STG binding for the instance body alias (.). That failure remains
open; a passing typecheck is not an execution claim.

The first-class composition gap is outside the instance mechanism: its dictionary
contains the actual method body, but the Prelude body registry lacks (.) while
typing supplies its scheme. Repair that registry, not instance aliases or STG's
missing-global diagnostic. Extract the basic combinator catalog and registration
loop from the17,538-line prelude_chirho.rs into prelude_chirho/functions_chirho.rs,
then generate the ordinary three-lambda composition body there. This removes
roughly240 lines from the root; it is not a claim that the remaining structural
debt is resolved. Preserve source-body precedence and test laziness as well as
the direct alias and the original instance dispatch.

The broad driver gate exposed one additional regression already present at351:
the heterogeneous-equality alias's outer RHS annotation uses a kind variable not
on the LHS. GHC9.14.1 still accepts this with GHC-16382, warning that a future
release will reject it. It is not a bad test today. Eight fresh controls show
that only the OUTERMOST RHS ascription introduces this legacy kind scope; nested
annotations, free RHS type variables, unknown nominal kinds and sibling/forall
scope escapes still reject. The naming pass now binds only free variables in
that outer classifier for this one alias's scope, using its existing collector.
The original driver test is unchanged. Future GHC policy and warning promotion
are not claimed by this compatibility repair.

## Occurrence follow-up results and next gate

The shared parenthesized-arrow path now recovers T26256a and T7903. Promoted
list occurrence indices recover T12734a; homogeneous equality recovers T15772
and T19682. These are five focused checks, not an inferred corpus count.
The equality RHS closure check is unchanged. First-class composition now has
a real Prelude body; direct, aliased, unused-argument and source-shadow controls
execute on STG, LLVM and Cranelift, as does the original function-instance
program. Their independent GHC9.14.1 observations, the list controls and the
eight alias-scope cases are retained as24 observations in producers-chirho.

The full driver run completed1773/1774 with zero ignored/filtered BEFORE the
alias naming repair. Its sole red is the unchanged heterogeneous-prefix alias
test; a fresh focused run passes it afterward. Alias controls8/8, naming136,
integration162 and canaries7 are green. The last two suites have zero
ignored/filtered. A full driver rerun on the repaired source is still owed;
1773 plus one focused pass is not being relabelled a1774 full-suite result.

Workspace all-target check, explicit CLI build and format check pass. Selected
all-target clippy completes with471 warning-message lines including summaries
and duplicates; this is not lint-clean. The relocated Prelude catalog's type-
complexity warning is repaired using a named body type, not suppressed. Its
composition controls pass afterward. Large pre-existing roots and directory
debt remain; the new source files are bounded subject modules.

Commands, exact result scopes and log hashes are in
producers-chirho/occurrence-gates-chirho.json. Next: save this isolated owned
checkpoint, run one frozen diagnostic per axis and compare membership against
351bdbb3 and main; then repeat the full driver gate before the next meaningful
landing. The full workspace execution and final two-pass gate remain owed.
The next root under read-only review is open-family row ownership, dependency
ordering and hidden-argument retention; no rows are registered speculatively.
Main121d4f2c, canonical row484, corpus membership and public labels are unchanged.
