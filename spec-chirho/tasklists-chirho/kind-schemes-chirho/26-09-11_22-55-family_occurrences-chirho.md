<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Family equation occurrence consumers, row484 continuation

Own isolated worktree; parent5d526afb, main121d4f2c unchanged. No canonical DB,
public measurement, membership, percentage policy or deployment change.

## Measured contract

The OpenFamilyRhsChirho source declares a poly-kinded Box, defines an open Base
equation returning Box Maybe, constructs Base Int Bool, and reads True back.
GHC9.14.1 executes True. The3c56cf99 CLI rejects it as Box @t6 versus bare Box.
Changing its signature to Base Int Int gives GHC's Bool-versus-Int rejection.
The driver regression test was shown0/1 before code changes; it exercises both
open and closed forms, exact output and the contradictory ordinary type argument.

## Implementation and gate

- [x] Share equation-local classifier checking between closed rows and top-level
  open rows of locally declared families, after declaration kinds are published.
- [x] Reuse solved nominal occurrence conversion for stored equations. An explicit
  equation policy prevents eager family reduction and pattern-wildcard warnings.
- [x] Require result variables to belong to converted matching inputs; no fresh
  RHS parameter or nominal stand-in is invented when a source binder is absent.
- [x] Driver control1/1 now green; both sources execute True and reject Int/Bool.
- [x] Explicit CLI build; actual T21583 now accepted.
- [x] Typing368/368; canaries7/7, zero ignored/filtered.
- [x] Workspace all-target cargo check passes.
- [ ] Driver integration:110/111. The existing legal ClassifierCycleChirho
  control is newly red; do not weaken or ignore it. No full-workspace test gate.
- [ ] Freeze/push checkpoint and run diagnostic corpus sets, not a landing gate.
- [ ] Preserve type-pattern kind ascriptions and their matching binder provenance,
  then repair all regressions before any main landing.

## Newly exposed missing representation

The legal row is EntryOfValKey ('EntryOfVal (_ :: Elem (k,v) kvs)) = k.
lower_chirho.rs's KindAnnotTypeChirho and ParenTypeChirho paths retain only the
type left of ::. The AST TypeChirho has no ascription variant. The old static
SynonymTypeConverter turned an RHS variable missing from its parameter list into
ConChirho("k"). That was not a valid equation or evidence of annotation support.
The new converter reports an unbound matching input instead, exposing the gap.
The cycle proof itself still terminates; this is a different missing consumer.

Next representation decision: a real type-kind ascription node, containing both
the annotated type and its kind plus their spans, with naming and kind checking
visiting both. Type-level family matching additionally needs the bound identities
from those annotations in its actual matching inputs (including promoted
constructor indices); merely storing a tag or adding RHS variables cannot supply
that contract. This is separate from the deferred data-family/type-data/refined
GADT-result declaration decisions. Do not begin that sweep in a frozen gate.

This is a checkpoint with a known regression, not a completed feature or a claim
that the GHC corpora pass. Diagnostic outcomes and reference records follow here.
