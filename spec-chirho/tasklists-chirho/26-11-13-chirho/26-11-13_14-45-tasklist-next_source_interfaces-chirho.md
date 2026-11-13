<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Checked source interfaces and linking

Owner HASKELUJAH/gpt_chirho, unit 751, base b2a48cba. Claude2 supplies architecture #33975 only; GPT writes and executes. Same warm target/worktree, no new compiler dependency, no main compiler-source landing.

- [x] Recover clean source, canonical ledger/capacity and the architectural decision.
- [x] Introduce defining original names and an arena-free closed checked-interface projection.
- [x] Re-intern declarations/schemes/instances into independent consumer arenas, separating scope from identity.
- [x] Check source modules in dependency order and link Core identities by original name; unresolved links fail.
- [x] Measure the nominal-type negative and cross-module contextual/default dictionary runtime against GHC; add identity/authority/growth/failure controls.
- [ ] Run relevant tests/checks/lint and fixed-profile comparison; document, scan, push and close canonical row.

Placement at brick 1: HIR owns original names, closed interface types and declaration-owned children in interface_chirho/. TC owns export/import projection in interface_chirho/. Driver owns source graph and Core linking in source_interfaces_chirho/. No TypeArena, NameId, evidence or scope handle crosses a module boundary. No legacy interface type is consulted. Public single-source entry points reuse the same empty-import path.

First scope: dependency-ordered source modules in the standalone home unit, plain imports, default exports or value-only explicit exports. Interfaces retain hidden declaration dependencies and class/instance children; export visibility never creates identity. Unsupported/failed producers publish nothing. Installed package identity, rich import/export syntax, cyclic modules and persisted artifact trust remain separate work; this brick's pure closed carrier is the same input used for in-process re-interning.

Architecture detail: the class dictionary constructor key is derived from its stored owner original name and a fixed role, rather than another freely named child. CompilerChirho denotes ghc-internal, verified at GHC Unit/Types.hs:600; user modules remain the standalone main unit. No manifest can admit partial/package source yet.

One measured refinement to #33975: imported overlap is a demand-time GHC error. The GHC-unused twin accepts and prints v. The union therefore declines an overlapping import instead of issuing a false semantic duplicate-instance refusal. Own-module duplicate validation is unchanged. Generalization fixtures were grouped under generalization_chirho/ byte-preservingly to keep the feature root bounded.

Growth correction before source freeze: repeated identical plain imports are deduplicated, queued file-module names are bounded/deduplicated, and every retained import-map/name string consumes HIR byte allowance before copying. This also bounds the existing own-name module/occurrence copies. A 2,000-repeat import control still checks under a 512 HIR ceiling. Fixture relocations preserve all seven blobs; checkpoint/fixture-relocations-chirho.json maps historical paths to current paths and source hashes. Historical receipts keep their original source basis; no claim that all older live worktree paths remain unchanged is made.
