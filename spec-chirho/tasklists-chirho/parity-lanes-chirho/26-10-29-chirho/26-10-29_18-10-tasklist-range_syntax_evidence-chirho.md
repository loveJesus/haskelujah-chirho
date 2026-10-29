<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 665: range syntax method identity and evidence Chirho

Owner: GPT range-evidence helper, from `ff133d3e`, independently of Claude2's Enum Double row.

- [x] Trace the range's missing type arm, lexical capture and evidence-join authority boundary.
- [x] Share canonical Enum enumeration schemes between builtin seeding and syntax typing.
- [x] Infer and unify every endpoint; record the range's own Enum reference evidence.
- [x] Mint standard method occurrences independently of lexical bindings, including RebindableSyntax.
- [x] Join only compiler-marked range occurrences to their evidence, without widening authority for written names.
- [x] Add endpoint rejection, namesake, user-instance, NoImplicitPrelude and active RebindableSyntax controls.
- [x] Obtain the machine-wide builder lease, compare all programs with GHC, and run focused tests.
- [x] Prove the byte-exact published `tcfail009.hs` verdict move against GHC and the baseline.
- [x] Normal-suite negative controls cover unresolved/conflicting evidence, non-Enum names, missing instances and incompatible endpoint types. Source-edit mutant loops are omitted at the parent's direction.
- [x] Warning-free workspace check and rustfmt.
- [x] Changed-line Clippy and final hygiene: 294 existing diagnostic locations read, 0 on 685 changed lines in 16 Rust files; rustfmt and diff-check clean; gitleaks scanned the source commit with no leaks.
- [x] Post the held tested source to the parent and release the builder slot for Claude2's atomic stack. No further builds are queued in this lane.

The four enumeration slots in polymorphic Enum dictionaries remain row 671. Double remains row 670. The controls define every user enumeration method explicitly.

## Measured source and boundary

The source is `df506113` on the repaired row 675 record `ade87151`. Compiler source was unchanged between the warning-free build, focused tests and frozen CLI. That CLI's SHA-256 is `9d5c858652a089278c0eb62ca5402f19f6d94a0607f8d9185ea2ffef321fad93`.

- GHC 9.14.1: seven exact programs, including a two-module imported namesake, and three rejected programs.
- Core 196/196, typing 371/371, dictionary evidence 166/166, and the driver library's evidence selection 35/35 pass. The dictionary target took 334.84 seconds. All seven positive programs run on the interpreter, LLVM and true Cranelift.
- The six single-module positive sources print wrong answers on the frozen baseline interpreter and GHC's output on the frozen candidate. The imported-namesake program is exact in the candidate's multi-module interpreter and both native project paths.
- The baseline accepts `tcfail009.hs`, and the candidate refuses the same bytes with E0200 naming Int and Integer. GHC independently refuses them with GHC-83865. The missing-Enum and Char/Bool negatives likewise change from accepted to refused for their actual obligations.

This source stays held for the atomic 665 + 670 + 671 stack. A Double range was accepted on the baseline and failed at runtime with `SubIntChirho: expected Int#`; the candidate instead refuses it at compile time with `no instance for Enum Double`. This is a measured boundary, not a compatibility improvement. The stack must supply correct Double and dictionary routes before landing.

Receipts are under this worktree's `tmp-chirho/r665-chirho/`: `oracle-r1-chirho`, `cli-cells-r1-chirho`, `check-r1-chirho.log`, `core-typing-r1-chirho.log`, `dictionary-r1-chirho.log`, `evidence-r1-chirho.log` and `build-r1-chirho.log`. No main, progress DB or broker write is made by this lane.
