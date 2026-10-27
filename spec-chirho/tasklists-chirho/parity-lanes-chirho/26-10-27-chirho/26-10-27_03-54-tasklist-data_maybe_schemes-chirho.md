<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# The Data.Maybe helpers at GHC's types Chirho

Row 611 (gpt_chirho #29919; checkpoint #29934; approved #29936). The bare schemes of `fromMaybe`,
`isJust`, `isNothing` and `maybe` took any constructor `f a`.

## Trace (#29934)

- **Seed:** `builtins_chirho/core_chirho/part_03_chirho.rs`.
- **Origin:** bea5612c (row 429, 2026-07-05) relaxed the four helpers for five driver tests. Each
  declared its own `data Maybe` and applied the builtin helpers to it, programs GHC 9.14.1 rejects
  (GHC-87543). The "certified-17" list it served was the driver suite, not GHC programs.
- **Base 26942425:**
  - every valid use is exact on all three engines, except qualified Data.Maybe aliases, which
    fail (row 608 DISAGREE);
  - every GHC-rejected misuse checks: the interpreter fails at run time, and the natives print
    wrong values.

## Checklist

- [x] The four schemes name `Maybe` again, at the seed. No Phase 3.9 override and no runtime-body
  change.
- [x] Row 608's table, counts and disagreements regenerated through the committed generator:
  - +5 pairs, -0: Data.Maybe `fromMaybe`, `isJust`, `isNothing` and `maybe`, and Prelude `maybe`;
  - agree 721 -> 726, disagree 684 -> 679; the no-bare-scheme (2,870) and not-typed-by-GHC
    (8,446) counts unchanged.
- [x] The five legacy tests rewritten as GHC-valid programs with the same values. GHC 9.14.1
  typechecks each, and ghci evaluates `main` to the expected value.
- [x] Controls embedded from GHC 9.14.1 (`maybe_helper_schemes_chirho.rs`):
  - the valid uses, a program-owned shadow and the qualified aliases on all three engines;
  - seven misuses as frontend rejections;
  - the old local-`Maybe` shape as a rejection.
- [x] Transition, from base to the candidate, on 16 probe programs: 30 GAIN, 18 hold-exact, 0
  losses. Row 608's 59-program battery: every cell holds. Corpus selector, 7 should_compile and
  1 should_fail file: 0 moved.
- [x] Gate r1 GREEN on 36ee5927 (main 26942425) in 432 s. The frontend package builds and
  eval_basic carry the time.
  - build with 0 warnings; naming 153, core 193, typing 366;
  - driver suites 361, and the driver library's frontend, qualified, import, scope and
    eval_basic tests 609;
  - Clippy: none of 537 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
- [x] Row 535 contract judge on the final CLI: GREEN, 126 cells, 73 movers and 9 holds.
- [x] Reach mutation restoring the free-`f` schemes:
  - the misuse and local-`Maybe` rejection controls accept ("we accepted it");
  - row 608's drift test fails at `Data.Maybe.fromMaybe`;
  - the valid and qualified controls hold;
  - row 608's generator, run under the mutant, drops exactly the five entries and reverts to
    721/684.
  The file was restored byte-identical.
- [x] Final CLI against base: 16 programs give 30 GAIN and 18 hold-exact; row 608's 59-program
  battery holds; the corpus selector moves 0.
