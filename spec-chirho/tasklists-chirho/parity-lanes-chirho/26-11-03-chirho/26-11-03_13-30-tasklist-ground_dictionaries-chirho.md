<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Ground dictionaries for conditional instances (claude_chirho, 2026-11-03)

Branch `ground-dictionaries-chirho`, on the S1 tip bddc428a (gpt_chirho #31138, #31144, #31146, #31147). Source only: root holds the builder.

## Seams
- [x] S1 contract posted (#31140); the typing writer landed it at bddc428a.
- [x] S2: `DictPassResultChirho.diagnostics_chirho`, reported only for unserved occurrences; the driver errors at the occurrence span with readable keys.
- [x] S3 Core consumer: checked join by class and span (`instance_rows_chirho.rs`), duplicate-span control.
- [x] S3 desugar producer: `DesugarOutputChirho.instance_row_keys_chirho`.
- [x] S3 typing producer `InferResultChirho.source_instances_chirho` (typing writer; root replay f8c0baf7).
- [x] S4 typing: `$own:<i>` for an occurrence of exactly an instance-context predicate (typing writer; root replay f8c0baf7).
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

## Round 2 under the lease (#31163, checkpoint 540e44d3)
- [x] Head-position refusals reported (growing context refused by its named bound).
- [x] Obligations built from proven rows; the registry only when selection cannot choose or rows live elsewhere (list contexts).
- [x] Rows only another generator answers are not a body (newtype alias, #31167).
- [x] Unproven matches refused by name: repeated head variable, linear arrow (#31163).
- [x] GHC-measured holds: newtype body (obligation half), repeated-variable negative and positive.
- [ ] Parser producer seam: `instance C ()` head (proposed fix measured: 11/11 on three engines).
- [ ] Typing producer seam: consistent matcher for `instance_sub_goals_chirho` (proposed).
- [ ] Run the matcher guard's driver control (slot returned first).

## Proof (root)
- [x] Compile: workspace all-targets check warning-free (root, f8c0baf7).
- [x] Core unit tests: 213/213 at 540e44d3.
- [ ] Three-engine controls: B01 (3, 0, 3), CondTwoContext, repeated list contexts, CondCyclic (7), shared sub-goals (32), shadowing hold, CondNoBase rejected, growing context rejected.
- [ ] GHC measurement of the four new controls (repeated list contexts, shared sub-goals, shadowing hold, growing context).
- [ ] Corpus axes against the baseline.
