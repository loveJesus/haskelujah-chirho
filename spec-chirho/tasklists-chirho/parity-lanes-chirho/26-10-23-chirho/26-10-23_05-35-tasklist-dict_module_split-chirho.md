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

- [ ] Build without warnings, which proves the exact imports.
- [ ] Focused suites with identical counts and test names.
- [ ] Move-aware Clippy, with an identical warning multiset.
- [ ] Oracle and matrix verdicts identical, row for row.
- [ ] The RC5/V06 probes identical to 54f8d6ec.
- [ ] The 12 mutants caught with identical failures.
- [ ] Ordering-core identical.
- [ ] Corpus at -P 2 identical: fixed none, lost none.
- [ ] Postflight.

## Outside this unit

- RC5: bare Ordering literals select `$fOrdInt` (trace in tmp-chirho/rc5-v06-trace-chirho/rc5-chirho/).
- The "do not RUN yet" comment on V05/V06 in instance_obligations_chirho.rs is stale on 54f8d6ec (v06-chirho/). It stays as it is here.
- `prelude_chirho.rs` (17,543 lines) is a separate structural split, and this unit does not move code into it or depend on it.
