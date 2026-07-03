<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Constraints Parser Boundaries Chirho

- [x] Reproduce the `Data.Constraint.Forall` `Forall1` alias overrun after the nested `instT` case body.
- [x] Stop type application parsing before an unseparated following `name ::` signature or `name =` binding.
- [x] Add a regression proving `Forall1 p = Forall p` and `inst1 :: ...` lower as separate declarations.
- [x] Reproduce the `Data.Constraint.Nat` `<=?` leak as closed type-family equations lowering into value bindings.
- [x] Make `type family` header scanning stop at `where` or result `::` only at top level, preserving kind-annotated parameters like `(m :: Nat)`.
- [x] Export the GHC.TypeNats `<=?` type-level operator in builtin interfaces.
- [x] Verify the reduced Nat package compiles and real `constraints` advances.
- [x] Smoke-test `transformers` and `mtl` after parser changes.
- [x] Fix chained type-operator RHS parsing so Data.Constraint.Nat arithmetic `Dict` proofs do not leak `+`/`*` outside the `Dict` argument.
- [x] Keep type-level operator aliases such as Data.Constraint.Symbol `type (++)` from shadowing value-level Prelude operators.
- [ ] Remaining frontier: GH55Spec `xf` as-pattern binding in infix `Num` methods and the `(\\)` entailment chain in `bar`.
