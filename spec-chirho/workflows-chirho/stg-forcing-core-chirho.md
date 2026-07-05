<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# STG Forcing Core Chirho

The remaining WI-007 failures are no longer missing type evidence. They are strict-site forcing gaps where runtime values reach `show`, primitive comparison/arithmetic, or case dispatch as unresolved thunks/PAPs. The rule for this phase is deliberately narrow: force only at strict demand sites, never when storing constructor fields.

## Current Failure Shape

- `twice (twice inc) 0` and Church numerals looked like `$PAP ...` runtime residues, but annotations proved they were missing evidence specialization. The dict pass now rewrites nested higher-order arguments and `showInt#`/`$prim_Show_show_Int` operands under proven `Int` keys before runtime forcing is considered.
- Sieve still returns `IntChirho(0)`, likely from a strict predicate/comparison path failing to force or resolve a delayed computation.
- User BST/map tests store lazy recursive constructor fields correctly, but later strict lookup/size traversal reaches unresolved heap values or tag `0`.

## Strict-Site Workflow

```mermaid
flowchart TD
    A[Runtime value demanded] --> B{Demand site}
    B -->|case scrutinee| C[force to WHNF only]
    B -->|primitive operand| D[force to primitive-compatible value]
    B -->|show/print operand| E[force until printable constructor or primitive]
    B -->|constructor field storage| F[store lazily unchanged]
    B -->|function position| G[enter function/PAP with Apply frame]
    C --> H[dispatch case alternative]
    D --> I[run primop]
    E --> J[render output]
    F --> K[later strict demand decides forcing]
    G --> L[return result or PAP]
```

## Guardrails

- Do not introduce universal recursive forcing across all heap pointers.
- Do not force lazy constructor fields during constructor allocation or list/map building.
- Prefer one helper per demand class over ad hoc call-site loops.
- If adding an annotation makes a PAP failure disappear, fix evidence/dictionary specialization first; do not paper over it with runtime forcing.
- Every runtime forcing slice needs a full probe gate because PAP/application semantics are shared by sections, composition, `fix`, and dictionary payloads.
