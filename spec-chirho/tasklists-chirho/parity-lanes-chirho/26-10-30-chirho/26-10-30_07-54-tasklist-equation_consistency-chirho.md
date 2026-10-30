<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Inferred equation consistency Chirho

Row 684 is an isolated prototype from the reject-frontier audit, under L.J.'s instruction to repair the negative cases as well as valid-source losses. Production home: the existing typing `infer_chirho/terms_chirho/matches_chirho.rs`; controls: a leaf under typing integration's `declarations_chirho/` category established by row 683. No new root target. GPT owns the source, main and ledger; Claude2 owns the builder for row 683 until its explicit release. No compiler execution during that lease.

The observed code discards failures while merging inferred equations' parameter types and reports result merge failures only at arity zero. The authoritative signature/expected-type path already keeps equation refinements independent and is outside this proposed change. Exact source witnesses are tcfail002 (`c (x:y) = x; c z = z`) and tcfail013 (`f [] = 1; f True = 2`). A fresh GHC oracle and complete both-axis comparison must establish the actual effect. GADT inference, local expected types, recursion and ordinary overloaded equations are positive risks, not assumed holds.

- [x] Read the exact failure paths and separate them from signature-seeded matching and observational evidence capture.
- [x] Independent source risk review identified unsigned GADT index diversity and expected-type fallbacks; signed GADT holds alone would miss it.
- [x] After #30833 SLOT-FREE, build an isolated candidate CLI and run six fresh GHC/baseline/candidate cases. Stop before a full sweep when the decisive positive fails.
- [x] Hold the prototype on the measured valid-source loss. No broad syntax exception was added.
- [x] Retain the patch and frozen CLI, restore the production source byte-identically, and record the exact failed boundary. No semantic landing or corpus gain is claimed.

This is not permission to silence GADT errors, reject every refined branch or change the case/do evidence-capture code. No hypothetical number is a measured gain.

## Held result

The prototype reports both inferred-parameter and inferred-result merge failures. It has a warning-free CLI build and rejects tcfail002 and tcfail013 as GHC does; the baseline accepts both. It also rejects a **GHC-valid local** GADT function: two local equations `tagChirho IChirho = (); tagChirho BChirho = ()`, used by an enclosing Int-valued binding, with I :: Tag Int and B :: Tag Bool. Baseline accepts it. The corresponding unsigned top-level example is rejected by GHC, so that example alone would have falsely approved the change. tc221 remains accepted by all three; its passing status cannot cover the local counterexample.

`tmp-chirho/equation-consistency-chirho/summary-chirho.json` records all exact source hashes, commands and return codes; raw diagnostics sit beside it. The GHC-accepted local case is the blocker. Proper branch-local refinement/expected-type provenance must precede reporting every mismatch. The source patch and candidate CLI remain under that directory; the typing source is restored exactly. No broad gate was run on a known invalid candidate.

The same run freshly checks row 682's committed UNRELATED_SUMS source exactly, including its actual pragma list: GHC, baseline and candidate all reject it. This closes the earlier one-pragma difference between that constant and the scratch oracle without altering the test or claiming a semantic change.
