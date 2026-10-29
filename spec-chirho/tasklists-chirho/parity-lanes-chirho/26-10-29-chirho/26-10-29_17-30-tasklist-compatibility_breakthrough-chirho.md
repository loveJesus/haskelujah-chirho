<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Compatibility breakthrough Chirho

L.J. asked to improve or close GHC compatibility and enlist claude2_chirho. Base: be548a9c. GPT owns row 661 and then 665. Claude2 owns independent row 670. One machine builder, two jobs and at most four test threads; GPT owns the DB. Overall compatibility remains open until evidence supports closure.

- [x] Refresh committed measurements, source state and lane ownership; enlist Claude2 via broker #30749.
- [x] Replay row 661's lint preparation bdc4b637; inference source is unchanged from the prepared base.
- [ ] Apply the prepared row 661 stage and independently verify item equality, mutation controls, test paths and corpus inference dumps.
- [ ] Run a focused row 661 gate, update moved workflow paths, and land the verified split.
- [ ] Implement range typing, standard Enum identity and occurrence evidence (row 665), with all-engine GHC controls and holds.
- [ ] Review and integrate Claude2's tested Enum Double change when ready.
- [ ] Run paired compatibility measurements if frontend semantics change, record exact movers and residuals, and report the verified improvement.

The published measurements predate this resumed work. The semantic gate must use an executable built from the current candidate; old CLI receipts are supporting evidence only.
