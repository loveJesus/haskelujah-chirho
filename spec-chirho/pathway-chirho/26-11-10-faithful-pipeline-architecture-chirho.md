<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# The faithful pipeline: architecture

2026-11-10. Companion to `26-11-10-ghc-replacement-pathway-chirho.md`. Status: PROPOSED with the pathway. Written so that an implementer on a smaller model can build one slice from this page plus the named GHC modules.

GHC references are to release 9.14.1 (tag object `961b2ffc`, commit `902339d3`). Module names were verified on 2026-11-10 against the retained tree (`tmp-chirho/pathway-chirho/references-chirho/ghc-9.14.1-chirho/ghc-9.14.1` in the union worktree; its `GIT_COMMIT_ID` is `902339d3`). Section 14 has the measured sizes.

## 1. Principles

1. **Faithful, not simulated.** Observable behaviour comes from compiling Haskell source. The compiler knows only GHC's wired-in names and the primop table.
2. **GHC's algorithm unless a deviation is recorded.** Section 12 is the ledger. A deviation needs a reason and a corpus that proves it harmless.
3. **One waist.** Core plus primops. Libraries above it, the RTS below it.
4. **Strangler.** `next` grows beside `legacy` behind `--pipeline next`. `next` never calls legacy typing, the dictionary pass or a hand-written interface.
5. **Fuel in three places, plus one ceiling.** Fuel: solver iterations, type-family reduction depth, inliner. No per-operation budgets. Interning does not bound total growth, so each module also has one ceiling on interned type nodes, reported as a resource refusal.
6. **No C toolchain.** Nothing in `next` requires a C compiler: Cranelift or Wasm output, a Rust link step, a Rust preprocessor, Rust big integers, and Rust versions of base's C helper files.

## 2. Pipeline and crates

```
source → lex, layout, parse            (keep: syntax, parser, ast)
       → resolve names                  (keep: naming, hardened on real code)
       → HIR                            (new: haskelujah-hir-chirho)
       → type check and elaborate       (new: haskelujah-tc-chirho)
       → desugar elaborated HIR to Core (new, thin; salvage the legacy match compiler where it is type-independent)
       → simplify → STG                 (keep: core IR and simplifier, without dict_chirho)
       → interpreter | LLVM | Cranelift | Wasm   (keep; one RTS)
```

| Crate | Fate |
| --- | --- |
| span, diagnostics, syntax, parser, ast, naming, package, cli, test-harness | keep |
| typing (135k lines) | legacy only; deleted at the end |
| core: `dict_chirho`, evidence parts of `desugar_chirho`, generated prelude | legacy only; deleted at the end |
| core: IR, simplifier, STG | keep; binders gain a representation field. Core's binders, case and type-application nodes carry the legacy `typing::TyChirho` today, so S0 gives Core its own type carrier or an adapter |
| runtime (interpreter) and rts (native) | merge into one RTS, section 9 |
| new: `haskelujah-hir-chirho`, `haskelujah-tc-chirho`, `haskelujah-prim-chirho` | sections 3 to 8 |

Kept from legacy, by gpt_chirho's source review (`tmp-chirho/pathway-chirho/factual-review-chirho/review-chirho.md` in the union worktree): Core expression shapes for explicit evidence; the producer-owned source, re-export and constructor-field contracts; the deterministic SCC algorithm; effective-extension validity; the Cabal component, options and discovery code. `next` never uses the `forall a. a` import fallback.

## 3. HIR: the checker's input

An adapter builds HIR from today's AST and name resolution, so legacy quirks stay behind it.

- Every binder and occurrence is a `NameIdChirho`: unique, package unit, defining module, occurrence text, namespace, and sort (local, top-level, wired-in). Today's source identities carry no package unit; the adapter adds it. No string lookup and no suffix fallback after this point.
- Operators are fixity-resolved. `where` is `let`. Record wildcards and puns are expanded. `do`, comprehensions, `if`, sections and literals stay as written, because their meaning is type-directed.
- Bindings arrive as strongly connected groups with their signatures attached and a "closed" flag for MonoLocalBinds.
- Implicitly quantified type variables are made explicit at their binding site, following GHC's renamer.
- Data constructors have one form for Haskell 98 and GADT syntax: universal and existential binders, context, fields with strictness and labels, result.
- Type and class declarations arrive in dependency order for kind inference.

## 4. Types, metas, levels

```rust
pub struct TyChirho(u32);                       // interned
pub enum TyNodeChirho {
    VarChirho(TyVarIdChirho),                   // skolem or bound
    MetaChirho(MetaIdChirho),
    ConChirho(TyConIdChirho),
    AppChirho(TyChirho, TyChirho),
    FunChirho { flag_chirho: FunFlagChirho, mult_chirho: TyChirho, arg_chirho: TyChirho, res_chirho: TyChirho },
    ForAllChirho { binder_chirho: TyVarIdChirho, vis_chirho: ForAllVisChirho, body_chirho: TyChirho },
    LitChirho(TyLitChirho),
}
pub struct MetaCellChirho { level_chirho: u32, kind_chirho: TyChirho, info_chirho: MetaInfoChirho, fill_chirho: Option<TyChirho> }
```

- Kinds are types. `Type` is `TYPE LiftedRep`.
- Tables: `TyConChirho` (kind, binders, roles, flavour: algebraic, newtype, synonym, family, class, primitive, promoted), `DataConChirho` (universals, existentials, equality spec, context, fields, tag), `ClassChirho` (variables, fundeps, superclasses, methods, associated types, dictionary constructor), class instances and family instances with rough-match heads.
- A meta is filled during generation only when its level equals the ambient level and the right-hand side passes the occurs, polytype, level and family checks of GHC's `checkTyEqRhs`. Otherwise an equality constraint is emitted and the solver decides. Untouchability and skolem escape therefore need no separate machinery.
- Result holes are GHC's `ExpType`: `CheckChirho(ty)` or `InferChirho(hole)`, with the fill and join rules of Note [fillInferResult].

## 5. Constraints and evidence

```rust
pub enum CtChirho {
    DictChirho { ev_chirho: EvVarChirho, class_chirho: ClassIdChirho, args_chirho: Vec<TyChirho>, flavour_chirho: FlavourChirho },
    EqChirho { lhs_chirho: TyChirho, rhs_chirho: TyChirho, role_chirho: RoleChirho, flavour_chirho: FlavourChirho },
    IrredChirho { pred_chirho: TyChirho, flavour_chirho: FlavourChirho },
    QuantifiedChirho { /* forall tvs. context => head */ },
}
pub struct ImplicationChirho {
    level_chirho: u32, skolems_chirho: Vec<TyVarIdChirho>, givens_chirho: Vec<EvVarChirho>,
    wanted_chirho: WantedChirho, ev_binds_chirho: EvBindsIdChirho, given_eqs_chirho: GivenEqsChirho, info_chirho: SkolemInfoChirho,
}
pub struct WantedChirho { simple_chirho: Vec<CtChirho>, implications_chirho: Vec<ImplicationChirho> }
```

- Class evidence is a term: an evidence variable, a dictionary-function application, a superclass selection, or a built-in (Typeable, KnownNat, KnownSymbol, CallStack, implicit parameter, HasField).
- Equality evidence is erased. A wanted equality carries only an identity for "solved or not". This is deviation DV-1.
- Evidence bindings belong to the implication that solved them and become `let` bindings at that implication's abstraction site in Core.

## 6. Generation

The checker walks HIR once, emits constraints and returns elaborated HIR (the same shapes plus wrapper nodes for type and evidence abstraction and application).

| Area | Rule set to port |
| --- | --- |
| Expressions, applications | `GHC.Tc.Gen.Expr`, `App`, `Head`: head plus arguments, lazy instantiation, visible type application, Quick Look only under ImpredicativeTypes, simplified subsumption |
| Patterns, matches | `GHC.Tc.Gen.Pat`, `Match`: a constructor with existentials, equalities or a context opens an implication with those givens; all equations share one `ExpType` |
| Bindings, signatures | `GHC.Tc.Gen.Bind`, `Sig`: check against a complete signature, else infer and generalize, else no generalization under MonoLocalBinds |
| Types in terms | `GHC.Tc.Gen.HsType`: bidirectional kind checking, invisible kind arguments, wildcards, scoped type variables |
| `do`, comprehensions, literals, records | elaborated against known-key names (`>>=`, `fromInteger`, `fromString`, `enumFromTo`); RebindableSyntax takes the names the renamer resolved |
| Unifier and helpers | `GHC.Tc.Utils.Unify`, `TcMType`, `Instantiate`: skolemise, instantiate, promote, `fillInferResult` |

## 7. Solver

A port of `GHC.Tc.Solver` and its submodules (`Solve`, `Equality`, `Dict`, `Irred`, `Rewrite`, `InertSet`, `Monad`, `Default`, `Types`), without coercion construction.

- Work list of simple constraints, then implications recursively, repeated while an outer meta was unified, up to the iteration fuel.
- Canonical forms: dictionary; equality with a type variable or family application on the left; irreducible; quantified.
- Rewriter: types are rewritten by the inert equalities and by family axioms, respecting flavour and role. Givens rewrite everything; wanteds rewrite wanteds, with the rewriter set kept so that one unsolved equality yields one error.
- Equalities: decompose injective constructors, functions, applications and foralls; orient variable pairs by GHC's rules; occurs check with cycle breakers for givens over families; newtype unwrapping at representational role when the constructor is in scope.
- Touchability: a meta may be unified inside an implication only if no implication between its level and the current one has given equalities. This replaces every legacy scope, projection and result-cell rule.
- Dictionaries: inert givens first; then instances, with overlap resolution and the given-overlap safety check; functional-dependency improvement (`GHC.Tc.Instance.FunDeps`); superclass expansion of givens with loop fuel.
- Built-in solvers (`GHC.Tc.Instance.Class`): Typeable, KnownNat, KnownSymbol, KnownChar, Coercible, HasField, implicit parameters and CallStack, `~`, `~~`, TypeError, Unsatisfiable, WithDict, DataToTag.
- Families (`GHC.Core.FamInstEnv`, `GHC.Core.Unify`): open families by compatible unique match; closed families by first match with apartness of earlier branches; injectivity improvement; the type-literal families of `GHC.Builtin.Types.Literals`.
- Generalization is `simplifyInfer`: solve, choose the quantified variables (monomorphism restriction, environment variables, variables fixed by outer equalities), pick quantifiable predicates, minimise by superclasses, skolemise.
- Top level is `simplifyTop`: representation, levity, multiplicity and kind defaulting; then class defaulting by the Report's rule, `default` declarations and ExtendedDefaultRules.
- Residual constraints become diagnostics that carry GHC's own error codes (table generated from `GHC.Types.Error.Codes`). should_fail then compares code sets, not only "rejected".

## 8. Declarations, instances, deriving

| Area | Port |
| --- | --- |
| Kind inference per declaration group, standalone kind signatures, generalization of kind variables | `GHC.Tc.TyCl` |
| Role inference and annotations, validity of declarations | `GHC.Tc.TyCl.Utils`, `GHC.Tc.Validity` |
| Class declarations, default methods and signatures, associated types | `GHC.Tc.TyCl.Class` |
| Instance heads, method checking, associated instances | `GHC.Tc.TyCl.Instance` |
| Pattern synonyms | `GHC.Tc.TyCl.PatSyn` |
| Deriving: specs, context inference by solving to a fixed point, method generation for stock classes, Functor-like classes, Generic, Data, Lift; newtype, anyclass, via | `GHC.Tc.Deriv` and submodules. Generated code is HIR and is checked like user code (DV-2) |
| Typeable metadata per module and type | `GHC.Tc.Instance.Typeable` |
| Class and family instance environments | `GHC.Core.InstEnv`, `GHC.Core.FamInstEnv` |

Wired-in and known-key names mirror `GHC.Builtin.Names`, `GHC.Builtin.Types` and `GHC.Builtin.Types.Prim`. When the library module that declares a wired-in entity is itself compiled (`data Bool` in `GHC.Types`), the declaration binds to the wired-in identity.

## 9. The waist: Core, primops, RTS

**Core out.** The existing Core IR is the target: `Var`, `Lit`, `App`, `Lam`, `Let`, `Case`, `TyLam`, `TyApp`, `PrimOp`, `ConApp`.

- A class is a data type with one constructor: superclass dictionaries, then methods. Selectors are `Case`. An instance is a top-level dictionary function. Constructor patterns bind their dictionary fields as ordinary variables.
- Newtypes are erased. Strict fields and bang patterns evaluate with `Case`. Bindings of unlifted type are strict.
- Every binder carries a representation (`PrimRepChirho`: lifted pointer, unlifted pointer, int, word, float, double, address, void, unboxed tuple shape). Backends read representations, never types.
- No `$prim_`, `$own` or `$sigma` name exists in `next`.

**Primops.** `haskelujah-prim-chirho` generates Rust tables at build time from a vendored copy of GHC's `compiler/GHC/Builtin/primops.txt.pp` (585 primops, 36 primitive types, 6 pseudo-operations): primitive types with their representations, primops with types and attributes, pseudo-operations. The virtual module `GHC.Prim` is served from that table. Each primop has one Rust implementation in the RTS and one conformance test against a GHC-produced result.

**RTS.** One crate, two engines.

- Word-based heap objects with headers: functions, partial applications, thunks with update and blackholing, constructors, arrays and byte arrays (pinned and unpinned), mutable variables, MVar, TVar, weak pointers, stable pointers, thread objects.
- Precise generational copying GC with pinned objects. Roots: the explicit interpreter stack, native stack maps, stable pointers, CAFs.
- Exceptions with masking and asynchronous delivery; green threads on one capability first; STM; `delay#`, `waitRead#`, `waitWrite#` through poll.
- FFI: static, dynamic and wrapper imports, `capi`, exports. The Cmm-implemented primitives of GHC's RTS are Rust functions here.
- The interpreter is the reference engine through G4 and stays for Template Haskell and GHCi. Native engines consume the same STG and heap layout.
- The detailed RTS design is its own document, written when track W starts.

## 10. Libraries and interfaces

- In 9.14.1 the whole core library is one package, ghc-internal, with modules named `GHC.Internal.*`. ghc-prim and ghc-bignum are five-line shims, and base is 259 re-export modules.
- The ladder, under `GHC.Internal`: `Types`, `Classes`, `CString`, `Magic`, `Tuple`; then `Base`, `Maybe`, `List`, `Char`, `Show`, `Num` with the `Bignum` modules, `Real`, `Enum`; then errors, call stacks, exceptions and Typeable; then ST, IORef, arrays, bits, words; then `GHC.Float`, Read, Generics, Foldable and Traversable; then the IO and Handle stack, concurrency and Foreign; then all of base.
- A module counts when it compiles from unmodified source. A temporary patch is recorded in a patch series that must shrink to nothing by G3.
- Until the real Handle stack compiles, test programs print through one small interim IO module. It is the only interim library code and is deleted at G3.
- The preprocessor profile must present GHC 9.14.1 and its base version with real configuration headers. Today's advertises GHC 8.10 and base 4.14 with synthetic headers.
- Interface files are our own format with a version hash: exports, fixities, types, classes, constructors, pattern synonyms, instances with orphan flags, defaults, and unfoldings for INLINE bindings. `.hs-boot` modules are supported from slice S2, because base needs them.

## 11. Build order

Each slice ends with a scoreboard bucket that must be green and a set of real modules that must compile.

| Slice | Content | Exit |
| --- | --- | --- |
| S0 | Crates, `--pipeline next`, interner, metas, levels, unifier, constraint types, solver for equalities, simple dictionaries and implications | Unit tests; monomorphic programs run |
| S1 | Haskell 2010 expressions, patterns, bindings, generalization, data, newtypes, synonyms, single-parameter classes and instances, defaulting, literals; with W0: unboxed values, primop calls, MagicHash | Programs using Eq, Ord, Show and Num from the real `GHC.Classes`, `GHC.Base`, `GHC.Show`, `GHC.Num` run on the interpreter (G1) |
| S2 | Interfaces, multi-module, hs-boot, stock deriving, records, multi-parameter classes, flexible instances and contexts, fundeps, overlap, basic kind polymorphism | `GHC.List`, `GHC.Enum`, `GHC.Real` and their neighbours compile; module and rename corpora |
| S3 | Rank-N, scoped type variables, type applications, partial signatures, implicit parameters, CallStack | Higher-rank bucket; `GHC.Err`, `GHC.Stack` compile |
| S4 | Existentials, GADTs, given equalities, untouchables | GADT bucket; escape and untouchable codes in should_fail |
| S5 | Type and data families, injectivity, type literals, DataKinds, full PolyKinds, standalone kind signatures, levity checks | Families bucket; Typeable and `GHC.Generics` compile |
| S6 | Newtype, via and anyclass deriving; Functor-like, Generic, Data, Lift; roles, Coercible; quantified constraints | ghc-internal and base type-check (front half of G3) |
| S7 | The long tail, ordered by frequency in boot packages and by should_fail code: pattern synonyms, HasField and record dot, unlifted newtypes, type abstractions, required type arguments, linear types | G2 |
| S8 | Template Haskell: the library from source, the Quasi operations, splices in the interpreter, quasi-quotes, reify | G4 |
| S9 | Checker speed, incremental and parallel compilation | after correctness |

Crate layout for `haskelujah-tc-chirho/src`, files under 1,000 lines: `types_chirho/`, `env_chirho/`, `constraints_chirho/`, `unify_chirho/`, `gen_chirho/`, `hstype_chirho/`, `tycl_chirho/`, `inst_chirho/`, `deriv_chirho/`, `solver_chirho/`, `elab_chirho/`. Tests live beside each directory in their own files.

## 12. Deviations ledger

| Id | Deviation | Consequence |
| --- | --- | --- |
| DV-1 | No coercion terms; equality evidence is erased | No Core Lint. Deferred type errors use evidence placeholders that throw. A program that needs an explicit kind coercion in a type is refused with a named diagnostic until a corpus needs it |
| DV-2 | Derived code is generated as HIR, not as GHC's syntax tree | None observable |
| DV-3 | Diagnostic text is ours; codes are GHC's | should_fail compares codes |
| DV-4 | Fuel replaces GHC's counters, with GHC's defaults (reduction depth 200, solver iterations 4) | Same accept and reject on programs within the defaults |

## 13. Oracle harness

- One scoreboard: corpus, case, stage (parse, rename, check, run on interpreter, run native), verdict, error-code set, hash of the normalized `-ddump-types` output, hash of stdout, exit code.
- GHC receipts are cached by source hash, flags and GHC version, and are never regenerated silently.
- Corpora: the full GHC test suite from the 9.14.1 source tree, every module of the boot libraries, then Stackage packages.
- Failure classes are generated from the scoreboard and choose the next unit of work.

## 14. Reference sizes

Measured on 2026-11-10 in the retained GHC 9.14.1 tree.

| Part | Size |
| --- | --- |
| Type checker, `compiler/GHC/Tc` | 120,041 lines in 74 files |
| of which `Gen` | 23,584 |
| `Utils` | 16,442 |
| `Errors` (types and printing; we need little of it) | 15,657 |
| `Solver` | 14,899 |
| `Deriv` | 8,097 |
| `Types` | 6,964 |
| `TyCl` | 6,174 |
| `Instance` | 3,759 |
| `Zonk` | 3,262 |
| Renamer, `compiler/GHC/Rename` | 18,774 lines |
| Desugarer, `compiler/GHC/HsToCore` | 23,969 lines |
| Diagnostic codes, `GHC/Types/Error/Codes.hs` | 706 codes |
| ghc-internal | 276 `.hs`, 27 `.hsc`, 24 `.hs-boot`; 102,782 lines |
| ghc-internal's C helpers | 16 C files (3,389 lines) and 3 Cmm files |
| base | 259 `.hs`, 7 `.hsc`; 24,528 lines; no C; 251 test programs |
| Boot packages in the same tree | containers 109 files, no C; transformers 31; mtl 25; bytestring 55 plus 6 C files; text 105 plus 5 C files |
| GHC's C runtime, `rts/` (what our Rust RTS replaces) | 177 C files and 17 Cmm files; 124,933 lines with headers |
| Test suite (separate archive, hash verified) | 520 `all.T` files, 11,205 Haskell files, 2,457 expected-output files |
| Test suite by area (Haskell files) | typecheck 1,883; th 568; rename 566; simplCore 525; indexed-types 487; parser 440; deriving 285; module 276; codeGen 263; polykinds 255; deSugar 175; patsyn 167; dependent 162; overloadedrecflds 161; ffi 121; concurrent 113; gadt 109; numeric 92; primops 66 |
| nofib benchmark programs, in the source tree | 544 Haskell files |
