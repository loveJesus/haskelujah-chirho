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
    join_chirho --> pass_chirho[Dictionary pass at every occurrence entry, occurrence_head_replacement_chirho and try_rewrite_method_var_chirho: prim row, an Eq/Ord proven list key's dispatch, or keyed selector from the record]
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
- A literal's `fromInteger` application is keyed by its own record (row 543).
  - `infer_type_key_chirho` gives that application the occurrence's Num record, and `Int` only when there is none.
  - It used to answer `Int` by name. So a method keyed from its first argument, as `/` is, missed the Fractional row whenever that argument was a literal: `1 / 2 :: Double`, a Double signature, and `7 / 2` under defaulting.
  - The interpreter then failed with `missing STG binding /`, and the native backends with `missing native binding /`.
  - Controls: `tests/dictionary_evidence_chirho/fractional_literals_chirho.rs`.
- The desugarer's blind Int primops take the float primop at a Double operand (row 547).
  - Infix and section `<`, `<=`, `>` and `>=` become `<#`, `<=#`, `>#` and `>=#` before any type is known. So does a prefix minus on an operand that is not a float literal, which becomes `negate#`.
  - `rewrite_method_refs_with_locals_chirho` gives such a primop its float counterpart when an operand keys as Double: binder types, type applications and literal records. The rule is `rewrite_chirho/double_primops_chirho.rs`.
  - The counterparts are `<.#`, `<=.#`, `>.#`, `>=.#` (ordered, so a NaN gives False) and `negateFloat#`.
  - `negate#` keys as its operand in `type_keys_chirho.rs`, so `(-1) / 2 :: Double` keys `/` at Double instead of Int.
  - `rewrite_expr_with_type_key_chirho` (`method_args_chirho.rs`) rebuilds primops while rewriting arguments at a key. When that key is exactly Double, it takes the same counterpart (row 553). Elements under `map (+ 0) ([-3 / 2, 1] :: [Double])` used to keep `negate#`, which was silently `-0.75` natively.
  - Before, the interpreter refused every such comparison (`LtIntChirho: expected Int#`), and the native backends compared IEEE bits as integers, which orders negative Doubles backwards.
  - Ordering of Strings, tuples and derived types is outside this rule: the seeded `Ord` class has only `compare`.
  - Controls: `tests/dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs`.
- Other keys reach their ordering through a body-backed `compare` (row 551, the first Ord brick).
  - When an operand of the desugarer's `<#`, `<=#`, `>#` or `>=#` keys as anything but Int, Char, Bool or Double, `rewrite_chirho/compare_route_chirho.rs` looks up `$prim_Ord_compare_<key>` with `lookup_dispatch_body_name_id_chirho`. That lookup accepts a body-backed row and never a missing-method stub.
  - If found, the primop becomes `test (compare a b)`, with one of the Ordering tests generated in `prelude_chirho/ordering_tests_chirho.rs`.
  - That serves Strings, lists, derived types and Ordering.
  - A key without such a row keeps the primop: `Maybe`, tuples. `max`, `min` and prefix `(<)` are later bricks.
  - Controls: `tests/dictionary_evidence_chirho/ordering_chirho/compare_route_chirho.rs`.

## Reference evidence (bricks 3 and 4)

```mermaid
flowchart TD
    ref_chirho[Constrained reference inferred: scheme instantiated, one wanted per predicate]
    ref_chirho --> capture_ref_chirho[capture_reference_evidence_chirho: span, predicates in scheme order]
    recursive_ref_chirho[Unsigned recursive reference: span and matching monomorphic assumption] --> pending_ref_chirho[RecursiveReferencesChirho retains source identity through substitutions]
    pending_ref_chirho --> finish_ref_chirho[finish_recursive_references_chirho: SCC generalization supplies predicates for those same type variables]
    finish_ref_chirho --> finalize_ref_chirho
    capture_ref_chirho --> finalize_ref_chirho[finalize_reference_evidence_chirho: a ground list → its whole key, ground_list_key_chirho; any other concrete type → its head; rigid or generalized variable → OWN_DICTIONARY_KEY_CHIRHO; open → nothing]
    method_chirho[Method occurrence at a rigid variable] --> own_chirho[finalize_occurrence_records_chirho: OWN_DICTIONARY_KEY_CHIRHO instead of the Int default]
    desugar_ref_chirho[Desugar: reference to a name whose scheme carries predicates → reference_occurrence_chirho mints an occurrence id, canonical = the binding]
    finalize_ref_chirho --> join_ref_chirho[Driver: reference_evidence by occurrence id, span-first]
    desugar_ref_chirho --> join_ref_chirho
    join_ref_chirho --> callsite_chirho[Pass: dictionary-parameter call site and bare reference — apply_reference_dicts_chirho per predicate: instance dictionary for a key, the binding's own parameter for OWN, today's guess for nothing; a proven whole list key never yields to the guess; no dictionary → missing_dictionary_chirho, an explicit lazy error]
    own_chirho --> selector_chirho[Pass: try_rewrite_method_var_chirho — $sel_C_m applied to the binding's own dictionary parameter]
```

- An own-parameter record names the signature predicate it stands for (`$own:k`, recorded when
  the signature is skolemized: `record_own_dictionary_indices_chirho`); the pass keys every
  dictionary binder by predicate index as well as by class, so two predicates of one class are
  two parameters. The class-only form (`$own`) remains for superclass extractions and for
  generalized variables, where the pass's class-keyed parameter is the right one.
- Ground list keys and complete arguments (2026-10-17, gpt_chirho #27285). List dictionaries
  and method rows are named per element (`$fOrd[Int]`, `$prim_Eq_==_[Int]`, `$fOrd[Char]` for
  `String`); the head `[]` names none of them. So a list whose every part is ground keeps its
  whole key, `ground_list_key_chirho` (`infer_chirho/evidence_chirho.rs`), in method
  occurrence records for every class (`Show` already did) and, through
  `ground_or_head_key_chirho`, in reference keys, local `let`/`where` specializations and
  literal evidence alike. An instance head registers as it DISPLAYS (`instance C [[Char]]` and
  one written `[String]` are both `$fC[[Char]]`), while the evidence spells a nested string
  `String`, the spelling `Show`'s own rows use; so every lookup tries the key, then the key
  with each whole-word `String` spelled `[Char]` (`registry_spellings_chirho`, token-aware:
  `ByteString` and `Data.String` are untouched), then for dictionary lookups the head `[]`
  of a whole list key and the head of a higher-kinded class key, as before. A ground
  dictionary's METHOD SLOT crosses the same gap: the desugarer names an instance's method rows
  by the head as written (`$prim_C_m_[String]`, `$prim_C_m_String`), the dictionary by the head
  as typed (`[[Char]]`, `[Char]`), so a head written with `String` got a missing-method stub
  in its slot. `written_method_row_chirho` (`dict_chirho/instance_chirho/string_spellings_chirho.rs`,
  the rule's one home) fills the slot with the ONE source row whose written key, spelled
  out, is the typed key (agy_chirho #27325). The slot tries the exact typed row first, then a
  generic head's confirmed written row (Generic instance heads, below), then this row; two
  such rows are ambiguous and refuse, never a lowest-id pick (gpt_chirho #27399). Still a
  stub: another synonym, an application head (written `Maybe Int`, typed `(Maybe Int)`) and
  a later argument of a multi-parameter head. A head with type variables (written `T b`,
  typed `(T t0)`) is the generic-head rule's.
  A whole key the checker proved is never replaced by the
  argument-shape guess, which reads the element off the syntax and collapses `[[Int]]` to
  `[Int]`. And a call receives a dictionary for EVERY predicate: where none can be built,
  `missing_dictionary_chirho` (`dict_chirho/rewrite_chirho/constrained_references_chirho.rs`)
  passes an explicit error naming the obligation (`no instance dictionary for `Ord [[Int]]`,
  required by `sameChirho``). As any non-atomic argument it is lowered to a thunk
  (`stg_lower_chirho.rs`), so it fails only if forced. Before, the argument was left out and
  the next term argument took its place: programs printed nothing more and exited 0. The
  evidence-LESS typed-argument site (an infix use mints no reference occurrence) obeys the
  same rule through `typed_reference_dicts_chirho`: every slot or none (none → the general
  path decides), never a partial spine; `3.0 `halfChirho` 2.0` with `(Show a, Fractional a)`
  under an `Int` key printed `$PAP halfChirho …` for GHC's `"1.5"` (gpt_chirho #27303). Its
  key comes from the surrounding call, so a dictionary it serves can be at the wrong type;
  reference evidence for infix uses is the separate cure.
- Ground keys, measured boundaries (ground gate on 7ad703e8, gpt_chirho #27462). Unmet and
  left open:
  - The equality SECTION `(== [1])` lowers to the Int primop `==#` with no dictionary at all,
    so `map (== [1]) [[1],[2]]` prints [True,True] where GHC prints [True,False]. It does so on
    4cfb9584 too. The lambda form `\x -> x == [1]` runs through the whole key.
  - `[[1]] == [[2 :: Int]]` with annotated literals prints True (GHC False), on 4cfb9584 too.
    The signed form stays loud, which is fail-closed. Settled by R2 (Proven list keys at
    method occurrences, below): it now fails by name.
  - Both are pre-existing, but a program that used to abort earlier now reaches them, so the
    unit stays held until their leads are settled.
  - A user-class method used DIRECTLY (`tagChirho x`) mints no occurrence evidence and never
    reaches a dictionary. It stays loud; only a constrained call or a standard method's
    occurrence reaches the keys.
  - The literal channel's whole key has no observable effect today. Literal evidence is
    recorded for Num, Fractional and IsString only, so only an OverloadedStrings literal at
    `[Char]` is list-typed, and that literal runs alike under the whole key and under 4cfb9584's
    head key. The rule is kept for one shared key function, and no behavior is claimed for it.
  - The proven-key guard trades an accidental right answer for a loud failure:
    `sameChirho [[[1]], [[1 :: Int]]]` printed True on 4cfb9584 (correct by accident) and now
    fails, naming `Ord [[Int]]`. Its unequal twin printed a wrong True there.
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

## Proven list keys at method occurrences (R2, 2026-10-17)

An `Eq` or `Ord` method occurrence whose evidence is a whole ground list key (`[[Int]]`,
`[Integer]`) that names no method row went through the keyed fallback, and when that found no
dictionary it came back from `occurrence_head_replacement_chirho` as the canonical id. The
rebuilt call recursed with no evidence, the method-application path keyed it off its first
argument, and the cons case of `infer_type_key_chirho` reads an unknown element as `[Int]`:
`[[1]] == [[2 :: Int]]` ran `$fEq[Int]`, whose `==#` compares the inner lists as empty string
prefixes, and printed `True`. Inside `sameChirho :: Eq a => ...`, the keyed fallback's
class-only fallback lent the enclosing `Eq a` dictionary instead, also `True`. Measured alike
on 4cfb9584 and 7ad703e8 (claude_chirho #27479, #27487; gpt_chirho #27481, #27488).

```mermaid
flowchart TD
    occ_r2_chirho[Eq or Ord occurrence: evidence a whole list key, no prim row] --> sel_r2_chirho[proven_list_selector_chirho: the canonical name selects through the evidence class]
    sel_r2_chirho --> reg_r2_chirho{registry_dict_chirho at the proven key}
    reg_r2_chirho -->|found| app_r2_chirho[selector applied to that dictionary]
    reg_r2_chirho -->|none| eng_r2_chirho{registry_dict_chirho at the proven key in engine_spelled_key_chirho, when it differs}
    eng_r2_chirho -->|found| app_r2_chirho
    eng_r2_chirho -->|none| syn_r2_chirho{argument-shape key agrees: key_agrees_with_proof_chirho over engine_spelled_key_chirho}
    syn_r2_chirho -->|agrees, registered| app_r2_chirho
    syn_r2_chirho -->|disagrees or unregistered| err_r2_chirho[selector applied to missing_dictionary_chirho: lazy, names the class and key]
```

- Agreement compares engine spellings: whole-word `String` as `[Char]`, `Integer` as `Int`.
  The defaulted `[1] == [2]` is proven `[Integer]` and is still served by `$fEq[Int]`; `[Int]`
  never stands in for `[[Integer]]`. Since R2b the proof's own engine spelling is tried right
  after the exact key, so `$fEq[Int]` serves `[Integer]` before any argument key is read; an
  agreeing argument key can only name the same dictionary, so agreement now guards against
  disagreeing keys and serves nothing the proof does not.
- After a proven miss, never the canonical id and never the class-only fallback. The error is
  a thunk, so a comparison that is never forced (an unused `let`, an empty `map`, the right
  side of a short-circuited `&&`) never fails.
- Measured cost: `[Just 1] == [Just (2 :: Int)]` printed GHC's `False` only because the
  runtime's legacy operand unboxing (`force_to_prim_chirho`) reached the field of the one-field
  `Just` under `$fEq[Int]`'s `==#`; it now fails, naming `Eq [(Maybe Int)]`, the key's spelling.
  `[Just 1] == [Nothing]` was loud already.
- Scope: `Eq` and `Ord` (`PROVEN_LIST_CLASSES_CHIRHO`). `Show` keeps its documented
  dictionary path for shapes without a generated row (`show [Just RedChirho]` prints GHC's
  answer through it); every other class keeps its path, unmeasured.
- Not on this route: `/=` desugars to `not (==)` with no provenance, so it carries no evidence
  and `[[1]] /= [[2 :: Int]]` still prints `False` (GHC `True`), the Eq method set's repair.
  Sections other than an `==` section with an origin (R1, below), and infix `<`, `<=`, `>`,
  `>=`, lower to Int primops before any occurrence exists.

## Equality sections (R1, 2026-10-17)

`build_section_body_chirho` lowered an `==` section straight to the Int primop `==#`, so the
dictionary pass never saw it: `map (== [1]) [[1], [2 :: Int]]` compared the lists as empty
string prefixes and printed `[True,True]` (GHC `[True,False]`), and a newtype with its own `Eq`
was compared as its raw `Int` (`[False,False]` for GHC's `[True,False]`). An `==` section whose
operator carries an origin now takes the builder's existing variable path: the opted-in
occurrence of `==`, the operator's origin recorded, and the application built as an infix `==`
builds it. The checker already types a section's operator through its variable path, so the
occurrence is dispatched by the checker's evidence: a method row (`$prim_Eq_==_[Int]`, a user
instance's row), a dictionary, or R2's named failure for a proven nested list (gpt_chirho
#27506, #27508).

- A section without an origin keeps `==#`. Every parsed section has one (the lowerer names
  its operator through `reference_name_chirho`), so the guard serves only producers without
  that metadata, and no parsed program can tell it apart.
- Only `==`. The engine's `Ord` dictionary holds `compare` alone: a variable `<` has no
  binding (`(<) 1 (2 :: Int)` fails with `missing STG binding <`), so the ordering operators
  keep their primops until an Ord method model gives them `compare`-backed defaults. `/=`
  waits for Eq's method set; `+`, `-`, `*` sections keep theirs, unmeasured.

## R2 at every occurrence entry (R2b, 2026-10-18)

R1's gate (r1r2-5827b4bf-chirho) went RED on its own nested section: `map (== [[1]]) [[[1]],
[[2 :: Int]]]` failed with `missing STG binding ==`. R2 lived only in
`occurrence_head_replacement_chirho`, which the argument-spine path calls. A call to a
non-method function with a lambda or method argument (`map`, `filter`, `any`, `zipWith`, a
user's `applyChirho`) goes through `try_rewrite_typed_higher_order_args_chirho` instead. It
guesses a key from the call's other arguments, and `rewrite_expr_with_type_key_chirho` hands
every variable, method occurrences included, to `try_rewrite_method_var_chirho`. That function
tried the occurrence's method rows, then replaced the guess with the proof, found no
dictionary, and returned the bare occurrence: `\x -> x == [[1]]` under `map` crashed the same
way on ground. Inside an `Eq a =>` binding it reached the class-only fallback, and the
enclosing dictionary answered: `pickChirho 'c' [[1]]` printed `True` where GHC prints `False`.

```mermaid
flowchart TD
    spine_r2b_chirho[argument-spine path: occurrence_head_replacement_chirho, exact method row first] --> shared_r2b_chirho[proven_list_occurrence_chirho]
    typed_r2b_chirho[typed entries: try_rewrite_method_var_chirho, spelled method rows first] --> shared_r2b_chirho
    shared_r2b_chirho -->|Eq or Ord at a whole list key| dispatch_r2b_chirho[proven_list_dispatch_chirho: the proven key, else the proven key in the engine's spelling, else an agreeing candidate, else the named lazy failure]
    shared_r2b_chirho -->|otherwise| rest_r2b_chirho[each entry's existing path]
```

- The spine path offers the call's own key, read lazily. A typed entry offers the key its
  caller read or guessed, before the proof replaces it. Either serves only when it agrees with
  the proof, and neither path reaches the class-only fallback for such an occurrence.
- Typed entries are the arguments of typed higher-order calls, dictionary-parameter calls,
  method applications and `fail`, and a method under a type application.
- A typed entry's guessed key rarely agrees: for `map (\x -> x == [1]) [[1], [2]]` it is the
  element of the cons case's collapsed `[Int]`, that is `Int`, against the proof `[Integer]`.
  The proof's own engine spelling (`[Int]`, then `$fEq[Int]`) serves such defaulted rows
  instead (gpt_chirho R2b disposition, option b), from the proof alone.
- Predicted, unmeasured until built: nested rows name `Eq [[Int]]` or `Ord [[Int]]` instead of
  crashing, the enclosing dictionary no longer answers, and the defaulted `==` rows print
  GHC's answer.
- Ord boundary, measured on the frozen R1/R2 CLI: the `Ord [Int]` list dictionary has no
  `compare` (`compare [1] [2]` fails with `missing method Ord.compare for [Int]`), and
  `compare` at `[Int]` crashes (`no matching alternative for tag 0`). A defaulted `compare` in
  a typed argument reaches that dictionary too; it waits for the Ord method model.

## A generated `Eq [String]` (A4, 2026-10-18)

The combined O1 + R2b gate (combined-d7081c8c-chirho) went RED on one matrix row:
`map (== ["a"]) [["a"], ["b"]]` was GHC-exact on ground and named `Eq [String]` after R1 and
R2b. No dictionary existed. `generate_conditional_ground_dicts_chirho` specialized
`Eq a => Eq [a]` only for `Int`, `Char`, `Bool` and `Double` elements. Ground's answer was
accidental: its section ran `==#` on the whole `[String]` values, which compares only their
first strings, and it was wrong on every measured variant whose first strings agree.

```mermaid
flowchart TD
    inst_a4_chirho["Eq a => Eq [a]"] --> gen_a4_chirho[generate_conditional_ground_dicts_chirho]
    gen_a4_chirho -->|Int, Char, Bool, Double elements| ground_a4_chirho["$fEq[Int], $fEq[Bool], $fEq[Double]"]
    gen_a4_chirho -->|Eq only: String elements| str_a4_chirho["$eqList_[Char] compares each String with ==#: $prim_Eq_==_[[Char]], $fEq[[Char]]"]
    str_a4_chirho --> r2_a4_chirho["R2 at a proven [String] key finds it under the spelling [[Char]]"]
```

- Each String element compares with the generator's `==#`. The runtime forces both operands
  and compares them whole as strings (`EqIntChirho` sits in the string-compare set beside
  `EqStrChirho`), measured exact on two strings. A mutant switching the comparator to
  `eqStr#` is observationally equivalent, so it was refuted and not added as a control.
- R2 is unchanged. Its exact-key lookup at `[String]` already walks the spelling `[[Char]]`,
  and the typed entries' spelled method rows find `$prim_Eq_==_[[Char]]`.
- Bounded: String elements only. A list of String lists still names `Eq [[String]]`, and the
  general nested specialization (`Eq [[Int]]` and beyond) remains R3's. `Ord [String]` is
  generated too since the Ord list unit below.

## A generated `Ord [T]` (2026-10-22)

gpt_chirho's disposition on #28041. Thirty-three probes on GHC 9.14.1 and two frozen CLIs found
`compare` at list types silently wrong or crashing, from two causes in the conditional
generator. The builtin `Ord a => Ord [a]` carries no method bodies, so every `Ord [T]` slot was a
missing-method stub. That stub, named for the ground key, also took one leading dictionary per
context class.

```mermaid
flowchart TD
    inst_ol_chirho["Ord a => Ord [a]"] --> gen_ol_chirho[generate_conditional_ground_dicts_chirho]
    gen_ol_chirho -->|Int, Bool, Double, String elements| ord_ol_chirho[generate_ord_list_dict_chirho]
    ord_ol_chirho -->|the element row is real, not a stub| row_ol_chirho["$prim_Ord_compare_[T] = $compareListBy $prim_Ord_compare_T"]
    row_ol_chirho --> dict_ol_chirho["$fOrd[T] = $Dict_Ord $fEq[T] row"]
    shared_ol_chirho["$compareListBy cmp xs ys: built once, no default alternative"] --> row_ol_chirho
    ord_ol_chirho -->|no real element row, or a method besides compare| stub_ol_chirho["generic path: $prim_Ord_compare_[T] fails by name"]
```

- One recursive `$compareListBy` serves every element type. It compares element by element
  with the given row, recurses on `EQ`, orders `[]` before `(:)`, and forces only what the
  answer needs (`compare (1 : undefined) [2]` is `LT`). With no default alternative, an element
  result that is not an `Ordering` fails instead of picking a branch.
- A row named for a key keeps its method's calling convention. The generic path's
  missing-method stub no longer takes leading dictionaries, so a caller that finds
  `$prim_<C>_<m>_[T]` by name gets the named failure, never a partial application that a
  default branch swallows. `map neededChirho [[1]]` used to print `[[1]]`.
- Measured before: `sortBy compare [[2],[1]]` printed `[[2],[1]]` and
  `maximumBy compare [[1],[3],[2]]` printed `[1]`. `compare [1] [2]` and its siblings crashed
  (`no matching alternative for tag 0`), and a constrained `compare` named the missing method.
- Bool and Ordering compare by constructor order through one generator,
  `generate_constructor_order_compare_chirho` (`instance_chirho/constructor_orders_chirho.rs`).
  Bool's row used to be `compare#`, an Int primop, applied to constructors. It returned the
  integer 0, so `compare False True` crashed and `sortBy compare [True, False]` kept
  `[True,False]`. Ordering's row is the table it always had.
- Bounded: element types only. `compare [[1]] [[2]]` still names `Ord [[Int]]` through R2.
  At non-scalar types, `<`, `<=`, `max`, `sort` and `maximum` stay loud: they are scalar
  primops until an Ord method model. `Ord (Maybe a)` has no dictionary. `compare LT GT`
  dispatched at Int until LT, EQ and GT were seeded (Builtin constructor seeds, below).

## Occurrence provenance (in progress, 2026-09-22)

The identity a span cannot supply is minted by the PRODUCER of each occurrence and carried on the
node. `OriginSupplyChirho` is a field of `ModuleChirho`; lowering creates it, and every later
producer borrows it through the module, so one sequence runs across all of them. A module is not
`Clone`, so the supply cannot fork.

```mermaid
flowchart TD
    lower_chirho[Lowering: stamps each reference and operator AT CONSTRUCTION] --> supply_chirho[(module origin supply)]
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
  every literal at construction. DO STATEMENTS now carry an origin too, on the operation each one
  SELECTS - its `>>=`, `>>` or `fail` - rather than on the statement; the checker records evidence
  for that operation under that origin. The join does not consume it yet, because it takes only
  the Reference role. List literals and arithmetic sequences get their own carriers next; until
  then their uses have no provenance and are served by span and position exactly as before.
- A literal in a pattern carries an origin but publishes no evidence yet: the checker captures
  nothing when it binds a pattern. That is a producer gap, not a transport gap, and it belongs with
  do-statement checking.
- Keying by origin gives GENERATED references evidence they never had: the checker used to skip
  a placeholder span at capture. That is a behaviour change, measured, not a neutral re-keying.

## Generic instance heads (2026-10-17)

```mermaid
flowchart TD
    inst_chirho[ground instance, no context] --> g1_chirho{G1, G2, G6: a class declared with one parameter and no superclass, one supplied head, no context}
    g1_chirho -- no --> typed_chirho[as before: typed key and name only]
    g1_chirho -- yes --> g3_chirho{G3: typed head T v1..vn, at least one argument, distinct variables}
    g3_chirho -- no --> typed_chirho
    g3_chirho -- yes --> g4_chirho{G4: the only instance this module sees with head T or a variable head}
    g4_chirho -- no --> typed_chirho
    g4_chirho -- yes --> g5_chirho{G5: every method has a row the slot takes: the exact typed row, or the row under ONE written spelling T x1..xn of distinct variables}
    g5_chirho -- no --> typed_chirho
    g5_chirho -- yes --> dict_chirho["dictionary keeps its typed name and key, $fC(T t0)"]
    dict_chirho --> slot_chirho["each slot: exact row for the typed key, then $prim_C_m_ + written spelling, then the Monoid fallback and the stub, as before"]
    dict_chirho --> alias_chirho["alias (C, T) in this pass's private registry, never over an existing key, never exported"]
    use_chirho[use site, evidence key T] --> selector_chirho[try_rewrite_method_var_chirho: no exact row, so the keyed selector finds the alias]
    alias_chirho --> selector_chirho
```

- Three spellings of one instance used to part (claude2_chirho #27335). The checker types the head
  `SBoolChirho b` as `(SBoolChirho t0)`. The desugarer names the method rows by the head as written
  (`$prim_Eq_==_SBoolChirho b`). A use site keeps only the outer head (`SBoolChirho`). The
  dictionary `$fEq(SBoolChirho t0)` was registered under a key no use site carries and held a
  missing-method stub, so the instance never ran: `==` fell to the raw method. In a class declared
  with one parameter and no superclass, a head that is a constructor applied to distinct
  variables, with no competing head for that constructor among the instances this module sees,
  is the only instance any `T x1 .. xn` can mean here (`generic_head_chirho`,
  `dict_chirho/instance_chirho/generic_heads_chirho.rs`). Both halves are needed: the alias lets
  the use site's existing keyed-selector fallback find the dictionary, and the written row fills
  its slot.
- The alias is private (gpt_chirho #27368). `instance_dicts_chirho` is a field of the pass's
  context, created empty by every `dict_pass_module_*` entry and run once per Core module; nothing
  outside `dict_chirho` reads it. The dictionary keeps its typed binder name, so no new name is
  discoverable downstream. A consumer module recomputes G1-G6 over its own class environment, where
  a downstream OVERLAPPING instance or a same-named type from another module competes (G4). The
  alias is entered with `or_insert`, and a later exact `insert` replaces it, so an exact
  registration always wins.
- Boundaries (gpt_chirho #27342, #27349, #27365, #27366). A row that needs a context never becomes
  an unconditional dictionary (G2, the eligibility filter as before). A specialized, contextual or
  catch-all head for the same constructor leaves the generic one unaliased (G4): a use site that
  kept only the head never guesses among them. A class with a superclass is refused (G6): its
  superclass slot is named by the instance's own key, and the class records superclass names
  without their arguments, so `class S [a] => C a` would reach `S`'s instance for `T` where GHC
  uses `S [T ..]`. The desugarer's key drops parentheses, so only an all-variable written argument
  list is accepted, as a position-wise bijection with the typed variables. A second spelling is
  ambiguous and refuses; a method without a row the slot lookup takes would make a stub
  reachable and refuses (coverage and lookup read the same spellings, agy_chirho #27369). Still
  unsupported: full-key use sites (a ground `T Int` key never falls to `T`), multi-parameter heads,
  lists and tuples, written synonyms, classes with superclasses (claude2_chirho #27367: typed
  superclass keys already dangle across instances, since variables are freshened per instance),
  and instances with a context, which have no dictionary at all yet (the conditional-instance
  unit). Cross-module overlap and same-name controls run only once a multi-module frozen build can
  exercise them. A user-class method used directly mints no occurrence evidence, so its use
  site never consults the alias; a constrained call or a standard method's occurrence does.
  G4 counts only instances that reach the class environment: a context-free instance whose
  head carries an overlap pragma is lost before typing today, and while it is, its generic
  rival must not be aliased (see the tasklist's blocker).

## Builtin constructor seeds (2026-10-23)

RC5, from gpt_chirho #28162 and #28187.

**The cause.** Typing seeded `True` and `False` but never `LT`, `EQ` or `GT`.
- A bare Ordering literal was therefore a fresh type variable.
- Its Eq and Ord occurrences defaulted to Int evidence (`default_occurrence_key_chirho`).
- The dictionary pass then served the Int row it was given.

**The fix.** `builtins_chirho/constructors_chirho.rs` now binds the three as mono `Ordering`
beside `True` and `False`.

```mermaid
flowchart TD
    seed_bs_chirho["constructors_chirho: LT, EQ, GT :: Ordering"] --> lookup_bs_chirho[an ExprChirho::ConChirho finds a scheme]
    lookup_bs_chirho --> occ_bs_chirho["compare LT GT: Ord resolved at Ordering before finalization"]
    occ_bs_chirho --> row_bs_chirho["$prim_Ord_compare_Ordering and $fEqOrdering rows"]
    module_bs_chirho["a module's own LT, EQ, GT"] -->|collect_module_shadowed_names_chirho removes the seed| own_bs_chirho[the module's own type]
```

- **Before:**
  - `compare LT GT` and `let x = LT in compare x GT` crashed with no matching alternative for tag 0
    at the Int row.
  - `sortBy compare [GT, LT, EQ]` printed `[GT,LT,EQ]`.
  - `LT == GT` and `GT /= EQ` crashed in Int primops.
  - `x :: Int; x = LT` was accepted and printed `LT`. `GT + (1 :: Int)` was accepted, then
    crashed in the Int addition primop.
- **Now:** the comparisons, the sort and the equalities answer as GHC 9.14.1 does. GHC's two
  rejections are rejections here too: `check` fails with a type mismatch naming `Ordering`. A
  single annotation was the old workaround, and it still gives the same answer.
- **Seven corrections, six run verdicts.** Seven behaviors are corrected (R1, R2, R5, R9, T12,
  T01 and T04 in the tests), but a run-level verdict moves for only six. `GT + (1 :: Int)` (T04)
  already failed its run on the base, because the accepted program crashed. Its correction is
  in the `check` stream, from accepted to the mismatch, which the committed rejection test and
  the gate's placement stage prove.
- **Unchanged:**
  - a module that defines its own `LT`, `EQ` and `GT` (under
    `import Prelude hiding (Ordering(..))`) keeps them;
  - a qualified `P.LT` resolves as before;
  - computed Orderings and user enums are untouched.
- **Tests:** `tests/dictionary_evidence_chirho/ordering_chirho/ordering_literals_chirho.rs`. It also keeps
  `succ LT` (T10) and `maximum [LT, GT, EQ]` (T11) with GHC's answers in an unasserted table,
  so the oracle still runs them. No committed test pins their current behavior.
- **Bounded, still open, each its own unit:**
  - `succ LT` (T10) used to crash in the Int addition primop. It is now rejected at `check`,
    naming the missing `Enum Ordering` instance (E0204). The class environment seeds Ordering's
    Eq, Ord, Show, Semigroup, Monoid and NFData, but not Enum, Bounded or Read.
  - `maximum [LT, GT, EQ]` (T11) still crashes in the Int `<=` primop, before and after the
    seed. It is the Ord method-model boundary recorded under `Ord [T]` above.
  - any other constructor the seed lacks still becomes a fresh variable, so an imported `:|` is
    accepted at Int;
  - Eq and Ord still default to Int without Num;
  - a user `data Ordering` under `Prelude hiding` still fails Show (T15).

## Current boundary

- An infix or section use of a class METHOD mints its own occurrence id and carries its
  operator's provenance, like a prefix use, unless the desugarer lowers that operator to a
  primop first. Those carry no occurrence and no evidence: infix `<`, `<=`, `>`, `>=`, `div`,
  `mod`; every section of `+`, `-`, `*`, `/=`, `<`, `<=`, `>`, `>=`, `^`, `**`, and an `==`
  section without an origin; a saturated prefix `div`, `mod`, `quot` or `rem` call. Infix
  `/=` becomes `not (==)` with no provenance. An infix or
  section use of a constrained function that is not a method still resolves through the
  shared canonical id: only a prefix reference mints a reference occurrence for per-predicate
  reference evidence.
- Class methods never mint reference occurrences (the method paths key on canonical ids);
  own-parameter evidence therefore reaches the standard method list only, and a user class
  method at a rigid variable is still dispatched by the pass's inferred parameter keys.
