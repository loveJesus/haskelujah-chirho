<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ground dictionaries for conditional instances (claude_chirho, 2026-11-03)

Branch `ground-dictionaries-chirho`, on the S1 tip bddc428a (gpt_chirho #31138, #31144, #31146, #31147). Source only: root holds the builder.

## Seams
- [x] S1 contract posted (#31140); the typing writer landed it at bddc428a.
- [x] S2: `DictPassResultChirho.diagnostics_chirho`, reported only for unserved occurrences; the driver errors at the occurrence span with readable keys.
- [x] S3 Core consumer: checked join by class and span (`instance_rows_chirho.rs`), duplicate-span control.
- [x] S3 desugar producer: `DesugarOutputChirho.instance_row_keys_chirho`.
- [ ] S3 typing producer `InferResultChirho.source_instances_chirho` (typing writer). The driver already consumes the field.
- [ ] S4 typing: `$own:<i>` for an occurrence of exactly an instance-context predicate (typing writer).
- [x] S4 Core: context rows register each leading binder under `$own:<i>`; a repeated class gets no class-wide entry.

## Core and driver
- [x] Structured entry `dict_pass_module_with_inputs_chirho`, with a compatibility wrapper.
- [x] Pre-pass after ownership seeding; admission by the rewrite's own rules.
- [x] Builder: structural selection, memo before descent (knot), exact rollback, named refusals, constructor authority for obligations.
- [x] Type-growth bounds (#31155): module-wide type work, per-goal weight, per-candidate copy preflight before the checker's matcher, settled top-level goals.
- [x] Bound evidence serves an obligation with an unproven constructor before refusing it (#31155).
- [x] Hooks: method variable, head replacement, reference slot.
- [x] Context-leading marks for every paired row with an indexable context.
- [x] Driver: S1 ground goals through the key join's identity decision; reference ground goals.
- [x] Workflow doc section (dictionary-evidence-chirho).
- [x] rustfmt clean (stdin mode); no compiler run.

## Proof (root)
- [ ] Compile and Clippy once S3 and S4 typing land.
- [ ] Core unit tests: selection, S3 join.
- [ ] Three-engine controls: B01 (3, 0, 3), CondTwoContext, repeated list contexts, CondCyclic (7), shared sub-goals (32), shadowing hold, CondNoBase rejected, growing context rejected.
- [ ] GHC measurement of the four new controls (repeated list contexts, shared sub-goals, shadowing hold, growing context).
- [ ] Corpus axes against the baseline.
