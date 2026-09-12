<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Promoted tuple syntax and family operands, row484

Checkpoint969bdfd8 is pushed and tagged promoted-tuples-before-chirho.
Diagnostic884/938 accept,249/767 reject is not landable: eleven main-relative
accept regressions remain. Main121d4f2c and canonical DB/public artifacts stay
untouched; no corpus CPU embargo is active.

## Brick 1: preserve the term before judging its binders

T14010's Fst/Snd report a result variable outside the matching inputs. Source
reading found that structured promoted parentheses consume only comma tokens,
not tuple elements; lowering then invents the promoted unit constructor. The
flat path also discards promotion on parenthesized tuples. This is a hypothesis
about the observed error until the AST control is executed, not a kind-policy
reason to waive the bound-variable check.

Recommendation: represent a promoted tuple as its existing promoted constructor
applied to its ordered operands, preserving constructor-only and unit forms.
Move promoted CST parsing and lowering out of their oversized root files into
focused promoted_types_chirho modules. Structured and flat paths must call one
tuple builder; no new AST variant or dependency. The ordinary tuple path stays
ordinary. If the recovered constructor lacks a classifier, provide its actual
builtin promotion contract, not an opaque kind chosen to make the example pass.
This local/reversible pre-beta change is within L.J.'s continue authority.

The primary GHC DataKinds guide distinguishes '(a,b) from (a,b). Fresh GHC
controls decide the exact source forms. A kinded family projection must recover
each coordinate, reject an incorrect equality, preserve a following declaration,
and agree across structured signatures and flat record fields. No claim covers
NoListTuplePuns or all malformed-syntax recovery.

## Checklist

- [x] Preserve the preceding implementation, evidence and remote checkpoint.
- [x] Reference-check independent tuple/projection controls; execute parser reds.
- [x] Preserve tuple constructor arity, promotion and all operands at both routes.
- [x] Verify kind/type consumers and independently specified negative controls.
- [ ] Focused/parser/typing/integration and explicit CLI gates; checkpoint/push.
- [ ] Frozen diagnostic and exact per-file delta/reason accounting.
- [ ] Full current driver/workspace and final two-pass gates before main landing.

## Measured implementation, before freeze

Three parser controls went0/3 to3/3. Seven GHC9.14.1 source controls had one
baseline driver pass and six failures. Parser preservation alone yielded6/7,
with the wrong-kind tuple now incorrectly accepted; the actual promoted tuple
contract makes7/7 pass. A fresh explicit CLI accepts T14010 and all five valid
controls, rejects the wrong coordinate with E0200 and the wrong kind with E0300.
Full parser366/366, typing380/380, integration200/200 and canaries7/7 are green
with no ignores/filters. Workspace all-target check, explicit CLI build and fmt
check exit0. Clippy exits0 with467 warning-message lines, including summaries
and duplicates; this is not lint-clean and is not comparable to earlier runs
that omitted the parser target. Final CLI SHA256 is
927ace4e215e68555ab35c9cd3a78456760f2bd5956ec911fa78cc0a996107c9.
The frozen diagnostic is pending, as are full current driver/workspace execution
and the final two-pass landing gate. Main and the published artifacts stay put.

On-demand intrinsic tuple contracts persist beyond the local signature scope
that first sees them. Unit, pair, 8- and32-component kind controls check exact
component classifier propagation and independent instantiation, without seeding
an entire arity catalogue into every fresh module. New promoted CST/lowering
modules remove code from the oversized roots; this is not a claim that their
remaining structural debt is closed.

Reference correction: two initial generated probes joined adjacent promotion
quotes as `'('`, which GHC lexed as a character literal. The retained v2 sources
include the required whitespace. The flat parser control also needed outer
parentheses around the complete Proxy argument. Both corrections preceded the
retained red runs; no expected result was copied from our output.
