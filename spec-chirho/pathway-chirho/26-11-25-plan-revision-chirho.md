<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Plan revision, 2026-11-25

Written by HASKELUJAH/claude_chirho on L.J.'s instruction of 2026-11-25: "take a stab at our plan see if it should be improved", "we want to be a drop in replacement for GHC (and some additional tools so its more like bun)", "but first line is the GHC replacement ability".

Status: PROPOSED to L.J. and to gpt_chirho, who holds the implementation pen. It amends `26-11-10-ghc-replacement-pathway-chirho.md` (sections 3, 5, 6 and 8) and `26-11-10-faithful-pipeline-architecture-chirho.md` (sections 10 and 11). Everything else in those two documents stands. The ordered steps are in `spec-chirho/tasklists-chirho/26-11-25_14-03-tasklist-next_library_ladder-chirho.md`.

## 1. Where the work stands

Read on 2026-11-25 from progress rows 740 to 764, the worktree `haskelujah-next-pipeline-s0-chirho` at `a4494857`, the GHC 9.14.1 reference tree and gpt_chirho's terminal. I ran no compiler.

| Measure | Value |
| --- | --- |
| Units closed in the new pipeline | 740 to 763, by gpt_chirho, 2026-11-10 to 2026-11-15 |
| Size of the new pipeline, tests included | 25,747 lines of Rust: checker 8,737, HIR 5,160, driver part 11,850 |
| Fixed check bucket (353 GHC test files) | 65 accepted, 50 refused, 238 "not supported yet" (row 763) |
| Fixed run bucket (13 programs) | 0 pass. Small programs with `print` and `putStrLn` run; a general `main` does not yet |
| Library source in use | 68 hand-picked byte spans, 18,281 bytes. That is 2.3% of the 15 vendored files and about 0.4% of ghc-internal (estimate from line counts) |
| Time per unit | 21 to 90 minutes for units 746 to 761; 3 h 7 min for 762; 6 h 3 min for 763 |
| Last compile, test or GHC run | 2026-11-17. Unit 764 is open and waits for a 340-second machine-wide build turn |

Scale, as an estimate from GHC's own sizes (architecture section 14): real base and the boot packages need on the order of 150,000 lines of Rust beyond today's 26,000. No date follows from that. The rung 0 and rung 1 gates below will give a measured rate for the revised method.

## 2. What is slowing it

1. **The library is entered by hand, one declaration at a time.**
   - Each library module has a manifest written by hand: the byte spans to keep, a replacement import list, and Rust code for the bodies the pipeline cannot compile (`library_chirho/mod.rs`, `library_chirho/backend_chirho/`).
   - Units 750 to 763 brought in `Num`, `Integer`, `Eq` and `Ord` for `Integer`, `String`, `Int`, `(++)`, `Show` for `Int` and `Integer`, and `putStrLn`: twelve units of work for 68 spans.
   - The cause lies underneath. The new pipeline has no unboxed values, no `MagicHash` and no primitive operations. Architecture slice S1 named them ("W0"), and track W never started. `Types`, `Classes`, `CString`, `Magic`, `Base`, `Num`, `Show`, `Enum`, `Real` and `Err` all declare `MagicHash`, and `Classes` has 37 standalone `deriving` instances. No module at the bottom of the library can compile whole, so pieces are cut out by hand.
   - `Integer` is an opaque Rust value. GHC's is `data Integer = IS !Int# | IP !BigNat# | IN !BigNat#`, and library modules use those constructors.
2. **Every test run is a heavy build.** The driver part of the new pipeline lives inside the legacy driver crate (94,689 lines, linked with the legacy checker, Template Haskell, LLVM, Wasm and Cranelift). The machine allows one heavy build at a time across all projects, and Haskelujah's turn has not come since 2026-11-17.
3. **Each unit carries a landing ceremony.** A branch per unit, an acceptance file binding up to 21 artifacts, a mutation control, a hashed report, and a second agent's consult or review in most units. Pathway section 6 already placed these at gates. L.J.'s direction of 2026-11-22 ("be the judgment, not a program runner") came after gpt_chirho's last active day.

The checker work itself is sound and stays: class evidence, superclasses, default methods, signatures, defaulting, interfaces between modules, synonyms.

## 3. Changes

### R1. Load the library as whole modules; the compiler computes what is usable

- **Source.** The complete, unmodified `libraries/ghc-internal` and `libraries/base` trees of GHC 9.14.1 (about 127,000 lines) are embedded with their licences. The module list and default extensions come from the two `.cabal` files evaluated for the target, not from a directory listing.
- **Levels.** Each top-level declaration group of each module gets the highest level it reaches.

| Level | Meaning | Enough for |
| --- | --- | --- |
| parsed | the module preprocesses and parses | fixities, export list |
| renamed | every name in the declaration resolves | scope verdicts |
| typed | its signature, or its data, class or instance head, kind-checks | type-checking a program that uses it |
| coded | its body checks and lowers to Core | running a program that reaches it |

- A declaration that fails stays at its level with a named blocker, for example "unboxed tuple pattern" or "primop `indexCharOffAddr#` not implemented". A declaration that depends on it is held at the level that dependency allows. No byte span, import list or manifest is written by hand.
- **Signatures first.** Almost every exported library function has a written signature over ordinary types. `typed` therefore needs parsing, renaming, kind checking and the primitive type table, and nothing from the runtime. User programs can be type-checked against the whole Prelude before any library body compiles.
- **Scoreboard.** One command writes a table (module, declaration, level, blocker) and a summary: declarations per level, and blockers ranked by how many declarations they hold back, directly and through dependencies. The top blocker is the next unit of work.
- **The library is the main acceptance corpus.** GHC accepts every line of it. A type error reported inside a library module is our bug, and that class is worked before any other.
- **Types are checked against GHC.** The type of every exported name is compared with GHC 9.14.1's own interface (`ghc --show-iface`) after one shared normalization. A difference is a blocker class.
- **Fail closed for user programs.** Four rules keep a partial library from giving a wrong verdict.
  1. A use of a name below `typed` answers "not supported yet" and names the blocker.
  2. "Not in scope" is answered only against a module that reached `renamed`.
  3. "No instance" is answered only when instance heads are `typed` in every place GHC could find that instance: the class's module, the modules of the type constructors in the head, and every orphan-instance module in the import closure. Otherwise the answer is "not supported yet". Type family equations follow the same rule.
  4. A program runs only when every declaration reachable from `main` is `coded` or on the interim list. Otherwise the answer is "not supported yet" and names what is missing.
- **Interim list.** Where a body cannot compile yet, a Rust body may stand in under the declaration's original name and source signature. A type may stand in as abstract, the form an `hs-boot` file gives it. Each entry appears on the scoreboard with the rung that retires it. Today's entries: the `Integer` arithmetic, the `Show` formatting bodies, and the IO boundary at `stdout` and `hPutStr'`. The list grows only at the IO boundary and is empty at gate G3.
- **Retiring the manifests.** A hand manifest is deleted when the scoreboard shows its declarations at the same level without it and the existing tests pass. Those tests stay as the regression net.

### R2. Unboxed values and primitive operations now (W0)

- Lexer and parser: `MagicHash` names and literals; unboxed tuples in types, expressions and patterns.
- Types: `TYPE rep` kinds; the primitive types and the types of all 585 primops, generated from GHC's `primops.txt.pp`. This part serves `typed` and comes first.
- Core: every binder carries its representation; a binding of unlifted type is strict.
- Interpreter: an unboxed value is immediate, an unboxed tuple is a multiple return, `State#` has no width.
- Primop bodies are written on demand, in the order the scoreboard ranks its "primop not implemented" blockers. They are compared with GHC's results on sampled inputs in one batch at the gate.

### R3. `Integer` from source

`GHC.Internal.Bignum.*` compiles from source with GHC's own pure-Haskell backend (`Backend.Native`, selected by `BIGNUM_NATIVE`). It needs only byte-array and word primops. If big-number speed matters later, Rust functions go in through GHC's own extension point (`Backend.FFI`, the `ghc_bignat_*` functions). The opaque Rust `Integer` is an interim entry and retires at rung 2.

### R4. The new pipeline in its own crates

- The driver part (`frontend_chirho/next_chirho`) moves to `crates/haskelujah-next-chirho`. It uses the legacy driver in two places only, both the shared STG lowerer (`stg_lower_chirho`), which moves to its own crate first. The legacy checker remains a dependency only as Core's type representation.
- The command line calls the new crate directly.
- Effects: a test run no longer rebuilds and links the legacy driver and three backends; "the new pipeline never calls the legacy checker" becomes a fact Cargo enforces; Clippy runs on the new crates alone at zero warnings, with no accounting for the 279 older warnings.
- Record the warm test-build time before and after. Under a minute, it is not a heavy build.

### R5. One branch, a light unit, a real gate

| | Unit (several a day) | Gate (each rung or slice) |
| --- | --- | --- |
| Check | tests of the new crates; the scoreboard difference shows no declaration dropped a level | full scoreboard; check and run buckets against GHC; legacy regression suite once; Clippy zero on the new crates |
| Review | none, except for a new solver rule or a change to the four fail-closed rules | the other lane reviews soundness |
| Record | one progress row in plain words, with the counts | a plain-words report to L.J. and AICEO, with the hours and tokens spent since the last gate |
| Git | commit to the one branch `next-pipeline-chirho`; secret scan of the outgoing range; push | merge into `main_chirho`; the default pipeline stays legacy |

Dropped from units: a branch per unit, acceptance files, mutation controls, hashed reports, per-unit consults.

### R6. Build order

- Slice S1 also takes bang patterns, stock deriving of `Eq` and `Ord`, `hs-boot` imports (19 modules have them; `Base` has three) and a preprocessor in Rust with the real GHC 9.14.1 profile. The quadratic constraint rescan, open since unit 744, is fixed before any module the size of `Base` is checked.
- Slices S3 to S7 become a menu. The scoreboard's top blocker picks the next item, counted across the library and the test buckets together.

| Rung | Modules under `GHC.Internal` | Gate |
| --- | --- | --- |
| 0 | all of ghc-internal and base | every module has a level; the type of every `typed` export equals GHC's; the check bucket is measured again |
| 1 | `Types`, `Classes`, `CString`, `Magic`, `Tuple` | all `coded`; `Eq` and `Ord` run from source |
| 2 | `Base`, `Maybe`, `List`, `Char`, `Show`, `Num`, `Bignum.*`, `Real`, `Enum`, `Err` | all `coded`; the 13 run programs pass; the interim list holds only the IO boundary. This is pathway gate G1 |
| 3 on | the ladder of architecture section 10 | as written there |

## 4. The product

**First line: a drop-in replacement for GHC 9.14.1.** "Drop-in" means these five things.

1. The one binary answers to the names `ghc`, `ghci`, `runghc`, `runhaskell`, `ghc-pkg` and `hsc2hs`, with GHC's flags. A flag that changes meaning and is not implemented is an error, never ignored in silence.
2. It reports itself as GHC 9.14.1 (`--numeric-version`, `--info`, `--supported-languages`, `__GLASGOW_HASKELL__`, the base version), so Cabal conditions and CPP select the same code as under GHC.
3. Same verdicts with GHC's error codes; same program output and exit code.
4. `cabal build -w haskelujah` and Stack work unchanged.
5. Not promised: reading GHC's compiled files (`.hi`, `.o`, `.a`), since we build every package from source; the GHC API and compiler plugins; the text of GHC's internal dumps. A package with C sources, `hsc2hs` or `capi` needs a C compiler on the machine, as it does with GHC.

Gates G1 to G6 of the pathway measure this line and are unchanged.

**Second line: one binary for the whole workflow, as Bun is for JavaScript.** It replaces `ghc`, `ghci`, `runghc`, `ghc-pkg`, the common path of `cabal` and the toolchain installer.

| Command | What the user gets | Rides on | Starts at |
| --- | --- | --- | --- |
| `run` | runs a file or script at once, dependencies declared in the file, nothing to install | interpreter, embedded library | G1 for the subset, G3 in full |
| `repl` | GHCi's commands | loading modules at run time | G4 |
| `build`, `add`, `install` | reads `.cabal` and `cabal.project` itself; solver; verified downloads; one shared cache | driver track | G5 |
| `test`, `bench` | the package's own suites; watch mode | `build` | G5 |
| `fmt`, `lint`, `doc` | formatter, linter, Haddock-compatible documentation | parser, renamer, types | after G5; each needs a short design |
| `lsp`, `mcp`, REST | editors and agents ask for checks, types, diagnostics and runs | checker | LSP at rung 0, when whole-library types exist; MCP and REST independent |
| `edit` | the Emacs-tradition editor extended in Haskell | interpreter, module reload | Rust core independent; Haskell extensions at G4 |
| targets | Wasm, cross-compilation, one static executable | backends | G5 to G6 |

- No toolchain manager is needed: the binary carries compiler, runtime and library source, and needs no C toolchain.
- Nothing on the second line takes agent time from the first before G3, except `run` and the scoreboard, which the first line needs anyway.

## 5. Decisions for L.J.

1. **Build turn.** Nothing has compiled since 2026-11-17. Haskelujah needs its build turns back from AICEO. After R4 the new crates' builds should be small enough to need no turn; the measurement in R4 will show it.
2. **The method.** R1 to R6 change how gpt_chirho works. They are sent to gpt_chirho as a proposal; L.J.'s word confirms or stops them.
3. **GitHub token.** Row 764 records that the token in the Git remote address was printed in a tool output on 2026-11-17 and that rotation is required. The canonical checkout's remote address still embeds a credential. After rotation, the keychain helper can hold it so the address carries none.
4. **Budget.** Still unset. Proposed: each gate report carries hours and tokens, and the cap is set after the rung 0 report gives a real figure.
5. **Model.** This revision settles the design through rung 2. Frontier-level design is next worth its cost for the runtime (heap, exceptions, threads) and for the solver slices with local equalities (GADTs, type families).

## 6. Considered and not adopted

- **Testing the runtime on programs exported from a specially built GHC.** It would let the runtime advance without waiting for the checker. It needs a custom GHC build and an importer. Reconsider if the runtime becomes the long pole after rung 2.
- **Module-level credit only.** Architecture section 10 counted a module only when all of it compiled. That gives no partial credit, which is what pushed the work toward hand-cut slices.
- **A Rust `Integer` type.** Library modules use GHC's constructors.
