<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Source-local boot declaration contracts, 2026-09-15

Owned row484 checkpoint on gpt-kind-schemes-chirho, based on
1b2e489450c535f630c107d3a6b6205c927c8726. No main compiler landing, canonical DB
closure, published corpus artifact update, denominator change or deployment.
The worktree is now inside haskelujah-workspaces-chirho; historical reference
records retain the paths at which their commands actually ran.

## Representation and ownership

Class declarations retain whether their context was written. An omitted empty
context can represent an abstract boot class; a written empty context cannot.
Abstract class agreement compares the checked head kind and functional-dependency
positions. Instance promises compare all head and context arguments in one closed
variable namespace against source-local implementation instances, including derived
ones. Imported instances are not evidence that the implementation declared a
promise. Shared registration also now retains every argument of multi-parameter
constraints rather than only the first.

Kind ownership is captured from the kind checker's local declaration inventory
before export selection and dependency closure. An imported type mentioned by a
boot signature is a dependency, not a declaration the importing module owns.
Private declared kinds remain available for agreement even without public exports.
The dependency transport map and declaration ownership map have separate jobs.

Class/instance parser and typing handlers are extracted into focused declaration
owners. No new dependency is introduced. The existing oversized root files remain
structural debt; extraction is not a repository-wide file-size compliance claim.

The workflow is documented in
spec-chirho/workflows-chirho/compiler-pipeline-chirho/module-search-authority-chirho.md.

## Behavioral controls and independent reference

The original twelve class/instance integration cases were six passes and six
failures before agreement: all six intended accepts failed at the unsupported
guard. They became twelve passes after the repair. Earlier negative greens only
proved the blanket guard rejected something, not matching reason-level agreement.

Three filesystem ownership controls were one pass and two failures before the
ownership split. The valid imported-type cycle failed, while a private conflicting
kind was wrongly accepted. All three pass after repair through both file-check
and file-compile entry points. The dual SOURCE cycle is independently accepted by
GHC 9.14.1 from each of its two source roots.

The audit added two controls that first failed against the unaudited code:
fresh randomized maps did not always report the alphabetically first mismatch,
and a deliberately missing local kind binding was silently omitted. Final typing
results include both. The missing-binding test is internal fault injection; no
valid source reproducing that absent-binding branch has been established.

First-error map keys are now ordered. Kind agreement distinguishes declaration
shape, unclosed contract and actual kind/classifier disagreement. Instance
comparison keeps one shared 16,384-candidate bound per boot/implementation pair;
it was not changed to an allowance multiplied by the number of promises.
Exhaustion never counts as agreement.

Fresh CLI observations agree with 44 of 45 GHC-reference source-root verdicts:
all 40 retained cases and four of five ownership observations. The one difference
is deliberately recorded, not counted as a pass:

- private_missing_head_chirho: an unused, unexported boot SecretChirho has no
  implementation declaration. GHC 9.14.1 accepts; the candidate reports that the
  boot type has no local implementation. The old declaration guard also required
  that local head. This remains an unresolved compatibility boundary, not proof
  that either broad interpretation of private boot promises is correct.

The first private matching reference used a phantom implementation parameter.
GHC rejected it for a role mismatch, so it was not the positive control intended.
The corrected source uses the parameter in a constructor field and is accepted.
Both the original disagreement and corrected source are retained.

GHC's separate-compilation documentation is the reference for boot declarations
and implementation agreement, including abstract classes and abstract closed
families. It does not by itself settle the unused-private-head edge observed here:
https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/separate_compilation.html#mutually-recursive-modules-and-hs-boot-files

## Frozen gates

All final commands completed with exit zero using the documented 16 MiB Rust
test-thread stack. The six test targets have zero failed, ignored or filtered tests.

| Target | Passed |
| --- | ---: |
| Parser library | 368 |
| Typing library | 392 |
| Naming library | 137 |
| Driver typing integration | 259 |
| Driver canaries | 7 |
| Full driver library | 1,776 |

Workspace all-target check, explicit CLI build and format check pass without
compiler warnings. Driver all-target Clippy exits zero but emits 454 warning-message
lines including duplicates and summaries. This is not lint clean. These are the
named targets, not a full workspace test run or a complete execution compatibility
claim.

The exact CLI SHA-256 stayed unchanged before and after all observations:

d76de19d240c2e5c4f47dc25b1f458975f030b920f16d1c471e987881282bd73

The gates ran against uncommitted but frozen source. Their parent/diff/untracked
manifest and log hashes are retained. Code-file hashes were independently matched
before evidence packaging so the final commit can be checked without reproducing
that pre-evidence worktree state. Raw gate output remains in the scratch path
recorded by the runner; it is not a durable input required to read these summaries.

## Focused corpus results and remaining work

Seventeen of the eighteen former SOURCE-cycle failures now pass the fresh CLI.
Compared with 1b2e4894's thirteen, the four recoveries are Tc267a, Tc267b, Tc271
and Tc271a. T6018a still rejects because abstract closed-family agreement is not
represented. The next representation must distinguish an abstract closed family
from a genuinely empty closed body; interpreting both as an empty equation list
would fabricate agreement.

All eleven older focused blockers remain failures: CoerceToVDQ, T13879,
T16204a, T16204b, T17067, T18129, T22560c, T23543, T26358, T13585 and T13585b.
Their source hashes and exact diagnostics are preserved. These observations are
not a complete census of remaining main-relative regressions.

No full 938/767 corpus measurement was run for this checkpoint. No focused
recovery is extrapolated into a new published total. Full class method/default/
associated-member agreement, nullary or quantified-context instance agreement,
abstract closed-family agreement and fixity agreement remain unsupported.
Nominal-name normalization, package-qualified ownership and imported runtime-body
linkage retain their separately documented limits.

## Evidence files

- references-chirho.jsonl: fifteen GHC 9.14.1 class/instance reference cases,
  including sources, commands and original diagnostics.
- ownership-references-chirho.jsonl: five corrected GHC source-root observations.
- ownership-reference-disagreement-chirho.jsonl: the original five observations,
  retaining the phantom-role mismatch rather than relabeling it an accept.
- observations-chirho.jsonl: one metadata record, 45 reference-comparison
  observations, 29 focused corpus observations and one summary; 76 records, not
  76 tests. The earlier 25 reference sources live in the parent evidence folder.
- gates-chirho.json: ten final gate records, exact commands, output hashes,
  pre-evidence input manifest and independently verified code-file hashes.
