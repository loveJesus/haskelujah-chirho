<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Source-typed Num Integer runs in the Rust interpreter

Owner HASKELUJAH/gpt_chirho. Unit 753 closes L.J.'s direction to continue the real-library route. Claude2 provided architecture and bounded source review; GPT wrote, executed and retains the pen. This follows 26-11-13_18-16-ai-ceo-report-next_real_num-chirho.md. No main compiler-source or public release acceptance is claimed.

Source is pushed at 1d6e51dad8996966b69aa3b2799fcb6a64138220 on next-pipeline-integer-num-chirho, crates b166f9df746d9e08aba68d3bfbe498636bf05f55. That source commit was read back before the metadata-only closure leaf 924308369842b49bf5290ee7ea9da0a818b3244b. Main compiler crates remain 04116a69eca349f4e7493fa60420d5aa53a4fb52. Canonical row 753 closed 2026-11-14T01:20:00.155Z; the final ledger/tasklist sync is metadata only.

The actual upstream Num Integer instance now runs addition, subtraction, multiplication, negation, absolute value and signum over the opaque pure-Rust Integer. Types come from six signature lines in the whole unmodified, hash-pinned GHC 9.14.1 Bignum/Integer.hs (60,857 bytes, 1f8ec2a8). The ordinary function names are not wired: HIR carries their source signatures plus a driver-manifest body declaration; TC checks closed Integer arrows and the exact arity; Core supplies explicit Integer-level primitive bodies; ordinary original-name linking joins them. No IS/IP/IN layout, GHC process or GMP is needed for these operations. The representation-specific bignum Haskell bodies are not claimed compiled.

Clause/head admission checks a known duplicate before partial-unit absence, even when the home instance omits methods. Other incomplete library scope/defaulting boundaries remain. The source Num class and its unchanged Integer instance remain slices, not complete ghc-internal modules.

| Gate | Measured result |
| --- | --- |
| Workspace/all-target check and CLI build | rc 0, zero compiler-message records |
| Healthy restored scoped tests | 142 next-driver +398 library/runtime/prototype =540 passed, none failed/ignored |
| Supported Integer binding evaluation | 17 exact results match GHC, including values beyond fixed widths and the 11/13/1 name-identity discriminator |
| Strictness | Both finite failing-operand × zero programs fail as GHC does; an unused partial application remains lazy and gives 7 |
| Deep demand / GC | GHC and next give 12 across 89 collections, and 10000 for a 10,000-deep non-tail sum across 108 collections |
| Native capability boundary | All nine tested emit/JIT entry points (LLVM, Wasm, Cranelift, JVM/BEAM) refuse opaque arithmetic even without a literal |
| Mutation | Resetting the arithmetic allowance per operation is killed; source restored byte-exact, healthy gates re-executed |
| File CLI / isolation | 17 expected file outcomes and check/check --dump-core with PATH=/usr/bin:/bin pass |
| Packaging | 528 listed paths, 3,661,433 existing-file bytes; four whole upstream source files, both licences and provenance included; stderr empty |
| Changed-span lint | 0 hits at exact 452a1c3a..1d6e51da; 260 OUTSIDE diagnostics remain unattributed |
| Secret scan | source and closure range (two commits), no leaks |

The owner-held arithmetic allowance reserves work/output before allocation and latches exhaustion: default 2^24 estimated 64-bit limb operations, 2^20 cumulatively charged output limbs, single-value ceiling 2^20 bits. Inputs remain immutable on failure. A source-valid large computation stops as Resource locally while GHC can return its sign; no semantic refusal or success is substituted. These are accounting limits, not CPU measurements.

Claude2 #34152 identified an operand-demand boundary risk. Core now forces operands left-to-right through DEFAULT cases inside every lambda; the primitive consumes WHNF and cannot nest the evaluator. #34155 cleared this fix at the 60-file source basis, source reading only. The generic primitive path already forced operands via frames, and consecutive primitive-return recursion was another reason to make the Core demand boundary explicit. We do not claim an independently executed pre-fix GC/stack failure.

A shared runtime growth cost remains. The deep-sum run takes 676,557 reduction steps, allocates 231,876 closures, ends with 282,655 code entries and about 190,000 last-live heap objects. Dynamic enter instructions and roots remain retained during a run. The tested run has an explicit step allowance, and dropping its machine retires that state; scalable code/root retirement is not solved. Existing module-wide Wanted rescans and inherited-interface copying remain open too.

The unchanged fixed 353-original check profile stays 22 ACCEPT /20 REFUSAL /311 Unsupported against unit 752, with all original source hashes held and zero movers/regressions. Its all.T flags/ways are omitted: this is neither a configured GHC compatibility ratio nor B-run acceptance. This unit's advance is supported numeric execution, not a change in that narrow bucket's count.

GHC reference data cover 17 byte-identical source programs. Two original recursive-loop evaluations hit their 8-second cap and remain timeout data; finite non-exhaustive-case twins replace the strictness obligation. The GHC-59692 duplicate, GHC-39999 home-class ambiguity and present type mismatch are semantic refusals. Hidden constructors and absent integerQuot remain Unsupported locally. The Integer rendering channel is a binding-evaluation harness, not a Show Integer implementation.

Limits: Num Int/Word/Natural, Eq/Ord/Show Integer, general defaulting, numeric literal patterns, full ghc-internal, native Integer/IO-main and B-run remain open. No full legacy-driver or whole-tree lint green is implied. Main's carried findings and historical evidence limitations are unchanged, not re-audited or waived.

The same source worktree and warm target were reused. No workspace or archive copy was added. The parent binary and superseded pre-forcing candidate are retained. The accepted CLI is f7c0395ac153716b4e89a5452a0e2b3caff0f9d36749ee7582882aac25f25e55 (60,393,848 bytes, 0555, nlink 1, fresh=false). A first zero-test filter, incorrect CLI command, and two derived licence/package assumptions are retained/corrected as method findings, not counted as product passes. Both source licences are byte-identical; the command correctly emits their shared complete notice once.

Architecture credit: Claude2 #34139; handoff reviews #34152/#34155. All were source-only, with explicit reading limits; final execution is GPT's, not independent engineering acceptance. The writer owns the final one-line UPSTREAM provenance update. No new agent work queue is opened by this report.

Evidence:
- Tasklist: spec-chirho/tasklists-chirho/26-11-13-chirho/26-11-13_19-09-tasklist-next_integer_num-chirho.md
- Acceptance: /Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-next-pipeline-s0-chirho/tmp-chirho/next-integer-num-753-chirho/checkpoint-chirho/accepted-chirho/analysis-chirho/acceptance-chirho.json
- Mapping/DAG: spec-chirho/workflows-chirho/next-pipeline-chirho/ghc-implementation-map-chirho.md
- Claude2 decision: tmp-chirho/next-integer-architecture-chirho/claude2-integer-boundary-decision-chirho.md
