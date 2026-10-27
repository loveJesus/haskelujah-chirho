<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 574: negative Int power

Base: `98b20b0e`, the exact main tip carrying row 574's progress reservation.

- [x] Reserve row 574 and announce the disjoint source lane.
- [x] Measure GHC 9.14.1 and the interpreter, LLVM and Cranelift before editing source.
- [x] Trace the shared Int `^#` implementation and its existing failure route.
- [x] Implement the smallest all-engine negative-exponent repair.
- [x] Add GHC-measured controls for direct, expression and first-class uses, plus nonnegative holds.
- [x] Prove the guard is reached by a focused mutation.
- [x] Update the native failure workflow.
- [x] Run the warm focused gate, changed-line Clippy, rustfmt, diff-check and gitleaks.
- [x] Rebase onto exact main and rerun the actual behavior controls.
- [ ] Land and close only row 574.

Scope is `(^)` at an `Int` result with a negative integral exponent. GHC fails with the named
`Negative exponent` payload, while main currently returns `0` silently on the interpreter, LLVM
and Cranelift. Nonnegative Int powers remain byte-for-byte behavioral holds. Double `(**)`, `(^^)`,
Double `(^)`, and unrelated numeric routing are outside this row.

## Measurement and root

GHC 9.14.1 fails with `Negative exponent` for direct `2 ^ (-1)`, a negative base and exponent,
a top-level negative exponent binding, first-class `map (2 ^)`, and an exponent annotated as
`Integer`. Exact-main Haskelujah instead exited successfully on every engine, printing `0` for
the scalar cases and `[0,1,8]` for the first-class case. The nonnegative controls printed `1`,
`1024`, and `-27` on GHC and all three engines.

LLVM and Cranelift already call one `haskelujah_pow_int_chirho` RTS wrapper. The interpreter had
the same `exponent < 0 => 0` rule independently. `integer_power_chirho::pow_int_chirho` is now
the one pure wrapping-power implementation: it returns `None` for a negative exponent. The
interpreter maps that to its existing `NegativeExponentChirho`; the native wrapper maps it to
the existing user-failure payload. Backend lowering is unchanged.

The reach mutation changed only the pure helper's negative branch back to `Some(0)`. The
all-engine named-failure control failed immediately because the interpreter returned `0`; the
source was restored to its pre-mutation SHA-256 and the control passed again.

## Gate checkpoint

The first build completed without warnings. The warm focused gate passed 65 RTS library tests,
58 interpreter-runtime tests, and all 46 `native_error_helpers_chirho` controls in 18 seconds.
The five negative-exponent forms fail named on the interpreter, LLVM and Cranelift; the expanded
nonnegative hold prints `1`, `1024`, and `-27` on all three. Clippy reported no diagnostic on a
changed line. Rustfmt and `git diff --check` are clean. The exact-main replay and final landing
remain open because row 573 finished on the same base while this gate ran.

## Final replay

Row 573 landed first and added native Double `**` beside this row in three source files. The
replay kept both routes; its sole content conflict was the RTS arithmetic test imports, resolved
as their union. The replay passed all 66 RTS tests, all 58 interpreter-runtime tests, Row 573's
all-engine `**` control, and both Int-power controls. The subsequent row 566 test-only correction
made the long-literal control native-only; after replaying onto that exact main, the complete
46-test `native_error_helpers_chirho` suite passed at the default 2 MiB test stack. The final
170-test replay took 21 seconds, formatting and diff checks stayed clean, and gitleaks found no
leaks in the row 574 commit.
