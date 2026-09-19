<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Class-method scope observations

Local diagnostic evidence, not a compatibility or main-line landing claim.
Every observation retains its exact source, SHA256, command, exit, timeout and
full stdout/stderr. Metadata records the tested binary hash before and after.

- reference-chirho.jsonl: thirteen sources through GHC 9.14.1 and parent CLI
  650be538. Ten valid sources; three GHC rejections.
- candidate-a-chirho.jsonl: the same sources through dffbc899, after hidden-kind
  linkage and batch ClassEnv transport, before method-universal rigidity.
- candidate-b-chirho.jsonl: the same sources through 5c145678 after rigidity.
  The twelve calibrated scope contracts agree with GHC: ten accepts, the K5
  kind error, and the method-parametric body error.
- dependent-reference-chirho.jsonl / dependent-candidate-chirho.jsonl: three
  additional dependent-class-head probes through GHC+650be538 and 5c145678.
  Both CLIs accept the valid shared-kind/two-instance case, reject the bad
  method result, and wrongly accept CChirho Int 'True (GHC-83865).

K6_neg_default_contradiction is NOT a calibrated method-body control. GHC rejects
its ambiguous class method before checking the purported contradiction, with
GHC-39999. Both parent and candidates accept it. The prediction and raw result
are preserved instead of rewriting the experiment. K2's original ambiguous
shadowing probe is excluded; K2b is the GHC-accepted corrected source.

The genuine method-parametric control is rejected by GHC-25897: an instance body
cannot specialize its method's forall b to Bool. Parent and candidate A accept
it; candidate B rejects E0200. Candidate B's hidden-kind cases also include two
instances at different kinds and a method-local forall shadowing a class kind
name. A declaration-only K7 is not proof of dependent instance-head validity;
the separate Int/'True counterexample explicitly bounds that claim.

The inherited instance-head representability gate and dependent monomorphic
class checking remain incomplete. This checkpoint does not relax or waive that
counterexample and does not claim runtime dictionary or full import-privacy
support. Full corpus membership and remaining broad gates are recorded in the
tasklist, not inferred from these selected cases.
