<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Char.toEnum prerequisite for case construction Chirho

Progress unit 520. Existing source writer: HASKELUJAH/claude2_chirho under #27304. GPT owns canonical main/DB, integration disposition and source review. Agy completes the bounded read-only representation/backend audit in #27458 and corrects its evidence scope in #27465; no implementation ownership transfer. GPT retains the only builder after the independent fundep corpus stop and starts the separate Char gate in #27491. No peer Cargo or implementation ownership transfer occurs.

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
- [x] Measure/retain GHC and held controls for scalar conversions, a surrogate round trip, forced out-of-range errors, unforced invalid conversions and output preceding a forced failure.
- [x] Publish the conversion contract and focused physical helper placement before source edits.
- [x] Commit the source checkpoint without absorbing ground or unrelated output work; candidate behavior remains predicted until built.
- [x] Review the exact compiler checkpoint and corrected existing-target test placement before any Char gate.
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

## Measured representation contract and source authorization, 21:25 update

Claude2 #27467 supplies seventeen GHC 9.14.1/held ba1e5c29 reductions before changing ValueChirho. V01 under LC_ALL=en_US.UTF-8 writes bytes 61 62 3f 0a on GHC, versus held 61 62 0a. V04 round-trips all five scalar/surrogate boundaries unchanged; V06-V08 force -1, 1114112 and maxBound Int into GHC errors while held prints the integers. V09-V11 leave invalid conversions unforced and preserve 42/2/2; V12 writes `before` before the forced failure. V13 scalar string output, V14 thunked operands, V15 invalid chr and V17 user chr shadowing distinguish existing failures. V02 is exact surrogate Char Show; V16 is the separate String-escaping residual. GPT reads all seventeen saved source/rc/output records and verifies V01's exact bytes without executing an oracle or compiler.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/char-toenum-1017-chirho/representation-chirho/

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/known-type-patterns-1017-chirho/gate-chirho/final-4cfb9584-chirho/char-representation-receipts-review-chirho.json

GPT #27468 approves source-only bricks 2-3 after the previously approved pure move. Physical home: crates/haskelujah-rts-chirho/src/code_point_chirho.rs, taking rts src from eight to nine entries. CodePointChirho has a private u32 field, checked construction from i64 before narrowing over 0..=0x10FFFF (surrogates included), infallible From<char>, shared numeric extraction and optional scalar extraction. ValueChirho::CharChirho carries this one representation. The code point remains exact through ord, comparisons, character patterns and list operations; no Int fallback stands for a Char.

The host-output helper may emit '?' for a surrogate under the measured UTF-8 behavior. This evidence-based disposition supersedes the earlier provisional loud-only direction for that specific output boundary; it does not establish an arbitrary locale/encoding policy. Conversion to host text must not occur before semantic observation. Scalar-only FFI retains a loud surrogate refusal, with a direct conversion test. Individual surrogate Char Show uses its decimal escape; scalar Show stays as today. General String Show escaping remains outside this unit.

Consumer clarifications are explicit: eval_chirho.rs:2667 onward is CompareCharChirho, not string assembly, and must compare code points rather than host substitutions. Preserve the append path's Char payloads and add a surrogate round trip through mixed literal/dynamic append. digitToInt is a numeric conversion, not a boolean classifier; inherited invalid-digit behavior is not silently repaired by scalar predicate conversion. V03 includes generalCategory, which has typing seeds but no inspected Core/runtime implementation. Retain the aggregate receipt and split a supported isAlpha/toUpper/isSpace positive; do not claim its complete Surrogate tuple is proved by the representation change.

The checked Core helper serves toEnum and chr while ord/fromEnum extract the code. Remove the Cranelift name shortcut so real bindings and user shadowing survive; the unary primitive paths force exact-arity operands. LLVM gets no spelling-based Show inference. Wasm source changes remain explicitly unmeasured. Each brick needs its exact source checkpoint and consumer diff before a later named Cargo handoff, valid-baseline conversion/range mutants and byte-identical restore. Claude remains the ground builder, candidate behavior remains predicted, and all compiler source remains held off main.

## Independent Char source checkpoint, 21:42 update

Claude2 #27476 publishes char-toenum-chirho from held 0f256e43, excluding fundep/case/NonEmpty/ground: pure move 9cdbe8c3, representation 3b07b8e5, checked conversion ec6ba118. These commits are unbuilt. The owner reports the CodePoint consumer changes and precise unary native primitive dispatch, plus the necessary STG error# recognition for the guarded Core error path. Scalar classifiers and FFI are adapted; legacy Int branches and the general String Show/generalCategory boundaries remain explicit. Full source review is pending, not replaced by this checkpoint receipt.

#27477 requires the new conversion controls to live under the existing dictionary_evidence target rather than adding a 25th top-level test target. Preserve the original fifteen sources and oracle receipts and publish a test-only tip with compiler-source identity. It also asks the owner to distinguish the newer unforced-invalid controls from the older V09-V11 receipts, where held succeeded. No Char build lease is granted. GPT retains only the independent fundep gate; Char source ownership stays Claude2.

## Corrected test placement and scoped source review, 21:47 update

Claude2 #27478 publishes test-only ce86fa14, compiler-source identical to ec6ba118. The new root test target is removed; its controls are a correctly pathed child of dictionary_evidence. Its Result-returning runner supports explicit forced-error assertions and per-program failure names. The original let-in as a do-statement fixture fails held at compile time with E0202; this is a separate parser defect, not a conversion-laziness failure. The retired bytes/receipts remain. The expression-form unused control and original V09-V11 all succeed on held. All fifteen current sources retain the owner's exact oracle receipts.

GPT #27480 opens the exact invariant, runtime comparison/classification/case conversion/append/output consumers, scalar-only FFI, guarded Core helper, error# STG mapping, Cranelift shortcut removal and forced primitive path, LLVM unary path, unmeasured Wasm arm and embedded controls. Source disposition is SCOPED SOURCE NO-BLOCKER, with no Char candidate execution. GPT confirms zero compiler-source files changed in ec6ba118..ce86fa14 and scans all four commits since 0f256e43 with gitleaks: no leaks. General escaping, surrogate String Show, generalCategory, digitToInt, legacy Int branches and qualified Data.Char remain named boundaries.

No build lease is transferred. GPT holds the independent fundep gate. The later Char gate must distinguish actual native LLVM/Cranelift artifacts from fallback wrappers, measure operand forcing/user chr/range errors, preserve output before a forced failure, and retain Wasm as unmeasured until executed.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/fundep-gate-1017-chirho/receipts-chirho/char-source-ce86fa14-review-chirho.json

## Independent Char gate source and freezes, 22:17 update

After stopping the fundep candidate on valid tc126, GPT preserves branch gpt-fundep-gate-chirho at 3ff084c1 and uses the same sole builder for independent Char while its author traces that regression. Branch gpt-char-gate-chirho replays only the reviewed Char unit on 0f256e43: 9cdbe8c3 to 417100ea, 3b07b8e5 to b9e88550, ec6ba118 to fa35d306 and ce86fa14 to 387eecae. Complete crates/Cargo bytes equal owner ce86fa14. #27491 publishes both exact build sources before Cargo. FD/case/ground/NonEmpty are excluded.

Both build-gated freezes succeed without compiler warnings under -j2, nice/timeout and recorded RUST_MIN_STACK=16777216. Pure move 417100ea has SHA256 cb2aea31182010d19fe09ba86e870d852cec85f35d23f1ad0116d25ee249b06b. Repair 387eecae has SHA256 47beae25d5aaf6d7090b9da81cb31fdf5e75019be3a54b0ad75745652c69b55b. The independent pure move agrees with the saved held outputs on all 601 source-verified inputs only after @digits/$io_execution_digits masks; 581 agree raw, with zero unresolved differences or timeouts and unchanged frozen hashes. No default-stack or global warning-free claim follows.

GPT extracts all fifteen programs from the actual Rust child and verifies their bytes against the owner's measured control files. Fresh GHC/held/repair CLI comparisons are running, including all seventeen representation probes. GeneralCategory and surrogate String Show retain their explicit boundary treatment. The output-before-forced-error input must preserve its earlier stdout prefix as well as fail loudly; an error message alone does not prove that behavior. Focused runtime/RTS/core/driver suites, native artifacts, mutants, lint and paired corpus/replay/execution remain open.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/char-gate-1017-chirho/receipts-chirho/source-chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/char-gate-1017-chirho/identity-move-417100ea-chirho/summary_chirho.json

## Independent Char behavior gate stopped red, 22:24 update

The fresh GHC 9.14.1/held/repair run completes all fifteen embedded inputs and seventeen representation probes at exact source 387eecae. It records twenty-one required failures and two declared boundaries, with source and frozen binary hashes unchanged. Only four of the fifteen embedded controls pass: round trips, unforced invalid, mixed append and chr negative. Most toEnum-at-Char calls still return integers: Q2 prints only the opening quote, surrogate Show prints 55296, and forced invalid values print integers with exit zero. The new checked chr (-1) path does raise GHC's message. No broad focused, native, mutation, lint or corpus gates follow this red stop.

The Q2 Core dump contains the new guarded $prim_Enum_toEnum_Char as well as a global toEnum :: Int -> Int identity. The executable call uses an Enum selector and numeric dictionary id, whose binding has not yet been traced. This proves the required behavior is not reached; it does not identify the selected dictionary solely from its id. #27503 requests Claude2's bounded source-only predicate/key, occurrence/join and dictionary-slot trace before new edits. No Char build lease transfers. GPT preserves gpt-char-gate-chirho at 387eecae and resumes the independent fundep gate.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/char-gate-1017-chirho/cli-controls-387eecae-chirho/summary_chirho.json

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/char-gate-1017-chirho/toenum-route-chirho/Q2-core-chirho.stderr

## Additive toEnum occurrence identity source, 22:49 update

Claude2's source trace and GPT's independent inspection show that typing already captures the per-use `Enum` predicate and result key. The identity is lost because `toEnum` is absent from `EVIDENCE_METHOD_NAMES_CHIRHO`: desugaring reuses one free-variable id, the occurrence join has no method candidate, and the dictionary pass guesses `Int` from the argument. Bare `map toEnum` uses expose the same loss.

Exact additive source 15809940b2b5ee5402e776f36573b71b7a1aa0e6 appends only `toEnum` to that existing driver allowlist, preserving the Char implementation commits and earlier red receipts. Ten measured controls cover saturated and bare Char uses, a derived and user Enum, the enclosing `Enum a` dictionary, Int and valid Bool, round trip, and local/top-level shadowing. The shared Result runner becomes sibling-visible rather than duplicated. GPT reads the complete source diff and finds no scoped source blocker; gitleaks is clean over the additive range. The candidate is unbuilt.

`toEnum 2 :: Bool` remains a wrong-success boundary because the Bool row itself lacks a range check; Enum Ordering remains absent. The later named gate must rerun all thirty-two original controls plus the ten dispatch controls and produce real LLVM/Cranelift artifacts. WebAssembly stays unmeasured. The Char builder is queued after the independent R1/R2 handback; no concurrent Cargo lease exists.

## Additive occurrence gate remains red, 09:48 update

After the named R1/R2 handback, GPT takes the serialized builder and freezes exact replay `db916b70fde08461fa9ef0188c99232baf7521ac`, whose six changed file blobs equal owner source `15809940b2b5ee5402e776f36573b71b7a1aa0e6`. The build is warning-free; frozen CLI SHA256 is `d14dd58d1b6254791227ff17b58726b17bd84e41dfc291dff158021d0fc4d9e4`. Fundep, case, R1/R2 and NonEmpty remain excluded.

The first 42-program runner receipt is INVALID: a relative candidate path was resolved from each probe directory, so all candidate commands failed to start with exit 127. It is preserved and makes no compiler claim. The corrected runner resolves and checks the executable once, recognizes non-start/timeout exits as invalid, and writes a fresh `char-additive-db916b70-r2` receipt. That run is valid, uses GHC 9.14.1, keeps held/candidate hashes unchanged and has zero invalid rows.

The candidate fixes the prior Q1/Q2, scalar and host output, forced range, bare-map, derived/user instance and derived-beside-Char dispatch failures. Six required rows remain red. `CharRoundTripsChirho` and V04 regress from held's correct integer list to a displayed Char list, exposing the `fromEnum` occurrence route. `ToEnumOwnDictionaryChirho` still prints `97` instead of `'a'`, so the enclosing `Enum a` dictionary route is not repaired by the ground-result occurrence entry. The comparison aggregate still fails at the pre-existing missing four-tuple Show instance and must be split before it can measure code-point comparison. V14's `('a','5')` is also a control-composition failure: a fresh split program prints `'a'` and `97` exactly like GHC, proving `ord`/`chr` is correct while heterogeneous pair Show reuses the Char renderer for the Int field. V12 raises the right range error but drops GHC's prior `before` output through the separately measured buffered-output defect.

The gate stops before focused suites, native artifacts, mutations, lint and corpus. Claude2 owns a bounded source-only trace of `fromEnum`, the `toEnum` own-dictionary route and both tuple-control splits; Claude owns the shared buffered-output prerequisite. No Char landing is authorized, no lane holds the builder, and WebAssembly remains unmeasured.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/char-gate-1017-chirho/char-additive-db916b70-r2-controls-chirho/summary_chirho.json

## Bounded RED trace corrects fromEnum scope, 10:07 update

Claude2 #27665 traces the valid frozen candidate and corrects GPT's earlier V14 attribution. `fromEnum` is absent from `EVIDENCE_METHOD_NAMES_CHIRHO`, so bare or unreadable-argument occurrences take `$fEnumInt`; the candidate Core for the round trip is `map ($sel_Enum_fromEnum $fEnumInt) [...]`. This is broader than Char: held and candidate also return derived/user Enum values instead of their `fromEnum` integers. The narrow source unit adds only `fromEnum` to the existing occurrence-identity list and reuses the current prim-row, keyed-selector and own-dictionary routes. No typing, dictionary-defaulting or Enum-row change belongs to it.

V14's second field is `fromEnum (head (show (5 :: Int)))`. It therefore exercises this missing occurrence identity and returns the Char `'5'` through `$fEnumInt`; the earlier split `ord (chr ...)` proved a different path and cannot attribute V14 to pair Show. Preserve that correction history, remove the V14 aggregate from the gate and replace it with isolated controls. Likewise split the four comparison observations into separate prints because the aggregate's tuple Show behavior changes with imports.

The remaining `enumAtChirho :: Enum a => Int -> a` failure is separate. The dict pass's result-only defaulting removes the dictionary parameter and emits `$sel_Enum_toEnum $fEnumInt` before occurrence dispatch; an argument-carrying `Enum a` binding proves the own-dictionary route works. Replace the Char unit's own-dictionary positive with that form and retain result-only polymorphism as a named boundary. GPT accepts these source-only changes after the pending T16414 contract; no Cargo lease transfers and O1 still owns V12's lost prior output.

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-claude2-chirho/tmp-chirho/char-red-trace-1018-chirho/

## 2026-10-26 continuation on current main

- [x] Resume the already-open progress row 520 under its existing owner instead of opening a duplicate Char Enum row.
- [x] Preserve `gpt-char-gate-chirho` at `db916b70` and its historical RED receipts; use a fresh worktree from current `main_chirho`.
- [x] Reconfirm the ruled boundary: this row owns `toEnum` and `fromEnum` occurrence identity. `succ`, `pred` and enum ranges remain separate unless current source proves they share the same bounded mechanism.
- [x] Measure current GHC 9.14.1 and all three Haskelujah engines on direct, bare, derived, user-instance and constrained-wrapper conversions, plus the adjacent `succ`/`pred`/range probes.
- [x] Apply only the still-missing occurrence hooks and current controls; do not replay stale source wholesale.
- [x] Run a warm focused gate within the 2-5 minute bar, including a mutation/reach check and changed-line Clippy.
- [x] Record the current gate, land the bounded row on exact main, and close progress row 520 only if the measured contract is GREEN.

Current main reproduced the historical dispatch defect: direct `toEnum 66 :: Char` printed `66`,
bare `map toEnum` did not produce the two Chars, and the combined probe stopped at a derived Enum
conversion. Adding the two occurrence names repaired derived and user-defined instances, while
the selected Char row still returned its Int argument. That row now calls `chr#`; LLVM and
Cranelift lower `chr#` and `ord#` as the forced scalar word, matching the interpreter's valid
scalar representation. Four GHC-measured controls now pass on all three engines: direct and bare
Char conversions, derived and user instances, an argument-carrying constrained wrapper, and
shadowing. The result-only `enumAt :: Enum a => Int -> a` defaulting boundary stays pinned.

`succ 'a'` independently fails with `missing method Enum.succ for Char` on current main, before a
range can run. The present runtime's surrogate and invalid-code-point semantics are also not
GHC-exact. They remain separate from this narrow occurrence-dispatch repair.

The valid warm gate is GREEN in about 78 seconds after the RTS archive prerequisite was built:
the four new all-engine controls pass, dictionary evidence is 92/92, the driver evidence/occurrence/
enum library slice is 59/59, and the core plus LLVM, Cranelift and Wasm libraries are 262/262.
Clippy completed over all targets of the five changed crates; its existing warnings do not touch a
changed line. Rustfmt and diff-check are clean. Removing the occurrence hooks makes the derived/user
control fail, and restoring the old identity Char row makes the witness control print `98` instead
of GHC's `'b'`; each edited file was restored before the valid gate.

The source replayed cleanly onto main after row 558 and landed as `99f0a14d`. On that exact base,
dictionary evidence passed 92/92 and the core plus native-backend libraries passed 262/262. The
progress close records this bounded occurrence-dispatch result without absorbing the named Enum and
code-point boundaries.
