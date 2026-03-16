<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

# GHC Test Suite (Local Verification) - ghc-tests-chirho

This directory contains test files extracted from the
[GHC compiler test suite](https://gitlab.haskell.org/ghc/ghc) for use as
verification inputs for the haskeluya-chirho Haskell compiler.

## Contents

| Directory              | GHC Source Path               | Purpose                          |
|------------------------|-------------------------------|----------------------------------|
| `parser-chirho/`       | `testsuite/tests/parser/`     | Parser tests (syntax, layout)    |
| `rename-chirho/`       | `testsuite/tests/rename/`     | Renamer / name-resolution tests  |
| `typecheck-chirho/`    | `testsuite/tests/typecheck/`  | Type-checker / inference tests   |
| `layout-chirho/`       | `testsuite/tests/layout/`     | Layout-rule (offside) tests      |
| `module-chirho/`       | `testsuite/tests/module/`     | Module system tests              |

## File Types

- `.hs` / `.lhs` -- Haskell source files (test inputs)
- `.stderr` -- Expected compiler error output
- `.stdout` -- Expected program output
- `.T` -- GHC test-driver metadata (test names, flags, expected outcomes)
- `.hs-boot` -- Boot files for mutual recursion tests
- `.script` -- GHCi script inputs

## Important Notes

- **This directory is gitignored.** It exists only for local verification and
  is not committed to the repository.
- These files are copyright their respective GHC contributors and are provided
  under the GHC license (BSD-3-Clause).
- To regenerate, run a sparse checkout of the GHC repo targeting
  `testsuite/tests/{parser,rename,typecheck,layout,module}`.
- Binary artifacts (`.o`, `.hi`, `.a`, `.so`, `.dylib`, `.exe`) have been
  removed; only source and expected-output files are retained.
