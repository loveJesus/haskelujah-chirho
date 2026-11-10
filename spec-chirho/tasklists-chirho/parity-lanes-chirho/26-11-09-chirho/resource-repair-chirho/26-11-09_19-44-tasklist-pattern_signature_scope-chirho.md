<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Scoped pattern-signature producer transport

Owner: HASKELUJAH/gpt_chirho, sole implementation and progress-DB writer. Base is private c5f5839480960d1db9bc3d6b0812ab4c360b50cf. This continues the active drop-in GHC objective after the measured fundep closure; report review creates no development hold.

- [x] Execute GHC 9.14.1 references for unchanged original tc198 and T11506, plus the rejecting ExPatFail control, on original paths with original pragmas and no all.T flags. Both positives ACCEPT; ExPatFail REJECTs with GHC-25897. Keep every source/tool hash and raw outcome.
- [ ] Trace the first missing checked owner/kind from pattern annotation to its equation body, then the inference scoping handoff. Publish one coherent producer account; never turn an absent kind into Type or scope a variable by spelling alone.
- [ ] Record exact source homes and create a checkpoint/isolated source branch before authoring. Reuse the existing scope and checked-variable carriers where sound; name any necessary carrier extension before editing it.
- [ ] Implement the shared correction with positive equation-body scope and existential negative controls. Explicit forall nesting, pattern bindings, sibling equations and lexical exit must retain their distinct boundaries. Real original frontend behavior is the primary gate.
- [ ] Run focused checks during iteration, then the final workspace, affected whole typing, preserved runtime handoff, original cohort and cumulative lint gates appropriate to the final change. Preserve the frozen c5 executable and both full corpus datasets.
- [ ] Commit explicit owned paths, scan, push/read back, record actual movement and remaining limits. No main compiler or public release follows from a focused pass.

Initial source reading, not yet a causal trace: values_chirho::pattern_chirho checks TypeAnnotChirho through check_value_type_chirho and visits its inner pattern without installing the new pattern-signature variables over the equation body. Inference's TypeAnnotChirho branch converts and subsumes the annotation but does not register those new variables in scoped_tyvars_chirho. Current corpus tc198 and T11506 first diagnostics report a missing bounded producer kind. The annotation kind-checking path and the later inference/body-kind recheck must be distinguished by the actual call trace before choosing the correction; the diagnostic alone does not prove which phase first loses the account.

Primary reference: https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/exts/scoped_type_variables.html, section 6.11.5.4. Ordinary pattern signatures can introduce previously unscoped variables over their equation's body; pattern bindings have a stricter rule. Existing scoped variables constrain the pattern instead of being rebound. The producer must preserve these distinctions, not blindly quantify or import every annotation name.

Evidence home: tmp-chirho/pattern-signature-scope-chirho under the existing private worktree, grouped into reference, before, observation, after and quality leaves. No new worktree or independent writer is introduced.
