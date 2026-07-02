# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

# Maybe Do Payload Dispatch Regression

- [x] Confirm `eval_maybe_do_dispatch_chirho` fails with the Identity row present.
- [x] Stash Identity row and confirm `eval_maybe_do_dispatch_chirho` also fails on clean `eb230556`.
- [x] Add payload type-key propagation for dispatched `>>=` continuations.
- [x] Re-run Maybe-do and Identity focused gates.
- [x] Update PRD/progress and commit safe fixes.
