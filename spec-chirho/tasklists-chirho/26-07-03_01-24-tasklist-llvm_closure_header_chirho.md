<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# LLVM Closure Header Chirho

- [x] Confirm `run_executable_partial_application_of_lambda_function_outputs_value_chirho` fails independently of the qsort rooting patch.
- [x] Diagnose closure word 0 as an untagged function pointer that `enter_thunk` can misclassify as a thunk header.
- [x] Tag native closure headers as function objects and mask the tag before indirect calls.
- [x] Run targeted backend tests and rebuilt 70-case probe.
- [x] Update PRD/progress and commit if the fix is clean.
