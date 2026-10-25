<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Workflow: solved Show evidence for print

`print` is a constrained Prelude function rather than a `Show` class method. The compiler must
carry its solved `Show` predicate through Core; native runtime bits cannot distinguish values such
as unboxed `True` and integer `1`.

```mermaid
flowchart TD
    source_chirho[Source: print value] --> infer_chirho[Type inference solves Show value_type]
    infer_chirho --> occurrence_chirho[Record print occurrence with full concrete Show key]
    occurrence_chirho --> join_chirho[Driver joins source-order occurrence id to evidence]
    join_chirho --> validate_chirho{Show constrains print's own scheme?}
    validate_chirho -->|no| fallback_chirho[Preserve canonical print path]
    validate_chirho -->|yes| instance_chirho{Concrete Show binding exists?}
    instance_chirho -->|no| materialize_chirho{Complete supported shape with backed leaves?}
    materialize_chirho -->|no| fallback_chirho
    materialize_chirho -->|yes| generate_chirho[Generate portable Show binding from proven key]
    generate_chirho --> rewrite_chirho
    instance_chirho -->|yes| rewrite_chirho[Rewrite to putStrLn applied to concrete show binding]
    rewrite_chirho --> core_show_chirho[Portable Core renderer: case, scalar show primops, string append]
    core_show_chirho --> llvm_chirho[LLVM lowering]
    core_show_chirho --> cranelift_chirho[Cranelift lowering]
    llvm_chirho --> rts_chirho[Native RTS string output]
    cranelift_chirho --> rts_chirho
```

## Invariants

- Evidence is admitted only when the class declares the referenced method or constrains the
  referenced function's own type scheme.
- Concrete structured keys retain their full shape, including lists, tuples, `Maybe`, and
  `Either`; unresolved type variables never become invented instance keys.
- Prelude constructor schemes propagate payload annotations through `Just`/`Nothing` to the
  enclosing `Maybe` type before occurrence evidence is finalized.
- Missing `Maybe`/`Either`/tuple/list renderers are composed deterministically from
  complete evidence. `Integer` remains an Integer key inside compound evidence and
  uses the engine's integer renderer; no string replacement fabricates an instance.
- Bootstrap lists and evidenced lists share one recursive worker generator. Each
  shape's element renderer is emitted once, with an explicit separator parameter.
- A user instance leaf needs an actual body, not just a name-table entry. Its show
  method can serve list/tuple elements at precedence zero. A hand-written show-only
  implementation is not assumed to implement showsPrec 11: that argument stays on the
  dictionary path. Derived Show is rendered from its constructor metadata instead
  (below); hand-written showsPrec behavior is not claimed.
- The dictionary pass rewrites only when the exact generated `Show` binding exists.
- Structured renderers use backend-neutral Core rather than interpreter-only compound primops.
- Scalar show primops used by portable Core have matching STG mappings, including `showChar#`.
- The RTS records allocation kind out of band before interpreting thunk headers; constructor
  payload bits cannot masquerade as thunk state.
- String append owns borrowed input bytes before any allocation that can trigger collection.

## Derived Show (row 533, 2026-10-25)

gpt_chirho #28285, #28295. The derived instance supplies `show`, the class's only method. Its
precedence comes from constructor metadata, rendered in Core with GHC's derived rules:

```mermaid
flowchart TD
    data_ds_chirho["data T = ... deriving Show"] --> meta_ds_chirho["derived_show_metadata_chirho: constructor order, form, labels, field types, precedence"]
    fixity_ds_chirho["infixl/infixr/infix p op (default infixl 9)"] --> meta_ds_chirho
    meta_ds_chirho --> driver_ds_chirho[driver passes it to dict_pass_module_full_with_evidence_chirho]
    driver_ds_chirho --> parse_ds_chirho{"parse_show_shape_at_chirho: every field of every constructor renders at its precedence?"}
    parse_ds_chirho -->|no| route_ds_chirho[existing route, unchanged]
    parse_ds_chirho -->|yes| helper_ds_chirho["$prim_Show_showsPrec_key d x: one recursive helper per concrete key"]
    helper_ds_chirho --> shapes_ds_chirho["Maybe, Either, tuple and list shapes, and other derived types, call it at their precedence"]
    helper_ds_chirho --> row_ds_chirho["a monomorphic type's public row $prim_Show_show_T becomes the helper at 0"]
    row_ds_chirho --> uses_ds_chirho[evidenced print, show occurrences and the instance dictionary]
```

- **Rules:**
  - prefix `C a1 a2` is parenthesized above 10, with fields at 11, and a bare `C` has no fields;
  - a record `C {l = a}` is parenthesized above 10, with fields at 0 and labels in declaration order;
  - a binary symbolic constructor `a1 op a2` is parenthesized above its precedence p, with both
    operands at p + 1 whatever the associativity.
- **Fail-closed:** a key with any field no shape renders (an unknown type, a hand-written
  show-only instance at 11, a function) keeps its existing route; nothing defaults to Int or to
  the runtime space heuristic. A recursive type is admitted when everything else in it renders.
- **Code:**
  - `crates/haskelujah-typing-chirho/src/deriving_chirho/show_chirho.rs` (the instance and the metadata);
  - `crates/haskelujah-core-chirho/src/dict_chirho/show_chirho/derived_chirho.rs` (the renderer);
  - the `DerivedChirho` shape in `show_chirho/shape_chirho.rs`.
- **Tests:**
  - end to end, `tests/dictionary_evidence_chirho/derived_show_chirho.rs`;
  - infix structure, in `show_chirho/derived_tests_chirho.rs`;
  - metadata, in `deriving_chirho/show_tests_chirho.rs`.
- **Seam:** unit 517 can describe NonEmpty `:|` as infixr 5 through the same metadata, without
  touching the renderer.
- **Not claimed:**
  - the runtime precedence methods (`showsPrec`, `shows`, `showParen` as functions) and the class defaults;
  - a hand-written show-only instance at nested precedence (C01);
  - a type with parameters reached through a dictionary;
  - newtypes (S09), standalone deriving, and GADT, prefix-symbolic and backtick constructor declarations.
