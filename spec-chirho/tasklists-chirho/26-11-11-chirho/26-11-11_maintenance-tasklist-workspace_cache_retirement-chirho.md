<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Workspace cache retirement

Owner: HASKELUJAH/gpt_chirho, unit 741. L.J. requested unused-workspace cleanup while the next-checker continuation is pending.

- [x] Read the current worktree-retirement workflow, live broker ownership, Git state and process occupancy.
- [x] Inventory all 126 registered worktrees; no absent directory or stale registration exists.
- [x] Measure exact inactive GPT-owned target/debug/incremental leaves: 11 candidates, about 11.99 GiB allocated.
- [x] Check realpaths, open-file occupancy and retained metadata for candidate pins.
- [x] Retire only the admitted incremental leaves and retain the guard result: Git state and outside-cache file counts hold; the aggregate stat digest differs and is not a preservation PASS.
- [x] Measure actual free space and record the result; commit/push maintenance metadata.
- [x] Resume the new checker's dependency-group/generalization work with a GHC-to-Haskelujah implementation map.

Keep the canonical checkout, active next-pipeline worktree, selected-union reference worktree, unmerged source, ignored receipts, named binaries, RTS archives, deps and every branch in place. No whole-target or worktree deletion is admitted by this unit. All evidence lives under canonical tmp-chirho/workspace-maintenance-26-11-11-chirho/.


## Result

Eleven inactive GPT incremental caches were removed, about 11.99 GiB allocated. Measured free space increased from 70,690,054,144 to 79,533,772,800 bytes, an 8.24 GiB gain. All 126 worktrees and branches remain; no stale registration exists. Source Git state and outside-cache file counts hold for all eleven candidates. The current S0 CLI, frozen bucket and unit-737 CLI match their retained SHA256 pins.

The first aggregate stat guard FAILED in all eleven candidates. It included hard-link counts, which cache unlinking can change on shared Rust object files; its snapshot stores only an aggregate digest, so it cannot localize the differences. Both before/after snapshots and the failed receipt are retained. This unit makes no full historical content-preservation PASS claim. No additional deletions followed that finding.

Evidence: canonical tmp-chirho/workspace-maintenance-26-11-11-chirho/{inventory-chirho.json,cache-sizes-chirho.json,cache-reference-check-chirho.json,before-chirho.json,after-chirho.json,retirement-result-chirho.json,closure-chirho.json}. The source-only next-checker continuation resumes after this maintenance checkpoint.
