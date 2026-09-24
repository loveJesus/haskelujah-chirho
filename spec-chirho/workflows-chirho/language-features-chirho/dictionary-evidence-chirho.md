<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Dictionary evidence workflow

The dictionary pass used to decide instances by looking at Core syntax (sibling arguments,
literal markers, constructor names, binder types) and by treating a binding's ground scheme
predicates as proof. The checker already solves every constraint; this workflow hands the
pass what the checker proved, one record per source token, and the pass consults a record
before any heuristic. Programs without a record keep today's path and today's verdict.

```mermaid
flowchart TD
    lit_chirho[Literal inferred: infer_expr_chirho pushes Num / Fractional / IsString wanted]
    lit_chirho --> capture_chirho[capture_literal_evidence_chirho: the literal's origin and span, class, literal type; a literal with neither identity is skipped]
    solve_chirho[Module solved: unification + Report defaulting composed]
    capture_chirho --> finalize_chirho[finalize_literal_evidence_chirho: type resolved to a concrete head → key; variables and rigid variables yield nothing; a key seen twice must agree]
    solve_chirho --> finalize_chirho
    finalize_chirho --> result_chirho[literal_evidence_by_origin_chirho: origin → LiteralEvidenceChirho, plus literal_evidence_chirho as the origin-free legacy span projection]
    desugar_chirho[Desugar: literal becomes App fromInteger-occurrence lit]
    desugar_chirho --> prov_chirho[record_occurrence_provenance_chirho: occurrence id → the literal's origin and role]
    desugar_chirho --> spans_chirho[record_literal_occurrence_chirho: occurrence id → literal span, for literals with no origin]
    result_chirho --> join_chirho[join_occurrence_evidence_chirho: by origin first, then span, then the per-name ordinal join]
    prov_chirho --> join_chirho
    spans_chirho --> join_chirho
    join_chirho --> pass_chirho[Dictionary pass occurrence_head_replacement_chirho: prim row or keyed selector from the record]
    pass_chirho --> stg_chirho[STG: the literal's fromInteger has its dictionary]
```

## Invariants

- A record names one source occurrence, so inference order does not have to match desugar order.
  The key is the occurrence's ORIGIN where the producer minted one, and its span only where it did
  not; a generated node used to mean "dummy span, no record", and now means "its own origin, its
  own record". **This invariant was stated here before it was true of method occurrences**: they
  were joined by a per-name ordinal until 2026-09-19. See "Method occurrences" below; the
  lesson is that an invariant in this document is not evidence about the code, and the join it
  describes has to be read.
- Only a concrete head is evidence. A literal whose type is still a unification variable or a
  rigid variable produces no record; the enclosing binding's dictionary parameter (or today's
  fallback) serves it.
- A key the checker inferred more than once (a re-check, a speculative branch) yields a record
  only when every inference agreed. An occurrence with an identity whose record is missing or
  conflicting receives nothing, and is never rescued by its span or its position.
- The join maps the checker's `Integer` default onto the engine's `Int` rows at the driver
  boundary and nowhere else.
- No heuristic in the pass is widened by this workflow; records are consulted first, and the
  heuristics remain the fallback until a gate retires them.

## Reference evidence (bricks 3 and 4)

```mermaid
flowchart TD
    ref_chirho[Constrained reference inferred: scheme instantiated, one wanted per predicate]
    ref_chirho --> capture_ref_chirho[capture_reference_evidence_chirho: span, predicates in scheme order]
    recursive_ref_chirho[Unsigned recursive reference: span and matching monomorphic assumption] --> pending_ref_chirho[RecursiveReferencesChirho retains source identity through substitutions]
    pending_ref_chirho --> finish_ref_chirho[finish_recursive_references_chirho: SCC generalization supplies predicates for those same type variables]
    finish_ref_chirho --> finalize_ref_chirho
    capture_ref_chirho --> finalize_ref_chirho[finalize_reference_evidence_chirho: concrete head → key; rigid or generalized variable → OWN_DICTIONARY_KEY_CHIRHO; open → nothing]
    method_chirho[Method occurrence at a rigid variable] --> own_chirho[finalize_occurrence_records_chirho: OWN_DICTIONARY_KEY_CHIRHO instead of the Int default]
    desugar_ref_chirho[Desugar: reference to a name whose scheme carries predicates → reference_occurrence_chirho mints an occurrence id, canonical = the binding]
    finalize_ref_chirho --> join_ref_chirho[Driver: reference_evidence by occurrence id, span-first]
    desugar_ref_chirho --> join_ref_chirho
    join_ref_chirho --> callsite_chirho[Pass: dictionary-parameter call site and bare reference — evidence_dict_for_class_chirho per predicate: instance dictionary for a key, the binding's own parameter for OWN, today's guess for nothing]
    own_chirho --> selector_chirho[Pass: try_rewrite_method_var_chirho — $sel_C_m applied to the binding's own dictionary parameter]
```

- An own-parameter record names the signature predicate it stands for (`$own:k`, recorded when
  the signature is skolemized: `record_own_dictionary_indices_chirho`); the pass keys every
  dictionary binder by predicate index as well as by class, so two predicates of one class are
  two parameters. The class-only form (`$own`) remains for superclass extractions and for
  generalized variables, where the pass's class-keyed parameter is the right one.
- A reference occurrence is seen through by every dictionary-parameter lookup
  (`canonical_id_chirho`), and the spine pre-pass leaves an evidenced reference in place so the
  call-site path can read its records; one it finds no evidence for is restored to its
  canonical id before STG, exactly as method occurrences are.
- The higher-order-argument rewriter hands an evidenced reference to the variable arm; a key
  guessed from sibling arguments never dispatches a reference the checker proved.
- An unsigned recursive call initially has no scheme predicates to capture. Its name,
  source span and matching monomorphic assumption are retained until the existing SCC
  generalization supplies the predicates. This delays evidence capture, not inference or
  generalization. The every-capture-at-one-span agreement rule is unchanged; a shadowed
  name with a different type assumption is not treated as that recursive binding.
- Nested calls with reference evidence take the normal evidence-first rewrite path. The
  surrounding result type is not substituted for the callee's predicate type (`Eq a`
  must not become `Eq [a]` merely because a recursive deduplicator returns a list).
- Definition-side and call-side dictionary parameters share
  `resolved_pred_dictionary_chirho`: omit a parameter only if the same ground or
  permitted-default dictionary actually exists. A result-only Applicative/Monad
  variable is not a numeric default, and cannot vanish from the caller's argument list.
- Polymorphic `pure`/`return` with a scheme's own dictionary must use its method selector
  before any enclosing-monad heuristic. Their result type may be IO, Maybe or list;
  retaining their bare names would incorrectly force every call onto IO.
- Execution tests call the same unsigned recursive renderer at Int, Bool and Double,
  with a String separator, and check recursive equality on list elements. These read
  the evidence back through behavior, rather than asserting a particular internal key.

## Local bindings (brick 8)

A `let`/`where` binding generalized over a class is not in the module environment and gets no
dictionary parameter from the pass. Its evidence is the type the enclosing code instantiates it
at: `generalize_local_chirho`'s quantified variables are marked local, every instantiation of
the binding records its fresh variables (`instantiate_scheme_parts_chirho`), and
`local_specialization_key_chirho` yields a key when all of them resolve to one concrete head,
transitively when an instantiation resolves to another local binding's variable (`isPrime n =
checkDiv n 2`). Methods, literals and constrained references at that variable carry the key;
two disagreeing instantiations prove nothing and keep today's path.

## Method occurrences (repaired 2026-09-19)

The desugarer mints one occurrence id per free reference of an opted-in class method, in
DECLARATION order. The checker counts references of that name in INFERENCE-VISIT order, and
instance method bodies are inferred last (phase 3e). `join_occurrence_evidence_chirho` matched
those two sequences BY POSITION under a count guard, so a module with two `show` references
exchanged their evidence: the same program printed `W3` or `WChirho 3` depending only on which
declaration came first. The occurrence was rewritten to `$prim_Show_show_Int` before any
dictionary path ran, so the user's instance was found under the wrong key, and the same defect
crashed ("no matching alternative for tag 66") when a nullary and a one-field instance shared a
module.

```mermaid
flowchart TD
    mint_chirho[resolve_var_chirho mints an occurrence id for an opted-in method] --> span_chirho[reference_occurrence_chirho records its SOURCE SPAN in method_occurrence_spans_chirho]
    infer_occ_chirho[Inference captures the predicate at that reference] --> record_chirho[MethodOccurrenceRecordChirho carries the same span]
    span_chirho --> join_chirho[join_occurrence_evidence_chirho: match record span to occurrence span]
    record_chirho --> join_chirho
    join_chirho -->|unique span, same name| consume_chirho[Record CONSUMED, occurrence IDENTIFIED]
    join_chirho -->|no span at the occurrence| leftover_chirho[Unidentified occurrences vs UNCONSUMED records, count-guarded, by position]
    consume_chirho --> evidence_chirho[occurrence_evidence_chirho]
    leftover_chirho --> evidence_chirho
```

- A proof serves ONE reference: a record the span join consumed can never be reused by the
  positional path, and vacancy in the consumer map is not ownership of a proof.
- An ambiguous or duplicated span identifies nothing and does not fall through to a positional
  assignment; its occurrence simply carries no evidence.
- The positional path exists only because the desugarer mints occurrences at several sites and
  only the variable-reference site records a span. Deleting it turns 19 driver tests red (all
  desugared shapes); restricting it to names whose occurrences all lack spans regresses a
  derived `Show` of a Bool field in a native round trip, where main is correct. It goes away
  when every mint site carries its own occurrence provenance, which a span alone cannot supply:
  one do or deriving node can create several references sharing a span, so the identity must be
  shared with the node that created them.

## Occurrence provenance (in progress, 2026-09-22)

The identity a span cannot supply is minted by the PRODUCER of each occurrence and carried on the
node. `OriginSupplyChirho` is a field of `ModuleChirho`; lowering creates it, and every later
producer borrows it through the module, so one sequence runs across all of them. A module is not
`Clone`, so the supply cannot fork.

```mermaid
flowchart TD
    lower_chirho[Lowering: stamps each reference and operator AT CONSTRUCTION] --> supply_chirho[(module origin supply)]
    dup_chirho[Lowering duplicates a guard fall-through] -->|remint the copy| supply_chirho
    splice_chirho[Splice expansion: each CONVERTED declaration] -->|remint before push; passthrough keeps its origins| supply_chirho
    derive_chirho[Early deriving: the fresh instances] -->|remint before extend| supply_chirho
    gnd_chirho[Late GND, gpt_chirho's seam: its generated declarations] -->|remint before extend| supply_chirho
    supply_chirho --> ast_chirho[Every occurrence in the AST carries a distinct origin]
    ast_chirho --> check_chirho[Checker: each record and each reference-evidence entry carries its occurrence's origin]
    ast_chirho --> desugar_chirho[Desugarer: the variable, operator and section mint sites record origin and role]
    check_chirho --> join_chirho{Join}
    desugar_chirho --> join_chirho
    join_chirho -->|occurrence has provenance| own_chirho[Only the proof of its own origin; missing or conflicting gives none]
    join_chirho -->|no provenance, genuine span| span_chirho[By span, records WITHOUT origin only]
    join_chirho -->|no identity at all| position_chirho[By position, count-guarded, records WITHOUT origin only]
    check_chirho -->|capture WITH an origin| owned_chirho[Published under its origin only]
    check_chirho -->|capture without an origin| legacy_chirho[Published under its span: the legacy projection]
```

- `occurrences_chirho.rs` in the AST crate is the one statement of where occurrences live. Every
  match in it is exhaustive with no fallback arm, so a new expression, pattern, statement or
  declaration form does not compile until it is classified.
- Fresh generated output is reminted in full. Moving or keeping an occurrence preserves its
  origin. A subtree duplicated as new code is reminted at that duplication. A binder re-read from
  an expression carries no origin.
- Equality and hashing of a name ignore the origin, so no lookup changes meaning.
- Occurrence carriers today: variable and constructor references, the operators of infix
  applications and sections, and literals — in expressions AND in patterns, since lowering stamps
  every literal at construction. List literals, arithmetic sequences and do statements get their
  own carriers next; until then their uses have no provenance and are served by span and position
  exactly as before.
- A literal in a pattern carries an origin but publishes no evidence yet: the checker captures
  nothing when it binds a pattern. That is a producer gap, not a transport gap, and it belongs with
  do-statement checking.
- Keying by origin gives GENERATED references evidence they never had: the checker used to skip
  a placeholder span at capture. That is a behaviour change, measured, not a neutral re-keying.

## Current boundary

- An infix or section use of a class METHOD mints its own occurrence id and carries its
  operator's provenance, like a prefix use. An infix or section use of a constrained
  function that is not a method still resolves through the shared canonical id: only a prefix
  reference mints a reference occurrence for per-predicate reference evidence.
- Class methods never mint reference occurrences (the method paths key on canonical ids);
  own-parameter evidence therefore reaches the standard method list only, and a user class
  method at a rigid variable is still dispatched by the pass's inferred parameter keys.
