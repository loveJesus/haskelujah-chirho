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

- [x] Measure exact positive, contradictory-kind, qualified and shadowing
 sources under GHC9.14.1 and immutable ef0a80b8; preserve predictions separately.
- [x] Demonstrate a main-path integration control red, then repair the producer
 only if those measurements support the hypothesis.
- [x] Verify fresh references/corpus input, focused frontend/library controls,
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

## Measured result

Eleven source graphs were run under GHC9.14.1, with predictions recorded first;
all predictions held. The immutable ef0a80b8 baseline disagreed on six: three
valid associated-instance uses rejected and three invalid First applications
accepted. The main-path integration test reproduced all six disagreements
before the production edit, then passed all eleven after it. Provider-qualified
uses, import hiding, local shadowing, an unrelated provider and a source module
named Data.Monoid are exercised; no general import-kind completeness is claimed.

The existing authored contract producer now supplies Type -> Type for First
from Data.Monoid or Data.Semigroup. Those are distinct nominal types sharing
a classifier, not one identity. Source companions retain priority. No equation
rigidity, fallback inference or global same-spelling lookup was weakened.

Fresh immutable CLI7b11cea6, hashed before and after replay, agrees on all eleven
reference graphs. Unchanged T13585a passes GHC9.14.1 and this CLI. T10808 remains
accepted. Five of the selected corpus files still reject: ClassDefaultInHsBoot,
ClassDefaultInHsBootA2, ClassDefaultInHsBootA3, T11754 and T14441. This targeted
selection does not establish the full remaining regression set or a new count.

Focused gates: typing integration439/439 and canaries7/7, zero ignored/filtered.
The package fixture is present before and after the live constraints test:
one test actually passed,1775 filtered,66.07s. Final format and CLI build pass.
Clippy exits0 with642 warning messages; none has a primary span on this diff's
changed crate lines. Existing lint and oversized-file debt remain failures of
the final quality target, not a clean gate.

Evidence: test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/default-annotations-chirho/scope-chirho/corpus-chirho/builtin-kinds-chirho/
contains exact source graphs, source hashes, GHC/baseline observations, fresh CLI
replay and focused gate receipts. No complete corpus pass or full driver suite
was rerun for this bounded checkpoint. Next: reduce the remaining associated
family failures, then remeasure the complete gate before any landing request.
