<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native unpack survives collection Chirho

Row 566 (gpt_chirho #29096). Row 557's measurement found that natively a long String used as a
list read freed memory. On main, `length "<2000 characters>"` printed 240 (GHC 2000), and a longer
one died with SIGBUS. String representation semantics remain row 560's.

## Roots, both needed

1. `haskelujah_unpack_string_chirho` borrowed its input's bytes across allocations, and left the
   suffix it had built unrooted while allocating each cell.
2. The collector's marker queued each child by its tagged value. The registry lookup at the
   queue's head missed it, so nothing below a root's direct children was scanned. Rooting the
   suffix alone kept only its first two cells; the probe showed the rest freed and reused, a cycle.

## Checklist

- [x] Own the input bytes, and root the built suffix around each cell allocation.
- [x] Queue a marked child by its registry address.
- [x] RTS controls: a four-cell tagged chain survives whole; an unpack keeps every cell across a
  collection in its middle, its walk bounded so a reused cell fails instead of looping.
- [x] The old marker as a mutant fails both, by "the cell at depth 2 was collected" and "the list is
  longer than its text"; the file was restored byte-identical.
- [x] Native control: a 2000-character literal's `length` and slice match GHC 9.14.1 on both
  natives. It was all-engine at landing; it is native-only since the interpreter's own long-list
  recursion overflows a 2 MiB test thread (at 1000 cells), which aborted the suite without
  `RUST_MIN_STACK` (gpt_chirho #29248).
- [x] Transitions, a fresh build of main 85b71faf against the candidate on all three engines:
  - 115 one-line programs: 2 gains, the long literal's `length` natively (240 to 2224); every
    other cell holds;
  - the long-literal program and row 535's 41 contract programs: 2 gains, the long literal
    natively (before: `20` and `21` with an empty slice); all 123 contract cells hold.
- [x] Gate GREEN in 115 s:
  - RTS 60, LLVM 38, Cranelift 42;
  - seven driver suites 168, native laziness among them;
  - Clippy: none of 488 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
