<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: a data head binder's kind annotation is mis-assigned to the declaration

**Found:** 2026-08-02, while designing the data/newtype return-kind `Constraint` check (lane 4c).
**Status:** open. The check that would have exposed it was **deferred**, not landed.
**Severity:** blocks a GHC-compatibility check (`GHC-55233`); no known wrong-answer today.

## What happens

In a data declaration head, a *binder's* kind annotation can end up recorded as the
*declaration's* `kind_sig_chirho`. Given

```haskell
data Foo (_ :: Constraint)          -- GHC ACCEPTS: Foo :: Constraint -> Type
```

`lower_data_decl_chirho` (`crates/haskelujah-parser-chirho/src/lower_chirho.rs`, the
`DoubleColonChirho` arm) produces

```
DataDeclChirho {
  name_chirho: "Foo",
  type_vars_chirho: [],                       -- the binder was lost
  constructors_chirho: [],
  deriving_chirho: [],
  kind_sig_chirho: Some(ConChirho("Constraint")),   -- WRONG: this is the binder's kind
}
```

The binder is dropped because `try_parse_kind_annotated_tyvar_chirho`
(`lower_chirho.rs:9058-9066`) bails when the binder is not a `VarIdChirho` token — `_` is not —
and the fall-through then treats the annotation as the declaration's own return kind.

Related fall-throughs on the same arm: a bracketed kind such as `(a :: [Constraint])` or
`(a :: [Type] -> Constraint)` defeats `parse_kind_atom_chirho`
(`lower_chirho.rs:9283-9345`) and takes the flat scanner, which splits on `->` and again
attributes the tail to the declaration.

## Why it matters

GHC rejects a data/newtype whose **return kind** is `Constraint` (`GHC-55233`, "Data type has
non-* return kind"). That is an unconditional rule — no extension licenses it — which makes it
one of the cleanest reject-axis checks available (target: `T14048a`).

It cannot be implemented today. `data Foo (_ :: Constraint)`, which GHC **accepts**, produces a
`DataDeclChirho` **identical in every field a guard can read** to `T14048a`, which GHC
**rejects** — the two differ only in `SpanChirho`. Three narrowings were tried and each was
refuted:

| narrowing attempted | killed by |
|---|---|
| require `type_vars_chirho.is_empty()` | `data Foo (_ :: Constraint) = MkFoo` and the empty-decl form |
| additionally require `constructors_chirho.is_empty()` | `data Foo (_ :: Constraint)` (EmptyDataDecls) |
| require a bare `ConChirho` with no arrow spine | both of the above |
| gate on an extension | `T14048a` itself enables `ConstraintKinds` |

## The fix

Make the data-head parser bind `_` (and bracketed/complex kinds) as a genuine
`TyVarChirho` with `kind_annotation_chirho`, so `kind_sig_chirho` only ever holds a kind the
source actually wrote after the declaration's own `::`.

Once `kind_sig_chirho` is trustworthy, the guard is small: follow the arrow spine's **tail**
through `ParenChirho`/`ForallChirho`, and reject when it is the tycon `Constraint` and the
module does not itself declare a type of that name.

## Scope note

Data **families** and data **instances** are not distinct AST nodes — `lower_chirho.rs:862-866`
returns `None` for any data declaration containing the keyword `family` or `instance`, so those
declarations are dropped entirely. The lane-4 map estimated this cluster at +4 to +6 files on
that basis; with families unreachable and the binder defect unfixed, the real reachable unlock
is **1** file (`T14048a`). Fixing the family/instance drop is a separate, larger piece of work.
