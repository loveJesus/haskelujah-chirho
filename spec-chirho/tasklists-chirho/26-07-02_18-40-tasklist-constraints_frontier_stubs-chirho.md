<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Constraints Frontier Stubs Chirho

- [x] Claim the single-builder slot for the `frontend_constraints_package_regression_chirho` lane.
- [x] Reproduce the current `constraints-0.14.4` package frontier with a targeted test only.
- [x] Add minimal `Test.Hspec` interface and type-scheme stubs for package test-suite modules.
- [x] Add `GHC.TypeLits` `charVal`/`KnownChar` exports needed by `Data.Constraint.Char`.
- [x] Split `Data.Typeable.typeRep` from `Type.Reflection.typeRep` so the proxy-argument API typechecks.
- [x] Add reduced regression tests for Hspec, `charVal`, and proxy-argument `typeRep`.
- [x] Record that the remaining package frontier is `Data.Constraint.Forall` quantified-constraint kinding.
