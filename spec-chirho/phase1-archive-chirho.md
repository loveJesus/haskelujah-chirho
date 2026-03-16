<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->

# Haskelujah Phase 1 Archive — Completed Priorities & Test Coverage

**Phase 1 span**: 2026-03-12 → 2026-03-14
**Final state**: 1273 tests passing, 19 crates, ~84,500 lines of Rust

---

## Completed Priorities (1–128)

1. Core IR improvements (let-rec, sections, list comprehension desugaring)
2. STG lowering improvements (recursive let handling in STG, proper letrec closure allocation)
3. End-to-end evaluation of data constructors, case dispatch, function application — literal boxing uses con_tags_chirho from STG lowerer for tag consistency
4. Function with case/conditional evaluation — ground dictionary resolution in dict pass, ConAppChirho in Core IR, case alt binder binding, arg_regs save/restore
5. Let expressions and higher-order functions in evaluation — let binding lowering, layout engine in-keyword fix, AppFromArgChirho for higher-order calls, ArgSourceChirho for runtime arg resolution
6. Where-clause lowering — parser lowering of WhereClauseChirho siblings, type inference for where-binds, desugarer scoping fix, enter_fun_chirho Update-frame loop fix
7. Pattern matching in case expressions — CaseLitChirho for literal dispatch, nested pattern desugaring via prebind_all_pat_vars_chirho + wrap_nested_cases_chirho
8. Source file discovery improvements — test-suite module discovery, Setup.hs/Setup.lhs detection, hierarchical module paths
9. Lambda/closure evaluation and let-rec in STG — free variable analysis, AllocFunChirho, StoreAllocFunChirho, enter_fun_chirho payload prepend
10. String/list literal support — StringChirho variant, string/char literal evaluation, list literal compilation via cons chains
11. Guard expressions in function equations and case alternatives
12. Basic I/O (putStrLn primop, do-notation desugaring) — PutStrLnChirho/PutStrChirho/BindIOChirho/ReturnIOChirho/ThenIOChirho primops
13. Prelude basics (otherwise, $, ., negate)
14. do-notation evaluation (>>=/>>/return through STG machine)
15. Recursive let-bindings in evaluation (letrec) — ArgSourceChirho, ThunkCodeChirho, mutual recursion, operator precedence
16. Type class instance resolution at runtime — ConAppChirho-based dictionary construction, $sel + $fClassType projection
17. Numeric literal overloading (fromInteger) — integer literals desugar to fromInteger, CaseLitChirho stack frame
18. Call-site dictionary argument insertion — dict_param_bindings_chirho, rewrite_method_refs_chirho
19. Superclass dictionary extraction — context reduction, superclass selectors, add_dict_params_chirho
20. Proper show for Int — ShowIntChirho primop
21. Prelude function bindings — not, id, const
22. Multi-parameter type classes and functional dependencies (see 56-57)
23. Conditional instance dictionaries (Eq a => Eq [a]) — generate_conditional_ground_dicts_chirho
24. /= operator — desugars to not (x == y)
25. String concatenation (++) — AppendStrChirho primop
26. List operations and I/O sequencing — head/length/sum/map/filter/foldr/foldl/reverse/append
27. String Eq/Show instances — EqStrChirho/ShowStrChirho/LengthStrChirho primops
28. Double/Float arithmetic through Num type class
29. Arithmetic sequences ([from..to]) — EnumFromToChirho primop
30. Show for lists (show [1,2,3]) — generate_show_list_int_binding_chirho
31. Variable-typed Double dispatch via dict transform
32. Fractional type class (/, recip, fromRational)
33. Read type class basics
34. Conditional instance dictionaries (specialized list instances)
35. Multi-parameter type classes data structures
36. Functional dependency integration into type inference
37. MPTC dict layout and call-site rewriting
38. Backtick infix syntax
39. Type class default methods
40. Where-clause class method body compilation
41. Prelude numeric/tuple functions (even, odd, abs, max, min, fst, snd)
42. String operations (words, unwords, take, drop, concat, intercalate)
43. Numeric conversions (toInteger, fromIntegral, ceiling, floor, round, truncate)
44. curry/uncurry as Prelude functions
45. Maybe/Either Prelude functions (maybe, either, fromMaybe, isJust, isNothing)
46. Where-clause compilation for user-defined instance methods
47. Multi-equation constructor pattern matching — merge_fun_binds_chirho
48. Operator section syntax (+)
49. Type class default methods (extended with defaults_chirho)
50. let/where in do-notation
51. Newtype deriving and GeneralizedNewtypeDeriving — newtype constructor erasure
52. Record syntax field access and update
53. Record update syntax (expr { field = newVal })
54. ScopedTypeVariables and RankNTypes — ForallChirho keyword, forall parsing
55. GADTs syntax — GadtConDeclChirho, GADT parsing
56. Multi-parameter type classes end-to-end — class_param_count_chirho, combined type key inference
57. Parsing functional dependencies from source
58. Conditional instance dictionaries fix — ConAppChirho vs AppChirho
59. List comprehension end-to-end evaluation
60. Import specification lowering
61. Higher-order list Prelude functions — map, filter, foldr, foldl, head, tail, null, length, reverse, zip, zipWith
62. Multi-module evaluation end-to-end — eval_modules_chirho, CoreId offsetting
63. Additional Prelude list functions — append, any, all, sum, product, concatMap, last, init
64. Deriving Eq/Show end-to-end + boolean operators (&&, ||)
65. Arithmetic sequence variants (enumFrom/enumFromThen/enumFromThenTo)
66. Polymorphic take/drop with thunk forcing
67. Ord/compare with Ordering data type, min/max
68. Prelude functions: elem, notElem, minimum, maximum, sort, insert
69. Num class abs/signum methods
70. Enum and Bounded type classes + derived Ord/Enum
71. Integral type class with quot/rem
72. Data.Char functions (ord, chr, isDigit, isAlpha, toLower, toUpper, etc.)
73. Floating type class (sin, cos, exp, log, sqrt, etc.)
74. Functor/Applicative/Monad type class hierarchy
75. Power/exponentiation operators (^ and **)
76. Complete Num Double instance dictionary
77. Complete numeric class stack + additional Data.List functions
78. Numeric literal defaulting / mixed Int+Double arithmetic
79. IO operations (getLine, getChar, readFile, writeFile, appendFile)
80. error/undefined/seq builtins + otherwise guards
81. Show for Maybe/tuples
82. IO ops + string builtins + escape sequences
83. Pattern matching fixes (negative literals, string patterns, as-patterns)
84. Function composition (.) operator fix
85. First-class IO functions
86. IORef mutable references
87. Data.Map BST Prelude
88. Free-variable cache in desugarer
89. Show True/False through typeclass machinery
90. Data.Map extended operations
91. Data.Set BST Prelude
92. Data.Set extended operations
93. String-as-[Char] interop
94. String ordering primops + Ord [Char]
95. Operator sections (left/right)
96. Lambda pattern matching
97. Numeric escape sequences
98. Data.Map extended operations (insertWith, findWithDefault, adjust, unionWith, etc.)
99. Additional list functions (nub, zip3, intersperse, isPrefixOf, isSuffixOf)
100. String-keyed Data.Map
101. 1000 tests milestone
102. Data.Maybe extras + Data.IORef Core IR wrappers
103. Feature combination e2e tests
104. fromJust/swap + mapDelete fix
105. Multi-equation literal pattern matching
106. Semigroup/Monoid type classes
107. Higher-order *By list functions (sortBy, insertBy, nubBy, maximumBy, minimumBy, on)
108. Data.Map/Set polymorphic keys with Ordering dispatch
109. Where-clause and let-expression type annotations
110. Type synonyms in instance heads and predicate resolution
111. IORef ModifyIORef fix
112. AVL balanced Map/Set trees + synthetic module interfaces
113. Cranelift/JVM/BEAM backend scaffolds
114. IO type system + e2e test stabilization
115. Polymorphic elem/nub/isPrefixOf + utility Prelude functions
116. Fix ignored IORef/deriving Ord tests + Eq/Show/Ord Ordering instances
117. Polymorphic ++ list append + exception handling framework + BST tests
118. Exception handling (catch/throw/try) end-to-end
119. User-defined 4-field constructors + complex data types + pattern gaps
120. Data.Set as runtime primops
121. Show for compound types + IO control flow
122. Data.Map as runtime primops
123. Exception handling try/catch/bracket/finally
124. STRef modifySTRef
125. Complex program patterns
126. find/groupBy Prelude + Map higher-order ops
127. tails/inits tests + type annotation :: + read+show fix
128. Algorithmic stress tests

---

## Phase 1 Test Coverage Breakdown

| Category | Count | Details |
|---|---|---|
| Golden parse tests | 10 | type_sig, data_decl, fun_bind, lambda, case, do_block, let_expr, list_comp, record, class_decl |
| Typing integration | 8 | identity fn, data constructors, if-expr, list/tuple/let literal, lambda, negative type error |
| Runtime tests | 65 | value types 8, heap 5, stack 5, primitives 10, evaluator 15, GC 10, FFI 13 |
| Exhaustiveness | 23 | con env 3, exhaustive 3, non-exhaustive 4, redundancy 3, function binding 3, newtype 1, classification 2, edge 4 |
| Naming/module | 25 | env 6, iface 6, resolve 13 |
| Kind inference | 24 | kind repr 6, subst 2, unification 5, env 1, data decl 4, class 2, type alias 1, newtype 1, edge 2 |
| Driver integration | 3 | multi-module import, complete/incomplete exhaustiveness |
| Unit tests (misc) | ~120 | subst, unify, infer, Core expr/pretty/simplifier/dict/desugar, LLVM/Wasm codegen, typeclass, deriving |
| Package tests | 39 | version 5, constraints 7, Cabal 11, Hackage 3, resolver 13 |
| Incremental tests | 37 | fingerprint 9, dep graph 9, artifact store 8, session/rebuild 11 |
| Cranelift tests | 3 | empty module, simple binding, lambda binding |
| JVM tests | 8 | constant pool 4, bytecode builder 2, class compilation 2 |
| BEAM tests | 17 | ETF 7, opcode builder 3, beam module 4, opcode definitions 3 |
| Driver e2e tests | ~890 | arithmetic, eval, case, pattern, closure, literal, guard, IO, Prelude, do-notation, recursion, typeclass, deriving, show, string, list ops, Map, Set, exception, IORef, algorithmic, etc. |
| **Total** | **1273** | 0 failures, 3 ignored (2 Cranelift, 1 doctest) |

---

## Workspace Crates at End of Phase 1

| Crate | Purpose |
|---|---|
| `haskelujah-span-chirho` | Source locations, file IDs, source map |
| `haskelujah-diagnostics-chirho` | Structured compiler diagnostics |
| `haskelujah-syntax-chirho` | Tokens, SyntaxKind, green tree nodes, lexer, layout |
| `haskelujah-parser-chirho` | CST parser (green tree builder) |
| `haskelujah-ast-chirho` | Abstract syntax tree types, CST→AST lowering |
| `haskelujah-naming-chirho` | Name resolution / scope analysis, module interfaces |
| `haskelujah-typing-chirho` | HM type inference, unification, kind inference, exhaustiveness, deriving |
| `haskelujah-core-chirho` | Core IR, AST→Core desugaring, dict-passing transform, simplifier |
| `haskelujah-backend-llvm-chirho` | Core → textual LLVM IR codegen |
| `haskelujah-backend-wasm-chirho` | Core → binary WebAssembly codegen |
| `haskelujah-backend-cranelift-chirho` | Core → Cranelift IR → native object files |
| `haskelujah-backend-jvm-chirho` | Core → JVM .class bytecode files |
| `haskelujah-backend-beam-chirho` | Core → BEAM .beam bytecode files |
| `haskelujah-driver-chirho` | Pipeline orchestration, CompileResultChirho |
| `haskelujah-runtime-chirho` | STG runtime: values, heap, eval loop, GC, FFI, exceptions |
| `haskelujah-incremental-chirho` | Incremental compilation: fingerprinting, dep graph, caching |
| `haskelujah-package-chirho` | Cabal file parser, version constraints, Hackage URLs |
| `haskelujah-cli-chirho` | Command-line interface (scaffold) |
| `haskelujah-test-harness-chirho` | Golden test utilities |

---

## Pipeline at End of Phase 1

1. Lex → 2. Layout → 3. CST Parse → 4. AST Lower → 5. Name Resolve → 6. Kind Infer → 7. Type Infer → 7b. Exhaustiveness Check → 8. Desugar → Dict Pass → Core → Simplify → Backends → 9. STG Evaluation → 10. GC → 11. FFI → 12. Exception Handling
