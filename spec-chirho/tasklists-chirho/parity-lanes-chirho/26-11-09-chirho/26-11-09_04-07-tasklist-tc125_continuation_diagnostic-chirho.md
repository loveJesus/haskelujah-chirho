<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Never-land tc125 continuation diagnostic Chirho

This isolated branch starts at `4f16c6779c2d740088275f4d58903814b7a40df5`. Root owns compiler execution and product integration. The product union, its artifacts and the reader's production source stay untouched. This is diagnostic source, not a solver repair or an acceptance claim.

- [x] Locate the existing budget, FD/P2 borrowers and terminal settlement in the four cleared Rust homes.
- [x] Preserve the first actual failed charge in the existing budget carrier, including requested and pre-failure remaining work/output.
- [x] Carry private FD segment/depth reasons and P2 borrower names to the existing error after unchanged settlement debits.
- [x] Verify the explicit source scope, unchanged numerical bounds and public APIs; format and diff-check without a compiler.
- [ ] Root-granted isolated execution, if later authorized. The current tc125 result remains RESOURCE_UNCLASSIFIED.

```mermaid
flowchart LR
    LoanChirho[Original borrowed allowance] --> WorkChirho[Unchanged FD or P2 work]
    WorkChirho --> DebitChirho[Charge exactly the spent allowance]
    DebitChirho --> ErrorChirho[Existing sticky E0300 with private failure detail]
```

Only `equality_normalization_chirho/mod.rs`, `fundep_generalization_chirho/solver_chirho.rs`, `types_chirho/quantification_chirho.rs` and `state_chirho/continuation_chirho.rs` may change production source. No bound, supply, success/decline/failure decision, fixture, dependency, typing API or harness changes. A nested bounded failure with no captured allowance/depth reason stays explicitly unattributed.
