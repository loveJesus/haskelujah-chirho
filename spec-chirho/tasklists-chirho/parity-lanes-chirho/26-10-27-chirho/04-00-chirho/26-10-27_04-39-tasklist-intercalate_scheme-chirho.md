<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# `intercalate` at GHC's type Chirho

Row 613 (gpt_chirho #29980; checkpoint #30018; ruling #30022). The bare scheme of `intercalate`
is `String -> [String] -> String`, where GHC 9.14.1's Data.List gives `[a] -> [[a]] -> [a]`.

**Status: HELD and source-free (#30022).** The lane runs rows 615, 616, then 613.

## Trace (#30018)

- **Scheme:** `part_03_chirho.rs` seeds the mono String scheme. No module key exists for
  intercalate in Data.List, Data.Text or ByteString.
- **Body:** one generated Core binding (row 579, `report_text_chirho.rs`). It is a list case
  plus the generated polymorphic lazy `append`, so it is representation-generic.
  - Since row 579, nothing produces `intercalateStr#`.
  - Only the binding's and binders' type annotations say `String`.
- **Text and ByteString:** no scheme or body of their own.
  - `T.`, `TL.`, `B.`, `BS.` and `BL.intercalate` resolve by bare name to that one binding.
  - Qualified, they keep the `forall a. a` placeholder.
  - Base: on Cranelift, `T.unpack` of the list-built result prints `""`.
- **Scope:** every builtin name is in scope with no import (`nub` checks), and unqualified
  imports never consult module keys.

## Measured prototype (scratch, reverted)

The prototype widened the scheme and retyped the body to `[a]`.

- **Transition, 25 programs, interpreter, LLVM and Cranelift:**
  - 17 GAIN, 7 RED and 5 CHANGED (loud stays loud);
  - 9 cells stay rejected with new text, and 37 hold;
  - no signal or timeout.
- **RED A:** n2 (no import) and n10 (`hiding`) go from rejected to accepted on all three engines,
  where GHC rejects them (GHC-88464).
- **RED B:** t6, valid Data.List plus Data.Text, compiles, and Cranelift prints its Text line as
  `""`.
- **Row 608 delta, measured through the committed generator:**
  - +1 entry, (Data.List, intercalate), and -0;
  - agree 727 and disagree 678.
- **Row 608's battery:** 2 GAIN (`r1_list_alias_prints` natively) and 175 holds.
- **Corpus selector:** 1 file (RuleEqs.hs), 0 moved.
- **Tests:** the only failure is row 608's `list_prints_stop_only_at_the_disagreeing_intercalate_chirho`.
  Its `L.intercalate` line now prints natively.
- **Receipts:** the probe runner and every result, in the worktree's ignored
  `tmp-chirho/r613-trace-receipts-chirho/`.

## Residual findings (recorded, not absorbed: #30022)

- **Arithmetic-sequence evidence loss:** `print (zip "ab" (map negate [1 .. 3 :: Int]))` fails
  with "missing STG binding `fromInteger`" on every engine.
  - It hits any two-argument call whose second argument maps over an arithmetic sequence.
  - A literal list, an explicit `enumFromTo` and a `let`-bound sequence are exact.
- **Num Char accepted:** `putStrLn (intercalate [1] ["a"])` checks and prints `a` on every
  engine; GHC rejects it (GHC-39999).
- **Unqualified Data.Map imports:** `import Data.Map (fromList, insert, toList)` fails at run time
  ("no matching alternative for tag 0"). The qualified `M.` spelling is exact.

## Checklist

- [x] Trace and checkpoint (#30018).
- [x] Prototype measured and reverted; the branch stayed source-free.
- [ ] Row 615 (builtin scope) closed.
- [ ] Row 616 (qualified Data.Text route) closed.
- [ ] Resume: regenerate row 608, rerun the entire transition table, then a new checkpoint.
