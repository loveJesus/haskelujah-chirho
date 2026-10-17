<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Parallel list comprehensions workflow

Parallel qualifier branches retain their source grouping through parsing and inference. Core
lowering reuses ordinary comprehensions for each branch, then combines their exported values
with shortest-input lockstep semantics.

```mermaid
flowchart TD
    source_chirho[Haskell ParallelListComp source] --> cst_chirho[CST parser preserves every top-level pipe]
    cst_chirho --> ast_chirho[AST stores parallel qualifier groups]
    ast_chirho --> infer_branch_chirho[Infer each group in an isolated lexical scope]
    infer_branch_chirho --> export_bindings_chirho[Capture each group's bound schemes]
    export_bindings_chirho --> infer_body_chirho[Bind all exported schemes for result inference]
    ast_chirho --> branch_lists_chirho[Desugar each group as an ordinary comprehension]
    branch_lists_chirho --> zip_chirho[Zip branch streams left-to-right]
    zip_chirho --> shortest_chirho[Stop when the shortest branch ends]
    shortest_chirho --> map_body_chirho[Map a pattern lambda that restores all branch bindings]
    map_body_chirho --> core_chirho[Ordinary Core and STG pipeline]
```

## Invariants

- A branch can use outer names and earlier qualifiers in that branch, but not sibling bindings.
- The result expression can use the bindings exported by every branch.
- A single `|` retains ordinary sequential and Cartesian comprehension semantics.
- Multiple top-level `|` branches advance lockstep and truncate to the shortest branch.
- A `let` qualifier's layout block ends at the following comma, pipe, or closing bracket, even
  on one line (the Report's parse-error(t) rule, `cst_parser_chirho/let_qualifier_chirho.rs`,
  shared with guards): the parser moves the block's virtual close there and drops the block's
  separators after it, so the statement or declaration after the comprehension stays in its
  enclosing do block, `where` or module. Before 2026-10-17 the close stayed after the `]`, where
  it closed the enclosing block instead and the next statement was lost or misread.
- Parallel lowering reuses ordinary comprehension, tuple-pattern lambda, `zip`, and `map`
  machinery rather than adding a backend-specific execution path.
