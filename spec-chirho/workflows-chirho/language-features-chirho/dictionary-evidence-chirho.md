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
    lit_chirho --> capture_chirho[capture_literal_evidence_chirho: span, class, literal type; dummy spans skipped]
    solve_chirho[Module solved: unification + Report defaulting composed]
    capture_chirho --> finalize_chirho[finalize_literal_evidence_chirho: type resolved to a concrete head → key; variables and rigid variables yield nothing; a span seen twice must agree]
    solve_chirho --> finalize_chirho
    finalize_chirho --> result_chirho[InferResultChirho.literal_evidence_chirho: span → LiteralEvidenceChirho]
    desugar_chirho[Desugar: literal becomes App fromInteger-occurrence lit]
    desugar_chirho --> spans_chirho[record_literal_occurrence_chirho: occurrence id → literal span]
    result_chirho --> join_chirho[join_occurrence_evidence_chirho: span-first, then the per-name ordinal join]
    spans_chirho --> join_chirho
    join_chirho --> pass_chirho[Dictionary pass occurrence_head_replacement_chirho: prim row or keyed selector from the record]
    pass_chirho --> stg_chirho[STG: the literal's fromInteger has its dictionary]
```

## Invariants

- A record names one source token: keys are spans, never per-name ordinals, so inference
  order does not have to match desugar order (a generated node has the dummy span and no
  record).
- Only a concrete head is evidence. A literal whose type is still a unification variable or a
  rigid variable produces no record; the enclosing binding's dictionary parameter (or today's
  fallback) serves it.
- A span the checker inferred more than once (a re-check, a speculative branch) yields a record
  only when every inference agreed.
- The join maps the checker's `Integer` default onto the engine's `Int` rows at the driver
  boundary and nowhere else.
- No heuristic in the pass is widened by this workflow; records are consulted first, and the
  heuristics remain the fallback until a gate retires them.

## Reference evidence (bricks 3 and 4)

```mermaid
flowchart TD
    ref_chirho[Constrained reference inferred: scheme instantiated, one wanted per predicate]
    ref_chirho --> capture_ref_chirho[capture_reference_evidence_chirho: span, predicates in scheme order]
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

## Current boundary

- Reference evidence covers names in the module environment (top-level and imported).
  A local `let`/`where` binding generalized over a class is not in that environment, gets no
  dictionary parameter from the pass, and its methods carry no evidence (`T039_fib_acc`).
- Infix references (`x \`f\` y`) and operator sections still resolve through the shared
  canonical id; only prefix references mint occurrence ids.
- Class methods never mint reference occurrences (the method paths key on canonical ids);
  own-parameter evidence therefore reaches the standard method list only, and a user class
  method at a rigid variable is still dispatched by the pass's inferred parameter keys.
