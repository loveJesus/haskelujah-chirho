<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Parity review and landing Chirho

The execution PRD owns machine/resource limits; the current coordination tasklist owns the concrete candidate, roles and acceptance evidence. Broker posts carry assignments and lease transitions. L.J.'s direct instruction wins over either record. A builder lease permits bounded focused work, not broad suites that the PRD reserves for explicit human authorization.

```mermaid
flowchart TD
    direction_chirho[Read direct instruction and project records] --> inventory_chirho[Verify Git state live identities and dependencies]
    inventory_chirho --> plan_chirho[Record scope owners and acceptance evidence]
    plan_chirho --> independent_chirho[One builder independently checks candidate behavior]
    plan_chirho --> source_chirho[Leader reviews source ownership and failure paths]
    plan_chirho --> scout_chirho[Read-only scout identifies structural reject causes]
    scout_chirho --> counts_chirho[Independent frozen-CLI reproduction and accept-risk checks]
    independent_chirho --> decision_chirho{Evidence supports the complete candidate?}
    source_chirho --> decision_chirho
    decision_chirho -- no --> repair_chirho[Record exact blocker and isolate owner repair]
    repair_chirho --> independent_chirho
    decision_chirho -- yes --> lease_chirho[Issue separate main and single-writer DB lease]
    lease_chirho --> freshness_chirho[Recheck instructions Git state and complete-range secret scan]
    freshness_chirho --> landing_chirho[Fast-forward exact approved tip and log scoped result]
    landing_chirho --> verify_chirho[Verify resulting refs artifacts and release leases]
    verify_chirho --> lanes_chirho[Assign next verified structural root causes]
    counts_chirho --> lanes_chirho
    verify_chirho --> followup_chirho[Later independent counterexample]
    followup_chirho --> correction_chirho[Reproduce and correct the recorded claim]
    correction_chirho --> next_candidate_chirho[Freeze external effects and review the next owner repair]
```

A baseline run measures a reproduction count. A prospective fix count remains an estimate until a candidate is tested against that baseline. Membership lists, reasons and behavior matter more than totals. A frozen executable retains its source commit and before/after hashes; a pre-existing receipt is attributed rather than presented as a fresh execution. Independent review includes stack prerequisites and withdrawn-intermediate regressions, not just the tip's newest feature.

Source work stays in owner worktrees under `haskelujah-workspaces-chirho`. The canonical checkout is reserved for the explicitly leased landing and coordinated bookkeeping. Unrelated dirty work is preserved; progress DB ownership must be handed over explicitly. Remote push and release are separately scoped effects.

A later counterexample narrows or withdraws the earlier disposition. Preserve earlier passing observations at their actual scope, record the new failing input, and decide between rollback and forward repair explicitly. A known crash becoming a silent wrong result is a material failure even when the program was already unsupported; a successful exit alone is not a behavior gate.
