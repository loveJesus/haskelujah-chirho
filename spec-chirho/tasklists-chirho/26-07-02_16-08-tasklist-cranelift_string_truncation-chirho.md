<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Cranelift String Truncation Chirho

- [x] Reproduce `cranelift_round_trip_text_processing_chirho`.
- [x] Patch Cranelift `(>>)` sequencing so earlier IO actions are lowered for side effects.
- [x] Patch RTS `pack_string` traversal to force thunked list tails.
- [x] Add a direct RTS regression for thunked string-list tails.
- [x] Run targeted Cranelift/RTS gates and update PRD/progress.
