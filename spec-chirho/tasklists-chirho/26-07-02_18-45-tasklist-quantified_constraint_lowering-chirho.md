<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Quantified Constraint Lowering Chirho

- [x] Reproduce `quantified_constraint_type_sig_parses_chirho` returning a heap pointer.
- [x] Confirm Core emitted `foo :: $Dict_? -> ...` from synthetic placeholder constraint lowering.
- [x] Preserve `forall ...` contexts as `ConstraintChirho::QuantifiedChirho` in `type_to_constraints_chirho`.
- [x] Add a lowerer regression for `(forall a. Show a) => Int`.
- [x] Verify the quantified signature eval test now returns `IntChirho(42)`.
- [x] Recheck the real `constraints` package frontier and document that `Data.Constraint.Forall` still needs higher-kinded quantified-constraint kinding.
