<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Class-method repair evidence

The first complete diagnostic of 90de548a was NOT landable. Frozen parent
650be538 accepted 886/938 and rejected 267/767; candidate 5c145678 accepted
880/938 and rejected 269/767. These are diagnostic counts, not new published
compatibility measurements. Every axis had zero initial or unresolved timeouts,
runtime panics, unexpected exits and output-limit failures. The archive retains
the existing runner's per-file source hashes, raw output, exits, binary hashes
before/after, exact runner command and source Git state.

Accept gains: ClassDefaultInHsBoot, ClassDefaultInHsBootA2, ClassDefaultInHsBootA3.
Accept losses: T11552, T13142, T14010, T15839a, T15839b, T18129, T21323, T3955,
T5676. All nine were accepted on main. Membership, not the net count, is the gate.

The two reject gains are wrong reasons (Claude2 #23813, diagnostics checked):
T15712 is rejected for unrelated method-skolem mismatches instead of GHC's
DerivingVia kind-arity error; T26137 hits our missing checked-kind metadata
diagnostic instead of GHC's DerivingVia role/coercion error. Neither is credited
as a capability. No denominator, label, artifact or main change is authorized.

A/B of the nine losses against revision A (dffbc899) brackets five at rigid
method checking: T11552, T13142, T14010, T21323, T5676. The other four already
fail A. This bracket does not by itself establish each mechanism.

The InstanceSigs reference records were measured before the repair. A renamed
written signature must scope its own variables into the body; a less-general
signature and a body violating a more-general signature must reject. GHC 9.14.1
accepts the two valid controls and rejects the three contradictions. The frozen
candidate wrongly rejects the renamed signature and accepts two contradictions.
The failing driver table reproduced all three discrepancies.

Inspection found that instance lowering collected signatures, then discarded
them while converting method declarations to local bindings. Preserve those
nodes before consuming them. Method-binder provenance also distinguishes source
universals from synthetic inference holes (notably the approximate builtin
Generic representation). This is not a complete InstanceSigs context entailment,
deriving, Generic representation or residual family-equality claim.

The repair is work in progress. Later evidence must name its own binary hash;
none of the parent/candidate observations belongs to the dirty repair.
