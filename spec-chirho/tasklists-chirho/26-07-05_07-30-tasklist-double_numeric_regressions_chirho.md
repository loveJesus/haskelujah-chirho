<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Double Numeric Regressions Chirho

- [x] Reproduce the four Double regressions from the 26-watch surface.
- [x] Trace why Double numeric methods are rewritten to Int primops.
- [x] Patch the narrow dict/evidence key path without touching unrelated dirty files.
- [x] Run targeted Double tests and probe.
- [x] Commit, push, and hand off to Claude for origin-68 + 27 + probe acceptance.

Result: occurrence evidence now preserves fractional generalized defaults as `Double` and suppresses class-only fallback for genuinely polymorphic generalized variables, allowing dictionary-passed bindings like `addDoubles` to remain polymorphic.

Gates:
- `cargo test -p haskelujah-typing --lib -j2 occurrence_record -- --test-threads=1`
- `cargo test -p haskelujah-driver --lib -j2 -- --test-threads=1 eval_num_double_negate_chirho eval_num_double_abs_chirho eval_num_double_signum_chirho eval_double_variable_add_chirho`
- fresh `target/debug/haskelujah` warm-up + `probe_chirho.sh` = `PASS=70 FAIL=0`
