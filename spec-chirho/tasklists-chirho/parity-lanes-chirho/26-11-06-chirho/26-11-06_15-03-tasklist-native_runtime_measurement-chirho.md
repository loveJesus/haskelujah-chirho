<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native runtime measurement Chirho

Owner: root's dependability_audit_chirho subagent; branch native-runtime-measurement-chirho, isolated from exact main 1c42c547 / crates a9a38e3d. Root owns progress database row 702. The sole kind scheduler owns every Cargo/GHC/product window; this checkpoint is SOURCE ONLY until a named grant. Numeric, package, kind, Reader, root lifetime, CAF behavior, representation and collector policy are excluded.

- [x] Read current AGENTS/worktrees/ownership and inspect the actual root and CAF implementation.
- [x] Name RTS child, runner child, child-only report environment and stress-test homes; root clears linkage and measurement footprint.
- [x] Add O(1) counter updates at existing allocation/root/collection boundaries; report cold after the existing native entry returns without introducing collection or changing stdout/exit.
- [x] Prepare a typed NOINLINE allocating self-tail loop that checks a live heap sentinel every iteration; collect both LLVM/Cranelift outcomes at N=4096 and 4N=16384 before judging them.
- [x] Require actual allocated objects/bytes and at least three collections, then compare fixed growth envelopes for root high-water and post-GC live memory. The expected RED is a prediction, not a measured failure.
- [ ] Source review, formatting/static checks and outgoing secret scan; commit/push only owned paths.
- [ ] Request the smallest literal gate through root, with source/tool/RTS artifact identity and no competing compiler run.
- [ ] Preserve actual warnings/failures, raw reports/artifacts and exact source identity; root folds the unit without calling the runtime repaired.

Planned gate, unrun: selected driver integration-target checking (including its actual harness/backend/RTS dependencies); one RTS native_measurement counter unit; one harness partial-report control; one driver native_runtime_measurement integration test (all four fresh native runs), with the harness's existing own-RTS snapshot; changed-span Clippy for the two library targets and one driver integration target. Estimate: 15 minutes with warm dependencies, 25 minutes cold, bounded child runs 30 seconds each. No GHC/corpus/release/CAF or semantic repair is part of this gate. The literal scheduler packet below remains UNRUN and requires the scheduler's actual grant.

From this worktree, with a scheduler-owned isolated CARGO_TARGET_DIR and per-stage bounded process groups/raw streams:

```text
cargo check --locked -p haskelujah-driver --test native_runtime_measurement_chirho
cargo test --locked -p haskelujah-rts --lib ffi_chirho::native_measurement_chirho::tests_chirho::observations_track_real_allocation_roots_and_retirement_chirho -- --exact --nocapture
cargo test --locked -p haskelujah-test-harness --lib native_chirho::runtime_measurement_chirho::tests_chirho::absent_partial_or_duplicate_reports_cannot_supply_measurements_chirho -- --exact --nocapture
cargo test --locked -p haskelujah-driver --test native_runtime_measurement_chirho native_self_tail_loop_retires_dead_objects_with_a_live_sentinel_chirho -- --exact --nocapture --test-threads=1
cargo clippy --locked -p haskelujah-rts -p haskelujah-test-harness --lib --message-format=json
cargo clippy --locked -p haskelujah-driver --test native_runtime_measurement_chirho --message-format=json
```

The integration target is expected to expose a real RED envelope. A frontend/backend/link/runtime/report/exercise failure is retained by its actual stage and is not relabelled as evidence of root retention. All four outcomes are collected before the test asserts. The gate owner must capture a fresh list for exact selected-name completeness, source/tool hashes before/after, and hash each retained artifact directory, including the exact linked RTS archive; no installed/release identity is claimed.

Each measured child gets HASKELUJAH_NATIVE_MEASUREMENT_REPORT_CHIRHO solely through Command::env. No process-global environment mutation occurs. The runner retains a bounded number of private directories containing each source, IR/object, executable, exact linked RTS, raw link/run streams and report, including failed assertions. The gate owner hashes and folds these before retiring them. A missing/partial/oversized report fails measurement; the product stdout/exit remains its own outcome. Instrumentation does not reset, force collection or change collection thresholds.
