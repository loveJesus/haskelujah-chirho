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
- `ShowFloatChirho` gives non-finite `Double` values GHC's exact spellings: `NaN`, `Infinity`,
  and `-Infinity`. Finite values use fixed notation for normalized decimal exponents -1 through
  6 and scientific notation outside that interval. Scientific mantissas keep a decimal point,
  including `1.0e7` and `1.0e-2`.
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
    nonempty_ds_chirho["import of Data.List.NonEmpty, no local NonEmpty: base's a :| [a], infixr 5"] --> meta_ds_chirho
    meta_ds_chirho --> driver_ds_chirho[driver passes it to dict_pass_module_full_with_evidence_chirho]
    driver_ds_chirho --> parse_ds_chirho{"parse_show_shape_at_chirho: every field of every constructor renders at its precedence, within the depth and key bounds?"}
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
- **Bounded (C5, gpt_chirho #28328):** the support check fails closed past the parser's own
  nesting bound (64) and for a key over 512 bytes. A non-regular datatype such as
  `data N a = Nest (N [a]) | Flat a`, or `data Seq a = Nil | Cons a (Seq (a, a))`, meets a new key
  at every level, so it keeps its existing route instead of recursing without end, even as a field
  of a type that is never printed.
  - A semantic `false` is final at every depth. A `false` found after the outermost check met the
    depth bound may rest on the bound, so it is not kept.
  - A `true` records the deepest nesting at which it held and answers only there or shallower. A
    chain deeper than the bound therefore fails at its root in any call order, while a shallow
    suffix renders.
  - A `true` that rested on a key still being checked further out is provisional. It answers later
    visits within the same outermost check, becomes final when that check renders, and is dropped
    when it does not. A cycle with fan-out is explored once per key, not 2^n times.
- **Dictionary route:** a monomorphic type's instance dictionary uses its public row, so
  polymorphic code reaches the same rendering (C04). That placement refined the trace, which had
  listed C04 outside the claim. A type with parameters reached through a dictionary keeps the
  plain derived `show`.
- **Parameters (row 531 re-advance):** the metadata takes the derived instance's own binders
  (`visible_instance_binders_chirho`), so a kind-signature tail counts as a parameter, as it does
  in the instance head. `data E :: Type -> Type deriving Show` describes `E x`, not a monomorphic
  `E`. Only an empty declaration reaches this case, since GHC rejects ordinary constructors after
  a result kind.
- **Code:**
  - `crates/haskelujah-typing-chirho/src/deriving_chirho/show_chirho.rs` (the instance and the metadata);
  - `crates/haskelujah-core-chirho/src/dict_chirho/show_chirho/derived_chirho.rs` (the renderer);
  - the `DerivedChirho` shape in `show_chirho/shape_chirho.rs`.
- **Tests:**
  - end to end, `tests/dictionary_evidence_chirho/derived_show_chirho.rs`;
  - infix structure and the bounded memo (nested datatypes, the depth bound in either call order,
    a fan-out cycle), in `show_chirho/derived_tests_chirho.rs`;
  - metadata, in `deriving_chirho/show_tests_chirho.rs`.
  - NonEmpty values (unit 517), in `tests/dictionary_evidence_chirho/non_empty_routes_chirho.rs`.
- **NonEmpty (unit 517):** `derived_show_metadata_chirho` adds base's `data NonEmpty a = a :| [a]`,
  `infixr 5`, for a module that imports Data.List.NonEmpty and declares no type, class or family
  named `NonEmpty`. The renderer is unchanged. A key reaches the entry when `:|` is typed (the
  builtin constructor seed) and the type is spelled bare `NonEmpty`, which an unqualified import of
  the type gives. An alias-qualified key such as `NE.NonEmpty Int` keeps its existing route; that
  boundary is unasserted until measured. A list that `NE.fromList` built but a signature types
  `NonEmpty` meets the helper's lone `:|` alternative, which has no list arm (rows P11-P15; their
  outcome is measured at the gate, not claimed here).
- **Not claimed:**
  - the runtime precedence methods (`showsPrec`, `shows`, `showParen` as functions) and the class defaults;
  - a hand-written show-only instance at nested precedence (C01);
  - a type with parameters reached through a dictionary;
  - newtypes (S09), standalone deriving, and GADT, prefix-symbolic and backtick constructor declarations.
