<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native Double and String compare Chirho

Row 539 started from `6f027ffd13953318ae2997ba2de841cd8a37582e`, which is main_chirho after row 538 plus row 540's DB-only open. Before its first commit it moved onto `1b804afaa91d49c02c290e0b736839dc5d82b9f7`, where row 540 is closed. The LLVM edit re-applied cleanly over row 540's `fromIntegral#` arm.

It owns `compareFloat#/2` and `compareStr#/2` on LLVM and Cranelift. With them, Compare2 prints GHC's `GT GT LT GT` natively instead of being refused on both backends.

It follows row 535 C10 (`a14baa14`): the Ordering is LT when less, EQ when equal, and GT otherwise. That is exactly how GHC's `Ord Double` instance defines `compare`, and how `Ord [Char]` orders by code point.

## Placement

- **LLVM.** `codegen_chirho/primitives_chirho.rs` lowers both primitives.
  - One helper turns a less flag and an equal flag into the Ordering tag, for all four compare primitives.
  - `eqStr#` and `compareStr#` share one `strcmp` emission.
- **Cranelift.** C10's `lower_chirho/ordering_chirho.rs` lowers both, through the same tag helper as `compare#`. `lower_chirho.rs` dispatches them.
- **Driver controls** (`native_error_helpers_chirho.rs`):
  - Compare2 itself runs through the interpreter and both native backends.
  - A GHC-measured edge program runs on both native backends.
- **The native failure workflow** records that both primitives have left the refused set.

## Semantics

- **Double.** `olt` gives LT, `oeq` gives EQ, and anything else gives GT. So NaN on either side gives GT, and -0.0 equals 0.0, as on GHC 9.14.1.
- **String.** C `strcmp` on the backends' NUL-terminated strings: negative gives LT, zero gives EQ, and positive gives GT.
  - `strcmp` compares bytes as unsigned char.
  - UTF-8 byte order is code-point order, which is GHC's order on `[Char]`.

## Boundaries

- **No interpreter change.** Two interpreter residuals were measured and recorded, not fixed:
  - Its `compareFloat#` gives EQ whenever either side is NaN (`partial_cmp(..).unwrap_or(Equal)`, `eval_chirho.rs:2688-2692`), while GHC gives GT. On the edge program the interpreter prints `EQ EQ EQ` for the first three lines, where GHC prints `GT GT GT`.
  - `print (compare (0 / 0 :: Double) 1)`, with integer literals at Double, fails in the interpreter with `runtime error: missing STG binding `/` for CoreId 8` (rc 1). This was measured at 6f027ffd, and again on the candidate over 1b804afa.
- **String representation stays as it is.** This unit uses the representation `eqStr#` already uses.
  - A String that reaches `compareStr#` as a cons list rather than a C string is the existing string/list mismatch, outside this unit.
  - An embedded NUL is likewise unrepresentable.
- **Nothing else moves.** No Wasm, no type inference and no other refused primitive.
  - P01, P04 and P05 stay refused.
  - P03 (`fromIntegral#`) landed as row 540. The edge program writes its Double literals in fractional form, which keeps both `fromIntegral#` and the interpreter's `/` residual out of the measurement.
- **The gate** uses this worktree's warm incremental target.
  - There is no machine-wide Cargo lock, no cold target, no serial mutation campaign and no whole-receipt-tree hash walk.
  - The negative cases are ordinary tests.
  - The candidate CLI runs once over row 535's saved 41-program contract.

## Checklist

- [x] Record the row, mechanism, boundaries and gate before source edits. Base measured at 6f027ffd: Compare2 is EXACT in the interpreter and refused on LLVM and Cranelift with `compareFloat#/2, compareStr#/2`.
- [x] Lower both primitives on LLVM, sharing the Ordering and `strcmp` helpers. `emit_compare_chirho` now holds all four compare primitives. `eqStr#` and the float arithmetic reuse `emit_strcmp_chirho` and `emit_double_chirho`, with the same temporaries in the same order.
- [x] Lower both primitives on Cranelift in `ordering_chirho.rs`, behind one `ordering_chirho` tag selection. `compareStr#` is refused when the `strcmp` import is absent.
- [x] Turn Compare2 into an exact interpreter/LLVM/Cranelift control, and add the GHC-measured native edge control.
  - Compare2 replaces C10's Cranelift-only Int/Char control. Its program starts with the same two lines and now also covers LLVM and the interpreter.
  - The native half of `assert_prints_everywhere_chirho` is now `assert_prints_natively_chirho`, which the edge control shares.
- [x] Update the native workflow at the changed seam, with the NaN residual.
- [ ] Run the warm gate: fmt, the focused suites, changed-package Clippy, diff check, and one candidate CLI pass over row 535's contract.
- [ ] Record the measured evidence.

## Measured evidence

- **GHC 9.14.1** (`/opt/homebrew/bin/ghc -O0`):
  - Compare2 prints `GT GT LT GT`.
  - The edge program prints `GT GT GT EQ EQ GT LT LT EQ LT GT GT LT`, with rc 0 and an empty stderr.
