<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: a GADT record constructor's fields and result type are both dropped

**Found:** 2026-09-06 by `claude2_chirho`, while landing the data-head scanner fixes
(`d697851e`, `3bf553dc`, `31c4ef8e`). Surfaced by the last of those: once the bogus kind
error stopped firing, the files underneath it still failed, for this reason.
**Status:** PARSER HALF FIXED 2026-09-07 (`b2027c35`, L.J.-directed). The braces are now
parsed and the fields exist; `main` survives; a field can be constructed, read back and
updated (driver eval test `eval_gadt_record_fields_are_readable_chirho`, exact output).
The RESULT-TYPE half is still open and waits on the AST shape decision below.
**Severity:** silent data loss in lowering. A declared constructor becomes nullary and its
record fields cease to exist, so field selectors and record updates are typed against
nothing. Costs at least `HardRecordUpdate` and `T14761c` on the accept axis.

## What happened (before b2027c35)

```haskell
{-# LANGUAGE GADTs #-}
data T where
  MkT :: { f :: Int, x :: Char } -> T
```

lowers to

```
ConDeclChirho::OrdinaryChirho { name_chirho: "MkT", fields_chirho: [] }
```

Both the fields and the result type are gone, and `MkT` is now a nullary constructor.

## Why — and the deeper cause the first diagnosis missed

The original diagnosis below blamed the LOWERING. That was only half true: the CST never
carried the fields in the first place, because `parse_gadt_con_decl_chirho`
(`cst_parser_chirho.rs`) parsed the name and `::` and then handed the whole signature to
`parse_type_chirho`, which cannot read `{`. The damage ran past the declaration and took
the module's remaining bindings with it — `main` did not survive to STG, while `check`
reported the file clean. Both halves are fixed in `b2027c35`.

`lower_gadt_con_decl_chirho` (`crates/haskelujah-parser-chirho/src/lower_chirho.rs`)
scans a `GadtConDeclChirho` node for two things: a `ConId` for the name, and the first
child node satisfying `is_type_kind_chirho` for the signature. A GADT *record* signature
is not one type node — the CST puts the braces in `RecordFieldsChirho` /
`FieldDeclChirho`, and **neither kind is listed in `is_type_kind_chirho`**. So
`sig_type_chirho` stays `None` and the function takes its fallback arm, which fabricates
an `OrdinaryChirho` with an empty field list rather than reporting that it could not read
the declaration.

## The structural part — needs a decision before code

`ConDeclChirho` has three variants and **none of them can hold what this construct means**:

| variant | carries |
|---|---|
| `OrdinaryChirho` | positional fields, no result type |
| `RecordChirho` | named fields, no result type |
| `GadtChirho` | a full signature type, no named fields |

A GADT record constructor needs *named fields* **and** a *result type* — GADT result types
are the point of the feature (`T1 :: { ft1 :: a } -> T a Int`). So this is not a lowering
patch; it is a fourth shape.

**Recommended shape (claude_chirho, endorsed):** do NOT add a fourth top-level variant.
Model it as GHC does — a GADT constructor always has a result type, and its arguments are
either positional or a record:

```rust
ConDeclChirho::GadtChirho {
    name_chirho,
    args_chirho: GadtConArgsChirho,   // PositionalChirho(Vec<TypeChirho>) | RecordChirho(Vec<FieldDeclChirho>)
    result_ty_chirho,
    span_chirho,
}
```

**Why this beats a sibling `GadtRecordChirho` — measured, 2026-09-07.** The intuitive
argument ("exhaustiveness forces every consumer to handle a new variant") is FALSE in this
tree, and so is the first correction to it ("changing a shape breaks every site"). Both
were claimed in the room and both were wrong. claude_chirho settled it by dummy edit +
`cargo check --workspace` + revert:

| change | sites the compiler forces you to visit |
|---|---|
| new `DeclChirho` variant | **1** (one non-exhaustive match, naming `iface_chirho.rs`) |
| added field on `DataDeclChirho` | 4 |
| **renamed** field on `DataDeclChirho` | **19** |
| added field on `ConDeclChirho::GadtChirho` | 4 |
| **renamed** field on `GadtChirho` (`ty_chirho`) | **13** |

(Lower bounds: `cargo check` skips crates whose dependency failed.)

So neither a new variant nor an added field is compiler-enforced here. **Only removing or
retyping a field every site already names is.** The recommended shape qualifies precisely
because it REPLACES `ty_chirho` — `GadtChirho { name_chirho, ty_chirho, .. }` stops
compiling. Phrased as "add an `args_chirho` field" it would have been ignored at 24 of the
29 `GadtChirho` sites, and this bug would have been re-created by its own fix.

That radius is large and worth knowing before starting: **162 `ConDeclChirho::` sites
across six crates** — typing 96, naming 24, parser 18, driver 12, th 7, core 5 — of which
**31 mention `GadtChirho`** (typing 16, driver/eval 6, parser 3, naming 3, th 2, core 1).

On the typing side the constructor scheme is already built from the signature, and the
result type is kept — `unify_constructor_result_chirho` depends on it for GADT refinement
— so the new work there is filling `con_field_names_chirho` for record-argument GADT
constructors, after which record patterns, wildcards and updates see the fields;
`infer_chirho/records_chirho.rs` reads that table and needs no change.

## How it hid

The corpus does not show this cleanly, which is why it outlived several lanes:

- `31c4ef8e` and earlier, a *different* defect fired first — the constructor's `::` was
  read as the declaration's kind signature — so these files failed with a kind error that
  masked this one. Fixing that turned `HardRecordUpdate` from `E0300` into `E0200`, which
  is what exposed this.
- Six corpus files with GADT record constructors PASS today (`T15586`, `T18802b`,
  `T25094`, `T7169`, `tc244`, `TcIncompleteRecSel`). They pass despite the fields being
  dropped, because nothing in them depends on the field types surviving. Absence of a
  failure is not evidence the construct works — the same lesson as README trap 8.

## What remains open

The result type. `MkG :: { bar :: F a } -> G a Bool Char` now yields a `RecordChirho`
whose fields are right and whose result is taken to be the declaration head, so a REFINED
result is silently replaced by the head's parameters — a wrong scheme rather than a
missing one. `HardRecordUpdate` still fails for exactly that reason
(`expected Float, found Int` on the type-changing update), which is the honest failure.
The shape decision below is what closes it.

## Reproduction

`haskelujah check` accepts the snippet above without complaint today; the defect is
visible only in the lowered AST. A parser unit test in the style of
`gadt_constructor_double_colon_is_not_the_declaration_kind_chirho`
(`lower_chirho.rs` tests) asserting `matches!(con, ConDeclChirho::RecordChirho { .. })`
or a new GADT-record variant will fail on the current tree.

## Two things for the brick after the shape is chosen

1. **The silent fallback must become a diagnostic.** `lower_gadt_con_decl_chirho`
   fabricating an empty-field `OrdinaryChirho` when it cannot read a signature is how this
   survived several lanes. A construct the lowering cannot represent should say so.
2. **The six currently-passing files need a test that depends on a field surviving.**
   `T15586`, `T18802b`, `T25094`, `T7169`, `tc244`, `TcIncompleteRecSel` are green today
   *with the fields dropped*. Without an exact-output driver test that reads a field back,
   they stay green for the wrong reason and this regresses invisibly.

## The larger finding this exposed

A new `DeclChirho` variant is seen by exactly one match in the compiler: every other match
over the enum has a `_` arm. The central AST has almost no exhaustiveness protection, so
representation decisions are currently being steered by which edit the compiler happens to
notice rather than by what the construct means. Making the AST-consuming wildcard arms
explicit — a bounded, run-time-neutral change in naming, typing, desugar and driver —
would make every future declaration form produce a compiler-generated to-do list instead
of a hand search. Proposed to L.J. as a step between the marker and the shapes; not
started.

## Related

- `spec-chirho/bug-data-binder-kind-misassigned-chirho.md` — the scanner defect that
  masked this one, now fixed.
- `spec-chirho/bug-record-field-forall-lowering-chirho.md` — the other open record-lowering
  gap.
