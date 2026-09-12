<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Data/newtype kind contracts

An inline `data T a :: K` annotation describes only the remaining result kind.
A separate `type T :: K` describes the complete kind in a separate lexical
scope. `DeclKindSigChirho` retains either or both; absence is not a fabricated
annotation. This distinction is shared by data and newtype declarations.

```mermaid
flowchart TD
  SourceChirho[Source declaration and optional standalone signature] --> LowerChirho[Retain complete signature and inline result separately]
  LowerChirho --> CompleteNamesChirho[Resolve complete signature outside head binders]
  LowerChirho --> HeadNamesChirho[Bind head variables then resolve inline result]
  CompleteNamesChirho --> CompleteKindChirho[Convert complete kind with independent kind-name cache]
  HeadNamesChirho --> HeadKindsChirho[Convert parameter kinds and inline result in one head scope]
  HeadKindsChirho --> ComposeChirho[Build an incomplete head with retained binder dependencies]
  CompleteKindChirho --> ReconcileChirho[Open rigid contract and consume each visible head binder]
  HeadKindsChirho --> ReconcileChirho
  ComposeChirho --> FieldsChirho
  ReconcileChirho --> FieldsChirho[Check constructor fields with head variables still in scope]
  LowerChirho --> ConstraintChirho[Reject written Constraint return kind on either contract]
  FieldsChirho --> GroupContractChirho[Check inference-group written variables before publication]
  GroupContractChirho --> ResultChirho[Kind environment and source diagnostics]
  ConstraintChirho --> ResultChirho
  LowerChirho --> VisibilityChirho[Keep every head binder and its visibility]
  VisibilityChirho --> SchemesChirho[Open all binders but apply visible parameters in constructor and selector types]
  VisibilityChirho --> DerivingChirho[Visible parameters form derived instance heads]
```

The standalone conversion swaps the kind-name cache instead of cloning the
growing environment. Fresh identities and accumulated substitutions survive;
the prior name cache is restored. Head annotations and an inline result share
their local kind identities. Every head binder owns a term identity after its
own annotation has been read. A complete signature is opened once and consumed
one visible binder at a time: a dependent domain substitutes that rigid head
term into the remaining contract; an ordinary arrow consumes only its domain.
The remaining contract then constrains the inline/default result. This uses
kind equality, not matching variable spellings or flattening a dependent
contract to ordinary arrows. The standalone lowerer consumes only the
declaration's first `::`; nested forall-binder annotations must survive it.
The cache swap is
O(1); this does not claim the whole module kind pass is linear.

Without a complete standalone contract the no-inline default is Type. The
isolated runtime-kind continuation lets a complete contract determine the
result's TYPE representation, but never invents another head argument. GHC 9.14.1
rejects `type T :: Type -> Type; data T where MkT :: T Int`: a missing head
argument is not supplied implicitly by that complete signature. Conversely,
`data T :: Type -> Type where ...` has a written result arrow and zero head
parameters, so its arrow is retained. Complete kinds are never prefixed twice.

Declaration-head `@a` binds a lexical variable without consuming an ordinary
type argument. `TyVarVisibilityChirho` preserves that distinction; shared parser
group handling prevents annotation tokens from becoming extra parameters. Kind
composition consumes only visible parameters as ordinary arrows. Constructor
result types additionally retain the solved invisible indices described below;
naming and field conversion keep every binder in scope. Deriving filters
at its entry boundaries, borrowing ordinary binder slices and copying only a
slice that actually contains invisible binders. This is linear in the head, not
in the growing module environment.

When interpreting source as a kind, `forall a ->` retains a visible argument
classified by the binder's kind; `forall a.` does not. This differs from inferring
the kind of a quantified term type, where both forms have the body's kind.
See GHC 9.14.1's [type-declaration binders](https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/exts/type_abstractions.html#invisible-binders-in-type-declarations)
and [required arguments](https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/exts/required_type_arguments.html).

## Isolated kind-binding prototype (row484)

The following is implemented on the isolated kind-schemes branch but is not a
main-line compatibility claim: the tasklist records known accept regressions
and gates still owed. The environment distinguishes monomorphic inference
bindings from polymorphic schemes with an explicit quantified-variable set.
Each use instantiates a scheme; substitution cannot rewrite its bound variables.
Written complete contracts are checked with rigid identities, not general
metavariables that can silently specialize.

Promoted data-constructor contracts use a separate kind namespace from
same-spelled type constructors. Known schemes instantiate at each occurrence;
an occurrence without constructor metadata gets an independent opaque kind,
never a shared monomorphic entry in the type namespace. This fixes accidental
cross-occurrence specialization, not full promotion checking: ordinary-constructor
lowering still discards some existential binder/context information needed to
produce complete promoted schemes.

```mermaid
flowchart TD
  RegistryChirho[Prebind local declaration heads] --> GraphChirho[Collect scoped kind dependencies]
  GraphChirho --> OuterGroupsChirho[Iterative dependency SCCs]
  OuterGroupsChirho --> PrepareChirho[Prepare heads and journal local scopes]
  PrepareChirho --> CompleteChirho[Publish complete signatures for recursive uses]
  CompleteChirho --> InferenceChirho[Remove completed edges and infer remaining SCCs]
  InferenceChirho --> BodiesChirho[Check all constructor bodies and class constraints]
  BodiesChirho --> WrittenChirho[Allow written-variable aliases but reject specialization]
  WrittenChirho --> PublishChirho[Rigidify checked contracts and publish the whole group]
  PublishChirho --> SignaturesChirho[Check consuming value signatures]
```

Dependency collection is phase-specific. The iterative SCC engine is shared
with value inference without changing value dependency collection. Scope
journals retain changed entries rather than clone the growing environment for
each declaration. The graph engine is O(V+E); that does not bound all recursive
kind conversion or claim that every language form has complete dependencies.

Effective defaults follow GHC2021 in the absence of an explicit edition:
PolyKinds enabled and CUSKs disabled. Ordered legacy-edition and extension
overrides are respected. With NoPolyKinds, a legacy complete-looking head must
still contribute body constraints before defaulting; an explicit standalone
contract remains complete independently. Incomplete written variables are
tracked through the whole inference SCC, so two written variables may be
equated but neither may become Type or an arrow. Complete contracts instead
use immediate skolems. Publication happens after the relevant group is checked.

Inline result-kind scope differs from a standalone signature: implicit kind
variables are still allowed alongside an inline forall; forall-or-nothing
applies only to the complete standalone signature. Elaborating the inline kind
captures actual source-variable identities, not every free variable in its
intermediate result. Application-result metas and unknown constructor kinds
are not written names. Only captured identities unresolved after elaboration
become contracts against subsequent body inference; head annotations retain
their original checking identities. This provenance is not a substitute for
a kind-term representation.

The isolated prototype distinguishes nominal terms, applications and dependent
functions from the kinds classifying those terms. Kind equality/substitution
live in a focused module. A required kind argument substitutes its term into
the result; supplying Bool is not equivalent to supplying Type simply because
both are classified by Type. Bound terms use lexical de Bruijn positions,
not inference-variable ids: substitution respects nested scopes and shifts
free argument references, and equality forbids exporting a local binder into
an outside inference hole. Scheme generalization/defaulting leave bound terms
alone. One scoped traversal owns these boundaries.

```mermaid
flowchart LR
  WrittenTermChirho[Written kind term] --> AbstractChirho[Abstract required binders into lexical positions]
  AbstractChirho --> SchemeTermChirho[Store term separately from its classifier]
  SchemeTermChirho --> DemandChirho[Check supplied argument against binder domain]
  DemandChirho --> SubstituteTermChirho[Substitute actual term without capture]
  SubstituteTermChirho --> ResultTermChirho[Check remaining application at dependent result]
```

AstKind now retains nominal names (including qualification and source spans)
separately from variables, and retains applications through lowering, naming,
dependency collection, kind conversion and TH reification. Nominal annotations
must resolve; they are not implicitly quantified holes. Reification preserves
these annotations, but the reverse TH conversion and binder visibility are
separate unfinished contracts. List/promoted annotation parsing is not yet
complete.

```mermaid
flowchart LR
  KindNameChirho[Nominal source kind and span] --> ResolveKindChirho[Resolve in type namespace]
  ResolveKindChirho --> ClassifyKindChirho[Validate applications in a temporary local scope]
  ClassifyKindChirho --> KeepTermChirho[Keep the kind term not just its classifier]
  KeepTermChirho --> RuntimeChirho[TYPE retains its RuntimeRep argument]
  RuntimeChirho --> ValueConsumerChirho[Functions and fields accept runtime value kinds]
  RuntimeChirho --> BoxedListChirho[Boxed lists still require lifted Type elements]
```

Runtime contracts and builtin literal kinds live in one focused child. TYPE
accepts a RuntimeRep argument; `TYPE Bool` is diagnosed before any value
consumer sees it. Type is the lifted boxed representation, while unlifted
and primitive representations remain distinct. Nat/Symbol/Char literals and
the supported literal-family contracts retain their respective kinds; promoted
lists preserve the common element kind instead of flattening to Type.
Validation scopes undo temporary bindings, and dependent applications interpret
an already-checked argument without recursively validating it again.
Finite representation constructors also carry their classifiers: Many/One are
Multiplicity, TupleRep/SumRep consume lists of RuntimeRep, and VecRep consumes
VecCount followed by VecElem. Ordinary and explicitly promoted occurrences use
the same contracts. TYPE Many and swapped vector arguments are errors, not
unconstrained nominal applications. These checks do not establish native vector
or unboxed-tuple execution support.

### Arrow multiplicities

The CST stores the atom after `%` inside `ArrowMultiplicityChirho`, not beside
the arrow's argument and result as another value type. The shared atomic type
grammar handles promotion ticks, parentheses, variables and parenthesized family
applications. Lowering retains an `ExpressionChirho` with its source span; only
bare `%1` and the Unicode linear arrow are fixed syntax sugar. Thus `%2` and
`%(1)` remain types requiring classification rather than becoming linear sugar.

```mermaid
flowchart LR
  MultiplicitySourceChirho[Percent plus multiplicity atom] --> MultiplicityNodeChirho[Dedicated CST child]
  MultiplicityNodeChirho --> MultiplicityTypeChirho[Retain AST type expression and span]
  MultiplicityTypeChirho --> MultiplicityScopeChirho[Resolve names and collect lexical binders]
  MultiplicityTypeChirho --> MultiplicityDependenciesChirho[Visit declaration dependencies]
  MultiplicityScopeChirho --> MultiplicityKindChirho[Require the Multiplicity classifier]
  MultiplicityDependenciesChirho --> MultiplicityKindChirho
  MultiplicityKindChirho --> MultiplicityConsumerChirho[Preserve fixed One syntax in existing consumers]
```

Ordinary spelling uses the type namespace first and only then the promoted
constructor namespace; explicit promotion uses the latter directly. Builtin
constructor classifier rows are not ordinary type bindings. A local constructor
displaces its same-spelled builtin row in the promoted namespace, independently
of a same-spelled type alias. Parameter-free nullary ordinary/record constructors
have a complete result classifier directly from their owner; other local forms
retain the explicit missing-metadata boundary rather than inheriting the builtin.
Qualification follows source import aliases. GHC.Types' Multiplicity interface
declares One/Many as members, so unquoted promotion still requires DataKinds.

Both type converters and the existing linearity consumer share recognition of
the fixed `%1`/quoted-One syntax (including parentheses). Unquoted aliases are
not assigned One from their spelling. Kind substitution visits annotation
expressions, but full multiplicity-term elaboration/reduction and TH reification
are not implemented. The current internal type representation still has only
One/Many; variable and family annotations are not retained there as symbolic
multiplicities. The existing linearity pass reports violations as warnings, not
GHC errors. A duplicated quoted-One parameter is therefore still a recorded
wrong acceptance, not a passing linearity claim. See the
[GHC 9.14.1 linear-type contract](https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/exts/linear_types.html).

Qualified kinds use the module's declared import aliases before consulting
builtin contracts; an arbitrary prefix is not stripped. The prefix function
constructor and arrow syntax both accept appropriate TYPE representations.
Arrow syntax and constructor-application kind terms unify structurally without
equating distinct nominal heads. Before an inference group is published,
unsolved representation and levity holes default to LiftedRep and Lifted;
source-named kind variables and complete quantified contracts are retained.
This follows [GHC's representation-defaulting rule](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/type_defaulting.html#kind-based-defaulting).
The group supplies its named variables once, so this does not scan the growing
environment on each declaration. This is not complete classifier metadata or
standalone-kind attachment for type aliases.

This is not complete kind-family or runtime-representation support. The
type-family reducer still lacks implicit kind indices. An unresolved family application cannot be treated as an arbitrary
fresh result, nor may hidden indices be replaced by newest-equation precedence.

Superclass kinds participate before class publication. Constraint lowering
delegates to the existing type/constraint conversion so a quantified or
variable-headed constraint is not silently discarded. A leading forall owns
the complete following type, including implication and arrow bodies. Known
standard higher-kinded class heads supply contracts; absent imported metadata
does not become a fabricated authoritative contract.

Each class method checks its implicitly quantified names in its own child kind
scope. Class-head identities and their inferred constraints remain shared;
method-local names do not leak into a sibling signature. The environment journal
undoes only scoped bindings, while the monotonic fresh-id boundary removes newly
introduced names from the declaration-local identity cache. It does not clone
the module environment or roll back constraints on class parameters.

```mermaid
flowchart LR
  ClassHeadScopeChirho[Shared class-head identities] --> MethodScopeChirho[Open one signature scope]
  MethodScopeChirho --> MethodCheckChirho[Check local implicit names and method kind]
  MethodCheckChirho --> ClassConstraintsChirho[Keep constraints on shared class parameters]
  MethodCheckChirho --> MethodCloseChirho[Discard only method-local names and bindings]
  MethodCloseChirho --> SiblingMethodChirho[Open next method independently]
```

This boundary does not extend the kind pass into local let/where signatures.
A local `% 'True` annotation and a non-nullary local constructor used as a
multiplicity remain measured wrong accepts. The former bypasses this pass;
the latter still lacks its promoted constructor classifier. Neither is a
validated multiplicity use merely because the top-level/class controls pass.

The local-instance consumer instantiates each class's quantified kind afresh,
infers argument classifiers in a journaled scope, and checks the resulting
application with kind equality. It no longer clones the whole environment or
compares only the old Star/Arrow tree shapes. Imported-class authority and the
historical extension-based instance-head representability guard remain outside
this unit: an explicit PolyKinds/FlexibleInstances control still demonstrates
that guard bypassing a wrong instance. A green reachable-path test does not
establish complete instance-kind validation.

A bare variable predicate remains a zero-argument predicate (`c`, not `? c`),
and its first use allocates a shared local kind even before an ordinary type
occurrence. Class/family/alias head scans consume whole annotated binder groups
with the same boundary helper as data/newtype. An unreadable annotation still
does not become faithfully represented, but cannot change the head's arity or
masquerade as the declaration's result annotation.

Implicit-parameter labels belong to the evidence namespace, not type-constructor
lookup. Retained constraint argument types are still visited. This does not
repair the older loss of implicit-parameter type annotations or claim faithful
dynamic evidence propagation; [GHC's implicit-parameter contract](https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/implicit_parameters.html)
is stronger than the current fresh-variable fallback. The row484 tasklist
records a GHC-rejected missing-payload-type control still accepted here.

Family equations and open instances now parse their complete left-hand side
with the normal CST type grammar, then split that application into the family
head and ordered patterns. Empty/nested promoted lists, literals, infix
constructors and parenthesized arguments are not rebuilt by separate token
scanners. The infix lowerer retains a written promotion tick in the AST;
an ordinary type constructor is not a promoted data constructor.

```mermaid
flowchart LR
  EquationSourceChirho[Closed equation or open instance] --> EquationCstChirho[Parse complete family application with shared type grammar]
  EquationCstChirho --> PatternSpineChirho[Keep head and ordered pattern nodes with source spans]
  PatternSpineChirho --> EquationVariablesChirho[Bind variables from this equation including nested promoted lists]
  EquationVariablesChirho --> EquationTypesChirho[Convert patterns and RHS in the same local scope]
  EquationTypesChirho --> FamilyRulesChirho[Register source-ordered equations]
  FamilyRulesChirho --> FamilyReductionChirho[Match patterns then reapply any extra result arguments]
```

Equation variables do not come from the family declaration's parameter names.
Open and closed registration share the same conversion; a local variable under
`Maybe` or a nested promoted list must remain matchable. The variable collector
visits each pattern node once with a set for deduplication. This does not claim
global linear normalization or complete unresolved-wanted reporting. The existing reducer's support for extra
arguments is preserved, not disabled to compensate for missing patterns.

### Local closed families in kinds (isolated row484 continuation)

Family results retain the optional kind, named result binder and dependency
annotation together. An explicit closed flag distinguishes `where {}` from an
open declaration. Equation patterns/results contribute dependency edges before
declarations are checked. Kind and type consumers adapt their terms to one
ordered matcher: a blocked earlier row cannot select a later catch-all, and a
stuck family is not nominally injective.

The family's kind slot uses the shared `DeclKindSigChirho` contract, so complete
standalone signatures are attached as complete heads, never inline tails.
Naming visits a complete signature outside the declaration-header scope; both
written contracts contribute dependency edges. `prepare_kind_head_chirho`
shares telescope application and binder reconciliation with data/newtype, while
family declarations retain their own result-kind and reduction-arity policy.
For example a zero-parameter family may return Maybe, and a family may return
Constraint; neither is a data-declaration rule. A full signature does not add
equation arguments. Ordinary family inference remains separate when no complete
signature is written. This does not supply missing hidden argument consumers.

That ordinary family path now uses the same head identities and telescope
composition as data declarations. In `data family F (k :: Type) :: k`, the
result refers to the supplied argument; it is not independently quantified.
`F Bool` consequently has kind Bool, and a family RHS with a remaining dependent
binder cannot replace an unrelated outer result with that binder. The shared
composition abstracts only actual occurrences, preserving non-dependent arrows.

Standalone-kind dispatch recognizes a complete declaration name: a constructor
identifier or a parenthesized type operator, followed immediately by `::` modulo
trivia. It does not scan through header parameters or following declarations.
`type (+++) :: ...` therefore retains the same complete contract as its prefix
equivalent instead of becoming a fabricated alias. Dispatch lives with the
existing family parser rather than growing the large parser root. A retained
signature matters to checking separate indexed equations, not only AST shape.
Parenthesized type-term annotations such as `(F Bool :: Type)` still have a
separate missing AST/consumer contract. Explicit kind applications are retained
by the isolated continuation below, with family-index consumers still incomplete.

Equation checking instantiates the actual complete scheme for each row. It
does not reconstruct its quantifiers from independently scoped header names.
`consume_kind_argument_chirho` applies both declaration and equation arguments:
a dependent result substitutes the supplied term, an arrow only checks its
classifier. The same fresh wildcard term feeds both substitution and the stored
pattern. Unknown dependent terms produce an error, not a guessed result contract.

A row's RHS may infer its implicit indices or anonymous patterns: GHC accepts
`Pick x = Int` at `Pick :: forall k. k -> k`, and accepts `Cast _ _ Refl x = Int`
when Cast's explicit indices are inferred as Type. Making all row variables
rigid is not sound. Fixed Bool indices still reject an Int result. Reduction
therefore stores the substituted patterns, not their unsolved predecessors.
An all-variable row may omit hidden inputs only when whole-row checking leaves
each input distinct and unconstrained and the stored RHS uses only bound
pattern variables. Otherwise reduction remains unrepresented. Constructor
patterns with hidden indices are not projected by this limited uniformity proof.

Represented context-free GADT signatures publish their promoted classifiers,
including the result index, after the constructor has been checked. Unsupported
contexts and higher-rank fields remain opaque rather than acquiring a fabricated
owner-only classifier. Local constructors shadow the seeded Refl classifier;
that classifier carries the same argument on both sides of homogeneous equality.
Stored type equations and signature conversion share the explicit-promotion name
constructor, retaining namespace and qualification. This does not claim complete
unticked promotion resolution or cross-module constructor-kind interfaces.

```mermaid
flowchart TD
  FamilyMetadataChirho[Retained result binder kind dependency and closed form] --> FamilyScopeChirho[Fresh equation scope and classifier checking]
  FamilyScopeChirho --> RowSchemeChirho[Instantiate complete scheme and consume dependent arguments]
  RowSchemeChirho --> EquationValidityChirho[Reject polymorphic equations and family patterns]
  EquationValidityChirho --> TransparentAliasesChirho[Expand synonyms before interpreting equations]
  TransparentAliasesChirho --> FamilyRowsChirho[Register fully represented local closed rows]
  FamilyRowsChirho --> ForwardChirho[Ordered shared reduction with output budget]
  FamilyRowsChirho --> ValidateInverseChirho[Validate determining variables and compatible RHS pairs]
  ValidateInverseChirho --> InverseChirho[Only proved dependencies propose argument equalities]
  ForwardChirho --> KindEqualityChirho[Ordinary occurs and rigid checking]
  InverseChirho --> KindEqualityChirho
  FreshOccurrenceChirho[Fresh polymorphic occurrence arguments] --> ForwardMatchChirho[Choose arguments making whole terms identical]
  ForwardMatchChirho --> KindEqualityChirho
```

Only the represented first-order fragment can certify inverse improvement.
Opaque terms, unsupported multi-row family results and exhausted verification
budgets do not certify it. Invalid determining-variable coverage and conflicting equations are
diagnosed before any use. Transparent aliases must be expanded: `Erase a = Bool`
cannot make `Family a = Erase a` injective. GHC9.14.1 independently rejects that
control and accepts its `Maybe a` counterpart. It also rejects forall types on
either side of a family equation (GHC-91510); those are validity errors, not
requests for capture-avoiding polymorphic family reduction.

A single covering equation may compose independently validated dependencies.
Starting at its result, the proof walks nominal constructor arguments and only
the validated injective positions of each exactly saturated family application.
Every declared determining input must be recovered. Missing metadata never
supplies a dependency, and the proof does not recursively validate another
declaration or assume its own annotation in a cycle. The local work limit yields
unproved, never permission to improve an argument. This validates the composition
contract; it does not add general nested-family inverse solving.

```mermaid
flowchart TD
  CoveringCompositionChirho[One equation with distinct variable patterns] --> ResultWalkChirho[Inspect result under local work budget]
  ResultWalkChirho --> NominalPathChirho[Follow nominal constructor arguments]
  ResultWalkChirho --> FamilyPathChirho[Require registered arity-matched injective positions]
  NominalPathChirho --> DeterminedInputsChirho[Collect only recoverable input variables]
  FamilyPathChirho --> DeterminedInputsChirho
  DeterminedInputsChirho --> CoverageProofChirho[Require every declared determining input]
  CoverageProofChirho --> PublishedProofChirho[Publish validated dependency]
  ResultWalkChirho --> UnprovedCompositionChirho[Unknown metadata or exhausted budget gives no proof]
```

This is intentionally stronger than GHC9.14.1's blanket ban on family-headed
injective results (upstream issue13248). The same composition beneath a nominal
constructor is accepted by GHC; an erasing inner family is rejected. Both source
forms and counterexamples are measured separately, not labelled all GHC-agreeing.
Tuple and unit terms now use the same nominal constructor representation in
signature interpretation and equation validation. Previously a unit argument
made that entire equation unrepresented, skipping its injectivity check; the
constructor-wrapped negative control exposed this before the checkpoint.

Matching fresh occurrence choices is separate from inverse improvement. It may
choose fresh instantiated variables to make two whole terms identical, but it
cannot derive equality of written or rigid arguments by cancelling a family
head. Leading explicit invisible forall binders remain quantified even when
other head classifiers await SCC inference. They are instantiated at constructor
uses, not specialized by those uses. Remaining classifier holes close only at
the normal publication boundary.

Kind-family reduction accounts for substituted output nodes before copying a
duplicating RHS. Exhaustion produces a specific work-limit diagnostic, not an
acceptance or a claimed kind mismatch. This local budget does not establish a
global bound on type inference, synonym expansion, or all normalization paths.
The generic type adapter gives numeric and named variables disjoint enum keys;
the string `tv0` cannot collide with numeric variable0.

An equation's lexical scope hides declaration-header variable identities before
its patterns are classified. The journal restores those bindings afterward.
Every underscore gets its own classifier and stored pattern variable; a single
converter owns all patterns and the result of one equation. Invisible family
head binders remain in lexical scope but do not add ordinary argument arrows.
This producer repair does not claim that explicit applications or hidden indices
are fully consumed. If reduction erases a dependent binder's final occurrence,
normalization removes that binder and shifts the surrounding indices; a still
referenced binder remains dependent.

Unqualified `*` under StarIsType is syntax for Type, not a named multiplication
operator. Qualified multiplication and NoStarIsType use normal name lookup.
The ordered source extension settings govern this distinction, and local TYPE
declarations remain nominal rather than acquiring the built-in representation
semantics. Source controls exercise both distinctions independently.

Still unfinished: hidden family indices, complete explicit family applications, open/associated
kind-family equation checking, higher-rank family result contracts, and complete
injectivity validation beyond this first-order fragment. No row with missing
hidden matching conditions is registered as a visible-only approximation. The retained
metadata is not a claim that those missing consumers now work. Design reference:
[GHC development guide, type families](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/type_families.html#injective-type-families);
executable controls use installed GHC9.14.1, not that guide's development version.

GHC-55233 is checked on both written contracts with one diagnostic, independently
of the existing Type/Constraint unification compatibility. Binder annotations
are not result annotations; local Constraint shadowing retains its previous
boundary. No broadening of that compatibility rule is part of this repair.

### Visible kind applications and local nominal indices (isolated row484)

TypeChirho and AstKindChirho retain an @ application separately from ordinary
application, with both operands and the complete source span. Naming and kind
dependency visitors traverse the supplied type; an unknown kind cannot disappear
because it follows @. The forall CST recognizer recognizes a binder prefix rather
than whitelisting the tokens in its annotation; the normal type grammar owns the
annotation and its boundary. Unsupported annotation forms remain separate gaps.

```mermaid
flowchart LR
  AtSourceChirho[Source @ or implicit head occurrence] --> AtAstChirho[Retain mixed application spine and binder specificity]
  AtAstChirho --> AtScopeChirho[Resolve both operands and open one ordered kind scheme]
  AtScopeChirho --> AtConsumeChirho[Skip inferred binders for @ and check the specified classifier]
  AtConsumeChirho --> AtSolvedChirho[Record solved invisible arguments by source span]
  AtSolvedChirho --> AtInputsChirho[Pass kind elaboration through named inference inputs]
  AtInputsChirho --> AtNominalChirho[Materialize local nominal indices in signatures and constructors]
  AtNominalChirho --> AtEqualityChirho[Retain indices during substitution and type equality]
```

Source schemes order binder dependencies before uses and retain explicit phantom
binders. An inferred classifier is not a specified argument; supplying @_ gives
an inference hole, not Type. Provisional family schemes bind only selected source
variables: classifier holes must remain shared until equation checking, rather
than being independently instantiated by every row.

KindElaborationChirho is a solved phase result, not namespace-preservation
metadata. The driver passes it through InferInputsChirho instead of reconstructing
arguments from names. This initial type consumer covers local data/newtype heads,
matching their qualified identity, not unrelated imports with the same basename.
Type conversion shares kind identities within the caller's lexical variable map;
it does not leak type variables across signatures. Phantom indices remain present
even when no field mentions them. Two GHC-checked executable controls construct a
kind-indexed value and read it through implicit and explicit signatures on STG,
LLVM and Cranelift, each requiring the independently measured output 42 plus LF.

Not complete: recursive monomorphic occurrences, synonym/family equation indices,
imported constructor schemes and imported specificity, and some higher-rank kind
annotations still lack complete consumers. The current T12045a reduction reaches
an unindexed recursive FreeCat field versus an indexed result; no compatibility
gain is claimed for that file. Keeping KindApp in the type IR is necessary but
does not establish that every producer has supplied its inferred arguments.

## Evidence boundary

Lowering controls retain both annotations and their exact source-slice spans, and keep
the following signature. Naming controls distinguish the two scopes and prove
both contracts are visited. Kind controls exercise composition, contradiction,
cache independence and the Constraint rule before constructor use can mask an
error. The driver uses one GHC-9.14.1-executed source for inline, standalone,
combined-data and combined-newtype contracts, with output 42/7/11/13 through
STG, LLVM and Cranelift. Execution and full-gate results are recorded in the
unit tasklist, not inferred from these source-level controls.

Still outside scope: arbitrary promoted/named/dependent kinds, imported
authoritative kind metadata, and the
separate data-family/type-data/refined-GADT-result AST decisions.
Symbolic standalone signatures, attachment to aliases/classes,
and duplicate/orphan signature diagnostics remain separate parser limitations.
The representation repair is not full TypeAbstractions checking: declaration-head
specificity matching, complete dependent constructor/selector quantifier metadata,
TH reification visibility, and authoritative cross-module kind schemes remain
unimplemented. The tasklist keeps independently reproduced counterexamples.
