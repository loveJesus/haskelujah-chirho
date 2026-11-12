<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Next written signatures

Owner: HASKELUJAH/gpt_chirho, unit 746. L.J. directs continued solo work; Claude is closed. Base 361447d1dd1d690440a775567bdc8943bd00529d, compiler crates b491108f9561c72250be5499e028930c3bfb1de1. Reuse the same worktree and warm target. Keep prior snapshots, frozen binaries and measurements intact.

Implement the signature prerequisite of design unit B: explicit/implicit rank-1 binders, contexts on type variables, checked bodies with declaration-owned skolems and Givens, and declared type/dictionary abstraction. No applied/context instances, superclass/default methods, higher-rank arguments, pattern/type-pattern scoped variables or defaulting are introduced by this unit. This is a real checker-to-interpreter vertical, not a signature acceptance shortcut.

Placement: HIR signatures own their binder and class declaration IDs. The TC signature module publishes a reusable scheme and checks the definition under a child Signature scope whose owned skolems/Givens never escape. Existing arena substitution, unification and Core evidence abstractions are reused. No separate predicate substitution or legacy dictionary pass is added.

- [x] Verify current Git/ledger state, closed Claude pane, compiler occupancy and capacity; start canonical unit 746.
- [x] Retain and execute GHC 9.14.1 references for declared dictionaries, rigid checking, nested same-name signatures, capture and invalid/ambiguous signatures.
- [x] Lower rank-1 signatures with lexical declaration IDs and explicit Unsupported boundaries.
- [x] Check signatures under owned skolems/Givens; preserve signed schemes and lexical evidence while inferring unsigned groups.
- [x] Execute declared dictionary parameters through the existing Core abstraction, including recursion and negative escape controls.
- [x] Run focused tests, workspace/all-target check, scoped lint and the unchanged fixed-bucket comparison at the checkpoint.
- [x] Update the GHC correspondence/workflows, scan the outgoing range, push coherent progress and close the canonical DB row.

Known open issue retained from unit 744: module-wide Wanted scans/repeated charged zonks can be quadratic and exhaust the module ceiling. Do not claim scalable source-library checking while it remains. GHC is a developer reference; the installed product remains Rust. Main compiler source is not changed by this isolated work.

## GHC scoping decision and measured controls

Read retained GHC 9.14.1 (902339d3), Tc/Gen/Bind.hs tcPolyCheck and Tc/Utils/Unify.hs tcSkolemiseCompleteSig plus its scoping notes. Under the accepted GHC2021/default profile, a leading explicit forall scopes over the RHS. An implicit inner signature captures that outer variable; an explicit inner forall shadows it with a fresh identity. Implicit outer signature variables do not scope over the RHS. Source binders therefore remain installed while lowering the matching RHS only for an explicit forall; the checker resolves their IDs through the actual signature-parent chain. Haskell2010 explicit-forall syntax remains Unsupported in the restricted profile rather than being silently enabled. The explicit ScopedTypeVariables pragma remains an unimplemented profile override; its GHC2021 leading-forall behavior is implemented without adding pattern-signature support.

A GHC-executed counterexample rejects using a scoped outer a as Char through an inner a->a declaration. Its positive twin, scoped RHS annotation, explicit shadow and fixed outer variable in a local context all accept/run. The draft that treated every inner variable as fresh was not executed on this counterexample; it was corrected from the source/behavior review before checkpoint claims.

Executed so far: all 13 tc tests and 54 next-driver tests pass, including twelve positive signature programs, seven invalid signature programs and three GHC-accepted Unsupported boundaries. The GHC references are retained under tmp-chirho/next-signatures-746-chirho/ghc-reference-r1-chirho/. Checks of whole workspace, scoped lint and fixed bucket are still pending. No claim of peer review while Claude is closed.


The checkpoint uses the same frozen 353-original profile: 17 ACCEPT, 10 REFUSAL, 326 Unsupported (parent 16/9/328). tc042 moves Unsupported to ACCEPT; tcfail153 moves Unsupported to REFUSAL. Both match fresh GHC 9.14.1 checks on the original bytes. No previously supported classification changed. all.T flags/ways remain omitted; this is not B-check acceptance or a compatibility ratio. Final gates: tc 13/13, next driver 54/54, workspace/all-target check zero compiler-message records. Scoped lint completed; literal changed-span attribution follows the source commit.


## Closure

Source ce1925cdcab1b42b84b669963b123caf95b53d3f, crates 1ddbf7229ddbc6ba4cb5d49e4525d0d850769a3c, pushed on next-pipeline-signatures-chirho. Tests 13 tc + 54 next-driver pass with none failed/ignored; workspace all-target check and the frozen CLI build have zero compiler-message records. Literal c4a30 vs exact 361447d1 over the source delta has zero changed-span hits, 255 OUTSIDE warnings unattributed. This is not warning-free-tree or full slice acceptance. Outgoing gitleaks is clean.

Frozen CLI: 96617ae0cf0405c7bc71eb07ce76e9064f355d3ea848a21dced2b60ce859d2b2, 57,146,552 bytes, 0555. Final receipt: tmp-chirho/next-signatures-746-chirho/build-r1-chirho/acceptance-chirho.json. Parent unit 744's named frozen CLI hash holds. No new worktree or dependency; the warm target was reused. Canonical DB unit 746 closes with this checkpoint; main compiler code remains 04116a69. Resource/library/richer-signature limitations above remain open. No peer review was claimed.
