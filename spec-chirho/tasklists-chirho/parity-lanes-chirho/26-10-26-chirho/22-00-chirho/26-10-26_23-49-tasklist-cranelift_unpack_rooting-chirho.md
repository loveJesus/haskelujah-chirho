<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# A list Cranelift unpacks stays rooted Chirho

Row 595 (gpt_chirho #29563, ruling #29585). On Cranelift alone,
`length (words (concat (replicate N "ab ")))` failed from N = 1,181. Row 588 found it. LLVM and
the interpreter were exact.

## Trace (#29584)

- Bisected on real `--cranelift`: exact through N = 1,180 (6/6 runs), failing from 1,181
  (6/6). The message varied by run: `<<loop>>`, "no matching alternative", "not valid UTF-8",
  or a hang.
- Collection off (scratch `GC_THRESHOLD_CHIRHO = 1 << 60`): exact at every size, so a
  collection freed a live value.
- Smallest reproducer: `length (concat (replicate N "ab "))`. dropWhile, takeWhile and lines
  over the same list fail too; filter and concatMap pass.
- A scratch quarantine RTS poisoned freed blocks and never reused them. It named the object: a
  24-byte cons cell `[1, 'b', tail]`, the second cell of the list unpacked from "ab ", entered
  right after collection 6 freed it.
- The window: `maybe_unpack_list_case_string_scrutinee_chirho` returned the fresh list unrooted.
  The alternative built `x : append xs' ys` (cons first, then the thunk capturing `xs'`), and a
  collection there freed the tail.
- LLVM roots produced heap values for the process lifetime. Under the quarantine it is exact at
  1,181 to 30,000.

## Ruling and fix

- Option (a), bounded: `root_unpacked_list_chirho` pushes the list at all three Cranelift unpack
  sites, through the existing process-lifetime push.
  - The case on a literal is the measured mover.
  - The `unpack` application and the `unpack#` primop share the window. `Data.Text`'s `unpack`
    reaches them once a step: a scratch RTS counter saw 10 calls at 10 steps and 20 at 20.
    Lowering markers showed each site reached. Swept over 45 sizes under the quarantine, no
    collection landed inside their window.
- Not widened: root-every-call, and scoped root frames.

## Checklist

- [x] Trace and checkpoint (#29584); ruling (a) (#29585).
- [x] Fix at the three sites. After it, k02 is exact at N = 2,000 and 2,001, 6/6.
- [x] Controls, from GHC 9.14.1's output:
  - the four walks at their failing sizes on both natives;
  - the same walks at 50 on every engine;
  - the Text sites on Cranelift.
- [x] Reach mutation removing the case-site push: the native control fails on Cranelift, 3/3,
  each by a named failure ("not valid UTF-8", "no matching alternative"). The file was
  restored byte-identical.
- [x] Real-Cranelift transition table, the exact base cbd7be40 against 15d60d9a:
  - the four reproducers: GAIN-EXACT on Cranelift;
  - the row 579 k02 cell: GAIN-EXACT on Cranelift;
  - row 588's ten probes, the row 576 battery and the row 535 contract: every cell holds;
  - marker audit: 528 compiled cells marked, 0 unmarked;
  - no signal or timeout.
- [x] Gate r1 RED on one item: the pre-existing collapsible_if around the `unpack` application,
  whose span held the edited lines. It is now a let-chain.
- [x] Gate r2 GREEN on e2b154c2 (main 9ecd15e1) in 185 s:
  - build with 0 warnings; LLVM 38, Cranelift 42;
  - eight driver suites 222, and the driver library filter 28;
  - Clippy: none of 531 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
  The crates interdiff from the measured 15d60d9a is only that let-chain.

## Residual

- Both backends keep GC roots for the process lifetime: Cranelift never pops, and LLVM pops
  only construction temporaries. So a native heap frees only unrooted RTS temporaries, and
  retention grows with the program. Scoped root frames are a separate bounded-growth row.
