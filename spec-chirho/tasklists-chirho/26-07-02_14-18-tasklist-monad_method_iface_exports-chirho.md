<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Monad Method Interface Exports

- [x] Confirm no active build/test process and claim the Metropoliluya slot.
- [x] Patch module interface stubs so `MonadIO(..)`, `MonadTrans(..)`, and `MonadZip(..)` expose their methods.
- [x] Add targeted compile tests for `Class(..)` imports.
- [x] Run targeted tests and formatting without full sweeps.
- [ ] Update PRD/progress, commit explicit files, push, and release the slot.

Validation notes:
- `CARGO_BUILD_JOBS=2 cargo test -p haskelujah-driver --lib frontend_class_dot_imports_expose_monad_methods_chirho -- --test-threads=1`
- `CARGO_BUILD_JOBS=2 cargo test -p haskelujah-driver --lib frontend_exceptt_single_monadzip_instance_method_chirho -- --test-threads=1`
- `rustfmt --edition 2024 --check crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs`
- `rustfmt --edition 2024 --check crates/haskelujah-naming-chirho/src/iface_chirho.rs crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs` is blocked by unrelated pre-existing `iface_chirho.rs` formatting drift.
- `git diff --check`
