# Lazy pattern bindings — tasklist (John 3:16)

Branch `do-selection-chirho`, its own commit BEFORE brick 5 is proposed, as gpt_chirho directed
(#25099). Brick 5 stops Core emitting a `fail` an irrefutable pattern never selected, and in doing
so it exposed this: a lazy bind that used to CRASH on a missing `fail` now returns a wrong value.
Trading loud for silent is not acceptable, so the lazy defect is repaired here rather than
documented as an exception.

## What is actually broken, measured on main's CLI bfd9ad99 against GHC 9.14.1

    shape                                      GHC   main                      verdict
    let (Just n) = Just 7; print n             7     7                         keep working
    let (a, b) = (3, 4); print (a + b)         7     7                         keep working
    let ~(Just n) = Just 7; print n            7     Just 7                    FIX
    let ~(Just n) = undefined; print 5         5     crash Prelude.undefined   FIX
    let (Just n) = Nothing;   print 5          5     crash no matching alt     FIX
    ~(Just n) <- pure (Just 7); print n        7     crash missing `fail`      FIX
    Just x <- ... (refutable, Maybe)     Just 9/Nothing   same                 keep working

TWO defects, not one, and only the first was exposed by brick 5:
1. `~` BREAKS DESTRUCTURING. `pat_to_alt_con_chirho` maps `PatChirho::LazyChirho` to
   `AltConChirho::DefaultChirho` while `pat_to_binders_chirho` takes the binders from the INNER
   pattern, so the variables bind to the whole scrutinee. Without the `~` the same pattern works.
2. LET PATTERN BINDINGS ARE STRICT. An UNUSED variable still forces the scrutinee, so `undefined`
   and a non-matching constructor both crash where GHC prints 5. This one has nothing to do with
   `~` or with selection, and is visible on main today.

## Why the obvious fix is wrong

Making `LazyChirho` derive its alt-con from the inner pattern would repair the destructuring and
BREAK non-strictness: the generated case would scrutinise eagerly, so `~(Just n) <- pure Nothing`
would fail at the bind where GHC fails only if `n` is demanded. gpt_chirho named this explicitly.
The shape has to be selector bindings, not a case.

## The shape to build

For a lazy pattern with variables v1..vk over scrutinee S, bind each variable to its own selector
and scrutinise nothing at the binding site:

    let v1 = case S of { <con> b1 .. bn -> b1 }
        v2 = case S of { <con> b1 .. bn -> b2 }
    in body

Each `let` is a thunk, so nothing is forced until a variable is demanded; demanding one selects
its field; and a mismatching constructor fails inside the thunk, which is on demand. The
desugarer already uses exactly this idiom for view patterns, with the note that a `let` is used
rather than a case because the STG lowerer does not bind case binders — so the technique is
established here, not invented.

## Bricks

- [x] 1. A selector builder: given a scrutinee id, a pattern and one bound variable, produce the
      selector expression. Shared by every path below rather than written per path.
- [x] 2. Lazy patterns in a do bind (`~p <- m`).
- [x] 3. Lazy AND plain pattern bindings in `let` and `where`, since a Haskell pattern binding is
      lazy whether or not it is written with `~`. This is the larger half and is pre-existing.
- [x] 4a. Controls for the DO half, seven of them in
      `crates/haskelujah-driver-chirho/tests/lazy_patterns_chirho.rs`, mutation-checked: with the
      lazy path disabled six fail and the refutable control correctly stays green.
- [x] 4b. Controls for the LET half: do-let, `where`, `let ... in`, and top level.
- [ ] 4. Controls, as gpt_chirho specified: do and let, non-strictness (unused binding does not
      force `undefined`), demand selects the field, a mismatching constructor fails only on
      demand, and the refutable Maybe control unchanged.
- [ ] 5. Gate: unit suites, driver library, integration suites, then the announced two-pass corpus
      pair for the whole unit before any landing proposal.

## Measured after brick 2, on the branch against GHC 9.14.1

    ~(Just n) <- pure (Just 7); print n            GHC 7   branch 7      demand selects the field
    ~(a, b) <- pure (3, 4); print (a - b)          GHC -1  branch -1     two selectors, right order
    ~(Just ~(a, b)) <- pure (Just (3, 4))          GHC -1  branch -1     nested, both levels
    ~(Just n) <- pure Nothing; print 5             GHC 5   branch 5      unused forces nothing
    ~(Just n) <- pure undefined; print 5           GHC 5   branch 5      unused forces nothing
    ~(Just n) <- pure Nothing; print n             fails   fails         only on demand
    Just x <- ... (refutable, Maybe)        Just 9/Nothing  same          unchanged

A trap paid for once and written into the module: the selectors must be built BEFORE the rest of
the block is desugared, because building them is what binds the variables in scope. The normal
pattern path takes its variables from case alternative binders, so `prebind_all_pat_vars_chirho`
deliberately does not bind an immediate variable child. Desugaring the body first leaves it
pointing at an id nothing binds, which surfaces as "missing STG binding `n`" at RUN time rather
than as a compile error.

## The four pattern-binding paths, measured (2026-09-28)

`"_patscrut"` marks every eager pattern-bind site, and there were four:

    desugar_do_chirho        do-block `let`      was strict   now lazy
    desugar_arm_rhs_chirho   `where`             was strict   now lazy
    desugar_expr_chirho      `let ... in`        was strict   now lazy
    desugar_module_chirho    top level           ALREADY lazy, measured not assumed

Plus the do bind itself for `~p <- m`, which was the shape brick 5 exposed.

    let (Just n) = Just 7; print n                GHC 7   branch 7
    let (Just n) = Nothing; print 5               GHC 5   branch 5     was a crash
    let ~(Just n) = undefined; print 5            GHC 5   branch 5     was a crash
    let (a, b) = (3,4); print (a - b)             GHC -1  branch -1
    where (Just n) = Just 7                       GHC 7   branch 7
    where (Just n) = Nothing, unused              GHC 5   branch 5     was a crash
    let (Just n) = Just 7 in n                    GHC 7   branch 7
    let (Just n) = Nothing in 5                   GHC 5   branch 5     was a crash
    top level (a, b) = (3,4)                      GHC -1  branch -1    unchanged
    top level (Just n) = Nothing, unused          GHC 5   branch 5     unchanged

## Boundaries gpt_chirho set

- Do NOT falsify failability, and do NOT pin a missing-`fail` crash as desired behaviour.
- Keep user-Monad dispatch (#25096 defect 1) and point-free constructor methods (#25096 defect 2)
  SEPARATE from this work. They are pre-existing, they are claude2_chirho's findings, and a
  user-defined Monad therefore cannot serve as an execution control here. IO and Maybe bound only
  those two routes.
- claude2_chirho's proposed dispatch key table is not verified repair until shadowing and
  cross-binding identity are exercised; nothing here depends on it.
