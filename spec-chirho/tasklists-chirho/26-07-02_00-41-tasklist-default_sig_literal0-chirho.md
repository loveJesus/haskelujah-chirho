<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# DefaultSignatures Literal 0 Chirho

- [x] Confirm Haskelujah worktree is clean before edits.
- [x] Avoid Cargo while unrelated machine-wide Rust work is active.
- [x] Trace why `default_sig_basic_eval_chirho` applies literal `0#`.
- [x] Patch the smallest PRD-aligned root cause.
- [x] Run targeted validation only, with `CARGO_BUILD_JOBS=2` and `--test-threads=1`.
  - [x] `CARGO_BUILD_JOBS=1 timeout 180 cargo test -p haskelujah-parser lower_default_signature_chirho -- --test-threads=1`
  - [x] `CARGO_BUILD_JOBS=1 timeout 240 cargo test -p haskelujah-driver --lib default_sig_basic_eval_chirho -- --test-threads=1`
  - [x] `CARGO_BUILD_JOBS=1 timeout 180 cargo test -p haskelujah-driver --lib default_sig_with_override_eval_chirho -- --test-threads=1`
  - [x] `CARGO_BUILD_JOBS=1 timeout 180 cargo test -p haskelujah-driver --lib eval_class_default_method_chirho -- --test-threads=1`
  - [x] `CARGO_BUILD_JOBS=1 timeout 180 cargo run -p haskelujah -- build .haskelujah-packages-chirho/transformers-0.6.3.0/`
- [x] Update `spec-chirho/prd_chirho.json` and progress DB.
- [x] Commit and push the bounded fix.
