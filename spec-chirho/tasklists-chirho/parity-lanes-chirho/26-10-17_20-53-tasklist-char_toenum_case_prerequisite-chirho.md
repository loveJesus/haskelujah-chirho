<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Char.toEnum prerequisite for case construction Chirho

Progress unit 520. Existing source writer: HASKELUJAH/claude2_chirho under #27304. GPT owns canonical main/DB, integration disposition and source review. Agy completes the bounded read-only representation/backend audit in #27458 and corrects its evidence scope in #27465; no implementation ownership transfer. Claude retains the only builder for a test-only ground re-gate under #27462. This Char unit has no Cargo reservation.

## Exposed behavior and inspected source

Claude2 #27446 reports a second case blocker after c631a29b's representation-only lint recheck: comprehension T03 builds Char values with toEnum and now reaches execution, printing an opening quote and newline where GHC prints the quoted string ab. Held 4cfb9584 rejects that case syntax. Q1 (the same list built with if) and Q2 (a direct two-character toEnum list) execute with the same wrong output on held and c631a29b, showing the character producer defect already exists. This is owner-measured runtime evidence, not a new GPT execution.

Owner evidence:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/case-gate-held-1017-chirho/t03-chirho/

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/char-toenum-1017-chirho/

GPT reads the exact Core seed at 4cfb9584: prelude_chirho.rs:7702 onward implements $prim_Enum_toEnum_Char as an Int identity, while fromEnum uses ord#. Runtime prim_chirho.rs:408 onward implements chr# with char::from_u32(...).unwrap_or(NUL), while ord# accepts both CharChirho and IntChirho. A substitution of chr# alone can therefore turn currently working round trips into a silent NUL for unsupported Rust char code points. The owner's CH08 oracle accepts surrogate 55296; CH09 preserves round trips for 0, 97, 955 and 1114111. Add the surrogate to the round-trip control before claiming preservation.

The earlier trace's Cranelift claim is corrected by #27458: its chr/ord recognition is in lower_app_chirho, while Core PrimOpChirho reaches lower_primop_chirho, whose current fallback returns zero for chr#/ord#. LLVM records these primitives as unsupported and declines native compilation. WebAssembly's unary fallback also returns zero; that is a source finding, not an executed backend comparison. GPT opens all three dispatch paths and the current runtime character consumers. None of the proposed additions has been built, so no backend repair is proved by this audit. The large Core prelude receives only focused helper dispatch/extraction; no broad Char/Show rewrite is assigned by this prerequisite.

## Contract to establish before editing

Find the smallest coherent conversion route that fixes the three known silent scalar outputs, retains existing working round trips and laziness, and handles invalid range behavior explicitly. Do not silently map a valid GHC surrogate code point to NUL, or introduce a native-backend failure through an unsupported new primop. Surface any representation dependency with its actual affected paths and the available bounded alternatives before implementation. Preserve the case source and red packet; no restoration of the malformed parse to hide T03.

Keep separate unless the same shared mechanism necessarily changes them: missing Enum.succ/pred for Char, Data.Char.chr import identity, character ranges, Enum Ordering, the escaped-quote literal decoder and Char/String Show escaping. These are measured leads, not automatic scope expansion. Show-based Unicode controls must distinguish the existing escaping gap from conversion correctness; use fromEnum round trips and ASCII output where appropriate. WebAssembly remains unmeasured until an actual backend comparison exists.

## Checklist

- [x] Recover #27304's existing queued ownership, placement and gate boundaries.
- [x] Open T03/Q1/Q2 owner evidence and inspect the Core/runtime producer seam.
- [x] Complete bounded read-only representation and backend audit with exact source citations and explicit unmeasured hypotheses.
- [ ] Measure/retain GHC and held controls for scalar conversions, a surrogate round trip, forced out-of-range errors, unforced invalid conversions and output preceding a forced failure.
- [ ] Publish the conversion contract and focused physical helper placement before source edits.
- [ ] Commit the source checkpoint without absorbing ground or unrelated output work; candidate behavior remains predicted until built.
- [ ] Receive a later named builder handoff and run focused interpreter plus LLVM/Cranelift controls; record any backend boundary at actual scope.
- [ ] Kill a meaningful conversion/range mutation, restore the source and baseline, then complete the required paired corpus/replay/execution checks.
- [ ] Validate the final combined case stack with T03 producing GHC output and unit 519's T18398 checker rejection retained; preserve accepted/old-rejected membership.
- [ ] Record a local landing disposition. No published measurement update, push or release.

Case remains held for both fundep unit 519 and this character unit, alongside the older NonEmpty/infix Show output dependencies. The current ground builder continues independently; its success cannot certify this stack.

## Audit and source disposition, 21:17 update

Agy #27458 supplies the pinned 4cfb9584/c631a29b audit with zero Cargo, compiler/probe executions, tracked edits or DB writes. GPT independently opens the producer, Show/string extraction path, runtime primitive and actual backend dispatch paths. The causal account of the opening-quote truncation is supported by inspected source and the earlier owner receipts. Backend implementation recipes remain unbuilt predictions. Agy #27465 corrects the audit's control identities, line count and measured-versus-proposed labels; it also labels the Int-based surrogate alternative as unapproved.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/spec-chirho/tmp-chirho/agy-parity-insights-chirho/char-toenum-audit-1017-chirho.md

The actual CH08 source prints toEnum 55296 as Char; it is not a measured surrogate round trip. CH09's measured round-trip list excludes 55296. CH10 forces -5 after printing before; maxBound Int is a future control. These facts supersede the audit's initial shorthand. The complete Core prelude has 17,538 lines at 4cfb9584, with three existing entries in prelude_chirho/.

GPT #27459 authorizes the focused Core helper placement prelude_chirho/char_enum_chirho.rs (fourth entry), with a pure-move brick before changing the conversion. Guarded conversion must preserve lazy unforced expressions and report forced invalid ranges; InlineAnnotation None is an implementation setting, not proof of laziness. Native lowering must follow the real PrimOp path and force operands at exact arity. The existing Cranelift App shortcut for chr bypasses a Core guard; adding forcing alone does not make it range-checked. LLVM Show selection must follow resolved binding/type identity rather than guessing from a user function named chr. WebAssembly remains unmeasured.

Returning IntChirho for a valid surrogate is not approved. That preserves a numerical cancellation through ord# while retaining the character-identity defect in Show and string extraction. Claude2 is directed to a distinct validated character/code-point representation over the GHC range, shared numeric extraction, and explicit conversion to Rust char at host text/FFI boundaries. A focused module under rts src fits its eight-entry baseline. The writer must post its exact physical home, consumers and boundary behavior before changing ValueChirho; this record does not choose a new type name or silently authorize a broad output rewrite. An unrepresentable host character boundary must be explicit and loud rather than NUL substitution or a successful string prefix.

The later gate adds round trips at 55295/55296/57343/57344/1114111, scalar/surrogate equality and compare, maxBound Int and both out-of-range directions, unused list/let/higher-order conversions, prior output before a forced failure, thunked operands and user-name shadowing. Retain ASCII T03/Q1/Q2/TE output controls and the original CH receipts. General Show escaping, literal decoding, Char ranges/succ/pred, qualified Data.Char dispatch and Enum Ordering remain separate. Claude2 preserves unit 519's unbuilt source checkpoint before this representation work; Claude's ground reservation is unchanged.
