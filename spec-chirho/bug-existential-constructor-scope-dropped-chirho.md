<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: existential constructor binders and contexts are dropped

**Found:** 2026-09-07, while landing source type-scope resolution.
**Status:** open; the naming pass has a bounded AST-fidelity deferral.

## What happens

The CST keeps the prefix in declarations such as:

```haskell
data HiddenChirho = forall hiddenChirho. ClassChirho hiddenChirho
                  => MkHiddenChirho hiddenChirho
```

`lower_con_decl_chirho` uses that prefix to locate `MkHiddenChirho`, but
`ConDeclChirho::OrdinaryChirho` and `RecordChirho` have nowhere to store the
`forall` binders or constructor context. They retain the fields and the full
constructor span while dropping the source scope.

The same loss occurs when a later field is itself rank-N. Before the
delimiter-depth fix in `parse_con_decl_chirho`, a nested `=>` was mistaken for
the constructor prefix's context arrow and even the constructor name and
earlier fields were discarded.

## Current trust boundary

The type-scope walker continues to reject a free variable in an ordinary
constructor field. It defers only a prefix constructor whose retained name
starts after its CST-derived constructor span. Symbolic infix constructors are
excluded because their left operand naturally starts before the operator name.
This is linear, source-span based, and does not inspect filenames or filesystem
state.

## Structural fix

Give constructor declarations an explicit argument shape plus existential
binders and context, or normalize every constructor to a full signature while
retaining record-field names. Every typing, deriving, exhaustiveness, naming,
Template Haskell, driver, and Core consumer must then state how it handles that
scope. Remove the span-based deferral after that AST migration lands.
