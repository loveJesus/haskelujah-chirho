<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Pathway to replacing GHC

2026-11-10. Written by HASKELUJAH/claude_chirho as lead, on L.J.'s instruction: "get us to replace ghc", "figure out the pathway", "do the deepest architecture right now". Status: direction given by L.J. on 2026-11-10 ("i hope this improves our system get it working"), taken as yes to decisions 1 and 2 unless he corrects it. The first slice starts on 2026-11-12 as a bounded test (tasklist `spec-chirho/tasklists-chirho/26-11-10_04-00-tasklist-next_pipeline_first_slice-chirho.md`).

Companion design: `26-11-10-faithful-pipeline-architecture-chirho.md` in this directory.

## 1. Where we are, measured

Private integration `5b76a1c6` (strict "dependable" profile), GHC 9.14.1 as oracle.

| Axis | Measure |
| --- | --- |
| typecheck/should_compile (938 files) | 710 accepted, 203 refused, 25 unclassified |
| typecheck/should_fail (767 files) | 476 refused, 7 unclassified, 284 accepted. These are directory labels; not every accepted file is a wrong accept (see below) |
| Vendored GHC test directories | layout, module, parser, rename, typecheck. typecheck/should_run has 161 Haskell files with no retained runtime measurement. No runtime corpus for codegen, numeric, lib, concurrency or FFI |
| Recent yield | units 732 to 737: 1 to 6 corpus files per unit, about two units a day with two agents |

What fails, from the failure-class tables of the unit 737 candidate `r5` (`tmp-chirho/pathway-chirho/failure-classes-chirho/` in the union worktree):

- Wrongly accepted should_fail files, by the error GHC expects: 51 are "no instance" (GHC-39999), 20 type mismatch (GHC-83865), 15 Coercible mismatch (GHC-18872), 10 rigid variable (GHC-25897). Class constraints and `coerce` are not really solved.
- 44 of the "wrongly accepted" files have no retained expected-error file. Whether each is a helper module is not established.
- Refused should_compile files: 64 type mismatch, 28 kind mismatch, 20 typed holes, 14 resource bound, 13 ambiguous type variable, 8 syntax.

The directory label is not the test's expectation. The per-test table built from GHC's `all.T` files (`tmp-chirho/pathway-chirho/measurement-repair-chirho/per-test-chirho.tsv`) shows:

- 22 cases in should_compile are expected to fail, and 37 cases in should_fail are expected to compile.
- 64 and 40 files have no named test of their own. The named-source denominators are 874 and 727, not 938 and 767.
- 75 and 49 cases carry compiler flags that the measurement omits.

Every number in the table above is therefore a directory-label profile, not agreement with GHC.

Re-scored against the per-test expectations (final unit 737 verdicts; tests with no local flags and no conditional setup; `rescore-summary-chirho.md` beside the per-test table):

| Expectation | Agreement |
| --- | --- |
| GHC expects the test to compile | 592 of 769 accepted (77.0%) |
| GHC expects the test to be rejected | 432 of 664 refused (65.1%) |

- The 23 and 7 resource-bound outcomes stay in the denominators and are never credited as refusals.
- Tests whose flags the measurement omits are kept apart: 38 of 76 accepted and 21 of 42 refused.
- This is a frontend `check` measure only; no program is run.

Code size (Rust lines, tests included): typing 135k, driver 107k, core 65k, parser 43k, naming 32k. Interpreter runtime plus native RTS 15k. LLVM, Cranelift and Wasm backends 17k together.

How the standard library exists today:

- `stdlib-chirho/` holds 15 Haskell files, 822 lines.
- About 480 library modules (base, transformers, mtl, lens, comonad and others) are interfaces written by hand in Rust (`haskelujah-naming-chirho/src/iface_chirho`).
- Prelude classes and instances are Core built by Rust code (`haskelujah-core-chirho/src/dict_chirho/prelude_chirho.rs`, 16,779 lines, plus 4,700 lines of submodules).
- An imported name with no known type is given `forall a. a` (`frontend_chirho/pipeline_chirho.rs`).
- Monad transformers, ST and Data.Map are shims; the July PRD audit recorded Template Haskell as recognizers, not splice evaluation.

## 2. Diagnosis

The corpus numbers move slowly for three structural reasons. Each one is a simulation of what GHC does, where replacing GHC needs the thing itself.

1. **Libraries are modelled, not compiled.** No real package is compiled from its source. Every new program needs more hand-written interface and more Rust-built Core. Compilers that kept their own base library (Hugs, JHC, UHC) never reached Hackage. The ones that did (GHCJS, Eta, Asterius) ran GHC's own base.
2. **Evidence is reconstructed after type checking.** Dictionaries are found by a 56k-line pass that re-derives them from names, spans and keys (`$own`, `$sigma:<scope>:<index>`, `$prim_Class_method_Type`). Unit 736 was six review rounds of wrong-dictionary and missing-dictionary cases from that join.
3. **Inference threads substitutions, with no implication constraints.** GHC generates constraints with levels and solves them afterwards. Ours unifies eagerly and patches each consequence: equality scopes, public projections, escape checks, and unit 737's result cell. That is why 284 should_fail files are accepted, and why GHC-valid programs depend on branch order (measured today: `PinnedResultChirho.hs`, GHC accepts both functions, `5b` rejects one, `r5` rejects the other).

Continuing unit by unit on this base reaches neither Hackage nor a sound checker.

## 3. Destination

"Replace GHC" means: unmodified Haskell packages build with Haskelujah and behave as they do under GHC 9.14.1. Six gates, each a number L.J. can watch.

| Gate | Exit criterion |
| --- | --- |
| G1 Honest core | The new pipeline compiles and runs Haskell 2010 programs with classes, instances and numeric tower taken from real library source. Zero shims in the new pipeline |
| G2 GHC type system | typecheck/should_compile ≥ 95% accepted, should_fail ≥ 95% refused with the same GHC error code, inferred top-level types agree with `-ddump-types` on ≥ 98% of accepted files |
| G3 Real base | GHC 9.14.1's own ghc-prim, ghc-bignum, ghc-internal and base compile from unmodified source (patch set empty) and base's own tests pass on the interpreter |
| G4 Boot packages | containers, bytestring, text, transformers, mtl, stm, array, deepseq, filepath, directory, time, process, unix, binary, parsec, exceptions and template-haskell build from their Hackage sources and pass their tests. Template Haskell runs splices in the interpreter |
| G5 Drop-in driver | `cabal build -w haskelujah` works for Simple packages. The 200 most depended-on Stackage packages build and pass tests. LLVM backend within 3x of `ghc -O1` on nofib |
| G6 Replace | ≥ 95% of a Stackage snapshot builds. Within 1.5x. GHCi, Wasm, profiling |

G2 and G3 run in parallel after G1. The library gates measure usefulness; the test suite keeps each feature exact.

## 4. The architecture, in five decisions

Details and build order are in the companion design.

- **A1. One faithful pipeline, grown beside the old one.** A new pipeline (`next`) shares the parser, renamer, STG and backends with today's (`legacy`). It replaces type checking, evidence and libraries. `legacy` is frozen for features and deleted when `next` passes every retained corpus.
- **A2. A constraint-based, elaborating type checker.** A new crate ports GHC's design: bidirectional generation, levels, implication constraints, an OutsideIn(X) solver. It emits Core with dictionaries already explicit, so the dictionary pass disappears. GHC's source at 9.14.1 is the specification, module by module.
- **A3. Libraries from source.** The standard library is GHC's own, compiled by us. Only GHC's wired-in names and the primop table are known to the compiler. No new interface, shim or recognizer is ever written by hand again.
- **A4. One narrow waist: Core plus primops.** The primop table is generated from GHC's `primops.txt.pp`. One Rust RTS implements it for the interpreter and for native code, on one heap layout.
- **A6. No C toolchain.** The compiler, the runtime and the standard library need no C compiler and no C library of ours.
  - Today the native path calls `clang` and `cc` (`haskelujah-cli-chirho/src/main.rs`) and preprocessing calls the system `cpp` (`source_chirho/cpp_chirho.rs`). The interpreter and Cranelift are already pure Rust.
  - `next` uses Cranelift or Wasm with a Rust link step, a preprocessor written in Rust, a Rust big-integer backend, and Rust versions of the 16 C helper files (3,389 lines) and 3 Cmm files in ghc-internal. LLVM stays an optional backend.
  - Limit: a Hackage package that ships its own C code needs a C compiler or a Rust substitute for that package.
- **A7. Better than GHC where behaviour is unchanged.** Diagnostics, memory safety, build speed (parallel and incremental), portability and a single runtime are ours to improve. An accepted program never behaves differently from GHC.
- **A5. GHC is the oracle at every phase.** Accept or reject with the error code, inferred types, and program output are compared against retained GHC receipts for the test suite, for each library module and for each package.

## 5. Tracks and first steps

Six tracks. The first steps are sized for the first week of agent time after L.J. confirms.

| Track | Scope | First step |
| --- | --- | --- |
| O Oracle | Corpora, receipts, scoreboard | Retain the GHC 9.14.1 source tree (libraries and full test suite). Scoreboard: corpus by stage. Failure-class tables for today's corpus |
| P Parser and renamer on real code | CPP, hs-boot, pragmas, every syntax form in the boot libraries; resolved unique names | Parse-rate and rename-rate per boot-library module with today's parser; fix by class |
| T Type checker | New crate, slices S0 to S9 of the design | S0 to S2: types, metas, levels, constraints, solver for equalities, dictionaries and implications; Haskell 2010 expressions, patterns, bindings, data, classes, instances; Core out |
| W Waist | Primop table, unlifted values, unified RTS | Generate the primop table; unboxed values and strict unlifted bindings in the interpreter |
| L Libraries | The ladder: ghc-prim, bignum, ghc-internal, base, boot packages | Compile the first real modules (`GHC.Types`, `GHC.Classes`, `GHC.CString`, `GHC.Base`) as far as T and W allow; every blocker becomes a failure class |
| D Driver | Interface files, package database, GHC-compatible command line, Cabal | Begins at G3 |

Unit 737 closes first. The corpus comparison of `r5` against `5b` is measured:

- should_compile 714 accepted (T2846, T18412, T26225b and T7196 gained);
- should_fail gains one refusal (tcfail002) and loses two (T14605, T17213).

Both lost refusals were accidental in `5b`. GHC rejects T14605 for a Coercible mismatch and T17213 for an inferred rank-2 type without RankNTypes; the legacy checker has neither check. `r5` lands once those two checks exist. It is the last feature unit in the legacy checker.

## 6. What stops, what continues, what is risked

- Stops: new features, shims, interfaces and recognizers in `legacy`. Per-unit mutation controls and guard layers. They return only at gate boundaries.
- Continues: `legacy` stays green as the regression net. Landing gate per unit: focused tests, corpus movers with zero regressions, secret scan.
- Risk, the checker takes longer than planned: slices are ordered by frequency in real library code, and each slice has its own corpus bucket, so slippage shows per slice, not at the end.
- Risk, two pipelines drift: they share parser, renamer and backends, and `next` never calls into legacy typing.
- Risk, credit burn: design is done once at frontier level (these two documents). Implementation and validation run on cheaper models and on gpt_chirho. The lead reviews at slice boundaries.
- Fallback if L.J. declines the pivot: keep the legacy checker and add the deferred owned equality for result holes and the class-driven units from the failure tables. Expect the current yield.

## 7. Decisions for L.J.

Answers (2026-11-10):

- Decisions 1 and 2: taken as yes on "get it working". The standard library is GHC's Haskell `base` source; GHC's source is the reference, and we improve on it where behaviour is unchanged.
- Decision 3: stay on 9.14.1. It is the newest stable release (9.14.2 is at release candidate 2 and 10.0.1 at alpha 1 on the official index). Move to 9.14.2 when it is final.
- Decision 4: no new lane now. claude2_chirho has tokens again on 2026-11-12 and takes the first slice.
- Decision 5: approved as a reference. Done on 2026-11-10: the source archive (32.4 MB) and the test-suite archive (8.2 MB) are hash-verified and unpacked under `tmp-chirho/pathway-chirho/references-chirho/ghc-9.14.1-chirho/` in the union worktree. The full test suite has 520 `all.T` files, 11,205 Haskell files and 2,457 expected-output files.

1. **The pivot (A1 to A4).** Freeze legacy features after unit 737 and build `next`. Recommended.
2. **Using GHC's source.** Port algorithms from GHC's compiler and compile GHC's libraries as our standard library. Both are BSD-3: allowed, with their notices kept in our tree and releases. Our own code stays MIT/Apache. Recommended.
3. **Version.** Keep GHC 9.14.1 as the single oracle and library set. Recommended, because every retained receipt is 9.14.1. The alternative is the GHC version of the current Stackage LTS, which builds more of today's Hackage.
4. **Fleet.** One more implementation lane for track W (the RTS and primops), for example reopening claude2_chirho. Recommended once S0 to S2 exist.
5. **Download.** Retain the GHC 9.14.1 source tarball on the encrypted volume as corpus and specification. Recommended.
   - The archive is `https://downloads.haskell.org/ghc/9.14.1/ghc-9.14.1-src.tar.xz`, 33,970,212 bytes.
   - No complete source tree for the four libraries is on disk today.

## 8. Product features beyond the compiler

L.J., 2026-11-10: "don't forget our other features planned such as an emacs like (but haskell not lisp) editor, mcp/rest to control it". They stay on the map and attach to the gates as follows.

| Feature | Today | Attaches to |
| --- | --- | --- |
| Editor in the Emacs tradition, extended in Haskell instead of Lisp | `haskelujah-editor-chirho` (759 lines), `haskelujah-gui-chirho`, CLI `edit` | The Rust core (buffers, windows, keymaps, commands, display) is independent of the compiler. Haskell as its extension language needs the interpreter to run real library code (G1) and to load and reload modules at run time, the same machinery as Template Haskell and the REPL (G4) |
| MCP and REST control of the editor and the compiler | `haskelujah-mcp-chirho` (212 lines), CLI `mcp` | A thin server over the editor's command table and the driver (check, build, run). Independent of the gates |
| Language server | `haskelujah-lsp-chirho` (439 lines), CLI `lsp` | Types and diagnostics from the new checker, from slice S1 on |
| REPL | CLI `repl` | The interpreter with run-time module loading (G4) |

These features are not specified in `spec-chirho` yet. Each needs its own short design before work starts.
