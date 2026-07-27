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
    instance_chirho -->|no| materialize_chirho{Supported flat structured key?}
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
- Missing flat `Maybe`/`Either` renderers are generated deterministically only from proven
  evidence and only when every field has a backend-neutral scalar renderer.
- The dictionary pass rewrites only when the exact generated `Show` binding exists.
- Structured renderers use backend-neutral Core rather than interpreter-only compound primops.
- Scalar show primops used by portable Core have matching STG mappings, including `showChar#`.
- The RTS records allocation kind out of band before interpreting thunk headers; constructor
  payload bits cannot masquerade as thunk state.
- String append owns borrowed input bytes before any allocation that can trigger collection.
