<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Kind binding and declaration checking — 2026-09-10

L.J.: continue working while I sleep. HASKELUJAH/gpt_chirho retains the builder
and canonical DB lease after row483's verified landing at 8bc16dad. This unit
is local/reversible, isolated on gpt-kind-schemes-chirho in the existing owned
kind-signatures worktree; main's fresh row483 CLI remains the A/B reference.
No percentage policy decision, site deployment or deferred declaration-shape work.

## Brick 1

Two measured wrong accepts share missing kind-binding metadata, but have
different checking rules. A standalone `type Box :: k -> Type` must not be
specialized by `data Box (a :: Type)`. The recursive tcfail225 body is accepted
by GHC 9.14.1 with CUSKs and rejected with NoCUSKs. The latter is a recursion/
declaration-completeness distinction, not the standalone subsumption rule.

The current environment stores bare kinds and treats every free variable as
universally quantified at every constructor use. Its unifier treats all kind
variables as flexible. Recommendation: explicit monomorphic/poly-kinded
bindings, quantifier-aware instantiation and substitution, and scoped rigid
checking of written contracts. Data/newtype checking should own one lifecycle:
head, complete/incomplete recursive binding, constructor fields, publication.

Place the binding machinery and its tests in focused kind child modules, and
centralize the duplicated data/newtype field checker there. Something is wrong
with the 3284-line kind root; extract this relevant unit rather than add another
large arm. No new dependency. Existing broad size/lint debt remains a failure,
not waived by this repair. Work must scale with the binder/contract, not clone
the growing environment for each local scope.

Confidence: high in the two reductions, medium in the integration. Alternative
of making every kind variable rigid rejects ordinary inference; making every
recursive use monomorphic rejects valid complete signatures. Neither is a fix.
Checkpoint kind-schemes-before-chirho at 8bc16dad bounds reversal. Required
dependent-kind binders, implicit versus specified quantifier metadata and
unsupported promoted kinds may require a finer representation before rigidity
is safe; reductions decide rather than per-corpus guards.

## Checklist

- [x] Re-read project/goal/Git; prior landing pushed and remote-exact.
- [ ] Record baseline/reference controls and audit completeness/binder consumers.
- [ ] Implement explicit binding/checking lifecycle with focused positive/negative tests.
- [ ] Run parser/naming/typing/driver controls and behavioral execution canaries.
- [ ] Freeze and run full workspace plus two complete passes on both corpus axes.
- [ ] Name every delta by reason; commit/push owned paths, main-path smokes and DB closure.

## Acceptance and limits

Controls must distinguish a rigid standalone contract from an unannotated valid
head, and CUSKs/NoCUSKs from ordinary monomorphic inference. Preserve alpha-
renamed kind scopes, invisible head binders, required-kind arguments, GADT
constructor-local variables, existing quantified-constraint class uses and
GHC-55233's declaration/binder distinction. Scheme instantiation must preserve
sharing internally and separate different uses, without substituting its bound
variables or allowing a local rigid identity into an unrelated declaration.

No claimed corpus gain estimate yet. Upstream baseline is 882/938 accept and
221/767 reject; full workspace3387, zero exclusions. The prior kind-control
source/oracle reductions and three-engine executions remain mandatory controls.
Full kind-dependent types, all class/family/synonym CUSKs, UnliftedNewtypes,
cross-module kind authority and generic record Eq/Show are not implied by a
green focused test. Any unsupported representation exposed by rigidity stays
named, never bypassed by a source-name or corpus-file condition.

## Resume state

Plan/checkpoint only. No compiler source edited or new gates run for this unit.
Canonical row484 is open; only main writes the DB. A read-only independent
review is checking the distinction between annotated kind variables and
recursive instantiation while the main worker audits environment consumers.
