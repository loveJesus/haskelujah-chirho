<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Associated-default scope evidence Chirho

This is a continuation of isolated row484, not a main landing or a replacement
for either published corpus measurement. The clean starting tree is02e74761,
with compiler code from97218fde. Main and the canonical progress DB stay held.

## Reference and source controls

Claude's original six GHC9.14.1 observations are preserved in
`reference-claude-chirho.jsonl`; `replay-input-chirho.jsonl` only supplies the
correct root file for the existing CLI observer. The parent's `candidate-chirho`
replay agrees2/6. The expanded reference runner records its predictions before
execution, exact source bytes/hashes, GHC version, command, exit and diagnostics.

The first12-case matrix is retained in `expanded-reference-chirho.jsonl`:
eight predictions held and four were refuted. The old explicitly built CLI
agrees4/12 in `expanded-before-chirho.jsonl`. The final17-case matrix adds
shared/distinct hidden-kind and omitted-result-kind controls; twelve predictions
held and five were refuted. All seventeen Rust integration sources and expected
verdicts were independently compared byte-for-byte with `final-reference-chirho`.
No expectation was selected from our compiler's output.

The measurements distinguish these contracts:

- An equation binds its own LHS variables, including written kind variables;
  a class parameter or a name appearing only on the RHS is not thereby in scope.
- Written equation kind variables are rigid. A family application's hidden
  inputs must remain distinct variables, not become Type or merge two slots.
- Class methods may determine parameter kinds before publication; defaults
  consume the finalized kind rather than participate in inferring it.
- An omitted associated-family result kind means Type. A method cannot turn
  that omitted result into Type -> Type.

The pre-audit repair's `after-chirho.jsonl` agrees23/23: all17 expanded controls
and the original six. Its executable hash is historical, not assumed current.

## Recovery and independent scope

Claude2's audit identified a real publication gap: after reporting an invalid
hidden input, the checker still recorded the rejected application as checked
kind evidence. Six direct boundary controls demonstrate1/6 before the repair
and6/6 after. The full test output and before/after file hashes are retained in
`recovery-controls-chirho.json`.

The repaired path withholds that occurrence after any new equation-local kind
error, reports the first invalid hidden slot deterministically, and swaps its
own identity cache instead of inheriting the class-method scope policy. A valid
default following an unrelated error still publishes its own record. LHS
shadowing and cache restoration are tested with a deliberately occupied outer
scope; the helper no longer relies on an earlier loop clearing it.

The limited annotation visitor is justified by declaration validity, which
requires the Paren/KindAnnot spine to end in a variable. It does not claim to
validate arbitrary family patterns or ignore invalid patterns as compatible.

## Diagnostic corpus, not publication

The first post-scope diagnostic under `corpus-chirho` completed890/938 accept
and265/767 reject. Relative the parent's891/264 it changes exactly two verdicts:

- T5481 is newly rejected for out-of-scope RHS class parameters. GHC9.14.1
  reports those same scope errors, and the unchanged upstream all.T explicitly
  uses `compile_fail` despite its should_compile directory. It remains counted
  as an accept-axis loss under the unchanged directory-based methodology.
- AssocTyDef04 is newly rejected for a default returning bare Maybe where Type
  is required. GHC9.14.1 rejects the same kind contract (GHC-83865).

`corpus-chirho/ghc-deltas-chirho.jsonl` retains the unchanged source, hash,
upstream test declaration and complete reference diagnostics for both files.
No corpus member, denominator, label or failure list was edited to remove the
T5481 loss. The nine earlier main-relative accept regressions also remain open.

## Gates and provenance

`interrupted-gates-chirho.json` is explicitly incomplete: formatting, workspace
all-target checking, parser371/naming137/typing401, integration429 and canaries7
completed; the full driver process was intentionally terminated at11:41 EDT
on2026-09-18 to honor Claude's corpus pass. Cargo's exit101 for that termination
is not a semantic test failure and is not a completed driver gate.

The final gate and CLI replay must describe the post-audit source hashes.
Their results and the final diagnostic replay are recorded when they finish;
an older green is not substituted for that proof.

## Associated-instance continuation

The later `corpus-chirho/associated-final-chirho` diagnostic is878/938 accept
and267/767 reject, before the dependent-default repairs. Twelve files are newly
rejected relative to the890/265 scope diagnostic, with no accept recoveries.
The run overlapped Claude's surviving driver-test child; tc089 timed out once
and passed an isolated retry. Zero unresolved timeouts or abnormal exits does
not make this a clean performance measurement. Main-relative accept losses
remain, and this checkpoint is not eligible for main.

`corpus-chirho/associated-instances-chirho/dependent-chirho` preserves19
independent GHC9.14.1 observations, their source hashes and both original
runners. The capture script checks those hashes and generates the Rust fixture
from exactly those bytes. `before-chirho.jsonl` agrees16/19. The new test executes
all19 cases (one Rust test, not nineteen separately reported test functions).
After preserving explicit equation arguments, retaining dependent classifiers,
rigidifying default LHS terms and supplying builtin promoted Bool/Ordering kinds,
the parameterized test and the explicit CLI replay both agree19/19. No output
or expected verdict was edited to match ours.

`focused-gates-chirho.json` records parser372/naming137/typing407,
integration434 and canaries7, with source hashes stable across those runs.
`targeted-regressions-chirho.json` records recovery of T11401, T16008, T17566
and T21205 from the twelve newly lost accept cases; the other eight still reject.
It does not replace a full corpus pass. The final lint cleanup and repeat gates
are recorded separately in `checkpoint-gates-chirho.json`, with a new explicit
CLI replay in `checkpoint-replay-chirho.jsonl` (19/19, no timeout). The repeat
parser372, integration434 and canaries7 are green with zero ignored/filtered.

`live-package-chirho.json` is a real failed optional package test, not a skip:
after linking the existing local cache, boring reports missing Rep and Eq.~~,
then constraints cannot import Data.Boring. The first --exact invocation ran
zero tests; the corrected command ran one (1775 filtered) and failed. Full-driver
and workspace-wide green claims are not supported by the focused gate. Clippy
still emits warning debt despite exit0. Main and public artifacts remain unchanged.
