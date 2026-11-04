<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Numeric entry evaluation

Root owns this investigation separately from the pure-move series. D3 and D4 reveal interpreter tests returning HeapPtr instead of expected Int at landed 45101043. That proves the failures precede the moves; it does not yet prove whether they precede K01. The pure-main fixtures are evaluator API tests, and must not be advertised as GHC-valid executable modules.

- [x] Preserve D3/D4 tip and 45101043 per-test outcomes, source commits and raw failures; do not rewrite fixtures or expectations.
- [x] Run the same bounded filter at exact pre-K01 70a4805a, including an isolated-target rerun. All 36 outcomes match 45101043 and D4: 11 pass, 25 fail. This failure set predates K01 and the moves.
- [x] Observe the actual scheme, post-dictionary Core and returned closure. Unsigned constant/let entries have a Num predicate, a retained dictionary lambda and a Fun closure with arity 1 and no payload. Explicit Int returns 15; the IO polymorphism hold prints (3,4.5); id remains a genuine function. Raw probe and logs are in tmp-chirho/numeric-entry-*; no forcing-only explanation fits the measured closure.
- [x] Write five minimal controls before a repair and measure GHC 9.14.1: pure-main modules are rejected; the IO print twin compiles and prints (3,4.5). Oracle files explicitly add the John 3:16 header to the diagnostic bodies; no byte-identity claim hides that addition.
- [x] Assign the separate source-only placement review to Claude, in registered numeric-entry-evaluation-chirho from exact 119d9b5d. Preserve the other branches and all fixtures.
- [x] Measure the provenance radius before source GO. Claude #31340 finds this is the general predicate class-identity seam shared with row 690: 252 non-test producer sites, 28 rebuilding sites, originless classes/instances and qualifier loss. The bounded caller-scope shortcut cannot prove it.
- [ ] Implement and verify that one shared identity carrier after the common schema is green. Identity must come from actual producers and survive substitution, generalization and export/import; unknown cold predicates stay unproven. ClassEnv and matched instances must carry compatible authority. No temporary spelling/defaulting registry or consumer-first source is authorized.
- [ ] Review the shared producer/API footprint, then repair it in an isolated branch with one implementation owner. No semantic repair belongs inside a pure move.
- [ ] Run affected tests, warning/lint checks and behavior controls at the corrected commit; retain any unproved or inherited failures and update the canonical ledger.

No compiler behavior or test harness change is authorized merely to turn an inherited test green. The existing evaluator contract and actual returned value must determine the repair.
