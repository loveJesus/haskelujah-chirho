<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Unit 523: integral Enum rows, a prerequisite of unit 520 (claude2_chirho)

The unit was opened by gpt_chirho's disposition on #27670. That disposition
rejected the generic Int fallback, because it lends another type's method once
the checker has proved K. The design is #27673, accepted at source scope with
the checked form. Branch `integral-enum-rows-chirho` sits on the held 0f256e43.
It does not depend on units 519 or 520, R1/R2, case or NonEmpty, and it carries
no Cargo until a named builder slot.

**Base.** In the files this unit touches, 0f256e43 matches main_chirho:
`dict_chirho/prelude_chirho.rs` and `dict_chirho/mod.rs` are the same blobs, and
both bases fill dictionary slots through `lookup_body_backed_name_id_chirho`
first. The two `crates` trees differ in other files: main 5277b0c4 has O1, and
held carries the generic-head, parser and typing commits.

## Contract, as implemented

- **Rows.** Sixteen keys get `$prim_Enum_toEnum_K :: Int -> K` and
  `$prim_Enum_fromEnum_K :: K -> Int`.
  - Signed: Int8, Int16, Int32, Int64.
  - Unsigned: Word, Word8, Word16, Word32, Word64.
  - Natural.
  - C: CChar, CInt, CLong, CUInt, CULong, CSize.

  Float and () are out.
- **fromEnum** is the identity.
- **toEnum** returns the tag inside the key's range. Outside it, it fails through
  `error` with GHC 9.14.1's measured message, naming the representation type
  for the C wrappers. Natural fails with `toEnum: unexpected negative Int`.
  Int64 and CLong are unchecked; the unsigned 64-bit keys and Natural check
  only for a negative tag.
- **Exact-key dispatch.** The rows are generated before the instance
  dictionaries, so `resolve_or_missing_method_id_chirho` takes them and
  generates no stub. The evidence route is unchanged.
- **Default target.** The C bounds and messages are the measured aarch64-darwin
  model. There is no target ABI map and no claim of cross-target C parity.
- **Failure route.** `error` is mapped on held; `error#` is not used. Per
  gpt_chirho's corrected review, the Char unit's `error#` mapping is NOT part of
  this unit.

## Placement

- New: `dict_chirho/prelude_chirho/integral_enum_chirho.rs` (the table, the
  generator and the guards) and its test module `integral_enum_tests_chirho.rs`.
- `prelude_chirho.rs` gains one `mod` line after `mod demand_chirho;` and one
  call before `// ── Bounded Int`. Both sit outside the hunks the Char unit
  changes (the mod list near line 4, Enum Char at 7699, chr/ord at 8740), so the
  combined replay has no overlapping hunk.
- The driver's `stg_lower_chirho/primitives_chirho/tests_chirho.rs` gets one
  focused computed-message control.
- New workflow: `language-features-chirho/integral-enum-rows-chirho.md`.

## Controls

**On this branch (core and STG, no end-to-end red test):**
- `every_integral_dictionary_takes_real_rows_chirho`: all 32 `$fEnumK` slots hold
  body-backed rows, never a missing-method stub.
- `from_enum_rows_are_the_identity_chirho`.
- `to_enum_rows_guard_exactly_the_table_bounds_chirho`: the guards and their
  order.
- `each_row_raises_ghcs_measured_message_chirho`: the table holds exactly the 16
  measured keys; 23 measured failures render GHC's exact text; 23 measured
  in-range tags pass every guard.
- `a_computed_error_message_fails_with_its_exact_text_chirho` (STG): the rows'
  `showInt#`/`++#`/`error` chain raises its exact text on held.

**On the combined Char replay** (the six rows already at 004cdbf4, plus a
pathed child generated from the measured files):
- the six never-wrong `toEnum` rows;
- 16 scalar round trips `print (fromEnum (toEnum 7 :: K))`, expecting 7;
- the signed negatives, expecting -1;
- the 23 failures, which must carry GHC's message;
- () as a green never-wrong control.

Float is a measured comparison only, not a test.

**Mutants** must terminate, and each must fail a named control:

| Mutant | Change | Must fail |
|---|---|---|
| MU1 | drop the call in `generate_enum_bounded_prelude_chirho` | `every_integral_dictionary_takes_real_rows_chirho` (stubs return) |
| MU2 | swap the order of the lower and upper guards | `to_enum_rows_guard_exactly_the_table_bounds_chirho` |
| MU3 | off-by-one: the upper guard uses `>=` | `to_enum_rows_guard_exactly_the_table_bounds_chirho` (op `>=#`), and on the combined replay the in-range bound rows |
| MU4 | name the C key instead of its representation (`{CInt}`) | `each_row_raises_ghcs_measured_message_chirho` |

These are predictions, not run.

## Measurements

GHC 9.14.1, held ba1e5c29 and candidate d14dd58d. Frozen hashes were unchanged
in every batch.

| Batch | Programs |
|---|---|
| list form | 62 |
| scalar form `print (fromEnum (toEnum t :: K))` | 46 |
| silence probes | 5 |

Receipts and the per-key table are in
`tmp-chirho/integral-enum-1018-chirho/design-table-chirho.md`, beside
`out-chirho/`, `scalar-out-chirho/` and `silent-out-chirho/`, in claude2_chirho's
worktree.

## Checklist

- [x] Design measured and accepted (#27673; gpt_chirho's disposition).
- [x] Source checkpoint: the rows, the core controls and the STG control are
  rustfmt-clean, and the measured tables are generated from the receipts.
- [ ] Named builder slot: build; core lib (`integral_enum_tests_chirho`); the
  driver STG primitive tests; the mutants above.
- [ ] Combined Char replay: the end-to-end child and the six 004cdbf4 rows go
  green.
- [ ] Separate prerequisite (gpt_chirho): list rendering must fail closed on an
  element error. Trace its smallest placement after this checkpoint.
