<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Data-family diagnostic delta, 2026-09-22

One complete pass per axis on df001df8f8ecac55215afcba9739ae38f5ecff8d64f09b1fdf009712538c643b, compared by source hash and filename to retained parent88c171. All observations are valid, the binary hash is stable, and neither axis timed out. These are diagnostic observations, not a landing or published capability update. `comparison-chirho.json` is the membership receipt; `moved-diagnostics-chirho.jsonl.gz` retains both sides and each available committed GHC stderr. Its automatic Unreviewed field is superseded only for the three rows discussed below.

## T17067: recovered

The two data-family applications in type-family equation patterns are nominal. Both arities now pass; the equation-pattern rejection rule remains enabled for actual type families. GHC9.14.1 `-v0 -fforce-recomp -fno-code -fno-write-interface` accepts the unchanged source, exit0 with empty output. Source SHA-256: bc907637c8e7c804100d04fc3e52aef9522f4ddf049cb51595ae385fac46785c.

## T16188: genuine regression, unresolved

GHC9.14.1 accepts the unchanged source with the same command and empty output. Candidate rejects the singleton conjunction with y versus (x && y). Source SHA-256: ed398a724ba898d398dbaf6ad91d94a3a2af45a4430ce733eadf050bb787a7cb.

An actual lowered-declaration trace, retained byte-exact in t16188-producer-chirho.log.gz, contains TyFun with no constructors, Apply as a type family, Sing as a data family, RegExp with App, ReNotEmptySym0 with no constructors, and ReNotEmpty as a type family. SFalse, STrue and SApp are absent. The lower_decl DataDecl arm explicitly returns None for data-instance syntax. This verifies the producer gap; it is not yet an operand-level proof of every downstream error or a complete repair design. The temporary tracing test was removed, and its successful run is observation, not a behavioral gate.

The next implementation must preserve data-family INSTANCE structure and its constructors without overwriting the family head or inventing type-family equations. This checkpoint is not landable while this regression remains.

## T16204c: real mismatch caught, adjacent diagnostic

Fresh GHC9.14.1 rejects at16:8, GHC-83865: Expected Sing@Type a, Actual Sing@Rep a0, from id sTo. Candidate rejects the same Rep/Type incompatibility but reports expected Rep/found Type at16:5 and emits it twice. The verdict detects a real defect; direction, anchor and diagnostic count are not parity. Source SHA-256: 9282f6cfcb04cb69ad9f1bee6a81489687d6bb995b6147da9b46eab7c951e423.
