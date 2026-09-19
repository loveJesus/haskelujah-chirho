<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Instance declaration obligations (reject axis) — claude_chirho, 2026-09-18

Branch `instance-obligations-chirho` off main 6db522ad, worktree `haskelujah-workspaces-chirho/haskelujah-claude-chirho`. L.J.'s direct "continue development"; gpt holds row 484 (kinds, families, boot) and its files.

## Why (measured on the wrongly-accepted list of 2026-09-10, 546 files, by each file's GHC .stderr)
A set of declaration-time obligations is never checked: an instance is registered
(`process_instance_decl_chirho`) and nothing asks whether it may exist.

**The census, corrected 2026-09-19** (gpt_chirho #23944, claude2_chirho #23939, both owning their part).
Against that 546-file list, the strict phrase "In the instance declaration for" matched **55** files. The
85 first quoted here was a loose substring match on "instance declaration", which also caught 18
family-instance files belonging to row 484; it is withdrawn. The per-brick count inside the same 546 is
duplicates 5, superclass obligations 14, Paterson 7, built-in class instances 4, fundep conflicts 3 and
coverage 2. Sixty of the 546 files carry no `.stderr` on disk, so any stderr-based census of that
list runs over 486 files, not 546 (claude2_chirho's denominator caveat).

After this lane landed (row 486, main 146672cf) that list is **539**: the duplicate brick took its five
matching files plus tcfail118, and tcfail056 by an adjacent reason. Size any later brick against the
current artifact, never against these historical figures.

## Design (surfaced at brick 1)
- One child module `crates/haskelujah-typing-chirho/src/infer_chirho/instance_obligations_chirho.rs`; `process_instance_decl_chirho` records each LOCAL instance with its span; one check runs after every local and derived instance is registered.
- Completeness argument for "no instance exists": an imported module cannot mention a type declared here, so for a head whose outer constructor is LOCAL the only instances that can match are this module's, plus catch-all instances of the class — which we can rule out only for classes declared here or seeded from base. The check fires under exactly that guard, never on "no instance registered". **Bounded by gpt_chirho's SOURCE-cycle counterexample, confirmed under GHC 9.14.1**: a module reached through a `SOURCE` import cycle CAN declare an instance for a type declared here, so the guard also requires that no non-Prelude module is imported. That is a guard on a completeness argument, not on a verdict.
- STRICT entailment (`entails_strictly_chirho`): by a given with superclass closure, or by an instance whose sub-goals hold strictly; variables are never assumed satisfiable; depth exhaustion is "unproved" and reports nothing. The lenient `entails_chirho` is untouched.
- Exact-output tests per brick; both corpus axes two passes at landing; every reject gain checked against its `.stderr` reason.

## Census (claude2's independent count, 2026-09-18; 60 of the 546 files have no .stderr and drop out of any stderr census)
Per brick inside main's wrongly-accepted list: duplicates 5, superclass obligations 14, Paterson 7, built-in class instances 4, fundep conflicts 3 + coverage 2. The 18 family-instance files under "instance declaration" are row 484's.

## Bricks
- [x] 0. GHC's numeric hierarchy in the seed (found while reading tcfail036): `Num` had the Haskell 98 superclasses `Eq`/`Show`, so `Num a` entailed `Eq a`, every Num dictionary carried two phantom superclass slots, and `Integral` reached `==` only through that leak. Now `Num` has none, `Integral` has `Real` and `Enum`, `Real Word` is seeded. Three assertions that pinned the report's definition were corrected in the same change (typing seed test; core layout test; core transitive-superclass test, which now walks Integral => Real => Ord => Eq). Gate `tests/numeric_hierarchy_chirho.rs`: GHC's own outputs on the interpreter, plus `==`/`show` under only `Num a` rejected. No corpus file moves for this reason (measured: no wrongly-accepted file's GHC complaint is an Eq/Show derived from a numeric context).
- [x] 1. Duplicate instance declarations (GHC-59692): local-local by alpha-equal heads; against non-local instances only when names cannot be shadowed. All five corpus files rejected for GHC's reason: TcNullaryTCFail, tcfail023, tcfail035, tcfail036, tcfail073. Tests `tests/instance_obligations_chirho.rs` (6).
- [~] 2. Superclass obligations: WRITTEN AND MEASURED, NOT ENABLED — see the workflow doc. First diagnostic accept pass with it enabled: 872 of 938, ten valid files falsely rejected (LoopOfTheDay1/2/3, T10335 by mangled or unexpanded contexts; six by duplicate guards since added). **Its context prerequisite is met as of 8d86bc95**: declaration contexts are now lowered with the signature grammar in THIS lane, on main, so the mangled-context half of those ten is re-measurable without waiting on another branch. What the kit still needs from brick 8 is the typing side: `process_instance_decl_chirho` keeps only the first argument of each context constraint (`PredChirho::new_multi_chirho` already exists, so this is small) and `supers_chirho: Vec<String>` drops superclass arguments. Re-run the diagnostic accept pass before enabling anything.
- [ ] 3. Paterson conditions without UndecidableInstances (GHC-22979) — needs faithful contexts too; `paterson_smaller_chirho` is in the kit.
- [ ] 4. User instances of built-in classes (GHC-97044).
- [ ] 5. Functional-dependency conflicts and coverage (GHC-46208, GHC-21572) — heads and fundeps only.
- [ ] 6. Landing of bricks 0 and 1: driver `--lib`, integration suites, curated, both axes two passes, per-file reasons, artifacts, DB row (writer lease coordinated).

## Added 2026-09-18 after the context brick's diagnostic pass

- [ ] 7. **Instance members must belong to the class** (GHC-54721 "`op2` is not a (visible) method of
      class `Foo`"): tcfail077 is wrongly accepted; tcfail056 is rejected today only through its real
      duplicate instance, while GHC stops at this rule first. The other four GHC-54721 files are
      associated TYPES (AssocTyDef01/07/08/09) and belong to the associated-family lane.
- [ ] 8. **Typing consumers of a faithful context** (precondition of bricks 2 and 3): keep every
      argument of an instance-context constraint, skip `~`/`~~` and `?`-marked premises explicitly,
      stop dropping `QuantifiedChirho` silently, and store superclass predicates with their arguments
      (the kit has the last part). `PredChirho` already carries `extra_tys_chirho` and
      `new_multi_chirho`, so the instance half is small; `supers_chirho: Vec<String>` is the wider one
      because every seeded class in `class_chirho.rs` writes it.

### What brick 8 alone will and will not fix (measured 2026-09-19, main 16902b06, CLI 965f2d07)

Four probes against GHC 9.14.1, predictions written first, evidence in
`/private/tmp/haskelujah-context-consumers-chirho.vbaCzY/`:

| case | GHC | ours today |
|---|---|---|
| `instance Convert a String => Render [a]` (UndecidableInstances) | `1;2;` / `yes;no;` | rc=1, "missing STG binding `renderChirho`" |
| `class Convert a String => Pretty a` with a default method using the superclass | `<7>` | rc=1, "missing method ConvertChirho.convertChirho for Int_[Char]" |
| a two-argument given whose SECOND argument selects the instance | `3@z` | rc=0 and EMPTY output |
| a quantified given in an instance context | `built` | rc=0 and EMPTY output |

Two of the four exit 0 and print nothing: a silent wrong answer, which a `check`-only corpus gate cannot
see. Brick 8 is necessary for all four and sufficient for none, because `generate_instance_dicts_chirho`
(`core-chirho/src/dict_chirho/instance_chirho.rs:673`) skips every instance that has a context, except
the one list-head path in `conditional_chirho.rs`, and keys dictionaries by the RENDERED head type.

**Architecture decision to surface, not to take quietly (for L.J. and gpt_chirho):** the GHC translation
is dictionary ABSTRACTION — `instance C a => D [a]` becomes a dictionary FUNCTION
`$fD[] :: DictC a -> DictD [a]`, applied at the use site — whereas this pass builds constant dictionaries
keyed by a rendered type string. Repairing the four probes for the right reason means introducing
dictionary functions, which is a redesign of a pass whose files are already 72 KB (`mod.rs`) and 215 KB
(`rewrite_chirho.rs`). The corpus-visible work (bricks 2 and 3, 14 + 7 files on the reject axis) does not
need it; these four runtime programs do. Sequence and scope are L.J.'s call.

## Found on the way (not claimed by this lane)
- Runtime, main, mine to take next: inside an instance body, a use of the class's OWN method at another type is dispatched to the instance being defined. `instance Num V where V a + V b = V (a + b)` dies with "no matching alternative for tag 0"; the same body through helper functions, through a user class, or without the inner call runs. Same failure for `compare x y` inside `instance Ord a => Ord (Box a)`.
- Runtime, main: `bigger :: Real a => a -> a -> Bool; bigger x y = x > y` at Double runs an Int comparison primop (`GtIntChirho: expected Int#`): `>` under a `Real a` context is not projected from the Real dictionary.
- Native, main (gpt's backends): for valid dictionary-passing numeric programs the interpreter matches GHC while Cranelift prints `double 1.5` as 3, `half 5` as 0 and garbage for custom-Num literals, and LLVM lacks `fromIntegral#`/`recip#`.
- Parser, main (fixed on gpt's branch): instance and class contexts are token-scanned, see the workflow doc.

## Boundaries
- Kind-shaped instance errors and data-instance return kinds: gpt's kind lane. Abstract-class instances (GHC-51758): the boot-class lane.
- No check fires on an incomplete instance universe; no guard is widened to pass a file.
