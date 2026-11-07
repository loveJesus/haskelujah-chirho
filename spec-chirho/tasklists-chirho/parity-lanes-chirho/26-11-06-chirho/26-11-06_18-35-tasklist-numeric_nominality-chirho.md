<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Nominal numeric equality Chirho

The ordinary unifier currently accepts both `Int ~ Double` and `Double ~ Int` with an empty substitution. This equates distinct type constructors without a conversion. Row 708 removes that one compatibility arm; numeric literals, explicit conversion and defaulting retain their own obligations.

- [x] Pin an isolated A9 source baseline (`075a9c95`, crates `a9a38e3d`) and census the normal-unifier consumers. Root owns canonical metadata and SQLite.
- [x] Name the source footprint before editing: bounded direct unifier units; one numeric-nominality driver child, ten Haskell fixtures and its two-line root mount. No new API, registry or runtime representation route.
- [x] Publish the production-equal test-only checkpoint; preserve the original MR lambda witness separately because A9 does not contain MR.
- [x] Pin and execute the ten independent GHC typing controls before any functional claim.
- [x] Execute the original unifier against the negative controls and retain actual wrong acceptance.
- [x] Remove only the normal numeric widening arm and its misleading comment in a separate source checkpoint. The `0afac144` test-only checkpoint retains the original production behavior for the later measured negative control.
- [x] Execute symmetry/nested rejection and same-type/metavariable preservation units, plus the paired driver controls; surface any literal or numeric-pattern producer prerequisite before expanding scope.
- [x] Complete focused warning-free checking, lint attribution, outgoing secret scan and push with exact source/evidence pins. Broader preservation and integration remain separate.

The driver controls compile source without running a program. Acceptance of an unannotated numeric binding establishes that it can be checked, not a particular inferred default type. The original MR lambda failure depends on restricted generalization and is not replaced by a prediction about this A9-only patch.

- [x] Run independent main-relative preservation: fresh frozen `1b71c06d` CLI against the source-equal, pin-matched retained actual-A9 baseline on both unchanged compiler axes.
- [ ] Root review, source integration and canonical landing. The broader kind engine and runtime/defaulting gaps remain separate open work.

## Measured checkpoint

Source checkpoints are `0afac1447f5e52ff52fad5c7fd4db6c168d8b407` (production-equal controls) and `1b71c06d3e5b152706fbb09931acdd32c24a2a35` (only the normal numeric widening arm removed, plus accurate comments). The tested candidate crates tree is `a98c1766bd6ccf9a45ebab0eb1effd895249d7ac`.

The explicit Haskell2010 GHC 9.14.1 reference plan matched all ten predictions: three invalid signed numeric values rejected with GHC-83865 and seven valid controls accepted. The original source-only plan used invalid lowercase case IDs; it remained unlaunched and was superseded by the module-name-only r2 correction. The reference result is in `tmp-chirho/numeric-nominality-708-chirho/test-only-a9-r1-chirho/ghc-nominality-r2-r1-chirho/summary-chirho.json`, SHA256 `f366b902c0cf674e47de8baa4f15c0dd807a70c44c2863f31be5a145d0a2292a`.

Before correction, two of three direct units failed, and the source negative collector explicitly accepted all three invalid fixtures. After correction, all three direct units and both source controls passed. The seven positive fixtures remained unchanged. The full typing library passed 480 tests, including the three direct units; these counts overlap and are not added. Changed-span Clippy found zero hits against `075a9c95`, with 256 diagnostics outside changed spans, not independently attributed to the parent. The paired summary is `tmp-chirho/numeric-nominality-708-chirho/product-before-after-r1-chirho/summary-chirho.json`, SHA256 `d3956d81c572fd096f522527a40b54961f601795746c1cec1def453364b4b31e`.

The first release attempt refused before Cargo because the separate kind gate contained its preserved cache symlink; nothing was removed or ignored. The r2 release used this already clean author worktree and its own target, without a target clone or relaxed guard. Cargo emitted one matching CLI artifact with `fresh: false`, optimization level 3 and zero diagnostics. The frozen read-only candidate SHA256 is `09ff7295f79b23db821953cc54ef6bc836303e105e95817689c6b79b411f1ac0`; its build summary is canonical `tmp-chirho/kind-engine-product-gate-chirho/checkpoint-1b71c06d-r2-chirho/summary-chirho.json`, SHA256 `b54affb66ff29320a8b8e34da6ecf987ee21cc8e030d716a55b20eba083eb636`.

Both fresh candidate axes completed: 938 should_compile and 767 should_fail cells. There were zero verdict movers and zero UNCLASSIFIED results against the retained actual-A9 binary `1ae0f2f48e0b6720ab7bdb3e2e989b668cbd96055506cdd70f6b17950520c0e7`; all 5,115 fresh raw files were byte-equal to its retained evidence. Corpus, stdlib, runner/options, all 10,382 cache files, source, tools and binaries remained pinned, and owned process cleanup was empty. The baseline was not rerun. The comparison is `tmp-chirho/numeric-nominality-708-chirho/preservation-1b71-preparation-r1-chirho/corpus-diagnostic-1b71c06d-r1-chirho/summary-chirho.json`, SHA256 `0b26bfcaf20e5d2e1d30d18a5460cc326100d28c0d1644c84121ff4a686abd26`.

A final read-only fold initially assumed every copied baseline file stayed identical. The existing resume controller intentionally rewrites two derived historical-comparison prose fields. That failed fold and exact diffs remain preserved; 5,129 protected baseline inputs, receipts and raw files stayed identical. This is no published measurement update, main landing, full runtime acceptance or broad-kind compatibility claim.
