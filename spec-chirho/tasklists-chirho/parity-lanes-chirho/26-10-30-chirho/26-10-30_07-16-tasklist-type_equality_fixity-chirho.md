<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Type equality fixity Chirho

Row 682, from the row 681 complete corpus-frontier audit. Source base a70a11c7 has the same crate tree as 75b4e3a9. GPT owns this isolated worktree, the sole builder, main and the ledger. No remote push or release.

- [x] Trace T18252's wrong grouping to the absent builtin equality fixity; GHC 9.14.1 `:info (~)` confirms non-associative precedence 4.
- [x] Change only the existing shared builtin fixity entry and add its workflow pointer.
- [x] Add grouping controls in the existing type-operator tests and a new leaf in typing integration (directory 13 to 14 entries, no root target).
- [x] Fresh GHC checks for four exact control strings: two positive proof programs, false equality, and invalid chained equality. Candidate verdicts agree.
- [x] Parser 401/401 and three integration groups pass. The two positive programs execute with GHC's answers on interpreter, LLVM and Cranelift; negative controls remain refused.
- [x] Workspace check is warning-free; changed-file Clippy has no diagnostics, formatting and diff-check pass.
- [x] One complete candidate pass over both axes, compared with the retained complete source-current baseline. The only movers are T18252 and T24845a, both REJECT to ACCEPT; should_fail is unchanged member for member. No abnormal result or fixture change occurred. Published paired artifacts await the combined declaration-validity candidate.
- [x] Two-commit secret scan, exact source review and local landing as fbef8a56 + 6ffe8072. Crate tree 8b4a5a175ed94322646c816a526037547e60f318 is byte-identical to the tested source/test tip. Row 682's ledger is closed at that scope; no push or release.

Receipts: `tmp-chirho/equality-fixity-chirho/`. The separate deriving-token prototype is HELD under `tmp-chirho/deriving-token-chirho/`: it checks two corpus files but newly prints wrong explicit-newtype Show output, so its source was restored and is absent from this unit. Its held patch and frozen CLI are retained.

This unit repairs precedence, not general non-associative syntax diagnostics, family injectivity, GADT strictness, deriving strategy semantics or the full GHC corpus.

## Gate record

Source 0c2b0db2 has the only production change. The later test extension adds GHC-rejected unrelated-sum and zero-product implications; GHC, the baseline and the candidate reject both. The three integration groups pass again after this extension. The two executable positive programs agree with GHC on the interpreter, LLVM and true Cranelift. Parser tests pass 401/401. The workspace all-target check has no warning. Existing workspace Clippy debt remains; the parser/driver all-target JSON contains 329 messages and 293 unique primary diagnostics, with none in the four changed Rust files. The first changed-line scanner call was void because its output directory was missing; the repaired call succeeds.

The frozen gated CLI is `tmp-chirho/equality-fixity-chirho/haskelujah-gated-chirho`, SHA-256 `83016e3cb7b2a67de4e8d781ccc11667411787f72af31d177d40c1214c4f7018`. `corpus-chirho/comparison-chirho.json` names its complete inputs and exact movers. The retained baseline is SHA-256 `e60d256fc0e66b7621910e186756cc67829d6c0c174a456ebd2150e47925b077`; it was not rerun for this one-pass comparison.

An arithmetic control assumption was corrected before committing tests: fresh GHC accepts the well-formed subtraction implication `(a - b ~ a - c) => Proxy a -> Proxy b -> Proxy c -> Proxy c`, while both our baseline and candidate reject it. It is an unchanged positive solver gap, never a rejection control. The initial arity-invalid version is void evidence. Existing `plus_interaction` already handles equal-left/equal-right addition cancellation; this unit makes the two corpus programs reach the existing solver rather than adding cancellation.
