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
- [x] 5. SUBSUMED BY 6. Once case alternatives, equations and lambdas match through
      `match_row_chirho`, its `~` case already binds a lazy sub-pattern through brick 2, so no
      separate AST elaboration is needed.
- [x] 6. Case alternatives, equations and lambdas through the same row matcher, which repairs
      N1-N6, K1/K2/K4/K5/K6 and the `~` positions (C, F, L, D rows). Built on branch
      `match-rows-chirho` (stacked on the a9548989 proposal): 6a case expressions (ee4ad26c,
      plus df47cda0 so the STG's newtype erasure accepts a trailing failure default), 6b-6d
      equations, lambdas and do binds (d0099ae4), controls (dc6f5bab), 6e list-comprehension
      generators (cb1ada98). The whole old per-position machinery is deleted.
- [ ] 6f. Pattern guards fall through (G4): guards must carry their qualifiers in the AST, a
      parser, typing and desugar change, and its own unit. Branch `pattern-guards-chirho`,
      stacked on 51cc17c6, started 2026-10-12 under L.J.'s "continue to push forward dev"
      (the design went to gpt_chirho in #25480 and is still unanswered; it is built for his
      review, not landed). Sub-bricks in "Brick 6f" below.
- [x] 7. `let whole@(a, b) = e` (X5): the parser read it as a function binding with a visible
      type argument. A tight `x@p` whose pattern is followed by `=` or `|` is now an as-pattern
      binding (6a97e07a); a spaced `f @t = ...` keeps the function path. Measured on 6a97e07a,
      frozen CLI d6c896eb: corpus unchanged (885/938, 235/767, lists identical), replay 288/288,
      execution 28/35 the same seven, driver library 1817/0, integration 117/0; 81 of 82 probes
      match GHC 9.14.1.

Bricks 5 and 6 may be a separate proposal. That sequencing is gpt_chirho's call.

## Brick 6f: guards carry their qualifiers (decided at its first brick)

GHC's own model: a guarded right-hand side is a list of arms, each a LIST of qualifiers
(boolean guards, pattern guards `p <- e`, and `let`) and a body. The arm succeeds when every
qualifier does, in order, each scoping over the ones after it and the body; ANY failing
qualifier goes on to the next arm, and after the last arm to the next row.

- AST: `GuardedExprChirho { quals_chirho: Vec<StmtChirho>, body_chirho, span_chirho }` replaces
  the single `guard_chirho` expression. A qualifier selects no operation (not a do statement).
- Parser: both guard sites (equations and pattern bindings through `lower_guarded_rhs_chirho`,
  case alternatives through their own loop) build these arms directly. The lowering that
  turned a pattern or `let` guard into ONE unguarded `if`/`case` expression, with the
  fall-through copied and reminted once per failing qualifier, is deleted: it is exactly why
  fall-through could not be expressed.
- Checker: one helper types an arm's qualifiers in a scope of the arm's own (boolean → `Bool`;
  pattern → bound against its source's type exactly as a case alternative binds against its
  scrutinee, including the GADT refinement fallback; `let` → local bindings), then the body.
  The case alternative's pattern step becomes one shared helper instead of two copies.
- Desugarer: guards move to `desugar_chirho/guards_chirho.rs`. Each arm is compiled in source
  order against a placeholder for the next arm; its qualifiers chain through
  `match_row_chirho` (a pattern guard is a one-pattern row against its source); the arms are
  joined bottom-up by the SAME join the rows use (an arm that cannot fail drops the rest; a
  continuation used once is inlined; used more than once, let-bound). After the last arm comes
  the row's fall-through, or the guard failure when there is no next row.
- Walkers (occurrences, linearity, exhaustiveness, recursive-do references, placeholder paths,
  Template Haskell conversion) visit qualifiers as statements. TH's `PatG` becomes a real
  pattern guard; today its pattern is DROPPED and its expression used as a boolean.

- [x] 6f-1. AST, parser, walkers and TH: guards carry qualifiers; the expression lowering is
      deleted. 6f-1 to 6f-3 are one commit: without the checker and desugarer halves, a
      pattern guard would have no meaning at all.
- [x] 6f-2. Checker: qualifier scoping, shared pattern step; the top-level pattern binding
      types its WHOLE guarded right-hand side (today it types only the first arm's body).
      Boolean guards are now checked as an `if` condition is: `| 'c' = 1` was ACCEPTED (R02).
- [x] 6f-3. Desugarer: `guards_chirho.rs`, qualifier chains, the shared join.
- [x] 6f-4. Probes against GHC 9.14.1, frozen CLI a2e6e442 (`tmp-chirho/pattern-guards-chirho/`):
      26 guard probes, 13 differed on the previous tip, 4 differ now (P04, P05, P19: a
      single-line `let` guard followed by another qualifier loops IDENTICALLY on both, a
      layout defect, brick 6g; P20: a pattern guard in a multi-way `if`, rejected on both).
      Rejects: 7 of 8 as GHC (R02 newly rejected; R06, a top-level pattern binding whose
      arms are `Char` and `Bool`, is still accepted on both because guarded-arm results are
      unified leniently for GADTs). S01 was WRONGLY REJECTED by the previous tip: the old
      lowering placed the next arm inside the pattern guard's scope, so `x + 1` saw the
      shadowing `Bool`; it now prints GHC's `(6,1,6)`. Nine controls beside the match-rows
      controls (23 there); six mutations, each caught by its own controls and restored by
      `cmp`: a pattern guard that cannot fall through (3 controls), a pattern guard that
      forces its source (the laziness control), the checker's arm scope leaking (the two
      scope controls), a lenient boolean guard (the `Bool` control), a top-level pattern
      binding typed by its first body (the top-level control).
- [x] 6f-5. Gates, all on the formatted tree a1c70656 and its frozen CLI a2e6e442: unit suites
      ast 13, parser 361, th 15, typing 354, core 129, naming 136; driver library 1826/0;
      integration 117/0; zero compiler warnings, and no clippy lint on any added line. The
      announced corpus pair (room #26955): 885/938 and 236/767, twice each, byte-identical;
      the accept list is unchanged and the reject axis gained exactly one file, T20768_fail
      (`\cases | 'c' -> ...`, GHC's first error, now that a boolean guard is checked as an
      `if` condition is); replay 289/289 diagnostics; execution 28/35, the same seven
      failures. Both measurement artifacts describe a1c70656.
- [x] 6g. A guard's `let` closes where its bindings end (the Report's parse-error(t) rule):
      `| let y = x * 2, y > 10 = y` on one line. Branch `let-guards-chirho`, stacked on the 6f
      tip f0b7ed4c, its own proposal. The parser closes the block at the `,` or the arm's
      `=`/`->` and moves the layout pass's later virtual `}` there, dropping the virtual `;` it
      placed for that block (`cst_parser_chirho/guard_let_chirho.rs`, a child module, so the
      6,400-line parser file does not grow). Probe set `probes-let-chirho/` (T01-T10): all ten
      match GHC 9.14.1, including T06, a line aligned with the closed block that continues
      the body. P04 and P05 now match; P19's guards are right and its remaining difference is
      a Show defect identical on the previous tip (`mapM_ print [Left "a b"]` prints
      `Left ("a b")`). Five parser controls and one eval control; three mutations, each caught
      by its own controls: an equation's `let` running on (4 parser + eval), a case
      alternative's (its parser control + eval), the close keeping the semicolons (the
      aligned-line control + eval). Gates on 49ad05fb and its frozen CLI 7619a077: unit suites
      green (parser 366), driver library 1827/0, integration 117/0, zero compiler warnings,
      no clippy lint on an added line; the announced corpus pair (room #26959): 885/938 and
      236/767, twice each, byte-identical, both lists identical to 6f's; replay 289/289;
      execution 28/35, the same seven failures. Both artifacts describe 49ad05fb.

- [x] 6h. Every local `let` through one helper. Found by probing, identical on the previous tip
      and confirmed in main's own source: a recursive or forward-referring function in a do
      block's `let` crashed on a missing binding (the do-let desugared a function's right-hand
      side BEFORE binding its name; even `let go :: Int -> Int; go 0 = 0; go n = ...` in `do`),
      and a comprehension's `let` kept only function bindings, so `let (a, b) = p` crashed and
      its functions could not recurse either. Branch `local-lets-chirho`, stacked on 6g: the do
      `let` and both comprehension `let` paths call `desugar_let_around_chirho` (now taking an
      `FnOnce` body, the honest contract), deleting three divergent copies (desugar_chirho.rs
      5,577 to 5,396 lines). Probes H01-H13 and Q01-Q04 against GHC 9.14.1: all match except
      Q05 (a multi-way `if` pattern guard) and Q06, which now fails for GHC's reason but does not
      stream GHC's partial `[1` first. Controls in eval_letrec_chirho.rs; three mutations, each
      dropping one re-routed `let`, each caught by its own control. Gates on 4100d02f and its
      frozen CLI a70b8eea: unit suites green, driver library 1831/0, integration 117/0, zero
      compiler warnings, no clippy lint on an added line; the announced corpus pair (room
      #26961): 885/938 and 236/767, twice each, byte-identical, both lists identical to 6g's;
      replay 289/289; execution 28/35, the same seven failures; 147 of 152 probes match GHC.
      Both artifacts describe 4100d02f.

- [x] 6i. A multi-way `if` gets real guard arms. Its arms were `cond -> e` only, so a pattern
      guard, a `let` or a second qualifier was rejected (Q05, P20, M02, M03, M07, M10). Branch
      `multiway-if-chirho`, stacked on 6h: `ExprChirho::MultiWayIfChirho { arms_chirho }`
      rather than `case () of _ | arms`, because a guarded right-hand side's arms unify their
      results leniently and TcMultiWayIfFail, rejected today through the strict if-chain,
      would have become a wrong accept. The arm lookahead no longer stops at a top-level `,`
      and reads a qualifier's `let` as the parser ends it. The checker's guard typing moved
      to `infer_chirho/guards_chirho.rs` (one arm routine; lenient arms for a right-hand side,
      strict for a multi-way if); Template Haskell's `MultiIfE` converts instead of becoming
      `__TH_UNSUPPORTED__`. M01-M10 and TcMultiWayIfFail match GHC 9.14.1 (two errors, GHC's
      two). Four controls; three mutations, each caught by its own control. Gates on 0f70ee23
      and its frozen CLI c57e0645: unit suites green, driver library 1835/0, integration
      117/0, zero compiler warnings, no clippy lint on an added line; the announced corpus pair
      (room #26963): 885/938 and 236/767, twice each, byte-identical, both lists identical to
      6h's; replay 289/289; execution 28/35, the same seven failures; 160 of 163 probes match
      GHC. Both artifacts describe 0f70ee23. The last unit stacked before review.

- [ ] 6j. REVIEW BLOCKERS of 6f-6i, found 2026-10-17 by gpt_chirho (#27093, #27096) and
      widened by claude2_chirho (#27103); repaired on branch `guard-repairs-chirho`, source
      only, built and checked by claude2_chirho as the sole builder.
  - [x] `otherwise` was recognised by its SPELLING in `desugar_chirho/guards_chirho.rs`, so
        a parameter, `where`, `let`, pattern guard, alternative or top-level `otherwise`
        took the wrong arm, and a shadow bound to bottom was never evaluated. Wrong on main
        too (O01, O03, O06, O07, O10-O14); 6f, 6g and 6i newly broke O05, O04 and O02/O08.
        Now an ordinary boolean guard; the Prelude's is `True` in the Core prelude.
  - [x] A pattern guard's GADT refinement was gone before its arm's result met the
        signature, so valid programs were rejected (R01-R04, R10, all accepted on main).
        Each arm's result is now decided inside its own scope, against the expected type
        when there is one; an arm that cannot meet it goes to the caller, which judges it as
        it judges an unguarded body. This also rejects, as GHC does, a later arm that cannot
        meet the signature (R11, R12), which main accepted.
  - [x] Controls: `eval_pattern_chirho/otherwise_binding_chirho.rs`,
        `eval_pattern_chirho/gadt_guard_chirho.rs`, and GF02/GF13 in `match_rows_chirho.rs`,
        each expectation measured on GHC 9.14.1.
  - [x] EXPOSED by the `otherwise` repair (#27114, #27116): the LLVM backend compiled each
        lifted closure with its own registry of emitted names, so a program able to fail in
        a function and in its `where` closure defined `haskelujah_error` twice and did not
        link. Latent on every build back to old main (claude2's D8/D9); the repair exposed
        it in llvm_round_trip_comprehensive_demo_chirho, whose `otherwise` arms now keep
        their failure path. The closure now takes the module's registry and hands it back
        (codegen_chirho.rs). Controls: `tests/native_error_helpers_chirho.rs`, every program
        through the interpreter, LLVM and Cranelift: D8, D9, a capturing closure, sibling
        and nested closures, and four error paths each failing with its own message.
  - [ ] claude2_chirho builds the exact candidate, runs these controls and the full matrix,
        and one mutation per unit; then the gates and the corpus pair.
  - Open and pre-existing, NOT repaired here (wrong on main too): whole-pattern GADT
    refinement takes a sibling component's equality as given (gpt_chirho's rigid-sibling
    control is accepted, as on main; it was rejected on 6f-6i only through the defect
    repaired above); an equation's constructor pattern against a concrete type is accepted;
    escape through a later case alternative and an existential escaping a guard (#27103).

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

## Brick 6 as built and measured (2026-09-30, cb1ada98, frozen CLI b29b839b)

- `match_rows_chirho.rs`: rows compiled in source order, joined bottom-up. A row that never falls
  through drops the rows after it; a continuation used once is inlined, and a row whose only
  use of it is its default merges with the next row's case on the same variable, so a plain
  constructor match is one switch and allocates nothing; a continuation used more than once is
  let-bound. Guards fall through by passing the next row to `desugar_guards_chirho`.
- Found on the way and fixed: the parser dropped a leading `!` or `-` from a case alternative's
  pattern (`!_ -> e` forced nothing, `-1 -> e` lost its sign); every case alternative used to
  force its scrutinee, so `case undefined of _ -> 5` failed where GHC prints 5; a guarded
  variable alternative became a second default that a later `_` shadowed; list-comprehension
  generators never matched their pattern.
- Measured: 80 of 82 probes match GHC 9.14.1 (X5, G4 remain); corpus 885/938 and 235/767, twice,
  byte-identical, lists unchanged; replay 288/288; execution 28/35, the same seven failures;
  driver library 1817/0 (14 new controls, four mutations each caught by its own control);
  integration 116/0; unit suites green, parser 360; zero warnings.
- `desugar_chirho.rs`: 7,417 lines at the start of this tasklist, about 5,600 now.
