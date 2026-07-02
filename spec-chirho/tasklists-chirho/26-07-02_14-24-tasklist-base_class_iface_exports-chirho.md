<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Base Class Interface Exports

- [x] Confirm no active build/test process and claim the Metropoliluya slot.
- [x] Patch base class interface stubs so `Class(..)` imports expose already-seeded methods.
- [x] Add targeted compile tests for the affected imports.
- [x] Run targeted tests and lightweight checks only.
- [x] Update PRD/progress, commit explicit files, push, and release the slot.

Validation notes:
- `CARGO_BUILD_JOBS=2 cargo test -p haskelujah-driver --lib frontend_class_dot_imports_expose_base_methods_chirho -- --test-threads=1`
- `CARGO_BUILD_JOBS=2 cargo test -p haskelujah-driver --lib frontend_class_dot_imports_expose_monad_methods_chirho -- --test-threads=1`
- `rustfmt --edition 2024 --check crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs`
- `jq empty spec-chirho/prd_chirho.json`
- `git diff --check`
