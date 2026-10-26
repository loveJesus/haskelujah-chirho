<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Runtime primitive evaluation Chirho

The interpreter evaluates a lowered primitive only after its arguments reach weak head normal
form. Primitive identity is preserved from Core through STG lowering; an implementation must
therefore match the Haskell operation rather than a similarly named host-language operation.

```mermaid
flowchart LR
    CoreChirho[Core primitive name] --> StgChirho[STG PrimOpKindChirho]
    StgChirho --> ForceChirho[Force required operands to WHNF]
    ForceChirho --> EvalChirho[Evaluate the matching runtime primitive]
    EvalChirho --> ValueChirho[Return a typed ValueChirho]
```

## Numeric rounding

- `ceiling#`, `floor#`, and `truncate#` use their corresponding directed `f64` operations.
- `round#` uses nearest-integer rounding with exact half ties resolved to the even integer,
  matching `round :: Double -> Int` in GHC rather than Rust's away-from-zero `f64::round`.
- This interpreter workflow does not imply native support. LLVM and Cranelift currently refuse
  `round#/1` before execution, which remains a separate backend parity unit.
