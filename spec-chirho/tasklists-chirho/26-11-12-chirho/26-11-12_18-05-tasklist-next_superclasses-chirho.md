<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Scope-owned superclass dictionaries

Owner: HASKELUJAH/gpt_chirho, unit 748; Claude2 is the bounded source reviewer. Base 795477ab6243b831c3cd91df3eea35264465b2a8. Reuse the same worktree/target. Main compiler crates stay unchanged. Pre-beta and reversible; GHC is a reference, not an installed dependency.

Implement the superclass part of design C as a running vertical. Class contexts are source classes on the class's own Type-kinded variable. Derived dictionaries are owner-local definitions, never caller parameters. Projection starts only from parameter/derived Givens, never solved Wanted evidence. Use one dictionary layout, with declared superclasses before methods. Check class cycles, close instance superclass obligations after all heads register, and minimize inferred contexts by transitive superclass reachability.

Placement: new TC class_chirho/superclasses_chirho.rs owns graph/search/projection validation; existing evidence and Scope carriers retain parameter-vs-definition ownership. HIR supplies hidden superclass selector IDs. Existing driver classes module consumes the shared layout. Default methods remain the following measured unit.

- [x] Read Claude2's review and current source/Git/ledger; start isolated unit 748 with the existing warm target.
- [x] Execute exact GHC references for field order, transitive/diamond projection, outer Given ownership, instance contexts and invalid hierarchies.
- [x] Retain superclass declarations/selector identities; reject class cycles under the shared budget.
- [x] Derive scope-owned evidence on demand; enforce source kind, lexical ownership and acyclic projection.
- [x] Emit ordered superclass fields/selectors and close instance obligations; minimize inferred schemes.
- [x] Execute positive/negative controls, whole TC/next-driver, workspace, scoped lint and the unchanged fixed-bucket comparison.
- [x] Obtain Claude2's bounded source findings, resolve concrete findings, record exact scope, scan/push checkpoint and close the canonical row.

Known boundaries: arbitrary superclass/context forms, overlap, equality superclasses, defaults and full library/interface/primop loading stay Unsupported. Existing quadratic module-wide Wanted rescans/re-zonks remain an open resource cost. No native or whole-slice claim.


## Review and execution

Claude2 reviewed the r2 source, identified wasted fallback searches, then caught an incomplete r3 gate. The final gate is confirmed in #33817 at all 27 final source/fixture hashes. The fixes preserve exact parameter arity, reject projections from solved Wanted evidence, filter both searches by stable Given arguments, retain a fallback for mutable arguments, and charge actual ancestor visits. The new controls cover two type variables at Bool/Char, a method using a context superclass, malformed projection evidence, mutable-argument unification, a bounded deep match, and an absent-argument miss that skips both walks. Review was source-only; no independent execution claim.

Ten positive GHC 9.14.1 programs compile/run and match the Rust interpreter; five invalid programs refuse, and one GHC-valid compound superclass stays Unsupported. The final MethodSuperclass fixture was re-executed after a whitespace-only cleanup, with output h; old exact reference bytes are retained. TC 21/21 and next-driver 72/72 pass, none ignored. Workspace/all-target check and the frozen CLI build have zero compiler messages. Literal c4a30 against exact 795477ab reports zero changed-span hits and 255 OUTSIDE warnings, unattributed. The retained first 353-original profile had 19 ACCEPT / 14 REFUSAL / 320 Unsupported: tc046 U->A, tcfail017/tcfail020/tcfail027 U->R. All four match fresh exact-source GHC checks with no local flags. No previously supported classification changed. The final post-review profile is a separate measurement with identical per-name classifications and no previously supported movement. all.T flags/ways remain omitted; this is not full B-check/B-run or GHC compatibility acceptance.


## Closure

Source c96bf198116bc4094a25af4494929a00244b0175, crates 083ffd40fcc2ff3f7762a7912addc71545748627, pushed on next-pipeline-superclasses-chirho. Final frozen debug CLI c7a39b4baf442901351452475a355799030366f82d8a657ad5de28a03744a108 is 57,430,632 bytes, mode 0555, nlink 1; build fresh=false. Evidence: tmp-chirho/next-superclasses-748-chirho/checkpoint-chirho/, with scoped review records under review-chirho/ and all prior iterations retained. Secret scan of the outgoing source commit is clean.

Canonical row 748 closes with this measured checkpoint. Main receives metadata only and retains compiler crates 04116a69. No new worktree, target or dependency was created. Current source basis and parent frozen CLI bytes hold. Claude2 reviewed source only; its unread edges remain explicit in the retained note. The module-wide Wanted rescan/re-zonk cost, richer superclass/instance contexts, defaults, interfaces/library loading and primop/native gates remain open. Next: defaults through retained declaration accounts and checked ordinary instance-method references; group driver test modules before adding another root entry (the root currently has 14).
