<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native thunk capture retirement Chirho

Owner: GPT dependability child, row 704; root alone owns the DB. Source starts at `39a5a8067d57a020a95f9a037a2fdaa018a0f512` in the separate `haskelujah-native-thunk-retirement-chirho` worktree. Original row 702 and row 704 A–C artifacts remain frozen in the measurement worktree. No compiler, native program or debugger has run for this follow-up.

- [x] Trace measured retention independently of the LLVM PAP arity fault: the next counter thunk captures a box containing the previous counter; evaluated thunks retain those obsolete slots, and the unchanged collector scans them.
- [x] Use one registry/kind/size-validated result publication helper in completed entry and explicit update; publish the result before retiring old captures, under the existing lock and without native allocation.
- [x] Preserve environments while callbacks are active, including explicit update followed by actual collection and further environment reads.
- [x] Prepare one isolated-process RTS protocol test for completed entry, inactive update, returned capture alias and active update; use registry membership only before dereferencing, with no intervening allocation when checking reclamation.
- [ ] Standalone rustfmt, whitespace review, owned-path review, outgoing-range gitleaks, source commit and push.
- [ ] Sole scheduler actual grant, pinned source/test/tool/own-RTS identities, bounded owned-group receipts and fresh exact-name listing.
- [ ] Compile and run the same test against the old `39a5` thunk update implementation in a separate disposable negative-control worktree; retain the expected obsolete-capture membership failure as raw evidence. A compile or protocol failure is not a valid negative control.
- [ ] Candidate RTS protocol test passes; existing thunk/frame controls pass; unchanged row 702 growth test passes both backends at both scales with the original exercise and envelope assertions.
- [ ] Preserve original constructor/closure/PAP failure evidence; a separate row 707 boxed-application repair owns that defect. No overall 704 behavioral pass before its relevant surviving-value controls pass.

Prepared literal focused commands, all unrun, only through the reviewed bounded group runner after the scheduler grant; use the granted worktree's own `CARGO_TARGET_DIR` and `RUST_MIN_STACK=16777216`:

```sh
cargo test --locked --offline -j2 -p haskelujah-rts-chirho --test native_thunk_retirement_chirho -- --list
cargo test --locked --offline -j2 -p haskelujah-rts-chirho --test native_thunk_retirement_chirho completed_thunks_retire_captures_without_losing_live_results_or_active_environments_chirho -- --exact --nocapture --test-threads=1
```

The negative-control test source must be byte-identical to the candidate test; only `crates/haskelujah-rts-chirho/src/ffi_chirho/native_thunks_chirho.rs` is restored from exact `39a5`. Pin both file hashes and the compiled test binary before execution. Expected RED is an unmeasured prediction until that old implementation compiles and the test actually rejects retained obsolete captures.

The production footprint is the existing 132-line thunk module plus its local helper. No collector, representation, GC threshold, root protocol, CAF, language, C main or numeric-entry change belongs in this commit. Clearing an environment is proportional to that thunk's allocated capture count, with no heap-wide scan or extra state.
