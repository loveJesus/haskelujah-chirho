<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Flat type syntax and list constructors

Some CST contexts, notably record fields, retain a flat token sequence instead
of structured type nodes. Both routes must preserve the same semantic type.
This is syntax preservation, not permission to repair an invalid kind later.

```mermaid
flowchart TD
    A[Source type] --> B{CST representation}
    B -->|Structured| C[Lower nested type node]
    B -->|Flat| D[Find balanced group and its complete contents]
    B -->|Instance or superclass token slice| K[Borrow original tokens and spans into shared flat grammar]
    K --> D
    D --> E[Recursively reconstruct the element, if present]
    C --> F[Shared list constructor/application conversion]
    E --> F
    F -->|No element: empty brackets| G[Con list constructor, no fabricated argument]
    F -->|Element present| H[List element type]
    G --> I[Ordinary naming and kind checking]
    H --> I
    I --> J[Type inference and execution]
```

`lower_chirho/flat_types_chirho.rs` owns the extracted flat-type reconstruction
helpers. Instance-head argument token slices and superclass contexts now enter
the same reconstruction function as flat record fields, rather than a second
token grammar. This keeps promotion ticks, literal arguments, tuple constructors,
applications and ascriptions attached to their original spans. Its
`list_type_chirho` is shared by structured list nodes and this flat path. An empty `[]`
is the constructor of kind `Type -> Type`; `[a]` is its application to `a`.
`[] Char` must therefore lower without a fabricated placeholder argument, and
bare `[]` as a record field must remain unsaturated so kind checking rejects it.

An instance argument keeps its outer parentheses until this shared atom grammar
reads them. The single arrow in `(->)` is a constructor, not an incomplete
function with two invented operands; `((->) Int :: Type -> Type) Bool` retains
that constructor, application and annotation. A bare arrow is not licensed as
an atomic type by this rule, nor is promoted arrow syntax.

The flat bracket scan owns the closing bracket at its own depth. In
`[[Int] -> Int]`, the inner closing bracket cannot discard the function tail.
The scan advances monotonically within that group and borrows the inner slice;
it does not clone the growing enclosing syntax tree. Existing recursive flat
reconstruction and its broader performance limits are not redesigned here.

Parser controls compare the semantic shapes of equivalent structured and flat
types, ignoring spans and parentheses but retaining constructor/application
identity, tuple arity and application arguments. They also require a following signature to survive. Driver controls
read fields and apply contained functions through STG, LLVM and Cranelift against
independently checked GHC outputs; a negative control requires a kind mismatch,
not just an unrelated error. The tasklist records which gates have actually run.

Constructor headers have a separate delimiter-aware context boundary in
`cst_parser_chirho/constructors_chirho.rs`, shared by explicit-forall and
no-forall declarations. A context arrow must not become a field or consume a
following declaration. Context lowering distinguishes an implicit parameter's
`?label :: payload` from a type-kind ascription. The classifier checker requires
a lifted payload type; it does not silently erase unknown payload names.

Limits: the outer instance-head splitter is still a separate consumer, and this
does not establish all unparenthesized instance forms or complete flat
forall/fixity support. Ordinary/record constructor context evidence is still not
represented in the constructor AST; the boundary/read-back controls do not prove
full implicit-parameter evidence behavior. GADT-result representation
and malformed-syntax diagnostics also remain separate contracts.
