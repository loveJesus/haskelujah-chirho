<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Instance occurrence and represented kind repairs

Frozen candidate 1b1304668f9e2e879cd267ea7f6a7967afb5c7ff14c6821994471772b0c8d6eb
recovers eight of the nine revision-B accept losses in the bounded eleven-file
replay: T11552, T13142, T14010, T15839a, T15839b, T18129, T21323 and T5676.
T3955 still rejects because deriving discards its written class argument and
fully saturates a higher-kinded newtype. This is not a complete corpus pass.

T15712 and T26137 now accept. Their revision-B rejections were respectively an
unrelated method-skolem mismatch and absent generated-instance metadata, not
GHC's deriving-via kind/role rules. No reject capability is claimed for them.

Six calibrated representation controls agree with GHC, including a real
method-universal contradiction, a user-defined Type distinct from the builtin
alias, and the unpromoted list-constructor kind error. The two generated empty
instances control occurrence identity, not GND strategy or runtime coercion.
GHC warns that DeriveAnyClass wins when both deriving extensions are enabled.

The first local-Type probe used an invalid import of LiftedRep as a RuntimeRep
constructor. GHC rejected the import, not the intended nominal-type mismatch.
It is retained in initial-reference-chirho.jsonl as a refuted probe and excluded
from fixtures_chirho.rs; the corrected import is calibrated in final-controls.

Every run records its command, exit, timeout and diagnostics; sources/hash are
retained in the reference controls and hashes in the corpus replay. Frozen
binary hashes match before/after; no timeouts occurred. Negative controls are
classified by their diagnostics, not only by nonzero exit.

Implementation: instance evidence uses the post-deriving declaration ordinal
and checked class name, not a shared DUMMY source span. Promotion quotes reach
the shared type parser. The builtin kind Type and TYPE (BoxedRep Lifted) share
the typing normal form, while local Type declarations retain nominal identity.
The control file is consumed through the driver's actual typecheck entry point.

## Associated-family shadowing follow-up

The frozen 1b130466 candidate above has a real false accept discovered while
reviewing import abbreviation. With an unqualified GHC.Exts import and a local
associated family TYPE r = Bool, it accepts Proxy Type -> Proxy Bool = id.
GHC 9.14.1 rejects at the intended Type/Bool mismatch. A companion explicitly
qualified ProbeChirho.TYPE application is accepted by both. Sources, hashes and
the before outputs live in shadowing-chirho/before-chirho.jsonl; these two cases
extend the calibrated representation table from six to eight.

The repair includes associated family heads in local type ownership, preserves
qualified primitive constructor identities, and refuses family/synonym suffix
fallback for known nominal heads. The local family itself still reduces. The
new import-name helper is extracted from the oversized driver root rather than
growing that root. Current-revision gates and the fresh immutable CLI replay are
recorded separately below; do not attribute the 1b130466 results to the fix.

Known boundary, separately measured: a local type synonym Type = Bool can still
capture KindChirho.Type from import qualified Data.Kind as KindChirho through
bare-name synonym fallback. Both parent650be538 and candidate40b2154b wrongly
accept Proxy KindChirho.Type -> Proxy Bool = id; GHC rejects GHC-83865. The
explicitly self-qualified local alias positive accepts in all three. Both
binary hashes were stable before/after. These observations are retained in
shadowing-chirho/qualified-alias-*-chirho.jsonl and excluded from the passing
behavior table. This pre-existing general qualified-synonym lookup gap remains
open; the primitive-constructor repair is not full definition-identity checking.

## Checkpoint verification

Final immutable CLI 8344419381552688b1af2dd8eaf2220a3b20f17d312379443963799f7dfb5278
repeats the eight recoveries and all eight representation verdicts with stable
before/after hashes and zero timeouts. T3955 still fails. The earlier 40b2154b
replays precede a mechanical regrouping of the instance checker arguments; that
change fixes its new clippy argument-count warning without suppression.

The final source passes typing407 and full typing integration445, zero
ignored/filtered. Parser372/naming138/core128 and the live package test passed
before that argument-only change; their exact scope and logs are retained in
shadowing-chirho/checkpoint-chirho/gates-chirho.json. Clippy reports 387 warnings
outside changed-line primary spans, not a warning-free result. Source hashes,
commands and final replay outputs accompany the receipt. Full corpus membership
and landing gates remain open. Main, canonical DB and published measurements
are unchanged by this checkpoint.

## Next deriving brick, not repaired here

The eight peer GHC controls and a replay on 1b130466 are retained in
gnd-next-brick-chirho.jsonl. G1/G2 reproduce the higher-kinded written-argument
loss; G3 requires a unary residual constraint (GHC-73993); G5 cannot eta-reduce
the representation (GHC-26557). G4 was a refuted negative prediction because
the class kind is polymorphic, so it is a valid positive. G7 pins that argument
to Type and is a genuine GHC-83865 negative that the candidate wrongly accepts.
G6/G8 accept. T3955 and G7 remain open; this checkpoint makes no GND soundness
or runtime-coercion claim.
