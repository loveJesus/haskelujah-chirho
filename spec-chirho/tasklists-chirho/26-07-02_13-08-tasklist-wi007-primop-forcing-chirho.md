<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# WI-007 / WI-015 Composition Dispatch Slice Chirho

- [x] Reproduce one strict-primop-looking failure with a targeted driver test: `eval_dot_compose_chirho`.
- [x] Inspect the STG runtime primop argument path and test a shallow force experiment.
- [x] Revert the runtime force experiment after it did not change the failure and the Core showed a dict-specialization bug instead.
- [x] Implement the narrowest sound dict rewrite fix: propagate proven argument type keys into lambda bodies and dict-parameterized user-function arguments.
- [x] Run targeted gates only with `CARGO_BUILD_JOBS=1` and `--test-threads=1`.
- [x] Update `spec-chirho/prd_chirho.json` and `progress-chirho.sqlite`.
- [x] Commit explicit files, push to `gh_chirho/main_chirho`, and post `SLOT-FREE`.
