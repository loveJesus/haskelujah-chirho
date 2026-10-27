<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Literals at a signature's rigid variable take the binding's dictionary Chirho

## Origin and scope

Row 584 is gpt's evidence repair (#29427). It was split at my checkpoint (#29436) and confirmed in #29438.
- It owns **mechanism A** only: publish the existing own key (`$own:N`) for a literal whose type resolved to a signature skolem.
- Mechanism B, a dictionary-passing `sum`/`product` (C3, N4), is row 586. Native `recip#` (row 583) stays held until both land.
- The branch `rigid-evidence-chirho` starts from main `bae743d0`. It is replayed onto the newest main before the gate.

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/rigid-evidence-chirho/probes-chirho` (C1, C3, F1, N1-N4). **Traces and Core dumps:** `trace-chirho`.
- **Main, silently wrong natively without `recip#`:**
  - N1 `incChirho x = x + 1` gives `(2.5,5)`; GHC gives `(3.5,5)`;
  - N2 `twiceChirho x = 2 * x` gives `1.0e-323`; GHC gives `2.5`.
  - The interpreter masks both: its float primops accept an Int operand.
- **The trace:**
  - `+` in N1 carries `(Num, $own:0)` and is served through `$dNum`. N3, with no literal, is right everywhere.
  - The literal's `fromInteger` occurrence had no evidence. `finalize_literal_evidence_chirho` published only concrete keys, so the pass defaulted the literal to `$fNumInt`.
  - In C1 the literal took `$fNumInt` while `/` took `$fFractionalDouble`.
- **With mechanism A:**
  - N1 and N2 match GHC on all three engines, and N3 holds.
  - C1 and F1 match GHC in the interpreter. The natives still refuse `recip#/1`, which is row 583.
  - The Core: N1's literal is `$sel_Num_fromInteger $dNum 1`, through the same `$dNum` as `+`. C1's literal comes from `$sel_Fractional_super_Num $dFractional`.

## Placement

- **The checker:** `typing-chirho/src/infer_chirho/evidence_chirho.rs`. In `finalize_literal_evidence_chirho`, a `ForallVarChirho` skolem takes `own_parameter_key_chirho`. An instance method's skolem stays unpublished, as for occurrences.
- **Controls:**
  - a typing unit test of the published key;
  - all-engine driver programs for N1 and N2, plus a two-predicate signature for the `$own:N` index;
  - C1, exact in the interpreter, with the natives pinned at the `recip#/1` refusal until 583.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`.

## Checklist

- [x] Measure on GHC and main; trace the evidence and Core; checkpoint the two mechanisms (#29436); gpt confirmed the split (#29438).
- [x] Mechanism A in the checker; N1 and N2 are exact on every engine; C1 and F1 are exact in the interpreter; the Core takes the own route.
- [x] **Moved-shape scan:** typing, core and all driver lib tests, plus dictionary_evidence, evaluation_values, typing_integration, numeric_hierarchy and native_error_helpers. 2618 of 2619 passed. The one that moved is the checker's own unit test, which pinned "rigid literals publish nothing". It is updated to the new contract, extended with `$own:0`, the class-only `$own`, and an instance method's skolem staying unpublished.
- [x] Controls measured on GHC as written; typing 3/3 and driver `fractional_literals` 10/10 pass.
  - The Fractional control first passed even without the fix: the interpreter masks an Int literal inside float arithmetic.
  - A returned literal (`twoChirho _ = 2`) makes it observable. Without the fix the interpreter stops at `ShowFloatChirho: expected Double#, got 2#`.
- [x] Reach (`tools-chirho/reach_584_chirho.zsh`, log `reach-r2-chirho.log`):
  - **M1, no key at the skolem:** it fails the typing test and both driver controls. Natively it shows main's old answers: `(2.5,5)`, `1.0e-323` and `((0.5,15),(3,2.0e-323))`.
  - **M2, class-only key ignoring the index:** it fails the typing test and the two-predicate control.
  - Both restores were byte-identical.
- [x] **A local diagnostic, never committed:** with the held `recip#` patch applied, C1 and F1 match GHC natively and C3 does not. C3 is row 586.
- [x] rustfmt is clean, and Clippy finds nothing in the touched files.
- [ ] Commit; replay onto the newest main; judge main against the contract copy; warm gate; record it.
- [ ] Land on main and close progress row 584 (gpt).
