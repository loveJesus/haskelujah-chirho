<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# An infix `$` stays an operator through lowering Chirho

## Origin and scope

Row 619.
- **Released** by gpt_chirho in #30406, after row 618 landed. Row 614 passed the `$` on to it.
- **Branch:** `dollar-ownership-chirho`, from main `27dc9540`.
- **Scope:** lowering only. The desugarer's ownership check (row 614) already treats an owned `$` correctly. The change only stops lowering from deciding first, so the design did not widen.
- **Lane:** `tmp-chirho/dollar-ownership-chirho/`, with the probes, the frozen CLIs, the corpus verdicts and the reach.

## The bug

A program's own `$` was silently the Prelude's on every engine.
- Fixity resolution (`resolve_infix_precedence_chirho`) turned every infix `$` into an application by its spelling, in both its branches: the one-operator case and a split in a chain.
- This happened before naming could say which `$` it was. A `$` the program defined, hid from the Prelude or bound in a `where` never reached its binding.

## The change (`crates/haskelujah-parser-chirho/src/lower_chirho/expr_syntax_chirho/fixity_chirho.rs`)

- **Lowering.** Both branches build an infix node for every operator, `$` included. `$` keeps its `infixr 0` fixity.
- **Desugaring, unchanged.** An operator the program owns takes its own binding (row 614's `names_owned_binding_chirho`, which runs before the builtin shortcuts). The Prelude's `$` becomes an application there, as before.
- **Typing, unchanged.** The Prelude's `$` is typed as before: `runST $ do ...` still type-checks.
- **Four parser tests** now assert the infix node where they asserted the application:
  - `bindings_chirho/guard_do_tests_chirho.rs`: `lower_infix_fun_bind_with_signature_and_do_rhs_chirho`;
  - `declarations_chirho/declaration_tests_chirho.rs`: `lower_show_paren_dollar_keeps_composition_rhs_with_char_literal_chirho`;
  - `expr_syntax_chirho/prefix_minus_tests_chirho.rs`: `f $ - 3 ^ 2` reads `(f $ (- (3 ^ 2)))`;
  - `expr_syntax_chirho/expression_tests_chirho.rs`: `lower_nested_dollar_keeps_its_right_association_chirho`, renamed from `lower_nested_dollar_to_right_associated_apps_chirho`.

## Measurements (GHC 9.14.1 -O0; main and the change, on the interpreter, LLVM and Cranelift)

- **5 movers to GHC.** On main each prints the Prelude's answer silently, on all three engines.
  - A hidden Prelude `$` redefined, signed and unsigned: 6 and 6, where GHC prints 7 and 7.
  - A `where`-bound `$`: 6, where GHC prints 7.
  - An `infixr 0` chain of an owned `$`: 7, where GHC prints 610.
  - An owned `$` applied to a `do` block: `once`, where GHC prints `once` twice.
- **2 holds stay GHC on both:** the Prelude's `$` in chains and before `map` (7, 0, 1); sections of an owned `$` (`([7],[20])`).
- **`runST $ do ...` type-checks on both.** Running it fails the same way on both, for a separate reason (below).
- **The corpus.** Both typecheck axes keep every verdict, file by file (`corpus-chirho/`):
  - should_compile: 884 accepted, 54 rejected of 938;
  - should_fail: 515 accepted, 252 rejected of 767.
- **The experiment** (`exp-no-rewrite-chirho`, before the tests were updated): the 5 movers at GHC. The focused suite failed only the 4 parser tests above, 1391 of 1395 passing.

## Findings, pre-existing and separate (`probes-chirho/bisect-chirho/`)

- **`runST` does not run.**
  - On main `27dc9540`'s interpreter, `print (runST (return (42 :: Int)))` prints `$IOActionChirho ($io_execution_6712_chirho 42)` and exits 0. GHC prints 42.
  - Summing a list through an `STRef` prints `$IOActionChirho (...)`, where GHC prints 55. The parenthesized form fails the same way, so it is not the `$`.
  - LLVM and Cranelift refuse that program: `does not support primitive(s): modifySTRef#/2, newSTRef#/1, readSTRef#/1, runST#/1`.
  - The interpreter's answer is silent and wrong; the natives fail loudly.
  - It stays out of this row. The control here checks only that the program type-checks.

## Checklist

- [x] Trace on GHC 9.14.1 and main's three engines before source (`probes-chirho/ghc-chirho`, `main-d1192c01-chirho`, `main-27dc9540-chirho`).
- [x] The change in `fixity_chirho.rs` and the four parser tests. Parser crate 404/404, no warning.
- [x] **Controls:** `crates/haskelujah-driver-chirho/tests/typing_integration_chirho/dollar_ownership_chirho.rs`.
  - It holds the 5 movers and the 2 holds, byte for byte as measured, on all three engines.
  - `runST $ do ...` must compile.
- [x] Both typecheck corpus axes compared file by file: identical.
- [x] The workflow doc: `compiler-pipeline-chirho/operator-fixity-chirho.md`, "The dollar (row 619)", and its DAG.
- [x] **Reach** (`tools-chirho/reach_619_chirho.zsh`, `reach619-r1-chirho`). Each mutation was restored byte-identical (SHA-256).
  - **M1, main's rewrite back in the one-operator branch:**
    - 3 parser tests fail: the nested chain, the `do` right side and `f $ - 3 ^ 2`;
    - the owned table fails, first at `DHiddenDollarSigChirho` on the interpreter;
    - the holds and `runST` pass.
  - **M2, main's rewrite back in the chain branch:**
    - 2 parser tests fail: the `showParen` composition and the nested chain;
    - the owned table fails, first at `DDollarChainChirho` on the interpreter;
    - the holds and `runST` pass.
  - **The restored tree:** 398/398 parser library tests, 3/3 controls.
- [ ] Commit; the broad moved-shape scan.
- [ ] Judge main `27dc9540` against the contract copy; gate r1 on the tip.
- [ ] The tip's probe reports; post to gpt_chirho. Then row 639.
