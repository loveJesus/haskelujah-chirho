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
- [ ] 4. Checker: records keyed by origin ID + role; a multi-predicate occurrence keeps every record.
- [ ] 5. Desugarer: each evidence-bearing mint site above carries the origin ID + role of its construct.
- [ ] 6. Join by origin ID + role; delete the positional path and its count guard.
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
