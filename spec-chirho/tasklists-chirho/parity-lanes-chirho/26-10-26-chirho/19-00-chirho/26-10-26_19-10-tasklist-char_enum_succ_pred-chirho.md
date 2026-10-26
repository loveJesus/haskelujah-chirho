<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 567: Char Enum successor and predecessor

Base: `ee7d29ac`, immediately after the bounded row 520 occurrence-dispatch landing.

- [x] Claim row 567 and isolate it from rows 565 and 561.
- [x] Measure GHC 9.14.1, the interpreter, LLVM and Cranelift on direct, higher-order and boundary uses of `succ` and `pred` at `Char`.
- [x] Trace the current `Enum Char` rows and the per-occurrence evidence route before editing source.
- [x] Implement only the bounded successor/predecessor repair, keeping ranges separate unless the trace proves one mechanism.
- [x] Add GHC-measured all-engine controls and named boundaries.
- [ ] Run a warm focused gate, changed-line Clippy, rustfmt, diff-check and gitleaks.
- [ ] Rebase onto exact main, rerun the mechanism controls, land, and close progress row 567 only if GREEN.

The present row does not claim Haskell literal decoding, String/Char rendering, native String
representation, result-only `toEnum` defaulting, surrogate code points, invalid `chr` semantics,
or `enumFrom`/`enumFromTo` until their source relationship is measured.

Current main failed the first `succ 'a'` use on every engine with `missing method Enum.succ for
Char`. The checker already supplied `Enum Char`; the dictionary had no body-backed `succ` or
`pred` row. The new rows shift `ord#` by one and restore `chr#`. Direct, mapped, constrained and
derived uses now reproduce GHC's eight-line output on all three engines. `succ maxBound` and
`pred minBound` keep GHC's exact payloads. GHC permits shifts into the surrogate interval, but
the current Rust `char` interpreter cannot represent those values; all three engines now refuse
them by the same named boundary instead of silently returning NUL.

The adjacent range measurement is a different mechanism. LLVM and Cranelift already print the
GHC-exact ordinal lists for `['a'..'e']` and `['e','c'..'a']`; the interpreter prints mixed
Char/Int lists. Row 567 does not alter range generation.
