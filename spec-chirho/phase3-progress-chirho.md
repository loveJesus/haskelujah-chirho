<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Haskeluya Phase 3 Progress — Completed Items

**Phase 3 start**: 2026-03-15

---

## Completed

1. **GADTs** — `data Foo where Con :: Type` syntax; ConDeclChirho::GadtChirho preserves full type sig; parser, type/kind inference, exhaustiveness, naming, desugaring, TH all handle GadtChirho; 5 e2e tests
2. **RankNTypes** — TyChirho::ForallChirho for non-prenex positions; alpha-rename unification; SimpleSubsumption strip; bind_pat_chirho polymorphic schemes; subsume_chirho for rank-N; 3 ty + 4 unify + 4 e2e tests
3. **ScopedTypeVariables** — scoped_tyvars_chirho on InferCtxChirho from function forall vars; reused in where-clause annotations; 3 e2e tests
4. **RecordWildCards** — `Foo{..}` pattern/expression; con_field_names_chirho maps constructor → fields; expression wildcards fill missing from scope; pattern wildcards bind remaining as fresh vars; 4 e2e tests
5. **ViewPatterns** — `(expr -> pat)` syntax; ViewPatChirho CST node; desugaring via let-binding (VarChirho inner) or case (constructor inner); 3 e2e tests
6. **PatternSynonyms** — `pattern Name args = pat` (bidirectional) / `pattern Name args <- pat` (unidirectional); PatSynDefChirho collection, expand_pat_syn_chirho substitution, pat_to_builder_expr_chirho for expression position; 3 e2e tests
7. **Strict data fields** — `!` before constructor fields; StrictnessChirho enum; enforce_strict_fields_chirho wraps args in case for WHNF; 2 parser + 4 e2e tests
8. **DerivingVia** — `deriving (Class) via ViaType` on newtypes; parser consumes `via Type` in deriving clauses; lowerer extracts deriving_via entries with separate class/via extraction; derive_via_chirho generates methods that unwrap newtype and delegate (Show/Eq/Ord/Num); 4 e2e tests
