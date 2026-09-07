<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Accept axis — promoted / named kinds (lane A2), 2026-09-06

Agent: `claude2_chirho` (HASKELUJAH:5). Slot + DB claimed in broker #21328, after
claude_chirho posted SLOT-FREE/DB-FREE (#21327) at `4f866dc3`.

Baseline, re-measured by me on a binary I built at `4f866dc3` — **failing set
byte-identical to the committed artifact**, so the landing replicates independently:
should_compile 869 of 938; should_fail 206 of 767.

## The map's premise was wrong — corrected here

`map-accept-axis-chirho.json` / `promoted_poly_kinds_collapsed_chirho` blames the
missing `KindChirho::NamedChirho` constructor and the `_ => StarChirho` fallback in
`type_to_kind_chirho`. Probing the binary first (README trap 8) shows that is not what
these files trip on. The naive shapes already pass: `data A (b :: [Symbol]) = A`,
`Proxy 'A`, `Proxy 'B`, `data Q (f :: k -> *)` all check clean today.

The actual defect is in the parser's data-head scanner. When a binder's kind annotation
is one `parse_kind_atom_chirho` cannot read (`[Symbol]`, `[[a]]`, `i ~> j`,
`forall k. k -> Type`), `try_parse_kind_annotated_tyvar_chirho` returns `None`, the
scanner falls through **into** the parenthesised group, and two things go wrong:

1. the binder's `::` fires the arm meant for `data T :: K where`, so the declaration
   gets a fabricated return kind — `data A (b :: [Symbol])` recorded
   `kind_sig_chirho: Some(ConChirho("Symbol"))`, the brackets simply dropped;
2. type variables *inside* the annotation get bound as extra head binders, inflating
   the declaration's arity (`[[a]]` bound `a`; `forall k. k -> Type` bound `k`).

This is the already-filed `spec-chirho/bug-data-binder-kind-misassigned-chirho.md`
(opened 2026-08-02 during lane 4, fix never written). Its prescribed fix is exactly
what landed here.

## Lane A2a — stop mis-assigning the binder's kind  ✅

- [x] Probe first: dump the lowered `DeclChirho` for both variants; confirm the AST,
      not the kind checker, is where they diverge
- [x] `unreadable_kind_binder_chirho`: find the binder name and the group's matching
      `)`, bind the variable unannotated, skip the whole group
- [x] Rejected the first attempt — guarding the `::` alone still walked the group and
      cost two regressions (`T21515` `[[a]]`, `tc269` `forall k. k -> Type`); both are
      the arity-inflation half, and both clear once the group is skipped whole
- [x] Accept gate, full 938 (find-based, pure-shell detector): **869 → 874**, new
      failures **none**. Fixed: T12923_2, T12923_3, T12928, T15772, T25597
- [x] `cargo test -p haskelujah-parser`: 292 passed / 2 failed, and the SAME two names
      and counts on the untouched baseline worktree at dd6d694a (the preprocessed
      `containers` lexer and intset tests); the nested-parens proptest overflows the
      default test stack there too. Verified against the baseline per trap 6, not assumed
- [x] Two new tests pin the invariant: a binder's kind never becomes the declaration's
      `kind_sig_chirho`, and an unreadable kind never inflates head arity
- [x] Zero build warnings; zero clippy hits inside the new code (checked every clippy
      line number in the file against the 2160-2270 range). NOTE: this crate carries 106
      pre-existing clippy warnings, 87 of them in `lower_chirho.rs` — flagged to L.J., not
      touched, because cleaning them is entangled with the big-file split
- [x] `cargo test -p haskelujah-driver --lib` (trap 17): FIVE tests fail —
      `frontend_record_pattern_field_types_follow_constructor_layout_chirho`,
      `eval_typeclass_chirho::{eval_show_nested_just,maybe_t_bind_success,type_app_nested_type}`,
      `extensions_chirho::prelude_list_expanded`. All five fail IDENTICALLY at 4f866dc3
      with this change reverted, so none is mine — but the suite is red at HEAD and that
      is reported to the room
- [x] Trap 17 runtime check: `haskelujah run` on a literal-bearing program prints
      correctly with an unsigned `main`; the signed `main :: IO ()` variant dies with
      `missing STG binding fromInteger`, which is the filed, pre-existing
      `bug-dict-pass-literal-key-from-sibling-chirho.md` (48 of 537 curated programs).
      Unreachable from this change: it runs only inside the data-head scanner, and that
      program has no data declaration
- [x] Reject axis, full 767: 206 -> 204. Both losses verified BY REASON (trap 15), not by
      error-present — see below
- [x] Two-pass measurement of BOTH axes on the final binary: accept 874 twice, reject 204
      twice, both byte-identical
- [x] Commit by named path (d697851e), artifact supersede (1271f0e8), DB row 468 (fe815aaf), pushed to gh_chirho, SLOT-FREE posted (#21333)

## The two reject-axis losses, by reason

Neither was rejected for GHC's stated reason; both were rejected by the arity-inflation
artifact this lane removes.

- `T13871` — we emitted `E0300 kind mismatch in type application: expected *, found
  k7 -> k8` at `data SFoo (z :: Foo a b) where`, because `Foo a b` is a kind we cannot
  read so `a` and `b` became extra head binders. GHC's actual objection is GHC-28374,
  "Data constructor `MkFoo` cannot be used here (it has an unpromotable context
  `(a ~ Int, b ~ Char)`)" — a promotion check we do not implement at all.
- `T15552` — same mechanism from `(kvs :: [Type])`. This file has no `.stderr` in the
  corpus, so under the README's own measurement-truth finding it may not be a genuine
  reject target.

Same class as `T16646Fail2`/`T25679` in the previous measurement. Recovering them for the
right reason needs the GHC-28374 unpromotable-context check — reject-axis work, not
attempted here.

## Lane A2b-1 — the `_` binder, and the check it unblocks  ✅

`try_parse_kind_annotated_tyvar_chirho` bailed on any binder that was not a `VarId`, and
`_` is not one, so `data Foo (_ :: Constraint)` still fell through to the declaration-kind
arm even after A2a. Both binder sites now share `is_head_binder_name_chirho`.

- [x] `_` accepted as a head binder name at both sites (readable and unreadable kinds)
- [x] The two forms are now distinct — `data Foo :: Constraint` is `[]` + `Some(Constraint)`,
      `data Foo (_ :: Constraint)` is `["_"]` + `None`
- [x] `declared_return_kind_is_constraint_chirho` in `kind_chirho.rs`: GHC-55233, the check
      lane 4 refuted four ways. Follows the arrow tail through `ParenChirho`/`ForallChirho`;
      stands down when the module declares its own `Constraint` (`local_kind_decl_names_chirho`,
      already present); reached only from the `data`/`newtype` arms, so `type family
      F :: Constraint` — legal in GHC — is untouched
- [x] Four new tests (1 parser, 3 typing) pinning both directions and the shadowing case
- [x] Gates, two passes each, byte-identical: accept **874 held, zero new failures**;
      reject **204 -> 205**, gained `T14048a`, lost nothing
- [x] `cargo test -p haskelujah-parser` 293 passed / same 2 pre-existing;
      `-p haskelujah-typing` 307 passed, 0 failed; zero warnings from either crate
- [x] Bug doc closed out, including a correction: it recorded "no known wrong-answer
      today", but the arity half was costing five accept-axis files and manufacturing two
      reject-axis rejections

## Lane A2b-2 — REFUTED as briefed, and what was actually wrong  ✅

Reachability check first (trap 8), against the 64 failures remaining at `c9e7c484`.
**Five of the six shapes the map said needed `KindChirho::NamedChirho` already pass**:

| probe | verdict |
|---|---|
| `data A (b :: [Symbol])` + `A '["x"]` | ok |
| `data A (b :: [[Type]])` | ok |
| `class Varpi (f :: i ~> j)` | ok |
| `data DimSimple (s :: Sig2 k)` | ok |
| `data HList (as :: [Type]) where` | ok |
| `A '(Int, Bool)` — promoted tuple | FAILS |

They pass because A2a made an unreadable binder kind *skipped* rather than
*misattributed*, and a flexible binder is sufficient for all of them. The promoted tuple
is the only genuine gap and appears in 2 of 938 files. So the enum-constructor work
across `decl_chirho.rs` + `kind_chirho.rs` was NOT built: it would have bought ~nothing.
The map's estimate went +14 -> +5 delivered -> ~0 remaining by that route.

**What was actually wrong was the same scanner, in a third disguise.** A GADT
constructor's `::` was read as the *declaration's* kind signature:

```haskell
data T where
  MkT :: { f :: Int, x :: Char } -> T   -- kind_sig became App(Int, Char); arity became 2
```

Both halves again — the brace fields folded into an application (the bogus E0300), and
the field NAMES, being VarIds, were collected as head binders. One field hid it (nothing
to fold); fields typed by variables hid it too (the fold stayed well-kinded and only the
arity was silently wrong, which is worse). The fix is the symmetric pair A2a needed:
`saw_where_chirho` stops both the kind-signature arm and the binder-collection arm.

- [x] Probed first; refuted the briefed feature before building it
- [x] `saw_where_chirho` guards both arms
- [x] Gates, two passes each, byte-identical: accept **874 -> 876**, zero new failures
      (`T3632`, `T16411`); reject **205 held**, nothing lost or gained
- [x] `HardRecordUpdate` and `T14761c` moved to different, later errors — progress, not
      passes; they stay in the failing list
- [x] Test pins the invariant; parser 294 passed / same 2 pre-existing, typing 311 / 0
      failed, zero warnings
- [x] `T14048a` still rejected — the GHC-55233 check from A2b-1 is unaffected

## The through-line of this whole lane

Three landings, three disguises of ONE defect: a data-head scanner that could not tell
whose `::` it was reading. Parenthesised binder groups (A2a), the `_` binder (A2b-1), and
GADT constructor lines (A2b-2). Each time the misread produced BOTH a fabricated
declaration kind AND arity inflation from binders that were never binders; each time
fixing only the `::` half left the arity half and cost files. The handoff map read the
symptom as a missing kind representation and estimated +14 for building one; the actual
total was +7 accept and +1 reject from roughly forty lines of scanner guards, and the
representation work turned out to be unnecessary.

## Still open (NOT done here)

- promoted tuples `'(Int, Bool)` — 2 files (`T14010`, `T18831`)
- the remaining 62 accept failures are other roots: 29 E0200, 20 E0300, 9 E0202
- GADT record constructor FIELDS are still dropped entirely by the lowering
  (`fields_chirho: []`), which is why `HardRecordUpdate`/`T14761c` only moved. That is a
  real lowering gap and the natural next lane here.

Of the 28 kind-axis failures, 11 carry a head binder our kind grammar cannot read and
5 clear with A2a alone. The rest need real representation, i.e. the map's original
`NamedChirho(String, Vec<KindChirho>)` — but now for a understood reason:

- `[k]`, `[[Type]]` — list kinds, nested (`T20922`, `T21515`, `T25597`)
- `i ~> j` — a type operator in kind position (`T14451`, currently E0301 infinite kind)
- `Sig2 k` — an applied named kind (`T25597`)
- `'(Int, Bool)` — promoted tuple (probe `q5`, still failing)
- `(Type :: Type)` — kind-annotated kind (`T18831`)
- `forall (k :: KIND) -> Ty k -> Type` — visible dependent quantification (`T13822`)

Both `AstKindChirho` (ast crate) and `KindChirho` (typing crate) need the constructor;
`parse_kind_atom_chirho` needs `[`/application/operator cases. Conservative guard from
the cluster brief still applies: build a named kind only under DataKinds/PolyKinds and
only when the head resolves, so Haskell2010 modules keep today's `*` fallback.

## Why this matters beyond the accept axis

`bug-data-binder-kind-misassigned-chirho.md` records that the GHC-55233 check
("data type has non-`*` return kind", target `T14048a`) was **unimplementable**: the
accepted `data Foo (_ :: Constraint)` lowered identically in every field a guard can
read to the rejected `T14048a`, differing only in span. Four narrowings were tried and
all four refuted. With `kind_sig_chirho` no longer holding a binder's kind, that check
becomes writable — one root cause, both axes. Note `_` binders are still not bound
(the helper requires a `VarId`); finishing that is part of A2b.

## Scope / ownership

Touches `crates/haskelujah-parser-chirho/src/lower_chirho.rs` only so far. A2b adds
`crates/haskelujah-ast-chirho/src/decl_chirho.rs` and
`crates/haskelujah-typing-chirho/src/kind_chirho.rs`. This widens the "typing crate
only" scope I posted in #21328 — the parser items were freed to me in #20534, and the
ast crate is unclaimed; gpt_chirho's naming-crate lane is untouched. Not touching
`infer_chirho.rs`, its child modules, `skolem_chirho.rs`, or the big-file split.
