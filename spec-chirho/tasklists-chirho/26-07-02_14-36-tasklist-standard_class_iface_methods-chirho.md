<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Standard Class Interface Methods

- [x] Confirm no active build/test process and claim the Metropoliluya slot.
- [x] Extend the central class method normalizer for standard method-bearing classes already seeded in typing.
- [x] Add compile-only coverage for `Class(..)` imports of those methods.
- [x] Run targeted validation and lightweight checks only.
- [x] Update PRD/progress, commit explicit files, push, and release the slot.

Validation notes:
- `CARGO_BUILD_JOBS=2 cargo test -p haskelujah-driver --lib frontend_normalized_class_dot_imports_expose_standard_methods_chirho -- --test-threads=1`
- `CARGO_BUILD_JOBS=2 cargo test -p haskelujah-driver --lib frontend_normalized_class_dot_imports_expose_extra_methods_chirho -- --test-threads=1`
- `rustfmt --edition 2024 --check crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs`
- `jq empty spec-chirho/prd_chirho.json`
- `git diff --check`
