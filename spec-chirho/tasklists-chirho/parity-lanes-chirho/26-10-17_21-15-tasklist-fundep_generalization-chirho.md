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
    uncertain, so the refutation is withheld.

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

Kept as boundaries, not tests:
- S01 and S02: the conservative superclass refusal keeps them accepted, while GHC rejects them.
- R03b, R05, R10, M02 and M03: pre-existing wrong MPTC output, re-measured at the gate.

Oracle receipt: `tmp-chirho/fundep-generalization-1017-chirho/embedded-chirho/oracle-chirho.txt`.

## Restored-baseline mutation plan (each one must fail a named test, then restore byte-identical)

| Mutant | Change | Must fail |
|---|---|---|
| MU1 | `fundep_newly_mono_vars_chirho` returns the empty set | rejected controls (N03 and others become quantified) |
| MU2 | the scope test treats TopLevel as Nested | rejected controls (F05 quantified) |
| MU3 | the scope test treats Nested as TopLevel | accepted controls (R08 refuted) |
| MU4 | drop `is_extended_by_another_class_chirho` | accepted controls (S00 refuted) |
| MU5 | match each determining position independently (ignore `merge_chirho`) | rejected controls (F16 withheld) |
| MU6 | `normalized_pred_args_chirho` clones instead of normalizing | rejected controls (R06 withheld) |

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
- [ ] Mutations MU1-MU6, with restores proven byte-identical.
- [ ] Changed-line Clippy (typing, the driver test target), gitleaks.
- [ ] Combined case stack (case + this unit + Char.toEnum): T18398 rejected for
  the checker reason, and the corpus pair repeated.
