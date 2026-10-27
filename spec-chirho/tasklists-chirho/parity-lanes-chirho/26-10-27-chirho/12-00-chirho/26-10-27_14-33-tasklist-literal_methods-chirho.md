<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# A literal's method is never a binding of the program's own Chirho

## Origin and scope

Row 660.
- **Found** as row 639's fromInteger transition stop (#30441): an unsigned `fromInteger x = 42 :: Int`, once shadowed, ran to the native timeout.
- **Released** trace-first by gpt_chirho in #30443, on main `220f4c96`. Row 639 stays held, with its E1 patch kept out of these measurements.
- **Widened** in #30446 to the three literal-conversion routes, with the same rule:
  - `fromInteger`;
  - `fromString` under OverloadedStrings;
  - `fromList` under OverloadedLists.
- **Reserved apart:**
  - row 665: ranges, `enumFrom*`;
  - row 666: `do` operations;
  - row 667: `fromListN` and custom IsList instances.
- **Branch:** `literal-from-integer-chirho`, from main `9b40a042`, which is `220f4c96` plus the progress DB.
- **Lane:** `tmp-chirho/literal-from-integer-chirho/`, with the probes, the frozen CLIs, the transition recorder and the reach.

## The bug

The desugarer resolved the method a literal stands for by its name, lexically (`resolve_var_chirho("fromInteger")`, `"fromString"`, `"fromList"`). So a program's own binding of that name, top-level or `where`-bound, captured every literal. GHC's literals use the standard methods unless RebindableSyntax is on.
- **One with a literal in its own body recursed:** the interpreter hit its step limit, and the natives ran to a timeout or died with SIGBUS.
- **A captured `fromString`** printed nothing on the interpreter and died with SIGSEGV on the natives.
- **A captured `fromList`** printed the program's answer silently.
- **A custom IsString literal** took the String instance, because the literal had no occurrence for its proof to reach.

## The change

- **`resolve_syntax_method_chirho`** (`crates/haskelujah-core-chirho/src/desugar_chirho/syntax_methods_chirho.rs`):
  - under ordinary syntax it mints the literal's own occurrence of the method, with the name's one canonical id behind it;
  - under RebindableSyntax it resolves lexically, as GHC defines.
- **The three literal sites** in `desugar_chirho.rs` call it.
  - `resolve_unscoped_var_chirho` now shares two small helpers with it: `canonical_free_id_chirho` and `mint_method_occurrence_chirho`. Its ids are allocated in the same order as before.
  - `desugar_chirho.rs` is 5,428 lines, so the row changes only these call sites and helpers there.
- **What does not change:**
  - The literal's origin, span and role are recorded as before.
  - An occurrence the checker proves nothing for falls back to the canonical id in `finish_chirho`.
  - A call the program writes keeps resolving to the program's binding.

## Measurements (GHC 9.14.1 -O0; main against the candidate, on the interpreter, LLVM and true Cranelift; every run under an external timeout)

- **14 movers reach GHC on all three engines, 42 cells.**
  - **`fromInteger`.** On main all of these are abnormal (step limit; native timeouts or SIGBUS):
    - signed `fromInteger x = 42`, (42,5);
    - `fromInteger n = n P.* 10` at Integer, (40,4);
    - literals only, 3;
    - a `where` namesake, 3;
    - a written call beside a literal, (9,3);
    - NoImplicitPrelude without RebindableSyntax, 21.
  - **`fromString`:**
    - the literal alone, `literal`;
    - a written call beside a literal, 7 and `literal`;
    - a `where` namesake;
    - NoImplicitPrelude;
    - a custom IsString instance, `Name "ab!"`. Main gave the String instance's `Name "ab"` on the interpreter and failed on the natives.
  - **`fromList`:**
    - the literal alone, [1,2,3];
    - a written call beside a literal, [3,2,1] and [4,5,6] (main silently gave [1,2,3] and [6,5,4]);
    - a `where` namesake, [1,2].
- **Holds on both:**
  - **The three RebindableSyntax literals:** 42, `literal!` and [3,2,1].
  - **Ordinary literals:**
    - Int, Integer, Double and defaulted Fractional;
    - negatives, `minBound` and the `2^63` magnitude;
    - a list, an Integer range under `sum`, and a lambda under `map`;
    - a user `Num` instance;
    - ordinary OverloadedStrings and OverloadedLists programs;
    - a program's own `fromRational` and `negate`, which no literal names.
- **Row 639's unsigned twin** stays (5,5) and terminates on all three engines. GHC prints (42,5); row 639 makes it exact.
- **Unchanged, other rows':**
  - a custom IsList literal is refused by the checker (`expected [t4], found Bag`), row 667;
  - ranges, rows 665 and 653;
  - `do` with an own `>>`, row 666;
  - prefix minus at a user `Num` newtype, below.

## Findings, pre-existing and separate

- **`print (-2 :: V)` for a user `Num` newtype.**
  - The interpreter prints `V -20`; GHC prints `V (-20)` (row 646's family).
  - Both natives fail with `Non-exhaustive patterns in function`. Prefix minus emits `negate#` whatever the type.
- **`print [1 .. 3 :: Integer]`** fails on the natives with `show: no native Show instance for a boxed value` (row 632's family?).
- **`([1, 2, 3 :: Int], [1 .. 3 :: Integer])`** fails with `no matching alternative for tag 47` (row 653's family).

## Checklist

- [x] Trace on GHC 9.14.1 and main's three engines before source: #30445; widened #30446.
- [x] The helper and the three call sites. rustfmt is clean and the build has no warning.
- [x] **The desugarer unit tests** in `desugar_chirho/provenance_tests_chirho.rs`:
  - a literal's method is a minted occurrence, not the program's binding, with its provenance, while a written reference keeps the binding;
  - under RebindableSyntax the literal takes the binding.
- [x] **All-engine controls:** the row 660 tables in `dictionary_evidence_chirho/fractional_literals_chirho.rs`, byte for byte as measured (`tools-chirho/gen_controls_660_chirho.py`). The family passes 18/18.
- [x] The workflow doc: `compiler-pipeline-chirho/value-scope-resolution-chirho.md`, "A literal's method is never a lexical binding".
- [x] **Reach** (`tools-chirho/reach_660_chirho.zsh`, `reach660-r1-chirho`).
  - **How each mutant runs.** Each records its route's lead probe under a 10 s external bound; a killed mutant's step limit or timeout is the expected receipt. Both files are then restored byte-identical (SHA-256).
  - **M1, M2, M3: the `fromInteger`, `fromString` or `fromList` site resolves lexically again.**
    - The lead probe hits the interpreter's step limit, and both natives time out at 10 s.
    - The literal-method unit test fails, as does only that route's mover table.
  - **M4: RebindableSyntax no longer keeps the lexical route.**
    - The lead probe prints 21 instead of 42 on all three engines.
    - The RebindableSyntax unit test and holds table fail.
  - **M5: the helper returns the canonical id instead of minting.**
    - The custom IsString literal falls back to main's `Name "ab"`.
    - The literal-method unit test and the existing provenance test fail.
    - The string movers table fails, first at `SCustomIsStringChirho`, and so does the ordinary holds table.
  - **The restored tree:** 3/3 unit tests and 5/5 control tables.
  - **Clippy:** no diagnostic in either changed crate.
- [ ] Commit; judge main against the contract copy; gate r1; the broad scan; post. Then row 639 resumes on this tip.
