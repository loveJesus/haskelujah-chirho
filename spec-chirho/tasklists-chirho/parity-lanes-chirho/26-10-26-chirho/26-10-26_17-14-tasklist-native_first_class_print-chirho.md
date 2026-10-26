<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native first-class print and Double spelling Chirho

Row 550 fixes the Show evidence lost when `print` is passed as a value, and makes native Double output use Row 549's shared GHC-compatible renderer. HASKELUJAH/claude2_chirho supplied source commit `0e06b53c` on exact main `72a338e8`; HASKELUJAH/gpt_chirho independently reviewed and landed it.

## Checklist

- [x] Preserve a solved `Show` occurrence when `print` appears inside an application spine such as `mapM_ print xs`.
- [x] Generate the backend-neutral `$print_via_show_chirho` helper in a dedicated Prelude child module.
- [x] Use `haskelujah_rts_chirho::show_double_chirho` from the native float FFI rather than Rust's generic formatter.
- [x] Measure first-class print and Double spelling controls on GHC 9.14.1 and run them on the interpreter, LLVM, and Cranelift.
- [x] Independently pass RTS 56/56, runtime 57/57, core 168/168, native error helpers 35/35, dictionary evidence 78/78, Show precedence 2/2, and IO actions 9/9.
- [x] Pass Clippy on the affected targets, rustfmt, diff-check, and gitleaks.
- [x] Land `0e06b53c` and record the progress evidence.

## Boundary

An integer-literal negative infinity inside a `[Double]` list still reaches `NegIntChirho` in the interpreter. Row 550 does not claim that separate negation/evidence residual.
