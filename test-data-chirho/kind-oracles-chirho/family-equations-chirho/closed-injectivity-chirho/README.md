<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Closed-family injectivity Chirho

Claude's references are independent GHC9.14.1 executions. `references-chirho.jsonl`
contains15 unchanged Bak/Foo/Bar sources, predictions, commands and full output;
`reference_chirho.py` is the original runner. K4 requires coverage by the whole
earlier prefix, not only the comparison partner. K3 forbids a later row supplying
coverage. F4/B4 require unreachable rows to stay unreachable to consumers.

`before-chirho.jsonl` is the explicitly built67762401 CLI on those15 sources:
11 verdict agreements. K0/K4/F0 are false declaration rejections; B0 reaches an
ordinary type-inference failure. F4 already rejects, but for the wrong reason.
The first15 integration controls passed6/15 before the repair by their stronger
reason predicates, not11/15. Verdict agreement and reason agreement differ.

The shared bounded validator now checks RHS-instantiated inputs against any
strictly earlier covering row. Only validated local closed-family dependencies
reach ordinary type inference. Ordered source-row occurrences bind the proof
to the typed body; missing/reordered/different rows do not qualify. Inversion
excludes unreachable rows and requires forward confirmation of the original
equality before committing substitutions. Written annotations alone are not
proof, and non-determining inputs remain unresolved.

`counterparts-chirho/references-chirho.jsonl` preserves8 more independent records.
N1/N2/P1/P2/P3/W1 supply6 integration controls for absent/invalid annotations,
partial injectivity and contradictory consumers. H1/H2 are invalid hidden-kind
sources, not positive contracts; they are retained rather than quietly repaired.
The separately preserved24 hidden-kind exploratory records likewise include
refuted predictions. Their GHC verdicts are observations, not evidence that this
compiler implements those cases. The valid H9d hidden-kind consumer remains a
named unsupported shape; no hidden-kind improvement is claimed here.

K5 is rejected by the determining-variable rule before pairwise overlap. GHC
names overlap first; these are different diagnostics of an invalid injectivity
promise, not byte-identical reasons. Imported/open-family improvement and
family-to-family cancellation are outside this repair. T6018/T6018a still fail
at open/imported family consumers after the repaired closed cases pass.

The corpus runner records complete verdict sets, input hashes, CLI/source
provenance, timeouts, unexpected exits and actual runtime panics separately.
It uses the published shell detector and checks an independent literal-byte
equivalent. The initial additional panic classifier was too broad: source
comments quoted in T12966/T21338 diagnostics matched it. Both outputs contain
real compiler errors, not runtime panics. Raw intermediate evidence is retained;
the corrected classifier requires a Rust runtime panic/overflow header.

These are branch diagnostics, not a published compatibility measurement or
main landing. Main's artifacts and denominators are unchanged. Regression-set
membership, not the aggregate count alone, controls readiness.

## Frozen checkpoint evidence

Parser370, naming137, typing398, integration316, canaries7 and full driver1776
pass, zero failed/ignored/filtered. Workspace all-target check, explicit CLI build
and formatting pass. The twelve modified source hashes in `gates-chirho.json`
still match. All-target parser/naming/typing/driver Clippy exits0 with465 warning
message lines including duplicates and summaries; this is not lint-clean.

All23 fresh CLI reference verdicts agree with their GHC observations. The exact
execution control in `counterparts-chirho/execution-chirho/` prints
`'c'\n1.0\n()\n` under both GHC9.14.1 native execution and the candidate's
STG `run`. This is not native-backend or imported-family execution coverage.

The final CLI SHA256 is
1f478b55aef03b87a00cae51fb62138e604903ad9fb7af5f2d99deef9da66bd1.
One final frozen diagnostic per axis reports883/938 and259/767, with no timeout,
unexpected exit or runtime panic. Two earlier pre-row-alignment diagnostics
produce identical verdict sets; their summaries are kept separately so they are
not mistaken for repetitions of the final binary. Final complete per-file
outputs, source hashes and exact sets are in `corpus-chirho/final-chirho/`.

A fresh67762401 parent build reports883/938 and260/767. Its accept set is
identical. The sole changed reject verdict is T23162c: the parent rejects the
valid Bak declaration with the old single-covering-row rule; candidate, main
and GHC9.14.1 accept the unchanged program. Upstream `all.T` explicitly uses
`compile`, not `compile_fail`, despite the file's directory. No denominator or
oracle has been changed. Parent provenance, complete verdict sets and that raw
diagnostic are in `corpus-chirho/parent-chirho/`.

Relative to committed main artifacts, these are15 accept gains and14 losses,
and47 reject gains and9 losses. Reject deltas are verdict counts, not audited
capabilities. The14 accept regressions still prohibit a main landing:
CoerceToVDQ, T13585/T13585b, T13879, T16204a/T16204b, T17067, T18129,
T20588d_aux, T20661_aux, T22560c, T23543, T26358 and T6018a.

The separate fresh-main bracket also settles T12102: main6db522ad emits real
E0300 diagnostics at constructor applications on lines18/19; GHC9.14.1 rejects
the illegal constraint in the declaration kind on line17 (GHC-01259).
The candidate accepts, as already recorded by the earlier ascription repair.
This is an accidental old rejection disappearing, not panic-only false credit
and not a previously implemented GHC-01259 rule. The general loose panic-word
detector remains a measurement limitation; no public count changes follow from
this bracket.
