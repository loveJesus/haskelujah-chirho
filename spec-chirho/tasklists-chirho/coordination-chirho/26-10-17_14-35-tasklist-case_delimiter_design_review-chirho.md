<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Case-delimiter design review Chirho

Owner: gpt_chirho. Original design proposal: claude2_chirho #27179; the historical baseline and lease at that review are retained below. Source candidate #27238 is now 0c588175 (compiler 8bb224b8) on the landed comprehension main c4e192cc. This is source review only: Claude holds the builder, and no candidate execution or source landing is claimed.

- [x] Inspect the actual alternatives loop, recovery fallback, guard ownership and proposed sibling-module placement.
- [x] Check additional enclosing-expression boundaries on GHC 9.14.1 and the frozen main CLI, retaining exact receipts.
- [x] Give a concrete placement/scope decision, with the comprehension candidate and do-block follow-up kept distinct.
- [x] Record the disposition and preserve the current builder/DB owners.
- [x] Review source candidate 8bb224b8, compare both moved bodies byte-for-byte, and review test-only 0c588175.
- [x] Replace the valid implicit-where residual's pinned failure with an error-or-exact-GHC-output property before the build.
- [ ] Validate both bricks on final main under the later named builder handoff, including behavior identity, focused suites, probe sets, corpus and a meaningful mutant.

## Decision and independent evidence

Placement approved via directed broker #27180 (one delivery): shared implicit-close helper, extracted case-alternatives loop and focused parser shape tests. Keep a separate pure-move brick. Case and lambda-case share the loop. Block ownership must derive from the current opener; explicit braces and a missing opener cannot be treated as implicit blocks. The boundary rule runs between alternatives after body/guard parsing, preserving alternative-owned guards and where clauses. The existing recovery fallback remains outside the declared repair scope.

Independent runs retained 28 exact commands with source SHA256, exit code, stdout/stderr and frozen CLI SHA256 before/after. GHC version measured 9.14.1; CLI ff4f0eb2b718a1a75ec80591e2f985981e8313756d33d28be26430e6c5a52f84 was unchanged. CD07 loses the tuple component on main, CD22 loses the updated second record field, CD12 is rejected by both and CD17 is an unchanged explicit-brace positive. The additional outer-case `of`, guard `->` and view-pattern `->` probes all print 7 on GHC and are rejected by main. They must join the boundary trace and controls; candidate gains remain unmeasured.

A let-in boundary and final record construction/update fields already agree with GHC and are preservation controls. Explicit and implicit alternative-owned where clauses before a tuple comma both print `(7,True)` on GHC and are rejected by main. The implicit declaration block may need its own early close before the case loop sees the comma; the producer must trace that ownership and either cover it in this mechanism or name the separate residual. A where-clause name escaping into another alternative is rejected by both. The first exploratory case_where_scope_chirho.hs is also invalid under GHC and is retained as such, never used as an accept target.

Receipts:
- `tmp-chirho/gpt-parity-review-chirho/case-delimiter-design-chirho/independent_boundaries_receipt_chirho.json` (22 commands).
- `tmp-chirho/gpt-parity-review-chirho/case-delimiter-design-chirho/independent_where_receipt_chirho.json` (6 commands).

Normative reference checked: [Haskell 2010 Report, chapter 10.3, Notes 3 and 5](https://www.haskell.org/onlinereport/haskell2010/haskellch10.html). Early implicit closure requires grammar-valid ownership; an explicit closing brace retains its explicit opener. This is a bounded parser repair proposal, not a claim of a complete implementation of the Report's layout algorithm.

At the original design review, f36a5975 stayed frozen until its comprehension gates finished, and case edits were queued as a separate branch/brick. Those gates and the comprehension landing are now complete. Do-block CD20 and the dollar-section runtime failure stay separate. GPT owns the canonical progress DB. No compiler source or tests were changed by GPT in this review; no Cargo, source landing or push.

## Source disposition and remaining gate

GPT independently verified that 55ce6238's parse_case_alts_chirho and close_implicit_block_here_chirho function bodies are byte-identical to c4e192cc. The new loop records opener ownership, consumes an already-placed owned close first, preserves foreign closes, and applies the approved implicit boundaries between alternatives. Existing alternative parsing consumes its own guard commas/bars and where clause before returning to that boundary. The shared helper's extraction, focused module placement and source mechanism are accepted for queued validation, not as a behavioral pass. Diff hygiene and gitleaks c4e192cc..8bb224b8 pass.

Directed review #27240 required a test correction: GHC's valid implicit-where-before-comma program may still error at the separate declaration boundary, but a success must equal `(7,True)` followed by a newline. Requiring failure would pin the old limitation and obstruct its later repair. Claude2 #27241 supplies test-only 0c588175: it reuses the existing Result-returning runner, accepts an error or that exact output, and leaves the three genuine GHC rejections as failure controls. GPT opened the diff and verified the compiler-source paths are unchanged from 8bb224b8.

The queue remains Claude's method gate, GPT's pattern build once the review findings are resolved, then Claude2's case gate. Before the final case freeze, merge the resulting current main. Required evidence is the pure-move binary's identity, focused parser/driver controls, CD/fixture and retained probe sets, combined explicit/nested-brace ownership and residual outcome, corpus membership and a meaningful mutant. The no-progress fallback, do blocks, dollar section and implicit where-declaration closure remain separate. Neither brick has been built yet.
