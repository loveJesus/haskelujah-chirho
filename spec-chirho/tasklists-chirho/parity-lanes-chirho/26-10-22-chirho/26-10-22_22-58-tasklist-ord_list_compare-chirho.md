<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ord over lists and fieldless enumerations Chirho

Owner: HASKELUJAH/claude_chirho. GPT owns coordination, canonical main and the progress DB.

Branch `ord-list-compare-chirho` starts from the GREEN combined tip 54067f26 (gate combined-54067f26-chirho) and follows gpt_chirho's disposition on #28041 in four commits. There is no Cargo lease.

The trace measured 33 probes on GHC 9.14.1 and the frozen ground and combined CLIs. Six rows were silent:
- sortBy and maximumBy with compare on `[[Int]]`;
- sortBy with compare on Bools;
- sortBy with compare on Orderings;
- a user list instance missing a method, under `map`;
- Chars built with `toEnum` (claude2's lane).

## Source

- [x] Brick 1, 9010717e: the Ordering compare generator moves into `instance_chirho/constructor_orders_chirho.rs`, byte-identical apart from `pub(super)`.
- [x] Brick 2, f93ca45f: the conditional list path's missing-method stub keeps its method's calling convention. Its leading dictionary parameters and their plumbing are removed.
- [x] Brick 3, bccda58f: `instance_chirho/list_orders_chirho.rs` generates `Ord [T]` for Int, Bool, Double and String elements from one recursive `$compareListBy`, which has no default alternative.
- [x] Brick 4, 183a2b55: Bool and Ordering compare by constructor order through one generator, and Ordering's Core is unchanged.
- [x] Every control's output was measured on GHC 9.14.1. On 54067f26 each control crashes, answers wrong, or names its dictionary.

## Gate

The source branch was never gated on 54067f26 (gpt_chirho #28072). After unit 524 and gpt's date shard, `replay_ord_chirho.sh` replayed the five commits onto main 28924e51 as `ord-list-compare-main-chirho` (#28085): 9010717e→40b58ace, f93ca45f→b073cab2, bccda58f→81dca451, 183a2b55→2c80223a, 4372b6d8→dea2a0b7. All 11 replay proofs were GREEN (union sha256 5190493d1fa4949a).

The base is CLI 926ccd4b (claude2_chirho's receipt-2, sha256 4b6fa999) with R2 22e94efd's lists. The first gate on the replay, ord-main-dea2a0b7-chirho, went RED at focused on D03, and that receipt stays as it is. The cause lay outside Ord; [the own-class unit](../26-10-23-chirho/26-10-23_02-50-tasklist-own_class_method_occurrences-chirho.md) on dea2a0b7 repairs it. The landing gate on both, combined-29db427b-chirho (frozen CLI 54f8d6ec), was GREEN in every stage:

- [x] Build, then focused suites: core lib, dictionary_evidence, and the two cd95568b missing-method tests by name.
- [x] Changed-line Clippy since 28924e51, and gitleaks.
- [x] Oracles over list_orders and string_lists. A derived runner reads the GHC-REJECTS marker.
- [x] Matrix: the 33 trace probes plus the controls, all 24 requirements met. Two rows stay wrong exactly as on the base: Chars built with `toEnum` (claude2's lane) and RC5's Orderings.
- [x] Mutations:
  - c1: the EQ arm stops recursing.
  - c2: nil against cons answers GT.
  - c3: the Ord arm is dropped.
  - c4: `[Char]` is dropped from Ord.
  - d1: Bool's order is reversed.
  - a1: the stub's leading dictionaries are restored.
  - s1: `[Char]` is dropped from Eq.
- [x] An Ordering `--dump-core` diff of `$prim_Ord_compare_Ordering`, with ids normalized, between the base CLI and the tip: identical.
- [x] Corpus paired against the base lists, run with the derived `-P 2` runner: membership, replay and execution as on the base.

## Outside this unit

- RC5: a nullary constructor argument dispatches at Int, so `compare LT GT` crashes and `sortBy compare [GT,LT,EQ]` answers wrong.
- RC6: the Ord method model. `<`, `<=`, sort, maximum and max fail loudly on non-scalar types.
- RC7: no `Ord (Maybe a)` dictionary exists.
- The `compare#` family's integer-0 fallback in eval_chirho.rs, after unit 524.
