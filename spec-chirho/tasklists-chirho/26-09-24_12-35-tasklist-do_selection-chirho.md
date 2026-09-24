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
- [ ] 3. Lowering selects. It knows the do block's qualifier, the tail position and the pattern, so
      it mints the origins and fills the fields. Controls: tail and let select nothing; a refutable
      pattern selects `fail`; a tuple, lazy or newtype pattern does not; QualifiedDo carries the
      qualified binding; RebindableSyntax keeps its own lookup rule.
- [ ] 4. Desugaring consumes the selection instead of reselecting. Behaviour-identical EXCEPT that
      Core stops emitting the dead `fail` for an irrefutable pattern — a real change, to measure.
- [ ] 5. Checker selects. Resolve the carried binding, instantiate its ACTUAL scheme, unify it
      against the statement, capture its predicates under the operation's origin and role. Not a
      manufactured `Monad` constraint, and not a mandatory shared `m`. In focused typing modules:
      `infer_chirho.rs` is 27,988 lines and must not grow another implementation.
- [ ] 6. Gate, as gpt_chirho specified: tuple/lazy and nested-refutable patterns with and without
      `fail`; qualified non-Monad operators; a required operator that is absent; tail and let
      statements; repeated operations. Plus the standing suites and an announced two-pass corpus
      pair before any landing proposal.

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

## References

- QualifiedDo: https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/qualified_do.html
- Shared producer remint walk: `occurrences_chirho.rs` in the AST crate.
- Workflow to update as this lands: `language-features-chirho/dictionary-evidence-chirho.md`, which
  today says do statements are still served by span and position.
