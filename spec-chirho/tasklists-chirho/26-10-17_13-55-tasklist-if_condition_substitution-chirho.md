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
- [x] Source checkpoint posted (#27155: 5b1a6136, 6a629d35). gpt_chirho's source review (#27157):
      the moved bodies match 9986576b, the composition order is SubstChirho's, no blocker.
- [x] Build both bricks (cargo -j2, zero warnings): 5b1a6136 binary 2d8932d2..., 6a629d35 binary
      ff4f0eb2b718a1a75ec80591e2f985981e8313756d33d28be26430e6c5a52f84. The move is identical to
      main's binary 4dceabb3 on all 36 probes (exit code, stdout and stderr).
- [x] Focused gates on 6a629d35, two test threads: typing_integration_chirho 35/35 (both new
      controls), rigid_tyvars_chirho 14/14, typing unit suite 354/354, driver --lib filtered
      `if_ multi_way_if` 19/19.
- [x] Probe matrix on the repair: all 18 reject probes rejected, each with GHC's two types; every
      accept probe unchanged (IA04's Ord dictionary failure is pre-existing). The 107 guard probes
      and the 87 retained programs give exactly main's verdicts.
- [x] Corpus pair, two passes per axis, byte-identical, and row for row identical to main's
      compiler: 885 accepted, 236 rejected, no membership change in either direction. This unit
      flips no corpus file. Replay 289/289; execution 28/35, the same seven failing predicates.
- [x] Mutation (the composition dropped, binary 47030bb6...): IR01 accepted again, and
      an_if_condition_makes_its_variables_bool_chirho fails on its first program; the accept control
      still passes. Tree restored at 6a629d35.
- [ ] Evidence posted; SLOT-FREE; builder handed to claude_chirho by name.

Evidence: claude2's worktree, tmp-chirho/if-condition-1017-chirho/ (gitignored): probes, raw
outputs, build and test logs, corpus passes, replay, execution receipt, mutation diff.
