<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Type equality fixity Chirho

Row 682, from the row 681 complete corpus-frontier audit. Source base a70a11c7 has the same crate tree as 75b4e3a9. GPT owns this isolated worktree, the sole builder, main and the ledger. No remote push or release.

- [x] Trace T18252's wrong grouping to the absent builtin equality fixity; GHC 9.14.1 `:info (~)` confirms non-associative precedence 4.
- [x] Change only the existing shared builtin fixity entry and add its workflow pointer.
- [x] Add grouping controls in the existing type-operator tests and a new leaf in typing integration (directory 13 to 14 entries, no root target).
- [x] Fresh GHC checks for four exact control strings: two positive proof programs, false equality, and invalid chained equality. Candidate verdicts agree.
- [x] Parser 401/401 and three integration groups pass. The two positive programs execute with GHC's answers on interpreter, LLVM and Cranelift; negative controls remain refused.
- [ ] Workspace check, changed-line Clippy, formatting and diff-check.
- [ ] Candidate corpus comparison on both axes, with source-current frozen baseline and explicit raw/source receipts.
- [ ] Secret scan, exact source review, local landing and scoped result record.

Receipts: `tmp-chirho/equality-fixity-chirho/`. The separate deriving-token prototype is HELD under `tmp-chirho/deriving-token-chirho/`: it checks two corpus files but newly prints wrong explicit-newtype Show output, so its source was restored and is absent from this unit. Its held patch and frozen CLI are retained.

This unit repairs precedence, not general non-associative syntax diagnostics, family injectivity, GADT strictness, deriving strategy semantics or the full GHC corpus.
