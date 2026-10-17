<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# If-condition review Chirho

## Scope and ownership

The lane was assigned in #27139 and activated in #27154 under L.J.'s instruction to advance parity. Claude2 authored branch `if-condition-chirho` and held the sole builder through its final packet; GPT reviewed source and frozen-CLI behavior. Main started at `9986576b` (compiler sources `e9c2ab25`). GPT took the canonical DB review lease in #27157 and the bounded main landing lease in #27174. No remote push, release, full driver suite or Cargo ghc_bulk run is authorized by this review.

Source checkpoint #27155: `5b1a6136` extracts ordinary and polymorphic-expected conditionals into `infer_chirho/conditionals_chirho.rs`; `6a629d35` repairs ordinary inference by returning the condition's Bool unifier as part of the expression substitution. The existing typing-integration file is 791 lines after the controls; the focused new module is 127. The oversized dispatcher shrinks.

## Checklist

- [x] Read the source checkpoint, both commit diffs, workflow, owner tasklist, current Git state and substitution-composition contract.
- [x] Verify both extracted helper bodies equal main's bodies after removing comments, whitespace and optional trailing commas, and translating caller span dereferences into by-value helper arguments. Inspect dispatch guards and arguments separately.
- [x] Review the repair's order: Bool unifier after condition inference, then branch substitutions. Both success and error return paths include the revised condition substitution; the polymorphic-expected path is unchanged.
- [x] Independently execute three additional composition controls on GHC, main, extraction and repair: two higher-order condition mismatches reject on GHC/repair but accept on main/extraction; the predicate/result-polymorphism positive prints `('a',3)` on all four. Binary hashes unchanged before/after twelve executions.
- [x] Review the exact final build, new controls, typing unit suite, named neighbours and complete producer probe matrix, including pure-move equivalence to main. Raw exit/stdout/stderr match main on all 36 extraction probes; all 18 accept probes remain byte-identical on the repair; all 18 negative probes reject.
- [x] Verify paired corpus membership, source/binary identities, replay and execution baseline boundaries. Both complete tables match main byte-for-byte; no corpus gain or loss, unique members and exit codes consistent, full source manifests unchanged, CLI hash stable.
- [x] Verify one behavior mutation caught by a named control; source restored and builder explicitly released. The committed reject control fails on the mutant, its positive control still passes; GPT's independent higher-order condition canary also rejects on the repair and accepts on the mutant.
- [x] Make the bounded landing disposition, reread instructions/PRD/Git and fast-forward `9986576b..c4cb9295`, preserving DB/review bytes. Integration row 505 records the unit separately from review row 503.
- [x] Close rows 503/505 and update both measurement provenance headers without changing labels, denominators or membership. Preserve historical rows through 502 and active GPT known-pattern row 504. Audit exact owned bookkeeping scope; the final LANDED broker receipt releases main and explicitly retains DB ownership for row 504.

## Evidence and current disposition

Source review #27157 found no blocker to validation; final packet #27170 and leader disposition #27174 accepted the unit for local integration. Main now carries source tip `c4cb9295`, whose compiler/test sources are identical to measured `6a629d35`. The known Ord-dictionary runtime failure IA04 is preserved and does not imply a typing rejection. No full correctness or green-workspace claim follows.

GPT receipts and added probe sources:

`tmp-chirho/gpt-parity-review-chirho/if-condition-chirho/pure_move_source_review_chirho.json`

`tmp-chirho/gpt-parity-review-chirho/if-condition-chirho/independent_composition_receipt_chirho.json`

The separate instance-method adversarial intake #27156 contains useful metadata, given-scope and substitution leads. GPT measured a mismatch in its recorded full baseline hash and requested correction in #27158. That report's predictions remain prospective. Missing declaration-superclass obligations are a separate unit; they are not silently included in this conditional repair.

## Final evidence and integration

Measured repair CLI SHA-256: `ff4f0eb2b718a1a75ec80591e2f985981e8313756d33d28be26430e6c5a52f84`. Both builds have zero compiler warnings. GPT verified normalized source identity for the extraction and byte-identical raw outcomes on its 36 probes; the repair adds 16 matching reject probes outside the corpus. All 18 positives retain main's exact raw outcomes, including IA04's known runtime failure. The wider guard/retained set comparisons are Claude2's attributed evidence, not a fresh GPT execution.

Actual focused test logs record typing integration 35/35, rigid types 14/14, typing units 354/354 and filtered driver conditional/multi-way-if tests 19/19. Initial shell-wrapper attempts failed before launching Cargo; those launch errors are retained and excluded from the successful test counts. The corrected commands and complete successful logs were inspected. The mutation log records the intended committed reject-test failure and unchanged positive-test pass; GPT independently ran its higher-order negative on frozen repair and mutant `47030bb6`, with stable hashes, seeing exits 1 and 0 respectively.

GPT checked both paired corpus tables against each other and the prior final tables from main, with full source hash manifests, unique relative members, and exactly the expected verdict/exit pairs. All 289 rejections replay; execution remains 28/35 with the same seven failing predicates. The previous default-stack parser SIGABRT is retained as a known baseline red; this typing-only unit does not repair it or claim a fresh parser-suite pass. Both published measurement artifacts retain their quote lines and protocol, with refreshed source/binary/receipt provenance.

Before fast-forwarding, GPT reread the project instructions, PRD resource limits, active review, final packet and Git state; main was an ancestor, the index was empty and only GPT's DB was tracked dirty. The merge preserved the DB and review bytes. Rows 503 and 505 are complete; row 504 remains open for the separately isolated known-pattern source lane. The bookkeeping commit owns only this review, the DB and the two measurement artifacts; compiler/test code remains exactly the tested candidate. No canonical rebuild or remote push.

Builder handoff #27170/#27172 gives Claude the sole slot for the instance-method candidate; its outer-list lifecycle blocker was repaired in `c6cc6593`, and Agy's A11 equality-ordering finding in `43a2e0b4`. Those candidates are separate, not landed or certified by this if review. Claude2 takes the comprehension implicit-let/layout-separator source lane next (#27174), with no Cargo slot. GPT's pattern extraction `ddb700b8` is an unbuilt source checkpoint in the reused GPT worktree and is not part of main.
