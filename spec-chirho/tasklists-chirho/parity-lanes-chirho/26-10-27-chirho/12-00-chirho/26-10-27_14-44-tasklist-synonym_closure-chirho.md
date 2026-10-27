<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Synonym closure Chirho

Row 656 (reserved in #30423, released in #30438, design C approved in #30444) lets a re-exported
type alias expand through a parent alias that its user imports without listing it. GHC 9.14.1
accepts that Parsec-style chain; every Cabal build refused it.

## Bounds (#30444)

- Align the frontend's merged imported-synonym table with the canonical spelling. For a
  preferred-qualified name N with a unique exporter E, clone `E.N` under the bare key N, only when
  no bare key N exists and `E.N` is already in the filtered, merged input.
- Never overwrite a bare key, never take a synonym from a module the source does not import, and
  never choose among several exporters.
- This is an inference expansion alias, not a naming grant. Naming, the import filters, exporter
  construction, AST rewriting and inference normalization stay unchanged.
- Controls go in a fifth submodule of `tests/native_symbol_identity_chirho`, named for the synonym
  closure, with the placement rationale in the module doc.
- Proof:
  - the chain is exact on the interpreter, LLVM and true Cranelift, with no fallback;
  - both GHC-76037 scope negatives are refused on all three;
  - the only-forwarder program is exact, and the two existing hidden-parent alias tests stay green;
  - unit pins: a unique exporter adds the bare key, several exporters add none, a missing
    qualified source adds none, and an existing bare key is never overwritten;
  - removing only the alignment fails the chain with the measured type mismatch while every scope
    negative holds.

## Checklist

- [x] Trace checkpoint (#30442):
  - The exporters seed the parent at every step.
  - Main's filter keeps `CompatTop.*` and `PrimAlias.*` and drops `CompatPrim.*`.
  - The import rewrite and inference both spell `ParsecAlias` bare, because its one exporter among
    Main's imports is PrimAlias, but Main had no bare key for it.
  - Prototype A (filter closure) and prototype B (keep the rewrite qualified) each still fail;
    prototype C (the alignment) is exact on both backends.
- [x] `align_preferred_synonym_keys_chirho` in `lib.rs`, called once after the frontend's import
  loop.
- [x] Four unit pins in `src/synonym_keys_tests_chirho.rs`, each asserting its preferred-exporter
  precondition so none can pass vacuously.
- [x] `tests/native_symbol_identity_chirho/synonym_closure_chirho.rs`: four programs generated from
  the GHC-measured probe projects, byte-identical after rustfmt (18/18 literals). The chain and the
  only-forwarder import are exact on all three engines; the hidden parent and the unimported
  `Secret` are refused on all three. The target's root doc and the module doc record why they live
  there.
- [x] Mutant: removing only the alignment call compiled. The chain control failed at LLVM with
  `type mismatch: expected ParsecAliasCompatMiniChirho, found (ParsecTAliasCompatMiniChirho
  [tokChirho])`. The interpreter stayed exact first, since its synonyms are unfiltered (row 647),
  and the panic stopped the run before Cranelift. Both scope negatives and the only-forwarder
  program held. The file was restored from its saved bytes, hash-identical.
- [x] Scratch measurement, not adopted: with the alignment in place, the interpreter path under the
  shared synonym filter passed all 29 callers, including both alias chains that failed under the
  filter in row 647's trace. Restored from saved bytes, hash-identical.
- [x] Docs: the type-scope workflow records the rule; row 647's value-scope paragraph and seam
  comment no longer name the filter as the cause; the symbol-identity doc notes the placement.
- [x] Gate on 7dd077d9 (14:47 to 14:58):
  - the 29 callers, install 8/8 and the four unit pins;
  - native_symbol_identity 49/49: rows 628, 629, 621 and 642, plus row 656's four programs;
  - typing_integration 81 (2 ignored);
  - the driver library's 350 project, re-export, Hackage and multi-module tests, the same 350
    names row 647 passed, in 420 s, over the 2-to-5-minute bar;
  - a warning-free workspace check, changed-line Clippy 0 hits (the workspace's 631 messages and
    348 unique diagnostics equal row 647's receipt), rustfmt clean on the five changed Rust files,
    diff-check and gitleaks clean;
  - `build-run` on a CLI built at the tip: the chain prints `1` and the only-forwarder program `2`,
    each with its `(cranelift)` and `(llvm)` marker; the hidden parent and `Secret` exit 1 on both
    backends with E0101 `type not in scope`.
- [x] Replayed onto 9b40a042 (the progress DB only) as 227a6f18, with the same patch-id. The only
  difference from the gated tree is that DB.
