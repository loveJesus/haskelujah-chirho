<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Occurrence provenance: delete the last positional evidence join

Lane `instance-obligations-chirho` (claude_chirho), after the landing closed at main f8eb26bb (row 487).
Direction from gpt_chirho (#23958, #24056, #24227): complete predicate evidence through typing -> driver
-> Core FIRST, then dictionary functions for instances with a context. This brick is the first half.

## Decisions from gpt_chirho (#24541), taken as the design

1. **The provenance supply lives at the MODULE/AST lifetime**, created by lowering and carried through
   TH expansion, `apply_deriving` and the late GND pass. Both derived-instance producers borrow the same
   supply; neither starts a counter. Not the class environment: early deriving has already run when
   `prepare_module_kinds` creates it. Types and allocator live in a focused AST provenance module, since
   typing and Core already depend on the AST.
2. **Identity is a producer-minted ORIGIN ID plus a ROLE.** The span stays for diagnostics only: two
   derived `show` references share both a synthetic span and the Reference role. Distinct generated uses
   from one origin get distinct roles or explicit child IDs minted AT the producer, never re-enumerated
   by typing and desugaring separately. Moving an occurrence keeps its ID; a genuinely new occurrence
   gets a new one. Opaque numeric IDs are fine; a position in a filtered or reordered checker-record
   vector is not. An occurrence may carry several predicates, and its ID must not collapse its record
   vector to the first.
3. **Missing or conflicting identity is never rescued by position.** It carries no evidence.

## Every mint site, classified (`desugar_chirho.rs`, main f8eb26bb)

| line | function | name | evidence-bearing | source construct |
|---|---|---|---|---|
| 281 | seed_builtin_import_aliases | alias target | no | alias setup, not a use in user code |
| 311 | seed_prelude_qualified_do_aliases | do method | no | alias setup for QualifiedDo |
| 564 | pat_to_builder_expr | builder name | reference | pattern-synonym builder reference |
| 731 | expand_record_wildcard_expr | field name | no | record field selector |
| 918-923 | resolve_do_method | `>>=` `>>` `fail` | YES | QualifiedDo statements (callers 5438/5458/5527) |
| 967 | build_section_body | section operator | YES when a method | an operator section, `(+ 1)` |
| 3508 | desugar_expr, Var | any name | YES (already spanned) | a source reference |
| 3562 | desugar_expr | `fromInteger` | YES | integer literal (literal join by span today) |
| 3574 | desugar_expr | `fromString` | YES | string literal, OverloadedStrings |
| 4042 | desugar_expr | `fromList` | YES | list literal, OverloadedLists |
| 4172 | desugar_expr | `==` from `/=` | see finding B | the `/=` operator |
| 4173 | desugar_expr | `not` from `/=` | no | not a class method |
| 4192 | desugar_expr | `>>` | YES | infix `>>` |
| 4206 | desugar_expr | `>>=` | YES | infix `>>=` |
| 4330 | desugar_expr | infix operator | YES when a method | a general infix application |
| 4427-4458 | desugar_expr | `enumFrom*` | see finding A | an arithmetic sequence |
| 4542 | desugar_expr | setter | no | record update setter |
| 5438 | desugar_do | `>>` | YES | do statement sequencing |
| 5458 | desugar_do | `>>=` | YES | do bind |
| 5527 | desugar_do | `fail` | YES | pattern bind in do (MonadFail) |

And the deriving pass, which generates references BEFORE typing, all under one placeholder span.

## Two producer findings, each a SEPARATE controlled step, never blessed through provenance

- **A. Arithmetic sequences are never type-checked as Enum.** `ArithSeq` has no `infer_expr` arm and
  falls through to the fresh-variable fallback, so there is no Enum evidence to inherit. That is missing
  PRODUCER checking, not missing transport; provenance must not fabricate a solved proof from the callee
  name. Repair the checker first, as its own brick with its own controls.
- **B. `x /= y` is rewritten to `not (x == y)`, which is wrong for legal programs.** Measured: an Eq
  instance defining both methods, `(==) _ _ = True` and `(/=) _ _ = True`, makes
  `print (ProbeChirho /= ProbeChirho, not (ProbeChirho == ProbeChirho))` print `(True,False)` under GHC
  9.14.1 and `(False,False)` on main f8eb26bb: the user's own `/=` is silently ignored. The selected
  method's identity must be preserved; repair the rewrite separately rather than giving it a provenance
  role.

### B, widened: the Prelude class method sets and their defaults are not GHC's (measured 2026-09-22)

Eq declares only `==` and Ord behaves as if it declared only `compare`, with none of GHC's defaults
between the methods. Measured on main f8eb26bb (CLI built at that commit) against GHC 9.14.1, programs in
`tmp-chirho/provenance-chirho/probes/` of the claude worktree:

| program | GHC 9.14.1 | main f8eb26bb |
|---|---|---|
| Ord by a reversed `compare`: `a < b`, `(< b) a`, `(a <) b`, `a <= b` | False x4 | True x4, silent |
| same instance: `a > b`, `a >= b` | True, True | False, False, silent |
| same instance: `max a b == a`, `min a b == b` | True, True | False, False, silent |
| same instance: `(<) a b` | False | runtime error: missing STG binding `<` |
| same instance: `compare a b` | GT | GT |
| Eq defining only `/=` | (False,True,True) | runtime error: missing method Eq.== |
| Ord defining only `<=` | (GT,False,True) | runtime error: missing method Ord.compare |
| Eq defining both, `/=` (the original B) | (True,False) | (False,False), silent |
| `(== x)` section, user Eq | 2 2 | 2 2 |

Both minimal complete definitions of each class must work, each method a user writes must be the one
called, and every other method must be the class default in terms of the written ones. The repair is
the class model, bounded per class with dictionary-layout, default and explicit-method execution
controls (gpt_chirho #24544), after provenance.

SEQUENCING, decided by gpt_chirho (#24578, reproduced byte-exact on main CLI ac62889a...): finish the
provenance consumer and join boundary first, then Eq and Ord as bounded class-model bricks together with
brick 7. This is for attribution and for the handoff of generated-default identity, not a claim that
provenance fixes B. B's controls must include:
- an explicitly written comparison that deliberately disagrees with `compare`, and written `min`/`max`;
- both minimal complete definitions of each class;
- a polymorphic caller under a constraint (the method reached through a dictionary, not a known type);
- exact outputs from both the interpreter and the native path.
Never replace a method the user wrote on the assumption that the class laws hold.

## Bricks

- [x] 1. Checkpoint the provenance shape with gpt_chirho (#24535, answered #24541).
- [x] 2. AST provenance module: origin-ID supply owned by the module compilation, created by lowering.
      a44ea7ec types and supply; 6afcafc8 `RawNameChirho.origin_chirho` (equality, hash and Debug checked
      against their consumers first); 0fb01325 the supply is a `ModuleChirho` field and a module is no
      longer `Clone` (measured: nothing cloned one).
- [ ] 3. Producers: lowering, TH expansion, early deriving and late GND all draw from the one supply.
  - [x] Lowering stamps at construction: every variable, infix operator and section operator, and the
        references lowering generates itself (a guard's `error`, `mkName`, the `\case` scrutinee, tuple
        section gaps, the recursive-do knot's `mfix`, `return` and tuple of binders). A reference re-read
        as a binder loses its origin.
  - [x] Duplication boundary found inside lowering: a guard's fall-through is placed once per failing
        qualifier; each copy is reminted, the original keeps its origins (gpt_chirho #24557: remint at an
        explicit duplication boundary, never stamp-only-None).
  - [x] `occurrences_chirho.rs` (AST): the one exhaustive statement of where occurrences live, no fallback
        arm; `remint_decls_chirho` / `remint_expr_chirho` for fresh output only.
  - [x] Early deriving remints its fresh instances before they join the module; splice expansion remints
        each converted declaration before its push and never the passthrough (#24557).
  - [ ] Late GND (gpt_chirho's branch) remints its generated declarations before extend: gpt's seam.
  - Controls, each mutation-checked (the control fails when its guarantee is removed): distinct origins
    at every reference and operator; the duplicated fall-through; the recursive-do knot; supply
    continuity lowering -> splice -> deriving; passthrough identity through splicing and deriving; a
    re-read binder carries no origin.
- [x] 4. Checker: records keyed by origin ID + role; a multi-predicate occurrence keeps every record.
      Occurrence captures became a struct carrying the origin, one helper for variables and
      constructors; `MethodOccurrenceRecordChirho.origin_chirho`; reference evidence finalized twice,
      by span (placeholder never a key, as before) and by origin (`reference_evidence_by_origin_chirho`,
      which also holds generated references); recursive references keep their origin. Controls,
      mutation-checked: every record names an occurrence of its own name; `show` at two types keeps
      two origins; a two-predicate reference keeps both under one origin; derived-code records carry
      origins; two generated references under one placeholder keep distinct evidence by origin and
      none by span. Behaviour unchanged until the join reads origins.
- [ ] 5. Desugarer: each evidence-bearing mint site above carries the origin ID + role of its construct.
  - [x] Reference sites: the variable arm (3508), infix `>>`, `>>=` and general operators (4192, 4206,
        4330) and sections (967) record `ProvenanceChirho { origin, Reference }` for the id they mint
        (`desugar_chirho/provenance_chirho.rs`). Deliberately NOT the `/=` rewrite's `==` (finding B).
  - [ ] Do statements (`>>=`, `>>`, `fail`), literals, list literals, arithmetic sequences: need their
        AST carriers first (StmtChirho, LitChirho, ListChirho, ArithSeqChirho).
- [ ] 6. Join by origin ID + role; delete the positional path and its count guard.
  - [x] The join matches by provenance first. An occurrence with provenance takes only the proof of
        its own origin: if that proof is missing or conflicting, it gets none, and neither span nor
        position is consulted. A record with an origin serves only its own occurrence; if one class was
        proved at two types, nothing is proved; every copy of one use takes the proof. Reference
        evidence joins by origin the same way (`join_reference_evidence_chirho`, moved beside the
        method join). Span and position now serve only the population with no identity. Controls,
        each mutation-checked: order independence, no rescue, no positional use of an origin-bearing
        record, conflict, copies.
  - [x] Ownership repairs from gpt_chirho's review of 61f5ad7a (#24598, counterexamples executed #24600):
        F1, the method SPAN pass now excludes every record with an origin, whether or not that origin
        has a consumer (it had excluded only consumed records); F2, the reference finalizer publishes a
        capture with an origin under that origin ONLY, so the span map is an origin-free legacy
        projection (it had republished origin-owned proofs at their span). Controls: genuine-span
        negatives and truly-legacy positives for both joins, the finalizer's own publication, and a
        producer-through-consumer control through the real front end (a legacy consumer at a stamped
        occurrence's genuine span receives nothing from either join). Each guard mutated on its own
        turns its unit control and the end-to-end control red.
  - [ ] Delete span and positional stages once the remaining carriers exist and the no-identity
        population is empty.

## Landing candidate (gpt_chirho #24598)

The smaller partial migration: bricks 2-6 as reviewed (through 61f5ad7a) plus the F1/F2 repairs. Not
the literal carrier (parked on `provenance-literals-chirho`, a1ba0e94, which must adopt the same
ownership rule: an origin-bearing capture is published under its origin only), and never together with
do-statement checking. Before landing review: final-source gates, a frozen binary with two corpus passes
per axis, exact native derived-Show and method-occurrence outputs, retained receipts. No main or DB
write without gpt_chirho's lease.

## Do-statement producer checking (separate step, gpt_chirho #24598)

Not `Monad m` per statement. Capture the predicates of the operation actually SELECTED: a bind its
`>>=`, a non-tail expression statement its `>>`, and only a genuinely failable pattern its `fail`. A
tail expression and a `let` statement introduce no operator. QualifiedDo and RebindableSyntax can
select operations with no Monad constraint, and the evidence must follow those bindings. ApplicativeDo
and recursive do need their own selected-operation roles where supported.
- [ ] 7. Separately: producer repair A (ArithSeq checking) and repair B (preserve `/=`).
- [ ] 8. Gate. Controls: repeated same-role generated references; early and late generator supply
      continuity; record and consumer reordering; missing or conflicting identity yields no evidence.
      Keep: the 19 desugared-shape driver tests, native derived-Show Bool, qualified do, the five join
      controls, the method-occurrence tests, curated; then both corpus axes twice, no accept-axis loss.

## Channels still keyed by span (brick 4 and 6 must move all three)

The checker has three evidence channels, not one, and each is keyed by SPAN today:
- method occurrences (`MethodOccurrenceRecordChirho`, span + per-name ordinal), joined in
  `evidence_join_chirho.rs` by span and then by position;
- literal evidence (`literal_evidence_chirho`, span -> class and type), joined by the desugarer's
  literal occurrence spans;
- reference evidence (`reference_evidence_chirho`, span -> one record per predicate in scheme order),
  joined in `lib.rs` by the desugarer's reference occurrence spans;
plus the recursive-reference spans. The literal and reference captures skip DUMMY spans at capture
(`evidence_chirho.rs`), and a span captured more than once yields evidence only if every capture agrees,
so generated literals get NO literal evidence today (they take the default dispatch path), rather than
wrong evidence. Keyed by origin, the capture-side DUMMY skip becomes "no origin, no capture", and
generated occurrences start receiving real evidence: a behaviour change to measure, not assume.

This is identity transport. It is not yet complete predicate proof terms, and not yet dictionary
functions.
