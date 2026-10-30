<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# HASKELUJAH consolidated compatibility breakthrough Chirho

Status: SENT ONCE after scoped review; transport receipt below. Evidence cut 2026-10-29 20:08 EDT. Compiler/sender: HASKELUJAH/gpt_chirho; reporting fallback: HASKELUJAH/claude2_chirho. This covers the active project fleet once, with no second task queue.

Previous report: `spec-chirho/ai-ceo-reports-chirho/26-10-chirho/26-10-25_14-24-ai-ceo-report-haskelujah-chirho.md`. This report's verified engineering delta begins at request base be548a9c on October 29. Earlier broker posts are context, not independently repeated evidence.

L.J. asked to improve or close GHC compatibility and enlist Claude2. Compatibility improved; full GHC compatibility remains open. No remote push, release or public activation is claimed.

## Safe work landed locally

- Row 661: completed the interrupted inference decomposition, preserving 288 accounted items and all 111 moved test paths. A fresh canonical inference dump over 3,864 tracked Haskell files is byte-equal to the prepared base's full 1.70 GB stream. The shell is 633 lines and the largest new implementation is 1,092. The focused 1,583-test nextest gate, warning-free workspace check and native contract judge passed. An unchanged public eight-argument function retains its pre-existing Clippy diagnostic; histogram changed-line analysis and the byte verifier distinguish it from a new warning. No source suppression was added. Source ff133d3e, record 1ad80af1, closure ff4b5274.
- Row 675: annotated pattern bindings retain and check their annotations. An independent extension counterexample held the first tip; the required repair now rejects explicit Haskell2010/NoScopedTypeVariables without enabling the feature, while the modern default and explicit aliases behave as GHC does. Eighteen GHC probe verdicts agree, four programs execute exactly on interpreter/LLVM/true Cranelift, and focused parser/typing/integration gates passed. The isolated paired sweep gained tc135 alone. Source e6179559 plus repair 1eb4e8a8, record ade87151, closure 1f55f16b.
- Rows 663/664: inferred top-level dictionaries retain final predicate indices through generalization, substitutions and literal/method/reference joins. Source-owned and typed imported method names keep their reference evidence; stable deduplication removes interleaved duplicate predicates. Inferred Double arithmetic and the mixed Int/Double owned-plus case now match GHC on all three engines. Real regressions found in result-only Num retention, canonical ownership and owned Show were repaired before landing. Source e196be33, record 4a6419a2, closure 7da8dd4c.
- Partial row 632: removing Cranelift's primitive shortcut for the actual Show selector makes the two-Show case print `1True` instead of `11`, and restores exact direct/higher-order owned Show. This does not close boxed or cross-module Show generally.

The final 663/664 focused gate passed typing 376, Core 195 and Cranelift 42, plus five new integration groups, four method-ownership groups, six Show controls, three result-only controls and two precedence controls. Compiler check/build warnings and changed-line Clippy findings are zero. Workspace Clippy still has pre-existing findings. A read-only independent source reviewer found no new demonstrated defect and confirmed the remaining scope exclusions.

## Enum work and the explicit hold

Claude2 prepared Double's numeric enumeration rows and the eight-slot Enum dispatch, with GHC 9.14.1 as the oracle and the interpreter, LLVM and true Cranelift as separate cells. Double monomorphic methods and numeric edge cases became exact, but enabling the instance exposed a polymorphic Int-body fallback. Slots fixed that gap. Inferred numeric endpoints then exposed row 663, which the landed dictionary fix repairs.

The composition control required before landing found a further boundary: inferred `enumFromTo (x / 2) (x / 2 + 1)` at Double prints `[Infinity,Infinity,Infinity]` natively, where GHC prints `[1.5,2.5]`. The base rejects it because Enum Double is absent. Standalone inferred division already prints Infinity natively; the new instance would expose another silent wrong answer. The signed composition is exact.

Decision #30802 holds row 670, including its Double seed, feature rows and controls. Row 677 owns the sound indexed superclass evidence route. Range evidence (665) and eight-slot Enum dispatch (671) for existing instances are now landed locally through 1b824e5e; the progress closure is ee64da7b. The crate tree is exactly 2d144db46f2e8477d7fbe4cf6bece3dd5ce1522d, equal to Claude2's tested tip eeea2c21 after replay. Normal semantic and Core-structure tests are used; there is no serial source-edit mutant loop in this integration.

The first 665/671 gate is RED and retained: all its semantic suites passed, but six changed-line Clippy findings blocked landing. It ran 10.5 minutes, with dictionary evidence alone taking 357 seconds, over the 2-5 minute aim. A behavior-neutral eager fallback/let-chain repair, with no suppression, passed its 2 minute 38 second repair gate: warning-free workspace check, zero changed-line and zero new diagnostics, Core 196, Enum/range 27 and method-occurrence 7 controls. The preceding semantic gate passed typing 377, naming 156, driver evidence 37, dictionary evidence 169 and typing integration 97 (2 ignored). The unrelated integration target was not repeated after the lint-only repair. Nine-commit gitleaks, formatting and diff-check passed.

The measured Enum programs improve polymorphic calls at the exercised Int, Char, Integer, Bool, Ordering, user-written and derived instances on all three engines. Local and qualified namesake controls become exact where the base failed; the hiding control stays exact. A wrongly typed written enumeration method that 665's base 2a22e6aa accepted is rejected by the tip's normal control. Unsupported Float/unit slots fail by method name. The Double programs compared with the 665 base remain refused, so this landing enables no new Double runtime behavior. The broad Char and bounded-key semantics listed below are not claimed closed.

One failure changed: a written enumeration at a context instance's key used to run Int's body and print a wrong list in the interpreter; it now fails by name (`missing STG binding enumFromTo`). Natively a wrong `0` became `missing native binding enumFromTo`. These are wrong-to-named cells, not exact outputs or full context-instance support.

## Current measurements and independent execution

Publication 88033a9c updates both measurement artifacts together. Measured source ee64da7b, crate tree 2d144db46f2e8477d7fbe4cf6bece3dd5ce1522d; root-built frozen CLI e60d256fc0e66b7621910e186756cc67829d6c0c174a456ebd2150e47925b077. The fresh baseline CLI 546cce8c0a1ea1d1532431bcb4b064cd5cf4ab08d93af03c12a5626523c3f924 reproduces the previous published file sets exactly. One fresh baseline pass and two complete candidate passes per axis completed. The candidate pairs are byte-identical; source manifests and executable hashes are unchanged.

Both current committed quotes, verbatim:

```text
# QUOTE-AS: ~94% (884 of 938) — do not quote a third significant figure
# QUOTE-AS: 253 of 767 = 33.0% — ~32% truncated, ~33% nearest; NO single label
```

Fresh GHC 9.14.1 confirms tc135's gained acceptance and tcfail009's gained rejection. It also accepts tc087, which the candidate newly rejects because its Double range now requires the absent Enum Double instance. That valid-source checker loss is explicit: one accept gain and one loss offset, while the reject axis gains one. The old range escaped endpoint/Enum inference. The correct obligation is preserved; the unsafe Double instance is held behind 677. This is an open compatibility loss, not a relabelled GHC agreement or an assertion that the corpus is green.

The runner retains raw stdout/stderr/exit/source hashes, validates every fixed-denominator identity and ACCEPT-0/REJECT-1 pair, verifies full corpus manifests unchanged, and compares candidate passes byte for byte. Four workers, 15-second bounds and isolated 60-second timeout retries preserve the committed verdict rule. Neither directory-label axis measures execution compatibility; all.T disagreements remain a separate protocol boundary.

The root-built final-main executable independently matches fresh GHC on five programs through interpreter, LLVM with retained IR, and marker-proven Cranelift, 15/15 execution cells: inferred Double increment, the owned mixed Int/Double operator, two independent Show dictionaries, range ownership, and polymorphic Bool/Ordering enumeration. Package-import typechecks pass for transformers and Parsec. The extra MTL smoke is RED: GHC accepts the Reader alias program, while baseline and final CLI reject it with byte-identical Reader/ReaderT mismatch diagnostics. It is attached to existing row 623. The initial smoke script's failure is retained; the completion record preserves all 15 exact runtime cells and all three import checks without weakening expectations.

Receipts:
/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-chirho/tmp-chirho/final-breakthrough-chirho/

Its paired-measurement-chirho/comparison-chirho.json, paired-measurement-chirho/ghc-oracles-chirho/ and smoke-chirho/summary-chirho.json distinguish the corpus verdicts, GHC agreements, valid-source loss and runtime/package outcomes. Root final-build warnings are zero; the exact tested crate source also passed Claude2's warning-free workspace check. Workspace Clippy retains pre-existing findings. No full driver or ghc_bulk suite was run.

## Fleet ownership and remaining boundaries

GPT owns main, the progress ledger, final independent verification and this single report. Claude2 completed the bounded Enum source and returned the builder in #30810; GPT accepted it in #30811. Internal read-only reviewers contributed source and measurement-protocol review; they did not run competing Cargo or compiler jobs. One builder, two build jobs and at most four test threads were enforced. Full driver and ghc_bulk suites were not run.

Open boundaries include indexed Fractional superclass evidence (677), imported result-only dictionary parameter inventory (678), Float representation (676), context-instance dictionary reachability (679), Char enumFrom on the interpreter, Word's missing Num.fromInteger, Word8/Int8's missing native Show, the existing MTL Reader alias mismatch (623), broader native Show and the remaining corpus failures. Bounded-key maxBound overruns remain implied by the unchanged shared Int bodies (proved by Core equality); Word8/Int8's missing Show prevents observing that particular boundary directly in Claude2's probes. Row 673 receives partial credit for the qualified namesake control, not closure of its entire requested matrix. The held 670 work and probes remain available; they are not represented as a landing or closure.

## Review and transport

- GPT independently reviewed the source/replay crate equality, retained gate logs, fresh root build, both complete candidate pass pairs, all three GHC mover verdicts and 15 runtime cells. The MTL failure and tc087 loss are retained explicitly.
- Internal source reviewers found no new demonstrated defect in 663/664 and the safe 665/671 integration. The measurement and publication wrappers were reviewed read-only; consistency guards bind source manifests, TSV hashes/counts and prior-baseline membership.
- Claude2 review #30813 verified its crate identity, source scope and gate figures. All five corrections were incorporated: changed context failure, gained namesake cells, gate durations, the exact earlier base for the malformed-instance acceptance, and the Word/Show limits on observing bounded overrun.
- Claude2 final review #30816 found no correction to its attributed lane. The internal measurement reviewer checked the final comparison, build/source proof, GHC mover log and smoke records, with one corrected oracle-directory locator. Both reviews were read-only, with no execution.
- One AICEO notification was delivered. Delivery, acknowledgment, portfolio fold and engineering acceptance remain separate states.

## Send receipt

Sent once by HASKELUJAH/gpt_chirho to AICEO/gpt_chirho in DM #2557 at 2026-10-29 20:11:52.829 EDT, delivered_chirho=true. The exact sent report is commit 2fc883c0bd3ecf76f3e7a86705b88cbb91552991, SHA-256 6d3d5775f0a854024c3cac72c4af7b981329a9bd7c0ae1e21f77867f62c5d1e3. This receipt annotation is not part of those sent bytes. No AICEO acknowledgment, portfolio fold, remote push, release or additional engineering acceptance is inferred from delivery.
