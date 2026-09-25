<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Execution measurement

Each section below is ONE measurement, newest first. A section states the source it
actually ran on and is never edited to describe a later source: every section other than
the first is HISTORICAL, and stays labelled that way.

## 2026-09-24 — the literal carrier (current)

Corpus source and execution source are the same commit:
`e8cd6bf4d68b46448d21667e2cf4c15000e67e1d` on branch `provenance-literals-chirho`, pushed
as `gh_chirho/provenance-literals-3-chirho`. Frozen DEBUG CLI
`fd0da8094f4086db49148e4ddfa31658e08a371bfa16acbdc0ca41f53a28db4a`, digest asserted
unchanged before and after the four corpus passes and again before and after the
execution set.

Receipts live in the measuring worktree under `tmp-chirho/literal-corpus-chirho/` and
`tmp-chirho/literal-landing-chirho/`. Those paths are relative to the REPOSITORY ROOT of
that worktree. Earlier artifacts wrote `../haskelujah-claude-chirho/tmp-chirho/...`, which
resolves only from a sibling lane directory and not from a repository root at all; that
locator is corrected here and in both measurement artifacts.

### Results at their actual scope

| Measurement | Source | Result | What it establishes |
| --- | --- | --- | --- |
| Corpus, accept axis | e8cd6bf4 | 885 of 938, twice, byte-identical | Typecheck verdicts only; list SHA identical to the committed artifact |
| Corpus, reject axis | e8cd6bf4 | 235 of 767, twice, byte-identical | Same, for the wrong-accept list |
| Rejection replay | e8cd6bf4 | 288 of 288 exit 1 with `error[E`, zero panic headers | Every rejection is a diagnostic, not a crash |
| Execution set | e8cd6bf4 | 28 of 35 predicates match GHC 9.14.1 byte for byte | Interpreter and native, 14 programs |
| Same set on main's CLI | e246670e | the same 28 of 35, predicate by predicate | So the seven failures are pre-existing, not caused by this change |
| Unit suites | 8730d067 | ast 13, core 129, naming 136, parser 349, th 15, typing 349 | 8730d067 differs from e8cd6bf4 in no crate line at all |
| Driver library | 8730d067 | 1797 passed / 0 failed, 592.73s | |
| Driver integration | 8730d067 | 85 passed across 9 targets, zero failed | canaries, curated, dictionary evidence, instance obligations, method occurrences, native laziness, numeric hierarchy, show precedence, typing integration |
| Verdict-rule controls | 059f6b6d | 20 of 20, plus 9 ordering controls | Tools only; the corpus above ran on the runner as committed BEFORE this repair |

The unit and integration suites ran on `8730d067`. It differs from `e8cd6bf4` in no crate
line whatsoever: the only files that differ are two `spec-chirho` documents, verified
mechanically. The earlier execution set on this branch's CLI `6e34983c` is retained as an
earlier receipt; that CLI and `fd0da809` differ in no non-test crate line, the only crate
files differing between `a091b2d0` and `e8cd6bf4` being two compiled solely under
`cfg(test)`. Their digests differ because the build is not bit-reproducible, and no
byte-identity between the two binaries is claimed.

### What the literal carrier does and does not reach

Transport now covers literals in expressions AND in patterns, stamped where lowering
builds them, and a literal with an origin is published under that origin only. Three
limits are explicit (gpt_chirho #24761 corrected all three against the code):

- a float literal never becomes `fromRational` — the desugarer converts only an integer
  literal, and a string literal under OverloadedStrings — so the `FractionalLiteral` role
  is capture and carrier only, not an end-to-end path;
- literal evidence is one `{class, type-key}` per origin, neither role-keyed nor
  multi-predicate, so it cannot represent a pattern literal's distinct `Num` and `Eq`;
- a pattern literal carries an origin but the checker captures nothing for it, so it
  publishes no evidence: a producer gap, not a transport gap.

The two placeholder controls test the two ENDS separately, one at capture in the checker
and one at the join in the driver. They are not one integrated producer-to-consumer
execution test.

### The seven execution failures, all pre-existing

Confirmed on this branch's CLI and on main's own CLI, identical predicate by predicate.

FIVE OF THE SEVEN ARE SILENT WRONG ANSWERS: the program exits 0 and prints something
plausible that is not what GHC prints. Only two announce themselves, one runtime error and
one compile failure. The silent five are the dangerous category and are counted here
explicitly rather than left to be tallied from the table, because a silent wrong answer
survives a green gate.

| Program | Engine | Ours | GHC 9.14.1 |
| --- | --- | --- | --- |
| DerivedEqOrd | interpreter | runtime error, `primop LtIntChirho: expected Int#, got (@N, 1#)` | `True/GT/BoxChirho 3/True` |
| DerivedEqOrd | native | `False/GT/ArcChirho/True` — wrong in the first and third fields, silently | `True/GT/BoxChirho 3/True` |
| DerivedMix | interpreter | `RecordChirho 1 True 2.5 120 "s"` — a `toEnum`-built Char prints as its code point, silently | `RecordChirho 1 True 2.5 'x' "s"` |
| GeneratedLiterals | interpreter and native | `NodeChirho LeafChirho 1 NodeChirho LeafChirho -2 LeafChirho 3` — derived `Show` drops every parenthesis, silently | `NodeChirho (LeafChirho 1) (NodeChirho (LeafChirho (-2)) (LeafChirho 3))` |
| RecursiveDo | native | compile fails, `invalid LLVM IR input: Instruction does not dominate all uses!` | `[1,2,4,8,16]` |
| Sections | native | `map (subtract 1)` prints a heap pointer, silently | `[0,1,2]` |

The derived-`Show` row was LOCATED in this window rather than introduced: deriving
generates only a `show`, built by string concatenation, with no `showsPrec` and no
precedence anywhere. That one cause explains all of it — nothing derived is ever
parenthesised, a negative field prints bare, `showsPrec` on a derived type is a missing
STG binding, and a derived value nested inside a Prelude constructor loses its parens too
because the dictionary layer refuses a show-only row at precedence 11. It is queued as its
own Show class-model brick, and that refusal must NOT be weakened to make it pass.

## 2026-09-20 — occurrence provenance for references (historical)

Corpus source: `11b24799dbdd78fcd0a319de14b08a331c44db2b`. Execution-gate source:
`6ed46217fd58a5c2e07e8775b32b38ac891ed4a7`, which differs from the corpus source only in
the comments of one file (verified mechanically: every changed crate line is a comment).
Measured by HASKELUJAH/claude_chirho on macOS arm64 in the isolated lane worktree.
Evidence lives in that worktree under `tmp-chirho/landing-chirho/`, not in `/private/tmp`:
a reboot on 2026-09-19 cleared `/private/tmp` and destroyed every receipt published that
day. The committed tests survived, which is the argument for putting controls there.

## Results at their actual scope

| Measurement | Source | Result | What it establishes |
| --- | --- | --- | --- |
| Curated GHC suite | 6ed46217 | passes, 486.68s | Its committed manifest declares 514 GHC 9.14.1 execution oracles |
| Driver library | 6ed46217 | 1778 passed; zero failed, ignored or filtered | Includes the native round trips and the five evidence-join controls |
| Driver integration | 6ed46217 | 202 passed across 19 targets, zero failed | |
| Evidence-join boundaries | 6ed46217 | 5 direct controls, 2 mutation-checked | Reached by no source program, so exercised at the join itself |
| Upstream should_compile | 11b24799 | 885 of 938 | Typecheck acceptance only; two byte-identical complete passes |
| Upstream should_fail | 11b24799 | 235 of 767 | Typecheck rejection only; two byte-identical complete passes |
| Replay of every rejection | 11b24799 | 288 of 288 credited | Exit 1 with `error[E`, no panic header, timeout or abnormal exit; every output kept |

The accept axis did not move: its failure list is identical file for file to the previous
artifact's, same SHA-256. The reject axis rises 228 -> 235 with no loss, all seven of one
shape. Neither percentage measures execution correctness, and neither corpus fully passes.

## Reproduction and provenance

```sh
tmp-chirho/landing-chirho/axis_pass_chirho.sh <frozen-cli> <corpus-dir> <out>
```

Four workers, 15 seconds per file, every timeout re-run alone at 60s; none occurred. The
runner records each file's EXIT CODE beside its verdict, so "no accepted file exited
nonzero" is a property of the pass file itself rather than of a separate audit. The frozen
CLI is `3f978db699e6c8552c764767b43e657d68cb44a37e585a3d89faf9e72013a042`, hashed before
and after all four passes and unchanged.

## What this measurement cannot see

Both axes run `haskelujah check`. Neither ever RUNS a program, so neither can observe the
defect this lane's second repair fixed: a class method's evidence joined by position rather
than by source span, which made `instance Show W where show (W n) = "W" ++ show n` print
`W3` or `WChirho 3` depending only on which declaration came first, at exit 0 with no
diagnostic. The driver suites and the curated oracles are the instruments for that class of
defect, and an instance WITH a context is still wrong at run time
(`instance Show a => Show (T a)` prints a derived rendering).

## Instance-context obligations and the evidence join (11b24799)

- **An instance whose own context cannot hold satisfies nothing.** A ground wanted was
  treated as solvable the moment an instance HEAD matched, so `instance Convert a String =>
  Render [a]` served `Render [Int]` with no `Convert Int String` anywhere. The
  certainly-unsolvable test now follows each candidate's sub-goals, stays conservative
  wherever it cannot prove the answer, and names the missing sub-goal as GHC does. Seven
  reject gains, zero accept losses.
- **A method's evidence belongs to its source reference.** The desugarer numbers
  occurrences in declaration order, the checker in inference-visit order with instance
  bodies last, and the join matched them by position under a count guard. Now it matches by
  SPAN. What remains positional serves only references nothing can identify: the desugarer
  mints occurrences at several sites and only the variable-reference site records a span,
  and the deriving pass gives every reference it generates one placeholder.
- **Three boundaries, all found by gpt_chirho reading the committed code**: a record
  consumed by the span join can never be reused by position; an occurrence carrying a
  genuine span belongs to the span join whether or not that join could use it; and a
  placeholder span is not an identity on either side. Five controls hold them, two
  mutation-checked against the previous rule.
- **Unexplained, recorded as such**: one curated run reported 57 failures on an intermediate
  commit; the same commit ran green minutes earlier, the red run overlapped concurrent cargo
  work in one target, and its log was destroyed with `/private/tmp`. Observed once,
  unreproduced, cause unestablished.

## Declaration contexts, duplicate instances and noninjective families (8d86bc95)

Lane `instance-obligations-chirho` (HASKELUJAH/claude_chirho), seven commits on main 6db522ad.
Two of them carry row 484 hunks with provenance and gpt_chirho's agreement: the record checking
of e028c1a3, and the T10808 half of its companion 327aaef7.

- **Declaration contexts use the signature grammar.** Class, instance and standalone-deriving
  contexts went through a token scanner. It kept the first constructor and every variable
  (`(Eq a, Show a)` became `Eq a a`) and dropped any segment with `forall` or `=>`. They now
  go through `type_from_flat_children_chirho` and `type_to_constraints_chirho`, in
  `lower_chirho/contexts_chirho.rs`; the scanner is deleted and the parser root shrinks by
  208 lines, from 17266 to 17058. On main, a class with two superclasses was rejected, and a two-member instance
  context died at run time. Four exact GHC 9.14.1 outputs in `tests/declaration_contexts_chirho.rs`
  now match through the interpreter.
- **The flat grammar** scopes a leading `forall` over `=>` and `->` in its body, and treats
  a depth-0 `::` as a kind annotation. Gains T15079, tc124 and T18831.
- **Rank-n record fields** are checked against their field type (e028c1a3), which holds T18802.
- **Duplicate instances (GHC-59692)** compare written spellings within one module. Five reject
  gains for GHC's reason, one under GHC-43085 (tcfail118), and one adjacent (tcfail056,
  where GHC stops first at GHC-54721).
- **GHC's numeric hierarchy** replaces the report's in the seed: Num has no superclasses, and
  Integral has Real and Enum.
- **Qualified equality operators** follow the exports measured in GHC 9.14.1: `Q.~` works through
  any qualifier, and `Q.~~` only where exported.
- **A declaration header's inline result kind** is not under forall-or-nothing. This holds T23514c.
- **Structural unification never inverts a type family** (327aaef7). This recovers T10808, and
  no should_fail verdict moves.

The first landing measurement of this lane, on 78b832ec, lost T10808. The operands at the
failing call were then observed with an instrumented build that was never committed:
`G t8 ~ G t7`, on two unsolved variables. The deferral guard declined it because the
family-blind structural unifier succeeded by decomposing. This settled the repair site.
Normalizing at the record site could not help, because `G t7` was already in normal form.
The record hunk had been carried without its companion; carrying the companion repaired it.

## Declaration-kind contracts and binder visibility (3db3b6a6)

### The 3db3b6a6 measurement, as recorded on 2026-09-10


Frozen compiler/test source: `3db3b6a69e30d59b60224346b541c245cc68b3e0`.
Measured by HASKELUJAH/gpt_chirho on macOS arm64 in the isolated repair worktree.
Subsequent evidence-only commits do not change the measured compiler or tests.

### Results at their actual scope (3db3b6a6)

| Measurement | Result | What it establishes |
| --- | --- | --- |
| Complete Rust workspace | 3387 passed; zero failed, ignored or filtered; cargo exit 0 | Unit, integration and doctest outcomes under the command below |
| Driver library, included above | 1773/1773 | Includes all 126 native round trips, with exact outputs and bounded children |
| Curated suite, included above | 537/537 | 514 execution oracles actually compared; 23 compile-only inputs |
| Independent GHC 9.14.1 reference | Prior complete 514/514 reference; all source hashes still match | Reference retained from the preceding lane, not 514 fresh GHC executions in this lane |
| Upstream should_compile | 882 of 938 | Typecheck acceptance only; two identical complete passes |
| Upstream should_fail | 221 of 767 | Typecheck rejection only; two identical complete passes |

Each axis's two complete passes agrees exactly. Relative to the previous lists,
accept rises 880 -> 882 (T11811/T20873, no loss), while reject falls 222 -> 221:
T22560_fail_b is newly rejected; tcfail225 and ExplicitSpecificity8 are no longer
rejected. The new rejection is a generic declaration-kind arity mismatch, not
complete GHC-57916 invisible-binder matching. The two losses remove accidental
arity errors; genuine GHC-25897 under NoCUSKs and GHC-57342 for inferred binders
with visible forall remain unimplemented. The reject artifact gives the exact
reason for every movement, including a same-arity counterexample to full binder
matching. No corpus input was edited.
Neither percentage measures execution correctness; neither corpus is fully passing.
The workspace's eight bulk tracking tests being green is not a claim that all
individual upstream inputs pass.

### Reproduction and provenance (3db3b6a6)

```sh
RUST_MIN_STACK=16777216 cargo test --workspace --no-fail-fast -j 3 -- --test-threads=4
```

No test-name skip/filter was supplied. `workspace-results-chirho.jsonl` retains
all 75 target result lines, including zero-test targets, and their aggregate.
One next-target stderr heading arrived before the prior target's stdout summary.
The summary parser pairs targets/results in launch order and requires each
libtest-announced count to agree; a deliberately mutated count is rejected.
Raw completed workspace log SHA-256:

`ce8c45abd24f2ffba344f0bd64f5e0b7aaafdaab6d62c3d5c3ed05947b244d12`

The explicitly rebuilt debug CLI used for all four upstream passes has SHA-256:

`9168f50b07a67d23a8ff45f4bb8f2a8690355b44e71ed9bd44dbf3a64d5ec703`

Its HEAD and digest were asserted before and after every pass. The pure-shell
runner used four workers and 15 seconds per input, with serial 60-second timeout
reruns; all four passes had zero unresolved timeouts and zero unexpected exits. Unexpected
nonzero exits fail the instrument rather than count as acceptance. Exact lists
and the required paired quotation labels remain in the two measurement artifacts.

After fast-forward to evidence checkpoint a1d702dc, the explicit main CLI build
completed without compiler warnings. Main CLI SHA256:
`77cc6d4f1caaba26358ff33e34d2074d6265847254a5ff10efad8cfb71b85eb7`.
All 19 bounded main-path smoke predicates passed with HEAD and CLI digest
unchanged: in-place T001/T002, both accept gains, the three repaired regressions,
T17705, Constraint rejection, three exact-output readbacks, five contradictory
declaration contracts and two Int/Bool field mutations. Smoke log SHA256:
`afc160baa74054c7a435d8b73d335a7bcf4b86e0eae67fa90c21558cd1c5fdc8`.
These are main-path behavioral checks, not a second full workspace/corpus gate.
The full gates above remain the isolated-worktree measurement; measured source
is unchanged. Main and branch source/evidence pushes were read back at a1d702dc.

`test-data-chirho/curated-oracles-chirho/ghc-9.14.1-chirho.jsonl` contains the
independent reference outputs and source hashes. Those 514 source hashes were
rechecked against the frozen tree. Its sibling read-only verifier reruns GHC;
it never invents answers from Haskelujah output. The reference manifest excludes
the 23 compile-only inputs and both upstream typecheck corpora.


Inline result kinds and complete standalone signatures are retained separately,
including when both are written. The former composes with the head; the latter
constrains that complete shape under an independent lexical kind-name cache.
Data/newtype head `@` binders remain lexical but consume no ordinary type
argument. Required forall kinds retain visible argument arrows rather than
erasing them as invisible foralls. Constructor/selector result applications and
derived instance heads use the same visible-parameter distinction.

Five parser, two naming, nine typing and six integration tests explain
3365 -> 3387. Parser333, naming134, typing344, typing integration33 and canaries7
all pass. Three identical-source programs match fresh GHC 9.14.1 output through
STG, LLVM and Cranelift: combined kind contracts 42/7/11/13, invisible-head field
reads 42/7, and a required-kind argument 19. Five GHC-invalid head/tail contracts
reject at reconciliation; two field mutations reject Int/Bool mismatches.

The first frozen trial lost T22560a/T22762/T23514c. Their reductions exposed
erased binder distinctions; preserving those distinctions recovered all three
without weakening the five negative contracts. Full final sets have no accept
losses. ExplicitSpecificity8's invalid declaration alone was accepted by main
before this lane; the old errors were only at its applications. A fresh GHC
pair also distinguishes CUSKs from NoCUSKs on the recursive tcfail225 shape.
That declaration-completeness/instantiation gap is not interchangeable with
standalone-signature subsumption, even though both can report rigid-kind errors.

The stronger generic record Eq/Show probe exposes a separate pre-existing
execution defect, also reproduced on main after removing both invisible binders:
Eq reaches an unresolved `==`, and Show loses record labels/newtype construction.
The new field-read execution test does not claim those operations work; the
full Eq/Show source is retained as an instance-head typechecking control, with
the execution limitation and reductions recorded in the tasklist. No old test
was removed or waived, and no wrong formatting was installed as an oracle.

Focused parser helpers/tests are 125/190 lines; kind helpers/tests are 110/280.
The parser root is still 17266 lines, kind root3284, infer root27788 and deriving
root3669: this is not size-gate compliance. Complete kind schemes, rigid kind
checking, full inferred/specified binder matching, constructor VTA metadata,
TH visibility and cross-module kind authority remain explicit limits.

## Flat list-type reconstruction (cb939e35)

All four list reconstruction routes share the existing AST distinction between
the constructor `[]` and an applied list `[a]`. The flat route no longer invents
a placeholder element for empty brackets; its bracket scanner also respects
nesting, retaining the function tail in `[[Int] -> Int]`.

Five parser controls and four driver tests explain 3356 -> 3365. Parser passes
328/328; typing remains 335/335; typing integration passes 27/27, canaries 7/7.
All five new parser controls were demonstrated red on the old scanner before
restoring the repair. Three identical-source programs execute on STG, LLVM and
Cranelift against fresh GHC 9.14.1 oracles: prefix-list field readback `kept`,
nested list-of-functions readback `42`, and the combined GADT-record case
`kept/42`. Unsaturated and overapplied list fields reject for a kind mismatch.

Flat helpers now occupy 559 lines with 185 lines of separate tests. The parser
root loses 528 lines but remains 17346 lines; this is not size-gate compliance
or the deferred broad split. No full flat-forall, deriving-strategy, warning-
policy or named-kind support is claimed. GHC-10107 is recorded as a concrete
open rule in the row-482 tasklist; preserving a spurious E0300 would not fix it.

## Lexical kind-binder scope repair (3683dae8)

Forall and quantified-constraint binders now have lexical lifetimes in kind
inference. The kind environment stores the kind of a type variable; the
kind-identity cache stores that variable when interpreted as a kind. The two
maps receive distinct appropriate identities and restore only touched entries
in reverse order. Binder annotations see preceding binders but not themselves;
new free discoveries and substitution/counter progress survive scope exit.
Declaration-head binders remain available through their fields, methods and RHSs.

Seven kind controls and five driver integration tests explain 3344 -> 3356.
Typing passes 335/335; typing integration 23/23. Four identical-source executions
match freshly run GHC 9.14.1 oracles on STG, LLVM and Cranelift: higher-kinded
shadowing in both source orders, required binders, explicit kind annotations,
and quantified-constraint shadowing. A genuine inconsistent-kind application
remains rejected by both GHC and our kind checker. Five new unit controls were
first shown red on the old implementation; the seventh annotation/cache control
was added afterward and is not falsely counted as a demonstrated baseline red.

Related conversion logic moved beneath kind_chirho/: conversion is 355 lines,
scope 58, tests 250. The root loses 377 lines but remains 3364 lines and therefore
does not meet the size gate. The independent source audit found no blocker.
Full kind schemes, structural standalone-signature/head reconciliation, complete
class-head kind validation and synonym alpha-renaming remain separate limits.

## Earlier lexical forall scope repairs (dace042b)

Surface conversion now assigns fresh identities to invisible and required forall
binders, then restores just their previous map entries in reverse order. Free
names first encountered inside the scope survive. Bookkeeping grows with the
number of binders, not the size of the enclosing environment. Leading signature
binders/contexts are converted in lexical order; result-spine contexts reopen
the matching converted binder IDs rather than depend on leaked map entries.

Quantification and definition scope have separate paths. Only the single
syntactically outermost invisible forall group scopes the RHS. Parenthesized
quantification still works, but parentheses and later forall groups do not export
their names to the definition. Explicit local foralls freshly shadow enclosing
scoped variables. The existing flat scheme-predicate representation is retained.

Seven typing controls and six driver integration tests explain 3331 -> 3344.
The complete typing suite passes 328/328 and typing integration passes 18/18.
Four new identical-source programs run on STG, LLVM and Cranelift against freshly
executed GHC 9.14.1 oracles: invisible scope/free discovery/enclosing scope;
required scope; parenthesized/local/consecutive leading scope; constrained result
foralls. Negative controls check the relevant rigid mismatch rather than any error.
Three scope unit controls were first demonstrated red against the original converter.

The consumer audit found no remaining in-scope dependency on the leaked map.
At that checkpoint, separate kind-map shadowing and the synonym RHS's fixed-ID/name-
substitution representation remained open. The higher-kinded shadowing program
printed 42/7 under GHC and failed E0300 in our compiler; 3683dae8 repairs that
kind-map lifetime above. Synonym alpha-renaming, nested qualified-type representation,
and named required RHS binders remain separate limits.

Both approximate public labels are carried forward unchanged, with current
counts. L.J.'s uniform policy decision on truncation versus nearest remains
pending. At 882/938 the accept label agrees under either policy; at 221/767 the
reject label still differs. No policy choice or deployment is part of this
measurement supersede. The earlier flat-list checkpoint held 880 accepts and
222 rejects, after removing T14761a's accidental rejection.

## Earlier infix type and binder repairs (18b7d1c3)

Lowercase backticked type variables now retain their namespace and lexical token
spans when normalized to prefix application. Scheme quantification orders its
existing variables by first source occurrence, including context occurrences before
the body; explicit forall order and class/scoped-variable handling are preserved.
Type-chain reduction uses the existing fixity table with linear push/reduce work.
Parenthesizing tc156's signature independently isolated the precedence defect.

Four parser and five typing controls were added. Driver typing integration at
that checkpoint was 12/12, including two identical-source executions on STG, LLVM and Cranelift:
signature ordering prints `42/7/11/True`; type fixity prints `3/True/'c'`.
Both sources were freshly executed under GHC 9.14.1. A reversed Int/Bool operand
control rejects for the expected type mismatch, also independently confirmed.
The class/scoped/rank-N control establishes typechecking only; the separate
custom-method runtime dispatch limitation is not hidden by an execution claim.

The 13 additional Rust tests explain 3318 -> 3331. The published accept rounding
changes ~93% -> ~94%; the reject rounding remains ~29%. No corpus source or oracle
was edited. tc192 now gets past infix naming but remains blocked on Arrows proc
notation. Full kind-dependency ordering and same-name nested-forall conversion
were open at that checkpoint; the surface/signature repair is described above.

## Earlier execution-correctness repairs (2f74126d)

The old curated reader compared 14 oracles and ignored 500 declared EXPECTED
headers. The first repair still missed two LANGUAGE-first inputs; the complete
prologue reader and an independent directive inventory now require all 514 to
execute. Wrong-output and unrelated-error controls prove the instrument can fail.
The initial 23 disputes were arbitrated against unchanged source under GHC
before choosing code, input or oracle repairs. The later full reference pass
found 24 invalid inputs; those received explicit imports/type annotation without
changing their expected output, and all 514 were rerun successfully.

The old native optional-result helper silently passed four crash/link outcomes.
The bounded helper now requires successful compilation/linking, normal child
completion and the oracle. The custom-list runaway, demand/laziness boundaries,
IO action reuse, dictionary evidence, thunk memoization and GC roots were repaired
at their respective representations. T2045's desugaring growth has a resource-curve
test; native/CPP temporary ownership has concurrent distinct-answer tests.
The tasklist records the reductions, failed intermediate gates and repair details.

## Limits and remaining work

- The full workspace command emitted no Rust compiler warnings. A separate
  targeted all-target clippy run for AST, parser, naming, typing and driver exited
  zero and emitted **462 distinct diagnostics** (685 raw records, including
  dependency/target duplicates). None has a primary span on any of the 1997 lines
  this lane changed across 21 files, measured against main 6db522ad. This targeted
  JSON count is not directly comparable with the older workspace mixed-output line
  count. Existing lint debt remains: this is not the project's zero-warning quality
  gate. Oversized files/directories remain structural debt (this lane moves the
  parser root from 17266 to 17058 lines and adds no source file over 320); no comprehensive
  split or lint waiver is claimed.
- A family equality left stuck at module end is still dropped rather than reported.
  GHC reports it ("non-injective type family ... ambiguous"), and it rejects an
  ambiguous signature such as `G a -> G a` at its own ambiguity check (GHC-83865)
  where we report use-site mismatches instead. No corpus file moves either way today.
- An instance of a first-order class at an APPLIED data type (`instance C (Box a)`)
  still fails to dispatch at run time, and an instance method that calls its own
  class's method at another type dispatches to the instance being defined. Both
  predate this lane and are recorded in its tasklists.
- Required licensed fixture slices run without the private package-cache symlink.
  Optional whole-package tests may return early if that cache is absent; a Rust
  pass from that path is not evidence of compiling the missing package.
- This is macOS execution with a 16 MiB Rust test-thread stack. Conditional Linux
  resource-limit code compiling here does not establish a Linux execution result.
- IO-action lowering covers STG and the two native backends; Wasm has not been
  migrated to that representation. Native round trips are not automatically
  three-engine comparisons; shared-source controls are named in the workflow.
- Data-family/type-data shapes, refined GADT record results and the other red
  upstream corpus inputs remain compatibility work. No site deployment occurred.

## The corpus runner and its one verdict rule (2026-09-22)

The runner is committed beside this document in `corpus-runner-chirho/`, so no landing carries a
private detector forward again. `classify_chirho.sh` holds the single verdict rule every script uses,
in the order gpt_chirho's review set (room #24609): a timeout (exit 124) or any exit other than 0 or 1
is classified first, then a REAL Rust panic header (`thread '...' panicked at`), and only then is a
diagnostic credited: REJECT needs `error[E` and exit 1, ACCEPT needs exit 0 and no `error[E`. The bare
substring `panic` is never a verdict, because quoted source can contain it.

- `axis_pass_chirho.sh`: one complete pass of an axis (parallelism 4, 15s, timeouts re-run alone at
  60s), optionally keeping every file's raw output.
- `replay_rejects_chirho.sh`: every REJECT row of saved passes re-run on the same frozen CLI, keeping
  stdout, stderr, exit code and source SHA-256 per file. The pass lists establish status and
  membership; the replay establishes that each rejection is a diagnostic and not a crash.
- `execution_chirho.sh`: execution evidence against GHC. A PASS needs `runghc` to exit 0, the tested
  command to exit 0, and stdout byte-identical to the reference; every command's stdout, stderr and
  exit code are kept, and a native compile failure is a FAIL with its output, never a skip.
