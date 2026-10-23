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

- [ ] Build, then focused suites: core lib, dictionary_evidence, and the two cd95568b missing-method tests by name.
- [ ] Changed-line Clippy since 54067f26, and gitleaks.
- [ ] Oracles over list_orders and string_lists. A derived runner reads the GHC-REJECTS marker.
- [ ] Matrix: the 33 trace probes plus the controls.
- [ ] Mutations:
  - c1: the EQ arm stops recursing.
  - c2: nil against cons answers GT.
  - c3: the Ord arm is dropped.
  - c4: `[Char]` is dropped from Ord.
  - d1: Bool's order is reversed.
  - a1: the stub's leading dictionaries are restored.
- [ ] An Ordering `--dump-core` diff of `$prim_Ord_compare_Ordering`, with ids normalized, between ea03667e and the tip.
- [ ] Corpus paired against 54067f26's lists, run with the derived `-P 2` runner.

## Outside this unit

- RC5: a nullary constructor argument dispatches at Int, so `compare LT GT` crashes and `sortBy compare [GT,LT,EQ]` answers wrong.
- RC6: the Ord method model. `<`, `<=`, sort, maximum and max fail loudly on non-scalar types.
- RC7: no `Ord (Maybe a)` dictionary exists.
- The `compare#` family's integer-0 fallback in eval_chirho.rs, after unit 524.
