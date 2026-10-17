<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# If-condition substitution (claude2_chirho)

Lane assigned by gpt_chirho in #27139 and activated in #27154 (L.J. 2026-10-17: "push dev
forward", parity with GHC). Branch `if-condition-chirho` in claude2's worktree, based on main
`9986576b`. Source checkpoint first, then claude2's builder lease (-j2, nice/timeout, at most two
test threads, focused gates; no full driver --lib, no ghc_bulk).

## The defect

`infer_expr_chirho`'s `if` arm unified the condition's type with Bool and applied that unifier to
the environment, but left it out of the substitution it returned. A definition whose condition
was a parameter therefore generalised as if the parameter were unconstrained:
`gChirho v = if v then (1 :: Int) else 0; main = print (gChirho 'x')` was accepted, while GHC
9.14.1 rejects it (GHC-83865). The polymorphic-expected `if` path already composed its condition's
substitution. Lead from claude_chirho's baseline notes, confirmed in agy_chirho's audited report and
gpt_chirho's receipt (K01 wrong accept, K02 control rejected, on main's binary 4dceabb3).

## Checklist

- [x] Baseline, my own 36 probes in tmp-chirho/if-condition-1017-chirho/ on GHC 9.14.1 and main's
      binary 4dceabb3: 16 of 18 reject probes wrongly accepted (the literal-condition and
      signature controls rejected); 17 of 18 accept probes run as GHC (IA04 fails at run time on
      the known Ord dictionary defect, unrelated).
- [x] Brick 1, placement (pure move): both `if` arms move to `infer_chirho/conditionals_chirho.rs`;
      infer_chirho.rs shrinks by 74 lines.
- [x] Brick 2, the repair: the condition's unifier is composed into the returned substitution.
      Controls in typing_integration_chirho.rs (8 rejects, 7 accepts, all GHC-measured); workflow
      `language-features-chirho/conditionals-chirho.md`.
- [ ] Source checkpoint posted (hashes and files).
- [ ] Build both bricks; brick 1 must be behaviour-identical to main on every probe.
- [ ] Focused gates: the new controls, typing unit suite, named neighbours.
- [ ] Probe matrix on the repair: every reject probe rejected for GHC's reason, every accept
      probe unchanged.
- [ ] Corpus pair, two passes per axis: accept membership must not lose a file; reject gains
      named with GHC's reason; replay and execution unchanged.
- [ ] One mutation of the repair, caught by a named control.
- [ ] Evidence posted; SLOT-FREE.
