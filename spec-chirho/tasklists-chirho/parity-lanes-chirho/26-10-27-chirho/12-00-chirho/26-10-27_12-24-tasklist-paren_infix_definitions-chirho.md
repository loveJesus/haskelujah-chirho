<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# A parenthesized function left side defines its operator Chirho

## Origin and scope

Row 618.
- **Released** by gpt_chirho in #30364, after row 641.
- **Trace and design posted** in #30367 and approved in #30368. The scope is the `funlhs` distinction in `cst_parser_chirho/bindings_chirho.rs` only; lowering stays untouched.
- **Branch:** `paren-infix-definitions-chirho`, from main `9421c5d9`.
- **Lane:** `tmp-chirho/paren-infix-definitions-chirho/`, with `trace-notes-chirho.md`, the probes and the findings.

## The bug

`(f . g) x = g (f x)` defined nothing named `.`, so every use failed as an `unbound variable`.
- `starts_pat_bind_chirho` called a `(` head with a name after it a pattern binding.
- The pattern parser cannot read `f . g`.
- The declaration broke into error nodes, a stray `g` and a binding of `x`. Lowering dropped the error nodes silently.

## The rule (`crates/haskelujah-parser-chirho/src/cst_parser_chirho/bindings_chirho.rs`)

- **`starts_parenthesized_funlhs_chirho`** recognizes Haskell 2010's `( funlhs ) apat { apat }`.
  - The inner `funlhs` is `pat varop pat`, or recursively another parenthesized one (`peek_after_parenthesized_funlhs_chirho`).
  - A `varop` is a variable operator that is not a prefix pattern operator, or a backticked variable name (`peek_after_varop_chirho`). It is never a constructor operator.
  - At least one argument must follow the `)`.
- **It is checked in two places:** `starts_pat_bind_chirho`'s `(` branch and `parse_fun_bind_chirho`'s `(` head.
- **`parse_parenthesized_funlhs_chirho`** lays the left side out as one flat run in the Match. Lowering already reads that run: `extract_fun_name_from_match_chirho` takes the operator after the first operand, and `lower_match_arm_chirho` takes every pattern in order.

## Measurements (GHC 9.14.1 -O0; main and the change, on the interpreter, LLVM and Cranelift)

- **9 movers to GHC.** On main all are errors.
  - `(f . g) x`, signed and unsigned, 12 and 12;
  - a new operator 19; a backticked name 15; `where`-bound 12;
  - two arguments 20; nested 20; an instance method with constructor operands 12; constructor operands (6,20).
- **3 holds stay GHC:**
  - the prefix form, 12;
  - parenthesized expressions, (8,21,-12);
  - parenthesized pattern bindings with a tuple and a constructor operator, 1/2/5.
- **GHC rejects `(a <+> b) = …`** ("Parse error in pattern"). It stays rejected here, as an unbound `<+>`.
- **The corpus.** The CST changes in 3 of 3,864 tracked `.hs` files: `read026.hs`, `unicode/utf8_024.hs` and `unicode/brackets.hs`. No verdict moves.
  - `read026`'s `(m1 <> m2) a1 = …` and `utf8_024`'s `(f ∘ g) x = …` now define their operators.
  - `utf8_024` stays rejected for a titlecase letter `ǅ` read as a type variable (line 182), on main as here.
  - `brackets.hs` is Arrows notation, which neither tree parses; only its error recovery regroups.

## Findings, pre-existing and separate (`probes-chirho/findings-chirho/`)

- **Row 649, reserved by #30368:** showing some tuples fails at runtime with `no matching alternative for tag 47`.
  - `print (3 :: Int, subtract 1 4)` fails.
  - A top-level `(a, b)` binding fails under `print (a, b)` but works in a `where`.
  - Printing the components one by one works.
- **A program's own `<.>` loses to a builtin one.**
  - `a <.> b = a * 10 + b` with a signature prints `False`; GHC prints 12.
  - A four-argument `<.>` fails with `primop GtIntChirho: expected Int#, got (…, …)`.
  - The same programs with `<%>` print GHC's answers.
  - The pointers: `<.>` is listed in `haskelujah-naming-chirho/src/iface_chirho.rs`, and in the builtin fixity table (`lower_chirho/expr_syntax_chirho/fixity_chirho.rs`).
  - The probes use `<%>` so they stay inside this row.
- **For row 615's builtin-scope inputs (#30368):** a Seq-typed `|>` is in main's scope (`type mismatch: expected Seq`), and GHC's Prelude has no `|>`.

## Checklist

- [x] Trace on GHC 9.14.1 and main's three engines before source: #30367; approved #30368.
- [x] The rule in `bindings_chirho.rs` (986 lines). Parser crate 404/404, no warning.
- [x] **Controls.**
  - Parser unit tests: `cst_parser_chirho/tests_chirho/paren_funlhs_tests_chirho.rs`.
    - The flat, nested, backticked and constructor-operand Match runs, and the operator and pattern count each lowers to.
    - The zero-argument, constructor-operator and tuple heads never define their operator.
  - All-engine controls: `typing_integration_chirho/paren_infix_definitions_chirho.rs`, with the 12 programs byte for byte as measured and the rejection.
- [x] **Reach** (`tools-chirho/reach_618_chirho.zsh`, `reach618-r2-chirho`). Each mutation was restored byte-identical (SHA-256), and the restored tree passes everything.
  - **M1, no parenthesized `funlhs` recognized:**
    - The flat-run unit test fails.
    - Both mover tables fail, first at `PDotSigChirho` and `PTwoExtraArgsChirho`.
    - The holds and the rejection pass.
  - **M2, no argument required after the `)`:**
    - The no-definition unit test fails.
    - The rejection control fails: "GHC rejects this program; we accepted it".
    - Everything else passes.
  - **r1 is void.** Its default tag list was one zsh word, so nothing ran; it is fixed.
- [x] The workflow doc: `pattern-matching-chirho.md`, "A parenthesized function left side".
- [ ] Commit; broad moved-shape scan.
- [ ] Judge main against the contract copy; warm gate; record.
- [ ] Land on main and close progress row 618 (gpt). Then row 619, then row 639.
