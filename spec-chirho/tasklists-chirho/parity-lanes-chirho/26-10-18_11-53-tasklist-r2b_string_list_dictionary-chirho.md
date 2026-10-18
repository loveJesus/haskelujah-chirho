<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# R2 at every entry, with a generated Eq [String] Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB. Branch r2b-o1-combined-chirho:
- R2b on R1/R2 5827b4bf: f4fb9164, 45c55a85, 43f8a995 and 0bf1ada7.
- O1, replayed: 590508ba to d7081c8c.
- The moved-guard let chain: 2fe84fae.
- The generated `Eq [String]`.

No fundep, Char, case, NonEmpty, integral Enum or main work is absorbed. No Cargo lease.

## Source

- [x] R2b: proven_list_occurrence_chirho shared by every occurrence entry, with the proof's engine spelling tried before a caller's agreeing key (#27663; option b). Fifteen controls are GHC-exact.
- [x] O1 replayed with the provenance and union check PASS: every blob equals its source tip, and patch-id 23fcc012 equals O1's.
- [x] First combined gate, combined-d7081c8c-chirho, receipts retained:
  - lint RED on the moved collapsible_if, fixed without suppression in 2fe84fae;
  - matrix RED on A4 only, with all 47 requirements met.
- [x] `Eq [[Char]]` generated for Eq only, with `==#` per String element. Nine controls are GHC-exact. The `eqStr#` comparator mutant is refuted as observationally equivalent.

## Gate

- [ ] Clean-tip rerun under a named slot, in order:
  - build and focused suites;
  - changed-line Clippy since 2f1bdfd6;
  - the oracle over four units;
  - the matrix, with A4 required exact;
  - the O1 rows;
  - mutations: typed-entries 3, string-lists 1, proven-on-r2b 6, ground 10, identity 7 and sections 2, with p3, p6, g4 and q2 declared;
  - the corpus against ground's 7ad703e8 lists.

## Integration

- [ ] GPT's landing disposition. The standalone R2b branch does not carry 2fe84fae or the `Eq [String]` commit.
