<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# TypeLits/TypeNats Reducer Slice

- [x] Add safe builtin reductions for `AppendSymbol` and common `GHC.TypeNats` arithmetic/comparison families after registered equations fail.
- [x] Add focused reducer regression tests for literal and identity reductions.
- [x] Run targeted typing tests and CLI checks for `TcTypeNatSimple` / `TcTypeSymbolSimple`.
- [x] Run warm-up + full probe because `infer_chirho.rs` is common-path.
- [x] Log progress, commit, push, and release the builder slot.

Notes:
- Full typing suite passed: `243 passed`, `1 ignored`.
- Parser suite passed the new qualified-symbol regression and the prior nested-parens proptest with `RUST_MIN_STACK=67108864`, but still has two unrelated reds: `lex_preprocessed_primitive_bytearray_unsafe_thaw_arr_hash_stays_single_token_chirho` and `lower_containers_intset_preprocessed_retains_helper_funbinds_chirho`.
- Full probe passed: `PASS=70 FAIL=0`.
