<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Constructor-headed instance dictionaries

Owner: HASKELUJAH/gpt_chirho, unit 747. L.J. says next; Claude remains closed. Base 5b413d22f39870dc4865306a6f0fe365349279c6. Reuse the existing next worktree and warm target; preserve prior snapshots and measurements. Main compiler source stays unchanged.

Implement the second part of class-evidence design B: constructor heads applied to distinct Type-kinded variables, with contexts on those variables. Match the known head one way, never specialize a Wanted variable to choose an instance. The instance scope owns its rigid binders and context dictionaries; Core emits a dictionary function with matching type/context argument order. No self Given, overlap selection, arbitrary flexible head/context, superclass, default method or numeric defaulting is introduced.

Placement: group TC class registration, evidence and matching under class_chirho/ to keep the source root below 15 entries. HIR retains declaration-owned instance binders/contexts. Reuse the arena substitution and scope-owned evidence, with strict-subterm instance contexts and the shared allocation ceiling bounding expansion. Existing module-wide rescan cost remains an explicit limitation.

- [x] Verify clean parent, canonical ledger, sole writer and capacity; start unit 747 and branch in the same worktree.
- [x] Retain exact GHC 9.14.1 references for compound/nested dictionaries, recursion, ordering, missing context and unsupported boundaries.
- [x] Lower instance binders/contexts and check constructor-head shape, kinds and duplicate identity.
- [x] Solve by one-way matching and emit context subgoals in the original Wanted scope; validate evidence before publication.
- [x] Abstract dictionary functions and execute the positive fixtures through Core/STG; reject missing or wrong evidence.
- [x] Run whole TC/next-driver, workspace, scoped lint and the unchanged frozen 353-original comparison.
- [x] Update GHC correspondence/workflows, scan outgoing commits, push checkpoint and close the canonical unit row.

Maturity: pre-beta, isolated and reversible. GHC remains a developer reference; installed code is Rust. Passing this vertical does not establish library loading, primop completeness, native execution or whole-slice acceptance.


## Checkpoint evidence

GHC 9.14.1 exact references are retained under tmp-chirho/next-context-instances-747-chirho/ghc-reference-r1-chirho/. Eleven positive programs run and agree through the Rust interpreter; six invalid programs refuse. Three richer GHC-accepted forms remain Unsupported. The Haskell2010 scoped-instance twin rejects under GHC and remains Unsupported locally. TC authority and bounded-expansion tests pass.

Final source gates: 15/15 TC and 64/64 next-driver tests, none failed or ignored. Workspace/all-target check has zero compiler-message records. Literal c4a30 against exact 5b413d22 over the whole source delta reports zero changed-span hits and 255 OUTSIDE warnings, unattributed. The 353-original comparison has 18 ACCEPT, 11 REFUSAL and 324 Unsupported, with only tc063 U->A and tcfail001 U->R; both match fresh GHC checks on original bytes. The final post-lint build/comparison has the same per-name classifications as the retained first profile. No previously supported classification changed. all.T flags/ways remain omitted, and neither profile is full B-check/B-run or compatibility acceptance. No peer review is claimed while Claude is closed.


## Closure

Source 37c836d4ad1f80bded0023025a3e3b379429cbeb, crates 71b1f5f130ba9fec7151ed09ab52f60087d48365, pushed on next-pipeline-context-instances-chirho. Gitleaks scanned the outgoing source commit with no leaks. Final frozen debug CLI a4343d0d0b891f2267deabb5b97923e3c63faf7824d18688a638bf916c445a36 is 57,217,752 bytes, mode 0555, nlink 1; its build has zero compiler-message records and fresh=false. Final evidence is under tmp-chirho/next-context-instances-747-chirho/checkpoint-chirho/.

The canonical row 747 records this measured checkpoint. Main receives documentation and ledger metadata only, preserving compiler crates 04116a69. No additional worktree, dependency or target was created. The TC source root has 12 entries and class_chirho/ has four. Prior frozen parent CLI bytes and current source basis hold. Shared allocation exhaustion remains Resource, and the module-wide rescan/zonk cost remains open. This is an isolated checker/interpreter feature, not a main-source, native, library or release acceptance.
