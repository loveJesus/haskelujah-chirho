<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Checked closed-family boot contracts

`reference_chirho.ts` records seventeen independent GHC9.14.1 cases, predictions
before execution, exact sources and hashes, commands, full diagnostics and a
separate timeout flag. All seventeen predictions agreed with GHC; none timed out.
Run with Bun and an output JSONL path. GHC writes only inside a fresh scratch root.

`boot_families_chirho.rs` in driver typing integration uses byte-identical sources
through file check and file compile. The pre-repair candidate passed7/17 tests;
the repaired candidate passed17/17. The failing-before set includes valid
abstract/empty/full contracts, alpha-renaming and equation read-back, plus
reason-level hidden-equation and invalid-implementation controls and the illegal
ordinary-source abstract body. These are frontend results, not execution tests.

The AST now distinguishes open, full closed and abstract closed family bodies;
invalid parser recovery cannot masquerade as an empty family. Checked source-local
rows own equation agreement, with shared hidden/visible variable identity and
source ordering. Abstract boot promises reveal no equations. Unit controls check
alpha-renaming versus altered results/hidden arity and bounded capture work.

T6018a now passes the former unsupported-boot-body boundary but still fails in
the implementation's closed-family injectivity checks (Bak's covering variable
row and Foo's shadowed row). Fresh GHC9.14.1 accepts T6018a. This checkpoint does
not claim that corpus file recovered, a full corpus pass, complete boot semantics,
or the separate defining-origin re-export repair. Main and published counts are
unchanged.
