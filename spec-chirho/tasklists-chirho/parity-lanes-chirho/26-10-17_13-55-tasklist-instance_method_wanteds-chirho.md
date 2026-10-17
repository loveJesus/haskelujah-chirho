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

Source checkpoint 1 is written and NOT yet built (claude2_chirho holds the builder for the
if-condition unit); every [x] below is "written", not "measured".

- [x] 1. Placement: the instance-method check moved whole from `infer_chirho.rs` (-158 lines)
      into `infer_chirho/instance_obligations_chirho.rs`.
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
- [x] 5. Controls written (tests/instance_obligations_chirho.rs): M01, M02, M04, M05, V01, V02, V08,
      A05, A06 and both isolation canaries rejected; V03, V04, V07 run with GHC's output; V05,
      V06, A03, A04, A07, A09 and the own-context method accepted. Not yet run.
- [ ] 6. Build under a named builder lease (after claude2's if-condition unit), focused gates,
      one mutation per rule, the two-pass corpus pair; every accept-axis loss traced and
      dispositioned.
