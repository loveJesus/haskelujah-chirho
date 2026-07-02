<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Multi-Module Unforced Result Chirho

- [x] Reproduce `eval_multi_module_function_call_chirho`, `eval_multi_module_three_modules_chirho`, and `eval_module_re_export_chirho` under the resource guard.
- [x] Diagnose the returned heap pointer as a PAP over an imported constrained helper, not a missing final force.
- [x] Seed imported constrained variable IDs into the dictionary-parameter call-site rewrite path while excluding class methods.
- [x] Harden `eval_modules_chirho` CoreId offsetting to account for binder IDs inside expression bodies, not only names map keys.
- [x] Re-run the three targeted multi-module representatives.
- [x] Update `spec-chirho/prd_chirho.json` with the resolved bucket and actual root cause.
