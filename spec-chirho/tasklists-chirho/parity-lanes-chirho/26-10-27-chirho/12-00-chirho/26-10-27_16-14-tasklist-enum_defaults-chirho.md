<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Enum defaults Chirho

Row 669 (released in #30540; option A in #30561; derived bounds in #30577; the metadata channel in
#30584) gives a program's Enum instances GHC's defaults for the methods they do not write, and
derived instances GHC's bounded `enumFrom` and `enumFromThen`. Branch `enum-defaults-chirho` from
main 708a1329, fast-forwarded to 3a8b038d (progress DB only).

## Checklist

- [x] GHC's chain (`:info Enum`: MINIMAL `toEnum`, `fromEnum`). The formula probe shows each default
  visits every Int between its endpoints' `fromEnum`; the independence probe shows no default calls a
  method the instance writes.
- [x] Main's mechanism: the seeded class lays out four slots; omitted `succ`/`pred` were named
  missing-method stubs, and the four enumerations ran the compiler's Int bodies on the program's values.
- [x] Option A: `enum_defaults_chirho.rs` in the new `defaults_chirho/` home (the Eq and Ord default
  files moved byte-identical, hashes `bfccfdb6` and `341b12c5`).
- [x] Derived bounds through the approved channel: typing's `deriving_chirho/enum_chirho.rs`, the
  driver's one dictionary-pass call, and Core rows built from the key's rows, constructors and `>=#`.
- [x] Controls (`tests/dictionary_evidence_chirho/enum_chirho/`, `enum_occurrences_chirho.rs` moved
  byte-identical): 9 GHC-measured programs, 18/18 literals byte-identical after rustfmt, all three
  engines; the Core assertion on the derived rows; 3 typing unit tests on the metadata.
- [x] Residuals measured on main, not this row's: a written enumeration call beside a program-owned
  namesake of its own name (m3, `GtIntChirho` on main and candidate alike), and an empty derived Enum
  admitted where GHC rejects it (GHC-30750). The derived `error` spelling is unmeasured, left for its
  owner.
- [x] Reach: four mutants, each compiled, killed and restored byte-identical. No derived `enumFrom`
  row, and no derived `enumFromThen` row: each killed by the derived bounds, the Core assertion and
  the derived capture program. No evidenced defaults: killed by the written, independence and
  capture programs. No `succ`/`pred` hook: also killed by the polymorphic steps.
- [x] Gate on 8b68918f, base 3a8b038d (16:46 to 16:52):
  - core 195/195, typing deriving 40/40, dictionary_evidence 161/161, method_occurrence_evidence
    7/7 and typing_integration 81 (2 ignored);
  - a warning-free workspace check; changed-line Clippy 0 hits, with the workspace's 631 messages
    and 348 unique diagnostics unchanged; rustfmt clean on all 14 changed Rust files; diff-check
    and gitleaks clean.
  - Not run: the driver library's project and re-export selection and the broad scan, which do not
    reach this row's paths.
