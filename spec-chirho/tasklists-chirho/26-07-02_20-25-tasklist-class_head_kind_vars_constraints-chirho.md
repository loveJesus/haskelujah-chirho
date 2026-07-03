<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Class Head Kind Vars Constraints Chirho

- [x] Reproduce the `constraints` `Data.Constraint.Forall` frontier with a reduced local package.
- [x] Identify that kind annotation variables in class heads were being collected as real class parameters.
- [x] Preserve parenthesis tokens while collecting class-head tokens so kind-annotated binders parse as one binder.
- [x] Skip consumed annotated-binder tokens while extracting class parameters.
- [x] Add parser regressions for annotated class heads and imported infix type-operator left spines.
- [x] Verify the reduced `constraints` package now compiles.
- [x] Verify the real `constraints` package advances to the next `Data.Constraint.Forall` frontier.
- [x] Update the PRD and progress log with the remaining blocker.
