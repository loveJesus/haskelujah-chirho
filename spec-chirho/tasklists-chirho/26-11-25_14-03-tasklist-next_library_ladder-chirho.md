<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Tasklist: next pipeline, library ladder

2026-11-25, HASKELUJAH/claude_chirho. PROPOSED; the pen is gpt_chirho's. Design and reasons: `spec-chirho/pathway-chirho/26-11-25-plan-revision-chirho.md` (R1 to R6). Unit and gate rules: R5 there. Tick a box when its exit holds.

## Before work resumes (L.J.)

- [ ] Build turns for Haskelujah are available again, or small builds are exempt.
- [ ] The GitHub token is rotated; the remote address no longer carries a credential.
- [ ] R1 to R6 confirmed or changed.

## Setup

- [ ] 1. Close unit 764 as it stands; no new byte-span work. Join the unit branches into one branch `next-pipeline-chirho`.
- [ ] 2. Move the shared STG lowerer to its own crate, then `frontend_chirho/next_chirho` to `crates/haskelujah-next-chirho`. Exit: the existing next tests pass; the legacy driver is not in the new crate's dependency tree; warm test-build time recorded before and after; Clippy zero on the new crates.
- [ ] 3. Fix the quadratic constraint rescan (worklist of unsolved constraints; no charge for a zonk that changes nothing). Exit: a generated module of 2,000 bindings checks in time linear in its size.

## Rung 0: every module has a level

- [ ] 4. Embed the whole, unmodified `libraries/ghc-internal` and `libraries/base` of GHC 9.14.1 with licences and one generated hash list. Module list and default extensions from the two `.cabal` files for the target.
- [ ] 5. Scoreboard command: table (module, declaration, level, blocker) plus summary with blockers ranked by declarations held back. Blockers are ranked over declaration dependencies, not module imports (R7). Exit: `parsed` and `renamed` numbers for every module.
- [ ] 6. Preprocessor in Rust with the real GHC 9.14.1 profile: target `aarch64-apple-darwin` as a Rust table (`26-11-25-rung0-inputs-chirho.md` section 2). Exit: every `.hs` module preprocesses; the token stream equals `ghc -E` on every module; every macro the library uses equals the installed GHC's headers.
- [ ] 6b. `hsc2hs` typing mode in Rust (inputs section 3). Exit: all 32 `.hsc` modules reach `parsed`.
- [ ] 7. Work the top parse and rename blockers until rungs 1 and 2 are fully `renamed`. `MagicHash` and unboxed tuple syntax belong here.
- [ ] 8. `typed`: signatures; data, newtype, class and instance heads; synonyms; `hs-boot`; `TYPE rep` kinds; primitive types and primop types, with `GHC.Internal.Prim` and `GHC.Internal.PrimopWrappers` generated from `primops.txt.pp`.
- [ ] 9. Type comparison with `ghc --show-iface` for every `typed` export, one shared normalization.
- [ ] 10. Fail-closed rules 1 to 3 at check time, each with a test that fails without the rule. Until step 13b, rule 4 stays a check before the run. Soundness review by the other lane.
- [ ] **Gate rung 0.** Scoreboard for all modules; types equal GHC's; check bucket measured again; report with hours and tokens; merge to `main_chirho`.

## Rung 1: `Types`, `Classes`, `CString`, `Magic`, `Tuple` run from source

- [ ] 11. Core representations; strict unlifted bindings; unboxed values and tuples in the interpreter.
- [ ] 12. Bang patterns; stock deriving of `Eq` and `Ord`, standalone and clause forms.
- [ ] 13. Primop bodies by scoreboard rank.
- [ ] 13b. Rule 4 at run time (R7): a typed trap with its own linkage, and an Unsupported host outcome that comparison checks before output and exit status. Exit: the four controls in the inputs note, section 5 item 5, pass; budget exhaustion keeps its precedence.
- [ ] **Gate rung 1.** The five modules all `coded`; `Eq` and `Ord` programs run from source with the seven `Int` comparison bodies retired; primop batch equals GHC.

## Rung 2: the Haskell 2010 core runs from source (gate G1)

- [ ] 14. `Base`, `Maybe`, `List`, `Char`, `Show`, `Num`, `Real`, `Enum`, `Err` to `coded`, by scoreboard rank.
- [ ] 14b. The exception path (R7): built-in `Typeable` evidence and GHC's type-constructor and module bindings with GHC's fingerprints; `SomeException` with `ExceptionContext`; the `HasCallStack` backtrace; top-level `IORef` constants; `raise#`, `raiseIO#`, `catch#` and masking; `runMainIO`. Exit: `error`, `throwIO` and an uncaught exception give GHC's output.
- [ ] 15. `Bignum.*` with `BIGNUM_NATIVE`; retire the opaque `Integer`, the five `Num Int` bodies, the `Show` formatting bodies and, if unit 764 lands, `returnIO`, `bindIO` and `thenIO`.
- [ ] 16. Delete every hand manifest whose declarations the scoreboard now covers.
- [ ] **Gate G1.** Every exported declaration of rung 2's modules `coded`; 13 of 13 run programs complete with no unsupported stop and equal GHC; the interim list holds only the IO boundary.
