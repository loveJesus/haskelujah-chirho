<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native print evidence tasklist

Owner: `gpt_chirho`

## Brick 1

- [x] Claim the remaining open native structured-value printing defect.
- [x] Preserve the measured boundary: runtime tags cannot distinguish unboxed `True` from
      integer `1`; the compiler must retain solved `Show` evidence.
- [x] Confirm typing already records `print :: Show a => a -> IO ()` occurrences, while the
      desugar allowlist and driver join currently admit only class methods.
- [x] Baseline native `print` and explicit `show` across scalar and structured values on a
      fresh binary.
- [x] Inspect pre-dictionary and post-dictionary Core for one scalar and one structured value.

## Fix

- [x] Thread concrete `Show` evidence through `print` occurrences without backend tag guessing.
- [x] Preserve standard numeric defaulting and the interpreter's correct `print` behavior.
- [x] Retain full structured instance keys when the solved `Show` predicate is concrete.
- [x] Keep unresolved evidence loud or on the existing conservative path; do not silently
      invent a structured instance.

## Gates

- [x] Focused typing, driver-join, dictionary-rewrite, and native round-trip regressions pass.
- [x] LLVM and Cranelift print the expected scalar and structured values.
- [x] Bounded affected evaluator surfaces and the full Core suite pass with resource caps.
      The broader `eval_` attempt hit its ten-minute cap and is not claimed as a full-suite pass.
- [x] Fresh interpreter/LLVM/Cranelift CLI matrix, owned-file Rust formatting, and
      `git diff --check` pass.
- [x] Commit explicit owned paths, push `main_chirho`, and announce builder release.
