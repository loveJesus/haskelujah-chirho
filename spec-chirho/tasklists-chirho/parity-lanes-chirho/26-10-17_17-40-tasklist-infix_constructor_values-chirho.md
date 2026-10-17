<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Registered infix constructor values Chirho

C1 from Claude2 #27313 is accepted after promoted-list normalization but fails at runtime with an unbound `:::`. Both existing Core infix paths special-case only list colon; user data constructors instead become variable references. This separate local unit preserves their registered constructor identity in both the ordinary and fixity-rebuilt paths. Constructor arity metadata already belongs to DesugarCtxChirho, so this adds no registry, dependency, or module. It touches only the two infix constructor branches, leaving Claude's dictionary key path separate. Builder is GPT; canonical main/DB remain GPT-owned; no remote push or release.

- [x] Trace both Core infix paths and surface the placement before editing (#27327).
- [x] Use the existing constructor registry for constructor applications in both paths.
- [ ] Measure the ordinary constructor baseline on GHC/current main and the repaired CLI; run C1 to exact output 5.
- [ ] Run focused Core/driver controls and catch the missing-constructor mutant.
- [ ] Include the final source in the combined pattern/corpus gate; land locally only with no accept loss.

The prior main does not reach C1 execution because of the promoted-cons representation error. A separate ordinary, non-DataKinds source establishes the infix constructor runtime defect independently. New controls do not require a known wrong result. Prefix higher-order constructor functions, imported constructor runtime registration and pattern-synonym builders remain separately measured boundaries unless the shared repair actually changes them.
