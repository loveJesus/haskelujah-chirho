<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Execution measurement — 2026-09-10

Frozen compiler/test source: `dace042b98e12ea3c8a885f17fa1e9c320b1481d`.
Measured by HASKELUJAH/gpt_chirho on macOS arm64 in the isolated repair worktree.
Subsequent evidence-only commits do not change the measured compiler or tests.

## Results at their actual scope

| Measurement | Result | What it establishes |
| --- | --- | --- |
| Complete Rust workspace | 3344 passed; zero failed, ignored or filtered; cargo exit 0 | Unit, integration and doctest outcomes under the command below |
| Driver library, included above | 1773/1773 | Includes all 126 native round trips, with exact outputs and bounded children |
| Curated suite, included above | 537/537 | 514 execution oracles actually compared; 23 compile-only inputs |
| Independent GHC 9.14.1 reference | Prior complete 514/514 reference; all source hashes still match | Reference retained from the preceding lane, not 514 fresh GHC executions in this lane |
| Upstream should_compile | 879 of 938 | Typecheck acceptance only; two identical complete passes |
| Upstream should_fail | 223 of 767 | Typecheck rejection only; two identical complete passes |

The accept set is byte-identical to its previously committed list. The reject
set gains GivenForallLoop with zero losses: its inner forall no longer captures
the outer signature variable, producing the rigid mismatch reported by GHC-25897.
The unchanged file was also freshly checked with GHC 9.14.1. Each axis's two passes agree exactly.
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

`b51621d1b0746614570b2e25ae9f7920fab8329b34a8ea5214c83be0edc1e11d`

The explicitly rebuilt debug CLI used for all four upstream passes has SHA-256:

`9d449ed5521d4798a5fa626ad08233409d023ecc393f2afc423a5fe363a19c32`

Its HEAD and digest were asserted before and after every pass. The pure-shell
runner used four workers and 15 seconds per input, with serial 60-second timeout
reruns; all four passes had zero unresolved timeouts and zero unexpected exits. Unexpected
nonzero exits fail the instrument rather than count as acceptance. Exact lists
and the required paired quotation labels remain in the two measurement artifacts.

Main landing and its separate rebuilt-CLI path/execution smoke checks are pending
at this evidence checkpoint. They are not implied by the completed worktree gates.

`test-data-chirho/curated-oracles-chirho/ghc-9.14.1-chirho.jsonl` contains the
independent reference outputs and source hashes. Those 514 source hashes were
rechecked against the frozen tree. Its sibling read-only verifier reruns GHC;
it never invents answers from Haskelujah output. The reference manifest excludes
the 23 compile-only inputs and both upstream typecheck corpora.

## Lexical forall scope repairs (dace042b)

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
Separate kind-map shadowing and the synonym RHS's fixed-ID/name-substitution
representation remain open. A higher-kinded shadowing program prints `42/7` under
GHC and fails E0300 on both baseline and repaired CLIs; the tasklist records its
reproduction. This lane does not claim that separate kind repair, synonym
alpha-renaming, nested qualified-type representation, or named required RHS binders.

Both approximate public labels retain their existing convention. L.J.'s decision
on truncation versus nearest remains pending; no policy change or deployment is
part of this measurement supersede. The exact reject count changes 222 -> 223.

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

- The full workspace command emitted no Rust compiler warnings. The separate
  workspace all-target clippy run exited zero but produced **636 warning-output
  lines**, including 42 Cargo summaries and duplicate rendering. A read-only
  comparison with the prior 535-line log found the same 493 normalized primary
  diagnostics at 492 source locations, accounting for extraction line shifts;
  the extra 101 lines repeat parser diagnostics in the mixed output format.
  No diagnostic identifies either new conversion module or the edited integration
  test. This is not the project's zero-warning quality gate. Oversized files and
  directories remain structural debt; no comprehensive split or lint waiver is claimed.
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
