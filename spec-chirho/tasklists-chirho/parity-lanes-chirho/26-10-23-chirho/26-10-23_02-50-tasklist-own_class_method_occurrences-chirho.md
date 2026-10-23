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

## U1-rev: where a proof lands (gpt_chirho #28111)

The first combined gate, combined-03a337d4-chirho (frozen CLI 010f7178), went RED at focused,
and that receipt stays as it is. D03 passed. A recursive conditional instance printed
`1:[2,3]`, and LLVM and Cranelift printed a pointer. A context-free `[a]` instance hit
"missing STG binding". Trace 3 then found a contextual ground head printing a `$PAP`.

- [x] `rewrite_chirho/occurrence_rows_chirho.rs` holds the three rules, with unit tests beside
  it:
  - self agreement before any row;
  - no ambient answer for a foreign proof, which restores the canonical route;
  - context-leading rows recorded from the class environment before rewriting, and served
    only with their ground context.
- [x] Both dispatch entries call them. `conditional_context_classes_for_prim_binding_chirho`
  now reads the same rule.
- [x] Controls with the own-class controls: two recursive rows, the `[Bool]` agreement row,
  and three context-leading rows. Each is byte-identical to a trace-3 program, measured as
  GHC / main 926ccd4b / Ord dea2a0b7 / opt-in 03a337d4:
  - recursive (direct, and through a `where`): `1:2:3:[]` / same / same / `1:[2,3]`;
  - `[Bool]` in the body: `1:2:[]bb` / same / same / `1:[2]b`;
  - context-leading: GHC `7`, `[7,7]` and `2`; main and Ord fail loudly with missing STG; the
    opt-in prints `$PAP`.
- [x] Boundaries, not pinned, because main gets them wrong too:
  - a recursive `TreeChirho a` instance: missing STG on all three CLIs;
  - a bare self occurrence `goChirho = describeChirho`: `1:[2,3]` on all three.

## U2: context-free list heads (gpt_chirho #28111)

- [x] `instance_chirho/generic_heads_chirho.rs` gains a list arm:
  - the typed head `[v]` is `[]` over one variable, with the written spelling `[v]`;
  - G1, G2, G4 and G6 are unchanged, so a competing list or variable head means no alias;
  - for the list head alone, a slot without a row is a method the instance genuinely omits,
    and takes the generated stub.
  - Unit tests are in `generic_heads_tests_chirho.rs`.
- [x] Controls in `generic_heads_chirho.rs`, measured as GHC / main / Ord / opt-in:
  - defined `[a]` under map: `[99]` / `[[1]]` / `[[1]]` / missing STG;
  - defined `[a]` direct: `99` / missing STG on all three;
  - competing `[a]` and `[Int]`: `(20,10)` / missing STG on all three, kept fail-closed.
- [x] The direct user-method boundary there closed with the opt-in: `generic` / missing STG /
  missing STG / `generic`. It is renamed and now pinned exact.

## Combined gate

Every run takes a fresh tag and is launched through `run_gate_ord_chirho.zsh` (no pipe), only
under a builder lease handed by name. Each RED receipt stays as it is:
- combined-03a337d4-chirho went RED at focused. U1-rev and U2 above repair it.
- combined-b0ace8e1-chirho (frozen CLI 4934941c) went RED at focused on one core unit
  assertion that still expected a list spine to be refused. The test-only commit d31a034f makes
  it state U2's list head.
- combined-d31a034f-chirho went RED at changed-line Clippy on one collapsible `if` whose line
  U1-rev touched. 29db427b collapses it into a let chain, with no change in behavior.

combined-29db427b-chirho, on tip 29db427b (frozen CLI 54f8d6ec), was GREEN in every stage:

- [x] Preflight: the tip differs from base source 926ccd4b only in the Ord stack's paths, this
  unit's paths and spec-chirho/.
- [x] Build: warning-free, CLI frozen.
- [x] Focused: Ord's suites and named tests, plus this unit's integration and unit tests by
  name, and compile_chirho's LLVM and Cranelift recursive custom-list round trips. D03, the
  landing control, now fails by name where GHC fails.
- [x] Changed-line Clippy since 28924e51, and gitleaks over 28924e51..29db427b.
- [x] Oracle over list_orders, string_lists, higher_order_occurrences and generic_heads, with
  every program as GHC expects. The extractor now reads rustfmt's split `const` (it raised
  KeyError on `NEEDS_CHIRHO`); Ord's files extract identically.
- [x] Matrix: all 24 of the Ord matrix's requirements met.
- [x] Mutations, each caught by its own control:
  - Ord's seven;
  - o1, the opt-in dropped;
  - o2, self agreement and no ambient answer reverted together;
  - o4, a context-leading row called bare;
  - the list-head set, g1 and g2.
- [x] Ordering-core: identical.
- [x] Corpus at -P 2 against the R2 lists: membership, replay and execution as on the base.

## Predictions, measured in combined-29db427b-chirho

- [x] D03 and the three missing-method controls fail by name. The context-free head reaches its
  stub through U2's `[]` alias: under g2 it answers `[[1]]` again. Its key spelling is not
  pinned.
- [x] U1-rev and U2 rows print the GHC answers measured in trace 3. The competing `[a]` and
  `[Int]` heads stay fail-closed, as planned.
- [x] The defined list method answers `[99]`, two types answer `([99],[2])`, the element method
  inside an instance answers `[1005]`, the constrained binding answers `(3,31)`, and the
  self-qualified program answers `([99],5)`.
- [x] The direct call keeps `99`, and cd95568b's two missing-method tests stay green.

## Outside this unit

- Seeded standard classes beyond the standard set, classes imported from another module,
  multi-parameter classes, and the spelling of operator methods.
