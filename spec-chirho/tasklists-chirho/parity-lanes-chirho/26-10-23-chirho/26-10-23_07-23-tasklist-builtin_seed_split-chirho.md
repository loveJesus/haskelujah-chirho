<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Builtin seed split Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (unit 529).

This is a structural unit that comes before RC5 (gpt_chirho #28163, approved in #28165). `seed_builtins_chirho` is 13,177 lines (infer_chirho.rs:7834-21010) inside a 27,710-line file. It moves into `infer_chirho/builtins_chirho/` under the placement contract tmp-chirho/seed-split-chirho/SEED-PLACEMENT-CONTRACT-chirho.md (sha256 f6479882).

The branch `builtin-seed-split-chirho` starts from exact main 1ac3fd2e36864b657869388f5ca3692090acae80, whose crates tree is e3e1ca9c. Nothing in this unit changes behavior.

## Source

- [x] C2 (aa911b3c): the builtins hierarchy and the parent's two lines, together in one structural commit. The parent goes from 27,710 to 14,534 lines; there are 21 new files, the largest 974 lines.
  - Hub `builtins_chirho/mod.rs`; `constructors_chirho.rs` (True/False/Nothing/Just); `transformer_types_chirho.rs` (the four hoisted builders).
  - Groups `core_chirho/`, `effects_chirho/`, `classes_chirho/` and `surface_chirho/`, holding part_01 to part_14. Each part is a contiguous original slice.
- [x] Binding order: the env mutation sequence is byte-for-byte the original's. Module constructors still override the seed.
  - Proven structurally: 15 sections are verbatim slices in call order, and 935 bound names keep their order. The gate's T17 row checks the override's behavior.
- [x] Rule B3, the only formatter rule: `transformer_types_chirho.rs` equals pinned rustfmt applied to its pre-format file, and each builder's parameters and body are token-identical to its closure's. Every other file is a rustfmt no-op.
  - The measured delta is three trailing commas, one closing each signature rustfmt wrapped: mk_state_t, mk_reader_t and mk_maybe_t.
- [x] The mechanical proof, `prove_seed_split_chirho.py`, then rustfmt, `diff --check` and gitleaks, all without Cargo.
  - The proof is GREEN, 18/18 on aa911b3c. All 22 touched Rust files are pinned-rustfmt no-ops, `diff --check` is clean, and gitleaks finds nothing in this unit's commits.
- [x] C3, docs:
  - `spec-chirho/workflows-chirho/compiler-pipeline-chirho/builtin-type-environment-chirho.md`, a Mermaid DAG running seed → module-shadow removal and rebinding → occurrence typing and defaulting. The hub's comment points to it.
  - This tasklist's ticks.

## Gate

The gate runs only under a builder handed by name. Its baseline is dict-split-0879c0db-chirho, plus a typing baseline measured at 1ac3fd2e.

- [ ] Build with 0 warnings. Focused suites plus the typing lib tests, with exact counts and name sets.
- [ ] Move-aware Clippy, typing included. gitleaks covers exactly this unit's commits.
- [ ] Oracle, matrix and the RC5/V06 probes are row-identical.
- [ ] The RC5 placement rows T01-T17, N01-N04 and R1-R9 are row-identical, base against candidate. Heap normalization applies only on R1, R5 and R9. T17 checks the module override, T16 that qualification stays exact. No verdict may move.
- [ ] Mutations, ordering-core, and the corpus at -P 2 are identical. Postflight passes.

## Outside this unit

- RC5 itself: `LT`/`EQ`/`GT` become `Ordering` in `constructors_chirho.rs` after this lands, with the #28162 controls and mutants.
- The parent's remaining debt: `impl InferCtxChirho` (6,770 lines) and the inline `mod tests_chirho` (6,066 lines).
- `:|`, rejecting unbound constructors in general, Eq/Ord defaulting without Num, and T15.
