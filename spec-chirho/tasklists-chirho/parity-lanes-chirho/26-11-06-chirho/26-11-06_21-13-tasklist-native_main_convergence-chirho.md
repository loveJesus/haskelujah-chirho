<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native main convergence candidate Chirho

Root delegated this isolated source composition on November 6, 2026. Root owns canonical main, SQLite, acceptance and landing; the sole kind scheduler owns compiler/native measurement. The preparation below is historical. All named candidate gates have now completed, and root lands the exact measured compiler source; the final bounded evidence and exclusions are recorded below.

- [x] Create `native-main-convergence-chirho` from clean main `c687abefa6b5da6ac98557a813995756b08e5aab`, crates `a98c1766bd6ccf9a45ebab0eb1effd895249d7ac`.
- [x] Restore only the 18 reviewed native source/test/harness blobs and two linked workflow pages from `f45cf7551525215efe132f5575d759ad4ff354a7`.
- [x] Verify every restored blob equals f45, all 13 paths belonging to 708 equal the parent, and composed crates equal `04116a69eca349f4e7493fa60420d5aa53a4fb52`.
- [x] Complete standalone rustfmt checks for all 18 Rust files, diff checks and the staged secret scan (104,757 bytes; no leaks). Rescan the exact outgoing commit range with redacted gitleaks output before pushing.
- [x] Root reviews the pushed source checkpoint and the scheduler pins a fresh candidate lease before execution.

The source union is `3237c7683005176f26231d88b96679e8da8c8c05` (measurement foundation), `cc2f44d0878a3a4586ba23229321df9a6b5abee3` (locked/offline/two-job nested freshness build), `39a5a8067d57a020a95f9a037a2fdaa018a0f512` (activation root frames), `70d0e81191674c2b5e49aa69c2b669184a1b91ee` (thunk publication/capture retirement), and f45 (boxed/PAP unary adapters). Full-branch preview against the parent is conflict-free, tree `7bd7df0649e8f63daafdc9aa3b19bb89793e364e`; its four historical tasklists are excluded here, as is docs-only `4650d0d8`. The root-owned progress DB stays unchanged. Unit 706 Program-role semantics, broad kind/Reader, package, C10, CAF and extraction work stay separate.

## Owned source paths Chirho

Every path below is restored verbatim from f45. The new checklist is the only additional tracked file.

```text
crates/haskelujah-backend-cranelift-chirho/src/codegen_chirho.rs
crates/haskelujah-backend-cranelift-chirho/src/lower_chirho.rs
crates/haskelujah-backend-cranelift-chirho/src/lower_chirho/root_frames_chirho.rs
crates/haskelujah-backend-llvm-chirho/src/codegen_chirho.rs
crates/haskelujah-backend-llvm-chirho/src/codegen_chirho/partial_applications_chirho.rs
crates/haskelujah-backend-llvm-chirho/src/codegen_chirho/root_frames_chirho.rs
crates/haskelujah-driver-chirho/tests/native_boxed_applications_chirho.rs
crates/haskelujah-driver-chirho/tests/native_root_lifetimes_chirho.rs
crates/haskelujah-driver-chirho/tests/native_runtime_measurement_chirho.rs
crates/haskelujah-rts-chirho/src/ffi_chirho.rs
crates/haskelujah-rts-chirho/src/ffi_chirho/native_measurement_chirho.rs
crates/haskelujah-rts-chirho/src/ffi_chirho/native_root_frames_chirho.rs
crates/haskelujah-rts-chirho/src/ffi_chirho/native_root_frames_chirho/tests_chirho.rs
crates/haskelujah-rts-chirho/src/ffi_chirho/native_thunks_chirho.rs
crates/haskelujah-rts-chirho/tests/native_root_frames_chirho.rs
crates/haskelujah-rts-chirho/tests/native_thunk_retirement_chirho.rs
crates/haskelujah-test-harness-chirho/src/native_chirho.rs
crates/haskelujah-test-harness-chirho/src/native_chirho/runtime_measurement_chirho.rs
spec-chirho/workflows-chirho/testing-chirho/native-runtime-measurement-chirho.md
spec-chirho/workflows-chirho/testing-chirho/native-root-lifetimes-chirho.md
```

## Prepared gate plan Chirho

This plan was prepared before execution. The completed candidate runs reuse reviewed `run_stage_chirho.py` SHA `b5999cba221d3b4b4d3a20e5d9bf5cf7371f9622771bba55271e75406a013f9d` and process-group helper SHA `b3f7bb1a9e6eb490b7c300793e094b07dfc584fac4a845c5e997a187813dba3f`, with fresh source/tool contracts and named grants. Every stage retains its full cap plus 30 seconds cleanup. Historical f45 verdicts were not reused as candidate outcomes. The first source-count preflight and the later unstarted failure control remain preserved separately.

All Cargo argv use `--locked --offline -j2`, unsuffixed package selectors, the candidate worktree as cwd, its own absolute `CARGO_TARGET_DIR`, `RUST_MIN_STACK=16777216` and the existing owned-group runner. This is not a separate POSIX main-thread stack-limit claim. Native tests use `--nocapture --test-threads=1`; test listing is distinct from execution.

| Pending gate | Existing target/selection | Full cap (seconds) |
| --- | --- | ---: |
| Workspace check | `cargo check --workspace --all-targets` | 600 |
| Actual own RTS build | `cargo build -p haskelujah-rts --message-format=json` | 180 |
| RTS control list | `native_thunk_retirement_chirho`, `native_thunks_chirho`, `native_root_frames_chirho` | 180 |
| RTS controls | The three exact names below, as in f04 | 120 |
| Boxed application list | Driver `native_boxed_applications_chirho` | 600 |
| Boxed application execution | Its one existing grouped/staged test; eight emitted outcomes | 300 |
| Survival list | Driver `native_root_lifetimes_chirho` | 300 |
| Survival execution | Its one existing returned-value test; six emitted outcomes | 720 |
| Growth list | Driver `native_runtime_measurement_chirho` | 180 |
| Growth execution | Its one existing self-tail test; four emitted outcomes | 480 |
| Laziness list | Driver `native_laziness_chirho` | 120 |
| Laziness execution | All six existing tests | 1080 |
| Failure list | RTS `native_failure_chirho` | 60 |
| Failure execution | Each of the two exact names below, separately | 30 each |
| Effect list | Driver `io_actions_chirho` | 120 |
| Effect execution | Each of the two exact names below, separately | 300 each |
| Scoped lint | Existing six-package Clippy argv, all targets, JSON diagnostics | 600 |

The RTS control names are `completed_thunks_retire_captures_without_losing_live_results_or_active_environments_chirho`, `native_entry_reaches_whnf_memoizes_full_values_and_survives_gc_chirho`, and `thunk_update_owns_a_framed_callback_result_before_collection_chirho`. Failure names are `program_failure_flushes_stdout_and_reports_the_exact_payload_chirho` and `a_thunk_that_demands_itself_reports_loop_chirho`. Effect names are `demanding_an_action_does_not_execute_it_chirho` and `a_shared_input_action_consumes_fresh_input_chirho`.

Scoped Clippy packages are `haskelujah-core`, `haskelujah-rts`, `haskelujah-backend-llvm`, `haskelujah-backend-cranelift`, `haskelujah-driver`, and `haskelujah-test-harness`. Apply the existing recursive changed-span attribution to every changed Rust file; retain diagnostics outside changed spans separately. The historical 0 changed-span hits/257 outside belongs to f45 only.

- [x] Pin the exact candidate commit/crates, complete source manifest, runner/helper/tools and fresh lease deadline.
- [x] Observe the candidate's actual RTS archive path/hash/size/mtime/mode and Cargo artifact identity after the own build. Build harness/test binaries from this candidate so their embedded manifest root is correct; retain and hash the private linked RTS snapshots in each emitted artifact directory.
- [x] Complete the workspace/RTS gates and ABC's 18 emitted outcomes with unchanged expectations and growth envelope, actual allocations/collections and both real backends.
- [x] Complete the existing six laziness tests, two failure neighbors and two effect neighbors, preserving stdout/status/error payload/effect order and cleanup evidence.
- [x] Verify source/tool stability, all receipt/stream/artifact hashes and empty owned groups; attribute scoped lint before root accepts any landing claim.

The old f45 RTS hash `d0243b22e31e0fc823d2561fa6d193c201bbbde1496990ddfcce66d7e4cdfb96` is historical, not a candidate archive identity or a reuse claim. Inode is an observation; SHA/size/mtime/mode plus retained-copy/test-binary identity own the accepted archive guard scope. Preserve the original phase-D NOT-PASS receipt, inode-only disposition, separate D continuation and E results verbatim. A new candidate run gets its own outcome. Its sources use explicit Int or polymorphic/string values, with no inspected Int/Double coercion dependency, but 708-integrated frontend/native behavior still requires the gates above. No full corpus, new harness, GHC, CAF implementation, Program-role acceptance or overall dependable-runtime claim is part of this candidate.

## Measured landing Chirho

Root cherry-picks source `325d6349228a19c85d51414375ed925d942344d5` as `e7ea264d712448923e5e39c4e5f36eaadce3e1da` on main. The resulting compiler crates are exactly the measured `04116a69eca349f4e7493fa60420d5aa53a4fb52`; all 13 nominality paths remain intact. No new compiler execution on canonical main is claimed.

Root verification `32bf968495047bd67fa7ade275e9af6c3e509b16409ee76e27a26953659685ef` joins all 23 actual receipts to their pinned summaries, rehashes 46 raw streams, 198 retained measurement files and all 819 source files, inspects archive/test-executable guards and checks the recorded groups are empty. Its local artifact is `tmp-chirho/coordination-records-chirho/runtime-sigma-chirho/native-final-root-verification-r2-chirho.json`.

| Completed scope | Summary SHA prefix | Actual result |
| --- | --- | --- |
| Workspace check | `5d2c324b` | All targets compile; no text compiler warning/error; plain output, not a JSON inventory |
| Own RTS | `b5e7a0cc` | Six Rust protocol controls; fresh compiled archive `02dd4282`, provenance `28d085b5` |
| Boxed applications | `e76d4012` | Eight LLVM/Cranelift outcomes; stdout 24, exit zero, 24 collections each |
| Returned-value survival | `c6ed1851` | Six constructor/closure/nested-PAP outcomes on both backends |
| Growth | `de52093e` | Four outcomes at 4,096/16,384 iterations; peak roots unchanged at 132 LLVM/100 Cranelift; post-GC live high-water unchanged at 16 objects/400 bytes |
| Laziness | `14ca8d68` | All six existing tests pass |
| Failure | `f47ddb5f`, `1858d879` | User-error and self-loop pass across two separate grants; original self-loop UNSTARTED preserved, no list/user-error replay |
| Effects | `914dd3dc` | Exact WHNF and fresh-input controls pass with unconditional STG/LLVM/Cranelift assertions |
| Required changed-span Clippy | `4375b71d` | Zero hits versus parent `c687abef`; 365 raw warnings deduplicate to 257 OUTSIDE diagnostics, not parent-attributed |

The candidate RTS is `02dd428258e54dedbc7c84dee466f7d3b846abb17afe4eacea6e171429b4107a`, 20,028,120 bytes. Archive SHA/size/mtime/mode remain the required identity; inode changes are observations with no inferred cause. The 198 retained files belong only to the ABC measurement helper. Ordinary neighbors delete their private programs; failure controls are Rust children. The original preflight refusal, historical f45 phase-D NOT-PASS, partial failure grant and raw-versus-unique lint-fold correction are not relabeled. This completes rows 704/707 at their named scope, not overall compiler, full runtime, CAF, Program-role, corpus or release acceptance.
