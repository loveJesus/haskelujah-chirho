<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 677: inferred Fractional ownership and superclass authority

Read-only trace on main `e3833d45` (03ce43a3 plus docs), 2026-10-30. No Cargo, GHC, compiler CLI, runtime program, tracked edit, broker or DB write was performed by this audit. This is a durable source trace for existing row677, not a new queue or implementation. Earlier execution receipts (inferred half gives native Infinity, signed half exact; held Enum Double composition gives native Infinity) are root/Claude2-reported. They were not rerun here.

## Finding and correction

**Current metadata cannot authorize Fractional a → Num a.** The obvious superclass selector exists, but its existence and the two class names do not prove the superclass has the same type argument. There is no retained builtin-class identity or typed superclass argument in `ClassEnvChirho` or the Core dictionary pass.

My earlier recommendation said to permit a "verified single-parameter Fractional→Num edge" before publishing inferred ownership. That verification is **not implementable from the existing name/arity/layout metadata**. Treat the recommendation as conditional on the provenance/typed-edge prerequisite below. Do not add Fractional to an allowlist or infer a Num owner from a class's spelling.

## Exact current producer path

For an inferred binding shaped like `half x = x / 2`:

1. Expression inference captures `/`'s Fractional predicate and the integer literal's Num predicate on the same type variable.
2. `infer_chirho/solving_chirho/schemes_chirho.rs:295–315` removes Num through class-name superclass context reduction, leaving the binding's Fractional predicate. This reduction itself uses name-only superclass metadata.
3. `schemes_chirho.rs:319–346` assigns a `Double` occurrence hint to a variable constrained by Fractional/Floating/RealFrac/RealFloat. It does not add that variable to `occurrence_generalized_vars_chirho` on this branch.
4. `record_inferred_dictionary_owners_chirho` records only the exact predicates of the final scheme as `(type variable, class name) → predicate index`. It retains conflicting/empty veto maps and intersects claims after substitutions; those safety properties must survive any repair.
5. Method finalization in `infer_chirho/context_chirho.rs:430–477` consults the numeric hint first and only considers an inferred owner in the no-hint generalized-variable branch.
6. Reference finalization in `infer_chirho/evidence_chirho.rs:357–377` likewise chooses the hint before ownership.
7. Literal finalization in `evidence_chirho.rs:450–480` excludes inferred ownership whenever the variable has a numeric hint. A Num literal also cannot find an exact Num owner after Num was context-reduced away.

The current owner table is:

`HashMap<TyVarChirho, HashMap<String, usize>>`

The published proof shapes are method/literal `(class_name_chirho, ty_key_chirho: String)` and reference predicates `(class_name_chirho, Option<String>)`. An indexed own key is only the string `$own:<predicate_index>`. It carries no source-class identity or superclass argument substitution.

## Exact current consumer path

The Fractional dictionary lambda is **already retained** for inferred half. `dict_chirho/parameters_chirho.rs:13–35` does not default Fractional: it returns None for that variable predicate. The Num/Enum/Bounded own-proof retention list in `rewrite_chirho/dict_params_chirho.rs:51–55` is therefore not this program's immediate lambda-retention blocker.

`add_dict_params_chirho` registers both a class-name entry and a raw `$own:<index>` entry for each parameter. `own_dict_chirho` in `rewrite_chirho/constrained_references_chirho.rs:537–553` tries the raw indexed key before the class fallback. Consequently:

`own_dict_chirho("Num", "$own:0", …)`

would return a raw Fractional parameter when index zero denotes Fractional. That object has the wrong dictionary layout for a Num selector. Producing this key without a matching projection consumer is unsound.

The existing superclass extraction block in `rewrite_chirho/dict_params_chirho.rs:380–450` does create selector applications and extraction lets, but caches the closure under `HashMap<String, CoreIdChirho>` by class name. Two Fractional roots collapse to one entry. Class-only `$own` can serve the existing single-root signed path, but does not establish a correct indexed two-root path.

## What the source actually retains about a superclass

| Carrier / producer | Retained information | Missing authority |
|---|---|---|
| Source AST `DeclChirho::ClassDeclChirho` | `context_chirho: Vec<ConstraintChirho>` with written superclass arguments and class parameter binders | Useful precursor, but not retained in the typed class environment. Imported class clones do not carry this source AST. |
| Typed `ClassDeclChirho`, `class_chirho.rs:128–152` | `name: String`, `supers: Vec<String>`, primary/extra type variables, methods/defaults/fundeps | No superclass arguments, declaration origin, builtin role, defining module or class identity. |
| `process_class_decl_chirho`, `declarations_chirho/class_instance_chirho.rs:59–68` | Calls `simple_class_chirho()` and stores only `class_chirho.text_chirho()` for each superclass | Explicitly discards `Num a` versus `Num Int` and qualified defining identity. |
| `ClassEnvChirho`, `class_chirho.rs:187–222` | Bare-name maps of declarations and instances | `add_class_chirho` overwrites by bare name; no provenance or shadow-invalidation marker. |
| `new_with_imported_class_env_chirho`, `context_chirho.rs:383–395` | Replaces the seed env with an imported clone | Class metadata has no origin to distinguish a source/imported namesake from the original builtin. |
| Driver dependency frontends | Clone accumulated `InferResult.class_env_chirho` into the next module's imported environment | The clone preserves the loss of superclass arguments and class origin. |
| Naming `IfaceOriginChirho` | Module-level Builtin/Source classification | Not a typed class-declaration identity. `IfaceTypeChirho` has no defining module or superclass predicates; re-exported classes cannot recover them from this. |
| Core `CoreProvenanceChirho` | Entry, compiler-owned value binding IDs, builtin value-reference IDs and imported value identities | No class identity or superclass argument evidence. Compiler ownership of a generated selector does not prove its class declaration was builtin. |
| `fully_known_class_names_chirho` | Whether instance inventory is considered complete | Not builtin provenance; source local classes are inserted, and imported environments clear it. |

Fresh type-variable numbers or equality with the seed's current field shape are also not declaration identity. They must not become authority shortcuts.

## Source and imported namesake boundary

The following is a **source-shape counterexample**, not a program executed by this audit:

```haskell
{-# LANGUAGE FlexibleContexts #-}
import Prelude hiding (Fractional(..))
class Num Int => Fractional a where
  (/) :: a -> a -> a
```

Its typed metadata has one class parameter, name `Fractional`, and `supers = ["Num"]`, just as the proposed name/arity check expects. Its actual superclass is **Num Int**, not Num a. A Num selector extracted from that dictionary cannot justify Num a. An equivalent class imported from another source module has the same problem because its `ClassEnvChirho` clone has no stronger metadata.

Likewise, inspecting only the current source module for a local `Fractional`/`Num` declaration is insufficient: an imported class may already have overwritten the seeded declaration, and a source re-export loses type-origin information. Generated `$sel_Fractional_super_Num` spelling is not proof of the standard edge.

Current solver/context-reduction code already has name-only superclass weaknesses; that is not authority to extend those weaknesses into new native dispatch. A row677 repair must not turn an unproven superclass argument into executable arithmetic.

## Reusable machinery, and its limit

Reusable Core construction exists:

- `DictLayoutChirho.super_slots_chirho: Vec<(String, usize)>` gives slot positions.
- `super_selectors_chirho: HashMap<(String, String), CoreIdChirho>` identifies generated selector values.
- `layout_chirho.rs:152–222` emits selector functions with dictionary source/result binder types.
- Existing extraction lets are `CoreExprChirho::AppChirho { selector, source_dictionary }` and are retained only when the rewritten body uses them.

These are reusable **execution mechanisms**, not a reusable typed superclass proof. They are all generated from the same name-only metadata and cannot recover lost argument substitution.

## Prerequisite for a sound bounded path

Before changing inferred evidence priority or publishing a projected owner, retain one of the following authoritative proofs from the producer:

1. **Canonical builtin-class provenance:** the surviving source and target declarations are explicitly identified as the standard builtin Fractional and Num, with the seed producer recording the identity-argument edge. Source registration/replacement must invalidate builtin authority, and imported clones/merges must preserve the current declaration's origin rather than recover authority from its spelling.
2. **Typed superclass predicates:** preserve superclass class identity and its actual argument types over the declaration's bound parameters, produced from the source AST or builtin seed. Only a verified substitution mapping the source argument to the requested Num argument authorizes projection. `Num Int` must remain Num Int; multiple-parameter, transformed-argument and qualified-class cases cannot use a name-only identity assumption.

An authoritative builtin edge is sufficient for a bounded standard Fractional→Num repair; it is not permission to treat all source classes with those names as that edge. A typed general carrier could support valid source-defined identity edges too, but that is a separate scope choice. No carrier is currently present after class registration.

After that prerequisite, existing evidence can still name `(requested class, predicate index)`, because the definition's authoritative scheme/class metadata identifies the source predicate. Core then needs a lookup keyed by **(predicate index, requested class)** and validated projection lets; an indexed proof must never fall back to a raw subclass dictionary or another same-class parameter. This retains both of two Fractional roots. Truly unowned/ambiguous variables keep their existing default policy, and nested local specialization remains separate.

`class_chirho.rs` is **4,001 lines**. Something is wrong here: new provenance/edge logic must not be bolted into it. A behavior-neutral structural split with explicit placement precedes new logic; a narrow data-carrier plumbing seam must be agreed with the parent rather than silently widening this audit into implementation.

## Decisive controls for row677

- Inferred versus signed half, including the integer literal's Num conversion and a direct Fractional method.
- An inferred forwarding function calling half, so reference evidence forwards the correct owner.
- Mixed Num/Fractional parameters and two observably distinct Fractional dictionaries. Core must project each Num dictionary from its own indexed source root.
- Standard Fractional imported directly and through a source forwarder, with honest origin handling.
- Local and imported namesake `Fractional` whose superclass is Num Int; no Num-a projection may be emitted. Also a transformed superclass argument and a multiple-parameter case must not acquire builtin authority.
- Overwritten builtin metadata must lose the old edge's authority, including imported replacement.
- Existing conflicting-owner, substitution-intersection, nested-local and genuinely ambiguous defaulting controls remain unchanged.
- The held670/677 Fractional-plus-Enum-Double composition must be exact on all three engines before enabling Enum Double or restoring tc087 acceptance. No compiler allowlist or acceptance-only check establishes native arithmetic correctness.

## Main source homes

All paths are relative to the canonical repository:

- `crates/haskelujah-typing-chirho/src/class_chirho.rs`
- `crates/haskelujah-typing-chirho/src/infer_chirho/declarations_chirho/class_instance_chirho.rs`
- `crates/haskelujah-typing-chirho/src/infer_chirho/solving_chirho/inferred_dictionary_owners_chirho.rs`
- `crates/haskelujah-typing-chirho/src/infer_chirho/solving_chirho/schemes_chirho.rs`
- `crates/haskelujah-typing-chirho/src/infer_chirho/context_chirho.rs`
- `crates/haskelujah-typing-chirho/src/infer_chirho/evidence_chirho.rs`
- `crates/haskelujah-core-chirho/src/dict_chirho/layout_chirho.rs`
- `crates/haskelujah-core-chirho/src/dict_chirho/parameters_chirho.rs`
- `crates/haskelujah-core-chirho/src/dict_chirho/rewrite_chirho/dict_params_chirho.rs`
- `crates/haskelujah-core-chirho/src/dict_chirho/rewrite_chirho/constrained_references_chirho.rs`
- `crates/haskelujah-naming-chirho/src/iface_chirho.rs`
- `crates/haskelujah-core-chirho/src/expr_chirho.rs`
