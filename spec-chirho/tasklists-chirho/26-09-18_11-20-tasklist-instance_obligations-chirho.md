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

**Corrected 2026-09-19 evening** (gpt_chirho #23958, verified here by byte count before accepting; my
first reading was loose and claude2_chirho relayed it):

- c3 prints exactly one byte, `\n`, not zero. Removing its SIGNATURE context makes it print `3@z`
  correctly, and both its instances are ground, so its route is the signature-context evidence capture
  dropping the extra type arguments, not instance dictionaries.
- c4 never demands its witness (GHC prints `built` without forcing), so it is a laziness control only.
  The finite witness-demanding control is c4b: `FixChirho LeafChirho` fully forced, where GHC prints
  `F(L)` and we print `LeafChirho`, ignoring the user's own `Show (FixChirho f)` instance. A wrong
  answer, not silence.
- The claim "brick 8 is necessary for all four and sufficient for none" is WITHDRAWN. It was traced for
  c1 only, where `generate_instance_dicts_chirho`
  (`core-chirho/src/dict_chirho/instance_chirho.rs:673`) skips every instance carrying a context except
  the one list-head path in `conditional_chirho.rs`. Each route gets named when it is traced.

**Decision taken by gpt_chirho (#23958), under L.J.'s continue-development delegation:** proceed, in this
lane, with complete predicate evidence through typing -> driver -> Core first, then general instance
dictionary functions built on the Core lambdas and applications that already exist, in focused modules,
with a checkpoint before the change and no growth of `rewrite_chirho.rs`. The GHC translation
is dictionary ABSTRACTION — `instance C a => D [a]` becomes a dictionary FUNCTION
`$fD[] :: DictC a -> DictD [a]`, applied at the use site — whereas this pass builds constant dictionaries
keyed by a rendered type string. Repairing the four probes for the right reason means introducing
dictionary functions, which is a redesign of a pass whose files are already 72 KB (`mod.rs`) and 215 KB
(`rewrite_chirho.rs`). The corpus-visible work (bricks 2 and 3, 14 + 7 files on the reject axis) does not
need it; these four runtime programs do. Sequence and scope are L.J.'s call.

### The Show bypass, measured 2026-09-19 (main 16902b06, CLI 965f2d07)

A user-written `Show` instance is bypassed whenever the constructor has a FIELD. Measured, with GHC
9.14.1 beside each:

| source | GHC | ours |
|---|---|---|
| `data PChirho = PChirho`, `instance Show PChirho` | `pp` | `pp` (correct) |
| `data WChirho = WChirho Int`, `instance Show WChirho` | `W3` | `WChirho 3` |
| `newtype VChirho = VChirho Int`, same instance | `V3` | `3` |
| `data QChirho a = QChirho a`, `instance Show a => Show (QChirho a)` | `Q<3>` | `QChirho 3` |
| the same one-field shape with a USER class instead of Show | `W3` | `W3` (correct) |
| a nullary and a one-field type in ONE module, both with Show instances | `pp` / `W3` | run-time crash, "no matching alternative for tag 66" |

So it is not contexts (the contextless monomorphic case fails), not newtypes (`data` behaves the same),
and not the general dictionary machinery (a user class at the same shape is correct). claude2_chirho
withdrew "the context is the whole discriminator" on this evidence (#23968), and the 09-18 note that
`instance Num V` dies with "no matching alternative for tag 0" does not reproduce: both forms now exit 0
printing the wrong value, which is the same bypass rather than a self-reference failure.

### Traced, and repaired: the evidence join was positional

The user's instance body IS generated and its dictionary IS correct. The occurrence was rewritten before
any dictionary path ran, to the binding named by the WRONG evidence.

- The desugarer mints one occurrence id per free reference of a class method in DECLARATION order.
- The checker bumps its per-name occurrence counter in INFERENCE-VISIT order, and instance method bodies
  are inferred last (phase 3e). Its own doc comment claimed "source order"; it was visit order.
- `join_occurrence_evidence_chirho` matched the two sequences BY POSITION, guarded only by a count.

So with two `show` occurrences in one module, `main`'s took the instance body's evidence (`Int`, from the
inner `show n`) and the body took `main`'s. **One line of source movement flips the answer**: the same
program prints `W3` with `main` declared first and `WChirho 3` with the instance first. Every
discriminator proposed that day (context, arity, newtype, quantification) merely correlated with "this
method name occurs more than once in the module".

**The repair** is the identity principle one level down: join by SOURCE SPAN, not by position. The
desugarer now records a span for the occurrences it mints for opted-in methods (they had none: an early
return dropped it), the checker's `MethodOccurrenceRecordChirho` carries the span of the reference it
describes, and the driver joins on that. Measured after the repair, each against GHC 9.14.1: the
declaration-order pair both print `W3`; the newtype prints `V3`; the nullary-plus-one-field module prints
`pp`/`W3` where it used to CRASH ("no matching alternative for tag 66"); the `where`-bound inner
reference prints `W3` where it used to die ("tag 0"); two instances with two uses print `W3Z4`.
Tests: `tests/method_occurrence_evidence_chirho.rs`.

**The positional path is not gone, and here is exactly what holds it, measured three ways.**

1. Delete it outright: 19 driver tests red, all DESUGARED shapes (do-notation binds, qualified do,
   mdo/rec, deriving functor and via, MPTC, fundeps, mutual recursion, six native round trips). Traced
   cause: in such a program EVERY method-occurrence span is `None`, because the desugarer mints those
   occurrences at sites that never recorded one.
2. Restrict it to names whose occurrences ALL lack spans (no mixing): 2 native round trips still red, and
   precisely: `print (id (MixChirho 2 True))` compiles to a binary printing `MixChirho 2 1`. The derived
   `Show`'s rendering of a Bool FIELD is a generated occurrence under a name that also has
   span-identified ones. **Main's native path prints `MixChirho 2 True` correctly**, so that restriction
   CAUSED a regression rather than revealing one (claude2_chirho asked the question that settles this).
3. Refuse every occurrence the span join did not uniquely match (f97ea64c, after gpt_chirho read the
   committed file in #24075 and found the boundary open): the SAME native regression returns,
   `MixChirho 2 1`. Traced rather than argued this time. The deriving pass gives every reference it
   generates the same placeholder span, and the checker's records carry it too:
   `JOIN-OCC show id=12 span=Some(Span(SYNTHETIC, 0..0))` twice, `JOIN-REC show ord=0 key=Int` and
   `ord=1 key=Bool` on the same span. The rule read that as two references contesting one span.
4. The landed shape (73560e17): a DUMMY or synthetic span is not an identity on either side, so it
   neither indexes the span map nor contests anything; a record carrying one identifies nothing; and an
   occurrence carrying one is unidentifiable rather than refused. A GENUINE span claimed by more than one
   occurrence is still refused and never rescued by position. The span join marks each record it CONSUMES
   and each occurrence it IDENTIFIES, and the positional path matches only unidentified occurrences
   against unconsumed records, count-guarded on those two filtered lists, so one proof can never serve two
   references (gpt_chirho's boundaries, rooms #24056 and #24075).

Four controls in `tests_chirho/occurrence_join_chirho.rs` hold those boundaries directly, because no
source program reaches them: a consumed record is never reused; two occurrences sharing a REAL span yield
nothing; unspanned occurrences take their records in order; and generated references sharing the
placeholder are still served. The second is mutation-checked. The first two needed a REAL file id to mean
anything, since a synthetic one is now explicitly not an identity.

It goes away when every mint site carries its own occurrence PROVENANCE. A source span alone is not
enough: one do or deriving node can create several references sharing a span, so the identity has to be
shared with the node that created them, never re-enumerated downstream (gpt_chirho, #24056).

### Gates on the landed shape (2026-09-20, after the reboot destroyed every earlier receipt)

curated ok (486s, exit 0); driver library 1777 passed, 0 failed, 0 ignored, 0 filtered, the four join
controls included; driver integration 19 targets, 202 passed, 0 failed. The join also moved out of
`lib.rs` (6635 lines, against a 1500 limit) into its own 252-line module beside those tests.

**An unexplained red stays unexplained.** A curated run on 4f6e61d5 reported 57 failures; the same commit
had run green minutes earlier, the red run overlapped concurrent cargo work of mine in one target, and
the log that would name the files was destroyed with the rest of /private/tmp. Observed once,
unreproduced, cause UNESTABLISHED. Today's clean run is not an explanation of it. claude2_chirho proposed
reproducing it under deliberate contention and then withdrew the proposal, on the grounds that one clean
trial cannot refute an intermittent race and that manufacturing contention would corrupt someone else's
measurement; both are right.

**Still wrong after this repair**, and now isolated to contexts: `instance Show a => Show (QChirho a)`
prints `QChirho 3` where GHC prints `Q<3>`, and the quantified-constraint control prints `LeafChirho`
where GHC prints `F(L)`. That is the contextful-instance dictionary work, unchanged by the join.

## Diagnostic corpus pass on f97ea64c (frozen CLI b15539b5, one pass per axis, zero timeouts)

| axis | landed main | lane tip | gained | lost |
|---|---|---|---|---|
| should_compile | 885 | 885 | none | **none** |
| should_fail | 228 | 235 | SCLoop, T5684, T5684b, T5684c, T5684d, T5684e, T5684f | none |

The accept axis is identical to the landed artifact file for file, which is what this pass existed to
measure: the stricter obligation rule is a rejection rule, so a false positive would have shown there.

Every gain is the same shape, an instance whose own context cannot hold no longer satisfying its head,
and each was checked against its own committed GHC stderr:

- **SCLoop** (GHC-39999, "No instance for `SC ()' arising from a use of `op'", 22:7): ours identical at
  the same line. The file's own comment is "it's all too easy to succeed with a bogus recursive
  dictionary", and that is precisely how we were succeeding.
- **T5684, T5684b, T5684e, T5684f** (GHC-39999, "No instance for `A Bool'"): ours identical.
- **T5684c, T5684d**: GHC reports TWO errors, `B Char b0` at 12:12 and `A Bool` at 13:12. We report the
  second only; the first names a predicate with an unsolved variable, which this rule deliberately
  refuses to claim. A matching reason on a PARTIAL error set, not parity.

## Found on the way (not claimed by this lane)
- CLI, measured 2026-09-19 while building the multi-module control: `haskelujah check ./Main.hs` resolves sibling modules in the same directory, and `haskelujah run ./Main.hs` does NOT (E0102 "could not find module" for each import, with either a relative or an absolute path). The runtime control therefore has to go through the driver's multi-module entry point rather than the CLI, and the CLI gap is its own repair.
- Runtime, main, mine to take next: inside an instance body, a use of the class's OWN method at another type is dispatched to the instance being defined. `instance Num V where V a + V b = V (a + b)` dies with "no matching alternative for tag 0"; the same body through helper functions, through a user class, or without the inner call runs. Same failure for `compare x y` inside `instance Ord a => Ord (Box a)`.
- Runtime, main: `bigger :: Real a => a -> a -> Bool; bigger x y = x > y` at Double runs an Int comparison primop (`GtIntChirho: expected Int#`): `>` under a `Real a` context is not projected from the Real dictionary.
- Native, main (gpt's backends): for valid dictionary-passing numeric programs the interpreter matches GHC while Cranelift prints `double 1.5` as 3, `half 5` as 0 and garbage for custom-Num literals, and LLVM lacks `fromIntegral#`/`recip#`.
- Parser, main (fixed on gpt's branch): instance and class contexts are token-scanned, see the workflow doc.

## Boundaries
- Kind-shaped instance errors and data-instance return kinds: gpt's kind lane. Abstract-class instances (GHC-51758): the boot-class lane.
- No check fires on an incomplete instance universe; no guard is widened to pass a file.
