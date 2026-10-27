<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Tuple Eq, Show and Ord at arities 4 to 15 Chirho

## Origin and scope

Row 604.
- **Measured:** with row 603 (#29742); assigned in #29744, second after 603, and cleared by 603's landing (#29817).
- **Scope:**
  - Tuple `Show`, `Eq` and `Ord` instance seeds at arities 4-15.
  - The tuple seed logic is extracted from `class_chirho.rs`, which held 4051 lines, rather than added to it.
  - The structured Eq (row 603) and Ord (row 572) rows widen to the same arity.
- **Branch:** `tuple-instances-chirho`, from main `c3876584`, replayed onto `6cb03a59` (row 607).

## Measured (GHC 9.14.1, /opt/homebrew/bin/ghc -O0)

- **Probes:** `tmp-chirho/tuple-instances-chirho/probes-chirho/sources-chirho`, with GHC outputs in `ghc-chirho`.
- **Main `c3876584`, loud on every engine:**
  - `print`/`show` of a 4- to 15-tuple fails type checking (`no instance for Show (Int, Bool, Char, [Char])`);
  - `==`/`compare` at arity 4 and above fails type checking, or (`TEqOrd4`) fails with `missing binding '=='`.
- **An earlier measurement-only prototype** (`proto1-seeds-chirho.patch`) seeded arities 4-15. With that alone, `Show` matched GHC on all three engines: the derived-Show generator was already arity-generic.
- **The candidate: all 11 programs match GHC on all three engines (33/33).**
  - `Show` at 5-8 and 15, `show` of a nested 6-tuple, negative and nested fields.
  - `==`, `compare`, `<` and `max` at 3, 4, 6 and 15.
  - Row 603's `elem`, `lookup` and `nub` on 4-tuples.
- **The bound:** GHC rejects `print` of a 16-tuple (`No instance for 'Show …'`), and so does the candidate.

## Placement

- **Typing:**
  - The new `class_chirho/tuple_instances_chirho.rs` holds `seed_tuple_instances_chirho` and `MAX_TUPLE_INSTANCE_ARITY_CHIRHO = 15`, re-exported from `class_chirho`.
  - The pair and triple seeds left `class_chirho.rs`, which falls from 4051 to 3968 lines. `b_var_chirho` stays, for the `Either` seeds.
  - The structural test is `class_chirho/tuple_instances_tests_chirho.rs`.
- **Dictionary pass:** `instance_chirho/structured_orders_chirho.rs` and `structured_equalities_chirho.rs` compose tuples up to the shared bound.
- **Controls:** `tests/dictionary_evidence_chirho/ordering_chirho/tuple_instances_chirho.rs`. `ordering_chirho/` now has 7 entries.
- **Workflow:** `language-features-chirho/dictionary-evidence-chirho.md`, after row 603's bullet.

## Checklist

- [x] Measure on GHC and main; prototype the seeds (measurement only).
- [x] The typing module, the arity bound, and the widened Eq/Ord rows. The candidate matches GHC on all 11 programs, and the 16-tuple bound holds.
- [x] Controls: GHC-measured, byte for byte; the structural and all-engine tests pass beside row 603's.
- [x] The typing and core libraries pass 558/558 on the candidate.
- [x] **Broad moved-shape scan** (`scan604-chirho.log`): 3154/3154 passed in 550 s, with no mover. It left out the 16 `proptest_chirho::eval` properties and `ghc_bulk`.
- [x] **Reach** (`tools-chirho/reach_604_chirho.zsh`, log `reach604-r1-chirho.log`):
  - **M1, the seeds stop at triples:** the structural test and both all-engine controls fail.
  - **M2, the Ord rows stop at triples:** the comparison control fails.
  - **M3, the Eq rows stop at triples:** the comparison control fails.
  - Every restore was byte-identical, and the restored tree passes.
- [x] **A gate gap, found and checked:** row 603's gate script (from row 597's) names no `haskelujah-typing` in its test or Clippy steps.
  - Row 604's gate adds typing to both.
  - A separate Clippy of typing finds no diagnostic on the 1343 lines changed since `0aba3695`, covering rows 603 and 604 (`clippy-typing-603-604-chirho.log`).
- [x] Commit; replay onto the newest main; judge main against the contract copy; warm gate; record.
  - The source `ec60dfb6` sits on main `c2a92a17`, which adds only the row 608 reservation to the progress DB.
  - **Main `c2a92a17`** was GREEN against the contract copy first (73 movers, 9 holds), and no cell of the 11 programs matched GHC there (`baseline-c2a92a17-chirho.log`).
  - **r1, GREEN on `ec60dfb6` (base `c2a92a17`) in 231.27 s:**
    - 1272/1272 focused tests, with the typing library now in the gate;
    - no Clippy diagnostic on the 194 changed lines in 7 files, with typing linted;
    - the contract judge GREEN;
    - rustfmt, diff-check and gitleaks clean (1 commit).
- [ ] Land on main and close progress row 604 (gpt).
