<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# BUG — the type checker is NON-DETERMINISTIC — **FIXED 2026-07-27 (47723b8c)**

> **STATUS: FIXED.** Root cause was signature schemes quantifying in `HashMap::values()`
> order (`ast_type_to_scheme_chirho`), so which fresh metavariable stood for which
> quantified variable swapped per process — state divergence with an *identical allocation
> count*, which is why eight downstream sorts moved the odds but never reached determinism.
> Fixed by sorting quantified vars by id (first-appearance order, which is also the
> TypeApplications-correct order for implicit quantification); explicit `forall` binders
> keep source order.
>
> **Acceptance:** `T25266.hs` 100 runs → **0 pass / 100 fail**, i.e. stable. `T2688` 12/12,
> `T3155` 0/12, `tcfail088` 0/12 — all stable. `should_compile` re-measured 849/89/938 with
> a **byte-identical** failing set; `should_fail` re-measured 149/618/767, also byte-identical.
> `eval_` 1002 passed / 0 failed.
>
> **Credit:** root cause traced by `claude2_chirho` via a fresh-variable allocation probe,
> after my own eight sorted sites had failed to reach determinism. I had grepped the
> offending line — `var_map_chirho.values()` — in my *first* search of this file hours
> earlier and moved past it. The lane swap that put fresh eyes on the problem is what closed
> it.
>
> The investigation below is kept in full: it records four falsified mechanisms, the probe
> method that worked, and the traps that cost real time. Read it as the record of how the
> bug was found, not as open work.

## Original report

**Found** 2026-07-27 by `claude_chirho` (HASKELUJAH) while re-measuring `should_compile`
after the GHC-91510 slice (`6ad1d7a7`).
**Severity: HIGH.** Same input file, same binary, no flags — the compiler accepts it on
some runs and rejects it on others. Builds are not reproducible, and every published
compatibility percentage carries noise.

## Repro

```
$ for i in $(seq 1 20); do
    ./target/release/haskelujah check ghc-tests-chirho/typecheck-chirho/should_compile/T25266.hs \
      >/dev/null 2>&1 && echo pass || echo fail
  done | sort | uniq -c
   9 pass
  11 fail
```

Roughly a coin flip. Not a timeout — the failing runs return promptly with a real
diagnostic.

## It is a clean binary split, not gradual drift

Six captured runs produced exactly **two** byte-distinct outputs:

| output | size | occurrences | content |
|---|---|---|---|
| A | 785 B | 4 | `error[E0200]: type mismatch: expected 'Void', found '[Int]'` |
| B | 205 B | 2 | clean accept (warning only) |

So the checker reaches one of two stable fixed points, chosen per process.

## Why this was nearly invisible

It only shows up if you measure twice. The `should_compile` corpus was measured once per
sweep, so `T25266.hs` landed on whichever side the coin came up and the number was reported
as exact. This is how the same corpus produced **850/938** in one sweep and **849/938** in
the next with *no crate change in between* (the only commits touching `crates/` between the
two measurements were a `cargo fmt` reflow and the GHC-91510 slice, and the slice is
provably not involved — it emits `E0206`, while this failure is `E0200` from inference).

I initially read the 849 as a regression caused by my own slice. It was not. Re-running the
single file is what exposed it.

## Blast radius measured so far

Re-running all **89** files that failed the second sweep: exactly **1** flipped
(`T25266.hs`); the other 88 fail deterministically. A second full corpus pass was started to
find flaky *passers* as well — the honest current statement of the number is therefore:

> `should_compile` = **88 deterministic failures + 1 file that is a coin flip**, i.e.
> 849–850 / 938, and it is not meaningful to quote a third significant figure.

## Likely cause

Rust's `std::collections::HashMap`/`HashSet` randomize iteration order with a per-process
seed. Any place the type checker iterates a hash collection and the *order* affects
unification, defaulting, or constraint-solving will produce run-to-run differences.
`crates/haskelujah-typing-chirho/src/infer_chirho.rs` alone mentions `HashMap`/`HashSet`
**159** times.

Note this is not the same defect as
[[bug-comprehension-letrec-capture-chirho]] — that one is a deterministic wrong answer at
runtime; this one is an unstable accept/reject decision at compile time. They should not be
conflated.

## PARTIALLY TRACED — 2026-07-27. Real order-dependent sites found and fixed; bug NOT closed.

The earlier claim in this file that the mechanism was "likely" hash-iteration order is now
**confirmed in part**: concrete sites were found where the type checker iterates a hash
collection *while mutating inference state*, and sorting them measurably changed the flip
rate. It did not eliminate it.

### Sites found and sorted

All in `crates/haskelujah-typing-chirho/src/infer_chirho.rs`:

| site | what it does | why order matters |
|---|---|---|
| imported type synonyms | registers synonyms into the context | registration order |
| imported type families | registers families | registration order |
| **imported value schemes** | binds schemes into the env | guarded by `lookup(..).is_none()`, so **which of a placeholder and a real import survives depends on what was bound first** |
| imported record fields | inserts field names | insert order |
| **defaulting hints re-key** | drains a `HashMap` and re-inserts keyed by the *substituted* var | two vars can substitute to the **same** var — last write wins, and "last" was a hash seed |
| **defaulting loop** (`check_deferred_preds_chirho`) | builds the defaulting substitution | the order ambiguous vars are defaulted decides the inferred types |
| **IO-defaulting loop** | builds the IO-defaulting substitution | same hazard |

The three bolded ones are genuine last-write-wins races, not merely cosmetic ordering.

### Measured effect — all three builds measured

| build | `T25266.hs` pass rate | deterministic? |
|---|---|---|
| before any fix | 9 / 20 (~45%) | **no** |
| after the first five sorts | 72 / 100 | **no** |
| after all seven sorts | 58 / 100 | **no** |

**Read this table for the right thing.** The pass *rate* is not the metric — determinism is.
A fixed compiler scores 0/100 or 100/100. All three builds are non-deterministic, so **none
of the seven sorts fixed the defect.**

The final two sorts (the defaulting loops) moved the rate 72 → 58, i.e. *toward* a coin
flip. That is not a regression in correctness — sorting strictly removes an order-dependence
— but it does show those two changed which outcome is more likely **without removing the
randomness**. So the dominant order-dependent site is still unfound and is **not** among
these seven.

Corpus safety of the seven sorts was verified separately: `should_compile` re-measured at
842/930 against 841/930 before, with a **byte-identical failing set** apart from `T25266`
itself flipping to pass. Zero regressions — and that one-file delta **is** the coin flip,
not an improvement. Do not quote it as progress.

### LOCALIZED BY INSTRUMENTATION — the divergence is in CONSTRAINT COLLECTION

Rather than guess at an eighth site, I added a temporary env-gated probe
(`HASKELUJAH_DETERMINISM_PROBE_CHIRHO`) dumping the **input** to defaulting — the sorted
ambiguous variables with their class sets — and ran the file 12 times. The probe has since
been removed; this is what it showed.

**Twelve runs produced FOUR distinct defaulting inputs**, and the shapes differ in size, not
merely in order:

```
 3 runs   vars=5  preds=8
 3 runs   vars=6  preds=11
 6 runs   vars=7  preds=11
```

That is decisive about *where* the bug is not. The number of collected predicates itself
changes between runs — 8 versus 11 — so by the time defaulting is reached the runs have
**already diverged**. Every site fixed so far (seeding, defaulting, IO-defaulting,
binding-group adjacency) is at or downstream of that point, which is exactly why sorting
them shifted the odds without ever reaching 0/100 or 100/100.

**The remaining work is upstream: constraint generation/resolution.** Something there
iterates a hash collection, or depends on one, in a way that changes *which constraints
exist*, not merely the order they are processed in.

Note the binding-group adjacency fix (`refs_chirho`, a `HashSet`, feeding `adj_chirho`) is a
genuine order-dependence and is kept — dependency-edge order decides binding-group traversal
and therefore type-variable allocation. But it did **not** change this file's probe
distribution at all (the same four hashes, same 3/3/6 split), so it is not the culprit here.

### SECOND PROBE — run, and it names the actual fault: METAVARIABLES SOMETIMES UNIFY

I then probed the constraint set **as generated**, before any resolution
(`HASKELUJAH_PREDPROBE_CHIRHO`, since removed), and ran the file 14 times.

**Fourteen runs produced ELEVEN distinct constraint sets**, in two sizes — `n=11` (11 runs)
and `n=8` (3 runs). Diffing an `n=8` run against an `n=11` run shows exactly what differs:

```
n=8   CanRunDB::v374      Monad::v374      MonadUnliftIO::v374     <- ONE variable
n=11  CanRunDB::v366      Monad::v364      MonadUnliftIO::v366
      CanRunDB::v375      Monad::v375      MonadUnliftIO::v375     <- THREE variables
```

On some runs three metavariables **collapse into one**; on others they stay distinct. That
is not renumbering — it is a different *number of type variables*, and therefore a different
constraint set handed to solving. `n=8` and `n=11` are genuinely different inference states.

So the fault is: **whether certain metavariables get unified depends on the order inference
visits things**, and that order varies per process. Everything downstream — defaulting,
resolution, the final accept/reject — is inheriting an already-divergent state.

### Also ruled out by this probe

The obvious candidates for "what sets visit order" are all deterministic:

- `fun_names_chirho` / `fun_matches_refs_chirho` are `Vec`s built by iterating
  `module_chirho.decls_chirho` (a `Vec`) — deterministic input order.
- `binding_groups_chirho` is Tarjan SCC over `Vec`s and `vec![bool]`, and its adjacency
  edges are now sorted — deterministic.
- Class method and default iteration walks AST `Vec`s, not the `HashMap`s it builds.

So the varying allocation order is being introduced somewhere else again. The next probe
should not be another dump — it should **trace fresh type-variable allocation itself**
(log every `fresh_var` with a short backtrace or a phase tag) and diff an `n=8` run against
an `n=11` run. The first allocation that differs is where the order diverges, and that is
the last hop to the root cause.

### Gate status for the eight sorts (completed after db4239e8 was committed)

`db4239e8` was committed while the broad driver run was still going, and that run was later
killed by the harness before reporting. The gate was therefore completed with targeted
suites instead, all green:

| gate | result |
|---|---|
| `haskelujah-typing --lib` | **255 passed / 0 failed / 0 warnings** |
| driver `eval_` (evaluation semantics) | **1002 passed / 0 failed** (199s) |
| driver `extensions_chirho` | **207 passed / 0 failed** (12s) |
| `should_compile` corpus, 938 files | **850 / 88**, failing set byte-identical to the committed artifact |

The `eval_` suite is the one that matters most for these changes: sorting alters the order
declarations are inferred and variables are defaulted, so if it changed program *meaning*
that is where it would show. It did not.

The broad `--skip proptest_chirho --skip project_chirho` run reached 1651 passed / 0 failed
before being killed; it was never observed failing.

### Ruled out by inspection (do not re-search these)

- `free_vars_chirho` already returns a **sorted** `Vec`.
- `ClassEnvChirho::instances_chirho` stores a `Vec` per class — deterministic within a class.
- The driver's `exporters_by_type_name_chirho` accumulates into a **set** and keeps only
  single-exporter entries, so `.next()` on a 1-element set is deterministic.
- The driver's method-occurrence map already sorts each id vector (`lib.rs:3172`).
- `iface_chirho.rs` has no hash iteration at all.
- Remaining loops in `kind/class/deriving/exhaust` iterate `Vec`s.

### Operational note for whoever runs the gate

The driver suite's `proptest_chirho` eval tests each take **over 60 seconds** — the runner
prints "has been running for over 60 seconds" for ~19 of them and they *do* eventually pass.
A full `-p haskelujah-driver --lib` run therefore takes hours and can look hung when it is
merely slow. Do not kill it on that basis (I did, and lost a nearly-complete run). Use
`-- --skip proptest_chirho` for iteration and run the proptests separately.

Also: **never pipe a background `cargo test` through `tail`** — nothing is written until the
process exits, so a live run is indistinguishable from a dead one. Two "silent deaths" in
this session were this, not OOM.

## ATTEMPTED FIX — 2026-07-27, reverted. Read this before trying again.

I implemented the deterministic-collections fix and **backed it out**. The tree is green;
the work is preserved at `spec-chirho/wip-chirho/det-hash-experiment-chirho.patch`
(913 lines, not applied).

### What worked

A shim (`det_hash_chirho.rs`) pinning the hasher to a fixed seed:

```rust
pub type DetBuildHasherChirho = BuildHasherDefault<DefaultHasher>;
pub type HashMapChirho<K, V> = std::collections::HashMap<K, V, DetBuildHasherChirho>;
pub trait NewDeterministicChirho { fn new() -> Self; }   // see below
```

Two tricks made it nearly free at the call sites:

1. **The `new()` trait.** `HashMap::new()` is inherent only on the `RandomState`
   specialisation, so with a custom hasher an in-scope trait method of the same name
   resolves instead. That kept **~240 existing `HashMap::new()` call sites compiling
   unchanged** instead of needing `::default()` everywhere.
2. **A free `map_from_chirho([..])`** replacing `HashMap::from([..])` (33 sites), which is
   also `RandomState`-only. A free function makes it a pure textual substitution with no
   paren surgery.

Result in the typing crate: **builds clean, 257 tests pass, 0 failures, 0 warnings**
(255 before, +2 new tests asserting iteration order is stable across independently built
maps). Only 7 import sites needed editing.

### Why it was reverted

The deterministic type **leaks through public API signatures**. `infer_module_with_imports_…`
takes `&HashMap<String, SchemeChirho>`; once that alias is deterministic, every caller must
match. Retyping the driver produced a ragged boundary — some maps deterministic, some still
`std` — and the error count went **2 → 4 → 35 and climbing**, reaching types produced by the
naming crate.

That makes this a **deliberate workspace-wide refactor, not a patch**: the alias has to be
adopted at the crate boundary (or workspace-wide) in one intentional pass, with the full
driver suite as the gate. Landing it half-finished in a shared tree at the end of a long
session would have left the driver non-compiling for other agents.

### What the next attempt should do differently

- Decide the boundary **first**: either (a) adopt `HashMapChirho` workspace-wide, or
  (b) keep public signatures on `std::collections::HashMap` and use deterministic maps
  strictly for internal, order-sensitive structures. (b) is smaller but only helps if the
  order-sensitive iteration is genuinely internal — which is **not yet established**.
- **Confirm the cause before paying for the refactor.** The typing-crate-only build was
  never measured against `T25266.hs`, because the release binary could not link until the
  driver compiled. So this experiment has **not** yet demonstrated that fixed-seed hashing
  actually fixes the flip. Do that first, on a branch, before touching crate boundaries.

That last point is the honest status: the fix is *plausible and well-scoped*, and it is
still **unverified**.

## How to confirm and fix

1. **Confirm cheaply**: swap the hash collections in the typing crate for `BTreeMap`/
   `BTreeSet`, or keep `HashMap` but pin a fixed-seed `BuildHasher`, then re-run
   `T25266.hs` 20×. If it becomes 20/20 either way, iteration order is confirmed as the
   channel.
2. **Fix properly**: a compiler's observable behavior must not depend on hash seed. Either
   use ordered collections on any path that feeds diagnostics or solver order, or sort at
   every point where a hash collection is iterated into an order-sensitive consumer.
   `captures_chirho.sort_by_key(...)` in `stg_lower_chirho.rs` shows the codebase already
   knows this pattern in places — it just is not applied consistently.
3. **Guard it**: add a regression test that checks a known-tricky module N times and asserts
   identical output every time. Without such a test this class of bug silently returns.

## Consequence for how we publish numbers

Any single-pass corpus measurement is now known to be ±1 at minimum. Measurement artifacts
should either report a range, or run each file until the result is stable. Quoting
`90.6%` as if it were exact overstates the precision of the method.
