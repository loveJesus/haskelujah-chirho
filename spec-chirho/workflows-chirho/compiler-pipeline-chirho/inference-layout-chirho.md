<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Inference homes Chirho

Row 661 keeps the public inference entry points and state in `crates/haskelujah-typing-chirho/src/infer_chirho.rs`. Its implementations are logical children through `#[path]`, so existing child modules retain their `super::` relationship and bytes. Cross-file methods use the listed `pub(super)` visibility; no public crate API is added.

```mermaid
flowchart TD
    shell_chirho[infer_chirho.rs: public entry and state] --> context_chirho[context_chirho.rs: construction and finalization]
    context_chirho --> declarations_chirho[declarations_chirho: module classes instances and dependency groups]
    declarations_chirho --> terms_chirho[terms_chirho: expressions bindings matches and expected types]
    declarations_chirho --> solving_chirho[solving_chirho: schemes unification validity and deferred predicates]
    terms_chirho --> solving_chirho
    solving_chirho --> types_chirho[types_chirho: families synonyms and AST conversion]
    terms_chirho --> types_chirho
    tests_chirho[tests_chirho: topic tests and shared support] --> shell_chirho
```

`builtins_chirho/` and `evidence_chirho.rs` remain beside these homes. Existing conversion files move to `types_chirho/`, instance and signature files to `declarations_chirho/`, and equality and rigid-variable files to `solving_chirho/`. Existing term helpers move to `terms_chirho/`. The inline tests retain their function names and bodies under topic modules; the receipt maps every changed path.

The split preserves inference behavior. Item accounting, string-aware comparison and mutation checks establish the move; canonical full inference dumps over the tracked Haskell corpus establish that schemes, predicates, substitutions, diagnostics and occurrence evidence retain their values and order. Semantic changes belong to later rows.

Row 708's numeric controls separate ordinary nominal equality from numeric producers. `Int` and `Double` are distinct constructors; a literal's class predicate or an explicit `fromIntegral` conversion supplies numeric behavior. The production-equal test checkpoint precedes the narrow unifier correction, and neither checkpoint by itself proves general defaulting or runtime representation behavior.

```mermaid
flowchart TD
    literal_chirho[Numeric literal] --> wanted_chirho[Existing Num or Fractional wanted]
    conversion_chirho[Explicit fromIntegral application] --> scheme_chirho[Existing constrained callable scheme]
    wanted_chirho --> solve_chirho[Ordinary inference and constraint solving]
    scheme_chirho --> solve_chirho
    signed_chirho[Signed value or nested type] --> unify_chirho[unify_chirho: structural nominal equality]
    solve_chirho --> unify_chirho
    unify_chirho --> equal_chirho[Equal constructor or metavariable substitution]
    unify_chirho --> mismatch_chirho[Distinct constructors report mismatch]
```

The row 708 source correction has a paired measured control: the original unifier accepted three invalid signed numeric values, and the corrected unifier rejects them while the seven valid literal/conversion/polymorphism controls remain checkable. The full typing library and scoped lint gate passed; a fresh frozen candidate preserved both existing compiler-axis raw streams byte-for-byte against the pin-matched actual-A9 baseline. Main `4cec50ac` contains that exact measured crates tree. See the numeric-nominality tasklist for exact commits, overlapping test scopes and receipt hashes. Wider runtime/defaulting behavior remains open.

## Opaque constructor transport Chirho

Unit719 keeps a producer's length-delimited constructor key intact in the existing `same_tycon_name_chirho` comparison. A dot inside the defining module is part of the opaque key, so two different declarations cannot agree through their last written qualifier component. The reserved `%constructor:` namespace, like promoted keys, selects complete-key equality only; recognizing it supplies no declaration role, identity, generativity or class proof. The existing compatibility route for legacy written qualified names and the ordinary numeric rule stay unchanged. Checked-kind unification already compares complete constructor names.

```mermaid
flowchart TD
    declaration_chirho[Actual selected declaration] --> key_chirho[Producer constructor key]
    key_chirho --> comparison_chirho[Ordinary Con comparison]
    written_chirho[Legacy written type name] --> comparison_chirho
    comparison_chirho --> opaque_chirho{Either name is an opaque constructor or promoted key?}
    opaque_chirho -->|yes| complete_chirho[Complete equality or mismatch]
    opaque_chirho -->|no| legacy_chirho[Existing written qualifier comparison]
```

The production-equal BEFORE at `1bdf91c1` has a direct unifier RED on the first plain pair of actual keys from `AlphaChirho.SharedChirho` and `OtherChirho.SharedChirho`; its reverse and nested arms are not reached after that panic. The source control also records a wrong-definer `compile_modules_chirho` acceptance, while same-producer substitution and direct/forwarded source aliases hold. The correction above is source-only until its separate AFTER gates run. No GHC reference, native behavior or whole-union acceptance follows from either the BEFORE measurements or the source edit.
