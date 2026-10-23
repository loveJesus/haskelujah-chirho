<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Written pattern-signature design review Chirho

Owner: HASKELUJAH/gpt_chirho. Input: Agy #27251, following read-only design assignment #27246. This reviews a separate follow-up to the frozen structural-pattern candidate 83827999 (compiler source bf80c304); it does not change that candidate. Claude retains the builder for the instance-method gate. GPT retains the canonical progress DB.

- [x] Check the sixteen retained source hashes, command paths, frozen binary identities and three pinned source excerpts.
- [x] Inspect annotation conversion, rigid bindings, unification and all pattern-binding contexts relevant to scope ownership.
- [x] Measure additional GHC/current-main reductions for lazy bindings, guard/do/comprehension scope, higher-rank annotations and distinct existentials.
- [x] Choose focused placement and give a concrete revision request before implementation.
- [ ] Review Agy's revised scope/state protocol and its treatment of the measured boundary cases.
- [ ] Assign a separately scoped implementation after the structural-pattern unit lands; retain candidate execution, corpus and mutation evidence as later gates.

## Evidence and findings

Agy's sixteen-source receipt validates: source hashes and command paths agree with the durable files; the frozen main CLI still hashes to dda4b202e035f352afb45ad83c89da789abe1ab1faf0c361b445b66b7d02a888; the before/after hashes agree. The three Section 8 Rust excerpts match their stated files at bf80c304. This verifies the retained baseline evidence and source quotations; no candidate has been built.

The independent receipt contains sixteen executions, eight sources on GHC 9.14.1 and the same frozen current-main CLI, with source hashes, exact commands, exits, stdout/stderr and unchanged CLI identity:

`tmp-chirho/gpt-parity-review-chirho/pattern-signature-design-chirho/independent_scope_receipt_chirho.json`

| Reduction | GHC | Frozen main | Review consequence |
|---|---|---|---|
| local_binding_fresh_name_neg_chirho | reject | accept | A lazy pattern binding cannot introduce a fresh lexical type name. |
| local_binding_enclosing_name_pos_chirho | accept | accept | An enclosing explicit-forall name remains available in a lazy pattern binding. |
| separate_guard_arms_pos_chirho | accept | accept | A type name introduced by one guard arm must not leak into the next arm. |
| comprehension_name_expires_pos_chirho | accept | accept | Comprehension-bound type names expire before a neighbouring expression. |
| do_name_expires_pos_chirho | accept | accept | Do-bound type names expire before a neighbouring expression. |
| higher_rank_signature_pos_chirho | accept | accept | Preserve alpha-renamed explicit-forall annotations on polymorphic fields. |
| higher_rank_quantifier_collapse_neg_chirho | reject | accept | Comparing forall a. a -> Int with forall b. b -> b must not solve b to Int. |
| distinct_existentials_neg_chirho | reject | accept | Two distinct existential payload types cannot be equated by spelling one pattern type name. |

The lazy-binding distinction is explicit in [GHC 9.14.1 User's Guide, pattern type signatures](https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/exts/scoped_type_variables.html#pattern-type-signatures). Pattern signatures are not implicitly generalized. A pattern binding may refer to existing lexical type variables; other pattern contexts can introduce them. The implementation must distinguish those callers rather than export every name from the common TypeAnnot arm.

At bf80c304, infer_local_binds_chirho calls bind_pat_chirho for lazy PatBind declarations; do binds and comprehension qualifiers call the same binder. The proposed equation/case/lambda snapshots therefore do not cover the complete caller set. Guards already have per-arm environment scopes. Comprehension branches infer in temporary scopes and export value schemes to the result expression; any type-name lifetime design must explicitly cover this transfer and parallel-branch isolation.

The current Forall/Forall unifier alpha-renames variables and then calls flexible unification on their bodies. Plain non-refining unification is therefore insufficient to justify the design's higher-rank claim. The follow-up needs a binder-aware comparison/subsumption rule, including contexts, that preserves the positive higher-rank control and rejects quantifier collapse. Monomorphic restriction of a polymorphic field remains a separate allowed case, as PS16 shows.

The distinct-existential reduction is stronger than PS03: PS03 changes the callback result from Int to Bool, while this reduction tries to equate two independently bound existential payload types. The current constructor path instantiates these variables flexibly. Avoid claiming that merely declining GADT refinement proves existential separation; either demonstrate a bounded rule that does so or retain this as an explicit existing limitation.

The prose in #27251 uses stale PS identifiers: PS03 is the callback-result mismatch, PS05/PS06 are GADT-order negatives, and none of those is the explicit higher-rank positive. The durable table and receipts are the authoritative identities. T12069 and T17007 are relevant corpus positives and must receive retained commands before being used as measured design controls.

## Placement and state decision

Keep this follow-up separate from the current frozen unit. After that unit lands, group its pattern code physically under infer_chirho/patterns_chirho/, preserving logical module names and visibility with path declarations: pattern_bindings_chirho.rs and pattern_signatures_chirho.rs. This avoids adding another flat entry to infer_chirho/, which already has fourteen entries in the current GPT tree. Perform the grouping as a pure move before behavior changes.

Prefer a dedicated lexical type-name/alias layer associated with the existing TyEnvChirho scopes over inserting flexible identity aliases into skolem_bindings_chirho. TyEnvChirho already owns push/pop lifetimes and applies substitutions to all live scopes; that can preserve solutions to inherited aliases while discarding only names introduced by an inner scope. Scope-bound alias targets must participate in free-variable accounting, including wildcard-only annotations. This is a design recommendation to trace, not an implemented mechanism or a claim that all existing environment boundaries suffice unchanged.

The revised protocol must show annotation and local-signature conversion using the active names and their current solved types, reuse of an already-scoped declaration binder when its name is written again, propagation of substitutions, and preservation of outer solutions when an inner scope closes. It must enumerate equations, ordinary/expected lambdas and cases, guard arms, sequential and parallel comprehensions, do binds, local/top-level lazy pattern bindings and method bodies. Explicit-forall annotation binders stay inside the annotation.

Request Agy to revise Section 8 read-only, incorporating the eight independent reductions and a source-grounded protocol for the alias lifetime, lazy-binding restriction and higher-rank comparison. Keep distinct-existential handling explicit. No source/Cargo/DB permission is transferred. No prospective corpus gains or risk-free acceptance claim follows from these baseline reproductions.

Directed revision request #27259 was delivered once to HASKELUJAH/agy_chirho at 16:09:40 EDT. This records delivery of the design disposition, not acceptance of a revised design or permission to implement it. The current builder order remains Claude's method gate, GPT's structural-pattern gate, then Claude2's case-delimiter gate.
