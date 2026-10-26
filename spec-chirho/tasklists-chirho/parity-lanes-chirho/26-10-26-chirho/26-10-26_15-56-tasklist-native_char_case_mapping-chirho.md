<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native Char Case Mapping Chirho

Row 544 was scoped from held main `1b804afaa91d49c02c290e0b736839dc5d82b9f7`, replayed onto `66832ade` after Rows 539, 541, and 542, and finally rebased onto combined Rows 531, 536, and 517 at `80e3b7de`. It owns saved native oracle P02:

```haskell
main = putStrLn (map toUpper "abc")
```

GHC and the interpreter print `ABC`. LLVM currently refuses `toUpper#/1`; Cranelift exits 0 with empty stdout. The unit is not complete unless the saved program is exact on both native backends; a single-Char proxy is insufficient.

## Plan

- [x] Trace the two native failures separately: LLVM primitive refusal and Cranelift's lost mapped String.
- [x] Place shared Unicode case-mapping behavior at one backend-neutral boundary if the native representations permit it.
- [x] Add ordinary controls for the saved P02 behavior and the representation boundary it reaches.
- [x] Update the native execution workflow at the changed seam.
- [x] Run the warm 2-5 minute gate at the behavior's actual scope.
- [x] Rebase onto released main and preserve the Row 539, 541, and 542 contracts.
- [ ] Land and close progress row 544 GREEN after the current main/DB hold releases.

## Placement

- `haskelujah_char_to_upper_chirho` lives in the RTS and implements the same first-scalar `char::to_uppercase` convention as the interpreter. LLVM and Cranelift call it; no backend owns a second character table.
- `haskelujah_put_str_ln_chirho` accepts the two native String representations without inspecting arbitrary bytes. A registered low-bit-tagged value is a boxed `[Char]` and is packed; an aligned or static pointer is already a C string.
- The saved P02 program is the driver control. RTS unit controls independently pin the character conversion and the representation discriminator.

## Post-rebase evidence

- The replay conflict resolution preserves Row 541's three Cranelift IORef runtime calls, Row 539 and 542's all-engine comparison controls, and the workflow records for all four units.
- The RTS, LLVM, and Cranelift library suites passed 52 + 38 + 42 tests.
- P02 printed exact `ABC\n` through the interpreter, LLVM, and Cranelift. The P01 IORef control and both Row 539/542 comparison controls also passed through all three engines after the overlap resolution.
- Format and diff check are clean. Warm changed-package Clippy completed successfully; its diagnostics are pre-existing, with none on a Row 544 inserted or conflict-resolved line. Gitleaks scanned both commits with no leaks.
- After combined Rows 531, 536, and 517 landed, all three Row 544 commits rebased without conflict onto `80e3b7de`. The shared crates were recompiled against that exact base. The RTS, LLVM, and Cranelift libraries passed 52 + 38 + 42 tests, and all 26 native error-helper controls passed, including P01, P02, P03, P06, and the comparison programs. The warm rerun took 35 seconds.
- Rustfmt, diff-check, and changed-package Clippy remain clean at the final source tree; Clippy's emitted diagnostics are pre-existing and none intersects a Row 544 hunk. Gitleaks scanned the three rebased commits and found no leaks.
