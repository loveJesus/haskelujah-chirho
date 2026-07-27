<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# BUG — comprehension letrec worker mis-resolves free variables when it captures a pattern-bound var

**Found** 2026-07-27 by `claude_chirho` (HASKELUJAH) while authoring a demo program for the site.
**Severity: HIGH — silent wrong answers.** No error, no warning; the program returns a
plausible-looking but incorrect list. Discovered because the canonical lazy primes sieve —
arguably the most famous Haskell program in existence — mis-compiles.

**Binary under test:** `target/release/haskelujah`, mtime 2026-07-26 10:56:52, which is
newer than the last `crates/**` commit (80b6e32b, 2026-07-26 06:14). So this reflects
committed HEAD, not a stale build.

## Minimal repro — one line apart

```haskell
-- BROKEN: take 8 (sieve [2..]) => [2,3,4,5,6,7,8,9]   (expected [2,3,5,7,11,13,17,19])
sieve (p:xs) = p : sieve [x | x <- xs, x `mod` p /= 0]

-- CORRECT: take 8 (sieve [2..]) => [2,3,5,7,11,13,17,19]
sieve (p:xs) = p : sieve (filter (\x -> x `mod` p /= 0) xs)
```

The broken form silently degenerates to the identity filter — every guard reads as `True`.

## What it is NOT (each ruled out by experiment)

| Ruled out | Evidence |
|---|---|
| `mod` / `rem` / `div` primop | `head xs \`mod\` p` under the same recursion is **correct** (`[1,1,1,1,1]`) |
| Integral dictionary threading | a top-level **monomorphic** `notDiv :: Int -> Int -> Bool` in the guard fails **identically** |
| Unforced thunk on `p` | `p \`seq\`` before the recursive call does **not** fix it |
| List comprehensions generally | same comprehension, same guard, **non**-recursive → correct |
| Recursion generally | recursive + `x + p > 1000` → correct (`[1,1000,1001,1002]`) |
| Laziness / infinite lists | comprehension over `[1..]` non-recursively → correct |
| Capture of `p` being wrong | `p > 999` guard correctly yields empty; `p` holds the right value |

## What it IS — discriminator matrix

Every row below was executed, not reasoned about. A letrec-bound comprehension worker under
recursion is necessary but **not sufficient** — row CA proves that.

| # | Guard | Captures `p`? | Result |
|---|---|---|---|
| W | `x + p > 1000` (pure primops) | yes | **correct** — `[1,1000,1001,1002]` |
| CA | `even (x + p)` | yes | **correct** — `[1,3,5,7]` |
| P | `x \`mod\` 2 /= 0` | no | **correct** — `[3,5,7,9]` |
| BC | `even x` | no | **correct** — `[1,2,4,6]` |
| CB | `notDiv x 2` (user fn, no capture) | no | **correct** — `[1,3,5,7]` |
| BB | `alwaysFalse x 1`, no recursion | n/a | **correct** — `[]` |
| AC | `notDiv x p` (user fn, defined before) | yes | **BROKEN** — identity |
| CC | `notDiv x p` (user fn, defined after) | yes | **BROKEN** — identity |
| BA | `alwaysFalse x p` | yes | **BROKEN** — `cannot apply arguments to literal 0#` |
| F | `x \`mod\` p /= 0` | yes | **BROKEN** — identity |
| Q | `x \`rem\` p /= 0` | yes | **BROKEN** — identity |
| S | `not (x \`mod\` p == 0)` | yes | **BROKEN** — identity |
| AA | `(x \`div\` p) * p /= x` | yes | **BROKEN** — identity |
| AD | fully `:: Int`-annotated `mod` guard | yes | **BROKEN** — identity |

BA is the smoking gun: the *function slot itself* resolved to a literal `0#`, so the
application head was not the closure it should have been. A literal `0` is also exactly what
the placeholder closures carry during lowering (`CodePtrChirho(0)`, "will be patched") —
worth checking whether some placeholder never gets patched.

**The exact discriminator is NOT yet pinned, and I will not pretend otherwise.**
An earlier draft of this file claimed the rule was "guard performs an environment lookup
while capturing `p`". **Row CA falsifies that** — `even (x + p)` is a non-primop call that
captures `p` and is correct. Declaration order is also ruled out: AC and CC differ only in
where `notDiv` is defined, and both fail.

### Three syntactic rules proposed and each falsified by experiment

I tried to find a clean surface rule. There isn't one. Recording the failures so nobody
re-derives them:

1. ~~"guard performs an environment lookup while capturing `p`"~~ — falsified by CA
   (`even (x + p)`: non-primop call, captures `p`, **correct**).
2. ~~"`p` passed directly as an argument to a non-primop call"~~ — falsified by DA
   (`notDiv x (p + 0)`, `p` wrapped in a primop, still **broken**), and by DB
   (`let q = p in notDiv x q`) and DC (`p` routed through a `where` helper), both broken.
3. ~~"user-defined functions break, Prelude functions are fine"~~ — falsified by FA
   (`gcd x p == 1`, Prelude, **broken**) and FB (`odd (x + p)`, Prelude, **broken**).

### NARROWED FURTHER 2026-07-27 — the guard VALUE is correct; the DISPATCH is wrong

Decisive new evidence. The guard expression computes the right `Bool` in every form —
including inside a comprehension, and inside a recursive function:

```haskell
s (p:xs) = (head xs `mod` p /= 0) : s xs   -- take 5 -> [True,True,True,True,True]   correct
s (p:xs) = (p `mod` 2 /= 0)       : s xs   -- take 6 -> [T,F,T,F,T,F]                correct
print (take 5 [x `mod` 2 /= 0 | x <- [1..]])       -- [T,F,T,F,T]                    correct
```

So the `Bool` is computed correctly and then **dispatched on incorrectly** inside the
capturing comprehension letrec worker. This eliminates the entire "the guard is
mis-evaluated / the wrong instance is selected / `p` is misbound" family of hypotheses at
once — the value arriving at the `case` is right.

Also checked and NOT the fault: the evaluator's `CodeChirho::CaseChirho` arm does force a
heap-pointer scrutinee (`emit_enter_chirho`) and saves/restores arg registers around the
case frame (`eval_chirho.rs:628`). And the two fixes landed on 2026-07-27 — the
determinism quantification-order fix (47723b8c) and the `TyApp` annotation fix (2a1a95f7) —
do **not** repair it: `take 8 (sieve [2..])` still yields `[2,3,4,5,6,7,8,9]`.

A live hypothesis was also **weakened**: the catch-all for an unresolved class method in
`instance_chirho.rs` is `"+#"` (addition), which would make a mis-resolved `/=` return a
number that a `case`-on-`Bool` could misread as `True`. But annotating the guard operand
(`((x \`mod\` p) :: Int) /= 0`) does **not** fix the sieve, so that path is not confirmed.
Recorded as weakened rather than quietly dropped.

**Where a fresh investigator should start:** the value is right and the `case` is wrong, so
compare the *runtime* dispatch of the working `filter` form against the broken comprehension
form — instrument the tag actually read at the `case` in each. Do not re-derive the value
path; it is proven correct above.

### CHARACTERIZED — the minimal pair (supersedes the guesswork below)

The trigger is **`not` / `/=`**, i.e. a guard whose scrutinee is a call returning a
*heap-allocated* `Bool` rather than a primop's immediate result.

```haskell
-- CORRECT — [1,3,5,7]
s (p:xs) = p : s [x | x <- xs,      (x + p) `rem` 2 == 0  ]
-- BROKEN — [1,2,3,4] (identity: guard always True)
s (p:xs) = p : s [x | x <- xs, not ((x + p) `rem` 2 == 0) ]
-- BROKEN — [1,2,3,4]
s (p:xs) = p : s [x | x <- xs,      (x + p) `rem` 2 /= 0  ]
```

Identical arithmetic. The only delta is the `not` wrapper. Core confirms it — one token:

```
JA (correct)  case ((v1056 …) …)        of { True -> keep ; False -> drop }
JB (broken)   case (v11 ((v1057 …) …))  of { True -> keep ; False -> drop }
                    ^^^ v11 = not
```

`not` is not itself broken: `map not [True,False]` → `[False,True]`.

**Statement of the fault.** Inside a list-comprehension `letrec` worker that captures a
variable bound by the enclosing function's constructor pattern, a `case` whose scrutinee is
a saturated call returning a boxed `Bool` dispatches to the **first alternative
unconditionally** (`True`), instead of forcing the closure and reading its constructor tag.
`==`, `>`, `+` survive because they lower to primops that yield an immediate value needing
no entry. This is the same family as the `no matching alternative for tag 0` crash in row N
and the `cannot apply arguments to literal 0#` crash in row BA — a closure being tag-read
without being entered.

Capture is still required: `x \`mod\` 2 /= 0` (row P) and `notDiv x 2` (row CB) both use
`/=` under recursion **without** capturing `p`, and both are correct.

This retro-explains every broken row: `notDiv`/`nonZero` are defined with `/=`; `odd` is
`not . even`; rows F/Q/S/AA/AD all route through `/=` or `not`.

### Superseded — how I got here (kept so nobody re-derives it)

`even (x + p)` over `[1..]` filters **correctly** (`[1,3,5,7]`; identity would be
`[1,2,3,4]`). `odd (x + p)` over `[2..]` — same shape, same "keep the odd ones" semantics,
same capture — degenerates to **identity**. Likewise `nonZero (x \`mod\` p)` with a
one-argument user function is broken, while `even` with one argument is not.

Two programs that are structurally equivalent behave differently. That rules out every
syntactic rule and points instead at something **sensitive to Core id assignment** —
which shifts with the number of preceding bindings, which Prelude pieces get pulled in,
and the shape of the module.

This fits the one hard runtime datum: `notDiv` was **CoreId 0** when it resolved to a
literal `0#`. In `lower_expr_chirho`, `VarChirho` consults `arg_param_indices_chirho`
*first* (`stg_lower_chirho.rs:227`) and only then `env_chirho`; a stale or colliding entry
in that flat id→slot map would silently turn a top-level function reference into an
arg-register read. `arg_param_indices_chirho` is mutated globally with bare
`insert`/`remove` rather than being scoped, and the letrec arm computes its next free
register as `max(values()) + 1` over the whole map (`:518-524`) — so entries leaking
across scopes are plausible.

**Next step for whoever fixes this: instrument, do not theorise.** Dump, for the failing
module, what `arg_param_indices_chirho` and `env_chirho` contain at the moment the guard's
head variable is lowered, and compare against the working `even (x + p)` module. That
single diff should settle in minutes what black-box probing could not settle in dozens of
runs.

## Core evidence — the two shapes

Both capture `p` (`v4` / `v12`) inside the letrec; the Core is structurally identical.
Only the guard's *evaluation mechanism* differs.

```
W  (CORRECT)  : p xs -> : v4 (v0 letrec { $lc_go_6 = \$xs -> case v7 of
                  : x $rest -> case (># ((v1081 v9) v4) ((v132 v2879) 1000)) of ...
AC (BROKEN)   : p xs -> : v12 (v1 letrec { $lc_go_14 = \$xs -> case v15 of
                  : x $rest -> case ((v0 v17) v12) of ...
```

`W`'s scrutinee is an inline primop result. `AC`'s scrutinee is `((v0 v17) v12)` — an
application whose head `v0` must be looked up in the closure environment, and that lookup
returns the wrong slot.

For contrast, the working `filter` form has **no letrec at all** — the predicate is a plain
lambda argument, and the identical guard expression works there:

```
G (CORRECT)   : p xs -> : v4 (v0 ((v6 \x -> (v12 (((v711 v3014) (mod# v7 v4)) ...))) v5))
```

## Hypothesis (stated as hypothesis, not established cause)

When the comprehension is desugared into a `letrec`-bound worker, capturing an enclosing
**constructor-pattern-bound** variable shifts the closure's free-variable environment
layout, and the indices for the *other* free references (top-level functions, class
selectors) are not adjusted to match. Primop-only guards never index the environment, which
is exactly why they survive.

Confirming this needs the STG closure-construction / free-variable numbering path, not more
black-box probing.

## Why this matters beyond the sieve

Any lazy local worker with a non-primop guard that closes over a pattern-bound variable is
affected. That is an extremely common Haskell shape. The failure is silent, so it will not
show up as a red test — it shows up as a wrong answer. This is also a reminder that
`should_compile` percentages measure acceptance, not correctness: this program *compiles*.

## Code-level finding — a real state-corruption defect (causal link NOT yet proven)

Reading `crates/haskelujah-driver-chirho/src/stg_lower_chirho.rs`, the closure-capture code
saves and restores lowering state around each closure body. The save/restore is
**asymmetric**: it removes entries from *two* maps but restores only *one*.

Both the `LamChirho` arm and the recursive-`LetChirho` arm do this:

```rust
for (slot_chirho, &(id_chirho, _)) in captures_chirho.iter().enumerate() {
    self.arg_param_indices_chirho.insert(id_chirho, slot_chirho);
    self.env_chirho.remove(&id_chirho);          // <-- removed here
}
```
— `stg_lower_chirho.rs:389-392` (Lam) and `:585-589` (letrec)

but the matching restore only ever puts back `arg_param_indices_chirho`:

```rust
for (id_chirho, prev_chirho) in saved_captures_chirho {
    match prev_chirho {
        Some(idx_chirho) => { self.arg_param_indices_chirho.insert(id_chirho, idx_chirho); }
        None            => { self.arg_param_indices_chirho.remove(&id_chirho); }
    }
}
```
— `stg_lower_chirho.rs:616-625` (and `:419-428`)

`saved_captures_chirho` is typed `Vec<(CoreIdChirho, Option<usize>)>` — it can only ever
hold arg-register indices, so **the `env_chirho` deletion is permanent for the rest of the
module lowering.** Any later reference to that id resolves without its heap-pointer binding.

Why this is plausibly load-bearing here: letrec binders *are* inserted into `env_chirho` as
placeholder heap pointers (`:495-498`) and *also* later given arg registers, so an id can
legitimately live in both maps — which is precisely the situation the asymmetric restore
corrupts.

**Stated honestly: the asymmetry is definitely a defect. I have NOT proven it is the cause
of the sieve miscompile.** Proving it needs a rebuild + differential test, which was blocked
behind a long-running gate suite when this was written. Whoever picks it up should first
write a failing test from the matrix above, then try making the restore symmetric
(save `env_chirho` alongside, restore both), and re-run — not assume.

## Mechanism — where the boxed-Bool scrutinee goes wrong

`stg_lower_chirho.rs:1110` lowers a `case` scrutinee via `lower_arg_source_chirho`. For a
complex (non-var, non-literal) scrutinee inside a function body, `:1212-1239` builds a
runtime thunk:

```rust
let max_idx_chirho = self.arg_param_indices_chirho.values().copied().max().unwrap_or(0);
let captures_chirho: Vec<ArgSourceChirho> =
    (0..=max_idx_chirho).map(ArgSourceChirho::ArgRegChirho).collect();
ArgSourceChirho::ThunkCodeChirho { code_ptr_chirho: entry_chirho, captures_chirho }
```

The thunk's code refers to captured values **by arg-register index**, and the capture list is
`0..=max` over `arg_param_indices_chirho`.

### CORRECTION — my first reading of this was wrong

An earlier draft of this file — and messages I sent to the fleet — called
`arg_param_indices_chirho` a "global, flat, **unscoped** map" and asserted the capture
window was therefore a bad guess. **That is not accurate, and I am retracting it.**
(Checked: the claim never reached a commit message, so nothing else needs amending.)

Reading the register model properly:

- Case alts **prepend** constructor fields: every existing index is shifted up by
  `num_binders`, and the alt's own binders take `0..num_binders` (`:1066-1077`).
- The map **is** saved and restored around each alt (`:1049`, `:1082`), around lambda
  bodies (`:367-428`), and around letrec RHS bodies (`:562-625`).

So the map *is* scoped, and `max(values()) + 1` really does equal the current frame size.
Walking the worked example — worker captures `p`→0, param `$xs`→1; entering the cons alt
shifts to `x`→0, `$rest`→1, `p`→2, `$xs`→3 — the scrutinee thunk captures `ArgReg 0..=3`,
which is exactly right. I could not make the window come out wrong on paper.

**Therefore the mechanism is NOT established.** The symptom is pinned precisely (see the
minimal pair above — that part is solid, reproduced many times). The cause is not. I am
recording this rather than shipping a fix built on a mechanism I had already talked myself
out of.

The one thing in this area that *is* a definite defect, independent of the above: the
save/restore around closure bodies is asymmetric. `env_chirho.remove()` at `:391` and
`:588` is never undone, because the matching restores at `:419` and `:616` carry only
`Option<usize>` arg-register indices, not env values. Alt binders *are* restored properly
(`:1083-1090`), which shows the intended pattern — it just is not applied on the capture
path. Whether that is load-bearing for this bug is unknown.

**Next step is instrumentation, not another hypothesis.** Dump the actual capture list, the
frame contents, and the resolved head value at the moment the guard's scrutinee thunk is
entered, for `JB` (broken) beside `JA` (correct). Those two differ by one token, so the
first divergence in that trace is the bug. I have now proposed four mechanisms and
falsified all four from the armchair; the fifth should come from a trace.

Related but distinct defect, still worth fixing: the save/restore around closure bodies is
asymmetric — `env_chirho.remove()` at `:391` and `:588` is never undone, because the
matching restores at `:419` and `:616` only carry `Option<usize>` arg-register indices.

**Do not "fix" this by forcing harder at the use site.** The capture window is the problem;
the thunk must capture the frame it was built in, not a range guessed from a global map.

## Repro files

`Refraction.hs`, `F/G/W/AC/BA/BB/BC.hs` were written to the session scratchpad. Anyone
picking this up can regenerate them from the tables above — each is 3–6 lines.
