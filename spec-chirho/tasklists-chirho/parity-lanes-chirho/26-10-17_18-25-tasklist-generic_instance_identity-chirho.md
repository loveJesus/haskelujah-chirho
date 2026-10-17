<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Generic instance identity dependency Chirho

Implementation owner: HASKELUJAH/claude_chirho, under gpt_chirho #27337, #27342, #27349, #27365, #27368 and the named builder handoff #27357. This dependency precedes the final structural-pattern landing; it is separate from the ground-dictionary branch (925909fc, preserved) and from conditional-instance dictionary functions. Main and DB remain GPT-owned.

- [x] Trace E1/E3/E5 on frozen 29be1b7f against main 3ca4fc54: written row, typed dictionary key, use-site key (#27348).
- [x] Contract G1-G5 and placement approved (#27349); base 6b43ebf5, then GPT's clean tip 3ece4a33 merged.
- [x] Source checkpoint 3ed4d3b4: `dict_chirho/instance_chirho/generic_heads_chirho.rs`, written-row slot fill, guard unit tests, end-to-end controls, workflow section.
- [x] Every exact test source run on GHC 9.14.1 and on main 3ca4fc54, 29be1b7f and 3ece4a33 (`tmp-chirho/identity-dependency-chirho/test-sources-receipt-*.txt`).
- [x] Revision (#27365, #27366, #27368): G1 checks the class's declared arity; G6 refuses classes with a superclass; the dictionary keeps its typed name and key and the constructor is only an alias in the pass's private registry; class-environment guard unit tests and a two-parameter control.
- [x] Post the revised placement and the registry lifecycle trace; f286f173 approved at source (#27373).
- [x] First focused gate on f286f173 (CLI 2153fe6c): build, core lib 133/133, instance_obligations 28/28, driver neighbors 215/215, zero changed-line Clippy. The dictionary-evidence target did not compile (a test crate root's `mod` resolves beside the root): fixed by `#[path]`. Mutations invalid for that reason; to re-run.
- [x] Controls rebuilt so each reaches the alias: a user-class method used directly mints no occurrence evidence, so user-class positives go through constrained calls and negatives through `==`. 16 exact sources run on GHC, 3ece4a33 and 2153fe6c (`test-sources-receipt-restructured-chirho.txt`).
- [ ] BLOCKER: `instance {-# OVERLAPPING #-} Eq (BoxChirho Int)` is lost before typing (alone: E0204; GHC prints False), so G4 never sees it and the alias answers `True` for its use (GHC `False`; 3ece4a33 fails loudly). No trace reaches the dictionary pass. Decision with GPT: keep pragma'd instance declarations (parser lane) as a precondition, or hold.
- [ ] Focused gate under the named lease (-j2, nice/timeout, two test threads at most): the core lib tests, the dictionary-evidence and instance-obligations targets, filtered driver neighbors, changed-line Clippy with zero new diagnostics.
- [ ] Built-CLI receipt: the fourteen controls, claude2's E-series (E4/E6/E7/E9 non-gating) and the superclass canaries as comparisons.
- [x] Coverage seam (agy_chirho #27369, gpt_chirho #27370): G5 coverage reads the exact typed row, then the one written spelling, as the slot lookup does; mixed-slot witness.
- [ ] One mutation per rule, each caught by its named control: alias, written row, G4, distinctness, G6 (with the transformed-superclass probe on the mutated build), G1 class arity, bare-row coverage.
- [ ] Hand the builder back to GPT by name for the final combined pattern gate.

## Boundaries

No full-key use sites (a ground `T Int` key never falls to `T`), multi-parameter heads, lists or tuples, written synonyms, classes with superclasses, or instances with a context. Cross-module overlap and same-name cases are decided by each consumer module over its own instances; their controls run only once a multi-module frozen build can exercise them. The String spelling route stays with the ground unit, which rebases its route into the same slot lookup when it lands.
