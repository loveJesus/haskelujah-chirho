<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# dict_chirho module split Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (row 528).

This is a structural unit, done before any RC5/RC6 semantic edit (gpt_chirho #28132, GO #28141). It covers `dict_chirho/rewrite_chirho.rs` (4730 lines) and `dict_chirho/mod.rs` (1613 lines), and the contract is tmp-chirho/dict-split-chirho/split-contract-chirho.md (sha256 6c6823ca).

The branch `dict-module-split-chirho` starts from exact main d1b817bb. Its crates tree, 9e388c04, is 29db427b's, so combined-29db427b-chirho is the behavioral baseline. Nothing in the unit changes behavior.

## Source

- [x] C2 (007f476b): mod.rs's inline tests move to `dict_chirho/dict_pass_tests_chirho.rs` (mod.rs goes from 1613 to 895 lines). The declaration `#[cfg(test)] #[path = "dict_pass_tests_chirho.rs"] mod tests_chirho;` keeps every test name.
- [x] Amendment B (gpt_chirho #28145), the only formatter rule for C2:
  - The tests file is byte-for-byte rustfmt 1.8.0-stable (254b59607d 2026-01-19), edition 2024, default config, applied to the four-line header plus an exact four-space de-indent of base mod.rs lines 895-1612.
  - The proof rebuilds that input, runs the pinned rustfmt and requires the exact bytes.
  - It also requires 4 reflow hunks, 722 -> 715 lines and 4161 -> 4158 tokens: exactly 3 trailing commas removed, at base lines 1009, 1016 and 1440. Comments and the 15 test names are identical.
- [x] C3 (ca7575c6): rewrite_chirho.rs (4730 -> 734 lines) keeps the pass entries and the shared recognizers, and seven private children take the rest:
  - `type_keys_chirho.rs`, the RC5 seam, with `infer_type_key_chirho` from mod.rs;
  - `numeric_defaults_chirho.rs`;
  - `method_args_chirho.rs`;
  - `call_keys_chirho.rs`;
  - `head_dispatch_chirho.rs`;
  - `method_refs_chirho.rs`;
  - `dict_params_chirho.rs`.
- [x] C3's only shape changes:
  - exactly 36 `pub(super)` qualifiers, each on a method with a measured cross-file caller;
  - exact imports per file;
  - the module-wide `#![allow(unused_imports)]` removed.
  Method bodies are byte-identical, and mod.rs goes from 895 to 515 lines.
- [x] Amendment B2 (gpt_chirho #28148), the only formatter rule for C3:
  - The proof rebuilds the pre-format C3 from exact C2 in a fresh temp dir and proves the original contract there. Each committed file must equal the pinned rustfmt's output for its pre-format file.
  - The delta must be exactly four signature hunks, where `pub(super)` pushes a one-line signature past 100 columns: `concrete_binder_type_key_chirho` (type_keys), `first_value_lambda_binder_id_chirho` and `first_value_lambda_param_expects_list_chirho` (call_keys), and `has_direct_return_or_pure_head_chirho` (head_dispatch).
  - Those hunks add +4 `,` tokens before `)`, with line growth 690 -> 693, 417 -> 421 and 625 -> 628 (+10).
  - Every other split file is a rustfmt no-op, and every method body is unchanged.
- [x] C4: the one path node in `monadic-dispatch-chirho.md`, and this tasklist's ticks.
- [x] The mechanical proof, `prove_split_chirho.py`, GREEN after C2 (11/11) and after C3 (32/32), with receipts in tmp-chirho/dict-split-chirho/. rustfmt, `diff --check` and gitleaks are clean, all without Cargo.

## Gate

The gate runs only under a builder lease handed by name, against combined-29db427b-chirho:

- [x] Build without warnings, which proves the exact imports.
- [x] Focused suites with identical counts and test names.
- [x] Move-aware Clippy, with an identical warning multiset.
- [x] Oracle and matrix verdicts identical, row for row.
- [x] The RC5/V06 probes identical to 54f8d6ec.
- [x] The 12 mutants caught with identical failures.
- [x] Ordering-core identical.
- [x] Corpus at -P 2 identical: fixed none, lost none.
- [x] Postflight.

### Measured: dict-split-0879c0db-chirho

The builder was handed by name in #28156 and returned in #28158. The tag is GREEN on exact tip 0879c0db, with 23 GREEN ledger rows and 0 RED.

- Frozen CLI: c0931c7499c55139665ef49be1ba34f70be761ed449a8c7b41604ed17bc48f7d. Receipt digest: 2429ea5c95c340d1713068627d031c536f603ed0b95ab66af50052896c32fbc4. Launcher: 4fe61db1efc2beeab1436a7c24d816c3e77600eb51ef68bc5317a424b6b1863e.
- Preflight: the 12 contract paths, both proofs rerun (c2 11/11, c3 32/32), and all 16 tools and 6 prior receipts at their pins.
- Build: 0 warnings with the module-wide allow removed.
- Focused: 148, 56, 28, 9 and 479 passed (1374 filtered out), with the receipt's exact counts and test-name sets.
- Clippy: the same 395 warnings as the receipt, matched through the move, and none on scaffolding. gitleaks scanned 4 commits and found no leaks.
- Oracle (4 files), matrix (24 requirements met) and probes: every row identical to the receipt on both CLIs. The probes follow rule #28152.
- Mutations: 7/7, 3/3 and 2/2 caught, with baselines, verdicts and new_failures identical to the receipt. The restore was exact: all 8 mutated files hash to their tip blobs.
- Ordering-core: both rc 0, both print LT, and `$prim_Ord_compare_Ordering` is IDENTICAL (16 lines).
- Corpus at -P 2: byte-identical passes. Membership, replay and execution are identical, with 28/35 passing, fixed none and lost none.
- Postflight: the frozen CLI, the gate script, all 16 tools, every prior receipt, the trace, the probes and the base CLI are unchanged. No cargo, rustc, clippy or CLI process was alive.

## Outside this unit

- RC5: bare Ordering literals select `$fOrdInt` (trace in tmp-chirho/rc5-v06-trace-chirho/rc5-chirho/).
- The "do not RUN yet" comment on V05/V06 in instance_obligations_chirho.rs is stale on 54f8d6ec (v06-chirho/). It stays as it is here.
- `prelude_chirho.rs` (17,543 lines) is a separate structural split, and this unit does not move code into it or depend on it.
