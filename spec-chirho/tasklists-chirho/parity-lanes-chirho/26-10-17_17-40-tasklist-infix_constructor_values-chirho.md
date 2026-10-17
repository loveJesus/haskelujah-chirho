<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Registered infix constructor values Chirho

C1 from Claude2 #27313 is accepted after promoted-list normalization but fails at runtime with an unbound `:::`. Both existing Core infix paths special-case only list colon; user data constructors instead become variable references. This separate local unit preserves their registered constructor identity in both the ordinary and fixity-rebuilt paths. Constructor arity metadata already belongs to DesugarCtxChirho, so this adds no registry, dependency, or module. It touches only the two infix constructor branches, leaving Claude's dictionary key path separate. Builder is GPT; canonical main/DB remain GPT-owned; no remote push or release.

- [x] Trace both Core infix paths and surface the placement before editing (#27327).
- [x] Use the existing constructor registry for constructor applications in both paths.
- [ ] Measure the ordinary constructor baseline on GHC/current main and the repaired CLI; run C1 to exact output 5.
- [ ] Run focused Core/driver controls and catch the missing-constructor mutant.
- [ ] Include the final source in the combined pattern/corpus gate; land locally only with no accept loss.

The prior main does not reach C1 execution because of the promoted-cons representation error. A separate ordinary, non-DataKinds source establishes the infix constructor runtime defect independently. New controls do not require a known wrong result. Prefix higher-order constructor functions, imported constructor runtime registration and pattern-synonym builders remain separately measured boundaries unless the shared repair actually changes them.

The first registry-only revision 8fd832b1 fixes the ordinary constructor chain but not C1: collect_con_strictness_chirho explicitly skips GADT declarations, so C1's constructor is absent from the Core arity table. The follow-up recognizes colon-led constructor operators lexically in both paths; the registry still covers named backtick constructors. On the combined dirty-source CLI 226e6ccc, fresh GHC/main/candidate executions match C1 (5), the ordinary left-associative tree (123), KR09 and KR09b (imported :| and :+), plus the unchanged non-infix KR09c. The embedded imported-constructor control now requires its exact output. This is a measured execution result, not yet a corpus, mutation, native-backend or landing claim. The arity-table omission for GADTs remains a metadata boundary outside these saturated binary operator applications.
