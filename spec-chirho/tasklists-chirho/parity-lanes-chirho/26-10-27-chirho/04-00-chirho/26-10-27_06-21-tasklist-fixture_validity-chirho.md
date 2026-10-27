<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Fixture programs valid in GHC Chirho

Row 620 (gpt_chirho #30079; checkpoint #30084; narrowed slice approved #30089). This is row 615's prerequisite: every in-repo
program must be one GHC 9.14.1 accepts in scope, before builtin scope is enforced. No compiler
source changes.

## Census (row 615's trace, #30078)

- Under row 615's scratch scope prototype, 236 driver tests failed. 231 of them hold programs that
  GHC rejects for scope.
- The other five are excluded: row 615's RED (three `$!` re-export tests, and QuickCheck's
  listed splitmix name) and the euler1 timeout.
- Those tests hold 238 programs. GHC 9.14.1 rejects 235 for scope only (GHC-88464/76037). The
  other three need nothing for scope.

## Method

- **Rewrite:** each name GHC reports out of scope gets an explicit import from its home module,
  read from GHC's export data.
  - NoImplicitPrelude programs import the names from Prelude.
  - Internal helpers move to the public APIs: `mapX` to `Map.x` and `setX` to `Set.x`; the
    transformer `bind`/`return` helpers to `(>>=)`/`return`; `both` to `bimap`.
- **Check:** every rewrite is checked with GHC `-fno-code`. Each program then runs before and
  after on the interpreter, LLVM and Cranelift (base CLI at 0f21c8f1).
- **Write-back:** each Rust literal is edited in its own style (raw, real newlines, `\n` escapes,
  continuations). Every edited literal is decoded again and must equal its GHC-verified program,
  inside its own test function.

## Strata

- **A, import-only:** 108 programs in 104 tests. All are GHC-valid, and all are identical on the
  three engines except one. That one, `eval_maybe_to_list_just`, differs only in a non-IO
  `main`'s Cranelift exit code (241 -> 225, each stable). The interpreter value it asserts is
  unchanged.
- **B, helper route:**
  - Data.Map: 53 programs. Identical on the three engines.
  - Data.Set: 25 programs. Behavior changes: `Set.size` reaches no runtime binding.
  - transformers: 12 programs. Behavior changes through the Monad instances.
  - `both` to `bimap`: 1 program. `bimap` has no runtime binding.
  - The last three are not rewritten around their gaps (#30089). Rows 622 (Data.Set),
    623 (MaybeT/ReaderT/StateT Monad routes) and 624 (`bimap`) trace them first.
- **C, beyond scope:** 36 programs stay GHC-invalid once imported. Making them valid needs
  semantic rewrites. They move to row 625, after the route rows (#30089):
  - exceptions (9);
  - NFData ambiguity (8);
  - ExceptT and WriterT (8);
  - ST (5);
  - Text as String, and `append` (2);
  - STM in IO (1);
  - an ambiguous monad under NoImplicitPrelude (1);
  - OverloadedLists (2).

## Checklist

- [x] Census, method and strata; checkpoint #30084.
- [x] Part 1, strata A and B-Map: 161 programs in 157 tests, across 10 test files.
  - Every edited literal is re-verified after rustfmt (161/161).
  - The base files were already rustfmt-clean.
  - The touched tests pass on base: 156 in the driver library, plus 1 in native_error_helpers.
- [x] Re-run on main 4ad98296, over row 606's Eq defaults: 156 + 1 pass.
- [x] Row 620 closes at this 161-program slice (#30089). The non-IO `main` exit code is incidental,
  and its asserted interpreter value holds.
- [x] Land on main and close the progress row (gpt).
  - Fast-forwarded tested tip `beddde3a`.
  - Independent landing checks: the exact 156 touched driver-library tests and the native helper
    control pass; rustfmt, diff-check and gitleaks are clean.
- Out of this row: the Data.Set, transformer and `bimap` routes (rows 622-624), stratum C (row 625),
  and the alias re-export (row 621). Row 615 waits for rows 620-625.
