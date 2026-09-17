<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Reject-axis reason audit: boot-class diagnostic vs 26f77cad (2026-09-17)

Before: `/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-kind-signatures-chirho/test-data-chirho/kind-oracles-chirho/family-equations-chirho/closed-injectivity-chirho/corpus-chirho/final-chirho/fail-observations-chirho.jsonl`
After: `/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-kind-signatures-chirho/test-data-chirho/kind-oracles-chirho/ascriptions-chirho/imported-families-chirho/source-boot-chirho/declarations-chirho/classes-chirho/corpus-chirho/fail-observations-chirho.jsonl`

Method: per file, the candidate's diagnostic text (after-run for gains, before-run for losses, first three lines quoted below) compared with the committed GHC `.stderr` and the source; classification MATCHING / ADJACENT / WRONG / UNCERTAIN by rule and anchor, never by count or first error alone. After-run exit codes are recorded for the losses so 'now accepted' is read from the data.

## AssocTyDef01 (gain) — MATCHING
- Note: default for an undeclared associated type (`OtherType`, not `OtherTyp`): GHC-54721 'is not a (visible) associated type of class'; candidate 'default names no declared associated family' at the same line (column at `type`, GHC at the name). Second candidate error (distinct-variable-arguments) is consequential noise.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: default names no declared associated family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef01.hs:9:5 || |

## AssocTyDef02 (gain) — MATCHING
- Note: `type Typ [b] = Int`: GHC-41522 'Illegal argument [b] ... must all be distinct type variables'; candidate 'associated default requires distinct variable arguments matching its family', same position 7:5.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: associated default requires distinct variable arguments matching its family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef02.hs:7:5 || |

## AssocTyDef03 (gain) — MATCHING (position adjacent)
- Note: `data Typ a` with `type Typ a = Int`: GHC-52347 'Wrong category of family instance; declaration was for a data family' at the default (6:5); candidate 'associated data family cannot have a type default' anchored at the data declaration (5:5). Same rule, different anchor line.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: associated data family cannot have a type default || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef03.hs:5:5 || |

## AssocTyDef05 (gain) — ADJACENT
- Note: `type Typ = Maybe` (0 params vs 1): GHC-12985 'Number of parameters must match family declaration; expected 1'; candidate rejects through the same default-validity check but words it as 'distinct variable arguments matching its family'. Right rule family (default LHS validity), arity not named.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: associated default requires distinct variable arguments matching its family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef05.hs:6:5 || |

## AssocTyDef06 (gain) — ADJACENT
- Note: `type Typ a b = Int` (2 vs 1): GHC-12985 arity; candidate same 'distinct variable arguments matching its family' wording. As 05.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: associated default requires distinct variable arguments matching its family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef06.hs:6:5 || |

## AssocTyDef07 (gain) — MATCHING
- Note: `type Typ a = Int` with no family head: GHC-54721; candidate 'default names no declared associated family'. This is the S2 form from #23163.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: default names no declared associated family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef07.hs:5:5 || |

## AssocTyDef08 (gain) — MATCHING
- Note: identical program to 07 under another module name: GHC-54721; candidate as 07.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: default names no declared associated family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef08.hs:4:5 || |

## AssocTyDef09 (gain) — MATCHING
- Note: top-level `type family OtherTyp a` plus class default `type OtherType a = Int`: GHC-54721 (undeclared in the class); candidate 'default names no declared associated family'.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: default names no declared associated family || --> ghc-tests-chirho/typecheck-chirho/should_fail/AssocTyDef09.hs:8:5 || |

## MultiAssocDefaults (gain) — MATCHING
- Note: two defaults for `A`: GHC-59128 'More than one default declaration for A' anchored at the class (7:1); candidate 'multiple defaults for an associated family' anchored at the second default (8:3).
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0206]: multiple defaults for an associated family || --> ghc-tests-chirho/typecheck-chirho/should_fail/MultiAssocDefaults.hs:8:3 || |

## T7892 (gain) — MATCHING
- Note: `class C (f :: * -> *) where type F (f :: *) :: *`: GHC-83865 'Expected kind * -> *, but f has kind *' at 5:4; candidate 'kind mismatch in associated family parameter: expected * -> *, found *' at 5:4. Exact.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0300]: kind mismatch in associated family parameter: expected `* -> *`, found `*` || --> ghc-tests-chirho/typecheck-chirho/should_fail/T7892.hs:5:4 || |

## VisFlag3 (gain) — MATCHING (wording adjacent)
- Note: class binder `hk :: forall k. k -> k` vs associated binder `hk :: forall k -> k -> k`: GHC-83865 'Expecting one more argument to hk' at 6:3; candidate 'kind mismatch in associated family parameter: expected k -> k, found forall (_ :: k) -> ...' at 6:3. Same check (associated binder kind against class binder kind); GHC phrases the visibility-flag difference as arity.
- Candidate before: exit 0;
- Candidate after: exit 1; error[E0300]: kind mismatch in associated family parameter: expected `k100040 -> k100040`, found `forall (_ :: k100044) -> bound 0 -> bound 0` || --> ghc-tests-chirho/typecheck-chirho/should_fail/VisFlag3.hs:6:3 || |

## T12803 (loss) — PRIOR REASON WRONG
- Note: before: 'kind mismatch in kind annotation: expected Nat, found *' at 6:20 = the `:: *` in `type family F a :: *` read as Nat multiplication (the star regression gpt fixed). GHC-21572 rejects the INSTANCE `instance C p (F q) => C p [q]` (9:10). Now honestly accepted; the instance-validity rule with a family application in the context is unimplemented.
- Candidate before: exit 1; error[E0300]: kind mismatch in kind annotation: expected `GHC.TypeNats.Nat`, found `*` || --> ghc-tests-chirho/typecheck-chirho/should_fail/T12803.hs:6:20 || |
- Candidate after: exit 0; (no diagnostic)

## T16946 (loss) — PRIOR REASON WRONG
- Note: before: 'kind mismatch in type application: expected rigid k, found (TYPE k)' at 9:17 (the associated `type Id c :: c x x`); GHC-71451 'Cannot generalise type; skolem k would escape its scope' at boom's signature (11:9). Different rule and location; now honestly accepted; skolem-escape in kind generalisation unimplemented.
- Candidate before: exit 1; error[E0300]: kind mismatch in type application: expected `rigid k100038`, found `(GHC.Prim.TYPE k100050)` || --> ghc-tests-chirho/typecheck-chirho/should_fail/T16946.hs:9:17 || |
- Candidate after: exit 0; (no diagnostic)

## T24553 (loss) — PRIOR REASON WRONG
- Note: before: 'kind mismatch in function type result: expected Nat, found *' at 8:34 (star-as-Nat); GHC-83865 rejects the forall placement in `type Bar = Foo :: forall r. * -> TYPE r -> *` against `Foo :: * -> forall r. TYPE r -> *` (8:12). Now honestly accepted; kind-signature forall-position comparison unimplemented.
- Candidate before: exit 1; error[E0300]: kind mismatch in function type result: expected `GHC.TypeNats.Nat`, found `*` || --> ghc-tests-chirho/typecheck-chirho/should_fail/T24553.hs:8:34 || |
- Candidate after: exit 0; (no diagnostic)

## T5853 (loss) — PRIOR REASON WRONG
- Note: before: 'kind mismatch in kind annotation: expected Nat, found *' at 6:23 and 7:26 (star-as-Nat in `type family Elem f :: *` / `Subst f b :: *`); GHC-25897 'Could not deduce Subst fa2 (Elem fb) ~ fb' inside the RULES pragma (15:52). Now honestly accepted; RULES-pragma equality deduction unimplemented.
- Candidate before: exit 1; error[E0300]: kind mismatch in kind annotation: expected `GHC.TypeNats.Nat`, found `*` || --> ghc-tests-chirho/typecheck-chirho/should_fail/T5853.hs:6:23 || |
- Candidate after: exit 0; (no diagnostic)

## T7368a (loss) — PRIOR REASON WRONG
- Note: before: 'kind mismatch in type application: expected *, found k -> k' at 7:33, i.e. the signature `fun :: forall (f :: * -> *). f (Bad f) -> Bool`, which GHC accepts; GHC-18872 'Couldn't match kind * with * -> *' is at the PATTERN `Bad x` (8:6). Now honestly accepted; kind mismatch through constructor pattern matching unimplemented.
- Candidate before: exit 1; error[E0300]: kind mismatch in type application: expected `*`, found `k100041 -> k100043` || error[E0300]: kind mismatch in type application: expected `*`, found `k100041 -> k100045` || error[E0300]: kind mismatch in type application: expected `*`, found `((* rigid k100047) rigid k100047)`
- Candidate after: exit 0; (no diagnostic)

## T8883 (loss) — PRIOR REASON WRONG
- Note: before: 'kind mismatch in function type argument/result: expected Nat, found *' at 8:21 (star-as-Nat in `type family PF a :: * -> *`); GHC-80003 'Non type-variable argument in the constraint: Functor (PF t)' when checking fold's inferred type (21:1). Now honestly accepted; the inferred-constraint validity rule unimplemented.
- Candidate before: exit 1; error[E0300]: kind mismatch in function type argument: expected `GHC.TypeNats.Nat`, found `*` || --> ghc-tests-chirho/typecheck-chirho/should_fail/T8883.hs:8:21 || |
- Candidate after: exit 0; (no diagnostic)

## Summary

- MATCHING: 9 — AssocTyDef01 AssocTyDef02 AssocTyDef03 AssocTyDef07 AssocTyDef08 AssocTyDef09 MultiAssocDefaults T7892 VisFlag3
- ADJACENT: 2 — AssocTyDef05 AssocTyDef06
- PRIOR: 6 — T12803 T16946 T24553 T5853 T7368a T8883

Reading (separate from the classifications): the eleven gains are the associated-default and associated-binder validity rules landing with GHC's own reasons (nine matching, two adjacent on arity wording). All six losses were rejections for a WRONG reason — four the `:: *` read as Nat multiplication, two kind-mismatch reports at the wrong declaration — so their disappearance is the star and associated-kind repairs making the candidate honest; six real GHC rules remain unimplemented and are named per file above. No count should be banked from the losses' side: the reject axis lost six false rejections, not six capabilities.
