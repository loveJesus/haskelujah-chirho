<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Actual-source Num route

Owner HASKELUJAH/gpt_chirho, unit 752, base da79fd1d. L.J.'s direction relayed in #34030: lets do it; finish it / use gpt_chirho to build. Claude2 supplies architecture #34005/#34030/#34031, source reading only. GPT writes and executes. Same worktree and target; main compiler source remains frozen.

- [x] Recover source/ledger/capacity and the four-stage route; checkpoint the base.
- [x] Stage 1: vendor complete unmodified Tuple.hs/Num.hs and the upstream license inside the driver crate; verify hashes, take declared spans, mint ghc-internal from the driver manifest and propagate partiality. Execute fst/snd; absent swap declines.
- [x] Stage 2: operator identities/signatures/definitions, single unambiguous infix application, original-name fixity records. Grouping-sensitive syntax declines until resolved-name regrouping exists.
- [x] Stage 3: preserve MINIMAL/INLINE through the real class where-walk account; validate ownership/default availability, preserving later methods/defaults. Other pragmas decline.
- [x] Stage 4: known-identity ordinary fromInteger/negate and the real Num class slice. Execute its default subtraction at a home type against GHC; absence/defaulting/orphan collisions decline under partiality.
- [x] Run relevant gates and unchanged fixed-profile comparison, verify Cargo-package embedded inputs/license, document, scan/push and close canonical row.

Placement at brick 1: driver next/library_chirho/ owns the immutable embedded manifest and vendor files (whole files plus upstream license), independent of the volatile/developer reference tree. Unit-qualified graph keys allow home and library modules with identical spellings to coexist. HIR/interface owns completeness/wired-type/fixity carriers, TC checks partiality at actual absence/instance decisions, and source syntax accounts stay at their real parser/class owners. No new solver concept or fake standard class is added.

The finish line is the actual library default at a home type, not just the tuple manifest or operator scaffolding. Num Int/Integer, subtract, defaulting, full ghc-internal, native/IO-main and B-run remain outside this route. Library slices are checked fragments and never complete-module absence authority.

Review correction #34053: slice imports now require explicit checked lists; top-level clashes are checked at uses and imported method visibility is checked at the instance owner. Installed dependencies never fall back to home source, and home Prelude discovery declines rather than seeding a substitute. Raw class pragma ancestry is insufficient: layout column, direct-child brace position and nested-body/tail checks now gate the account. Fixed declarations retain whole-file upstream bytes and license.

Focused iteration: 126 next-driver tests passed after these corrections. Final broad checks, CLI/build/fixed-profile comparison, source-package inventory and source review are pending; this line is not the final acceptance record.


Final source checkpoint: 8865b7f2bc135d1d6bf98894bb181dcc04fb9eaa, crates 7d31de0bc6e8ec68f7cb9be3dfa9dae7463904c7, on next-pipeline-real-num-chirho. The canonical main compiler tree remains 04116a69.

Accepted evidence is under tmp-chirho/next-real-num-752-chirho/checkpoint-chirho/accepted-chirho/analysis-chirho/acceptance-chirho.json. Workspace/all-target check and CLI build have zero compiler-message records. AST/parser/naming/HIR/TC/Core: 857 passed; next driver: 133 passed; none failed/ignored (990 scoped tests, not the full legacy driver runtime suite). All eight supported byte-identical GHC oracle programs execute with matching character output through the Rust interpreter. Twenty file-CLI outcomes, two valid occurrence declines, an isolated-PATH linked Core dump and the embedded licence readback pass. Cargo package inventory includes all three whole upstream files and both licences exactly once; 507 paths, 3,569,172 existing-file bytes, no package-list stderr.

Fixed 353-original profile versus unit 751: 22 ACCEPT / 20 REFUSAL / 311 Unsupported, with all source hashes unchanged and no previously handled mover. New accepts: tc005, tc008, tc037. New refusals: tcfail013, tcfail031, tcfail037, tcfail040, tcfail148. Fresh GHC 9.14.1 accepts/rejects the byte-identical movers accordingly; the refusal reason classes are type mismatch, Prelude-name ambiguity, ORD ambiguity and field-kind mismatch. Flags/ways remain omitted; these are dispositions, not a configured GHC ratio or B-run PASS. The initial false-accept candidate e06ea0d0 and the intermediate b8da28de are retained diagnostics, not acceptance.

Clippy at exact da79fd1d: zero changed-span hits over 3511 changed lines / 64 files; 265 OUTSIDE diagnostics remain unattributed, not parent-proven and not whole-tree green. The whole source commit passes gitleaks (one commit, no leaks). Owned non-vendor diff whitespace check passes; upstream LICENSE spaces and Num.hs EOF are preserved byte-identical as required. The known-name-to-lexical mutant is killed, restored byte-exact, and healthy next tests pass afterward.

Claude2's architecture and bounded source reviews #34053/#34065/#34082 are credited as source reading only. Corrections include explicit slice import lists, use-site clashes, actual Prelude name-only scope, visible-method checks, pragma layout/delimiters/member tails, and both manifest group edges. Self-qualified references and wired constructor expression forms remain Unsupported; a fresh GHC accepts the two review witnesses. Embedded-asset failures also decline rather than convicting user source. The writer tested these final small corrections; no independent execution acceptance is claimed.

Current accepted CLI: 0c2d3472b2278f6d742ceee5b3f1e64925524f774c1ce66c3f9f1a67181d8dc3, 60,252,808 bytes, 0555, nlink 1, fresh=false. Same warm target and worktree were reused; no extra workspace was created. Num Int/Integer, arithmetic, subtract, defaulting, full Prelude/ghc-internal, native/IO-main and B-run remain the next boundaries.
