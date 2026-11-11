<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Next checker: dependency groups and generalization

Owner: HASKELUJAH/gpt_chirho, unit 742. Continue the approved first slice from S0 on next-pipeline-generalization-chirho. L.J. clarified that the GHC adapter is an implementation map: concise notes explaining GHC concepts, our corresponding data and necessary invariants. The product target remains an ordinary Cargo/Rust installation.

Placement: HIR owns identity-based dependency groups; tc owns schemes, level-based quantification and instantiation; the existing next driver owns explicit Core type abstraction/application. No new third-party dependency, legacy checker call or handwritten library interface is added.

- [x] Preserve clean S0 source, CLI and fixed-bucket pins; verify current sole writer and available disk.
- [x] Read GHC 9.14.1 tcPolyInfer/simplifyInfer and the legacy deterministic dependency ordering.
- [x] Add stable dependency-first SCC groups over HIR IDs, including nested reference dependencies.
- [x] Infer recursive groups monomorphically at a deeper level; solve equalities before quantifying; keep outer/environment variables unquantified.
- [x] Instantiate each published scheme freshly at each occurrence and carry explicit type arguments/abstractions to Core.
- [x] Execute positive local/top-level/mutual recursion and negative outer-capture/polymorphic-recursion controls, with GHC references.
- [x] Re-measure the unchanged first-slice B-check bucket against S0; retain Unsupported/resource outcomes and any regression.
- [x] Run workspace check, focused tests, scoped lint and secret scan; commit/push only measured owned progress and close the canonical DB unit.

The Haskell-2010 equality-only subset has no class predicates yet. MR/defaulting, class evidence, explicit polymorphic signatures, data/pattern elaboration and source-library loading remain distinct unfinished work. Inferred SCC exports may retain shared vacuous quantifiers; visible type application and exact GHC inferred-type presentation are not claimed by this unit.

The first slice is not passed by these controls. All 353/13 frozen inputs and original benchmark workloads remain unchanged.


## Measured checkpoint

Nine tc controls and fifteen next-driver controls pass. The five owned positive programs evaluate to q/l/m/s/u respectively in the interpreter and GHC 9.14.1. The two negatives are refused: GHC-83865 for captured-environment Bool/Char mismatch and GHC-25897 for polymorphic recursion. Original tc022/tc023/tc024 also compile under fresh GHC 9.14.1 references. All ten reference source hashes hold.

The 353-row unchanged original-file next profile moves only tc022/tc023/tc024 from refusal to ACCEPT. Counts move from 5 ACCEPT / 5 refusal / 343 Unsupported to 8 / 2 / 343, with no status regressions against S0. These debug-CLI observations omit all.T flags/ways and do not pass the full configured first slice. No numeric/library/runtime B-run claim follows.

Workspace all-target check has zero compiler-message records. Changed-span Clippy against d3f71037 has zero hits and 263 unique OUTSIDE diagnostics, unattributed. The debug candidate was freshly rebuilt and retained at SHA256 298c3a5e7e5bba1d1ee22a6e991227ab15a15f85a3b2bcfc29e0c4bfcf2a6a3d (56,177,016 bytes, 0555). S0 CLI dacafbb6 and the frozen bucket 8c9551ea remain byte-equal to their pins.

Retained evidence: tmp-chirho/next-generalization-742-chirho/{ghc-reference-r2-chirho,build-r1-chirho,b-check-r1-chirho,b-check-comparison-chirho.json,gates-r1-chirho,format-confirmation-chirho,final-closure-chirho.json} in the same worktree. The implementation map is spec-chirho/workflows-chirho/next-pipeline-chirho/ghc-implementation-map-chirho.md. All source files remain below the size limit.

The final formatting check found blank EOF lines in new fixtures and the map. Those were removed. Pre-format GHC inputs and R1 raw outputs remain retained; R2 freshly rechecked all ten final source hashes and reran the five positive evaluations. All fifteen next-driver controls pass again after this formatting-only change. Production Rust and candidate CLI bytes are unchanged from the full gate run.

Source checkpoint: 1db99f134218ce18b7bbf0ccfb08d35421febf66, crates ea48626fe2d2a21e500e7dde81375e09566911e6, pushed and read back on gh_chirho/next-pipeline-generalization-chirho. Canonical DB unit 742 is closed; compiler source remains isolated from main.
