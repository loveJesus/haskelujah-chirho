<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Builtin import kind contracts, row484 continuation

Owned isolated branch gpt-kind-schemes-chirho, parent3883a6d8 pushed and remote
verified. Main6db522ad, canonical DB and public measurement artifacts remain
untouched. Continue the previous import-contract tasklist without enlarging its
historical log. Local compiler development is reversible and pre-beta; no public
release or authority transfer is implied.

## Question and placement

T13585a imports Data.Monoid.First and rejects the associated RHS Maybe a because
a's inferred classifier has become rigid. The import-contract producer currently
authors only Identity's known Type-to-Type contract; First receives no checked
kind. Hypothesis: missing builtin metadata, not permission for an associated RHS
to specialize the instance's quantified kind. Measure this before editing.

If confirmed, extend the existing driver/frontend_chirho/contracts_chirho.rs
producer with measured closed nominal contracts for the specific builtin
providers. Import selection, qualified aliases, source companion precedence
and local shadowing remain the shared path. Do not seed every same-spelled
constructor globally, add a use-site exception, or guess unknown import kinds.
Bound the authored catalogue by the selected provider, not accumulated module
state. No new dependency or environment representation is needed.

## Checklist

- [ ] Measure exact positive, contradictory-kind, qualified and shadowing
 sources under GHC9.14.1 and immutable ef0a80b8; preserve predictions separately.
- [ ] Demonstrate a main-path integration control red, then repair the producer
 only if those measurements support the hypothesis.
- [ ] Verify fresh references/corpus input, focused frontend/library controls,
 format and changed-line lint; retain all failures and scope limits.
- [ ] Commit and push owned evidence/code/docs with remote-tip verification.
- [ ] Repair the remaining corpus failures and run the lane's full final gates;
 no main landing, denominator/label change or canonical DB closure beforehand.

## Inherited state

Parent3883a6d8 passes parser372, naming138, typing407, typing integration438 and
canaries7, zero ignored/filtered. The live constraints-package test passes one
actual case with1775 filtered; earlier missing-cache self-skips are not reused
as proof. Six selected failures still reject: ClassDefaultInHsBoot/A2/A3,
T11754, T13585a, T14441. This is not a complete corpus count. Clippy has642
warning messages, none on that checkpoint's changed lines; lint-clean remains
unclaimed. Claude's main-based landing is separately held for T10808. His seat
is provider-exhausted and no edits, builds or landing are performed on his behalf.
