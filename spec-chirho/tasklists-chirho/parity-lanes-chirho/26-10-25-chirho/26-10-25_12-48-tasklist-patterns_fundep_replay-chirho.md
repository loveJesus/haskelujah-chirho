<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Patterns and fundep stack replay onto current main Chirho

L.J. directed the fleet on 2026-10-25 to "continue with GHX test passing", read as the GHC tests. The two published typecheck quotes have not moved since 10-17 (3199bf9c). The measured gains held off main come first: the patterns stack (row 504, gate GREEN on 4cfb9584) adds six GHC-confirmed should_fail rejections, and fundep unit 519 on top of it (gate GREEN on bc930d30, recorded in c1e04ecb) adds four more and moves tc231 to the rejection GHC 9.14.1 also gives.

Owner: claude2_chirho replays source-only on branch `patterns-fundep-main-replay-chirho` in its own worktree (gpt_chirho GO #28243). gpt_chirho owns the builder, main and the DB; progress unit 531 (#28246). RC5, case, Char, NonEmpty (517) and Ord are not absorbed. RC5 lands first, so this stack is advanced onto the post-RC5 main before any gate (#28254).

## Base and range

- Base: canonical main 0ec9f8f5, a DB-only child of bd690d83 (same crates tree).
- Held source: 492aa6d8..c1e04ecb, 32 non-merge commits and 6 merges. The merge base with main is 492aa6d8.
- Replayed: 28 commits with `cherry-pick -x`, in held order. Then come the style commit da817309, the docs commit eed6f558, the placement move 05d60cf0, and this record's update.
- Not replayed, already on main as patch-equal commits: 3ed4d3b4 = 8042b797, 4d51fe74 = e166b4c9, f286f173 = cf465667, 11ffb89d = 5c41fd88 (the generic-head identity series).
- Not replayed, merges: five equal their automatic merge. 40e61a8f carries one manual resolution, the `instance_signatures_chirho` and `pattern_bindings_chirho` module union, which 616647af reproduces line for line.

## Commit map

Patch-ids are `git patch-id --stable` of each commit's own diff.

| # | replay | held | patch-id | why it differs |
| --- | --- | --- | --- | --- |
| 1 | 616647af | ddb700b8 | differs | module union with main's `instance_signatures_chirho` |
| 2-9 | d370ec4e, d1eaf3fb, a32b61db, 25b099ee, 55802a1d, ff6dcc99, 34edf03f, 450d9db5 | bf80c304, e61fb960, 93a295db, 83827999, a2182700, 5f0365e5, ba762d3c, 49b71e7b | equal | |
| 10 | 4e79027c | 39aa0d7f | differs | parser move carries main's current body; typing keeps main's module layout |
| 11-12 | 500bbdb0, 8c95908f | 8fd832b1, 97b400e5 | equal | |
| 13 | 786dc3b0 | be4b6ce7 | differs | deriving seam, below |
| 14-16 | 741eb905, 6c814c38, 40a52cf2 | 29be1b7f, 6b43ebf5, 3ece4a33 | equal | |
| 17 | d5c8b44c | c32f071a | differs | the lowering rule is already on main (cfd829e3); adds the held parser control, tasklist and doc |
| 18 | 98ad5e4c | ca4976dc | differs | the calibration is already on main (b0317963); carries the tasklist and doc lines |
| 19-20 | bd9ebe0b, 7e8b4cdf | 4cfb9584, 0f256e43 | equal | |
| 21 | 9fa71052 | 12a043b1 | differs | context only: the moved blobs are identical; one context line is main's `mod builtins_chirho;` |
| 22-28 | ea47ff84, cab1b611, ecc4e81f, 7b75b175, d3a3d72f, 6ee5eec2, d6120deb | 74b1dfbc, 68d5264c, 805c249b, cd3010e0, c3d4ffe6, bc930d30, c1e04ecb | equal | |

## Per-path resolutions

Seven paths conflict in `git merge-tree --write-tree 0ec9f8f5 c1e04ecb`, the same seven reported for 0f256e43.

1. core `dict_chirho/instance_chirho.rs`, `instance_chirho/generic_heads_chirho.rs`, `instance_chirho/generic_heads_tests_chirho.rs` and the driver's `dictionary_evidence_chirho/generic_heads_chirho.rs`: main's blob. Held touches these only through the four patch-equal commits, and held's c1e04ecb blob of each equals main's blob at cf465667 or 5c41fd88, both ancestors of 0ec9f8f5. Main's later edits are therefore its own additions.
2. typing `deriving_chirho/declaration_heads_chirho.rs`: main's blob with be4b6ce7's `visible_instance_binders_chirho` block (and its two helpers) appended, byte-identical to held. The standalone arms keep main's `TyVarChirho::visible_binders_chirho` route.
   - R2 integration chose that route after cfd829e3, which used be4b6ce7's synthetic binders on the standalone route, failed main's zero-binder control (`missing STG binding ==`).
   - Inline data and newtype deriving in `mod.rs` call the helper exactly as held does. The held producer control `inline_kind_arity_chirho` is why: GHC accepts it, and the strict known-type checks reject it without kind-arrow arguments.
   - Against held, the file differs only in those standalone arms (4 lines added, 14 removed).
3. parser `lower_chirho.rs`: equal to held's c1e04ecb blob. Main's two changes since the base are both inside held already: pragma trivia (c32f071a is patch-equal to cfd829e3 on this path) and the standalone head lowering (70d907ac), which lives in a function held moves. 4e79027c moves main's current body, byte-identical apart from `pub(super)`.
4. parser `lower_chirho/instance_signature_tests_chirho.rs`: equal to held's c1e04ecb blob. Main's copy is held's minus `a_standalone_deriving_keeps_its_class_beside_a_pragma_chirho`, which R2 removed because it needs the declaration-head parser stack carried here.
5. Two declared extras outside the seven:
   - parser `type_syntax_chirho/declaration_heads_chirho.rs` is held's blob plus main's 4-line comment, because both sides made the identical code change.
   - held's unsharded copy of the generic-instance-identity tasklist is not re-added; main's shard copy is byte-identical.

## Proofs at the replay commit d6120deb

- Every other path equals merge-tree's automatic union exactly. The replay tree 1422f54a differs from merge-tree's 14d21866 only in the seven resolved paths and the two declared extras.
- Linear: 28 commits, no merges, base exact.
- rustfmt 1.8.0-stable (254b59607d 2026-01-19), edition 2024: 27 of 28 changed Rust files are stdin no-ops.
  - `class_chirho.rs` was not a no-op at held c1e04ecb itself: 74b1dfbc widened `match_ty_chirho` past the line width.
  - da817309 applies rustfmt's own reflow of that one signature and nothing else.
- `git diff --check` is clean.
- gitleaks 8.30.1: no leaks in 28 commits. The final range is rescanned at the checkpoint.
- The 16 new files carry the John 3:16 header; the largest is 791 lines.

## Limits, flagged

- `typing/src/infer_chirho/` held 15 entries in the union (base 13, held 14, main 14), over the under-15 rule. Under gpt's placement GO (#28259), 05d60cf0 moves the three `signature_*` modules byte-for-byte into `infer_chirho/signatures_chirho/` and keeps their module names through path attributes. The parent now holds 13 entries and the child 3; the blobs are unchanged and all four touched files are rustfmt no-ops.
- Older debt, unchanged in kind:
  - `driver/tests/` has 25 entries.
  - The top-level `tasklists-chirho/` has 153.
  - `lower_chirho.rs` shrinks from 16,931 to 16,331 lines, `infer_chirho.rs` from 14,534 to 14,197, and `cst_parser_chirho.rs` grows from 6,403 to 6,432.
- The held tasklists move into the `26-10-17-chirho` shard (now 13 entries). Nothing referenced their old paths.

## Advances onto later main

Each advance is `git rebase --onto <new main> <old main>` of the same 32 commits. Every commit keeps its original "cherry picked from <held>" line. Earlier branches stay as reviewed checkpoints.

1. RC5 (2026-10-25, #28284): onto 9944ed4c, giving de92912a (tree 3bc326f0) on `patterns-fundep-rc5-replay-chirho`.
   - RC5's 7 paths and the replay's 40 are disjoint.
   - All 32 commits keep their patch-ids and messages.
   - The tree equals `git merge-tree --write-tree 9944ed4c ed1b667e`.
   - rustfmt is a stdin no-op on 31/31 changed Rust files. `git diff --check` is clean, and gitleaks finds no leaks in 32 commits.
2. Rows 532, 533 and 534 (2026-10-26, #28409): onto 3c29010c, giving 37f431c0 (tree 4aa36878) on `patterns-fundep-534-replay-chirho`.
   - The landed main's crates tree, 0cfd246e, is row 534's gated tree.
   - Main's 29 commits since 9944ed4c are the import scope for join and &, derived Show and the Data.Map repair, with their reports and DB closes.
   - All 32 commits keep their `patch-id --stable`, their full messages and their authors.
   - The tree equals `git merge-tree --write-tree 3c29010c de92912a`. A per-commit dry replay with merge-tree and commit-tree meets it with no conflict.
   - Blob-level union:
     - main's 29 paths carry main's blobs;
     - the replay's 47 paths (a rename counts on both sides) carry de92912a's;
     - the one shared path, typing `deriving_chirho/mod.rs`, is `git merge-file`'s clean three-way merge.
   - 2122dbda then applies the approved binder adaptation (#28369). Derived Show metadata takes `visible_instance_binders_chirho`, the inline-deriving instance's own binders. Row 533's metadata had read only the written binders, so `data E :: Type -> Type deriving Show` was described as monomorphic while its instance head is `Show (E x)`.
     - A structural test pins the metadata's parameters to the instance head's.
     - The workflow doc gains a Parameters bullet.
     - Only empty declarations reach this case: GHC rejects ordinary constructors after a result kind.
   - Hygiene over 3c29010c..2122dbda:
     - rustfmt 1.8.0 is a stdin no-op on 33/33 changed Rust files;
     - `git diff --check` is clean;
     - gitleaks 8.30.1 scans 33 commits and finds no leaks.
   - Ordering (#28411): row 535 (P1) lands first. This advance stays source-only: no build, freeze or builder request.
     - After row 535 lands, the stack advances onto that exact main and these proofs repeat.
     - Then comes the freeze and the unit 517 re-derivation.
3. Row 535 (2026-10-26, #28446): onto adedc966, giving 639359d9 (tree 18d3472f) on `patterns-fundep-535-replay-chirho`, then the binder adaptation 17510b31 and the record commit 31832765 (tree 2b131234).
   - Main's crates tree is now 0a9f3727, row 535's gated source tree. Row 535's 18 commits change 17 paths: the Cranelift and LLVM backends, the RTS failure route, a driver test helper, a workflow doc, a tasklist and the DB. None of them is one of the replay's paths, and no directory gains files from both sides.
   - All 34 commits over 3c29010c, the binder and record commits included, keep their `patch-id --stable`, full messages and authors.
   - The 32nd commit's tree equals `git merge-tree --write-tree adedc966 de92912a`, and 31832765's tree equals `git merge-tree --write-tree adedc966 863669a2`. Per-commit dry replays of both ranges with merge-tree and commit-tree meet the same trees with no conflict.
   - Blob-level union over 9944ed4c: main's 45 paths carry main's blobs, the replay's 47 carry de92912a's, and the one shared path, typing `deriving_chirho/mod.rs`, is still `git merge-file`'s clean three-way merge.
   - The binder commit's diff is byte-equal to the prepared patch (sha256 155f74a1), its three blobs equal the drafts, and their pre-images are the same blobs on adedc966. The 32nd commit plus the patch gives the binder commit's tree.
   - Hygiene over adedc966..31832765:
     - rustfmt 1.8.0 is a stdin no-op on 33/33 changed Rust files;
     - `git diff --check` is clean;
     - gitleaks 8.30.1 scans 34 commits and finds no leaks.
   - Row 535 moves main's native SectionsChirho run from a printed pointer to R2's named failure, with the execution corpus at 30/35. Question 6 compares against that main.

## Measured questions for the post-RC5 gate

1. The RC5 union: `tests/dictionary_evidence_chirho.rs` needs an additive module union and `dictionary-evidence-chirho.md` a docs union (#28256). Prove both.
   - Answered at de92912a: RC5's paths and the replay's are disjoint, so neither file needed a union.
2. The two output holds, under the #28247 rule:
   - Each exact infix Show/operator and NonEmpty control runs on a frozen base and on the candidate, with GHC 9.14.1 as the oracle.
   - A regression is RED.
   - A newly enabled wrong answer is RED and names unit 517 or the narrow Show prerequisite.
   - Identical wrong or loud output on both sides is a pre-existing boundary outside this claim.
3. The deriving seam: main's five standalone controls must still print GHC's answers: ZeroBinderHeader, BinderHeader, KindedWrittenHead, PolymorphicSite and PhantomIndexedField. Held's producer controls, including `inline_kind_arity_chirho`, must pass beside them.
4. The parser control `a_standalone_deriving_keeps_its_class_beside_a_pragma_chirho` must pass.
5. The corpus pair moves exactly as held measured, with no other movement:
   - should_compile: tc231 changes from accept to reject.
   - should_fail: exactly T7368a, T9318, tcfail004, tcfail005, tcfail012, tcfail182, FDsFromGivens2, FunDepOrigin1b, T18406 and T20064 change from accept to reject, each with a fresh GHC rejection.
6. The execution pair keeps main's verdict set, using the narrow native Sections rule (#28082).
7. The binder adaptation:
   - the structural test `a_kind_signature_tail_gives_the_metadata_the_instance_heads_parameters_chirho` passes;
   - reverting only its `show_chirho.rs` hunk fails that test by its assertion, with the expected text copied from a real log;
   - two controls must print GHC 9.14.1's `0` with an empty stderr on both base and candidate: W1, an empty `data E :: Type -> Type deriving Show`, and W2, `data H a :: Type -> Type deriving Show`.
8. Row 534's Data.Map controls stay non-interference holds on the repaired main (#28409). The candidate's streams must equal main's for D01, D02, M01–M08 and `eval_import_data_map_chirho`.
   - T07 and Q01 are NonEmpty rows, not Data.Map controls. Their main-to-replay movement is measured, never assumed.

## Checklist

- [x] Map the held stack against current main: patch-equal commits, merges, conflicts and the R2 integration's earlier choices.
- [x] Replay 28 commits on exact 0ec9f8f5, resolving the seven conflicts as above.
- [x] Prove the union, patch-ids, rustfmt, diff-check, gitleaks, headers and limits.
- [x] Move the held tasklists into their date shard and record this replay.
- [x] Post the source checkpoint for gpt's review.
- [x] After RC5 lands, advance onto that main and prove the RC5 union (de92912a).
- [x] After rows 532–534 land, advance onto 3c29010c with the approved binder adaptation and prove the raw union (2122dbda).
- [x] After row 535 lands, advance onto that exact main and repeat the union proof (#28411, #28446).
- [ ] Build the gate tooling for the measured questions. Request the builder by name before any freeze or gate (#28446).
- [ ] Freeze the re-advanced replay as the unit 517 baseline.
- [ ] Gate, then a docs-only gate record; main and the DB stay gpt_chirho's.
