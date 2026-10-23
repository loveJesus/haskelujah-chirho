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
- **fromEnum** is the identity at the 11 keys whose every value fits Int:
  Int8, Int16, Int32, Int64, Word8, Word16, Word32, CChar, CInt, CLong and
  CUInt. The five wide keys keep the missing-method stub: Word, Word64,
  Natural, CULong and CSize (revision 1).
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
- `every_integral_dictionary_takes_real_rows_chirho`: 27 `$fEnumK` slots hold
  body-backed rows, never a missing-method stub. That is the 16 `toEnum` slots
  and the 11 narrow `fromEnum` slots.
- `wide_from_enum_slots_stay_missing_method_stubs_chirho`: the five wide
  `fromEnum` slots (Word, Word64, Natural, CULong, CSize) stay generated
  missing-method bodies naming `Enum.fromEnum` and the key.
- `from_enum_rows_are_the_identity_chirho`: the 11 narrow keys.
- `to_enum_rows_guard_exactly_the_table_bounds_chirho`: the guards and their
  order.
- `each_row_raises_ghcs_measured_message_chirho`: the table holds exactly the 16
  measured keys; 23 measured failures render GHC's exact text; 23 measured
  in-range tags pass every guard.
- `a_computed_error_message_fails_with_its_exact_text_chirho` (STG): the rows'
  `showInt#`/`++#`/`error` chain raises its exact text on held.

**On the combined Char replay** (the six rows already at 004cdbf4, plus a
pathed child generated from the measured files; revised by revision 1):
- the six never-wrong `toEnum` rows, still all green because every `toEnum` row
  stays;
- 11 exact scalar round trips `print (fromEnum (toEnum 7 :: K))` at the narrow
  keys, expecting 7;
- five never-wrong wide controls at Word, Word64, Natural, CULong and CSize:
  GHC's answer or a loud `missing method Enum.fromEnum for K`;
- the signed negatives, expecting -1;
- the 23 `toEnum` failures, which must carry GHC's message;
- () as a green never-wrong control.

Float is a measured comparison only, not a test.

**Mutants** must terminate, and each must fail a named control:

| Mutant | Change | Must fail |
|---|---|---|
| MU1 | drop the call in `generate_enum_bounded_prelude_chirho` | `every_integral_dictionary_takes_real_rows_chirho` (stubs return) |
| MU2 | swap the order of the lower and upper guards | `to_enum_rows_guard_exactly_the_table_bounds_chirho` |
| MU3 | off-by-one: the upper guard uses `>=` | `to_enum_rows_guard_exactly_the_table_bounds_chirho` (op `>=#`), and on the combined replay the in-range bound rows |
| MU4 | name the C key instead of its representation (`{CInt}`) | `each_row_raises_ghcs_measured_message_chirho` |
| MU5 | generate an identity `fromEnum` for a wide key (Word64) | `wide_from_enum_slots_stay_missing_method_stubs_chirho` |

These were predictions until build gate 2, which ran all five: each failed its
named control (see below).

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

## Revision 1 (gpt_chirho's disposition of 4b98964b): wide fromEnum stays a stub

4b98964b is kept as the honest first checkpoint. This revision is a new commit
on top of it.

**The contract blocker was the unconditional `fromEnum` identity.** Measured on
GHC 9.14.1 by gpt_chirho (`/private/tmp/integral-enum-fromenum-audit-chirho/`;
its first exit ledgers were invalid and were re-run, while stdout and stderr
were valid):
- `fromEnum (maxBound :: Int64)` gives 9223372036854775807;
- Word32 and CUInt maxima give 4294967295;
- `fromEnum (maxBound :: Word64)` fails, and so do Word, Word64, CULong and CSize
  at `9223372036854775808`, with `Enum.fromEnum{Word or Word64}: value (...) is
  outside of Int's bounds (...)`;
- Natural at `9223372036854775808` fails with `fromEnum: out of Int range`.

Held fails every one loudly through its missing method.

**Source limit.** `parse_integer_literal_chirho` (parser `lower_chirho.rs`)
returns i64 and gives `unwrap_or(0)` for an out-of-range literal, and the AST,
Core and runtime ints are i64. A value above `i64::MAX` cannot keep its
identity, so a wide `fromEnum` identity would answer wrongly for any future
producer (Num, Read, FFI).

**Changes:**
- `IntegralKeyChirho` gains `identity_from_enum_chirho`. It is true for the 11
  keys whose every value fits Int and false for Word, Word64, Natural, CULong
  and CSize. The generator emits no `fromEnum` row for a wide key, so its
  dictionary keeps the generated missing-method body.
- All 16 checked `toEnum` rows are unchanged.
- Core controls: 16 body-backed `toEnum` slots plus 11 body-backed `fromEnum`
  slots, 27 in all. The new `wide_from_enum_slots_stay_missing_method_stubs_chirho`
  requires the five wide slots to stay generated missing-method bodies naming
  `Enum.fromEnum` and the key, and requires the wide set to be exactly those
  five keys. The identity test covers only the 11 narrow keys.
- The workflow's boundary is corrected. The earlier sentence "unreachable
  because toEnum cannot construct it" was too narrow: a row serves every value
  producer.

## Build gate 1 (gpt_chirho, ad96f6d5): RED only at changed-line Clippy; revision 2

**What passed:**
- The build is warning-free.
- core lib 138/138, including all five integral Enum tests.
- The STG computed-message control, 1/1. An earlier `--exact` filter that ran 0
  tests is retained as invalid.

**What failed:** changed-line Clippy reported a new `clippy::enum_variant_names`
at `integral_enum_chirho.rs:37`. Every `OutOfRangeChirho` variant ended in
`Chirho`, which the naming rule requires, and no suppression is allowed.

**Revision 2 (source only):** the unchecked variant is folded into
`BoundsChirho` with no bounds.
- Int64 and CLong are keys an Int tag cannot leave, so `int_sized_chirho` gives
  them `lower_chirho: None, upper_chirho: None` and GHC's Int64 range text. No
  guard ever renders that text.
- The enum keeps two variants, `BoundsChirho` and `NegativeChirho`, below
  Clippy's variant-name threshold of three.
- The generated Core is unchanged: a key with no bound yields the tag without a
  guard, as before.
- The guard test's expectation for Int64 and CLong is still no guards, now
  through the bounds arm.

**Doc correction:** the Controls list above said all 32 slots are real. It now
says 27 real slots plus the five wide stubs, matching the source and revision 1.

## Build gate 2 (claude2_chirho, 6f2bc8c4): GREEN at unit scope

Run on 2026-10-22 in the named builder slot that gpt_chirho handed over, and
posted as #28043. Cargo ran with `-j2` under `nice` and `timeout`, and tests
used two threads. Gate 1's receipt in gpt_chirho's
integral-enum worktree is untouched: its 9-file SHA-256 manifest is the same
before and after.

- **Build:** `cargo build -j2 --bin haskelujah`, rc 0, with no warning or error
  line. The frozen CLI is
  `2d25202253a2327ee5521f86608697a95bab55d7a8cdb51f8a3b8d6c4accfded`.
- **Core lib:** 138/138. All five integral Enum tests passed, each listed by its
  full name.
- **STG control:** run with `--exact` and its full path, it printed `running 1
  test` and `1 passed; 1847 filtered out`.
- **Changed-line Clippy against 0f256e43:** core and driver with
  `--all-targets`, so both touched test modules were linted. No diagnostic falls
  on a changed line. The three diagnostics in touched files sit on unchanged
  `prelude_chirho.rs` lines that gate 1 also reported. Gate 1's
  `enum_variant_names` is gone.
- **gitleaks** over `0f256e43..6f2bc8c4`: 3 commits, no leaks.
- **Mutants:** five valid kills. One command ran the baseline (5/5 on the exact
  source), every mutant and the restored source. Each mutant recompiled core,
  ended with rc 101 and no timeout, and failed its named control on an `assert`
  line:
  - MU1: `every_integral_dictionary_takes_real_rows_chirho`;
  - MU2 and MU3: `to_enum_rows_guard_exactly_the_table_bounds_chirho`;
  - MU4: `each_row_raises_ghcs_measured_message_chirho`;
  - MU5: `wide_from_enum_slots_stay_missing_method_stubs_chirho`.

  Every restore was byte-exact by SHA-256 with a clean `git diff`, and the
  restored source passed 5/5.
- **Corpus pair** on the frozen CLI: two passes per axis, byte-identical to each
  other and to held's passes (CLI ba1e5c29, 0f256e43's compiler).
  should_compile is 885 ACCEPT and 53 REJECT; should_fail is 521 ACCEPT and 246
  REJECT. All 1705 sources match held's, and every raw output matches once the
  worktree root is masked.
- **Replay:** all 299 rejections are ordinary diagnostics.
- **Execution pair:** `execution_chirho.sh` on held and on the candidate, over
  14 programs and 7 rejection predicates, against runghc 9.14.1. Both score
  28/35 with the same seven known failures, which is unit 519's preserved
  verdict set. The three outputs that still differ after masking are inside
  those failures: a heap id, an LLVM value number, and a pointer that changes on
  every run.
- **Postflight:** HEAD, the tree and both frozen CLIs are unchanged, and no
  builder process is left.

**Declared deviation.** The corpus passes used a copy of the committed
`axis_pass_chirho.sh` with two workers instead of four, to stay within the
lease's two-thread limit. The worker, the verdict rule, the sort and the
TIMEOUT re-run are the committed files.

**Scope.** `haskelujah check` runs only the front end, so for this unit the
corpus pair is an identity control. The execution pair is the stage that
reaches the new rows. End-to-end evidence dispatch still waits for the 523+520
combined gate: the combined child and the six 004cdbf4 rows run only on the
combined Char replay.

Receipts are in `tmp-chirho/integral-enum-gate2-1022-chirho/` in claude2_chirho's
worktree.

## Checklist

- [x] Design measured and accepted (#27673; gpt_chirho's disposition).
- [x] Source checkpoint 4b98964b: the rows, the core controls and the STG
  control are rustfmt-clean, and the measured tables are generated from the
  receipts.
- [x] Revision 1, source only: the wide `fromEnum` stays a stub (11 identity
  rows); the controls, workflow and tasklist are revised.
- [x] Build gate 1 (gpt_chirho, ad96f6d5): build, core lib 138/138 and the STG
  control are green; changed-line Clippy is RED (`enum_variant_names`).
- [x] Revision 2, source only: the structural Clippy fix and the doc correction.
- [x] Re-gate at a named slot: changed-line Clippy, core lib, the STG control and
  the mutants above (build gate 2, 6f2bc8c4, GREEN at unit scope; #28043).
- [ ] Combined Char replay: the end-to-end child and the six 004cdbf4 rows go
  green.
- [ ] Separate prerequisite (gpt_chirho): list rendering must fail closed on an
  element error. Trace its smallest placement after this checkpoint.
