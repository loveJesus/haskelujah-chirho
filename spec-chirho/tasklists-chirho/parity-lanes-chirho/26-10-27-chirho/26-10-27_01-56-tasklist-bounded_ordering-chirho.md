<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 605: Bounded Ordering Chirho

- [x] Reserve Row 605 in the canonical progress database.
- [x] Measure `minBound` and `maxBound :: Ordering` on GHC 9.14.1 and all three engines.
- [x] Trace the Bounded instance seed, evidence route, and generated Prelude rows.
- [x] Implement only the ground `Bounded Ordering` instance and body-backed bounds.
- [x] Add direct, first-class, structural, hold, and reach controls.
- [x] Run the focused gate and contract judge.
- [ ] Land through the canonical single-writer chain and close Row 605.

## Scope Chirho

- Cover direct and first-class `minBound` and `maxBound` at `Ordering`.
- Preserve the existing Int, Char, and Bool Bounded behavior.
- Keep structured Eq, qualified-name resolution, local Enum specialization, and tuple arity expansion outside this row.

## Placement Chirho

- Extend the existing Ordering-focused Prelude module when its current responsibility fits.
- Add no new top-level module unless the trace proves the existing module would mix responsibilities.
- Use body-backed dispatch and keep missing rows failing closed.

## Evidence Chirho

- GHC 9.14.1 prints `LT`, `GT`, `(LT,GT)`, and `[LT,GT]` for the direct,
  polymorphic, and list-shaped `Ordering` probes. Before this row, every engine
  rejects the same program with `E0204: no instance for Bounded Ordering`.
- The class environment now seeds only the ground `Bounded Ordering` instance.
  `$fBoundedOrdering` takes body-backed rows whose right-hand sides are exactly
  the nullary `LT` and `GT` constructors.
- `minBound` and `maxBound` join the occurrence-evidence population. A
  result-only `Bounded a =>` binding keeps its dictionary only when its body
  proves the exact `$own:k` predicate; Int, Char, and Bool retain their existing
  answers.
- A hidden-and-redefined pair of bounds remains the program's own pair on the
  interpreter, LLVM, and Cranelift.
- Reach M1 removed only the Ordering-row generator. The structural control
  failed because `$prim_Bounded_minBound_Ordering` was not body-backed.
- Reach M2 removed only the two occurrence names. The all-engine control failed
  in the interpreter with `no matching alternative for tag 0 at @2271`, proving
  that the result-only polymorphic binding had defaulted instead of taking its
  own dictionary.
- Both mutated files were restored byte-identical before the gate:
  `prelude_chirho.rs` is
  `1c189b023d0ace24013d052d08791131ab6c61a69fb8d05f7134a113160746d9`, and
  `evidence_join_chirho.rs` is
  `a2d9d811ec0c4d3f166d2a67b908a5738381162494a66e48dd4d4607e725eb19`.

## Gate r1 Chirho

- Source commit `a0733819` builds the workspace with no warning.
- The bounded focused run is GREEN at 678/678: typing 365/365, core 191/191,
  and dictionary evidence 122/122. The latter includes the interpreter, LLVM,
  and real Cranelift controls for the new Ordering bounds and the program-owned
  hold.
- One initial command selected the entire driver library, including unrelated
  60-second scale cases. It was deliberately interrupted after 885 clean passes
  and is not counted as gate evidence; the three intended bounded suites above
  replace it.
- Changed-line Clippy is GREEN: 454 diagnostic locations were read against 150
  changed Rust lines in seven files, with no diagnostic on a changed line.
- The 41-program contract judge is GREEN at 126/126, accounting for 73 movers
  and 9 holds. Row 605 moves no contract cell.
- Rustfmt, diff-check, and gitleaks are clean; gitleaks scanned the source
  commit and found no leak.
