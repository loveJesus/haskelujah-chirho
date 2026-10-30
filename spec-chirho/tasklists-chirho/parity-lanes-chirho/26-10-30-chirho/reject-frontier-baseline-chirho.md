<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Reject-axis frontier audit, 30 October 2026

Read-only review requested by HASKELUJAH/gpt_chirho. Source pin: `75b4e3a9`; current inspected HEAD `a70a11c7` has the identical `crates` tree `2d144db46f2e8477d7fbe4cf6bece3dd5ce1522d`. No Cargo, GHC, rustc, Haskelujah CLI, broker, Git mutation, or DB operation was run for this audit. This file is the sole owned scratch output.

The full 514-file accounting is complete. A causal trace for every file is **not** complete. The ledger below distinguishes upstream action, identical-source retained GHC verdict, and source-obstacle topic; topic counts are not prospective flip counts. Current compiler roots were checked directly, and the strongest reductions/code homes are recorded below. No published membership or denominator is relabelled.

## The primary accounting

The committed 514 names equal both final candidate pass ACCEPT sets in `tmp-chirho/final-breakthrough-chirho/paired-measurement-chirho/{candidate1-chirho,candidate2-chirho}/should_fail-chirho.tsv`.

| Current upstream all.T action | Files |
| --- | ---: |
| compile_fail | 420 |
| multimod_compile_fail | 27 |
| compile | 28 |
| compile_and_run | 1 |
| multimod_compile | 1 |
| Unlisted/helper or no unique registration | 37 |
| **Total preserved directory-label wrong-accept members** | **514** |

Thus 447 are registered negative actions, 30 are registered positive actions, and 37 are unlisted/helper files. The static inventory is `tmp-chirho/frontier-closure-chirho/upstream-cases-chirho.json`; it preserves literal setups, action arguments, all.T location, and source hashes. all.T was parsed as Python AST, never executed. A directory named should_fail is not itself an oracle that every .hs file must be rejected.

The October 17 retained standalone GHC 9.14.1 table has an entry for all 514. Every current source SHA-256 and .stderr SHA-256 matches that table (zero source differences, zero stderr differences). Therefore its verdicts remain attributable to these exact bytes, but **they are retained runs, not fresh compiler executions in this audit**. That command omitted all.T flags and did not reconstruct the full upstream test driver.

| Retained standalone GHC verdict | Registered negative | Registered positive | Unlisted/helper | Total |
| --- | ---: | ---: | ---: | ---: |
| ACCEPT | 3 | 25 | 22 | 50 |
| REJECT | 444 | 5 | 15 | 464 |
| **Total** | **447** | **30** | **37** | **514** |

The 444 negative-action standalone rejects include six source -Werror tests, which measure warning policy rather than invalid types. A prospective ordinary type-soundness repair set therefore starts with 438 such cases, and still needs each case's actual flags/context checked; this is a planning subset, not a new published denominator. The 15 rejected helper/unlisted files may fail only because of the standalone import/project context.

Protocol exceptions that must not be "fixed" by rejecting valid sources:

- 25 upstream compile/run files and 22 valid helper modules are accepted by the retained GHC run. Their full names are in P01/P02 below.
- `tcfail167.hs` needs all.T's `-Werror`; `T10698.hs` is `expect_broken(10698)`; `T5236.hs` is a compile_fail registration that standalone GHC 9.14.1 accepts.
- Five positive-action files reject under the retained standalone command: `AmbigFDs.hs`, `T18851b.hs`, and `T18851c.hs` have source comments documenting a September 2025 upstream change from rejected to accepted fundep behavior; `T16502.hs` is expected-broken; `T13292.hs` is a multimodule compile with `-fdefer-type-errors`. They are version/harness investigations, not clear reject targets.
- `ContextStack1.hs` has `-freduction-depth=10` in all.T. `T9497d.hs` uses `-fdefer-type-errors -fno-defer-typed-holes`. Global "defer everything" or "reject every hole" rules would violate these distinctions.
- The six source -Werror files are `MissingExportList03.hs`, `T14761a.hs`, `T3966.hs`, `T3966b.hs`, `TyAppPat_ScopedTyVarConflict.hs`, and `tcfail204.hs`.

## Current architectural roots and suggested repair order

Unless a crate is explicitly named, checker source paths below are relative to `crates/haskelujah-typing-chirho/src/`. Parser lowering paths are relative to `crates/haskelujah-parser-chirho/src/`.

### 1. Start with bounded declaration validation, then typed superclass obligations

The most isolated first repair is a parent/constructor validator before registering data/newtype constructors. `infer_chirho/declarations_chirho/module_chirho.rs:366` takes a GADT constructor's entire written signature and registers it without requiring its return head to be the declared parent. For example `T13300.hs` declares `data W where WI :: Int; WD :: Double`; both constructors are not constructors of W. Other source witnesses: `T7175.hs`, `tcfail155.hs`, `tcfail176.hs`, `T18357.hs`, `T18357b.hs`. These six are registered negatives; GHC's syntactic parent rule must be followed before alias/family normalization (the W alias witness is intentional). No candidate flip count is claimed.

At `module_chirho.rs:384-425`, ordinary newtype field types are collected and passed to fun_n with no exact-one-field proof. `T15523.hs` has no field; `T23308.hs` has two; `T2126.hs` has no constructor. `T15796.hs` and `T17955.hs` have newtype contexts; `tcfail156.hs` has an existential; `T18891a.hs` and `T21447.hs` are representation-polymorphic GADT newtypes. Keep these as separate validity predicates instead of a blanket ban on GADT-style newtypes. `validity_chirho.rs:131-207` currently licenses rank/qualified types and checks unlifted newtype fields; it is not a full declaration validator. Put new checks in a category module rather than growing that near-limit file.

The highest-leverage class root is equally concrete but requires typed metadata: `infer_chirho/declarations_chirho/instance_obligations_chirho.rs:157` calls only `report_duplicate_instances_chirho`. It never proves an instance's superclass context. `class_instance_chirho.rs:60-67` stores only superclass **names** in Vec<String>, dropping typed argument applications and MPTC parameter positions. A names-only patch is unsound.

Small closed witness, copied structurally from `tcfail106.hs`:

```haskell
class S a
class S a => C a where opc :: a -> a
class S b => D b where opd :: b -> b
instance C Int where opc = opd
instance D Int where opd = opc
```

GHC rejects missing S Int. Both method bodies can find the other instance, so method-body checking cannot substitute for instance-superclass validity. `tcfail109.hs` is the critical MPTC control: `class Eq ce => Collects e ce`; the obligation constrains the SECOND class parameter. `tcfail017`, `019`, `020`, `042` exercise applied heads and contexts; T20666/a/b exercise family-shaped superclasses. The 14 C01 members are an inspected source family, not a promise of 14 flips.

A safe design preserves typed superclass predicates and class parameter order; substitutes the instance head; judges obligations from context with explicit proven/refuted/deferred outcomes. Preserve open imported-class uncertainty and prohibit a self-generated dictionary from proving its own superclass. `tcfail223` and `T22891` are cycle/size traps; accepted `LoopOfTheDay1/2/3`, `LocalGivenEqs`, and `T20996` are required holds. Do not turn depth exhaustion into proof.

### 2. Proof obligations are still lost at open/ground and generalization boundaries (reserved row662)

`solving_chirho/schemes_chirho.rs:254-283` partitions constraints using only the primary argument's free variables. A ground primary argument passes that test vacuously. Current code DOES reject certainly-unsolvable closed-ground predicates before storing them, so the old claim "all ground predicates are ignored" is false. Solvable ground predicates such as Num Int still remain in schemes, and extra MPTC variables are omitted from that placement test.

`context_chirho.rs:385-394` replaces the seeded class environment with an imported environment and clears the entire fully_known_class_names set. `validity_chirho` then cannot issue the same closed-world refusal for an imported type/class universe. This is the previously measured two-module Cell-with-no-Num acceptance from row668: `five :: Cell; five = 5` can be absorbed rather than rejected when the universe is marked open. This audit did not rerun that program.

`module_chirho.rs:676-725` generalizes then replaces the inferred scheme with a signature scheme after type subsumption. Type subsumption alone is not proof that the signature givens entail all wanted class predicates. `deferred_predicates_chirho.rs:306-346` still skips ambiguous bare variables and classes with no usable instance inventory, after its correct closed-ground check. `class_chirho.rs:383-415` returns true for depth>64 or any unresolved type variable, conflating deferred with proven.

Repair direction: visible-instance proof for fully ground goals, eliminate PROVEN ground goals from schemes, diagnose only REFUTED goals when the universe is authoritative, keep UNKNOWN deferred with ownership and span. Include all MPTC argument variables in placement. Preserve dictionary/evidence consumers; do not simply erase all ground predicates or globally flip optimistic entails to false.

### 3. Kind sorts, applications and synonym saturation

`kind_chirho.rs:241-244` unconditionally unifies Type/Star with Constraint. `kind_chirho/conversion_chirho.rs:36-46` walks a class constraint's arguments but never looks up/applies the class kind or checks its arity. Source witnesses include `T1633.hs` (Functor Bool), `T11464.hs` (Eq(Either a)), `T3540.hs` (equality constraint in a value-type position), and `tcfail146.hs` (class constraint used as a field type). The 101 old kind-family cases include separate runtime representation, kind-variable rigidity, explicit binder visibility, and extension licensing problems; they are not all one unifier fix.

`kind_chirho.rs:729` returns None for an underapplied synonym without a saturation diagnostic. `tcfail100.hs` is the simple witness: `type A i = i; type B = A`. `tcfail101`, `107`, `129`, and `T15954` expose higher-order uses. A separate saturation validator must honor LiberalTypeSynonyms expansion context; do not reject every partially applied intermediate AST node blindly.

Something is wrong with the size of `kind_chirho.rs` (over 3,000 lines). A mechanical category split should precede a broad checker extension, with the existing same-output proof discipline. A bounded validator can live in a child module.

### 4. Family declaration information is erased before solving

`haskelujah-ast-chirho/src/decl_chirho.rs:235-246` has no injectivity field on TypeFamilyDecl. The lowerer `lower_chirho/classes_chirho/type_families_chirho.rs:146-174` detects `= result | result -> ...` only to avoid treating annotation names as parameters; it never preserves the annotation. Thus `T10836.hs`'s `Foo Int=Int; Foo Bool=Int` violating result->argument injectivity cannot be checked by inference. F04's five files need metadata plus validity, not only a solver edit.

`infer_chirho/types_chirho/type_families_chirho.rs:19-63` stores equation vectors, putting newer registrations first. It does not validate equation compatibility or distinguish why an overlap is legal. F05's T13981A/B/C and UnliftedNewtypesOverlap exercise imported conflicting equations/data family instances; helper B/C must not be judged in isolation as standalone negative tests.

Associated defaults: `AssocTypeFamilyChirho` carries name/type-variable names/default RHS/default binder names, but no data-versus-type family category. `class_instance_chirho.rs:177-196` silently falls back to declared parameters when default binder arity differs. F03's 10 sources separately cover wrong class member, missing family declaration, category mismatch, non-variable/duplicate binders, wrong arity, duplicate defaults, and an unknown default method. A bounded associated-declaration validator can address these independently of injectivity. F06's two wrong LHS-family names also need the written equation head preserved/validated; do not guess it from the enclosing declaration.

### 5. Non-GADT failures must not disappear as GADT leniency

Current `terms_chirho/expressions_chirho.rs:450-468` discards case-alternative result unification errors; `:530-539`, `:549-556`, and `:587-595` discard do-statement shape failures. `terms_chirho/matches_chirho.rs:157` discards per-equation argument unification failure. `bindings_chirho.rs:40/62` discards list-comprehension guard/list-shape failure.

Actual witnesses: `tcfail013.hs` is `f []=1; f True=2`; `tcfail002.hs` is `c(x:y)=x; c z=z`; T7851/tcfail168 place bare print/putChar as a non-last do statement before a genuine IO action. These have no GADT refinement that could justify incompatible shapes. T13311 is the rank-N function version of the do error.

Report real inconsistency on ordinary paths, preserving branch-local GADT refinement and RebindableSyntax/QualifiedDo's selected operators. Do NOT change `do_operations_chirho.rs:99`'s Err(_) evidence-capture arm: it is intentionally observational and does not compose an inference substitution. The historical bucket misclassified tcfail002 as skolem-related; it is moved to U01 here.

### 6. Rank/rigidity, roles, and naming need explicit semantic checks

`unify_chirho.rs:254-266` strips a Forall wrapper against a concrete type, letting bound variables participate as ordinary variables. `subsume_chirho:360-456` instantiates expected foralls flexibly, rather than a full bidirectional skolem discipline. Signature body checking now skolemizes many top-level/local cases; do not resurrect the old claim that all signatures are flexible. `tcfail104.hs` and `T21530b.hs` are concrete rank/subsumption witnesses; general higher-rank TyChirho also has no qualified-predicate field. A proof-preserving rank-N representation/checker is a separate repair from row675's PatBind-only annotations.

Role declarations/newtype constructor visibility are not represented by the class solver. The Coercible seed is actually reflexivity (a,a), not a universal (a,b) instance: an old comment claiming universal acceptance is stale. Repeated variables are consistently matched now. The unresolved problems are role/representation proof, abstraction, and where magic-class goals are deferred. Source witnesses TcCoercibleFail/TcCoercibleFail3/T23731/T22645 exercise distinct roles and constructor scope; never hardcode coerce rejection by syntax.

Naming: `expressions_chirho.rs:95-103` emits only a warning for every `_`/`_name` expression, without consulting defer-typed-holes. `:148-174` returns a fresh type for an unbound expression constructor without a diagnostic. N01 has nine holes; N07 has unbound B in term/pattern positions. `T12947` is particularly important: deferring typed holes must NOT defer an ordinary unbound constructor. N03's seven cases concern Main/export/ambiguity; N04's four duplicate type binders, N05's unhidden local+ versus Prelude+ ambiguity, and N06/N08/N09 need their own declaration/scope checks. The entire 27-case naming topic is subdivided below.

## Confidence and non-claims

- Source and oracle metadata, the full current membership, both retained candidate pass sets, and all hashes were checked. No fresh compiler execution or prospective changed-verdict measurement was performed.
- The 101 miscellaneous declaration sources and 25 family-declaration sources were read and regrouped by their actual declarations, rather than GHC code alone. The 41 miscellaneous kind sources were likewise split into concrete syntax/type categories. Representative sources in the remaining families were followed to current code; the remaining ledger topic labels are retained review indices, not one-to-one causal proofs.
- Historical source mechanisms corrected: current instance methods DO have rigid judgment and a closed-ground leftover check; Coercible seed is reflexive; top-level signature checks DO use skolems; the ground checker DOES reject authoritative closed misses. Each still has explicit support/open-context boundaries. Old counts cannot prove those mechanisms still explain every member.
- Exactly 18 of the October17 532 members have left the current list; none were added. All514 current source/stderr bytes match that oracle table. This reduces stale evidence risk but does not replace a fresh upstream-driver run.
- No acceptance-only change, diagnostic suppression, directory-label relabelling, or blanket unknown=>reject shortcut is recommended. Each source repair needs fresh GHC oracle controls plus both published axes, including valid-source losses and supported execution paths.

## Disjoint source-obstacle index

This is an exact inventory partition, not an exact causal partition. Negative means upstream compile_fail/multimod_compile_fail. Positive/helper entries remain visible and must not be rejected merely to increase the preserved directory-label score.

| Topic | Files | Upstream negatives |
| --- | ---: | ---: |
| C01 instance superclass obligations | 14 | 14 |
| C02 functional dependencies | 12 | 12 |
| C03 ambiguity / predicate placement | 29 | 27 |
| C04 signature givens versus wanteds | 7 | 7 |
| C05 unresolved class predicates | 22 | 21 |
| C06 implicit-parameter constraints | 3 | 3 |
| C07 instance overlap selection | 6 | 6 |
| C08 solver termination budget | 2 | 2 |
| C09 TypeError / KnownNat / family evaluation | 8 | 8 |
| D01 hs-boot interface consistency | 15 | 6 |
| D02 GADT parent / newtype constructor validity | 14 | 14 |
| D03 deriving eligibility | 2 | 2 |
| D04 record declaration / update / selector validity | 6 | 6 |
| D05 default method signature / body validity | 5 | 5 |
| D06 termination and superclass-cycle declaration rules | 13 | 13 |
| D07 class / instance head and context validity | 17 | 17 |
| D08 disabled extension syntax / type licensing | 11 | 11 |
| D09 type application / abstraction binder validity | 10 | 10 |
| D10 strict / unlifted binding validity | 6 | 6 |
| D11 pattern synonym / forbidden binding shape | 4 | 4 |
| D12 malformed syntax accepted after recovery | 3 | 3 |
| D13 pragma binding / signature validity | 4 | 4 |
| D14 defaulting / monomorphism restriction validity | 2 | 2 |
| D15 GHC tuple-size limit | 2 | 2 |
| D16 promotion with unpromotable context | 1 | 1 |
| F01 roles / representation / abstraction | 15 | 14 |
| F02 type equality / given mismatch | 4 | 2 |
| F03 associated default or class-member declaration | 10 | 10 |
| F04 missing injectivity metadata / validation | 5 | 5 |
| F05 family equation overlap / cross-module conflict | 4 | 2 |
| F06 family equation names wrong family | 2 | 2 |
| F07 family forall binders unused in LHS | 2 | 2 |
| F08 associated declaration kind-binder identity | 2 | 2 |
| K01 kind sort, arity, and class application | 40 | 40 |
| K02 levity / runtime representation | 13 | 13 |
| K03 rigid and polymorphic kind variables | 7 | 7 |
| K04 declaration inferred/specified binder visibility | 5 | 5 |
| K05 visible kind application arity / visibility | 6 | 6 |
| K06 datatype / newtype return representation kind | 10 | 10 |
| K07 uninferrable kind variable | 5 | 5 |
| K08 constraints in kind | 2 | 2 |
| K09 NoDataKinds licensing | 6 | 6 |
| K10 unsaturated type synonym | 5 | 5 |
| K11 synonym implicit binder arity | 1 | 1 |
| N01 typed-hole diagnostic policy | 9 | 9 |
| N02 unbound constructor with deferred holes | 1 | 1 |
| N03 Main / export / imported-name ambiguity | 7 | 7 |
| N04 duplicate type-abstraction binder | 4 | 4 |
| N05 source versus Prelude ambiguity | 1 | 1 |
| N06 noncontiguous duplicate method equations | 1 | 1 |
| N07 unbound expression/pattern constructor | 2 | 2 |
| N08 duplicate parallel-comprehension binder | 1 | 1 |
| N09 class method mistaken for record field | 1 | 1 |
| P01 upstream positive / retained GHC accepts | 25 | 0 |
| P02 valid helper / retained GHC accepts | 22 | 0 |
| P03 harness flag, broken-test, or oracle drift | 3 | 3 |
| P04 source Werror diagnostic policy | 6 | 6 |
| R01 higher-rank subsumption | 7 | 7 |
| R02 rigidity / existential / equation consistency | 19 | 17 |
| R03 impredicativity / forall licensing | 23 | 23 |
| U01 equation / pattern consistency | 3 | 3 |
| U02 do / guard / case consistency | 4 | 4 |
| U03 expression, signature, or family consistency | 13 | 12 |
| **Total** | **514** | **447** |

## Complete 514-member ledger

All paths in this table are under `ghc-tests-chirho/typecheck-chirho/should_fail/`. The retained GHC column is the October17 identical-source standalone run, not a fresh oracle. all.T locations and flags in the shared JSON are authoritative over this compact action column. `—` means no stderr diagnostic code, not no source issue. Every file remains an ACCEPT in both current retained candidate passes.

| File | all.T action | Retained GHC | Topic | stderr first code |
| --- | --- | --- | --- | --- |
| AmbigFDs.hs | compile | REJECT | R02 rigidity / existential / equation consistency | — |
| AssocTyDef01.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef02.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef03.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef04.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| AssocTyDef05.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef06.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef07.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef08.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| AssocTyDef09.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| ClassOperator.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| CommonFieldResultTypeMismatch.hs | compile_fail | REJECT | D04 record declaration / update / selector validity | — |
| CommonFieldTypeMismatch.hs | compile_fail | REJECT | D04 record declaration / update / selector validity | — |
| ContextStack1.hs | compile_fail | REJECT | C08 solver termination budget | — |
| ContextStack2.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| CustomTypeErrors01.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| CustomTypeErrors02.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| CustomTypeErrors03.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| CustomTypeErrors05.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| DataToTagFails.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| ExplicitSpecificity1.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| ExplicitSpecificity2.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| ExplicitSpecificity5.hs | compile_fail | REJECT | K04 declaration inferred/specified binder visibility | — |
| ExplicitSpecificity6.hs | compile_fail | REJECT | K04 declaration inferred/specified binder visibility | — |
| ExplicitSpecificity7.hs | compile_fail | REJECT | K05 visible kind application arity / visibility | — |
| ExplicitSpecificity8.hs | compile_fail | REJECT | K04 declaration inferred/specified binder visibility | — |
| FD2.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| FD3.hs | compile_fail | REJECT | C02 functional dependencies | — |
| FDsFromGivens.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| FailDueToGivenOverlapping.hs | compile_fail | REJECT | C07 instance overlap selection | — |
| LazyFieldsDisabled.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| LevPolyBounded.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |
| LongWayOverlapping.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| MissingDefaultMethodBinding.hs | compile_fail | REJECT | D05 default method signature / body validity | — |
| MissingExportList03.hs | compile_fail | REJECT | P04 source Werror diagnostic policy | — |
| MultiAssocDefaults.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| OrdErr.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| PatSynArity.hs | compile_fail | REJECT | D11 pattern synonym / forbidden binding shape | — |
| PatSynUnboundVar.hs | compile_fail | REJECT | D11 pattern synonym / forbidden binding shape | — |
| SilentParametersOverlapping.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| SpecPragmasFail.hs | unlisted/multiple | REJECT | U03 expression, signature, or family consistency | — |
| StrictBinds.hs | compile_fail | REJECT | D10 strict / unlifted binding validity | — |
| T10194.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T10285.hs | multimod_compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T10285a.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T10351.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T10495.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| T10534.hs | multimod_compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T10534a.hs | unlisted/multiple | REJECT | F01 roles / representation / abstraction | — |
| T10619.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T10698.hs | compile_fail | ACCEPT | P03 harness flag, broken-test, or oracle drift | — |
| T10715.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T10715b.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T10836.hs | compile_fail | REJECT | F04 missing injectivity metadata / validation | — |
| T10971b.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T10971c.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T10971d.hs | multimod_compile_fail | REJECT | U02 do / guard / case consistency | — |
| T11355.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T11356.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T11464.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T11514.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T11563.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T11623.hs | compile_fail | REJECT | F06 family equation names wrong family | — |
| T11672.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T11698.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| T11947a.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T11948.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| T11974b.hs | compile_fail | REJECT | D14 defaulting / monomorphism restriction validity | — |
| T11990a.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| T11990b.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| T12035.hs | multimod_compile_fail | REJECT | D01 hs-boot interface consistency | — |
| T12035a.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T12042a.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T12045b.hs | compile_fail | REJECT | K05 visible kind application arity / visibility | — |
| T12045c.hs | compile_fail | REJECT | K05 visible kind application arity / visibility | — |
| T12063.hs | multimod_compile_fail | REJECT | D01 hs-boot interface consistency | — |
| T12151.hs | compile_fail | REJECT | D05 default method signature / body validity | — |
| T12170a.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| T12177.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T12178a.hs | compile_fail | REJECT | D13 pragma binding / signature validity | — |
| T12373.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| T12430.hs | compile_fail | REJECT | F04 missing injectivity metadata / validation | — |
| T12563.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T12729.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T12837.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T12918a.hs | compile_fail | REJECT | D05 default method signature / body validity | — |
| T12918b.hs | compile_fail | REJECT | D05 default method signature / body validity | — |
| T12947.hs | compile_fail | REJECT | N02 unbound constructor with deferred holes | — |
| T12966.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T13068.hs | multimod_compile_fail | REJECT | D01 hs-boot interface consistency | — |
| T13068a.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T13068m.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T13075.hs | compile_fail | REJECT | D10 strict / unlifted binding validity | — |
| T13292.hs | multimod_compile | REJECT | C03 ambiguity / predicate placement | — |
| T13292a.hs | unlisted/multiple | REJECT | C03 ambiguity / predicate placement | — |
| T13300.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T13311.hs | compile_fail | REJECT | U02 do / guard / case consistency | — |
| T13487.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T13506.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T13530.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| T13677.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T13871.hs | compile_fail | REJECT | D16 promotion with unpromotable context | — |
| T13909.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T13981A.hs | multimod_compile_fail | REJECT | F05 family equation overlap / cross-module conflict | — |
| T13981B.hs | unlisted/multiple | REJECT | F05 family equation overlap / cross-module conflict | — |
| T13981C.hs | unlisted/multiple | REJECT | F05 family equation overlap / cross-module conflict | — |
| T13981F.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T14048b.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T14048c.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T14055.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T14390.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T14605.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T14607.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T14761a.hs | compile_fail | REJECT | P04 source Werror diagnostic policy | — |
| T14761b.hs | compile_fail | REJECT | D12 malformed syntax accepted after recovery | — |
| T14884.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T14904a.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T14904b.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T15067.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| T15438.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T15474.hs | compile_fail | REJECT | K07 uninferrable kind variable | — |
| T15515.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T15523.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T15527.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| T15552.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T15552a.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| T15629.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |
| T15648a.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T15712.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T15767.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T15793.hs | compile_fail | REJECT | K05 visible kind application arity / visibility | — |
| T15796.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T15799.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T15801.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T15807.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T15816.hs | compile_fail | REJECT | K05 visible kind application arity / visibility | — |
| T15850.hs | multimod_compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T15850_Lib.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T15883.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T15883b.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T15883c.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T15883d.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T15883e.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T15954.hs | compile_fail | REJECT | K10 unsaturated type synonym | — |
| T16059a.hs | compile_fail | REJECT | K08 constraints in kind | — |
| T16059b.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T16059d.hs | multimod_compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| T16059e.hs | multimod_compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T16204c.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| T16255.hs | compile_fail | REJECT | K05 visible kind application arity / visibility | — |
| T16275A.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T16275B.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T1633.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T16414.hs | compile_fail | REJECT | C07 instance overlap selection | — |
| T16453E1.hs | multimod_compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| T16453E2.hs | multimod_compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| T16453S.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T16453T.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T16456.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T16502.hs | compile | REJECT | F02 type equality / given mismatch | — |
| T16512b.hs | compile_fail | REJECT | F04 missing injectivity metadata / validation | — |
| T16517.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |
| T16627.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| T16646Fail.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| T16646Fail2.hs | compile_fail | REJECT | F02 type equality / given mismatch | — |
| T16821.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T16829a.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T16829b.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T16946.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T17021b.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| T17077.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T17139a.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T17173.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| T17213.hs | multimod_compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T17213a.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T17284.hs | compile_fail | REJECT | D14 defaulting / monomorphism restriction validity | — |
| T17301.hs | compile_fail | REJECT | K07 uninferrable kind variable | — |
| T17355.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T17562.hs | compile_fail | REJECT | K07 uninferrable kind variable | — |
| T17566b.hs | compile_fail | REJECT | F08 associated declaration kind-binder identity | — |
| T17566c.hs | compile_fail | REJECT | F08 associated declaration kind-binder identity | — |
| T17567.hs | compile_fail | REJECT | K07 uninferrable kind variable | — |
| T17567StupidTheta.hs | compile_fail | REJECT | K07 uninferrable kind variable | — |
| T17594c.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| T17594d.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| T17594g.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| T17773.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T17940.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T17955.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T18127a.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T18357.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T18357a.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |
| T18357b.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T18455.hs | compile_fail | REJECT | D13 pragma binding / signature validity | — |
| T18640a.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T18640b.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T18640c.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |
| T18714.hs | compile_fail | REJECT | K08 constraints in kind | — |
| T18723a.hs | compile_fail | REJECT | D15 GHC tuple-size limit | — |
| T18723c.hs | compile_fail | REJECT | D15 GHC tuple-size limit | — |
| T18851.hs | compile_fail | REJECT | F02 type equality / given mismatch | — |
| T18851b.hs | compile | REJECT | F02 type equality / given mismatch | — |
| T18851c.hs | compile | REJECT | R02 rigidity / existential / equation consistency | — |
| T18891a.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T18939_Fail.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T1897a.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T19109.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| T19142.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T19187.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T19187a.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| T19397E1.hs | multimod_compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| T19397E2.hs | multimod_compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| T19397E3.hs | multimod_compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| T19397E4.hs | multimod_compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| T19397S.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T19415b.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T19915.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T19977a.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| T19977b.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| T20043.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T20122.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T20189.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T20241b.hs | compile_fail | REJECT | C09 TypeError / KnownNat / family evaluation | — |
| T20260.hs | compile_fail | REJECT | F06 family equation names wrong family | — |
| T20289.hs | multimod_compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T20289_A.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T20542.hs | compile_fail | REJECT | C07 instance overlap selection | — |
| T20588.hs | multimod_compile_fail | REJECT | D01 hs-boot interface consistency | — |
| T20588_aux.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T20588c.hs | multimod_compile_fail | REJECT | D01 hs-boot interface consistency | — |
| T20588c_aux.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T20666.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T20666a.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T20666b.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T20873d.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| T21130.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T21158.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| T2126.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T21327.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| T21444.hs | compile_fail | REJECT | D04 record declaration / update / selector validity | — |
| T21447.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T21530a.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T21530b.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| T22141a.hs | compile_fail | REJECT | K09 NoDataKinds licensing | — |
| T22141b.hs | compile_fail | REJECT | K09 NoDataKinds licensing | — |
| T22141c.hs | compile_fail | REJECT | K09 NoDataKinds licensing | — |
| T22141d.hs | compile_fail | REJECT | K09 NoDataKinds licensing | — |
| T22141e.hs | multimod_compile_fail | REJECT | K09 NoDataKinds licensing | — |
| T22141e_Aux.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T2247.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T22478c.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| T22560_fail_a.hs | compile_fail | REJECT | K04 declaration inferred/specified binder visibility | — |
| T22560_fail_c.hs | compile_fail | REJECT | N04 duplicate type-abstraction binder | — |
| T22560_fail_d.hs | compile_fail | REJECT | K04 declaration inferred/specified binder visibility | — |
| T22560_fail_ext.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| T22570.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T22645.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T22684.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T22891.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T22912.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| T22924a.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T22924b.hs | compile_fail | REJECT | C08 solver termination budget | — |
| T2307.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T23162c.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T23308.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T23427.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T23514a.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T2354.hs | compile_fail | REJECT | D13 pragma binding / signature validity | — |
| T23731.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T23731b.hs | multimod_compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T23731b_aux.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T23734.hs | compile_fail | REJECT | F07 family forall binders unused in LHS | — |
| T23776.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| T23778.hs | compile_fail | REJECT | F07 family forall binders unused in LHS | — |
| T24298.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| T24470a.hs | compile_fail | REJECT | K11 synonym implicit binder arity | — |
| T24553.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T25325.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T25679.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| T25949.hs | multimod_compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T25949_aux.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T26004.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| T26137.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| T26255c.hs | unlisted/multiple | REJECT | C05 unresolved class predicates | — |
| T26318.hs | compile_fail | REJECT | K09 NoDataKinds licensing | — |
| T2688.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| T2806.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T2846b.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T3102.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T3176.hs | compile_fail | REJECT | D04 record declaration / update / selector validity | — |
| T3323.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T3468.hs | multimod_compile_fail | REJECT | D01 hs-boot interface consistency | — |
| T3468a.hs | unlisted/multiple | REJECT | D01 hs-boot interface consistency | — |
| T3540.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T3592.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T3966.hs | compile_fail | REJECT | P04 source Werror diagnostic policy | — |
| T3966b.hs | compile_fail | REJECT | P04 source Werror diagnostic policy | — |
| T4875.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T4921.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T502.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| T5051.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T5084.hs | compile_fail | REJECT | D13 pragma binding / signature validity | — |
| T5095.hs | compile_fail | REJECT | C07 instance overlap selection | — |
| T5236.hs | compile_fail | ACCEPT | P03 harness flag, broken-test, or oracle drift | — |
| T5246.hs | compile_fail | REJECT | C06 implicit-parameter constraints | — |
| T5300.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T5689.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| T5691.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T5858.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T5978.hs | compile_fail | REJECT | C02 functional dependencies | — |
| T6001.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| T6018Afail.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T6018Bfail.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T6018Cfail.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T6018Dfail.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T6018fail.hs | multimod_compile_fail | REJECT | F04 missing injectivity metadata / validation | — |
| T6018failclosed.hs | compile_fail | REJECT | F04 missing injectivity metadata / validation | — |
| T6022.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T6069.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| T6078.hs | compile_fail | REJECT | D10 strict / unlifted binding validity | — |
| T6161.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| T7019.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T7019a.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T7175.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| T7210.hs | compile_fail | REJECT | D12 malformed syntax accepted after recovery | — |
| T7264.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T7279.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T7368.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T7437.hs | compile_fail | REJECT | D05 default method signature / body validity | — |
| T7525.hs | compile_fail | REJECT | C06 implicit-parameter constraints | — |
| T7545.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T7778.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T7809.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T7851.hs | compile_fail | REJECT | U02 do / guard / case consistency | — |
| T7857.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| T7869.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| T7892.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T7989.hs | compile_fail | REJECT | D04 record declaration / update / selector validity | — |
| T8030.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| T8034.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| T8262.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| T8392a.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| T8428.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| T8514.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| T8570.hs | multimod_compile_fail | REJECT | D04 record declaration / update / selector validity | — |
| T8570a.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T8570b.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| T8912.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| T9109.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| T9201.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |
| T9415.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| T9497d.hs | compile_fail | REJECT | N01 typed-hole diagnostic policy | — |
| T9634.hs | compile_fail | REJECT | D12 malformed syntax accepted after recovery | — |
| T9739.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| T9858a.hs | compile_fail | REJECT | R03 impredicativity / forall licensing | — |
| T9858b.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| TcCoercibleFail.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| TcCoercibleFail2.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| TcCoercibleFail3.hs | compile_fail | REJECT | F01 roles / representation / abstraction | — |
| TcNoNullaryTC.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| TcStaticPointersFail02.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| TcStaticPointersFail03.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| Tcfail186_Help.hs | unlisted/multiple | ACCEPT | P02 valid helper / retained GHC accepts | — |
| TyAppPat_ExistentialEscape.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| TyAppPat_MisplacedApplication.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| TyAppPat_NonlinearMultiAppPat.hs | compile_fail | REJECT | N04 duplicate type-abstraction binder | — |
| TyAppPat_NonlinearMultiPat.hs | compile_fail | REJECT | N04 duplicate type-abstraction binder | — |
| TyAppPat_NonlinearSinglePat.hs | compile_fail | REJECT | N04 duplicate type-abstraction binder | — |
| TyAppPat_PatternBinding.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| TyAppPat_PatternBindingExistential.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| TyAppPat_ScopedTyVarConflict.hs | compile_fail | REJECT | P04 source Werror diagnostic policy | — |
| TyAppPat_TooMany.hs | compile_fail | REJECT | D09 type application / abstraction binder validity | — |
| TyfamsDisabled.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| UnliftedNewtypesConstraintFamily.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| UnliftedNewtypesFail.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| UnliftedNewtypesFamilyKindFail1.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| UnliftedNewtypesFamilyKindFail2.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| UnliftedNewtypesInfinite.hs | compile_fail | REJECT | K06 datatype / newtype return representation kind | — |
| UnliftedNewtypesInstanceFail.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| UnliftedNewtypesMismatchedKind.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| UnliftedNewtypesMultiFieldGadt.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| UnliftedNewtypesOverlap.hs | compile_fail | REJECT | F05 family equation overlap / cross-module conflict | — |
| VisFlag1.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| VisFlag1_ql.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| VisFlag2.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| VisFlag3.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| VisFlag4.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| VisFlag5.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| fd-loop.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| mc19.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| mc20.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| mc23.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| mc24.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| mc25.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| tc168.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tc168b.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| tcfail002.hs | compile_fail | REJECT | U01 equation / pattern consistency | — |
| tcfail008.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail010.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail013.hs | compile_fail | REJECT | U01 equation / pattern consistency | — |
| tcfail017.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail018.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail019.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail020.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail021.hs | compile_fail | REJECT | D11 pattern synonym / forbidden binding shape | — |
| tcfail027.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail030.hs | compile_fail | REJECT | N03 Main / export / imported-name ambiguity | — |
| tcfail032.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail037.hs | compile_fail | REJECT | N05 source versus Prelude ambiguity | — |
| tcfail038.hs | compile_fail | REJECT | N06 noncontiguous duplicate method equations | — |
| tcfail040.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail041.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| tcfail042.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail043.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail044.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| tcfail047.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| tcfail050.hs | compile_fail | REJECT | N07 unbound expression/pattern constructor | — |
| tcfail054.hs | compile_fail | REJECT | N07 unbound expression/pattern constructor | — |
| tcfail071.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail072.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail075.hs | compile_fail | REJECT | D10 strict / unlifted binding validity | — |
| tcfail076.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail077.hs | compile_fail | REJECT | F03 associated default or class-member declaration | — |
| tcfail080.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail086.hs | compile_fail | REJECT | D03 deriving eligibility | — |
| tcfail089.hs | compile_fail | REJECT | D11 pattern synonym / forbidden binding shape | — |
| tcfail090.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| tcfail092.hs | compile_fail | REJECT | N08 duplicate parallel-comprehension binder | — |
| tcfail093.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail095.hs | compile_fail | REJECT | D10 strict / unlifted binding validity | — |
| tcfail096.hs | compile_fail | REJECT | C02 functional dependencies | — |
| tcfail097.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail098.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail099.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail100.hs | compile_fail | REJECT | K10 unsaturated type synonym | — |
| tcfail101.hs | compile_fail | REJECT | K10 unsaturated type synonym | — |
| tcfail102.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| tcfail103.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail104.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| tcfail105.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail106.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail107.hs | compile_fail | REJECT | K10 unsaturated type synonym | — |
| tcfail108.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail109.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail114.hs | compile_fail | REJECT | N09 class method mistaken for record field | — |
| tcfail116.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail117.hs | compile_fail | REJECT | D03 deriving eligibility | — |
| tcfail119.hs | compile_fail | REJECT | U01 equation / pattern consistency | — |
| tcfail121.hs | compile_fail | REJECT | C07 instance overlap selection | — |
| tcfail122.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| tcfail123.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| tcfail124.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail125.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail126.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail128.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail129.hs | compile_fail | REJECT | K10 unsaturated type synonym | — |
| tcfail130.hs | compile_fail | REJECT | C06 implicit-parameter constraints | — |
| tcfail131.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail133.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail137.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail138.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail139.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| tcfail142.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail143.hs | compile_fail | REJECT | C02 functional dependencies | — |
| tcfail145.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail146.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| tcfail149.hs | compile_and_run | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail150.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| tcfail151.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| tcfail152.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail154.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail155.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| tcfail156.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| tcfail157.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail158.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| tcfail159.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| tcfail162.hs | compile_fail | REJECT | K01 kind sort, arity, and class application | — |
| tcfail165.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail167.hs | compile_fail | ACCEPT | P03 harness flag, broken-test, or oracle drift | — |
| tcfail168.hs | compile_fail | REJECT | U02 do / guard / case consistency | — |
| tcfail170.hs | compile_fail | REJECT | C02 functional dependencies | — |
| tcfail171.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail172.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail173.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| tcfail174.hs | compile_fail | REJECT | R01 higher-rank subsumption | — |
| tcfail176.hs | compile_fail | REJECT | D02 GADT parent / newtype constructor validity | — |
| tcfail179.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail181.hs | compile_fail | REJECT | C03 ambiguity / predicate placement | — |
| tcfail186.hs | multimod_compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| tcfail188.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail189.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| tcfail190.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail191.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail193.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail198.hs | compile_fail | REJECT | R02 rigidity / existential / equation consistency | — |
| tcfail199.hs | compile_fail | REJECT | U03 expression, signature, or family consistency | — |
| tcfail200.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| tcfail202.hs | compile_fail | REJECT | C02 functional dependencies | — |
| tcfail203.hs | compile | ACCEPT | P01 upstream positive / retained GHC accepts | — |
| tcfail203a.hs | compile_fail | REJECT | D10 strict / unlifted binding validity | — |
| tcfail204.hs | compile_fail | REJECT | P04 source Werror diagnostic policy | — |
| tcfail208.hs | compile_fail | REJECT | C04 signature givens versus wanteds | — |
| tcfail209.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| tcfail209a.hs | compile_fail | REJECT | D08 disabled extension syntax / type licensing | — |
| tcfail211.hs | compile_fail | REJECT | D07 class / instance head and context validity | — |
| tcfail213.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail214.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail215.hs | compile_fail | REJECT | K02 levity / runtime representation | — |
| tcfail216.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail217.hs | compile_fail | REJECT | D06 termination and superclass-cycle declaration rules | — |
| tcfail218.hs | compile_fail | REJECT | C07 instance overlap selection | — |
| tcfail223.hs | compile_fail | REJECT | C01 instance superclass obligations | — |
| tcfail224.hs | compile_fail | REJECT | C05 unresolved class predicates | — |
| tcfail225.hs | compile_fail | REJECT | K03 rigid and polymorphic kind variables | — |

## Receipts and provenance

- Current committed list: `spec-chirho/ghc-should-fail-measurement-chirho.txt`.
- Current repeated per-file verdicts: `tmp-chirho/final-breakthrough-chirho/paired-measurement-chirho/candidate{1,2}-chirho/should_fail-chirho.tsv` and each pass's raw receipts.
- Current upstream action inventory, including exact flags, arguments, all.T line and source hashes: `tmp-chirho/frontier-closure-chirho/upstream-cases-chirho.json`.
- Identical-source retained GHC oracle: `spec-chirho/tmp-chirho/agy-parity-insights-chirho/receipts-chirho/02-ghc-9.14.1-532-should-fail-chirho.tsv`.
- Prior membership seed, used as a review index and corrected where actual source differs: `spec-chirho/tmp-chirho/agy-parity-insights-chirho/receipts-chirho/04-disjoint-rc-family-membership-532-chirho.json`.
- Upstream source and diagnostics: `ghc-tests-chirho/typecheck-chirho/should_fail/`, including all.T, .stderr, helper .hs, and .hs-boot files.
