<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Complete predicate evidence and instance dictionary functions

Decision owner: HASKELUJAH/gpt_chirho, 2026-09-19. Implementation owner: claude_chirho's
isolated instance-obligations lane. Row 484's kind/GND work remains separate.
Authority: L.J.'s standing continue-development delegation. This is reversible compiler work,
not a new external/public deployment, reporting-policy choice or DB writer lease.

## Decision and placement at brick 1

Proceed with the typing-context brick, then an explicit runtime continuation: complete predicate
evidence through the existing typing/desugar/driver/Core boundary, followed by general instance
dictionary functions. No fresh human approval is needed for this bounded implementation direction.
The four existing failures are acceptance inputs, not proof that all four have one cause.

Use focused child modules for evidence representation, instance dictionary construction and
application. Do not grow the 4944-line `dict_chirho/rewrite_chirho.rs` or the 1606-line `mod.rs`
with another parallel guessing path. Existing Core lambdas/applications are the target; no backend,
language extension, package dependency or replacement solver is implied by this decision.

Before the dictionary representation changes, checkpoint the coherent context brick and create its
local pre-change tag ending in `before-chirho`. Keep the replacement in a separate commit with its
behavioral controls. The next design checkpoint is the concrete shared evidence/instance-identity
carrier and its producers/consumers, not a completed whole-pass rewrite.

## Measured boundary, not inferred from error wording

Main at c74db426 uses CLI
`965f2d07c3e959f05973ebfe2678921102b9dfecbe68347dd4941210742340c5`, hashed unchanged before
and after the bounded runs. GHC is 9.14.1. Full source, hashes, stdout, stderr and exits are in
`test-data-chirho/dictionary-evidence-chirho/context-consumers-chirho/observations-chirho.json`.

- **Ground MPTC control:** two context-free instances `TagChirho Int Bool` and
  `TagChirho Int Char`. Without a context on `describeChirho`, both compilers print
  `3@z\n`. Adding only `TagChirho Int Char =>` to that signature makes ours exit zero
  with stdout exactly `\n`, while GHC still prints `3@z\n`. This is a blank line,
  not zero output bytes. No conditional-instance dictionary is needed for either source.
- **Source loss independently visible:** `capture_reference_evidence_chirho` captures
  `(class_name, pred.ty)`, omitting `extra_tys_chirho`; finalization and the driver
  transport that shortened record. That is a real contract loss. The precise route from
  that loss to the blank line still needs operands/evidence observed at the consuming call.
- **Quantified demand control:** replacing the Pair `Show` body with
  `error "pair show witness forced"` still lets GHC print `built\n` through the
  original ``show value `seq` "built"`` shape. Printing the full string instead emits
  `F(` then fails at that intentional error. The original prefix-only probe has valid
  `main :: IO ()`, but does not force the quantified witness. Keep it as a laziness
  control; add a finite fully forced output at two type instantiations for the witness.
- Our renamed quantified-prefix control exits zero with stdout exactly empty. This is a
  runtime failure, but not yet a demonstrated failure of *applying* the quantified given.
- The ground generator skips nonempty instance contexts. Its separate list path generates
  specializations for four named element types and retains only context-class names. It is
  not general dictionary abstraction. The existing `conditional_dicts_chirho` metadata
  describes builders, so audit/reuse or replace that route rather than adding a second owner.

## Bounded implementation sequence

- [x] Read the current tasklist, four reference probes, dictionary producers and evidence transport.
- [x] Measure the ground context toggle and quantified prefix/force controls; retain exact bytes.
- [x] Choose the reversible runtime continuation and keep its scope separate from brick 8's claim.
- [ ] Claude: preserve all arguments and binder scope in ordinary/quantified predicates. A quantified
  premise is not a flat class name; absent or unsupported evidence must not silently become proof.
- [ ] Claude: show one shared full-predicate identity/evidence carrier from typing to Core. Preserve
  class identity, the ordered argument vector, lexical given/quantified binders and predicate index.
  Rendering is diagnostic text, not the join between an instance and its method bodies.
- [ ] Observe the c3 evidence/operands and recover both second-argument choices and their same-class,
  distinct-predicate controls. Recover c2's ground/default-superclass behavior separately; do not
  attribute either ground case to the absence of conditional dictionary functions.
- [ ] Generalize conditional instances into dictionary functions with context-evidence parameters;
  apply the selected instance's evidence at uses. Ground dictionaries are the zero-parameter case.
  Reuse stable definition identities between source lowering, typing selection and Core bodies;
  preserve superclass/default-method argument substitution and instance-local lexical scope.
- [ ] Demand-driven construction, finite/cycle-aware evidence search, and lazy recursive dictionaries:
  no Cartesian enumeration of concrete types, unbounded compile-time expansion or new type allowlist.
- [ ] Exact runtime outputs: list and non-list conditional heads, two instantiations of one instance,
  two predicates of one class, full MPTC arguments, default superclass projection, alpha-renamed
  binders, an imported class/instance, and a finite quantified witness forced at two types.
- [ ] Counterparts must reject missing/contradictory evidence for the intended rule; constructor and
  instance methods that intentionally fail must still fail. No silent empty program, invented
  method/dictionary identity, success-shaped fallback or loss of a reachable effect.
- [ ] Run focused gates while iterating; before any main landing run complete driver execution,
  curated/workspace gates and both corpus axes twice with exact membership/reason audits. None of
  today's bounded probes is a full-corpus claim or a waiver of main's regression gate.

## Alternatives and correction cost

Keeping only more ground string-key specializations is cheaper locally but retains the observed
identity and open-type limitations; it is not the chosen end state. Replacing the entire inference
solver/backends would be a different, unjustified scope. If the carrier changes after review, its
isolated checkpoint makes the correction local. Unresolved runtime cases remain explicitly open;
they are not relabeled as passing because a typing or unit suite is green.
