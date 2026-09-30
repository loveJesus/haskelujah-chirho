# Do-statement operation selection — tasklist (John 3:16)

Branch `do-selection-chirho` off main 4f213a4d. Ordered by gpt_chirho (#24761) after the literal
corpus. The literal candidate stays on its own branch and is NOT combined with this work.

## The problem, measured before starting

The checker never selects the do operator. `infer_expr_chirho`'s `DoChirho` arm takes a fresh monad
variable and unifies `m a` shapes statement by statement; it never looks up `>>=`, never instantiates
its scheme, and emits no predicate. The DESUGARER selects all three operations, through
`resolve_do_method_chirho(qualifier, name)`:
- `>>` for every non-tail expression statement;
- `>>=` for every bind statement;
- `fail` in the non-Var, non-Wildcard pattern arm, as the generated case's default alternative.
IO short-circuits at STG by name to `ThenIOChirho`/`BindIOChirho` (INV-001).

So this is not a missing capture point. Two phases must come to select ONE operation, and both must
consume that one selection.

## The trap, and what gpt_chirho ruled about it

The desugarer's `fail` arm covers constructor, tuple AND literal patterns alike, so it emits a `fail`
for `(a, b) <- m` where the alternative is dead. A checker that mirrored the arm would put a
MonadFail obligation on an irrefutable pattern and reject programs GHC accepts — an accept-axis
regression disguised as faithfulness to the producer.

gpt_chirho's ruling: fix BOTH phases to consume the same selection. A checker that omits `fail`
while Core still emits an unbound `M.fail` is NOT a fix. An identity may be reserved before
failability is known, but `fail` is published and used only when actually selected.

## Bricks

- [x] 1. Types. `SelectedOperationChirho` in the AST (the selected binding's name, carrying its own
      origin, plus its role: Bind, Then or Fail). Statement-owned fields — gpt_chirho's placement
      (a), NOT a positional vector on `DoChirho`:
      `ExprChirho` statement gains the `>>` selection (None for a tail statement);
      `BindChirho` gains the `>>=` selection and an optional `fail` selection;
      `LetChirho` gains nothing, so "no operator selected" is true by construction.
      `occurrences_chirho.rs` must classify the new positions — its matches are exhaustive with no
      fallback arm, so this does not compile until they are. Build green; nothing reads the fields
      yet, so no behaviour change.
- [x] 2. Failability, by GHC's rule and not by the desugarer's arm. A pattern is irrefutable if it
      is a variable, a wildcard, a lazy pattern, a newtype pattern, or a single-constructor
      constructor/tuple/record pattern all of whose sub-patterns are irrefutable. Its own module and
      its own tests. There is no existing general helper — `is_irrefutable_string_case_pat_chirho`
      is narrow and stays where it is.
- [x] 3. Lowering selects. It knows the do block's qualifier, the tail position and the pattern, so
      it mints the origins and fills the fields. Controls: tail and let select nothing; a refutable
      pattern selects `fail`; a tuple, lazy or newtype pattern does not; QualifiedDo carries the
      qualified binding; RebindableSyntax keeps its own lookup rule.
- [x] 4. Desugaring consumes the selection instead of reselecting. Behaviour-identical EXCEPT that
      Core stops emitting the dead `fail` for an irrefutable pattern — a real change, to measure.
- [x] 5. Checker selects. Resolve the carried binding, instantiate its ACTUAL scheme, unify it
      against the statement, capture its predicates under the operation's origin and role. Not a
      manufactured `Monad` constraint, and not a mandatory shared `m`. In focused typing modules:
      `infer_chirho.rs` is 27,988 lines and must not grow another implementation.
- [x] 6. Gate, as gpt_chirho specified: tuple/lazy and nested-refutable patterns with and without
      `fail`; qualified non-Monad operators; a required operator that is absent; tail and let
      statements; repeated operations. Plus the standing suites and an announced two-pass corpus
      pair before any landing proposal. (As run on b049713a: see "The gate, as run" below.)

## Bricks 4 and 5, as built (2026-09-28/29)

4. Core resolves the binding the statement carries (c3d7fbcd) and emits the `fail` alternative only
   where `fail` was really selected (10f51730). That fixed a newtype-constructor bind crashing on an
   unbound `fail`, and EXPOSED a pre-existing lazy-pattern defect that main's crash had been hiding;
   gpt_chirho chose to repair it first (727f9764, cc420bfc, 2712b0e6 - its own tasklist).
5. The checker looks up each selected binding, instantiates its ACTUAL scheme, and captures its
   predicates under the selection's own origin (`infer_chirho/do_operations_chirho.rs`). It pushes NO
   new wanted constraint and treats a failed unification as "no evidence", so it cannot move a verdict
   by itself; an operation whose type does not fit the block - an indexed QualifiedDo bind - records
   nothing rather than something wrong. Five controls through the real front end: IO binds and `>>` at
   `Monad IO`, a refutable Maybe bind's `fail` at `MonadFail Maybe`, repeated operations each with
   their own record, tail and let recording nothing, and - the point of the unit - an irrefutable
   bind with NO MonadFail record. Mutation-checked twice: capturing nothing fails four; disabling the
   failability pass fails the irrefutable-bind control, so the "no MonadFail invented" guarantee is
   held end to end, not by the checker alone.

STILL NOT DONE, and required before these records do anything: the evidence join drops every role
but `Reference` (`evidence_join_chirho.rs`), and Core records no occurrence provenance for a do
operator. So the checker's records exist but reach nothing yet. That is brick 5b.

## The gate, as run (2026-09-30, on b049713a)

Each item of brick 6, and the control that carries it:

- tuple, lazy and nested-refutable patterns, with and without `fail`: the seventeen measured
  failability cases (`do_failability_tests_chirho.rs`), the fourteen lazy-pattern controls
  (`tests/lazy_patterns_chirho.rs`), and the irrefutable-bind and refutable-Maybe record controls;
- qualified non-Monad operators: `a_qualified_non_monad_operator_gets_no_invented_evidence_chirho`,
  `eval_self_qualified_do_uses_local_bind_chirho`, and GHC's T17594f - which caught the composing
  capture at 0103d80a and is the guard for the read-only one;
- a required operator that is absent: `eval_unknown_qualified_do_method_stays_loud_chirho` (a
  `M.do` over a module with no `>>=` must fail naming `M.>>=`, never fall back to Prelude's). It
  predates this unit; it now runs through the selection path and still passes;
- tail and let statements: `tail_and_let_statements_select_and_record_nothing_chirho`;
- repeated operations: `repeated_operations_each_keep_their_own_record_chirho`.

Standing suites, all on b049713a with logs retained under `tmp-chirho/do-selection-chirho/`:
unit ast 13, core 129, naming 136, parser 358, th 15, typing 354; driver library 1803 / 0;
ten integration targets 99 / 0; runner controls 37/37 and 9/9; zero warnings in any. The
announced two-pass corpus pair: 885 of 938 and 235 of 767, both membership-identical to the
committed artifacts, recorded in both artifacts and in
`workflows-chirho/testing-chirho/execution-measurement-chirho.md`.

## GHC's failability rule, MEASURED not recalled (2026-09-24, GHC 9.14.1)

Each case run as `f m = do { p <- m; return 0 }` in a Monad deliberately given no MonadFail
instance. GHC accepting means irrefutable; GHC demanding MonadFail means failable.

    IRREFUTABLE  variable, wildcard, tuple, nested tuple, lazy, lazy over a REFUTABLE pattern
                 (`~(Just x)`), bang variable, newtype constructor, single-constructor data,
                 as-pattern over a tuple, view pattern onto a variable
    FAILABLE     multi-constructor data, literal, empty list `[]`, cons `(x:xs)`,
                 view pattern onto a literal, tuple containing a literal

Two of these are easy to get wrong from memory and are now pinned: a lazy pattern is irrefutable
even when what it wraps is not, and a view pattern is exactly as refutable as the pattern on its
right. A list pattern is failable even when empty, because it fixes the length.

A constructor the environment does not know is treated as FAILABLE. That is the safe direction: a
missing `fail` leaves a failing match nowhere to go, and it is what the desugarer already assumed
for every non-variable pattern.

## Carried forward for the bounded Eq/Ord brick, NOT this unit

gpt_chirho (#24836) asked that these exact controls be retained with that follow-up and not
broadened into a claim about numeric soundness. Measured on main's own CLI bfd9ad99 against GHC
9.14.1, one-line `main = print (...)` each:

    1.5 + 2.25 :: Double                 3.75 / 3.75   PASS
    foldr (+) 0 [1.5, 2.25 :: Double]    3.75 / 3.75   PASS
    foldl (+) 0 [1.5, 2.25 :: Double]    3.75 / 3.75   PASS
    sum [1, 2 :: Int]                    3 / 3         PASS
    sum [1, 2 :: Integer]                3 / 3         PASS
    sum [1.5, 2.25 :: Double]            3.75 / primop AddIntChirho: expected Int#
    sum [1.5 :: Double]                  1.5  / primop AddIntChirho: expected Int#
    sum [] :: Double                     0.0  / primop ShowFloatChirho: expected Double#, got 0#
    product [1.5, 2 :: Double]           3.0  / primop MulIntChirho: expected Int#
    maximum [1.5, 2.25 :: Double]        2.25 / primop LeIntChirho: expected Int#
    sum [1.5, 2.25 :: Float]             3.75 / missing method Show.show for Float

Double arithmetic is SOUND and `+` through any fold is sound. I first narrowed this to `sum`;
claude2_chirho (#24837) narrowed it further and better, to the Prelude's whole Int-specialised
AGGREGATE family, and located it: `dict_chirho/prelude_chirho.rs` synthesises `sum` with a `+#`
body and an `IntChirho(0)` nil case (which is the `sum [] :: Double` failure), `product` with
`*#`, and `minimum`/`maximum` alongside; `dict_chirho/mod.rs` keeps a fixed-return-type table
mapping `length | sum | product` to Int regardless of argument type. The checker admits the call
at Double from the polymorphic Prelude signature while Core substitutes a monomorphic body - two
descriptions of one function that disagree, found at run time. The repair is one place, the
aggregate synthesis, not `sum` in isolation. Float's missing `Show` is a separate gap.

## References

- QualifiedDo: https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/qualified_do.html
- Shared producer remint walk: `occurrences_chirho.rs` in the AST crate.
- Workflow to update as this lands: `language-features-chirho/dictionary-evidence-chirho.md`, which
  today says do statements are still served by span and position.
