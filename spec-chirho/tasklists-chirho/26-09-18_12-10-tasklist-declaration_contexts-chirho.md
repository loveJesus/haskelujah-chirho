<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Declaration contexts lowered with the type grammar

Lane: `instance-obligations-chirho` (claude_chirho), parser-only brick requested by gpt_chirho in room
message #23274 after they confirmed the repair is separable from progress row 484. It lands as its own
commit BEFORE the superclass-obligation kit is reapplied.

## The defect (main 6db522ad)

Class superclass contexts, instance contexts and standalone-deriving contexts all go through one token
scanner (`build_instance_context_chirho` + `parse_simple_constraint_segment_chirho` in the 17k-line
`lower_chirho.rs`). It splits on commas only at parenthesis depth 0, takes the FIRST constructor token
as the class and EVERY variable token as an argument, and returns nothing for a segment containing
`forall` or `=>`. Nothing in the AST records the loss.

| written | lowered today |
|---|---|
| `(Eq a, Show a) =>` | `Eq a a` (Show lost) |
| `C1 x T1 =>` | `C1 x` (constructor argument lost) |
| `Duper (Fam a) =>` | `Duper a` (application flattened) |
| `class (B d, C d) => D d` | superclass `B` only |
| `(forall b. Eq b => Eq (f b)) =>` | dropped whole |
| `instance forall a. Eq a => C [a]` | context dropped whole |

Signature contexts do NOT have this defect: they are lowered as a type and converted by
`type_to_constraints_chirho`. The repair is to use that one grammar everywhere.

## Placement decisions (brick 1)

- New module `crates/haskelujah-parser-chirho/src/lower_chirho/contexts_chirho.rs` owns declaration
  contexts AND the type-to-constraint conversion (moved out of the root, which shrinks). Tests beside it
  in `context_tests_chirho.rs`. Same file names as row 484's branch so the two compose by review.
- The old scanner is deleted, not kept as a fallback.
- Contexts go through `type_from_flat_children_chirho` (the flat grammar the signature path uses), not
  through the older `lower_type_from_token_slice_chirho`, which has no `forall`/`=>` rule at all.
- Two gaps in the flat grammar are prerequisites and are fixed in it, each with its own tests:
  1. a leading `forall` must scope over `=>` and `->` in its body (today the `=>`/`->` split runs first,
     so `forall b. Eq b => Eq (f b)` becomes `(forall b. Eq b) => Eq (f b)`);
  2. a depth-0 `::` inside a group is a kind annotation: keep the type on its left, as main's structured
     and token-slice paths already do (today `(n :: Nat)` becomes the application `n Nat`).
- `type_to_constraints_chirho` keeps main's behaviour (variable-headed predicates still become the `?`
  marker the naming and typing layers already know). Changing that is a single-grammar question for
  signatures too, and row 484 already carries an answer.
- A leading `instance forall a b.` telescope binds the instance's variables. It is stripped before the
  context/head split is lowered; it is NOT a quantified given.

## Bricks

- [ ] 1. `contexts_chirho.rs` + move `type_to_constraints_chirho`/`collect_app_class_chirho`; delete the
      scanner; three callers unchanged in shape.
- [ ] 2. Flat grammar: forall-first, `::` annotation. Tests in `flat_type_tests_chirho.rs`; the existing
      kind-signature test that pinned `(forall f. Type -> Type)` moves to GHC's scoping.
- [ ] 3. Instance and standalone-deriving: strip the leading forall telescope (context AND head).
- [ ] 4. Parser tests: tuple members kept; concrete and multi-argument predicates; applications as
      arguments; equality; synonym application; kind-annotated argument; explicit instance forall (with
      and without a context); nested quantified given in class and instance contexts; standalone deriving.
- [ ] 5. Driver tests with GHC's own outputs for every dictionary claim (two-constraint instance context
      using both dictionaries; two-superclass class projecting the second superclass).
- [ ] 6. Gates: parser, typing, core, driver `--lib`, driver integration, curated; then both corpus axes
      twice with per-file reasons. No landing on an accept-axis regression.
- [ ] 7. Workflow doc `language-features-chirho/flat-type-syntax-chirho.md`, DB row, room report.

## What the gates found (2026-09-18)

- **Runtime evidence, GHC 9.14.1 outputs** (`/private/tmp/haskelujah-declaration-contexts-chirho.JQLXxV`,
  predictions written first): on main's lowering a class with two superclasses is REJECTED ("could not
  deduce `Sized a`"), and a two-member instance context or an explicit instance forall dies at run time
  ("missing STG binding `pretty`"). With the brick all four print GHC's output. Three of my eight first
  predictions were refuted: the failures I blamed on contexts at `Box a`/`Maybe a` heads are a separate,
  older defect (below).
- **`Eq.~~` in boring's instance context** reached the naming scope check for the first time
  ("type not in scope: `Eq.~~`"), which broke the constraints package. GHC contract measured (9 probes,
  `qualified-equality/`): `Q.~` is accepted through ANY qualifier (out of scope it is warning GHC-12003);
  `Q.~~` must be exported by the qualifier's module (Data.Type.Equality, GHC.Exts, GHC.Types yes; Prelude
  no, GHC-76037). Fixed in the naming phase; signatures had the same defect all along.
- **Lane commit 3f27f0af had a false duplicate**: `Lifting Functor (Strict.StateT s)` vs
  `(Lazy.StateT s)` in the constraints package. Fixed by comparing only identical written spellings
  (see instance-obligations workflow); mutation-checked.
- **`orphan_instance_warning_produced_chirho` used a program GHC rejects** (`instance Show Int`,
  GHC-59692). It now uses `instance Show (a -> b)`, which GHC reports as GHC-90177.
- **`frontend_constraints_package_regression_chirho` is vacuous without the package cache.** It returns
  early unless `.haskelujah-packages-chirho/` exists beside the workspace; row 484's worktree has no such
  link, this one does. Both regressions above were visible only here.
- **Pre-existing red, not mine**: `proptest_chirho::parser_handles_nested_parens_chirho` overflows the
  default 2 MB test stack in a debug build on main 6db522ad (verified with my parser edits stashed and a
  visible rebuild). The parser suite is run with `RUST_MIN_STACK=16777216` until that is repaired.

## Found on the way, NOT repaired here

- **An instance at an applied data type does not dispatch at run time** (`instance Describe (Box a)`,
  `instance Pretty a => Describe (Maybe a)`; present in the 2026-09-07 baseline binary). The dictionary
  pass names a context-free instance from the checker's rendering of the head (`$fDescribe(Box t1)`,
  method stub `$prim_Describe_describe_(Box t1)` = "missing method") while the desugarer names the
  methods from the source rendering (`$prim_Describe_describe_Box a`); instances WITH a context are
  skipped by `generate_instance_dicts` ("only ground instances for now") and only list heads have a
  conditional path. Direct calls die with "missing STG binding"; through a constrained function the
  program prints an EMPTY line. No driver test covers a first-order class at an applied data type.
- `r4`: a hand-written `Show (Fix f)` instance is ignored and the derived-looking text is printed
  (`Cons 1 (Cons 2 Nil)` instead of GHC's `Fix(Cons 1 Fix(Cons 2 Fix(Nil)))`): same family.

## Known consumers that still truncate (NOT this brick)

- `process_instance_decl_chirho` keeps only the FIRST argument of each context constraint and drops
  quantified ones; `process_class_decl_chirho` stores superclasses by NAME. After this brick the AST is
  faithful and these two are the remaining loss; they belong to the superclass-obligation brick.
