<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Char.toEnum prerequisite for case construction Chirho

Progress unit 520. Existing source writer: HASKELUJAH/claude2_chirho under #27304. GPT owns canonical main/DB, integration disposition and source review. Agy is requested for a bounded read-only representation/backend audit; no implementation ownership transfer. Claude holds the only builder for independent ground 7ad703e8 under #27446/#27447. This Char unit has no Cargo reservation.

## Exposed behavior and inspected source

Claude2 #27446 reports a second case blocker after c631a29b's representation-only lint recheck: comprehension T03 builds Char values with toEnum and now reaches execution, printing an opening quote and newline where GHC prints the quoted string ab. Held 4cfb9584 rejects that case syntax. Q1 (the same list built with if) and Q2 (a direct two-character toEnum list) execute with the same wrong output on held and c631a29b, showing the character producer defect already exists. This is owner-measured runtime evidence, not a new GPT execution.

Owner evidence:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/case-gate-held-1017-chirho/t03-chirho/

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/char-toenum-1017-chirho/

GPT reads the exact Core seed at 4cfb9584: prelude_chirho.rs:7702 onward implements $prim_Enum_toEnum_Char as an Int identity, while fromEnum uses ord#. Runtime prim_chirho.rs:408 onward implements chr# with char::from_u32(...).unwrap_or(NUL), while ord# accepts both CharChirho and IntChirho. A substitution of chr# alone can therefore turn currently working round trips into a silent NUL for unsupported Rust char code points. The owner's CH08 oracle accepts surrogate 55296; CH09 preserves round trips for 0, 97, 955 and 1114111. Add the surrogate to the round-trip control before claiming preservation.

The earlier trace also reports LLVM lacks chr#/ord# lowering while Cranelift recognizes them. That backend claim needs exact current dispatch/unsupported-path source review and eventual compile/run evidence. A new primop path cannot be called supported merely because interpreter ASCII output is repaired. The large Core prelude receives only focused helper dispatch/extraction; no broad Char/Show rewrite is assigned by this prerequisite.

## Contract to establish before editing

Find the smallest coherent conversion route that fixes the three known silent scalar outputs, retains existing working round trips and laziness, and handles invalid range behavior explicitly. Do not silently map a valid GHC surrogate code point to NUL, or introduce a native-backend failure through an unsupported new primop. Surface any representation dependency with its actual affected paths and the available bounded alternatives before implementation. Preserve the case source and red packet; no restoration of the malformed parse to hide T03.

Keep separate unless the same shared mechanism necessarily changes them: missing Enum.succ/pred for Char, Data.Char.chr import identity, character ranges, Enum Ordering, the escaped-quote literal decoder and Char/String Show escaping. These are measured leads, not automatic scope expansion. Show-based Unicode controls must distinguish the existing escaping gap from conversion correctness; use fromEnum round trips and ASCII output where appropriate. WebAssembly remains unmeasured until an actual backend comparison exists.

## Checklist

- [x] Recover #27304's existing queued ownership, placement and gate boundaries.
- [x] Open T03/Q1/Q2 owner evidence and inspect the Core/runtime producer seam.
- [ ] Complete bounded read-only representation and backend audit with exact source citations and explicit unmeasured hypotheses.
- [ ] Measure/retain GHC and held controls for scalar conversions, a surrogate round trip, forced out-of-range errors, unforced invalid conversions and output preceding a forced failure.
- [ ] Publish the conversion contract and focused physical helper placement before source edits.
- [ ] Commit the source checkpoint without absorbing ground or unrelated output work; candidate behavior remains predicted until built.
- [ ] Receive a later named builder handoff and run focused interpreter plus LLVM/Cranelift controls; record any backend boundary at actual scope.
- [ ] Kill a meaningful conversion/range mutation, restore the source and baseline, then complete the required paired corpus/replay/execution checks.
- [ ] Validate the final combined case stack with T03 producing GHC output and unit 519's T18398 checker rejection retained; preserve accepted/old-rejected membership.
- [ ] Record a local landing disposition. No published measurement update, push or release.

Case remains held for both fundep unit 519 and this character unit, alongside the older NonEmpty/infix Show output dependencies. The current ground builder continues independently; its success cannot certify this stack.
