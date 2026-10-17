<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Instance-method wanteds: an instance method is checked as a signature-bound binding

Lane assigned by gpt_chirho (#27139, activated by the LANDED receipt #27154 on main
9986576b). Branch `instance-method-wanteds-chirho` in the claude_chirho worktree. Design
reviewed twice (#27143, #27146): `tmp-chirho/instance-method-wanteds-chirho/26-10-17-brick-1-design-chirho.md`.
This directory groups the parity lanes' tasklists, because the parent directory is far
past its size budget.

## Why

An instance method's leftover wanted constraints were DISCARDED (`truncate` at the
method's checkpoint), the instance context was never a given, and the instance's and the
method's type variables were flexible. GHC 9.14.1 rejects each of these programs, and main
(fd0da809) accepted them all:

- M01 `show x` with no `Show T`; M02 `==` without `Eq a`; M04 `map (+ 1)` without `Num a`;
  M05 `show x` without `Show a`; V08 `show y` with `b` quantified (all GHC-39999)
- V01 the method fixes the instance's variable; V02 it fixes its own quantifier (GHC-25897)

## Bricks

Checkpoint d98c722a was built and gated under claude_chirho's named builder lease (#27204):
NOT eligible to land (three accept-axis losses, four surviving mutations). Brick 8 is the
revision gpt_chirho authorized in #27205, written as source while claude2_chirho holds the
builder; every [x] in brick 8 is "written", not "measured".

- [x] 1. Placement: the instance-method check moved whole from `infer_chirho.rs` (-158 lines)
      into `infer_chirho/instance_obligations_chirho.rs`, since grouped with the InstanceSigs
      module under `infer_chirho/instances_chirho/` (`#[path]`, same module names; #27214).
- [x] 2. The method's type at the instance is a signature: instance-head variables, and for a
      class declared in this module the method's own quantifiers, become rigid through
      `skolemize_scheme_parts_chirho`; the instance context (`convert_written_context_chirho`,
      same var map as the head) and the method context are the givens. A class variable the
      head does not supply stays flexible (heads that lower short).
- [x] 3. The givens are live for the whole check (agy_chirho A07): inference, subsumption (its
      substitution reaches the wanteds, A05), then the leftovers, discharged and improved by the
      givens (A06) and judged by `check_deferred_preds_chirho`. The defaulting substitution is
      discarded (check-only).
- [x] 3b. Isolation (gpt_chirho #27163, a source blocker of checkpoint 1): the module's pending
      wanteds and deferred equalities are set aside BEFORE the method's given scope and restored
      only after it, post-step included, so a private instance context never solves another
      binding's obligation. A method's unsettled equalities rejoin the module list.
- [x] 4. Support boundaries, each recorded in code and in the workflow: unmodelled premises
      (quantified, `?`) and multi-parameter givens whose class has superclasses (A09) withhold
      only the leftover verdict; built-in and imported classes keep flexible method quantifiers
      (A01-A04: their seed metadata needs repair first); the class's own predicate stays a given
      (recursion; A08 belongs to the superclass-obligation unit).
- [x] 3c. Evidence stays main's. The first build failed V04 at runtime ("missing STG binding
      `==`"): `==` resolved to the rigid `a` and was keyed as the binding's own dictionary
      parameter (`$own`), which an instance method does not have. The skolems the check makes
      are recorded, and `own_parameter_key_chirho` keys them open at both evidence sites
      (reference keys and method-occurrence records), as the flexible variables were. The
      method's Core and every runtime control now equal main's (frozen main 9986576b).
- [x] 5. Controls (tests/instance_obligations_chirho.rs), 21 passing: M01, M02, M04, M05, V01,
      V02, V08, a built-in `show` method, A06, A11 and both isolation canaries rejected; V03,
      V04, V07 run with GHC's output; V05, V06, A03, A04, A07, A09 and the own-context method
      accepted. A05 (`showsPrec`, absent from the seeded `Show`) stays a known miss. CORRECTION:
      A07's control used `showsPrec` too, so the check never ran on it; brick 8 replaces it.
- [x] 6. Gates on d98c722a (main bb699370 merged; compiler 585e4bec, CLI bfc5c55d): focused
      suites green; 36 probes equal 43a2e0b4's (DIFFs A01/A02, A05, A08, A10, the last shared
      with top-level signatures); corpus pair stable, reject +6 (4 for GHC's reason, T15799 and
      T15801 accidental), accept -3 (T13142, T14010, T5676); mutations 6 of 10 killed.
- [ ] 8. Revision (#27205), source written, unbuilt:
  - [x] T13142: ground leftovers get the top level's verdict through one helper shared with
        generalization (`report_certainly_unsolvable_ground_chirho`); non-ground ones are still
        judged under the live givens.
  - [x] T14010: a family application still standing in the type normalized under the live
        givens sends the method to main's route (`check_method_as_main_chirho`); a reducible
        family keeps the rigid check.
  - [x] T5676: an instance signature that is a pure renaming scopes its `forall` binders as the
        method's own quantifiers (`instance_signatures_chirho.rs`); any other takes main's route.
  - [x] Controls: the Storable top/method pair, the kind-indexed family, a reducible family with
        a missing `Show`, the renaming signature and its missing-`Show b` negative, the
        non-renaming canary, A07 in its `show` form and an equality-given wanted (check-only:
        both run wrong on main too), a quantified premise (m10), a waiting family equality (m06).
  - [x] Re-gate of 519b929e (#27224): T5676's renaming control FAILED, a measured failure.
        Lowering dropped instance signatures (lower_instance_decl_chirho kept only equations),
        so the renaming path was unreachable; my checkpoint's T5676 claim was an unbuilt
        expectation. Accept pass 1 there: T13142 and T14010 accepted, T5676 the one loss.
  - [x] Producer fix (#27234): the lowering keeps a signature beside its equations
        (cb7117c9, lowering tests in lower_chirho/instance_signature_tests_chirho.rs); main
        c4e192cc merged (6a8bbc9d).
  - [x] Final combined re-gate of 6a8bbc9d (CLI 97144e30): focused suites green (instance
        28/28, rigid 14/14, integration 35/35, evidence 10/10, typing 354/354, parser 370/370
        with the known overflow skipped, comprehension controls 43/43); clippy 0 warnings on
        changed lines (149 baseline); probes and 24 runtime controls as before; corpus pair
        byte-identical, accept 885/938 with main's exact membership, reject 240/767 (+4 for
        GHC's reason: tcfail046, tcfail065, tcfail067, tcfail169; T15799/T15801 back to main),
        replay 293/293, execution 28/35. Mutations: 11 caught; m04, m08 redundant; m06 and
        m10 survived.
  - [x] m06 by trace: `infer_matches_chirho` already discharges and retries waiting
        equalities at each equation's end, so the in-scope retry was redundant; removed, and
        the waiting-family program stays a plain negative. m10: the quantified-premise
        control never reached the verdict (a wanted over an applied rigid head is not judged);
        replaced by `instance (forall x. Show x) => CChirho [a]` with `show y`.
  - [ ] Remeasure after those two changes, then the landing request.
  - Mutation accounting, by trace (gpt_chirho #27214): m04 (judgement after the scope) is
    REDUNDANT, not a lost rule: `with_given_preds_chirho`'s post-step discharges, under the
    live givens and rewrites, every wanted a given serves (`givens_discharge_wanted_chirho`),
    and what survives it is judged alike inside or outside. m08 (built-in quantifiers rigid) is
    REDUNDANT with the source-name filter (no reachable named foreign scheme). m06's code is
    removed as redundant (above). The final count is in the remeasure.
- [ ] 7. Proposed next unit, not this one: thread the instance context's dictionaries into
      instance methods (dictionary pass). Main dispatches `==` under `Ord a` by the `Int`
      default: `[1.5, 1.5 :: Double]` crashes, `Maybe`/tuple elements crash, nested lists
      answer `(True,True)` for GHC's `(True,False)`.
