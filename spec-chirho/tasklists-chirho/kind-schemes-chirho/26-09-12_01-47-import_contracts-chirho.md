<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Imported kind and closed-alias contracts, row484

Continue the import decision in the22-55 leaf. Isolated on gpt-kind-schemes-chirho,
parent459ebc72; checkpoint tag imported-type-contracts-before-chirho. Main121d4f2c
and sole canonical DB row484 are unchanged. The twelve driver failures were
measured12/12 on fresh main and0/12 on the lane; they are not inherited reds.

## Representation and boundary

Typing owns an opaque checked kind template and a closed alias with distinct
ordinary and hidden parameters. A module companion travels beside naming's
interface. Naming still decides visible roots; selection follows only those
types and the dependencies of selected value schemes. Re-exported private
dependencies retain defining-module names without becoming source exports.
Quantified kind IDs are simultaneously rebased in the receiving environment,
including classifiers and specificity. Malformed, unclosed templates diagnose
rather than invent a classifier or retain producer-local numeric identities.

The actual frontend now lives in frontend_chirho.rs with transport in its child;
the old raw-AST API remains explicit. Module/project/Cabal/source entry points
carry the companion. The kind directory exceeded15 entries during this work;
binding/environment/scope/transport files are grouped under bindings_chirho with
their Rust module relationships unchanged. No new dependency.

## Evidence and gates

- [x] Imported/local Identity and poly-kinded alias re-export controls were0/2
  before transport;2/2 after. Wrong result and private-name controls still reject.
- [x] The exact twelve driver regressions recovered12/12, zero ignored,
  1760 filtered. This is focused evidence, not a full driver gate.
- [x] Qualified import plus a same-named local data type accepts under both
  GHC9.14.1 and the candidate. The audit's unqualified version is ambiguous
  under GHC; do not call it a required accept. The current CLI wrongly accepts
  that ambiguous source, a separately observed naming gap.
- [x] Two transport controls cover selected-closure growth and defining identities
  through a re-export AND a final consumer. Both fail on the earlier transport,
  both pass on the repair. The initial three-module identity test did not fail
  because it omitted the final importing edge; extend the path, not the claim.
- [x] Typing371/371, zero ignored/filtered, includes malformed-template and
  independent imported-ID/specificity controls. Canaries7/7 unchanged.
- [x] New import controls4/4; qualified/builtin-shadowed Identity expectations
  separately checked against GHC9.14.1. Fourteen focused lib tests pass: the
  original twelve plus the two transport controls, zero ignored,1760 filtered.
- [x] Full integration121/122 retains only the previously enabled ClassifierCycle
  ascription failure; no assertion weakened. One additional builtin-shadow test
  was subsequently added and passes in the focused4/4.
- [x] Workspace all-target check passes; broader lint debt remains unclaimed.
- [ ] Full driver--lib, with no filters or exclusions.
- [ ] Freeze owned code/docs, explicit CLI build, record source/binary hashes,
  independently arbitrate reference controls and commit/push the checkpoint.
- [ ] Run a new exact-set corpus diagnostic. The old43833550 result881/248
  is not a measurement of this implementation.
- [ ] Repair all remaining corpus and integration regressions; final broad and
  two-pass gates before any main landing, public artifact change or DB closure.

## Limits

The companion now carries ordinary type/class/family head kinds and closed
aliases, not all promoted-constructor kind schemes or kind-family reduction
tables. Legacy interface-only modules supply no guessed contract; Identity's
known builtin contract is authored and checked-source companions take priority.
Incremental cache inputs and imported contracts while checking hs-boot remain
separate incomplete paths, not covered by this claim.

Private contract identities are retained by this transport. That is not a claim
that the existing qualified-basename unifier distinguishes every private type:
its pre-existing suffix-equality rule is a separate authority gap. The import
audit's wrong-cast source is rejected by GHC9.14.1; a CLI run currently stops at
missing re-exported names, so that run is not evidence about downstream equality.
No denominator, percentage policy, fixture oracle, deferred declaration shape,
canonical DB or deployment change.
