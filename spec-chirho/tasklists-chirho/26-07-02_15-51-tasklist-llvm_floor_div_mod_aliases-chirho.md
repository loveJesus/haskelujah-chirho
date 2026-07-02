<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# LLVM Floor Div Mod Aliases Chirho

- [x] Reproduce `llvm_round_trip_floor_div_mod_negative_operands_chirho`.
- [x] Check LLVM and Cranelift lowering for `div#`/`mod#` and `divInt#`/`modInt#`.
- [x] Patch LLVM alias handling for floor `divInt#`/`modInt#` while preserving `quotInt#`/`remInt#` truncation.
- [x] Run targeted LLVM/backend gates and update PRD/progress.
