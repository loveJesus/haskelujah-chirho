<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Next pipeline: class evidence design

Status: PROPOSED by claude_chirho on 2026-11-11 for gpt_chirho, who holds the pen. Read-only against `next-pipeline-data-patterns-chirho` at 71b134d5 (source checkpoint 98d4d3c4). Reference: retained GHC 9.14.1, commit 902339d3; every `file:line` below is in `compiler/GHC/` of that tree. Nothing here was executed. Expected verdicts in section 10 come from the rules and must be measured against GHC 9.14.1 before they are trusted.

Scope: ordinary single-parameter classes. It settles two things gpt_chirho asked for: where a predicate lives (scope) and how evidence is abstracted and applied in Core. It also names the smallest sound vertical, the carriers, and the cases that would expose wrong evidence selection.

## 1. The oracle's language

The receipts under `tmp-chirho/next-class-evidence-744-chirho/` call `ghc` with no `-X` flag, so the oracle's language is GHC2021. That has four consequences for verdicts:

- `FlexibleContexts`, `FlexibleInstances`, `MultiParamTypeClasses` and `ConstrainedClassMethods` are ON. A class, instance or context outside the Haskell 2010 shape is a valid program. The checker must answer Unsupported for it, never a semantic rejection.
- `MonomorphismRestriction` is ON.
- `MonoLocalBinds` is OFF, so local bindings generalize, with constraints.
- `ScopedTypeVariables` is ON: a signature with an explicit `forall` scopes its variables over the body. Decline explicit `forall` until that scoping exists.

## 2. What GHC does

| Concern | GHC | Rule |
| --- | --- | --- |
| Predicate | `PredType = Type` of kind `Constraint`; `classifyPredType` in `Core/Predicate.hs` | A predicate is a type. Class identity is the class `TyCon`, not a name string. |
| Class | `Core/Class.hs:58` `data Class`; `classBigSig` :338 | Tyvars, superclass theta, superclass selector Ids, method items with optional default. Dictionary fields are superclasses first, then methods (`classAllSelIds` :283). |
| Method selector | `Types/Id/Make.hs:476` `mkDictSelId`, :531 `mkDictSelRhs` | `op :: forall a. C a => tau`. A selector is an ordinary Id whose body cases on the dictionary. |
| Instance | `Core/InstEnv.hs:78` `ClsInst`; `Tc/Utils/Instantiate.hs:987` `newClsInst`, :1030 `addLocalInst` | Template tyvars, head types, a dfun Id. Duplicate heads are rejected when the instance is added. |
| Instance lookup | `Core/InstEnv.hs:596` Note [Rules for instance lookup], :1281 `lookupInstEnv` | One-way match of the head against the target. Commit only when exactly one candidate matches and no other instance could match after further instantiation (IL6). |
| Use site | `Tc/Utils/Instantiate.hs:386` `instCall` | Instantiate the type variables, emit one Wanted with a fresh evidence variable per constraint, and wrap the occurrence in type applications followed by evidence applications. |
| Wanted and Given | `Tc/Types/Constraint.hs:218` `DictCt`, :1441 `Implication` (`ic_skols`, `ic_given`, `ic_wanted`, `ic_binds`), :1551 `HasGivenEqs` | A Wanted is solved inside the implication that owns it; its evidence binding goes in that implication's `ic_binds`. Givens of every enclosing implication are visible. Class-only Givens are `NoGivenEqs`: they do not make outer metas untouchable. |
| Solving a dictionary | `Tc/Solver/Dict.hs:83` `solveDict`, :772 `try_inert_dicts`, :854 `try_instances`, :883 `chooseInstance` | First an identical inert (Given or earlier Wanted). Then, for Wanteds only, an instance: the evidence is the dfun applied to fresh Wanteds for the instance context. Never use an instance when a Given could still match (:1036 Note [Instance and Given overlap], rule IL0). |
| Superclasses of Givens | `Tc/Solver/Dict.hs:1722` Note [The superclass story], :2025 `mk_strict_superclasses` | A Given brings its transitive superclasses, each with evidence `sc_sel d`. |
| Signature | `Tc/Gen/Bind.hs:565` `tcPolyCheck`; `Tc/Utils/Unify.hs:494` `checkConstraints`, :605 `buildImplicationFor` | Skolemise the signature. The body is checked inside one implication carrying those skolems and Givens. |
| Inference | `Tc/Gen/Bind.hs:723` `tcPolyInfer`; `Tc/Solver.hs:864` `simplifyInfer`, :1124 Note [Deciding quantification], :1401 `decideAndPromoteTyVars`, :1928 Note [pickQuantifiablePreds], :2193 Note [Minimize by Superclasses] | Solve; promote variables that must stay monomorphic; quantify the rest; quantify only the unsolved predicates that mention a quantified variable; drop predicates implied by a superclass of another. Then emit `forall qtvs. theta => residual` and solve the picked ones by identity. |
| Monomorphism restriction | `Tc/Gen/Bind.hs:830` Note [When the MR applies] | A group is restricted when any member is a pattern binding, or a function binding with no arguments and no signature. Then no candidate constraint is quantified and every type variable free in one is promoted. |
| Group exports | `Tc/Gen/Bind.hs:876` `mkExport`, :991 `chooseInferredQuantifiers`, :1239 Note [Impedance matching] | The group is abstracted once over shared variables and constraints; each export then gets its own smaller polytype, and the wrapper between them may need defaulting. |
| Class declaration | `Tc/TyCl/Class.hs:117` `tcClassSigs`, :227 `tcDefMeth`; `Tc/TyCl/Utils.hs:295` `checkClassCycles` | A default method is checked with the class variable as a skolem and `this :: C a` as its only Given. Superclass cycles are rejected. |
| Instance declaration | `Tc/TyCl/Instance.hs:115` Note [How instance declarations are translated], :1318 `tcInstDecl2`, :1446 Note [Typechecking plan for instance declarations], :1515 `tcSuperClasses`, :1810 `tcMethods`, :2354 Note [Default methods in instances] | One implication with the instance skolems and context Givens, and one inner implication per superclass and per method. An omitted method with a default becomes `$dmop @tys (dfun tys ds)`. A method body sees no "self" Given: `C (T a)` inside it is solved by ordinary instance lookup. |
| Superclass soundness | `Tc/TyCl/Instance.hs:1576` Note [Recursive superclasses], :1700 Note [Solving superclass constraints] | Superclass Invariant: every superclass field is non-bottom. A superclass witness may come only from (sc1) a dictionary argument, (sc2) a superclass of a non-bottom dictionary that is Paterson-smaller than the instance head, (sc3) a dfun application. |
| Recursive dictionaries | `Tc/Solver/InertSet.hs:506` Note [Example of recursive dictionaries]; `Tc/Solver/Monad.hs:529` `updSolvedDicts` | A dictionary solved by an instance is cached before its sub-goals are solved, so evidence bindings may be mutually recursive. |
| Desugaring | `HsToCore/Binds.hs:264` `dsAbsBinds`, :1584 `dsHsWrapper`, :1637 `dsEvBinds`, :1722 `dsEvTerm` | Binding: type lambdas, then dictionary lambdas, then the evidence bindings as a `let` (recursive where needed), then the body. |
| Top level | `Tc/Solver.hs:131` `simplifyTop`; `Tc/Solver/Default.hs:947` `applyDefaultingRules` | Unsolved Wanteds are reported once, at the end of the module, after defaulting. |

## 3. Decisions

- D1. A predicate is an arena type of kind `Constraint`. Add one kind constant and register each class as a declared constructor of kind `Type -> Constraint`. A view function returns class identity and argument. Reason: instantiation, zonking and the free-variable walk already exist for types, and constructor classes will need the kind check. Do not build a second substitution path over a separate predicate struct.
- D2. Evidence terms are in A-normal form: every argument is an evidence variable. This follows `chooseInstance`, and it is what makes scoping checkable and recursion expressible.
- D3. One implication scope owns its skolems, its Givens and its evidence bindings. A Wanted's binding is always written into the scope that emitted the Wanted, whichever scope's Given solves it.
- D4. Evidence abstraction keeps the current per-binder style. Each member of a group is abstracted over the group's type variables and Givens, and a recursive reference re-applies the group's own variables and Givens. GHC builds one tuple instead; the semantics are the same and sharing of the inner closure is lost. Record this in the implementation map.
- D5. Unsolved dictionary Wanteds are reported only at module end. Intermediate solves are opportunistic and never report.
- D6. Signature and class variables are `RigidIdChirho`. Inferred binders may stay frozen metas. The only requirement is one predicate `is_flexible` (an unfilled, non-generalized meta) used everywhere a rule asks "could this still change". Converting inferred binders to rigids at quantification, as GHC's `skolemiseQuantifiedTyVar` does, would remove the second representation; it is preferred but not required for soundness.
- D7. Instance methods are elaborated inline in the dfun. GHC's separate `$c` bindings exist for RULES and specialisation only.

## 4. Carriers

```rust
// haskelujah-tc-chirho
pub struct EvVarChirho(u32);            // module-unique; one binding site

pub enum EvTermChirho {
    VariableChirho(EvVarChirho),
    InstanceChirho {                    // GHC evDFunApp
        instance_chirho: InstanceIdChirho,
        type_arguments_chirho: Vec<TyChirho>,
        dictionary_arguments_chirho: Vec<EvVarChirho>,
    },
    SuperclassChirho {                  // GHC evSelector on a superclass selector
        class_chirho: NameIdChirho,
        index_chirho: u32,              // position in the class's declared superclass list
        argument_chirho: TyChirho,
        dictionary_chirho: EvVarChirho,
    },
}

pub struct WantedDictChirho {
    pub predicate_chirho: TyChirho,     // kind Constraint
    pub evidence_chirho: EvVarChirho,
    pub scope_chirho: ScopeIdChirho,
    pub span_chirho: SpanChirho,
    pub origin_chirho: WantedOriginChirho,   // OccurrenceChirho | SuperclassChirho { naked_chirho } | InstanceContextChirho
}

pub struct GivenDictChirho {
    pub predicate_chirho: TyChirho,
    pub evidence_chirho: EvVarChirho,
    pub superclass_depth_chirho: u32,   // 0 = written or quantified; n = n selections away
    pub blocked_chirho: bool,           // always false in units A to C; see section 9
}
```

`ScopeChirho` gains `kind_chirho` (group, signature, instance, method, superclass, default method), `skolems_chirho`, `givens_chirho` and `evidence_bindings_chirho: Vec<(EvVarChirho, EvTermChirho)>`. `given_equalities_chirho` stays false for every scope in this design.

`SchemeChirho` gains `context_chirho: Vec<TyChirho>` between the binders and the body. `instantiate_chirho` must accept a rigid binder as well as a generalized meta, and returns the instantiated context beside the type arguments.

Declarations, held by the checked module and keyed by identity:

```rust
pub struct ClassInfoChirho {
    pub name_chirho: NameIdChirho,
    pub variable_chirho: RigidIdChirho,
    pub superclasses_chirho: Vec<TyChirho>,          // predicates over the class variable, declared order
    pub methods_chirho: Vec<MethodInfoChirho>,       // declared order: name, scheme, optional default binding
}
pub struct InstanceInfoChirho {
    pub class_chirho: NameIdChirho,
    pub binders_chirho: Vec<RigidIdChirho>,
    pub context_chirho: Vec<TyChirho>,
    pub head_chirho: TyChirho,                       // the class argument, over the binders
    pub scope_chirho: ScopeIdChirho,
}
```

The instance environment maps a class to its instances. Lookup is a one-way match returning `NoMatchChirho`, `UniqueChirho(instance, substitution)` or `StuckChirho`. With Haskell 2010 heads the match is "same head constructor, bind the arguments in order", but write it as a match so later head forms change the rule and not the carrier.

The checked module replaces `instantiations_chirho` with two records, and Core reads only these:

```rust
pub enum WrapStepChirho { TypeChirho(TyChirho), EvidenceChirho(EvVarChirho) }
pub wrappers_chirho: HashMap<ExprIdChirho, Vec<WrapStepChirho>>,     // occurrence: applications in order

pub struct AbstractionChirho {
    pub type_binders_chirho: Vec<TyChirho>,
    pub dictionary_binders_chirho: Vec<EvVarChirho>,                 // the scope's depth-0 Givens, in order
    pub scope_chirho: ScopeIdChirho,                                 // its evidence bindings wrap the body
}
pub abstractions_chirho: HashMap<NameIdChirho, AbstractionChirho>,
```

A step list rather than two flat vectors is deliberate: a method with its own type variables needs `op @a d @b d2`, and Core should stay lintable.

HIR additions: a class declaration (variable, superclass assertions, method signatures, default bindings), an instance declaration (variables, context, head, method bindings), and a signature with a context and its implicitly quantified variables in first-occurrence order. Two points matter for soundness:

- In an instance, the method binding's left-hand name resolves to the class's method (GHC `lookupInstDeclBndr`). It introduces no name. An occurrence of that name inside the body is the ordinary selector. gpt_chirho's `InstanceMethodScopeChirho` fixture measures this.
- Each binding carries `restricted_chirho`, computed from the source form: a function binding with zero argument patterns and no signature. `f = \x -> e` is restricted even though its body is a lambda. It cannot be recovered from the HIR body.

## 5. Generating constraints

- Occurrence of any name with a scheme: instantiate, then for each context predicate allocate an `EvVarChirho` and emit a Wanted in the current scope. Record the wrapper: type steps, then evidence steps, in scheme order. A method selector is not special.
- A reference to a member of the group being inferred emits no Wanted. Its wrapper is filled when the group closes: the group's type binders, then the group's Givens. This is what GHC's monomorphic recursive Id means in the per-binder style.
- Signature binding: open a signature scope one level deeper. Its skolems are the signature's variables and its Givens are the context, one `EvVarChirho` each. Infer the body in that scope and want `inferred ~ signature body`. A signed binding is always its own group.
- Instance and default-method bodies are checked after all top-level value groups, as GHC orders `tcInstDecls2` after `tcTopBinds`. Class headers, method schemes and instance heads are registered before any value group.

## 6. Solving

Equalities run to a fixpoint first. Dictionary solving never fills a meta in this design, so it runs after them and needs no interleaving. Defaulting, when it arrives, fills metas and re-enters both.

For a Wanted `C t`, with `t` zonked:

1. Identical inert. Walk the Wanted's scope and its ancestors for a Given with the same class identity and the same zonked argument. Prefer depth 0, then the shallowest superclass path, then context order. Bind `evidence := VariableChirho(given)`. Within the same scope, an earlier Wanted with the identical predicate may be reused the same way. Superclass Givens are produced on demand: when no direct Given matches, search the superclass closure of the Givens at `t` and write one `SuperclassChirho` binding per selection step into the Wanted's scope. The closure is finite because class cycles are rejected.
2. If the head of `t` is flexible, stop: the Wanted is stuck. No instance may be chosen for an unfilled meta. This is also all that rule IL0 requires while Givens are on bare type variables.
3. If the head of `t` is a rigid or generalized variable, only a Given can solve it. Leave it unsolved.
4. If the head is a type constructor, look up the instance. On a unique match, allocate a Wanted in the same scope for each instantiated context predicate and bind `evidence := InstanceChirho { .. }`. A child of a naked superclass Wanted is not naked. On no match, leave it unsolved; that never changes.

Two predicates are identical when class identity matches and the zonked arguments are the same interned node. `replace_chirho` with an empty map is that zonk.

## 7. Closing an inferred group

After the group's equalities are solved, for the group scope S at outer level N:

1. Candidates are the unsolved Wanteds whose scope is S or a descendant of S, and whose predicate mentions no skolem bound by a scope strictly between S and the Wanted.
2. If the group is restricted, set the level of every flexible meta free in a candidate to N and pick nothing. The candidates stay where they are. A later use may fix the variable; otherwise module end reports it.
3. Quantify as today: metas above N reachable from the members' types.
4. Otherwise pick the candidates that mention a quantified variable. A picked predicate must be headed by a type variable. One headed by a constructor is Unsupported, not an error: GHC2021 would quantify it.
5. Remove a picked predicate that is a transitive superclass of another picked predicate at the same argument. Order the rest by first emission.
6. Add each survivor to S as a depth-0 Given with a fresh `EvVarChirho`, record the abstraction for every member, and run the solver again. The picked Wanteds, and any nested inside inner scopes, are now solved by step 1 of section 6. Their bindings land in their own scopes.
7. An unpicked candidate stays. This is the measured "captured constraint floats" case: `let inner _ = render outer` leaves `Render alpha` unsolved in the inner scope until the enclosing function quantifies `alpha`.
8. A group with two or more members, where a picked predicate's variable is not free in every member's type, needs GHC's impedance matching. Answer Unsupported until section 9's unit D.

At module end: solve once more; any remaining dictionary Wanted is an error. One whose argument still holds a flexible meta is the ambiguity report; the others are "no instance". Both are GHC-39999 in 9.14.1. `finish_metas_chirho` runs only after this check reports nothing. A dictionary must never be looked up at the `Any` image.

## 8. Declarations and Core

Class `C` with superclasses `S1..Sm` and methods `op1..opn`:

- Dictionary constructor with `m + n` lazy fields, superclasses first. Its key uses the existing role-tagged constructor identity with a dictionary role.
- One selector binding per field: `\d -> case d of C:C f1 .. f(m+n) -> fi`. Method selectors bind the method's own name; superclass selectors get fresh names (`$p1C` in GHC).
- Default method `$dmop = /\a. \this. letrec <scope evidence> in body`, checked in a default-method scope whose only Given is `this :: C a`.
- Checks: superclass cycle (GHC-29210); every method type mentions the class variable.

Instance `instance ctx => C (T as)` with scope I:

```
$fCT = /\as. \gs.
         letrec <evidence of I>
         in C:C (letrec <evidence of SC_1> in sc_1) ..
                (letrec <evidence of M_1> in body_1) ..
```

- Each superclass field is a naked superclass Wanted `S_k (T as)` in its own child scope of I.
- Each written method is checked in its own child scope against the method type at `T as`.
- An omitted method with a default is `$dmop @(T as) self`, where `self` is an `InstanceChirho` term for this instance at its own binders and Givens, bound in I. Do not tie a knot through a self dictionary, and never add self as a Given.
- An omitted method without a default is Unsupported until Core has a named failure for it. GHC only warns.
- Checks: duplicate head (GHC-59692); method not in the class (GHC-46284).

Evidence terms in Core: a variable is a Core variable; `InstanceChirho` is the dfun applied to its types then its dictionaries; `SuperclassChirho` is the superclass selector applied to the argument type then the dictionary.

Every binding is `/\tvs. \dicts. letrec <evidence of its scope> in body`. All evidence bindings of one scope form one recursive `let`, and a dictionary argument is never forced when a dictionary is built.

Because a group's Givens and scope evidence are copied into every member, the elaborator allocates Core ids for evidence variables at each binding site through a scoped map. Looking up an evidence variable that is not in that map is an internal invariant failure, reported as Invalid. It is never Unsupported and never repaired by a fallback.

A restricted top-level binding elaborates to `alias = letrec d = $fCBool in op @Bool d`. It is a CAF, so the sharing the restriction exists for is kept.

Expected shape for two measured fixtures:

```
-- PolymorphicDispatchChirho
applyChirho = /\a. \g. \value. renderChirho @a g value
resultChirho = letrec d1 = $fRenderBool; d2 = $fRenderChar in
               case applyChirho @Bool d1 True of { 'b' -> applyChirho @Char d2 'q'; _ -> '!' }

-- CapturedConstraintChirho
captureChirho = /\a. \g. \outer.
                letrec inner = /\b. letrec d = g in \ignored. renderChirho @a d outer
                in inner @Bool True
```

## 9. The smallest sound vertical and the order after it

- Unit A, the twelve measured fixtures. Classes with no superclass and no default; instances at heads with no type variables; inferred groups with constraint generalization, the restriction and floating; no signature other than the method signatures in a class. It needs sections 4 to 7 without skolem scopes. Everything else is Unsupported.
- Unit B. Polymorphic signatures with contexts on type variables; instances `C (T a1 .. an)` with contexts on those variables. This adds skolem scopes and written Givens. `quantify_group_chirho` must then treat a rigid from an enclosing signature as a constant.
- Unit C. Superclasses and default methods.
- Unit D. Numeric literals and defaulting, then export minimization for groups (impedance matching needs defaulting to be faithful).
- Unit E. Methods with their own type variables and contexts, and class variables of higher kind. `Functor` and `Monad` need both, and so do `RealFrac` and `Semigroup`.
- Unit F. Flexible heads and contexts, overlap rules IL1 to IL6 with potential unifiers, the full Given-overlap guard, the solved-dictionary cache, and deriving.

Why units A to C select evidence soundly, and what each argument depends on:

1. One instance at most. Heads are a constructor applied to distinct variables and duplicates are rejected, so a type has at most one matching instance, and the match is made only once the head constructor is known. Later unification cannot change it.
2. A Given matches by identity. Its argument is a rigid or frozen variable, compared as an interned node together with the class identity. No name text is involved.
3. Givens and instances cannot compete: a Given's argument is a variable and an instance head is a constructor application.
4. A superclass is selected by its position in the class's own list.
5. The Superclass Invariant holds without a size check. Every instance-context Given is on a bare head variable, so it and all of its superclasses are Paterson-smaller than the head. Rule (sc2) therefore never blocks. This depends on Unsupported being answered for every other context form. The `naked_chirho` and `blocked_chirho` fields exist from unit A so that unit F changes a rule and not a carrier.
6. Solving terminates and builds no cyclic evidence: each instance step replaces `C (T ts)` by predicates on strictly smaller types. The solved-dictionary cache is therefore not needed before unit F, where it becomes required.
7. Evidence is closed. A term refers only to variables bound in its own scope or an ancestor, or to a top-level dfun. The elaborator's scoped map checks this mechanically.
8. Nothing ambiguous is chosen: a flexible head is never matched against an instance, and the `Any` image is applied only after no dictionary Wanted remains.

## 10. Cases that expose wrong evidence selection

Names follow the fixtures: `RenderChirho` with `renderChirho :: a -> Char`. The twelve fixtures already cover missing instance, ambiguity, wrong class, method mismatch, restricted alias, duplicate instance, monomorphic use, polymorphic dispatch, captured constraint, mutual recursion, method scope and single-instance generalization. These are the gaps.

Unit A:

| Case | Program core | Expected | A wrong implementation would |
| --- | --- | --- | --- |
| Two variables, one class | `pair x y = (renderChirho x, renderChirho y)`; `pair True 'q'` | accept, `('b','q')` | key evidence by class alone and pass one dictionary twice |
| Shadowed method name | `applyLocal renderChirho x = renderChirho x`; `applyLocal (\u -> 'z') UnitChirho`, where `UnitChirho` has no instance | accept, `'z'` | demand a dictionary for a lambda-bound name |
| Result-directed selection | `class DefChirho a where defChirho :: a`, instances at `Bool` and `Char`; `(defChirho, defChirho) :: (Bool, Char)` | accept, both instances | dispatch on an argument that does not exist |
| Nested local generalization | `let h x = renderChirho x in (h True, h 'q')` | accept | keep `h` monomorphic, or give both uses one dictionary |
| Captured and own constraint together | `outerFn o = let h x = (renderChirho x, renderChirho o) in h True` used at two types | accept | quantify the outer predicate at `h`, or leave its evidence unbound |
| Restricted local binding | `g x = let y = renderChirho x in y` used at two types | accept | quantify at `y`, or lose the binding's scope |
| Restricted, never fixed | `alias = renderChirho` with no use | reject, GHC-39999 | pick an instance, or use `Any` |
| Unreachable variable | `v = renderChirho (defChirho)` with instances for both classes | reject, GHC-39999 | pick an instance |
| Group needing export minimization | `p x b = if b then renderChirho x else q b`; `q b = p loop b` with `loop = loop` | Unsupported in ours; GHC rejects as ambiguous | accept with an unconstrained dictionary |

Units B and C:

| Case | Program core | Expected | A wrong implementation would |
| --- | --- | --- | --- |
| Two classes, argument order | `f :: (FirstChirho a, SecondChirho b) => a -> b -> (Char, Char)` with one-method classes of the same shape | accept; values tell the dictionaries apart | swap the two dictionaries silently |
| Inner signature reusing a variable name | `f :: RenderChirho a => a -> Char; f x = let g :: RenderChirho a => a -> Char; g y = renderChirho y in g 'q'`; `f True` | accept, `'q'` | key Givens by variable text and pass the outer dictionary at the wrong type |
| Signature too general | `f :: a -> Char; f x = renderChirho x` | reject, GHC-39999 | quantify silently |
| Ambiguous signature | `f :: RenderChirho a => Char` | reject, GHC-39999 | accept and choose at the use |
| Givens do not block outer unification | `outerFn y = let g :: RenderChirho a => a -> Bool; g x = y in g` | accept | report an untouchable variable |
| Compound dictionary | `instance RenderChirho a => RenderChirho [a]`; use at `[[Bool]]` | accept | build the wrong nesting |
| Method needing its own instance | the list instance's method calls `renderChirho` on the tail | accept, terminates | add a self Given |
| Superclass field index | `class (ACh a, BCh a) => CCh a` with two methods; a function with only `CCh a` calls all four | accept; four distinct values | select a method where a superclass is stored |
| Two-step superclass | three classes in a chain; the top method through the bottom Given | accept | stop after one selection |
| Missing superclass instance | `class ACh a => BCh a`; `instance BCh Bool` with no `ACh Bool` | reject, GHC-39999 | build a dictionary with a bottom superclass |
| Superclass needs context | `instance BCh [a]` with no context, where `ACh [a]` needs `ACh a` | reject, GHC-39999 | take the superclass from nowhere |
| Default uses this instance | `q x = notCh (p x)` as a default; two instances defining only `p` differently | accept; values differ per type | bind the default to one instance's methods |
| Default overridden | an instance defines `q` itself | accept; the instance's `q` | prefer the default |

## 11. Not covered here

Multi-parameter classes and functional dependencies change steps 3 to 5 of `decideAndPromoteTyVars`. Equality superclasses change touchability. Quantified constraints, implicit parameters, `Typeable` and the other built-in solvers, `Coercible`, deriving, and cross-module instance identity are separate designs. Cross-module identity needs the class and instance keys to carry unit and module, as the data constructor keys already do.
