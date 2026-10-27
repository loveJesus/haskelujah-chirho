<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# A qualified reference names an import, never a local binder Chirho

Row 598 (gpt_chirho #29622; checkpoint #29708; ruling #29717). `L.reverse`, `M.filter` and
`P.length` must stay distinct from same-named unqualified local binders, and a qualifier no
import binds must be rejected as GHC 9.14.1 rejects it (GHC-76037).

## Trace, by layer (#29708)

- **Parser:** correct. `L.reverse` arrives with qualifier `L` and text `reverse`.
- **Resolver:** row 536's check read written qualified references only for qualifiers closed
  by explicit lists. Nothing checked that a qualifier names an import. So `X.reverse`,
  `Data.List.reverse` with no import, `Data.List.reverse` under `import Data.List as L`,
  `X.+`, `X.Just` and `Prelude.print` under `import qualified Prelude as P` were all accepted
  and ran on every engine. Row 532's Q01/Q02 and row 536's I6 are this class.
- **Checker:** `lookup_value_scheme_with_qualified_suffix_fallback_chirho` fell back to the
  bare name through every scope. A `let nonexistentChirho` answered for `L.nonexistentChirho`,
  and a local `foo` typed `Main.foo`.
- **Desugar:** the variable arm used the full spelling only when it was bound in scope, then the
  bare text through the scope chain. The infix and section paths used the bare text alone. So
  a `let`, `where`, lambda, `case` or top-level binder captured the qualified reference.
- **Typing seed, its own defect (R2):** driver Phase 3.9 types a qualified builtin alias as the
  placeholder `forall a. a` when no `Module.x` key exists. `print (L.reverse xs)` then defaults
  Show to Int: natives fail "no native Show instance for a boxed value", and beside a local
  binder every engine fails "no matching alternative for tag 47". With `==` instead of print it
  is exact everywhere.

## Ruling and fix (#29717: prototype A as scoped)

- **Resolver:** a written qualified variable, operator, constructor or record construction whose
  qualifier is neither an import's effective qualifier nor the module's own name is E0100 not
  in scope, with the note "no module named `X` is imported". The walk now runs for every module.
- **Checker:** the bare fallback for a qualified name reads only the outermost scope (the module
  level and the builtins), never a local binder (`lookup_outermost_chirho`).
- **Desugar:** `resolve_written_reference_chirho` serves the variable arm, the infix fallback and
  `build_section_body_chirho`. A qualified reference not bound under its full spelling takes the
  module's own top-level binding when the qualifier is the module; otherwise it is a name no
  scope binds, by its bare text.
- **The own top-level namesake.** The checkpoint held it to today's resolution "until row 597
  lands", because two native `reverse` bindings crashed LLVM (exit 139) and Cranelift refused
  the second declaration. Row 597 landed (02f6c268) with `$prelude_` spellings. On that main the
  unheld route is exact on all three engines, so the hold was dropped.
- **Accepted (#29717):** a sibling user module's alias beside a local namesake moves natively
  from the local's wrong answer to "missing native binding `twice`". Base gives that same failure
  for every sibling-module reference in a single-file native compile, plain `import Lib`
  included.

## Checklist

- [x] Trace and checkpoint (#29708); ruling (#29717).
- [x] Prototype measured on 36431dbe, then 71b0dbf9 and 02f6c268. Cost: `check` on a 149-line
  qualified module took 1.69-1.74 s on base and 1.71-1.73 s on the candidate. (A 9,005-line probe
  overran the 2-5 minute bar and was stopped, #29702.)
- [x] Source: `desugar_chirho/qualified_references_chirho.rs`; call sites in `desugar_chirho.rs`;
  typing `env_chirho.rs` and one fallback in `infer_chirho.rs`; naming `value_scope_chirho.rs`.
- [x] Controls, embedded byte for byte from the GHC 9.14.1-measured files (12/12 verified):
  - `typing_integration_chirho/qualified_references_chirho.rs`:
    - three mover programs on the interpreter, LLVM and Cranelift: every local binder form;
      the module's own name and the implicit Prelude's; a headerless module beside an
      import's namesake;
    - a hold program: `C.toUpper`, `L.sort` under a signature, `L.reverse` compared, and a
      seeded Data.Map alias beside a local;
    - seven unimported-qualifier rejections, and the checker's `L.nonexistentChirho` beside a
      `let`.
  - Row 532's Q01/Q02 are asserted, no longer recorded only (`import_scope_chirho.rs`).
  - Resolver units: `every_written_qualifier_names_an_import_or_this_module_chirho` and
    `an_import_binds_its_qualifier_in_every_form_chirho`.
- [x] Gate r1 GREEN on cbf85ec9 (main 4c353787) in 352 s, over the bar because the frontend
  package builds took about 160 s; they reach the resolver check on real packages:
  - build with 0 warnings; naming 153, core 190, typing 364;
  - 13 driver suites 335, and the driver library's frontend, qualified, import and scope tests 279;
  - Clippy: none of 650 warnings on a changed line; rustfmt, diff-check and gitleaks clean.
- [x] Transition table, exact base 4c353787 against the candidate, 46 programs on GHC 9.14.1, the
  interpreter, LLVM and Cranelift (marker-checked):
  - 61 GAIN cells, 0 losses, no signal or timeout;
  - 2 CHANGED cells: the sibling-module alias, as ruled (#29717);
  - holds: 23 exact, 3 rejected, 45 failing (R2's placeholder cells), 9 accepted (the
    pattern, type and `Main.reverse` boundaries) and 1 wrong (the local Enum residual).
- [x] GHC typecheck corpus, the files with a qualified-looking token (408 should_compile and 285
  should_fail; no other file can move, because `check` never reaches the desugarer and both
  checks act only on qualified names): 0 moved. should_compile is 370 accepted and 38 rejected on
  both sides; should_fail is 214 and 71.
- [x] Reach mutations, each killed by a named assertion and restored byte-identical:
  - the desugar route on the scope chain fails both mover controls;
  - the resolver check off fails its unit test and both driver rejections (U01, Q01);
  - the checker reading every scope fails the `L.nonexistentChirho` control.
- [x] Replayed over row 605 (cbcf7372, no shared path): build with 0 warnings; naming 153,
  core 191; typing integration and dictionary evidence 171.

## Residuals

- **R2:** qualified aliases of builtin modules keep the placeholder scheme. Giving Prelude,
  Data.List, Data.Char and Data.Maybe aliases the bare builtin scheme made 20 more cells
  exact, but turned two loud failures silent:
  - `C.toUpper` beside `where toUpper = succ`: the interpreter prints `("AB",100)`. That is
    the local-generalization Enum residual, which prints 100 with no qualifier at all;
  - a module's own top-level `reverse` beside `L.reverse` with `print`.
  It is its own row, after row 597 (landed) and that residual.
- **Boundaries:**
  - unimported qualifiers in patterns (`X.Just y ->`) and types (`1 :: X.Int`) still check;
  - `Main.reverse` naming an import still checks;
  - Data.Map has no native primitives (`mapFilter#`), and `Data.Map.filter` has no key;
  - sibling modules are not linked in a single-file native compile.
