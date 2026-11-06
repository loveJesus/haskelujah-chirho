<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native boxed application arity Chirho

Owner: GPT dependability child, row 707; root owns the DB. This separate source unit follows row 704 retirement checkpoint `70d0e81191674c2b5e49aa69c2b669184a1b91ee` on `native-thunk-retirement-chirho`, based on frozen `39a5`. Source work only; compiler/native/debugger/GHC execution awaits the sole kind scheduler's actual grant.

- [x] Correlate original B crash image UUID with the retained executable: the failing LLVM PAP entry expects two arguments but receives one; a later application jumps to scalar address 20. Preserve the original crash, not an inferred GC diagnosis.
- [x] Extract boxed-call/PAP responsibilities into `codegen_chirho/partial_applications_chirho.rs`; reduce the large parent and retain ordinary worker implementations.
- [x] Give boxed PAPs unary entries; each under-saturated stage allocates the next PAP, preserving original worker environment and already supplied argument values.
- [x] Give escaped captured multiargument closures unary entry adapters, while preserving direct workers and their capture-field layout.
- [x] Make grouped boxed applications consume one argument per entry, rooting each intermediate actual result before further application/allocation; every new entry owns a checked 704 frame.
- [x] Prepare separate grouped/staged captured-closure, grouped PAP and known captured-worker PAP controls with real collection requirements in both backends. Original B and row 702 workload/test files remain byte-identical.
- [ ] Standalone formatting, owned-path/diff review, outgoing-range gitleaks, separate source commit and push.
- [ ] Scheduler-granted fresh exact-name lists, warning-free source check, compiled test/tool/own-RTS hashes and bounded owned-group receipts.
- [ ] Run and retain the row 704 same-test old-update negative control; candidate RTS protocol/neighbor tests pass before any retirement acceptance claim.
- [ ] Original six survival cases and eight new boxed-application cases pass with independently specified stdout and actual collection exercise. Preserve each source, executable, IR/object, linked RTS, link/run streams/status and runtime report.
- [ ] Unchanged row 702 growth workload passes both backends/scales with the original allocation/collection requirements and root/post-GC envelopes; preserve original RED packets.
- [ ] Existing laziness, error/blackhole and effect neighbors pass at their named scope; no broad release/corpus claim follows from this gate.

Prepared new-test literal commands (unrun), only through the reviewed bounded owned-group runner with own target directory, `RUST_MIN_STACK=16777216`, pinned candidate RTS and a scheduler grant:

```sh
cargo test --locked --offline -j2 -p haskelujah-driver-chirho --test native_boxed_applications_chirho -- --list
cargo test --locked --offline -j2 -p haskelujah-driver-chirho --test native_boxed_applications_chirho grouped_and_staged_boxed_applications_preserve_captured_values_across_gc_chirho -- --exact --nocapture --test-threads=1
```

Frozen assertion contract: all eight new native cases return status 0 and `24\n`; each actually allocates at least 4096 objects and performs at least three collections. The four shapes are grouped captured binary closure, staged captured binary closure, grouped two-argument PAP, and a PAP over a known captured local worker. Original row 704 supplies the unchanged staged nested-PAP case plus constructor and single-argument closure neighbors. A missing report, inadequate exercise, source/protocol drift, timeout or compiler warning is a failure/inconclusive result, never a pass.

Remaining gap: escaped bare function values retain the old raw-pointer/all-arguments ABI. That representation carries no arity authority, so arbitrary under/over-application of an unknown bare value remains unsupported/unsafe and is not repaired or hidden by unary boxed entries. No new tags, layout, runtime registry, CAF, C main, numeric or package changes belong to this unit.

Placement debt: the existing LLVM parent already exceeds the line limit; this unit removes 219 lines from it and isolates the new responsibility in a child below 300 lines. The existing driver test directory was already above the entry limit (29); adding the explicit behavioral test makes 30. This is recorded existing structural debt, not a size-gate pass. No unrelated file moves are mixed into this correctness unit.
