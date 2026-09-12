<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Open-family head defaults, row484

Owned checkpoint e7439871 and diagnostic3cf1b965 are committed. The frozen
diagnostic is883/938 accept and249/767 reject, with thirteen main-relative
gains and twelve losses. Main121d4f2c, canonical DB484 and public artifacts
remain unchanged. Builder/DB lease retained; no corpus CPU embargo now.

## Brick 1: policy, not inverse improvement

GHC9.14.1 -ddump-tc shows T11348's TrivialFamily has Type -> Type. Our open
head incorrectly generalized its unannotated parameter. Its row then correctly
refused to match an unknown hidden input, exposing that earlier policy error.
An open row is not allowed to infer the missing equality in reverse.

Recommendation: retain shared lexical head preparation but default unannotated
visible parameters and an omitted result kind on a direct OPEN family to Type.
Keep explicit parameter/result kinds and standalone schemes authoritative.
Closed families still infer from their equations; associated families use
their existing class-owned path and are not part of this change. No new AST
variant, dependency, nominal-name guard, or whole-environment copy is needed.

Primary contract: GHC kind-polymorphism guide, principles of kind inference
and complete user-supplied signatures. No RHS means defaulting unless a source
kind says otherwise. Fresh GHC observations, not the rule wording alone, decide
the controls, including NoCUSKs implied by StandaloneKindSignatures.

This is pre-beta/local/reversible work under L.J.'s continue direction. Confidence
high on T11348's producer defect, medium on corpus reach. Alternative: defer the
repair, leaving a demonstrated false rejection. Correction cost is one isolated
checkpoint; main landing still requires zero main-relative accept regressions.

## Checklist

- [x] Preserve pushed open-row implementation and frozen corpus diagnostic.
- [x] GHC classifier reductions localize T11348; source policy checked against
  primary documentation rather than inferred from a family-row match.
- [x] GHC controls for input/result defaulting, written polymorphism, standalone
  signatures, mixed heads and closed-family inference; demonstrate candidate reds.
- [x] Apply open-head policy before publishing the family scheme.
- [x] Focused, typing/integration and explicit-CLI gates.
- [ ] Commit/push separately, then freeze source and CLI for the diagnostic.
- [ ] One frozen diagnostic per axis and exact set/reason accounting.
- [ ] Full current driver/workspace execution and final two-pass landing gates.

The first closed-family control misused a Bool-valued family result as a kind.
GHC rejected the probe; its corrected source puts the family in Proxy's index
and compares the resulting type against Proxy False. The original reference
record is retained in scratch, not silently reclassified as valid.

## Focused result

The eight source controls ran5/8 before the policy repair: unannotated input
and result kinds were wrongly accepted at Bool, and T11348 was wrongly rejected.
All eight pass afterward, without changing their assertions. Explicit named-kind,
standalone, mixed-head and polymorphic-tail controls remain valid; the closed
Bool-to-Bool family still infers and reduces. Typing379/379 also passes with
zero ignored/filtered. Full integration193/193 and canaries7/7 pass with zero
ignored/filtered. Explicit CLI build and workspace all-target check pass; clippy
exits0 with461 warning-message lines including duplicates/summaries, not lint-clean.
The hashed CLI recovers T11348; all eleven independently observed reference
sources agree with GHC's verdict in a24-input focused observation. Eleven other
known main-relative accept regressions remain in that focused set. This is not
a new corpus figure. Full current driver1774 and full workspace execution remain
owed; previous full-driver results must not be presented as current evidence.
