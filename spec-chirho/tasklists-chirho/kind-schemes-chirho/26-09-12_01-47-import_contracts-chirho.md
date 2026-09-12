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
- [x] Full driver--lib1774/1774, zero failed/ignored/measured/filtered,
  Cargo exit0,1088.76s with the documented16MiB thread stack. The twelve are
  recovered in the full target, not just the earlier focused bracket.
- [x] Freeze owned code/docs, explicit CLI build, record source/binary hashes,
  independently arbitrate reference controls and commit/push the checkpoint.
- [x] Frozen3458e547 is pushed and remote-exact. Five fresh CLI classifier
  verdicts agree with GHC9.14.1, including the wrong-Either kind diagnostic.
  Seven module-graph GHC references are recorded separately; candidate links
  name in-process tests, not identical CLI runs, and two open audit probes have
  no candidate-control claim.
- [x] One diagnostic pass per axis completed2026-09-12 02:20:19EDT:
  accept881/938,reject248/767, zero timeouts/unexpected exits. Both exact failing
  sets are byte-identical to43833550. Nine main-relative accept regressions and
  eight gains are unchanged. This is not the final two-pass landing gate.
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

## Frozen provenance and next step

Source3458e5471eebb5a2b5cc9fa75ce60dcc7d7b067f stayed clean before/after both
axes; explicit debug CLI SHA256
03e59700eb6073db34a585a7b27582861077641bb4c6b7f60d047dce9ed0534e stayed
unchanged. Driver binary SHA256
ea6bab555ca939975593c8bf29620a3e70f198fa361ddaefd052e55669d5f492 and the
complete log digest are in classifier-contracts-chirho/import-driver-gate-chirho.json.
Sibling import-classifiers/import-module-references/import-diagnostic JSONL files
retain exact sources, reference verdicts, failing-set hashes and main/prior deltas.
All are under test-data-chirho/kind-oracles-chirho/classifier-contracts-chirho/.
Raw per-file logs and provenance events remain under
/private/tmp/haskelujah-import-contracts-chirho.X1TVaO/.

The independent annotation audit confirms two necessary halves: retain both
children of the source kind ascription, then materialize the promoted constructor's
solved hidden indices inside the actual matching pattern. Adding an RHS variable
to a whitelist or the family's own hidden inputs does not represent that contract.
The fresh AscribedKeyChirho reduction is accepted by GHC9.14.1 and rejected by
the frozen CLI for the unbound matching input plus both distinct equality witnesses.
It is a next-step source probe, not a repaired or committed execution oracle yet.
Main remains121d4f2c; CPU embargo released, builder/DB lease retained.
