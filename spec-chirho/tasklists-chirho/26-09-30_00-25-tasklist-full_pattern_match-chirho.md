<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Full pattern matching — tasklist

Branch `lazy-positions-chirho`, off the withdrawn do-unit tip 3f7695d3. Started 2026-09-30.

## Why this exists

The do unit's lazy repair bound each variable of a pattern binding through a selector that
checked only the PATH to that variable. Haskell 2010 §3.17.3 rule (d) binds each variable as
`case v of p -> x_i`: the WHOLE pattern is matched when any variable is demanded. So demanding
`x` in `(x, Just y) = (1, Nothing)` must fail, and the tip returned 1. The same unit also made a
BANGED pattern binding lazy. Seven regressions against main, all measured on GHC 9.14.1, main's
compiler (fd0da809) and the tip (e6199f92), withdrawn in room #25463 / #25467, and confirmed
independently by claude2_chirho (#25464, #25468):

    W1 do-let   let (x, Just y) = (1, Nothing); print x      GHC fails  main crash  tip 1
    W3 let-in   let (x, Just y) = (1, Nothing) in x          GHC fails  main crash  tip 1
    W4 where    x where (x, Just y) = (1, Nothing)           GHC fails  main crash  tip 1
    W5 do-bind  ~(x, Just y) <- pure (1, Nothing)            GHC fails  main crash  tip 1
    B2 let-in   let !(a, b) = undefined in 5                 GHC fails  main fails  tip 5
    B3 where    5 where !(a, b) = undefined                  GHC fails  main fails  tip 5
    B7 let-in   let !(Just n) = Nothing in 5                 GHC fails  main fails  tip 5

Measuring for the repair found the ORDINARY match machinery unsound too. None of this is a
regression; all of it is identical on main, and none of it is visible to either corpus axis,
which are check-only:

- a literal BELOW a constructor is never tested (`is_nested_con_pat_chirho` calls it simple and
  `wrap_nested_cases_chirho` skips it): `f (Just 3) = 1; f _ = 0` returns 1 for `Just 4`, in
  equations, case alternatives and pattern bindings alike (N1-N6, K4, W7);
- a record pattern binds listed fields BY POSITION, so fields out of declaration order are
  swapped and a partial pattern binds the wrong field (K1, K5, W9);
- `[x, y]` as a case alternative or pattern binding, and a nested constructor under a lambda,
  crash on a missing STG binding (K2, WA, K6);
- `~` in every match position binds the whole scrutinee or crashes, and a top-level `~` loops
  (16 of 22 position probes differ from GHC);
- a bang is dropped: `let !x = undefined` and an inner `(a, !b)` do not force (B1, B5).

Probes, the runner and three compilers' raw outputs: `tmp-chirho/lazy-positions-chirho/`.

## Architecture (decided at brick 1)

- ONE single-row matcher, `desugar_chirho/row_match_chirho.rs`: match one pattern against a
  variable, continue with a success expression built once every variable is in scope, or
  evaluate a failure expression. It handles every `PatChirho` shape, including nested literals,
  records by DECLARED field order, lists as cons chains, as, view, bang and `~`.
- Pattern bindings through it, in `desugar_chirho/lazy_patterns_chirho.rs`: `p = e` becomes
  `$patbind = e`, one shared full match `$m = case $patbind of p -> (x1..xn)` and a selector
  per variable, as the Report defines it. `!p = e` also forces a match WITNESS before the body:
  the match is forced, not the variables (B6 stays 5, B7 fails).
- `desugar_chirho.rs` (7,417 lines, five times the file limit) is touched only at call sites.
  Splitting it is recorded as its own pure-move unit, not done here.
- The desugar_chirho/ directory goes from five files to six.

## Bricks

- [x] 1. The row matcher, with every shape above (`row_match_chirho.rs`).
- [x] 2. Pattern bindings through it at all four sites (do-let, `where`, `let ... in`, top level)
      and for `~p <- m`; the old per-path selector code is deleted, not kept as a fallback. The
      three local sites now share two helpers instead of three copies of the same loop, and
      `desugar_chirho.rs` is about 330 lines shorter.
- [x] 2b. FOUND ON THE WAY, and required: the parser SKIPPED a binding's leading `!`
      (`parse_value_decl_chirho`), so the desugarer never saw a banged binding at all. main
      passed B2/B3/B7 only because every pattern binding there was strict. The bang is now kept
      as a BangPat inside the PatBind, the pattern-binding parser accepts guards, and a parser
      control pins both forms.
- [x] 3. Controls beside the existing fourteen, each against GHC 9.14.1: a demanded variable
      whose own path matches while a sibling does not, in each binding position (W1, W3-W6); a
      literal sibling (W7); an out-of-order record (W9); a list pattern (WA); bang witness
      controls B1-B7, with B6 as the positive control that the match is forced and not the
      variable. 17 new, 31 in the target, all green. Mutation-checked, each restore verified
      by `cmp`: skipping the literal test fails only the literal control; an unforced witness
      fails exactly the three banged-binding controls; a witness that forces the VARIABLE fails
      only B6; record fields by listed order fails only the record control; an unforced inner
      bang fails only B5's control.
- [x] 4. Gate: unit suites, driver library, integration suites, execution set, announced
      two-pass corpus pair. Only then may the do unit be proposed again, with steps 1-2 inside.
      All on 41eff8f6 (after a formatting-only commit: main was rustfmt-clean, and the 17
      files these two units had left unformatted were formatted, nothing else), frozen CLI
      81b46d20: corpus 885/938 and 235/767, twice, byte-identical, both lists unchanged;
      288/288 rejections are diagnostics; execution 28/35, the same seven failures with
      identical output; driver library 1803/0; integration 116/0; unit suites green; runner
      controls 37/37 and 9/9; zero warnings. Every probe set rerun on the frozen CLI.
- [ ] 5. SUBSUMED BY 6. Once case alternatives, equations and lambdas match through
      `match_row_chirho`, its `~` case already binds a lazy sub-pattern through brick 2, so no
      separate AST elaboration is needed.
- [ ] 6. Case alternatives, equations and lambdas through the same row matcher, which repairs
      N1-N6, K1/K2/K4/K5/K6 and the `~` positions (C, F, L, D rows).

Bricks 5 and 6 may be a separate proposal. That sequencing is gpt_chirho's call.

## Guard fallthrough is broken too (measured 2026-09-30, identical on main)

A row whose guards all fail must fall through to the NEXT row. Ours raises "Non-exhaustive
guards" instead, in every position except a variable alternative followed by `_`:

    G1 case  x | x > 10 -> 1; _ -> 2            on 5          GHC 2       ours 2      ok
    G2 eqn   f x | x > 10 = 1; f _ = 2          f 5           GHC 2       ours error
    G3 case  Just x | x > 10 -> 1; Just x -> x  on Just 5     GHC 5       ours error
    G4 eqn   pattern guard, then f _ = 0        (Just 6)      GHC 0       ours error
    G5 \case x | x > 10 -> 1; _ -> 2            on [5, 20]    GHC 2/1     ours 2/2  SILENT
    G6 eqn   guard with where, then f _ = 2     f 5           GHC 2       ours error

The execution set's GuardFallthroughChirho passes only because its guards live in ONE equation
and end in `otherwise`. Two causes: `desugar_guards_chirho` ends every guard chain in
`error "Non-exhaustive guards"`, with no way to reach the next row; and when any guard is a
pattern or `let` guard, lowering turns the WHOLE guarded right-hand side into one unguarded
expression (`lower_guard_arms_to_expr_chirho`), so fallthrough cannot even be expressed.

## Brick 6 design: rows over the single-row matcher

- `compile_rows_chirho(scrutinees, rows, failure)`: row `i` is its patterns matched left to
  right against the scrutinee variables by `match_row_chirho`, its `where` bindings scoped over
  its guards, and its guards; every failure, whether a pattern or the last guard, continues
  with `$next_{i+1}`. The continuations are let-bound thunks, exactly as
  `desugar_nested_case_chain_chirho` already does, so the size stays linear. Rows are desugared
  in SOURCE order, which the evidence join's ordinals rely on.
- This IS the Report's semantics (rows top to bottom, columns left to right, a row that forces
  a diverging column diverges), so correctness does not depend on any grouping.
- Performance: a first column of constructors in consecutive rows costs one tag test per row
  instead of one switch. Grouping them (the constructor rule of classic match compilation) is
  a later optimization, to be taken only if the execution set or the euler benchmarks show it.
- Order of work, each measured against the probes and the full driver suite:
  6a case expressions (and `\case`), replacing the string eq-chain special case, since the
     matcher already compares strings with `eqStr#`; 6b function equations
     (`compile_multi_pattern_case_chirho`); 6c lambdas (one row, GHC's lambda failure); 6d do
     binds (one row, failure is the selected `fail`, else a pattern failure); 6e list
     comprehension generators (failure skips the element); 6f pattern guards, which need
     guards to carry their qualifiers in the AST: a parser, typing and desugar change, and its
     own step.

## Measured after bricks 1-3 (wip2 CLI, against GHC 9.14.1)

Every binding-position probe matches GHC: W1-W9 and WA (10 of 10), the bang rows B1-B5 and
S1-S5 (B7 and B6 included), the top-level `~` rows T1-T3, and the extra controls X1-X4 (a
partial record, an unused sibling mismatch, a recursive knot). The position probes that still
differ are all MATCH positions, which is bricks 5-6: C1, C3, C4, C5, C7, C8, D1, F1, F3, F4,
K1, K2, K4, K5, K6, L1, L3, L5 and N1-N6. F5 only looks like a match (both exit 1), because
ours fails on a missing binding rather than GHC's pattern failure.

Also pre-existing and still open, identical on main: `let whole@(a, b) = e` (X5) is parsed as a
function binding `whole` with a visible type argument `@(a, b)`, and fails at run time on an
unbound name. It is a parser defect, recorded here and not in this unit.
