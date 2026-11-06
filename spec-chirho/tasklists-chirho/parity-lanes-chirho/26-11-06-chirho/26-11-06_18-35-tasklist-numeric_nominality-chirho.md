<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Nominal numeric equality Chirho

The ordinary unifier currently accepts both `Int ~ Double` and `Double ~ Int` with an empty substitution. This equates distinct type constructors without a conversion. Row 708 removes that one compatibility arm; numeric literals, explicit conversion and defaulting retain their own obligations.

- [x] Pin an isolated A9 source baseline (`075a9c95`, crates `a9a38e3d`) and census the normal-unifier consumers. Root owns canonical metadata and SQLite.
- [x] Name the source footprint before editing: bounded direct unifier units; one numeric-nominality driver child, ten Haskell fixtures and its two-line root mount. No new API, registry or runtime representation route.
- [x] Publish the production-equal test-only checkpoint; preserve the original MR lambda witness separately because A9 does not contain MR.
- [ ] Pin and execute the ten independent GHC typing controls before any functional claim.
- [ ] Execute the original unifier against the negative controls and retain actual wrong acceptance.
- [x] Remove only the normal numeric widening arm and its misleading comment in a separate source checkpoint. The `0afac144` test-only checkpoint retains the original production behavior for the later measured negative control.
- [ ] Execute symmetry/nested rejection and same-type/metavariable preservation units, plus the paired driver controls; surface any literal or numeric-pattern producer prerequisite before expanding scope.
- [ ] Complete focused warning-free checking, lint attribution, outgoing secret scan and push with exact source/evidence pins. Broader preservation and integration remain separate.

The driver controls compile source without running a program. Acceptance of an unannotated numeric binding establishes that it can be checked, not a particular inferred default type. The original MR lambda failure depends on restricted generalization and is not replaced by a prediction about this A9-only patch.
