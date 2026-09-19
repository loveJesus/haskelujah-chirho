<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Class-method hidden kinds, row484

Resume isolated gpt-kind-schemes-chirho at 30b83956 under L.J.'s continue-dev
instruction. Main 6db522ad, peer branches, canonical DB and public measurements
remain untouched. Local pre-beta compiler work is reversible; no landing is implied.

## Placement and hypothesis

The retained eight GHC 9.14.1 graphs show four valid methods failing when their
class parameter has an inferred or explicitly polymorphic kind. An explicit
Type-kinded parameter and a value signature using the same family equation pass.
Trace the checked class-parameter classifier and method's hidden family input
through registration and instantiation. Do not erase hidden inputs, relax
family matching or infer a kind from a corpus name.

The first implementation choice belongs at the existing typing declaration /
method-scheme boundary, using the kind checker's represented evidence. Keep
new logic in a focused sibling module; do not grow the oversized infer root.
Decide exact metadata ownership from the trace before adding fields. Preserve
ordinary method variables, class-parameter order, source ownership and bounded
work per declaration/use.

## Checklist

- [x] Replay retained reference graphs on the immutable current CLI and add
 driver controls that demonstrate the valid failures and genuine contradictions.
- [x] Trace classifier/term identities and settle the smallest shared repair.
- [x] Implement without dropping or fabricating checked kind evidence.
- [ ] Run focused parser/naming/typing/driver, canaries and the live package
 control; compare complete corpus membership at the meaningful gate boundary.
- [ ] Retain source/hash/result evidence, limits and workflow; commit only owned
 paths, push and verify the exact remote tip.

## Acceptance and limits

Local and imported class methods must agree with GHC, with and without defaults,
at more than one parameter kind. A wrong associated result and an ordinary
method-polymorphism violation must still reject for their real reason. Test the
main frontend, not a temporary tree shape. T14441 is independent until evidence
joins it. Passing selected files never establishes the remaining corpus count.

## Resume state, 2026-09-19 16:17 EDT

The channel is the existing kind elaboration: class heads now publish ordered
checked kind binders separately from type-application heads. ClassDecl retains
their corresponding typing variables; methods use the same lexical seed map as
ordinary class parameters. Each checked instance occurrence supplies its hidden
arguments before the method scheme is specialized. Remaining method universals
are skolems, not flexible variables chosen by an implementation body.

The eight retained source graphs are tested through BOTH the file-check graph
and compile_modules. The latter previously failed to transport ClassEnv, so an
imported contradictory implementation could pass because its class was absent.
It now forwards the checked class environment like the file-based entry point.
This is a checking fix, not proof of runtime dictionaries or full import privacy.

Revision A (before method-universal rigidity) passed typing 407, core 128 and
typing integration 443; frozen CLI dffbc899494101b993f73d39728d20d90f29b8654a8dd3af763e11e688ffe6f5.
That CLI still accepted the measured invalid method-parametric control. Revision B
adds rigidity and passed four focused matching tests. Its fresh broad gates pass
typing 407, core 128 and typing integration 444, with zero ignored/filtered tests.
The fresh CLI is 5c145678e45ee3996fd22ce3b078c65e2986e4612b102c72da28e95a0b2ee495;
all twelve calibrated scope cases agree with GHC, with hashes stable before/after.
Never attribute revision A's broad gate to revision B.

Peer K2 and K6 negative predictions rejected in GHC for ambiguity rather than
the intended contract. K2b replaces K2; K6 is retained as refuted evidence but
excluded from the behavioral table. Twelve scope controls include the actual
parametricity contradiction and three family-method cases used at two kinds.
Three new dependent-class-head probes are measured separately before deciding
their scope; the parent accepts a head whose supplied kind and promoted argument
disagree, so that is not presumed fixed by the method channel.
Current CLI also accepts that bad dependent head; valid two-kind instances and
the genuine bad method result retain their reference verdicts. The new method
channel is not a claim of complete dependent instance-head checking.

Evidence lives beside the retained graphs in associated-methods-chirho/method-scopes-chirho.
Claude2's bounded read-only review #23794 found no capture/identity defect in
the method helper, confirmed the lexical ordering and noted its maintenance
dependence. The workflow now states that ordering explicitly.

No fresh corpus claim, canonical DB write, main merge, public-label change or
deployment. Broad current-revision gates, corpus membership, clippy and the live
constraints package remain explicit obligations before any landing decision.

## Storage interruption receipt

L.J.'s request to free disk space paused new builds. Owner-verified regenerable
outputs were removed with cargo clean using each exact cache directory as its
target; no source, worktree, branch, database, retained binary or scratch evidence
was removed. Allocated sizes before deletion, in KiB:

- haskelujah-gpt-chirho/target/debug/incremental: 6,201,068.
- haskelujah-gpt-flat-types-chirho/target/debug/incremental: 1,437,656.
- haskelujah-claude2-chirho/target: 2,312,760 (explicit owner clearance #23752).

All paths are under /Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho.
Total allocated footprint: 9,951,484 KiB, about 9.49 GiB. All three were verified
absent afterward; other target contents and Git status were preserved. A plain
rm attempt was denied and removed nothing; only the scoped Cargo cleans executed.
uv prune encountered an active lock and was cancelled without force or deletion.
AICEO owns the separate internal user-cache cleanup. Concurrent df changes are
not attributed to this lane; final receipts were posted as #23774 and #23775.
