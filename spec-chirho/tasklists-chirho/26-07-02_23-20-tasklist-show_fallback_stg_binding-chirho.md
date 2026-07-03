<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Show Fallback STG Binding Chirho

- [x] Reproduce the design-gated conditional instance case and defer it rather than defaulting user-class literals to `Int`.
- [x] Reproduce current post-b520 IORef/STM representative failures as loud missing STG binding `show` errors.
- [x] Add a body-backed generated Prelude `show` fallback using `showInt#`, matching the existing `print` universal-show fallback.
- [x] Verify `eval_ioref_write_read_chirho` and `eval_stm_new_read_tvar_chirho` pass.
- [x] Verify the fresh CLI 70-case probe remains `PASS=70 FAIL=0`.
- [x] Record adjacent IORef arithmetic/PAP residuals as separate from the `show` fallback fix.
