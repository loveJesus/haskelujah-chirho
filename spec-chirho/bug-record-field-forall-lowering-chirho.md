<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: record-field lowering splits on `=>` before lifting a leading `forall`

**Found:** 2026-08-02, while landing the validity-walker lane (lane 4).
**Status:** open. Routed around, not fixed — the validity walker skips record fields because of it.
**Severity:** latent today (it manufactures an AST that does not match the source), but it
blocks a real GHC-compatibility check and mis-scopes rank-N record fields.

## What happens

Record constructor fields are the ONE constructor site our parser does not lower through
`lower_type_chirho`. `parse_record_fields_chirho` (`crates/haskelujah-parser-chirho/src/cst_parser_chirho.rs:925`)
bumps the field type as raw tokens, so `lower_record_field_chirho`
(`crates/haskelujah-parser-chirho/src/lower_chirho.rs:7350`) takes the flat fallback
`type_from_flat_children_chirho` (`lower_chirho.rs:7380`).

That path splits on the top-level `=>` (`lower_chirho.rs:7440`) **before** it lifts a leading
`forall` (`lower_chirho.rs:7493`). So

```haskell
data R = R { fld :: forall a. C a => t }
```

lowers to

```
QualChirho { context_chirho: [QuantifiedChirho { .. }], body_chirho: .. }
```

— a **quantified constraint the source never wrote**. The source wrote an ordinary rank-2
field with a plain class context.

A second symptom on the same path: `fld :: forall a. a -> String` lowers as
`(forall a. a) -> String`, i.e. the quantifier's scope is wrong.

## Why it matters

The validity pass (`crates/haskelujah-typing-chirho/src/validity_chirho.rs`) walks constructor
fields to implement GHC's `GHC-91510` "Illegal polymorphic/qualified type" check. When record
fields were included, six should_compile files that GHC accepts were rejected, all through this
artifact rather than through any defect in the programs:

`Vta1`, `LocalGivenEqs`, `T3018`, `T11339`, `T11339b`, `T11339c`

They are rejected because a `QuantifiedChirho` in constraint position requires
`QuantifiedConstraints`, which none of them enable — correctly, since none of them *wrote* a
quantified constraint.

It also costs a legitimate reject-axis check: GHC does apply the polytype rule to record fields.

## The fix

Move the "Handle `forall`" block at `lower_chirho.rs:7493` **above** the `=>` split at
`lower_chirho.rs:7440`, so a leading `forall … .` is lifted first and record fields lower as

```
ForallChirho { body_chirho: QualChirho { context_chirho: [ClassChirho ..] } }
```

like every other type site. That fixes both the false quantified constraint and the
`(forall a. a) -> String` mis-scoping.

## Why it was not fixed in lane 4

It is a parser change that touches **every record field containing a `forall`** — `tc140`,
`T22537`, `T14154`, `T4310`, `UnliftedNewtypesForall`, `T7891`, and more. It needs its own
938-file accept gate and its own 767-file reject count, not a bundle into a typing-crate guard.

When it lands, re-enable the record arm in `walk_con_decl_chirho`
(`validity_chirho.rs`, currently `ConDeclChirho::RecordChirho { .. } => {}`) and re-run both
axes; `T7019`'s record field becomes reachable, though `T7019`'s *actual* GHC error is on a
Constraint-kinded type synonym and still needs kind information the syntactic pass lacks.
