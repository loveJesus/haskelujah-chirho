<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: `Con {}` is rejected as an under-applied constructor

**Found:** 2026-09-07 by `claude2_chirho`, while verifying why `T18869` started being
rejected after the GADT-record parser fix (`b2027c35`).
**Status:** open. PRE-EXISTING for ordinary records; `b2027c35` extended its reach to GADT
record constructors, which is how it was noticed.
**Severity:** false rejection of valid Haskell. No corpus file exercises it today, so no
gate sees it.

## What happens

```haskell
data T = MkT { f :: Int }
t :: T
t = MkT {}
```

GHC ACCEPTS this. Omitting fields in a record construction is legal; the missing fields
are bottom, and GHC emits the `-Wmissing-fields` warning. We reject it:

```
error[E0200]: type mismatch: expected `(Int -> T)`, found `T`
```

The construction is being treated as an under-applied constructor rather than as record
syntax, so the arity check fires.

Verified on the binary at `c9e7c484` (before `b2027c35`): the ordinary-record form above is
already rejected, so this is not new. The GADT-record form
(`data T where MkT :: { f :: Int } -> T`, `t = MkT {}`) was ACCEPTED there only because the
constructor had no fields at all; with `b2027c35` it has fields, and it now takes the same
wrong path.

## The one place it currently helps, by accident

`should_fail/T18869.hs` became a correct rejection at `b2027c35` because of this defect.
GHC rejects that file under **GHC-95909** — `MkFoo {}` omits a *required strict* field,
which is the real rule. We reject it for arity. The verdict agrees; the reason does not.
Counting it as a reject-axis win would be the `T16646Fail2`/`T25679` class again, so the
artifact records it as accidental. **Fixing this bug will therefore COST one reject-axis
file unless GHC-95909 is implemented in the same change.**

## The fix

Record construction and constructor application are different syntactic forms and must not
share the arity check. `Con {}` and `Con { a = 1 }` should be checked against the
constructor's FIELD set, not its argument arity; a missing field is legal and (per GHC)
warn-worthy, an unknown field is an error. Note the companion defect in the other
direction, which claude_chirho has queued: an update naming a field no constructor declares
is currently ACCEPTED and dies at run time with a missing `$setField_...` STG binding.

Doing both together gives the honest pair: unknown field is an error, missing field is
allowed; and GHC-95909 (missing *strict* field is an error) is the third piece that
recovers `T18869` for the right reason.
