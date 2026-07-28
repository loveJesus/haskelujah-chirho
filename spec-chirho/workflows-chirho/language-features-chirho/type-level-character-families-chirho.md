<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Type-level character family workflow

```mermaid
flowchart TD
    source_chirho[Type-level character literal and TypeLits family application]
    source_chirho --> cst_chirho[CST preserves the literal token text]
    cst_chirho --> ast_chirho[AST stores the literal text]
    ast_chirho --> codec_chirho[Shared Haskell character codec decodes escapes]
    source_chirho --> iface_chirho[GHC.TypeLits exports CharToNat and NatToChar]
    iface_chirho --> kind_chirho[Kind environment assigns unary family kinds]
    codec_chirho --> reducer_chirho{Concrete valid argument}
    kind_chirho --> reducer_chirho
    reducer_chirho -->|CharToNat c| nat_chirho[Emit codepoint Nat literal]
    reducer_chirho -->|NatToChar n| char_chirho[Emit canonical Char literal]
    reducer_chirho -->|Variable or invalid codepoint| stuck_chirho[Preserve stuck family app]
    nat_chirho --> evidence_chirho[Normalize equality evidence]
    char_chirho --> evidence_chirho
    evidence_chirho --> gates_chirho[T14934a and T19535 plus bounded regression gates]
```

## Invariants

- Term and type-level character literals share one escape parser; family reduction cannot
  reinterpret source text differently from expression lowering.
- Qualified and unqualified TypeLits family names reduce identically.
- `CharToNat` returns the Unicode scalar value of a concrete character.
- `NatToChar` reduces only a valid Unicode scalar value; out-of-range and surrogate codepoints
  stay stuck.
- Symbol and Nat family reduction behavior remains unchanged.
