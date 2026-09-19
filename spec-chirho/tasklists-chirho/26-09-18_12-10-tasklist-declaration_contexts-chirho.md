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

- [x] 1. `contexts_chirho.rs` + move `type_to_constraints_chirho`/`collect_app_class_chirho`; delete the
      scanner; three callers unchanged in shape.
- [x] 2. Flat grammar: forall-first, `::` annotation. Tests in `flat_type_tests_chirho.rs`; the existing
      kind-signature test that pinned `(forall f. Type -> Type)` moves to GHC's scoping.
- [x] 3. Instance and standalone-deriving: strip the leading forall telescope (context AND head).
- [x] 4. Parser tests: tuple members kept; concrete and multi-argument predicates; applications as
      arguments; equality; synonym application; kind-annotated argument; explicit instance forall (with
      and without a context); nested quantified given in class and instance contexts; standalone deriving.
- [x] 5. Driver tests with GHC's own outputs for every dictionary claim (two-constraint instance context
      using both dictionaries; two-superclass class projecting the second superclass).
- [x] 6. Gates: parser, typing, core, driver `--lib`, driver integration, curated; then both corpus axes
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

## First diagnostic corpus pass (bbdf98b4, one pass per axis, zero timeouts)

| axis | main | lane | gained | lost |
|---|---|---|---|---|
| should_compile | 882 | 883 | T15079, T18831, tc124 | T18802, T23514c |
| should_fail | 221 | 229 | TcNullaryTCFail, tcfail023, tcfail035, tcfail036, tcfail056, tcfail073, tcfail118, T15807 | none |

- The three accept gains are the repaired grammar itself: rank-n record fields (T15079, tc124) and
  `(Type :: Type)` (T18831). `spec-chirho/bug-record-field-forall-lowering-chirho.md` (open since
  2026-08-02) asked for exactly the forall-first move; its follow-ups (re-enable the record arm of the
  validity walker; retire `record_field_type_scope_reliable_chirho`) are separate, verdict-moving bricks.
- Both losses had passed on main for the wrong reason and are repaired for GHC's reason:
  **T23514c** `data P5 :: forall a . k -> Type`: a declaration header's inline result kind is not under
  forall-or-nothing (8 GHC probes in `result-kind-scope/`; a value signature and a STANDALONE kind
  signature are, GHC-76037; `data T :: forall a. a -> b` is rejected by GHC for its return kind,
  GHC-55233, which an old naming test had pinned as a scope error). **T18802**: the record field is
  rank-2 for the first time, so construction/update must hand the field type to lambda checking; the
  three record hunks of row 484's e028c1a3 (gpt_chirho) are carried unchanged with their tests.
  Still rejected on both branches, GHC accepts: a rank-n argument of a POSITIONAL constructor applied
  to a lambda.
- Reject gains by reason: five GHC-59692 exact; tcfail118 same defect under GHC-43085 (derived +
  written `Eq Foo`, GHC reports the overlap at the deriving use and names the same two instances);
  tcfail056 ADJACENT (a real duplicate, but GHC stops earlier at GHC-54721 "`<=` is not a (visible)
  method of class `Eq`"; we have no such rule, tcfail077 is its twin); T15807 was a WRONG-reason gain
  from the forall-or-nothing misfire and is gone after the naming repair. Expect +7.

## First landing measurement (78b832ec, frozen af676aad, two byte-identical passes per axis, zero timeouts)

| axis | main | lane | gained | lost |
|---|---|---|---|---|
| should_compile | 882 | 884 | T15079, T18831, tc124 | **T10808** |
| should_fail | 221 | 228 | TcNullaryTCFail, tcfail023, tcfail035, tcfail036, tcfail056, tcfail073, tcfail118 | none |

claude2_chirho reproduced the same 54 failing names with an independently written runner (#23500,
#23792). **Not landable**: T10808 is an accept-axis loss.

- **Observed mechanism** (2026-09-19, local instrumented build, never committed; binary sha256
  13fbd3e3..., evidence
  `/private/tmp/claude-501/-Volumes-ENC-4TB-WDB-CHIRHO-dev-aleluya-personal-aleluya-haskelujah-chirho/3bea0419-e38e-4eae-88bd-cfdd63bd68c6/scratchpad/lane5/fameq-chirho/manifest-chirho.md`).
  Checking the updated field
  `y = y r1` against the OUTPUT record's field type meets `G t8 ~ G t7`: two family applications stuck on
  two distinct unsolved variables (`t7` is the output record's parameter, `t8` the selector's). The
  stuck-family guard in `unify_normalized_chirho` classifies the pair as stuck, then asks the
  family-blind structural unifier (`unify_chirho(..).is_err()`), which succeeds by decomposing; so the
  guard does not defer and the application arm derives `t8 ~ t7`, inverting a non-injective family.
  `out_ty` is already in normal form (`G t7`), so normalizing it at the record site cannot help.
- **Provenance.** gpt_chirho's e028c1a3 (record checking, carried here as 192a1ce1) was followed 24
  minutes later by 327aaef7, which recovered T10808 by deferring `G a ~ G b` instead of deriving
  `a ~ b`. The carry omitted that companion. With that rule toggled on, probes A/F/T10808 accept
  (GHC's verdict) and D, P3, P4, P5 reject with byte-identical diagnostics.
- **Repair, agreed by gpt_chirho (#23806; split verified by claude2_chirho, #23805):** 327aaef7's T10808
  half was carried exactly as commit 8d86bc95 (`defer_stuck_family_equality_chirho` + its call site, its
  two driver tests, its workflow paragraph).
- **Residual, not this repair:** a deferred family equality still stuck at module end is dropped, where
  GHC reports it ("non-injective type family ... ambiguous"). P3 shows the related gap: GHC rejects the
  signature `GChirho a -> GChirho a` in its ambiguity check (GHC-83865); we report use-site mismatches.

## Landing measurement (8d86bc95, frozen CLI 28c98d8c, two byte-identical passes per axis, zero timeouts)

| axis | main | lane | gained | lost |
|---|---|---|---|---|
| should_compile | 882 | 885 | T15079, T18831, tc124 | none |
| should_fail | 221 | 228 | TcNullaryTCFail, tcfail023, tcfail035, tcfail036, tcfail056, tcfail073, tcfail118 | none |

- Binary digest identical before and after all four passes; claude2_chirho verified the membership of
  all four lists against main's committed artifacts (#23870).
- The should_fail reject membership is byte-identical to the 78b832ec measurement taken WITHOUT the
  carried family rule, so that rule moves no reject verdict in the 767 and exactly one accept verdict.
- Focused gates on 8d86bc95: parser 343, naming 136, typing 344, core 128; driver integration 190 across
  18 targets; curated ok; driver `--lib` 1772/1773, the miss being the euler1 scale test timing out at
  its 60 s deadline under load average ~38 (it passes alone in 27.8 s, and passed inside the complete
  workspace run).
- Label: `~29% (228 of 767)` is carried forward under the artifacts' interim rule. Nearest rounding would
  now read ~30%, the first whole-point move on this axis since the label was set at 222 of 767. L.J.'s
  truncation-versus-nearest decision governs it and is still open.

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
