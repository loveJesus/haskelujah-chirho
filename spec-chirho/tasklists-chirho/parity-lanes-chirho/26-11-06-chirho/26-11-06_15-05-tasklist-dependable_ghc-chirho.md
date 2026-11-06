<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Dependable GHC critical path Chirho

Authority: L.J., November 6, 2026: calculate the path to a dependable GHC and coordinate the fleet to optimize for it. Root owns this checklist, main, integration, the progress DB and consolidated reporting. Existing implementation owners and the sole compiler scheduler remain in place. Coordination is row 701; runtime measurement is row 702; package truth is row 703. Numeric entry remains row 692, types row 688 and Reader row 699.

This is a pre-beta compiler with local, reversible development work. Actual risks include silent wrong results, false acceptance, unbounded retention and false package-install success. Shared cache/data replacement, public release and other projects are outside this assignment.

## Destination and proof

The PRD's full destination remains a GHC-compatible compiler for real Haskell programs and Hackage packages, including the named transformers/mtl/parsec smoke set and lens milestone. The first usable profile is the measured Darwin arm64 host with interpreter, LLVM and actual Cranelift execution. Wasm and the other targets retain their project goals but have separate acceptance evidence. Completion of this profile does not establish the full destination.

The PRD names GHC 9.6/Haskell2010 plus common extensions; current reference receipts use GHC 9.14.1. Record the exact reference version, extensions, package versions and platform for each workload. Do not silently change the compatibility contract or mix results across versions. Source compatibility does not imply GHC object/interface ABI compatibility.

A dependable milestone must demonstrate all of the following on one identified candidate:

- Valid supported programs build, link and produce the reference's stdout, exit, files, effect order, laziness and exceptions. The selected backend really executes; a fallback is a separate outcome.
- Invalid programs are refused at the appropriate producer/checking boundary. A runtime missing dictionary, unsupported feature or timeout is not successful type-system coverage.
- Real package bodies execute. Resolution, interface availability, typechecking, source registration, binary installation and application execution are separate claims.
- Bounded-live-set programs retire dead objects and roots; sharing and recovery preserve behavior. A correct small checksum alone cannot establish this.
- A clean installed CLI and matching RTS work outside the checkout. Clean and incremental builds, failed builds and interrupted publication have explicit behavior.

No overall percent-complete or delivery date is justified yet. Track wrong acceptance, wrong observable behavior, real package executions and remaining blocking causes separately. Published compatibility counts remain owned by the two committed measurement artifacts and their paired QUOTE-AS rule.

## Starting evidence, November 6

- Main `1c42c54724d583aa775543483d1434d5826bf52c` has compiler crates `a9a38e3ddd40b7bbf4782564d66b9556bbd85b07` (A9). The later main commits are records; frozen `39e26ab9` measures the earlier D8 compiler and is not the current-main executable.
- Main's paired eval-basic run measured 169 passes and 164 failures: 163 HeapPtr-versus-Int assertions and a separate mapM empty-output failure. Numeric-entry investigation already found retained dictionary lambdas; blindly forcing/unwrapping heap values is not a repair. Pure evaluator expressions and GHC-valid IO executables need distinct contracts.
- Reader `54dd469f` is isolated. v7 measured corrected W3/W4 and native/IO/merge controls; v9's eleven search-path controls and lint pass are owner-reported at their thin scope. RD3/RD4 false acceptance and LLVM boxed-Show remain open. Final combined preservation, release and corpus reach are unmeasured.
- Kind/identity source remains isolated. Finish its current bounded grouped-comprehension brick, then converge. The unchanged grouped-column case exposes a real missing `groupWith` body; a source-owned transformer contrast cannot close that supplier gap.
- Source audit confirms the installer can register success after compilation errors, invents nonexistent interface paths and records wrong dependency versions. Claude2 reports the old shared DB has 410 entries and no claimed interface artifacts; that read-only audit does not authorize modifying it.
- Cabal dependency collection retains frontend artifacts but discards dependency Core. A package typecheck is therefore insufficient proof that its bodies link and run. The native CLI also depends on a checkout-relative RTS path.
- Native roots are retained beyond their useful lifetime, including across self-tail loop backedges. The current GC/thunk machinery already has substantial implementation and controls; this calls for measured lifetime/sharing repairs, not a collector rewrite based on stale PRD wording.

## Parallel ownership and next deliverables

| Owner | Active brick | Acceptance and next handoff |
| --- | --- | --- |
| Root, HASKELUJAH/gpt_chirho | Current A9 baseline review, isolated integration and acceptance | Resolve actual kind/Reader producer seams, independently review failures, freeze one candidate with matching RTS, then land only its supported scope. |
| Kind writer, row 688; sole scheduler | Close the current grouped typing/resource-budget brick; prepare fresh A9 baseline using the existing controllers | Preserve the real groupWith gap. Coherent source checkpoint, ordinary preservation and explicit collector/identity/default handoff precede another syntax family. |
| Claude, row 692; Reader handoff | Trace constrained numeric entry from checker scheme through dictionary arguments to evaluator result | Name the public API/program contract and shared causal repair; preserve function/action/data and numeric ambiguity negatives. No blanket HeapPtr unwrap or expectation change. |
| Claude2, row 703 | Truthful source-package installation and publication, then package execution | Actual input/check/save failures refuse without damaging prior published state; no fictitious .rhi/object claims. L0, own-library L0.5 and external-body L1 become executable acceptance. |
| Native measurement worker, row 702 | O(1) RTS measurements and bounded allocating self-tail stress | Actual allocations and several collections required; compare N/4N peak roots/live memory with a fixed live set. No root-lifetime, CAF or collector change in this first checkpoint. |

Assignments are delivered as broker #32492, #32493, #32495, #32500 and internal owner handoffs. Root verified both external workers actively reading their assigned sources; Claude2 returned the concrete package audit in #32497/#32499. New implementation work stays in owner worktrees under `haskelujah-workspaces-chirho/`.

## Ordered convergence gates

- [x] Read live Git/PRD/task records and identify source-specific evidence; preserve existing holds.
- [x] Assign disjoint numeric, package and runtime work; keep one compiler/RTS/GHC scheduler.
- [x] Prepare a fresh actual-main A9 baseline packet rather than relabel the historical D8 binary. Owner plan hash: `bc384fa6e60a59025f24873726da6c3c4409f88409c50625328c0679793354d5`; prepared, not launched at this record.
- [ ] Execute the reviewed fresh A9 baseline at a granted window; retain exact compiler/RTS/toolchain/corpus/stdlib/cache identities. Attribute resource/protocol failures without converting them to type refusals.
- [ ] Finish the current coherent kind brick, preserve ordinary-program behavior and expose all still-open valid/invalid disagreements. No new feature family ahead of convergence.
- [ ] Integrate Reader with that checkpoint in a separate root worktree. The current pushed-tip preview has eight conflict paths: generic heads, dict context, backend compile/evidence join, module scanner, driver lib, the extracted class monolith and builtin-identity workflow. Move the seeded Monad default to the owning class child; preserve both identity and default authority.
- [ ] Review the resolved type/evidence/superclass/default/supplier transport with both writers; compile and run the affected ordinary controls on the combined source. Textual merge success alone is insufficient.
- [ ] Repair row 692 at its measured semantic boundary. Evaluate explicit-typed values, constrained ambiguity, genuine closures, data and IO twins; retain the separate mapM IO failure until causally repaired.
- [ ] Close RD3/RD4 through complete typing obligations, not a partial Core instance view. Resolve boxed-Show through its actual representation/consumer contract. Attribute the five unmeasured eval-advanced numeric cases using a real parent comparison when that evidence is needed.
- [ ] Complete package truth: strict load/check/save failures, genuine dependency versions, supported generated Cabal modules, staging/atomic visibility and tested recovery. Source capability is recorded honestly; prior cache/DB remains untouched by probes.
- [ ] After the exact collector handoff, retain and link non-builtin dependency Core with package/module identity. Keep the kind writer's selected-import schemes and class/family/source fields intact. Prove an external body actually changes application output.
- [x] Measure root retention. Root verifies all 105 retained files, nine stage receipts, three exact-name lists and four native reports: four times the iterations retain four times the post-collection objects in both backends. Measurement row 702 closes as a confirmed RED, not a repair.
- [ ] Implement scoped frames with safe return-value and tail-argument transfer, then test escaping results through forced GC. Removing roots without transfer proof is unsafe. This ordinary lifetime repair is row 704; collector, representation and CAF changes are excluded.
- [ ] Prove per-binding CAF sharing separately: repeated demands do not re-evaluate the RHS, same-spelling bindings remain distinct, recursive CAFs report a blackhole, and reusing an IO action repeats its effects. Reuse existing thunks; do not memoize effect results.
- [ ] Freeze the integrated candidate and run the unchanged positive/negative compiler axes plus real execution workloads at the meaningful landing boundary, under existing authorization/resource rules. Attribute each mover to its GHC contract; keep Unsupported and unclassified resource failures distinct.
- [ ] Package a relocatable CLI/RTS/store configuration and run the same application outside the checkout, including clean rebuild, edited-module rebuild, invalid input and failed/interrupted publication.
- [ ] Publish only the acceptance scope actually established and release the next package/profile work from that evidence. No unexplained correctness failure in a claimed supported scenario.

These gates express dependencies, not a global lock: source diagnosis, fixtures, package truth and runtime measurement proceed while the current type brick closes. Root integration can begin after a coherent checkpoint; it does not wait for all future Haskell features.

## Real package ladder

Use source-only scratch copies pinned by content tree hash; exclude old `dist-chirho` artifacts. That proves equality to today's cache, not independent Hackage authenticity. No shared store writes or network access during this unit's probes.

1. L0: a useful three-module app on base; prove build, link, actual backend and output.
2. L0.5: unchanged `haskell-say-1.0.0.0` own-library executable, exercising its library-to-executable link and supported Cabal generated-module behavior.
3. L1: an external app that invokes that real package's body, with exact reference output. This must fail until dependency bodies are retained and linked.
4. Expand by dependency seam: cross-package operators/fixity, dictionaries, numeric methods, transitive dependencies, then the PRD smoke packages and lens. Pin each exact version and demanded functionality before selecting it as acceptance.

Each rung includes a type error and an unsatisfied dependency bound, a failed build with no runnable stale output, clean rebuild, and an edited-module rebuild. Capture GHC reference execution, not just `-fno-code`. Unsupported Cabal flags must be explicit; do not silently imply threaded RTS compatibility.

## Work and verification discipline

- Optimize time to an accepted, integrated useful program. Agent utilization, test totals and moved-file counts are not progress measures by themselves.
- One coherent semantic brick per implementation writer; one prepared next brick. A shared-file seam needs an exact handoff, not another competing implementation.
- Keep workers productive with bounded source/fixture/review work while the sole scheduler grants actual compiler windows. Do not launch more inference work merely to fill seats.
- Use focused checks during repair and broad/adversarial checks on the combined candidate. Reuse the current runner/controller; new harness behavior needs a failing control, but routine pin changes do not justify a new protocol.
- Stop elective structural extraction for now. Necessary separation inside an active repair still applies; stale oversized files remain debt, not permission to add another responsibility.
- Preserve old failure receipts and scopes. Correct a wrong prediction directly; do not weaken its assertion or call a different positive control a repair.
- Outgoing coherent source is committed/pushed with scoped checks and gitleaks. Main/SQLite remain root-only. An isolated push or a thin green packet is not a dependable release.

The existing [landing workflow](../../../workflows-chirho/development-chirho/parity-review-chirho.md) and [execution oracles](../../../workflows-chirho/testing-chirho/execution-oracles-chirho.md) govern delivery. Primary reference distinctions: [GHC compilation phases](https://downloads.haskell.org/ghc/9.14.1/docs/users_guide/phases.html) and [Cabal build/run/test commands](https://cabal.readthedocs.io/en/stable/cabal-commands.html).

## Producer diagnosis and active handoffs, 15:27 EST

Claude #32507 traces `main = id 7` to missing monomorphism restriction at generalization: typing produces a constrained polymorphic entry and the downstream dictionary/WHNF machinery faithfully returns a function. Root reviewed the source and [Report section 4.5.5](https://www.haskell.org/onlinereport/haskell2010/haskellch4.html#x10-930004.5.5); this is a causal source diagnosis, not proof that a repair fixes every numeric failure. The separate empty-output mapM case is a predicted same-boundary consequence until measured.

- Kind's coherent source is pushed as `29a5b2b272df9624ff03cbd38e47f88462373e4a`, crates `6f327fa57ca7dab79bb4cdb33ca26bacd3ce7a58`. Owner reports 1,836 library passes and 18/19 grouped controls, with unchanged missing groupWith and one naming-policy Clippy finding retained. No complete engine acceptance follows.
- Kind explicitly lends Claude the SCC/generalization/default-finalization regions on that exact checkpoint, including only necessary context tracking. Broker #32510 clears ordinary MR implementation there. MR withholding is distinct from representation/skolem protection; classify whole groups, preserve signatures/NoMR and name incomplete pattern/nested coverage.
- New MR defaults retain actual predicate identity and shape. Source/unknown class homonyms gain no compiler authority; candidate satisfaction must not recreate unproved predicates. New MR variables bypass the old automatic Monad-to-IO, custom-class leniency and Integral/Bounded exception. Module-local uses resolve before final ambiguity refusal. A real executable entry's IO requirement is separate from a generic binding named main or an evaluator API entry.
- Root's separate `haskelujah-dependable-integration-chirho` worktree is assembling kind29 plus Reader54dd; exactly eight conflicts confirmed, no canonical compiler change. The scanner migration returns the existing successfully reached/admitted source set only after final atomic interface publication, preserving supplier identity without an origin/path heuristic.
- The fresh A9/kind29 preservation window is actually running under the sole scheduler: epoch 1793996669 to outer 1793998169, internal 1793998139. Plan `a74cefa8` supersedes the earlier unrun preparation; windowed checkpoint `c50cc292` adds a full-original-bound check before each stage without shortening. Owner subsequently reports a fresh warning-free A9 release frozen as CLI `1ae0f2f48e0b6720ab7bdb3e2e989b668cbd96055506cdd70f6b17950520c0e7` with all 66 standing diagnostic cells complete. Its compatibility disagreement is retained as a diagnostic, not a baseline build/protocol fault. Feature checkpoint and four-axis completion remain unmeasured at this cut. Canonical compiler stays A9.
- Native measurement source is pushed as `cc2f44d0`, crates `1649caab33d1b3645fe4071482af36dd385100e4`, independently reviewed and queued after that window. Its six commands and nested RTS build use locked/offline/two-job policy. No actual measurement or lifetime/CAF repair has run.

Package row 703 remains an active disjoint implementation. These advances release concrete downstream work; they do not close numeric, Reader, type-engine, package or runtime acceptance.

## Execution checkpoint, 16:23 EST

- Root independently confirms native measurement source `cc2f44d0`, exact linked RTS `8d232a04`, all four successful outputs `35`, and real collection counts 5/7. LLVM peak roots grow 32,783 to 131,087; Cranelift 28,684 to 114,700. Post-GC registered objects grow 16,000 to 64,000 in both. Root proof `9570f9d` is retained under `tmp-chirho/row702-root-review-chirho/`. These are runtime allocation/root counters, not RSS. Row 704 is implementing explicit LIFO/owner-thread frame boundaries and safe return/tail handoffs in isolated source.
- Kind/Reader assembly `5e7f86f3`, crates `8cd86a61`, passes the actual preliminary workspace all-target check with zero warnings. Both writers reviewed the original contracts; source fixes preserve the original failed check and add a declaring-class arity guard. The isolated branch is being published with its source/check record; main compiler remains A9. Combined behavior, Clippy and final preservation are still required.
- Current-main A9 versus kind29 axes have 356 directory transitions. Contract review separates configured expectations, retained executed references and unverified rows; 155 intended-positive compile cases need attribution and shared repairs. A fresh 397-case replay is a separate diagnostic, not the whole engine's acceptance. Minimal DatatypeContexts/NoRankN consumer policy fix `1bea865e` is independently measured by its owner and included in the isolated assembly. Pattern-synonym existential handling remains a separate repair with a falsified alias-erasure prediction retained.
- MR source `85bc27d4` is pushed but not yet compiled at this cut. It includes real seed-context identities and the explicit program-entry purpose. Program and evaluator callers have different contracts; the original polymorphic binding is preserved. Typed entry-call dictionary elaboration, signed/non-simple patterns, nested restrictions and identity/primitive/rebound gaps remain open. Root authorizes intent-based test-call adaptations while preserving refusal diagnostics, and requests an unrelated Read/Show ambiguity control rather than treating result-connected variables as full group coverage.
- Package A `488c1265` corrects source publication/dependency truth; B `ccd47356` uses one producer for frontend artifacts and dependency Core, with topological linking and explicit name-collision errors. Both are uncompiled at this cut. Without declared builtin capability versions, nontrivial builtin bounds are refused, including haskell-say's base bound. That missing metadata is not fabricated from CPP knobs or a reference GHC installation. Source-check-only Paths getters remain explicitly limited. The real package ladder inputs are committed; reference outputs and execution are still pending.
- The sole scheduler has an actual 25-minute sequential preliminary compile window for immutable MR85 and packageB snapshots, each with its own full 600-second bound and cleanup reserve. No behavior/native/reference result is inferred. Later source changes remain separate from those pinned gates.

## Functional gate handoff, 16:44 EST

- Root's isolated kind/Reader assembly is pushed at `3a55090aa08d191a49e1d9438734046e26c3c088`, with the same compile-checked crates `8cd86a61d5c370e8c073d0aad50cdba2d8fb4b53`. Its workspace all-target check passes with zero warnings. Combined behavior and final preservation remain pending; canonical compiler source remains A9.
- Actual preliminary MR `85bc27d4` workspace all-target check returns zero errors and warnings. The later follow-up `7075a7df`, crates `c099e182`, adds the restricted-group residual control and explicit executable-purpose call sites. Its 27 pinned GHC controls contain 14 accepted executions and 13 refusals. Root verifies all fixture hashes and clears reference measurement through the sole scheduler. Functional measurement awaits the established all-exit owned-group lifecycle; source predictions and the three named open controls are retained.
- Package B `ccd47356` passes its preliminary workspace check with one real unused-wrapper warning. Commit `100b5c9e` removes that wrapper and adapts its three test callers without a suppression; it is not yet recompiled. Root clears meaningful functional measurement after splitting packet `3e42b4e2` into schedulable phases and applying locked/offline/two-job policy to the own RTS prebuild. A 1,800-second stage cannot fit an already-started 30-minute window with cleanup; a 1,500-second final Clippy cannot be promised in the same plan. Real GHC/package execution is still pending.
- Native repair `39a5a806`, crates `5bde3535`, now passes the actual workspace all-target check in 61.295 seconds with zero Cargo JSON warnings/errors, stable source pins and an empty owned process group. Receipt `b4754d68` is retained in the detached preliminary gate. Independent source review finds no concrete blocker in inspected frame ownership, caller result, self-tail, thunk/PAP or construction handoffs. This establishes compile-reviewability and bounded source review only. The owner is preparing frame, emitted survival, effect/failure and unchanged row-702 growth measurements before any runtime PASS.
- Broker incident `32548`: Claude2 reports two ungranted Cargo builds accidentally ran in canonical main because an unquoted zsh message heredoc evaluated backticked commands. Root confirms main still has only its own DB/tasklist changes. The mutable `target/debug/haskelujah` is replaced and cannot represent its former bytes; no restoration or new main build is attempted. An independent read-only audit checks the frozen baseline and runtime evidence. Message bodies now use Python-written files or quoted heredocs. Incident recovery is row 705, separate from semantic compiler acceptance.

The next measurements are MR reference/functional behavior, package source/build/run truth and native root retirement. Main source is not advanced merely because isolated source compiles.

The subsequent MR GHC-only return matches all 27 predictions: 14 accepted reference executions with exact stdout and 13 refusals. Root verifies the retained summary `d06fdb06`; no Haskelujah behavior follows from that result. The incident audit also completes: frozen A9/candidate pins, 392 isolated integration artifact paths and all 105 native-702 manifest files remain intact. Row 705 closes with no named packet invalidated; only the former mutable CLI identity is excluded. The audit proof is retained under `tmp-chirho/coordination-incidents-chirho/32548-incident-audit-chirho.json`.

## First package functional return, 16:58 EST

Phase-one v2 on `100b5c9e` stops as a finding: 99 package-library tests pass; the driver builds without warnings and maps all 67 requested names. The first 40 driver controls have 39 passes and one associated-type-family transport failure involving a rigid kind variable and imported `PrimState`. Root verifies complete exact-name outcomes and clean source/process receipts; Cargo's final `test failed` line is not a Rust compiler diagnostic. The generated Paths-module importer and dependency-Core structural controls pass. No native package body execution follows from this scope. The remaining 27 are unrun and receive a separate bounded continuation; the original failure and stop remain unchanged. Source owners inspect the family seam before claiming a package regression or inherited status.

A separate read-only review confirms fallback interface union can retain a shim-only export or earlier value metadata over a same-key selected package module. A later repair belongs at the dependency producer, preserving the generic fragment-union contract and different package exposures. Interface exposure, executable bodies and link-only RTS supply remain distinct; module counts are inventory, not capability or version proof. This finding does not expand the frozen functional gate.
