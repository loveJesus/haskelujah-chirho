<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Package ladder inputs (row 703)

These are the inputs for the first rungs of the real-package ladder. No reference output is
committed yet. Each rung's expected stdout is pinned here only after an actual GHC 9.14.1 run.

| Rung | Project | What it proves when it passes |
|---|---|---|
| L0 | `l0-chirho/`: Main, App.CalcChirho, App.ReportChirho; base only; both helpers export `labelChirho` | a useful multi-module app builds, links and prints GHC's output, and same-named bindings in two modules stay distinct |
| L0 negative | `l0-bad-chirho/`: one module puts a String where an Int belongs | an invalid program is refused and leaves no executable |
| L0.5 | `packages-chirho/haskell-say-1.0.0.0/`, built as a project (`haskell-say-exe`) | an unchanged package's own library reaches its own executable, with the listed, unshipped `Paths_haskell_say` |
| L1 | `l1-chirho/`: `haskell-say ==1.0.0.0` from a store | a real external package's body runs; this must fail until dependency Core is linked |
| L1 negative | `l1-unsat-chirho/`: `haskell-say >=2` | an unsatisfiable bound is refused |

`packages-chirho/haskell-say-1.0.0.0/` is the package's own source, BSD3, unchanged. It matches the
workspace's shared package cache copy minus that copy's stale `dist-chirho/` build output.
`make_ladder_scratch_chirho.py --shared-cache <cache>` re-proves the match.

`make_ladder_scratch_chirho.py --out <fresh dir>` lays out one run without any compiler. A run then:
1. installs the package with `haskelujah install --from <run>/haskell-say-1.0.0.0-src --store
   <run>/.haskelujah-packages-chirho`;
2. builds each rung with `haskelujah build-run`;
3. compares stdout bytes with the GHC run of the same sources.

Builds stay out of these committed inputs: every build happens in the scratch copy. Probe
executables are deleted once their receipts are written.

The install step is currently expected to refuse haskell-say, because its library bounds
`base >=4.7 && <5` and the compiler declares no `base` version (see
`spec-chirho/workflows-chirho/development-chirho/package-source-install-chirho.md`). The
`--copy-store` control lets the build rungs run without that step.
