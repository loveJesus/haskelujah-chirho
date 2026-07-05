<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Sieve Equality Dispatch

- [x] Reproduce `eval_sieve_primes_count_chirho` and confirm the missing `==` STG binding.
- [x] Inspect generated Core for the `filter (\x -> x mod p /= 0)` predicate.
- [x] Patch the narrow dict/type-key propagation gap without broad defaulting.
- [x] Run targeted test, watch surface, fresh probe.
- [x] Log, commit, push, and hand off for independent acceptance.
