<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native symbol identity Chirho

Row 628 (gpt_chirho #30102; split #30109; brick 1 approved with corrections #30126). This row owns
layers 4 and 5 of the native namesake collision: backend symbols, and backend builtin dispatch.
Row 629 owns layers 1 to 3 (interfaces, desugaring and merge identity).

## Trace (#30107)

- **Cranelift** names each top-level function by its raw binder name, so namesakes share a
  FuncId and fail with "Duplicate definition". The CLI then falls back to LLVM.
- **LLVM** names each top-level function `haskelujah_<mangled name>`, and its calls name the
  same spelling, so namesakes collapse into one symbol. Today's mangler is not injective even
  across distinct names (`f'`/`f_prime`, `a.b`/`a_b`).
- **Builtin dispatch reads spellings.** Cranelift matches any id named `print`, `show`,
  `return`... LLVM's print fallback calls `@haskelujah_show` by spelling, and a user's
  `undefined` meets the builtin runtime function's symbol.
- **Measured (base 151a7af7):**
  - n2, n4 and n8 are silently wrong, with Cranelift falling back; n3 is exact.
  - n1, n5 and n6 are wrong; they are row 629's inputs.
  - b2 (an own `show`) prints `7` on Cranelift; b7 (an own `undefined`) fails on LLVM.

## Design (#30125, corrected by #30126)

- **One symbol table.** A deterministic table maps each top-level binding's CoreId to
  `haskelujah_<mangled>.<CoreId>`, injective and disjoint from every name a backend writes. Both
  backends use it for every definition and reference.
- **The entry is explicit.** It keeps `haskelujah_main`, and the driver records it: each module
  offers its own `main`, and the program's root module decides.
- **Typed provenance on the CoreModule:**
  - the entry;
  - compiler bindings (the dictionary pass's generated bindings);
  - builtin references (free ids whose names come from builtin interfaces or the builtin
    registry, and from no source module).
  Backends consult provenance, never a spelling or an unowned id. A free id that is neither
  linked nor a builtin reference fails closed.

## Checklist

- [x] Trace (#30107), brick 1 (#30125), and the corrections (#30126).
- [x] Core: the provenance and symbol-table types, with the CoreModule field on 103 literals.
- [x] Producers:
  - the dictionary pass records its compiler bindings;
  - the driver records builtin references and the entry;
  - the merge offsets, joins and remaps provenance;
  - native IO preparation records its wrapper and its bindings.
- [x] LLVM and Cranelift consume the table and the provenance.
- [x] Controls (`tests/native_symbol_identity_chirho.rs`, `tests/native_identity_chirho.rs`):
  - n2, n4 and n8 exact on LLVM and true Cranelift, with n3 holding;
  - entry controls E1-E3, where a dependency defines or exports `main`;
  - b2, b7, a libc namesake (L5: `strcmp` beside string comparison), and the mangler
    collisions M1 and M2;
  - the user print/show/getLine/compare family (B1, B2, B4, U1), plus B8;
  - table determinism and injectivity, and provenance offset, join and remap.
- [x] Gate on current main, then post the tip; report n1, n5, n6 and x1, x2, x4 as row 629
  inputs. Gate commit `eeb0d156` replayed patch-identically as `358e2d68` over row 626.
- [x] Land on main and close progress row 628: landed as `c5fe42c5`; independent replay checks
  passed core 197/197 and native symbol identity 12/12.

## Measured (main d99f8d2d against the tip; both CLIs built in one tree; GHC 9.14.1)

- **Movers to exact:**
  - n2, n4 and n8 on both backends. Main was wrong, and Cranelift fell back to LLVM.
  - E1-E3 on both. Main's LLVM printed nothing, and its Cranelift fell back.
  - b2 on Cranelift (`7` became `<7>`), and b7 on LLVM (`Prelude.undefined` became `43`).
  - L5 on Cranelift: main refused `compareStr#/2`, because the program's own `strcmp` took the
    libc import.
  - M1 and M2 on LLVM: main printed `(2,3)` and a heap word.
- **Fails loud instead of silent:** K4, where B defines its own `comparing` and A uses Data.Ord's.
  Main bound A's builtin reference to B's `comparing` and exited 1 with no message. The tip fails
  named, `missing native binding comparing`, as K1 does without B.
- **Holds:** n3, b1, b3-b6, b8, u1, l1-l4, l6, x3, K1-K3.
- **Unchanged and wrong, row 629's inputs:**
  - n1 (`8`, GHC `700`), n5 and n6;
  - x1, x2 and x4: a dependency's own show, print and compare imported into Main.
- **b7 on the interpreter** still fails. That is not this row's engine.
- **Sweep:** 276 programs (row 620's fixtures and the contract battery) on the interpreter, LLVM
  and real Cranelift. All 828 cells are identical: rc, stdout and stderr, paths and heap words
  masked.
- **One regression, found and fixed before the gate:** my first draft also guarded LLVM's
  IO-program check with provenance, and b8 printed a stray `0`. That check decides whether `main`'s
  value is printed, not a symbol, so it is unchanged.
- **Changed-line Clippy is 0.** Three pre-existing nested `if`s that came to hold an edited line are
  now let-chains. The 37-argument `define_function_body_chirho` signature is byte-identical to
  main.
