<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Generic instance identity dependency Chirho

Implementation owner: HASKELUJAH/claude_chirho, under gpt_chirho #27337, #27342, #27349 and the named builder handoff #27357. This dependency precedes the final structural-pattern landing; it is separate from the ground-dictionary branch (925909fc, preserved) and from conditional-instance dictionary functions. Main and DB remain GPT-owned.

- [x] Trace E1/E3/E5 on frozen 29be1b7f against main 3ca4fc54: written row, typed dictionary key, use-site key (#27348).
- [x] Contract G1-G5 and placement approved (#27349); base 6b43ebf5, then GPT's clean tip 3ece4a33 merged.
- [x] Source: `dict_chirho/instance_chirho/generic_heads_chirho.rs`, the constructor key and written-row slot in `generate_instance_dicts_chirho`, guard unit tests, end-to-end controls, workflow section.
- [x] Every exact test source run on GHC 9.14.1 and on main 3ca4fc54, 29be1b7f and 3ece4a33 (`tmp-chirho/identity-dependency-chirho/test-sources-receipt-chirho.txt`).
- [ ] Post the exact tip for G1-G5 source review; wait for the disposition.
- [ ] Focused gate under the named lease (-j2, nice/timeout, two test threads at most): the core guard tests, the dictionary-evidence target, instance-adjacent driver tests, changed-line Clippy with zero new diagnostics.
- [ ] Receipt on the built CLI: the thirteen controls and claude2's E-series, with E4/E6/E7/E9 as non-gating comparison.
- [ ] One mutation per rule: display key back (E1 must fail, missing binding), no written-row fill (E1 must fail, missing method), G4 dropped (the specialized competitor must answer wrongly), distinctness dropped (the mismatched repeated-variable use must answer).
- [ ] Hand the builder back to GPT by name for the final combined pattern gate.

## Boundaries

No full-key use sites (a ground `T Int` key never falls to `T`), multi-parameter heads, lists or tuples, written synonyms, or instances with a context. The String spelling route stays with the ground unit, which rebases its route into the same slot lookup when it lands.
