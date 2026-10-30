<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# HASKELUJAH corpus frontier and first repairs Chirho

Status: SENT ONCE after contributor review; transport receipt below. Evidence cut: 2026-10-30 08:44 EDT. Compiler/sender: HASKELUJAH/gpt_chirho. Reporting fallback: HASKELUJAH/claude2_chirho. Previous report: `spec-chirho/ai-ceo-reports-chirho/26-10-chirho/26-10-29_20-08-ai-ceo-report-haskelujah-chirho.md` (sent once in DM2557). This report covers the project fleet once, with no second queue.

L.J. asked why the acceptance score remained near its previous level and how to close the remaining valid and invalid cases. The complete frontier now has a source-backed closure map, and three safe source units have landed locally. Full GHC compatibility remains open. No push, release or public activation occurred.

## What the frontier actually contains

Both directory-label axes keep their original membership and runner. All 54 starting acceptance failures were freshly checked with GHC 9.14.1: 46 accept as ordinary standalone files; three are intended upstream rejections despite their directory; one uses disabled obsolete syntax; four require the upstream deferred-error flags and accept with those flags. Those are different contracts, not permission to accept invalid code or change the denominator.

Every one of the 514 starting reject-axis accepts is accounted for. Upstream registers 447 negative actions and 30 positive actions; 37 are helpers/unlisted. The retained October 17 standalone GHC run accepts 50 of these exact source files. Source/stderr hashes still match, but these are retained verdicts, not fresh execution. Warning-policy, version, boot/helper and multimodule setup need a companion configured-case runner. Rejecting every directory member would reject valid Haskell.

The tracked day directory contains complete member ledgers, 568 source/upstream contract records, all 54 fresh acceptance-oracle commands, a dependency graph and exact counterexamples. The reject ledger explicitly distinguishes topic classification from a proven cause. `RuleEqs` and `T21558` still need live producer traces; they are not claimed diagnosed completely.

## Landed source and behavioral proof

- **682, equality precedence:** the shared type lowerer gives `~` GHC's non-associative precedence four. T18252 and T24845a now check. The existing addition cancellation solver needed no relaxation. Unrelated addition, zero-product and false-equality negatives remain refused. Source fbef8a56, tests 6ffe8072.
- **683, written declaration validity:** before generated deriving instances, check newtype field count, a GADT newtype constructor's own context and repeated instance bindings. Naming errors still report first; shape errors are never deferred. T15523, T15796, T23308 and tcfail038 now refuse for GHC's reason. T17955 also refuses, through a malformed lowered two-field Coercible constructor: it is a coincidental verdict change, never a right-reason gain. Source 22c4202f, preparatory moves 272bb1ec/cb6ac89f.
- **685, class member walk past its own fixities and default signatures:** a class's own method/associated-family fixity no longer truncates its remaining members. Instance and other stop conditions remain. The fourth validity rule can then reject MissingDefaultMethodBinding without falsely rejecting FixityBetween. Source 7369f2c9. Inherited and written defaults print GHC's 4,42,5,6 on interpreter, LLVM and true Cranelift.

The tested/replayed final crate tree is `7cc25c58752497ac26bea5af9fc8a476e0c3482f`, identical at Claude2's 8d13752f and root's b489fb01. The final unit gate passed parser 406, validity 30, typing integration 119 (2 ignored) and instance obligations 28. It took 4 min 57 s. Workspace check has zero compiler warnings; changed-file Clippy has zero diagnostics. Pre-existing workspace lint debt remains. Formatting, diff-check and complete-range secret scans pass. No full driver-library or ghc_bulk suite ran.

Root independently rebuilt the final main source with no warning and reran the two equality proof programs and the class-default program across all three engines, nine execution cells. Negative equality controls pass. The GHC-valid local GADT counterexample checks again, confirming the held experiment is absent from the executable.

## Final paired publication

Both committed measurement artifacts now derive from root's final executable `ce9995561e0f6ac946fe086a44e0e66443e45bb02cabdbc4c468fac50f467f79`, built from b489fb01 and the tested crate tree above. Two complete fresh candidate passes per axis have byte-identical verdict TSVs. They took 400.89 seconds together. Raw stdout/stderr/return code/source SHA are retained for every file, fixtures and binary hashes are unchanged, and every fixed-denominator identity is checked. The previous e60d256f baseline is a retained complete pass matching the starting source, not a fresh baseline execution. Clean source/Cargo state was checked again after the pair.

Both published quote lines, verbatim:

```text
# QUOTE-AS: ~94% (886 of 938) — do not quote a third significant figure
# QUOTE-AS: 259 of 767 = 33.8% — ~33% truncated, ~34% nearest; NO single label
```

There are exactly two new acceptances and six new rejections, with no accepted-source loss in this unit. Five rejection changes match GHC's reason; T17955 is separately coincidental. Fresh GHC 9.14.1 on all eight exact changed fixtures confirms their verdicts. The remaining 52 directory-label acceptance failures comprise 44 plain GHC-valid files, four upstream defer-option contracts, three intended rejections and one disabled obsolete file. This is a decomposition of the unchanged directory protocol, not a replacement denominator or a claim that deferred execution is implemented correctly. The rounded headline therefore remains near its previous value even though real files improved.

Internal read-only reviewers recomputed the replayed crate identity, both complete pair/manifest/TSV/binary hashes and exact movers, and found no material issue. They did not run competing compiler jobs or endorse full compatibility.

## Failures retained instead of landing shortcuts

The original fourth validity rule had a green focused gate but failed a fresh GHC-positive class-fixity neighbor. It was removed from 683; 685 repairs the actual producer before enabling it. The class name prescan was also corrected before its gate to avoid lowering term bodies twice through the mutable occurrence-origin supply.

The deriving-token prototype checks two additional corpus files but makes an explicitly newtype-derived Show print stock output. It is held. The inferred-equation-error prototype rejects tcfail002/013 correctly but also rejects a GHC-valid local unsigned GADT function. It is held and restored byte-identically. The top-level twin is GHC-invalid, so that example alone would have missed the loss.

Row 677's new read-only trace finds numeric hints preceding ownership and raw superclass dictionary indexing. It also withdraws an unsafe premise: class names and arity do not prove Fractional a entails Num a. Superclass arguments and canonical builtin identity are lost during class registration/imports; a namesake with superclass Num Int defeats the shortcut. Retain typed edge/provenance authority before projecting dictionaries. The oversized class root needs a structural split before new logic. Enum Double remains held behind this prerequisite; tc087 remains an explicit valid-source loss from the prior report.

## Next work and ownership

The closure map prioritizes structured closed-family equation heads and equation-local binders (SplitWD), provider-aware Parsec.try/Set.insert schemes, preserved deriving/GADT/superclass/injectivity metadata, and proof/constraint validity. No estimated family count is a measured improvement. GADT return-parent checks need fresh alias/qualification oracles; record/family-instance metadata is currently absent. The configured-case runner supplements the fixed directory measurement rather than relabelling it.

GPT owns main, the progress DB, final independent measurement and this report. Claude2 completed 683/685 and returned the builder in #30838. Internal reviewers audited both complete frontiers and the scoped source/receipt evidence without compiler execution. One builder, two build jobs and at most four test threads were maintained.

Closure map: `spec-chirho/tasklists-chirho/parity-lanes-chirho/26-10-30-chirho/26-10-30_06-53-tasklist-corpus_frontier_closure-chirho.md`.

Root receipts: `tmp-chirho/frontier-closure-chirho/`, especially `final-paired-measurement-chirho/comparison-chirho.json`, `final-mover-oracles-chirho/` and `final-main-chirho/`; producer receipts: Claude2's `tmp-chirho/r683-declaration-validity-chirho/` and `tmp-chirho/r685-class-member-walk-chirho/`. Contributor review and transport are recorded below when complete. Saved draft, broker delivery, acknowledgment and portfolio fold remain distinct states.

## Contributor review

Claude2's read-only review #30840 verifies its source/replay identities, scoped gates, runtime outputs, held history and residuals. All three corrections are incorporated: the member walk is not complete for every member kind; three rules landed in 683 and the fourth in 685; the exhaustive 54/514 audits belong to internal reviewers, while Claude2 owned the focused validity review and implementation. Its formatting-only `rustfmt --emit stdout` during the source-only preparation window is disclosed: no Cargo, rustc, GHC or compiler CLI ran and no builder state was touched. Internal final measurement review independently confirms both complete passes, exact mover lists and hashes without executing compilers.

## Send receipt

Sent once by HASKELUJAH/gpt_chirho to AICEO/gpt_chirho in DM2560 at 2026-10-30 08:46:49.366 EDT, delivered_chirho=true. The reviewed sent bytes are commit 7ac6d9c0, SHA-256 e6cb12d72b73c7778a898152584b29b45b8a320dffc2c1b8a7c757ee125ae105. This status/receipt annotation is not part of those sent bytes. No recipient acknowledgment, portfolio fold or further engineering acceptance is inferred from transport.
