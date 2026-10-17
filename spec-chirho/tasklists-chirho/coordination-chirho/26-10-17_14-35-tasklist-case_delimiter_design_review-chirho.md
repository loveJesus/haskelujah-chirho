<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Case-delimiter design review Chirho

Owner: gpt_chirho. Advisory proposal: claude2_chirho #27179. Current main: 6e543a02; compiler sources match frozen ff4f0eb2. Comprehension candidate f36a5975 is frozen, unbuilt. claude_chirho keeps the builder slot; no Cargo or source edits in this review.

- [x] Inspect the actual alternatives loop, recovery fallback, guard ownership and proposed sibling-module placement.
- [x] Check additional enclosing-expression boundaries on GHC 9.14.1 and the frozen main CLI, retaining exact receipts.
- [x] Give a concrete placement/scope decision, with the comprehension candidate and do-block follow-up kept distinct.
- [x] Record the disposition and preserve the current builder/DB owners.

## Decision and independent evidence

Placement approved via directed broker #27180 (one delivery): shared implicit-close helper, extracted case-alternatives loop and focused parser shape tests. Keep a separate pure-move brick. Case and lambda-case share the loop. Block ownership must derive from the current opener; explicit braces and a missing opener cannot be treated as implicit blocks. The boundary rule runs between alternatives after body/guard parsing, preserving alternative-owned guards and where clauses. The existing recovery fallback remains outside the declared repair scope.

Independent runs retained 28 exact commands with source SHA256, exit code, stdout/stderr and frozen CLI SHA256 before/after. GHC version measured 9.14.1; CLI ff4f0eb2b718a1a75ec80591e2f985981e8313756d33d28be26430e6c5a52f84 was unchanged. CD07 loses the tuple component on main, CD22 loses the updated second record field, CD12 is rejected by both and CD17 is an unchanged explicit-brace positive. The additional outer-case `of`, guard `->` and view-pattern `->` probes all print 7 on GHC and are rejected by main. They must join the boundary trace and controls; candidate gains remain unmeasured.

A let-in boundary and final record construction/update fields already agree with GHC and are preservation controls. Explicit and implicit alternative-owned where clauses before a tuple comma both print `(7,True)` on GHC and are rejected by main. The implicit declaration block may need its own early close before the case loop sees the comma; the producer must trace that ownership and either cover it in this mechanism or name the separate residual. A where-clause name escaping into another alternative is rejected by both. The first exploratory case_where_scope_chirho.hs is also invalid under GHC and is retained as such, never used as an accept target.

Receipts:
- `tmp-chirho/gpt-parity-review-chirho/case-delimiter-design-chirho/independent_boundaries_receipt_chirho.json` (22 commands).
- `tmp-chirho/gpt-parity-review-chirho/case-delimiter-design-chirho/independent_where_receipt_chirho.json` (6 commands).

Normative reference checked: [Haskell 2010 Report, chapter 10.3, Notes 3 and 5](https://www.haskell.org/onlinereport/haskell2010/haskellch10.html). Early implicit closure requires grammar-valid ownership; an explicit closing brace retains its explicit opener. This is a bounded parser repair proposal, not a claim of a complete implementation of the Report's layout algorithm.

Timing and leases: f36a5975 stays frozen until its comprehension gates finish. Case edits then proceed on a separate branch/brick. claude_chirho retains the builder until the already scheduled named handoff to claude2_chirho. Do-block CD20 stays a separate follow-up; the dollar-section runtime failure stays a separate lead. GPT owns the progress DB (504 still active). No compiler source, tests or compatibility measurement artifacts changed during this review; no Cargo, source landing or push.
