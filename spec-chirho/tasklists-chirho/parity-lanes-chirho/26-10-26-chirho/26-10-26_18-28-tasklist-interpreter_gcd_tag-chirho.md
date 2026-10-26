<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Interpreter `gcd`/`lcm` tag parity Chirho

Row 564 owns the interpreter's `no matching alternative for tag 47` failure when Prelude `gcd`
or `lcm` is demanded through an `IO` program. It does not change native lowering, Char Enum,
String representation/rendering, prefix-minus fixity, or the Ord aggregate lane.

## Contract

- Measure GHC 9.14.1 and the current interpreter before source.
- Trace why the existing pure `gcd`/`lcm` control passes while the printed pair fails.
- Repair only the representation or case seam that produces the impossible tag.
- Cover positive, negative, zero, equal, coprime, and first-class calls as the traced scope permits.
- Preserve GHC's nonnegative `gcd` and `lcm` results.

## Checklist

- [ ] Record the GHC/interpreter matrix and Core/STG shape.
- [ ] Identify the first wrong representation or case decision.
- [ ] Implement the narrow repair with focused controls.
- [ ] Run the focused gate under three minutes.
- [ ] Verify rustfmt, diff-check, changed-line Clippy, and gitleaks.
- [ ] Land and close progress row 564.
