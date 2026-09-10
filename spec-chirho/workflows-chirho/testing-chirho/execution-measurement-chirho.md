<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Execution measurement — 2026-09-10

Frozen compiler/test source: `3683dae897963c4d61bc3737796a7f2969a76bc8`.
Measured by HASKELUJAH/gpt_chirho on macOS arm64 in the isolated repair worktree.
Subsequent evidence-only commits do not change the measured compiler or tests.

## Results at their actual scope

| Measurement | Result | What it establishes |
| --- | --- | --- |
| Complete Rust workspace | 3356 passed; zero failed, ignored or filtered; cargo exit 0 | Unit, integration and doctest outcomes under the command below |
| Driver library, included above | 1773/1773 | Includes all 126 native round trips, with exact outputs and bounded children |
| Curated suite, included above | 537/537 | 514 execution oracles actually compared; 23 compile-only inputs |
| Independent GHC 9.14.1 reference | Prior complete 514/514 reference; all source hashes still match | Reference retained from the preceding lane, not 514 fresh GHC executions in this lane |
| Upstream should_compile | 879 of 938 | Typecheck acceptance only; two identical complete passes |
| Upstream should_fail | 223 of 767 | Typecheck rejection only; two identical complete passes |

Both upstream result sets are byte-identical to their previously committed lists:
879/938 accept and 223/767 reject, with zero gains or losses. Each axis's two
complete passes agrees exactly. This kind-scope repair fixes GHC-valid programs
outside the measured upstream subset; unchanged counts do not imply unchanged behavior.
Neither percentage measures execution correctness; neither corpus is fully passing.
The workspace's eight bulk tracking tests being green is not a claim that all
individual upstream inputs pass.

## Reproduction and provenance

```sh
RUST_MIN_STACK=16777216 cargo test --workspace --no-fail-fast -j 3 -- --test-threads=4
```

No test-name skip/filter was supplied. `workspace-results-chirho.jsonl` retains
all 75 target result lines, including zero-test targets, and their aggregate.
One next-target stderr heading arrived before the prior target's stdout summary.
The summary parser pairs targets/results in launch order and requires each
libtest-announced count to agree; a deliberately mutated count is rejected.
Raw completed workspace log SHA-256:

`3c90864404f5bca70a59ffc9e042e91707e48e4dbedb6662d9157892da1e87dc`

The explicitly rebuilt debug CLI used for all four upstream passes has SHA-256:

`f83c9fc56f51eeaf6107032cfbb0b9aad8d60f2515864977edd47ce3bdb321ab`

Its HEAD and digest were asserted before and after every pass. The pure-shell
runner used four workers and 15 seconds per input, with serial 60-second timeout
reruns; all four passes had zero unresolved timeouts and zero unexpected exits. Unexpected
nonzero exits fail the instrument rather than count as acceptance. Exact lists
and the required paired quotation labels remain in the two measurement artifacts.

Post-landing main-path checks are recorded after the source/evidence fast-forward;
the complete gates above retain the isolated worktree binary's provenance.

`test-data-chirho/curated-oracles-chirho/ghc-9.14.1-chirho.jsonl` contains the
independent reference outputs and source hashes. Those 514 source hashes were
rechecked against the frozen tree. Its sibling read-only verifier reruns GHC;
it never invents answers from Haskelujah output. The reference manifest excludes
the 23 compile-only inputs and both upstream typecheck corpora.

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

Both approximate public labels retain their existing convention. L.J.'s decision
on truncation versus nearest remains pending; no policy change or deployment is
part of this measurement supersede. The exact reject count previously changed 222 -> 223; it is unchanged here.

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
  targeted all-target clippy run for typing and driver exited zero but emitted
  **552 warning messages**, including dependency/target duplicates. None has a
  primary span in the new kind child modules or the edited integration file.
  This targeted JSON count is not directly comparable with the older workspace
  mixed-output line count. Existing lint debt remains: this is not the project's
  zero-warning quality gate. Oversized files/directories remain structural debt;
  no comprehensive split or lint waiver is claimed.
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
