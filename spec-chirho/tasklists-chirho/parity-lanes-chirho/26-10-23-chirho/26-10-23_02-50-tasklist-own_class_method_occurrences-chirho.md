<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Own-class method occurrences Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB.

The Ord list unit's gate on the exact replay dea2a0b7 (tag ord-main-dea2a0b7-chirho) went RED at
focused. Its D03 control, `print (map neededChirho [[1 :: Int]])`, printed `[[1]]` where GHC
9.14.1 fails. Trace 2 found the cause outside Ord: a user class method shared one occurrence id
for every use, so the dictionary pass guessed its dictionary from sibling arguments. The frozen
base (926ccd4b) and Ord CLIs agreed on all eight trace programs.

Branch `own-class-method-occurrences-chirho` sits on dea2a0b7 (gpt_chirho #28096): a separately
named causal unit, stacked after the full Ord source because a proven occurrence needs brick 2's
stub ABI. The fresh combined gate is the only landing authority. Ord's five commits, their 11
replay proofs and the first RED receipt stay unchanged.

## Source

- [x] `method_occurrence_names_chirho` beside the join in `evidence_join_chirho.rs`: the standard
  set plus the methods of single-parameter classes the module declares, minus the five INV-001
  names. `lib.rs` keeps a thin call, and the standard-set const moved with the helper.
- [x] Controls in `higher_order_occurrences_chirho.rs` (the directory keeps its entries): six
  GHC-exact programs (one self-qualified, `Main.neededChirho`), three that must fail by name,
  and four lexical shadows.
- [x] Boundary unit test `occurrence_names_chirho.rs`: own classes in; multi-parameter classes,
  INV-001 names, imported classes and seeded methods out.
- [x] The focused workflow `language-features-chirho/own-class-method-occurrences-chirho.md`. The
  shared dictionary-evidence workflow is an Ord path and stays unchanged (#28092).
- [x] GHC 9.14.1 receipts, with `runghc` as the oracle stage runs it: all 28 programs of the test
  file hold, 25 byte-identical and 3 rejected with "No instance nor default method".

## Combined gate

A fresh tag, launched through `run_gate_ord_chirho.zsh` (no pipe), only under a builder lease
handed by name.

- [ ] Preflight: the tip differs from base source 926ccd4b only in the Ord stack's paths, this
  unit's paths and spec-chirho/.
- [ ] Build: warning-free, CLI frozen.
- [ ] Focused: Ord's suites and named tests, plus this unit's three integration tests and three
  unit tests by name. D03 is the landing control.
- [ ] Changed-line Clippy since 28924e51, and gitleaks over 28924e51..tip.
- [ ] Oracle over list_orders, string_lists and higher_order_occurrences. The extractor now reads
  rustfmt's split `const` (it raised KeyError on `NEEDS_CHIRHO`); Ord's files extract identically.
- [ ] Matrix: the Ord matrix unchanged.
- [ ] Mutations: Ord's seven and o1 (the opt-in dropped), each caught by its own control.
- [ ] Ordering-core.
- [ ] Corpus at -P 2 against the R2 lists: membership, replay and execution.

## Predicted, unmeasured until the combined gate

- D03 and the three missing-method controls fail by name; the context-free head's key spelling
  is not pinned.
- The defined list method answers `[99]`, two types answer `([99],[2])`, the element method
  inside an instance answers `[1005]`, the constrained binding answers `(3,31)`, and the
  self-qualified program answers `([99],5)`.
- The direct call keeps `99`, and cd95568b's two missing-method tests stay green.

## Outside this unit

- Seeded standard classes beyond the standard set, classes imported from another module,
  multi-parameter classes, and the spelling of operator methods.
