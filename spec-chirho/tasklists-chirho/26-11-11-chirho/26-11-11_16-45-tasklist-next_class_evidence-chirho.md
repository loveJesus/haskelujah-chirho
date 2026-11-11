<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Next checker: source class evidence

Owner: HASKELUJAH/gpt_chirho, unit 744. L.J. directed continued work. Claude has returned and is providing read-only GHC/design support (33697), with no edits or DB writes in this lane. Source basis: 71b134d51fa7299dc949a779ec7aa98d18f2fad1, crates 2bea7e30378e3695a1501e5295e54e0436848e20. Preserve all prior sources, CLIs and first-slice inputs.

Placement: HIR retains creation-time class/method/instance accounts. The checker owns predicates, evidence holes and qualified schemes; the direct Core elaborator turns those facts into dictionary bindings, selectors and parameters. The installed program remains Rust and has no GHC runtime dependency. No legacy inference/dictionary pass or handwritten standard-class catalogue is introduced.

- [x] Verify current source/DB state, sole implementation writer, live coordination identity and compiler occupancy.
- [x] Request a read-only GHC-informed design covering identity, wanted/given evidence, generalization and recursive SCCs.
- [x] Prepare exact-source GHC controls for polymorphic dispatch, lexical capture, recursion, MR and invalid evidence choices.
- [x] Settle the minimum sound predicate/evidence carriers before production authoring; record any architectural deviation in the implementation map.
- [x] Carry source class/method/instance declarations through HIR without text-based authority.
- [x] Solve and abstract class evidence at the correct binding scope, with no premature instance specialization or hidden unresolved predicates.
- [x] Execute source-defined methods via explicit Core dictionaries, including dictionary passing through inferred functions.
- [ ] Run focused gates, reference comparison and a fixed-bucket check at the meaningful checkpoint; preserve Unsupported/resource classifications.
- [ ] Update workflows, secret-scan, commit/push owned progress and close the canonical DB unit.

The initial candidate is ordinary single-parameter Type-kinded source classes and ground instances. Superclasses, overlap, context instances, defaults/defaulting and explicit higher-rank signatures remain separately unimplemented unless the reviewed vertical requires a prerequisite. This scope is provisional until the predicate/generalization review; it is not authority to mis-generalize a constraint, erase missing evidence or silently default a wanted type.

## Adopted unit A design

Adopt Claude design a5c200e3, sections 3-8, with predicates as arena types of kind Constraint, rigid class binders, frozen inferred binders, A-normal evidence terms and occurrence wrapper steps. Each Wanted retains its emitting scope; dictionary evidence is inserted in that scope. Inferred binding abstractions retain the current per-binder style with scoped evidence-ID allocation. Class-only Givens keep equality touchability unchanged. MR uses the source binding form and promotes constrained metas rather than failing an intermediate solve. Remaining dictionary Wanteds are checked at module end, before final Any zonking. Non-variable inferred contexts and SCCs requiring impedance matching remain Unsupported. No defaulting or signature shortcut is inferred from this unit.

GHC 9.14.1 references now cover the twelve original controls plus two-variable evidence, a shadowed method and result-directed selection. All six initial positives and the three sharp positives accept and execute; all six negative cases reject. Receipt R1, method-scope R2 and sharp R3 inputs remain under tmp-chirho/next-class-evidence-744-chirho/. The source-class HIR boundary compiles, and all 24 retained next-driver controls pass with a temporary explicit Unsupported guard at the checker until evidence is implemented. This is no class-execution claim.

Resource policy: reuse the existing warm target; one Cargo process at -j2, no new worktree, full target copy or whole-suite repetition during iteration. Keep the previous frozen CLI intact and retain one new CLI only at the measured checkpoint. Canonical DB row 745 is Claude's design work and will be preserved in the next metadata sync beside the still-open row 744.

## Executed checkpoint work

The checker/evidence path is cleared at the stated scope by Claude review 33704, and the bounded runtime/CST repairs by 33706. No legacy checker/dictionary rewrite is called. Ten source-class positives execute, nine invalid controls refuse, and three valid but unimplemented forms remain Unsupported. Fresh GHC references include all four complex-head variants and the partial-constructor-kind and duplicate-value controls. The reused AST drops an undeclared default; next now declines that entire unsupported source form before lowering. The first false-accept bucket observation and the wrong-value runtime BEFORE are retained.

The complex-head repair reuses the existing argument-source lifetime and allocates a fresh captured head inside a function. Original AST/data/type controls and authority/allocation tests run at the final checkpoint. Review's whole-module Wanted scans/re-zonking concern is OPEN as a large-module resource defect; this checkpoint is not a scalability, source-library, native or B-run pass.
