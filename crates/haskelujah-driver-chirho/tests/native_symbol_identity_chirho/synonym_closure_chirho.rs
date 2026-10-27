// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 656: a re-exported type alias expands through a parent alias its user never imported by
//! name. A type name whose one exporter is among a module's imports is spelled bare by the import
//! rewrite and by inference, so its synonym needs the bare key the frontend now aligns; that key
//! serves expansion only, and naming still refuses a hidden name the source writes. Each program
//! and its expected output or rejection are GHC 9.14.1's, measured on the text embedded here, and
//! each runs on the interpreter, LLVM and true Cranelift.
//!
//! Placement: these programs test type-synonym expansion, not symbol identity. They live in this
//! target only because its harness already runs a multi-module Cabal program on all three
//! engines, which keeps the integration-test root at 27 entries (gpt_chirho #30444).
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
