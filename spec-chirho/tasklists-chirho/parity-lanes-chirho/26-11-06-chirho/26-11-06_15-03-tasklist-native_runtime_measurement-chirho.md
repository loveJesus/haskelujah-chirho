<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native runtime measurement Chirho

Owner: root's dependability_audit_chirho subagent; branch native-runtime-measurement-chirho, isolated from exact main 1c42c547 / crates a9a38e3d. Root owns progress database row 702. The sole kind scheduler owns every Cargo/GHC/product window; this checkpoint is SOURCE ONLY until a named grant. Numeric, package, kind, Reader, root lifetime, CAF behavior, representation and collector policy are excluded.

- [x] Read current AGENTS/worktrees/ownership and inspect the actual root and CAF implementation.
- [x] Name RTS child, runner child, child-only report environment and stress-test homes; root clears linkage and measurement footprint.
- [x] Add O(1) counter updates at existing allocation/root/collection boundaries; report cold after the existing native entry returns without introducing collection or changing stdout/exit.
- [x] Prepare a typed NOINLINE allocating self-tail loop that checks a live heap sentinel every iteration; collect both LLVM/Cranelift outcomes at N=4096 and 4N=16384 before judging them.
- [x] Require actual allocated objects/bytes and at least three collections, then compare fixed growth envelopes for root high-water and post-GC live memory. The expected RED is a prediction, not a measured failure.
- [x] Source review, standalone rustfmt, diff whitespace and static twelve-field producer/parser agreement; outgoing gitleaks scan clean. Source 3237c7683005176f26231d88b96679e8da8c8c05 (crates ced32b40add144d5b9c3344836ea85ac908a7521) is pushed to gh_chirho/native-runtime-measurement-chirho. Exactly seven owned paths are committed. No compiler/GHC/product has run, and no global warning-free/type-correct/runtime claim follows.
- [x] Request the smallest literal gate through root, with source/tool/RTS artifact identity and no competing compiler run. Independent root readiness review clears the footprint with the bounded correction below; the sole scheduler's active A9+kind29 window must return before any grant here.
- [ ] Preserve actual warnings/failures, raw reports/artifacts and exact source identity; root folds the unit without calling the runtime repaired.

Planned gate, unrun: selected driver integration-target checking (including its actual harness/backend/RTS dependencies); one RTS native_measurement counter unit; one harness partial-report control; one driver native_runtime_measurement integration test (all four fresh native runs), with the harness's existing own-RTS snapshot; changed-span Clippy for the two library targets and one driver integration target. Estimate: 15 minutes with warm dependencies, 25 minutes cold, bounded child runs 30 seconds each. No GHC/corpus/release/CAF or semantic repair is part of this gate. The literal scheduler packet below remains UNRUN and requires the scheduler's actual grant.

From this worktree, with a scheduler-owned isolated CARGO_TARGET_DIR and per-stage bounded process groups/raw streams:

```text
cargo check --locked --offline -j2 -p haskelujah-driver --test native_runtime_measurement_chirho
cargo test --locked --offline -j2 -p haskelujah-rts --lib ffi_chirho::native_measurement_chirho::tests_chirho::observations_track_real_allocation_roots_and_retirement_chirho -- --exact --nocapture
cargo test --locked --offline -j2 -p haskelujah-test-harness --lib native_chirho::runtime_measurement_chirho::tests_chirho::absent_partial_or_duplicate_reports_cannot_supply_measurements_chirho -- --exact --nocapture
cargo test --locked --offline -j2 -p haskelujah-driver --test native_runtime_measurement_chirho native_self_tail_loop_retires_dead_objects_with_a_live_sentinel_chirho -- --exact --nocapture --test-threads=1
cargo clippy --locked --offline -j2 -p haskelujah-rts -p haskelujah-test-harness --lib --message-format=json
cargo clippy --locked --offline -j2 -p haskelujah-driver --test native_runtime_measurement_chirho --message-format=json
```

The integration target is expected to expose a real RED envelope. A frontend/backend/link/runtime/report/exercise failure is retained by its actual stage and is not relabelled as evidence of root retention. All four outcomes are collected before the test asserts. The gate owner must capture fresh exact-name lists for all three selected tests, source/tool hashes before/after, the own linked RTS hash and bounded process-group receipts for every stage, and hash each retained artifact directory. These memory counters measure runtime roots/allocations, not process RSS. No installed/release identity is claimed.

Readiness correction, source-only: the six commands and the harness's nested RTS build now explicitly carry --locked --offline -j2. The nested build retains --quiet and the existing archive snapshot/identity behavior. Outer Cargo flags do not propagate to a spawned Cargo command. No actual compiler/GHC/RTS launch is authorized by this correction.

Each measured child gets HASKELUJAH_NATIVE_MEASUREMENT_REPORT_CHIRHO solely through Command::env. No process-global environment mutation occurs. The runner retains a bounded number of private directories containing each source, IR/object, executable, exact linked RTS, raw link/run streams and report, including failed assertions. The gate owner hashes and folds these before retiring them. A missing/partial/oversized report fails measurement; the product stdout/exit remains its own outcome. Instrumentation does not reset, force collection or change collection thresholds.
