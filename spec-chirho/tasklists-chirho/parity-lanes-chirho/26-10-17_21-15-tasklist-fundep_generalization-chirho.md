<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Unit 519: fundep generalization, the case prerequisite (claude2_chirho)

This is the source-only unit for gpt_chirho's canonical record
`26-10-17_20-37-tasklist-case_fundep_prerequisite-chirho.md`, cleared by
SOURCE GO in #27457 after the route in #27449 and its revision in #27454.
Branch `fundep-generalization-chirho` sits on the held 0f256e43. No Cargo
until a named builder slot.

## Bricks

1. 12a043b1: the preserved pure move 7a87f118, same `patch-id`. `infer_chirho/`
   goes from 14 to 13 entries.
2. The rule itself:
   - `infer_chirho/fundep_generalization_chirho.rs` is the focused module, and
     `infer_chirho/` is back at 14 entries.
   - In `infer_chirho.rs`, the explicit `GeneralizationScopeChirho` reaches the
     three real call sites: `generalize_chirho` (top level), `generalize_local_chirho`
     (nested) and the function group in `infer_module_chirho` (top level).
     Besides that, there is one closure call, one placement call in the
     partition loop, and one module-end call before the bare-variable skip.
   - `match_ty_chirho` becomes `pub(crate)`.
   - The controls go in `tests/rigid_tyvars_chirho/fundep_generalization_chirho.rs`.
   - Workflow: `language-features-chirho/fundep-generalization-chirho.md`.

## Contract, as implemented

- (A) mono0 is the environment plus the givens, kept separately.
  - The deferred wanteds' fundeps close it over their NORMALIZED arguments, using
    every declared from/to position, until nothing changes. The result is the
    newly monomorphic variables.
  - Quantifiers are the type variables minus mono0 and minus the newly ones.
  - A wanted that mentions a newly variable, and no rigid one, is never
    quantified at top level. In a nested binding it is quantified only when it
    mentions a quantifier in any position.
  - Every other wanted keeps today's partition. Declined wanteds stay deferred.
- (B) The module-end refutation applies the final substitution, normalizes ALL
  arguments and checks given entailment again. It reuses the faithful,
  fully-known, magic, `~` and implicit-parameter exclusions. It reports
  `no instance for P` only when all of these hold:
  - no other known class extends P's class (an explicit conservative refusal,
    because given entailment cannot see a superclass projected with permuted
    arguments; this keeps S00 accepted);
  - some fundep has every determining argument CLOSED: no variable, rigid
    skolem, synonym, stuck family or `#` head;
  - a determined argument still holds a variable, flexible or rigid (F10);
  - no instance matches the determining positions under ONE merged
    substitution. A derived instance of the class, an arity mismatch, a
    synonym or family in a head, and a prefix tuple or arrow constructor (which
    `match_ty_chirho` does not equate with the sugared form) all count as
    uncertain, so the refutation is withheld. So does an applied variable in an
    instance head where matching meets list, tuple or arrow syntax
    (`needs_syntax_split_chirho`, revision 3 for tc126).

## Controls (each embedded program re-measured on GHC 9.14.1 exactly as built)

Rejected by GHC and accepted by held today, except R09:
- N01, N02, N03
- F04, F05, F06, F09, F09b, F10, F12, F16
- R04, R06
- R09 (already rejected)

Accepted by GHC and by held:
- F01, F02, F03, F07, F08, F09c, F14, F15, F17
- R00, R01, R02, R03, R07, R08
- S00
- M01 runs and prints 3.
- Revision 3: H01, H02, H03, H05, and tc126 (whole, from the corpus). The
  frozen 3ff084c1 CLI rejects each of these five.

Rejected by GHC, accepted by held, and rejected by 3ff084c1 (revision 3):
- H04, the nullary determinant: it must stay a certain mismatch.
- R11, the family determinant solved later: the candidate MU6 witness.

Kept as boundaries, not tests:
- S01 and S02: the conservative superclass refusal keeps them accepted, while GHC rejects them.
- R05, R10, M02 and M03: pre-existing wrong MPTC output, unchanged at the gate.
- R03b: CHANGED from held's empty line (rc 0) to a loud `missing STG binding
  methChirho` (rc 1); GHC prints 6. R11b fails the same way.
- N01m and N03m (and N03g, N03h): a later top-level binding absorbs the leftover
  through the existing first-argument partition. N03p, with `main` first, is
  rejected. See the workflow's boundaries.

Oracle receipt: `tmp-chirho/fundep-generalization-1017-chirho/embedded-chirho/oracle-chirho.txt`.

## Built gate 1 (gpt_chirho #27472): red on R02, revised

At replay eb366a62, the rejected controls and M01 passed, but the accepted
table failed on exactly RangeR02Chirho: `no instance for CChirho t10 ExChirho b`.
The local `g p = methChirho p ExChirho b` leaves the wanted `C p Ex b`, which
the existing partition never quantifies because `b` is rigid, while `g` is
generalized over `p`. Held dropped that leftover without a verdict. The new
module-end refutation judged it outside its scope.

Revision: the refutation abstains for any leftover that mentions a variable
in `local_generalized_vars_chirho`. Every negative control is top-level, or its
variable is newly monomorphic and so never quantified, and stays rejected.
R02n and R02u (GHC rejects, held accepts) stay accepted as named boundaries.

| Mutant | Change | Must fail |
|---|---|---|
| MU7 | drop `mentions_locally_generalized_chirho` | accepted controls (R02 refuted) |

## Restored-baseline mutation plan (each one must fail a named test, then restore byte-identical)

| Mutant | Change | Must fail |
|---|---|---|
| MU1 | `fundep_newly_mono_vars_chirho` returns the empty set | rejected controls (N03 and others become quantified) |
| MU2 | the scope test treats TopLevel as Nested | rejected controls (F05 quantified) |
| MU3 | the scope test treats Nested as TopLevel | accepted controls (R08 refuted) |
| MU4 | drop `is_extended_by_another_class_chirho` | accepted controls (S00 refuted) |
| MU5 | match each determining position independently (ignore `merge_chirho`) | rejected controls (F16 withheld) |
| MU6 | `normalized_pred_args_chirho` clones instead of normalizing | SURVIVED on 3ff084c1: R06's synonym is expanded upstream. Candidate witness R11 (by source, accepted under MU6), not yet run |

## Built gate 2 on 3ff084c1 (gpt_chirho #27485, #27486, #27489)

- Green: the six focused suites; all 601 retained inputs equal to held after
  the two declared masks; all 44 FD probes with no lost GHC agreement.
- Mutation: MU1, MU2, MU4, MU5 and MU7 caught. MU3 was caught elsewhere: R08
  stays accepted, R09 becomes wrongly accepted, and the follow-up was caught.
  MU6 survived. Changed-line Clippy: 0 new.
- Corpus, accept axis: STOP.
  - tc126 is a real regression: GHC and held accept it, the rule rejected it.
  - tc231 is a correct new rejection (GHC-39999) in the accept bucket.
  - No reject axis, repeat, replay or execution has run yet.

Revision 3 (source only, for tc126) adds `needs_syntax_split_chirho`. It mirrors the
shapes `match_ty_chirho` descends through, and marks a position uncertain where
an applied variable of the pattern meets list, tuple or arrow syntax. The
descent through lists, tuples, arrows and the prefix `[]` has no natural
control yet. Only the descent through an applied constructor (H05) does.

| Mutant | Change | Must fail |
|---|---|---|
| MU8 | drop `needs_syntax_split_chirho` from the instance test | accepted controls (H01-H03, H05, tc126) |
| MU9 | an applied-variable pattern is uncertain whatever the target | rejected controls (H04 withheld) |
| MU10 | the walk does not descend (top-level pairs only) | accepted controls (H05) |

## Checklist

- [x] Trace, route, revision and source GO (#27449, #27454, #27457).
- [x] Measured controls N, F, R, S and M; embedded programs re-measured exactly as built.
- [x] Brick 1, the replayed move; brick 2 source, rustfmt-clean.
- [ ] Named builder slot: build freeze; typing lib; driver `rigid_tyvars_chirho`
  (all, with the three new tests), `dictionary_evidence_chirho`, and the
  filtered driver lib.
- [ ] Identity against frozen ba1e5c29 on every retained probe (only fundep
  probes may change), then a corpus pair against ba1e5c29: no accept-axis loss,
  and every new rejection GHC-confirmed.
- [x] Built gate 2 on 3ff084c1 (gpt_chirho): focused suites green, MU 5/7 plus
  the MU3 follow-up caught, MU6 survived; corpus STOP on tc126.
- [x] Revision 3, source only: `needs_syntax_split_chirho`, with controls H01-H05,
  tc126 and R11, each measured on GHC 9.14.1, held and 3ff084c1 as embedded.
- [ ] Re-gate revision 3 at a named slot: focused suites, MU6, MU8-MU10 (MU6
  with R11), then the full corpus pair, reporting tc231's bucket delta.
- [ ] Changed-line Clippy (typing, the driver test target), gitleaks.
- [ ] Combined case stack (case + this unit + Char.toEnum): T18398 rejected for
  the checker reason, and the corpus pair repeated.
