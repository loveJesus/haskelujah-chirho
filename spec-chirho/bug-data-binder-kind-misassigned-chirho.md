<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: a data head binder's kind annotation is mis-assigned to the declaration

**Found:** 2026-08-02, while designing the data/newtype return-kind `Constraint` check (lane 4c).
**Status:** FIXED 2026-09-06 by `claude2_chirho`, in two commits — `d697851e` (the binder
half, for bracketed/complex kinds) and lane A2b (the `_` half, plus the `GHC-55233` check
this document said was blocked). Kept for the record because the reasoning below is the
reason the check was refused four times, and because the "Scope note" on data families is
still open.
**Severity (when open):** blocked a GHC-compatibility check (`GHC-55233`); it also caused
wrong answers on the accept axis, which this document did not anticipate — see below.

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

## The fix — as landed

Exactly as prescribed. `is_head_binder_name_chirho` now admits `_` alongside `VarId` at both
binder sites, and `unreadable_kind_binder_chirho` binds the variable and skips the whole
parenthesised group when the kind is one `parse_kind_atom_chirho` cannot read. The two forms
are now distinct:

| source | `type_vars_chirho` | `kind_sig_chirho` |
|---|---|---|
| `data Foo :: Constraint` (GHC rejects) | `[]` | `Some(Constraint)` |
| `data Foo (_ :: Constraint)` (GHC accepts) | `["_"]` | `None` |

The guard is `declared_return_kind_is_constraint_chirho` in `kind_chirho.rs`: follow the
arrow spine's tail through `ParenChirho`/`ForallChirho`, reject when it is the tycon
`Constraint`, and stand down when the module declares its own type of that name
(`local_kind_decl_names_chirho`, which already existed). It is reached only from the
`data` and `newtype` arms, so a `type family F :: Constraint` — which GHC allows — is not
touched.

## What this document underestimated

It recorded the defect as "no known wrong-answer today". That was wrong: the same
fall-through also bound the annotation's **own type variables** as extra head binders, so
`data SOP (xss :: [[a]])` silently gained an `a` argument. That arity inflation was costing
five accept-axis files (`T12923_2`, `T12923_3`, `T12928`, `T15772`, `T25597`) and was
manufacturing two accept-axis-shaped rejections on the reject axis (`T13871`, `T15552`,
both documented in the should_fail artifact). Fixing only the `::` misattribution, without
also skipping the group, reproduces the arity half and costs `T21515` and `tc269` — that
was tried first and caught by the gate.

## Scope note

Data **families** and data **instances** are not distinct AST nodes — `lower_chirho.rs:862-866`
returns `None` for any data declaration containing the keyword `family` or `instance`, so those
declarations are dropped entirely. The lane-4 map estimated this cluster at +4 to +6 files on
that basis; with families unreachable and the binder defect unfixed, the real reachable unlock
is **1** file (`T14048a`). Fixing the family/instance drop is a separate, larger piece of work.
