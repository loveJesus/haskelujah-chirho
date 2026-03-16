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
9. **NamedFieldPuns** — `{-# LANGUAGE NamedFieldPuns #-}`: `MkPoint{x, y}` as shorthand for `MkPoint{x=x, y=y}` in both patterns and expressions; lower_field_assign_chirho fills missing `=` with VarChirho of field name; 3 e2e tests
10. **MultiWayIf** — `{-# LANGUAGE MultiWayIf #-}`: `if | g1 -> e1 | g2 -> e2 | otherwise -> e3`; CST MultiWayIfExprChirho node; parser peek_after_if_is_pipe_chirho detects `if |` pattern; lowerer desugars to nested IfChirho; `otherwise` Core IR binding added as `True` ConApp; 3 e2e tests
11. **NumericUnderscores** — `{-# LANGUAGE NumericUnderscores #-}`: `1_000_000`, `0xFF_FF`, `0b10_10`; lexer accepts `_` in all numeric literal loops (decimal, hex, octal, binary, float, exponent); parse_integer_literal_chirho and float parsing strip underscores before conversion; 1 lexer test + 3 e2e tests
12. **Enum succ/pred** — Added `succ`/`pred` as Enum class methods; derive_enum_chirho generates direct constructor-to-constructor mappings; `$prim_Enum_succ_Int`/`$prim_Enum_pred_Int` primops; `succ Red` → `Green`, `pred Blue` → `Green`; 3 e2e tests
13. **TupleSections** — `{-# LANGUAGE TupleSections #-}`: `(,1) 42` → `(42,1)`, `(1,) 99` → `(1,99)`; parser detects leading commas and consecutive commas as gaps in paren expressions; lowerer generates lambda with `$ts_N` parameters for each gap position; `ExprChirho::LamChirho` wrapping `ExprChirho::TupleChirho`; works with `map (,True) [1,2,3]`; 3 e2e tests
14. **StandaloneDeriving** — `{-# LANGUAGE StandaloneDeriving #-}`: `deriving instance Show Foo`; CST StandaloneDerivingDeclChirho node; parser consumes `deriving instance [context =>] Class Type`; lowerer extracts class/types; deriving pass looks up matching data/newtype decl and delegates to existing derive functions; 3 e2e tests (Show, Eq, Ord)
15. **DeriveAnyClass** — `{-# LANGUAGE DeriveAnyClass #-}`: unknown class names in deriving clauses generate empty instance declarations relying on default methods; derive_anyclass_chirho produces InstanceDeclChirho with empty methods; 1 e2e test
16. **UnicodeSyntax** — `→` `←` `∷` `⇒` `∀` `λ` as alternatives to `->` `<-` `::` `=>` `forall` `\`; lexer recognizes Unicode codepoints and maps to existing token kinds; 3 e2e tests (arrows+double-colon, lambda, fat-arrow constraint)
17. **ImportQualifiedPost** — `import Data.Map qualified as Map` syntax; parser accepts `qualified` after module name in import declarations; lowerer already handles position-agnostic `qualified` token; 1 e2e test
18. **DerivingStrategies** — `deriving stock (Show)`, `deriving newtype (Num)`, `deriving anyclass (MyClass)` strategy keywords; parser skips `stock`/`newtype`/`anyclass` VarId before class list in deriving clauses; 2 e2e tests
19. **PackageImports** — `import "base" Data.List` syntax; parser skips package string literal before module name in import declarations; 1 e2e test
20. **RoleAnnotations** — `type role T nominal phantom` syntax; parser consumes `type role` declarations as skipped type-sig nodes; 1 e2e test
21. **DeriveDataTypeable** — `deriving (Typeable)` / `deriving (Data)` / `deriving (Lift)` / `deriving (NFData)` generate empty instance declarations; avoids unsupported-class warning for these well-known classes; 1 e2e test
