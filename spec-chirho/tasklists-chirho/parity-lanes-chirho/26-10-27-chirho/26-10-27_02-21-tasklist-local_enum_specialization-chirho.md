<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 607: Local Enum specialization Chirho

- [x] Reserve Row 607 in the canonical progress database and create an isolated worktree from
  `main_chirho` reservation commit `0aba3695`.
- [x] Measure the local-generalization `Enum` residual on GHC 9.14.1, the interpreter, LLVM and
  real Cranelift.
- [x] Trace the exact generalized variable, instantiation and evidence record before source.
- [x] Bound the repair against Row 601 result-only Enum and Rows 598/603/606.
- [x] Implement the smallest proven evidence-key repair and update its workflow.
- [x] Add all-engine movers, existing-route holds and a mutation-sensitive control.
- [x] Run the focused warm gate, changed-line Clippy, rustfmt, diff-check and gitleaks.
- [ ] Land through the canonical single-writer chain and close progress Row 607.

## Initial boundary Chirho

- The measured residual is `print (map succ "ab", toUpper 'c') where toUpper = succ`: the
  interpreter prints the local result as an Int instead of a Char while the native word
  representation can mask the route.
- Row 601 already owns a top-level result-only `Enum a => Int -> a` dictionary parameter. This
  row owns only a locally generalized binding whose concrete instantiation should specialize its
  method occurrence.
- Qualified-name resolution, structured Eq and Eq's `/=` method semantics remain outside this
  row.

## Measured mechanism Chirho

- GHC 9.14.1, LLVM and real Cranelift print the local alias as a Char; the interpreter printed its
  ordinal Int. Typing had already resolved the locally generalized variable to `Char` and finalized
  `(Enum, Char)` for the RHS method reference, but desugaring minted no occurrence id because
  `succ` and `pred` were absent from `EVIDENCE_METHOD_NAMES_CHIRHO`.
- Row 607 adds exactly those two symmetric Enum methods to that population. The existing join and
  dictionary pass then consume the proof; no new inference or Core dispatch rule is added.
- A local user definition of `succ` or `pred` keeps its own body. On the reservation base, seeded
  `Word`, `Int8` and unit step rows already fail by name; the candidate retains those exact named
  residuals on the interpreter, LLVM and real Cranelift.
- Reach mutation: removing only `succ` and `pred` from the population changes the local line from
  `('f','e')` to `(102,101)` in the interpreter. Restoring the two entries byte-for-byte restores
  the all-engine answer.

## Gate Chirho

- The focused warm gate passed 238/238 tests: 35 driver evidence-library tests, 124 dictionary
  evidence tests, 7 method-occurrence tests, 23 evaluation-value tests and 49 typing-integration
  tests.
- The workspace built with no warning. Changed-line Clippy was clean; rustfmt and diff-check were
  clean.
- The row 535 contract judge passed all 41 programs and 126 engine cells, accounting for the 73
  landed movers and 9 holds. Row 607 moved no contract cell.
