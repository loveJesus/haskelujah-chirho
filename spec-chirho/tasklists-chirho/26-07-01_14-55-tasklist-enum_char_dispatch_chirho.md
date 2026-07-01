<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Enum Char Dispatch Chirho

- [x] Reproduce `fromEnum (chr 97)` returning a `Char`.
- [x] Inspect dictionary method selection for `Enum` and `Data.Char` Prelude bindings.
- [x] Patch `Enum Char` to use `ord#` and infer `chr` as `Char` for dict selection.
- [x] Run adjacent enum/char eval tests and fresh CLI repro.
- [x] Log progress, commit, push, and coordinate handoff.
