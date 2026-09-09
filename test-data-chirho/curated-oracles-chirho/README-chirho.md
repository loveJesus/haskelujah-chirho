<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Curated execution oracle provenance

`ghc-9.14.1-chirho.jsonl` records a complete independent reference run on
2026-09-09: **514 execution programs, 514 matching outputs**, GHC 9.14.1,
zero reference errors, warnings or timeouts. Each record holds the source SHA-256,
declared expected output and actual GHC stdout. Only final LF characters are
normalized; spaces and interior newlines remain significant. Exit status must be
zero. Source hashes, not a later branch tip, bind the claim to exact inputs.

The first complete pass on the pre-repair inputs matched 490 outputs and rejected
24 invalid sources. The final records for those 24 retain the original hash and
GHC diagnostic: T006 required an Int annotation for its custom class, and 23 files
needed explicit Prelude hiding for a locally declared Either, Ordering or
enumFromTo. No expected-output header changed in this repair. All 514 programs
were rerun after the input changes, not just the 24 failures.

This manifest covers the direct-root curated programs only. It excludes the 23
compile-only curated inputs and both upstream typecheck corpora. It is not a
Haskelujah pass report or certification of later source changes. The separate
unfiltered workspace gate must execute and compare those same 514 oracles.

Earlier post-reader-repair claims of 514 compared executions were two too high:
T527 and T536 put a pragma before their metadata and were silently compile-only.
The prologue reader and independent execution-directive inventory now cover that
gap. This reference run includes both inputs.

Recheck the exact recorded inputs with installed GHC 9.14.1 and GNU `timeout`:

```sh
rtk proxy bun test-data-chirho/curated-oracles-chirho/verify-reference-chirho.ts
```

The verifier requires the recorded source hashes and declared execution inventory,
uses a fresh empty working directory with closed stdin, and gives each source a
15-second deadline plus two seconds to terminate. It never rewrites sources,
oracles or this manifest. Any changed source requires a new independent reference
measurement; a previous matching answer is not proof about changed bytes.
