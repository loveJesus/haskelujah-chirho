<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Builtin seed split Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB (unit 529).

This is a structural unit that comes before RC5 (gpt_chirho #28163, approved in #28165). `seed_builtins_chirho` is 13,177 lines (infer_chirho.rs:7834-21010) inside a 27,710-line file. It moves into `infer_chirho/builtins_chirho/` under the placement contract tmp-chirho/seed-split-chirho/SEED-PLACEMENT-CONTRACT-chirho.md (sha256 f6479882).

The branch `builtin-seed-split-chirho` starts from exact main 1ac3fd2e36864b657869388f5ca3692090acae80, whose crates tree is e3e1ca9c. Nothing in this unit changes behavior.

## Source

- [x] C2 (aa911b3c): the builtins hierarchy and the parent's two declarations plus their blank separator, together in one structural commit. The parent loses 13,179 lines and gains those 3, going from 27,710 to 14,534 lines; there are 21 new files, the largest 974 lines.
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

- [x] Build with 0 warnings. Focused suites plus the typing lib tests, with exact counts and name sets.
  - r2: 0 warnings. Each suite matched its baseline's exact name set:
    - core-lib 148;
    - dictionary_evidence 56;
    - instance_obligations 28;
    - io_actions 9;
    - driver-filtered 479, with 1374 filtered out;
    - typing-lib 354.
- [x] Move-aware Clippy, typing included. gitleaks covers exactly this unit's commits.
  - r2: the short and long forms were judged separately, and both were exact. Counts as tip/base, short form then long form:
    - core: short 130/130, long 4/4;
    - driver: short 375/375, long 87/87;
    - haskelujah: short 353/353, long 78/78;
    - typing-rooted: short 56/56, long 0/0.
  - No error and no glue diagnostic. gitleaks scanned exactly the 4 unit commits and found no leaks.
- [x] Oracle, matrix and the RC5/V06 probes are row-identical.
  - r2: all 4 oracle units matched, the matrix met its 24 requirements, and the 12 probes passed under the #28152 rule.
- [x] The RC5 placement rows T01-T17, N01-N04 and R1-R9 are row-identical, base against candidate. T17 checks the module override, T16 that qualification stays exact. No verdict may move.
  - The heap rule was corrected by measurement (#28171). The plan said "only R1, R5 and R9", but runs of one frozen binary disproved that (self-variation-c0931c74-chirho, 151ca265).
  - Normalization now applies only to the runtime stderr of R1, R5, R9, T04, T10, T11 and T12, at exact counts 1/1/2/1/1/2/2.
  - r2: all 30 rows were identical in `check` and `run`. The base reproduced all 54 c0931c74 trace receipts, T16 and T17 stayed EXACT, and no verdict moved.
- [x] Mutations, ordering-core, and the corpus at -P 2 are identical. Postflight passes.
  - r2 mutations caught 7/7, 3/3 and 2/2 with identical new_failures, and the restore was exact.
  - ordering-core was IDENTICAL at 16 lines.
  - The corpus passed 28/35, the same as before: fixed none, lost none.
  - The hardened postflight was GREEN.

### Gate runs

Receipts live in tmp-chirho/seed-split-chirho/gate-chirho/, untracked.

- **v1, seed-split-5576cc0a-chirho: RED at lint** (#28176). Nothing after lint ran.
  - Before it, build, base-typing, the six focused suites and gitleaks were GREEN.
  - The typing Clippy pair counted 60 base warnings against 56 tip.
  - The four missing warnings are in unchanged dependencies: diagnostics render_chirho.rs:113 and :114, diagnostics lib.rs:297, and ast expr_chirho.rs:327. The cold base worktree printed them in short form. In the warm tip, Cargo replayed them from cache in long form, which the v1 judge did not parse.
  - Typing-rooted warnings were equal, 56/56.
  - Preserved receipts:
    - run f0a2a60b7fcb51c99b84f658ebf6525d4b6db4352852f9454ebfb42e7a93bc79;
    - launcher 7e240fd7cac24a5f907940847758cc72e0829f92ba132ca3896d6e0861b9535c;
    - ledger d9b2237cbc51d8ffb045f9aa0747acd92da2014a60921a76748a9aa9484c48f0;
    - trace red-trace-5576cc0a-chirho/ 9f7991ea30c773f0a58cd892ea4e518d04b4c041fb5a7ca24bed67f955531fc8.
- **r2 judge boundary** (#28177, tooling only):
  - Short and long forms are judged separately and never merged, so a diagnostic that changes form is RED.
  - The typing pair judges only diagnostics rooted under crates/haskelujah-typing-chirho/. The receipt pairs judge every crate.
  - Errors in either form stay global, and a diagnostic of either form on new glue is RED.
  - Proofs: selftest 41/41, negative controls 25/25.
- **r2, seed-split-5576cc0a-r2-chirho: GREEN, 11/11 stages** (#28184). A fresh run from build, 08:40-09:26 EDT.
  - Ledger 121134ad3d6e0739c63c18a91643681970ba88ef56b6db458541e8fd1feeb231: 26 lines, all GREEN.
  - Exit codes 39bb44f2e3612db3374e4214518b892ef11f469139fd93faa9bcd412ce6758bb.
  - Run directory b76ade14f05057993403ab58759db2078107417fa64edeb7ac147ac407bc253f (12,936 files).
  - Launcher d996e6c80391cf68a00bc6ccaf1c35becd8407efcf51750755abfa8775ade6fa.
  - Frozen CLI 2ddfca2589e2b25a7d9b598112cc87446e1a925947767b76f37920613e76c3c8.
  - Tool pins:
    - gate 66f1347fad5df10e8cbf415e0bda0da9875b34bcfbcbd92b48852146249fe70f;
    - judge abe62fa40461be4f43d4590d8c7a7f1a19c318402f77a5df55742c1e02810c45;
    - orchestrator 73fa1058c8ee340e3f8b4e31d4d1c1c836589f87db048b4cec6656b5d1c1273a.
  - Tooling receipts:
    - r2: 9a747d055cae05d1883aea8234018c51d1b061dd86a15b5c1ad800a70ec891ce, directory 54d72c96e00d7a8e07cc4651197b4a52dd4c5f69f8208beecbc392500938e6ce;
    - v1: a71c32607403c9638b827395c0d547395050671751fdb172bab45eb9e128131b.
  - The base worktree haskelujah-seed-base-chirho stays detached at 1ac3fd2e, with crates tree e3e1ca9c, and clean.

## Outside this unit

- RC5 itself: `LT`/`EQ`/`GT` become `Ordering` in `constructors_chirho.rs` after this lands, with the #28162 controls and mutants.
- The parent's remaining debt: `impl InferCtxChirho` (6,770 lines) and the inline `mod tests_chirho` (6,066 lines).
- `:|`, rejecting unbound constructors in general, Eq/Ord defaulting without Num, and T15.
