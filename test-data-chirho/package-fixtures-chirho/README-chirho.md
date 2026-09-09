<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Pinned upstream regression inputs

Parser and driver tests share these checked-in inputs. They are selected source
slices, **not complete packages or build dependencies**. Optional whole-package
integration tests still require their separately provisioned package cache.
Never point those tests at this partial inventory and call it a package build.

The versioned directories preserve upstream package-relative paths, identifiers,
copyright notices and licenses. Cabal files provide the original package/version
metadata for driver CPP preprocessing; they do not claim all listed modules have
been vendored. Inputs were copied from the local Hackage cache on 2026-09-08.
No source semantics were edited; terminal newlines were normalized. Upstream and
generated file sizes are intentionally preserved.

| Package version | Selected source |
| --- | --- |
| adjunctions 4.4.4 | Data.Functor.Contravariant.Rep |
| clock 0.8.4 | System.Clock.hsc |
| containers 0.8 | Data.IntSet.Internal plus include/containers.h |
| hashable 1.5.1.0 | Data.Hashable.FFI, XXH3, Mix plus package include headers |
| parsec 3.1.18.0 | Text.Parsec and its Pos, Error, Prim, Char, Combinator, Perm slice |
| primitive 0.9.1.0 | Data.Primitive.Types, ByteArray |
| text 2.1.4 | Data.Text.Internal.Fusion.CaseMapping |
| th-abstraction 0.7.2.0 | Language.Haskell.TH.Datatype |
| transformers 0.6.3.0 | legacy/pre711/Data/Functor/Classes.hs |

## Preprocessed parser controls

The parser tests consume pinned outputs so they do not depend on host CPP
behavior. Driver preprocessing tests consume the raw sources and exercise the
actual preprocessing path; the two scopes must not be confused.

The first two files in package-local preprocessed-chirho directories were generated on
2026-09-08 with GHC 9.14.1 (aarch64-darwin), using its Haskell-aware preprocessing
setup. Both commands completed with exit 0 and empty stderr:

```sh
ghc -E -cpp -optP-P -fforce-recomp \
  -I.haskelujah-packages-chirho/containers-0.8/include \
  -o /tmp/intset-preprocessed-chirho.hs \
  .haskelujah-packages-chirho/containers-0.8/src/Data/IntSet/Internal.hs
ghc -E -cpp -optP-P -fforce-recomp \
  -o /tmp/bytearray-preprocessed-chirho.hs \
  .haskelujah-packages-chirho/primitive-0.9.1.0/Data/Primitive/ByteArray.hs
```

The LINE pragmas record original paths; they do not read those paths at test time.
Regeneration with another GHC/platform deliberately changes conditional source
and must be reviewed as a fixture update. Raw inputs distinguish preprocessing
changes from parser changes.

| Pinned generated file | SHA-256 |
| --- | --- |
| containers-0.8/preprocessed-chirho/intset_chirho.hs | 1fd0c8e3ee5f90f15077d55a945d31b79aaa0aacd7e5d13d16d526ea1c0322d9 |
| primitive-0.9.1.0/preprocessed-chirho/bytearray_chirho.hs | 9dcb03d17437fcb370be6eae0103e9d6017836589bd2e2dc7809883ce20a684a |
| th-abstraction-0.7.2.0/preprocessed-chirho/datatype_chirho.hs | 3965f37f147bfd9a7909c0d64bc1f579f2599f1da05fef02ea48cef3325c83db |

The Datatype input was generated with the same GHC/version on 2026-09-08:

```sh
ghc -E -cpp -optP-P -fforce-recomp \
  -o /tmp/datatype-preprocessed-chirho.hs \
  test-data-chirho/package-fixtures-chirho/th-abstraction-0.7.2.0/src/Language/Haskell/TH/Datatype.hs
```

That command completed with exit 0 and GHC-98887: the upstream source requests
GeneralizedNewtypeDeriving together with Safe Haskell, so GHC ignores that extension.
The warning is recorded rather than suppressed; the raw upstream source is unchanged.
The parser control consumes the resulting conditional source instead of trying to
parse both sides of CPP directives.
