<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native ++ and literal tails read whole Chirho

Row 576 (gpt_chirho #29274), the row 560 remainder: roots 2 and 3 of row 557's trace. The
design was accepted on bounds, and the walker inventory was confirmed before source (#29280).

## Measured on main 03430956

- Root 2: native `++#` read a runtime list as "". Six programs were silently wrong:
  - `"<" ++ reverse "abc" ++ ">"` printed `<>`;
  - `length ("ab" ++ reverse "cd")` was 2;
  - `print ("<" ++ reverse "ab")` printed `"<"`;
  - also `map succ`, two runtime operands, and `replicate` with `++`.
- Root 3: a literal tail cut the list.
  - `putStrLn ('a' : "bc")` printed `a` natively.
  - `('a' : "bc") ++ "d"` printed `d` natively and `ad` in the interpreter.
  - `print ('a' : "bc")` was refused natively.

## Design (A/B/C)

- (A) Append packs a list operand. Both operands are rooted before either is packed, the roots
  are balanced, and both byte sequences are owned before the result allocation; the GC mutex is
  never held across thunk entry or allocation.
- (B) Only pack and the native show reader read a forced, nonzero tail as a literal's UTF-8
  buffer. **Refinement (#29287): registration decides, not the low bit.** A byte-aligned literal
  at an odd address panicked both natives on a misaligned read when the low bit was used.
  `native_maps` is untouched, and the NUL, invalid-UTF-8 and unencodable lines are unchanged.
- (C) The interpreter's append walk absorbs only a literal tail.

## Checklist

- [x] Walker inventory: only pack and the native show reader walk a [Char] list (#29279).
- [x] A, B and C; the 15 measured programs move exactly the nine programs' 19 cells to GHC-exact.
- [x] Controls:
  - the nine-case program on all three engines, generated from runghc's output;
  - an RTS proof that a buffer operand survives a collection inside append's packing;
  - pack reading a literal tail at even and odd addresses.
  RTS 68/68, native_error_helpers 46/46.
- [x] Row 557's literal-tail refusal test is replaced by the exact control. The refusal line is
  retained for a non-UTF-8 buffer or a value that is neither a list nor a buffer.
- [x] Reach mutations, each killed by a named assertion, every file restored byte-identical:
  - A: append reads a list as a buffer;
  - the rooting: the root window roots nothing. The first, end-to-end form was INVALID: it hit
    undefined behavior reading the freed operand. Append's root window is now a helper whose
    control checks liveness inside it: `(false, false)`, not `(true, true)`;
  - B in pack;
  - B in the show reader;
  - C in the interpreter.
- [x] Battery: a fresh build of main 76ed56e4 against the candidate on all three engines.
  - 130 one-line programs: 30 gains, all of them the nine measured programs, counting their copies
    in the earlier battery (m01, m02, m03, m05, t03). No other cell moves.
  - 48 programs: the nine-case control gains on all three engines, and all 123 row-535
    contract cells hold.
- [x] Gate GREEN:
  - RTS 69, runtime 58, core 174, LLVM 38, Cranelift 42;
  - seven driver suites 182, and the driver library filter 179;
  - Clippy: none of 499 warnings on a changed line; diff-check and gitleaks clean.
- [x] Rebased over row 575, then re-gated against a fresh main 04a3b6a0: the same 30 one-line and
  3 program gains, nothing else moving, the contract holding. RTS 70, runtime 58, core 174,
  LLVM 38, Cranelift 42, driver suites 183, driver library 179, Clippy 0 of 499 on changed lines.
- [x] Replayed onto 0d4a5b8e: row 577 shares no crate path with this row, and the merge is clean.
