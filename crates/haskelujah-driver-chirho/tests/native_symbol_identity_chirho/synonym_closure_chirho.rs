// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 656: a re-exported type alias expands through a parent alias its user never imported by
//! name. A type name whose one exporter is among a module's imports is spelled bare by the import
//! rewrite and by inference, so its synonym needs the bare key the frontend now aligns; that key
//! serves expansion only, and naming still refuses a hidden name the source writes.
//!
//! Row 668: the interpreter gives each module only the synonyms of the modules it imports, as
//! every native build does. Before, a module saw the bare synonyms of every module compiled
//! before it, so a type named like one of them was rewritten to that synonym's right-hand side: a
//! derived `Show` became `instance Show Int`, and the interpreter refused the program as a
//! duplicate instance. Written synonyms keep their import scope: qualified-only, hidden and
//! unexported ones stay unwritable, qualified and aliased ones resolve.
//!
//! Each program and its expected output or rejection are GHC 9.14.1's, measured on the text
//! embedded here, and each runs on the interpreter, LLVM and true Cranelift.
//!
//! Placement: these programs test type-synonym expansion and scope, not symbol identity. They
//! live in this target only because its harness already runs a multi-module Cabal program on all
//! three engines, which keeps the integration-test root at 27 entries (gpt_chirho #30444).
//! workflow: compiler-pipeline-chirho/type-scope-resolution-chirho

use super::import_identity_chirho::{ProgramChirho, assert_exact_chirho, assert_refused_chirho};

/// the hidden-parent chain: Main imports PrimAlias without `ParsecAlias` and uses CompatTop's re-exported `GenParser`. GHC prints '1'.
const CHAIN_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "PrimAliasCompatMiniChirho",
            r#"module PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho(..), ParsecTAliasCompatMiniChirho(..), ParsecAliasCompatMiniChirho) where
data IdentityAliasCompatMiniChirho aChirho = IdentityAliasCompatMiniChirho aChirho
newtype ParsecTAliasCompatMiniChirho sChirho uChirho mChirho aChirho = ParsecTAliasCompatMiniChirho { unParsecTAliasCompatMiniChirho :: sChirho -> mChirho aChirho }
type ParsecAliasCompatMiniChirho sChirho uChirho aChirho = ParsecTAliasCompatMiniChirho sChirho uChirho IdentityAliasCompatMiniChirho aChirho
"#,
        ),
        (
            "CompatPrimMiniChirho",
            r#"module CompatPrimMiniChirho (GenParserCompatMiniChirho) where
import PrimAliasCompatMiniChirho (ParsecAliasCompatMiniChirho)
type GenParserCompatMiniChirho tokChirho stChirho aChirho = ParsecAliasCompatMiniChirho [tokChirho] stChirho aChirho
"#,
        ),
        (
            "CompatTopMiniChirho",
            r#"module CompatTopMiniChirho (module CompatPrimMiniChirho) where
import CompatPrimMiniChirho
"#,
        ),
        (
            "Main",
            r#"module Main where
import PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho, ParsecTAliasCompatMiniChirho)
import CompatTopMiniChirho (GenParserCompatMiniChirho)
coerceCompatMiniChirho :: GenParserCompatMiniChirho tokChirho stChirho aChirho -> ParsecTAliasCompatMiniChirho [tokChirho] stChirho IdentityAliasCompatMiniChirho aChirho
coerceCompatMiniChirho parserChirho = parserChirho
main :: IO ()
main = print (1 :: Int)
"#,
        ),
    ],
    r#"1
"#,
);

/// Main imports only the forwarder CompatTop. GHC prints '2'.
const ONLY_TOP_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "PrimAliasCompatMiniChirho",
            r#"module PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho(..), ParsecTAliasCompatMiniChirho(..), ParsecAliasCompatMiniChirho) where
data IdentityAliasCompatMiniChirho aChirho = IdentityAliasCompatMiniChirho aChirho
newtype ParsecTAliasCompatMiniChirho sChirho uChirho mChirho aChirho = ParsecTAliasCompatMiniChirho { unParsecTAliasCompatMiniChirho :: sChirho -> mChirho aChirho }
type ParsecAliasCompatMiniChirho sChirho uChirho aChirho = ParsecTAliasCompatMiniChirho sChirho uChirho IdentityAliasCompatMiniChirho aChirho
"#,
        ),
        (
            "CompatPrimMiniChirho",
            r#"module CompatPrimMiniChirho (GenParserCompatMiniChirho) where
import PrimAliasCompatMiniChirho (ParsecAliasCompatMiniChirho)
type GenParserCompatMiniChirho tokChirho stChirho aChirho = ParsecAliasCompatMiniChirho [tokChirho] stChirho aChirho
"#,
        ),
        (
            "CompatTopMiniChirho",
            r#"module CompatTopMiniChirho (module CompatPrimMiniChirho) where
import CompatPrimMiniChirho
"#,
        ),
        (
            "Main",
            r#"module Main where
import CompatTopMiniChirho (GenParserCompatMiniChirho)
idParserChirho :: GenParserCompatMiniChirho Int Int Int -> GenParserCompatMiniChirho Int Int Int
idParserChirho p = p
main :: IO ()
main = print (2 :: Int)
"#,
        ),
    ],
    r#"2
"#,
);

/// Main derives `Show` for its own `CellChirho` and imports one value from a module whose synonym has that name. GHC prints 'CellChirho True\n41'.
const LOCAL_SHOW_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "Main",
            r#"module Main where

import SynSourceChirho (sourceValueChirho)

data CellChirho = CellChirho Bool deriving Show

main :: IO ()
main = do
  print (CellChirho True)
  print sourceValueChirho
"#,
        ),
    ],
    r#"CellChirho True
41
"#,
);

/// Main takes `CellChirho` from a module that never imports the synonym of that name, and gives a class instances at it and at `Int`. GHC prints 'cell\nint'.
const IMPORTED_CLASS_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "DataSourceChirho",
            r#"module DataSourceChirho (CellChirho (..), fullCellChirho) where

data CellChirho = CellChirho Bool deriving Show

fullCellChirho :: CellChirho
fullCellChirho = CellChirho True
"#,
        ),
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "Main",
            r#"module Main where

import DataSourceChirho (CellChirho (..), fullCellChirho)
import SynSourceChirho (sourceValueChirho)

class DescribeChirho a where
  describeChirho :: a -> String

instance DescribeChirho Int where
  describeChirho _ = "int"

instance DescribeChirho CellChirho where
  describeChirho _ = "cell"

main :: IO ()
main = do
  putStrLn (describeChirho fullCellChirho)
  putStrLn (describeChirho sourceValueChirho)
"#,
        ),
    ],
    r#"cell
int
"#,
);

/// only MidChirho imports the synonym, and Main declares its own `CellChirho`. GHC prints '(False,42)'.
const UPSTREAM_ONLY_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "MidChirho",
            r#"module MidChirho (bumpChirho) where

import SynSourceChirho (CellChirho)

bumpChirho :: CellChirho -> CellChirho
bumpChirho = (+ 1)
"#,
        ),
        (
            "Main",
            r#"module Main where

import MidChirho (bumpChirho)

data CellChirho = CellChirho Bool

openChirho :: CellChirho -> Bool
openChirho (CellChirho flagChirho) = flagChirho

main :: IO ()
main = print (openChirho (CellChirho False), bumpChirho 41)
"#,
        ),
    ],
    r#"(False,42)
"#,
);

/// Main writes the synonym through its qualified import. GHC prints '42'.
const QUALIFIED_USE_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "Main",
            r#"module Main where

import qualified SynSourceChirho as S

countChirho :: S.CellChirho
countChirho = 1

main :: IO ()
main = print (S.sourceValueChirho + countChirho)
"#,
        ),
    ],
    r#"42
"#,
);

/// Main writes the synonym through a second, aliased import of its module. GHC prints '42'.
const ALIASED_USE_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "Main",
            r#"module Main where

import qualified SynSourceChirho as Cells
import SynSourceChirho (sourceValueChirho)

countChirho :: Cells.CellChirho
countChirho = 1

main :: IO ()
main = print (sourceValueChirho + countChirho)
"#,
        ),
    ],
    r#"42
"#,
);

/// Main writes the hidden parent `ParsecAlias` it never imported; GHC rejects it (GHC-76037).
const HIDDEN_PARENT_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "PrimAliasCompatMiniChirho",
            r#"module PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho(..), ParsecTAliasCompatMiniChirho(..), ParsecAliasCompatMiniChirho) where
data IdentityAliasCompatMiniChirho aChirho = IdentityAliasCompatMiniChirho aChirho
newtype ParsecTAliasCompatMiniChirho sChirho uChirho mChirho aChirho = ParsecTAliasCompatMiniChirho { unParsecTAliasCompatMiniChirho :: sChirho -> mChirho aChirho }
type ParsecAliasCompatMiniChirho sChirho uChirho aChirho = ParsecTAliasCompatMiniChirho sChirho uChirho IdentityAliasCompatMiniChirho aChirho
"#,
        ),
        (
            "CompatPrimMiniChirho",
            r#"module CompatPrimMiniChirho (GenParserCompatMiniChirho) where
import PrimAliasCompatMiniChirho (ParsecAliasCompatMiniChirho)
type GenParserCompatMiniChirho tokChirho stChirho aChirho = ParsecAliasCompatMiniChirho [tokChirho] stChirho aChirho
"#,
        ),
        (
            "CompatTopMiniChirho",
            r#"module CompatTopMiniChirho (module CompatPrimMiniChirho) where
import CompatPrimMiniChirho
"#,
        ),
        (
            "Main",
            r#"module Main where
import PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho, ParsecTAliasCompatMiniChirho)
import CompatTopMiniChirho (GenParserCompatMiniChirho)
leakChirho :: ParsecAliasCompatMiniChirho Int Int Int -> ParsecAliasCompatMiniChirho Int Int Int
leakChirho x = x
main :: IO ()
main = print (1 :: Int)
"#,
        ),
    ],
    r#"type not in scope: `ParsecAliasCompatMiniChirho`"#,
);

/// Main writes `Secret`, which Other exports but Main does not import; GHC rejects it (GHC-76037).
const SECRET_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "Other",
            r#"module Other (Secret, val) where
type Secret = Int
val :: Int
val = 1
"#,
        ),
        (
            "Main",
            r#"module Main where
import Other (val)
x :: Secret
x = val
main :: IO ()
main = print x
"#,
        ),
    ],
    r#"type not in scope: `Secret`"#,
);

/// Main writes the synonym bare after importing its module only qualified; GHC rejects it (GHC-76037).
const QUALIFIED_ONLY_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "Main",
            r#"module Main where

import qualified SynSourceChirho as S

countChirho :: CellChirho
countChirho = 1

main :: IO ()
main = print (S.sourceValueChirho + countChirho)
"#,
        ),
    ],
    r#"type not in scope: `CellChirho`"#,
);

/// Main writes the synonym its import hides; GHC rejects it (GHC-76037).
const HIDDEN_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "Main",
            r#"module Main where

import SynSourceChirho hiding (CellChirho)

countChirho :: CellChirho
countChirho = 1

main :: IO ()
main = print (sourceValueChirho + countChirho)
"#,
        ),
    ],
    r#"type not in scope: `CellChirho`"#,
);

/// Main writes the synonym MidChirho uses but does not re-export; GHC rejects it (GHC-76037).
const NOT_REEXPORTED_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "SynSourceChirho",
            r#"module SynSourceChirho (CellChirho, sourceValueChirho) where

type CellChirho = Int

sourceValueChirho :: CellChirho
sourceValueChirho = 41
"#,
        ),
        (
            "MidChirho",
            r#"module MidChirho (bumpChirho) where

import SynSourceChirho (CellChirho)

bumpChirho :: CellChirho -> CellChirho
bumpChirho = (+ 1)
"#,
        ),
        (
            "Main",
            r#"module Main where

import MidChirho (bumpChirho)

countChirho :: CellChirho
countChirho = 1

main :: IO ()
main = print (bumpChirho countChirho)
"#,
        ),
    ],
    r#"type not in scope: `CellChirho`"#,
);

#[test]
fn a_re_exported_alias_expands_through_its_hidden_parent_chirho() {
    assert_exact_chirho(CHAIN_CHIRHO);
}

#[test]
fn importing_only_the_forwarder_still_expands_chirho() {
    assert_exact_chirho(ONLY_TOP_CHIRHO);
}

#[test]
fn the_hidden_parent_stays_unwritable_chirho() {
    assert_refused_chirho(HIDDEN_PARENT_CHIRHO);
}

#[test]
fn an_unimported_synonym_stays_unwritable_chirho() {
    assert_refused_chirho(SECRET_CHIRHO);
}

#[test]
fn an_earlier_modules_synonym_never_renames_a_local_type_chirho() {
    assert_exact_chirho(LOCAL_SHOW_CHIRHO);
}

#[test]
fn an_unimported_modules_synonym_never_renames_an_imported_type_chirho() {
    assert_exact_chirho(IMPORTED_CLASS_CHIRHO);
}

#[test]
fn a_synonym_used_upstream_stays_upstream_chirho() {
    assert_exact_chirho(UPSTREAM_ONLY_CHIRHO);
}

#[test]
fn a_qualified_import_brings_its_synonym_qualified_chirho() {
    assert_exact_chirho(QUALIFIED_USE_CHIRHO);
}

#[test]
fn an_aliased_import_brings_its_synonym_under_the_alias_chirho() {
    assert_exact_chirho(ALIASED_USE_CHIRHO);
}

#[test]
fn a_qualified_only_synonym_stays_unwritable_bare_chirho() {
    assert_refused_chirho(QUALIFIED_ONLY_CHIRHO);
}

#[test]
fn a_hidden_synonym_stays_unwritable_chirho() {
    assert_refused_chirho(HIDDEN_CHIRHO);
}

#[test]
fn a_synonym_not_re_exported_stays_unwritable_chirho() {
    assert_refused_chirho(NOT_REEXPORTED_CHIRHO);
}
