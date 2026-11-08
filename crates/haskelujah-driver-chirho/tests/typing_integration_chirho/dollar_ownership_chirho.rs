// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Source-owned and selected compiler application boundaries. The older row619 fixtures and
//! row714 compile contrasts have measured GHC references. The new first-class/laziness runtime
//! controls assert their named behavior without claiming a separately executed GHC reference.
//! workflow: compiler-pipeline-chirho/operator-fixity-chirho

use super::{assert_compile_success_chirho, assert_prints_everywhere_chirho};

/// A program's own `$`, signed and unsigned, a `where`-bound one, a declared `infixr 0` chain,
/// and one applied to a `do` block.
const OWNED_DOLLARS_CHIRHO: [(&str, &str, &str); 5] = [
    (
        "DHiddenDollarSigChirho.hs",
        "module Main where\nimport Prelude hiding (($))\n($) :: (Int -> Int) -> Int -> Int\nf $ x = f (f x)\nmain :: IO ()\nmain = print ((+ 1) $ 5)\n",
        "7\n",
    ),
    (
        "DHiddenDollarUnsignedChirho.hs",
        "module Main where\nimport Prelude hiding (($))\nf $ x = f (f x)\nmain :: IO ()\nmain = print ((+ 1) $ (5 :: Int))\n",
        "7\n",
    ),
    (
        "DWhereDollarChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((+ 1) $ (5 :: Int))\n  where\n    f $ x = f (f x)\n",
        "7\n",
    ),
    (
        "DDollarChainChirho.hs",
        "module Main where\nimport Prelude hiding (($))\ninfixr 0 $\n($) :: (Int -> Int) -> Int -> Int\nf $ x = f x * 10\nmain :: IO ()\nmain = print ((+ 1) $ (* 2) $ 3)\n",
        "610\n",
    ),
    (
        "DDollarDoChirho.hs",
        "module Main where\nimport Prelude hiding (($))\n($) :: (IO () -> IO ()) -> IO () -> IO ()\nf $ x = f x >> f x\nmain :: IO ()\nmain = id $ do\n  putStrLn \"once\"\n",
        "once\nonce\n",
    ),
];

/// What held: the Prelude's `$` in chains and before `map`, and sections of an owned `$`.
const DOLLAR_HOLDS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "HPreludeDollarChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print $ (+ 1) $ (* 2) $ 3\n  mapM_ print $ map (subtract 1) [1, 2 :: Int]\n",
        "7\n0\n1\n",
    ),
    (
        "DDollarSectionsChirho.hs",
        "module Main where\nimport Prelude hiding (($))\n($) :: (Int -> Int) -> Int -> Int\nf $ x = f (f x)\nmain :: IO ()\nmain = print (map ((+ 1) $) [5], map ($ 5) [(* 2)])\n",
        "([7],[20])\n",
    ),
];

#[test]
fn an_owned_or_local_dollar_keeps_its_own_binding_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OWNED_DOLLARS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn the_prelude_dollar_and_owned_sections_hold_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in DOLLAR_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// `runST $ do ...` keeps type-checking with `$` kept as an operator: the Prelude's `$` is typed as
/// before. (Running `runST` is a separate gap on main; GHC prints 55.)
#[test]
fn run_st_dollar_do_still_type_checks_chirho() {
    assert_compile_success_chirho(
        "HRunSTDoChirho.hs",
        "module Main where\nimport Control.Monad.ST\nimport Data.STRef\nsumST :: [Int] -> Int\nsumST xs = runST $ do\n  ref <- newSTRef 0\n  mapM_ (\\x -> modifySTRef ref (+ x)) xs\n  readSTRef ref\nmain :: IO ()\nmain = print (sumST [1 .. 10])\n",
    );
}

// Row714 uses the exact measured reference bytes and caller options. These tests
// enter the ordinary search/admission pipeline; none fabricates an interface.
use haskelujah_driver::{
    FrontendOptionsChirho, compile_source_with_search_path_and_options_chirho,
    eval_modules_with_machine_chirho, eval_source_with_machine_chirho,
};
use haskelujah_span_chirho::SourceMapChirho;
use std::path::PathBuf;

struct ApplyFixtureChirho {
    file_name_chirho: &'static str,
    source_chirho: &'static str,
    providers_chirho: &'static [(&'static str, &'static str)],
    options_chirho: &'static [&'static str],
    refusal_chirho: Option<&'static str>,
}

const RANK_APPLY_OPTIONS_CHIRHO: &[&str] =
    &["-XHaskell2010", "-XRankNTypes", "-XNoImpredicativeTypes"];
const IMPREDICATIVE_APPLY_OPTIONS_CHIRHO: &[&str] =
    &["-XHaskell2010", "-XRankNTypes", "-XImpredicativeTypes"];
const APPLY_PROVIDER_0_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module DollarReexportProviderChirho (($)) where

import Prelude (($))
"#;
const APPLY_PROVIDER_1_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module DollarCloneProviderChirho (($)) where

infixr 0 $
($) :: (firstChirho -> secondChirho) -> firstChirho -> secondChirho
functionChirho $ argumentChirho = functionChirho argumentChirho
"#;
const APPLY_PROVIDER_2_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module OwnDollarProviderChirho (($)) where

import Prelude (Int, (+))

($) :: Int -> Int -> Int
leftChirho $ rightChirho = leftChirho + rightChirho + 1
"#;
const APPLY_PROVIDER_3_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module DollarCloneProviderChirho (($)) where

($) :: (firstChirho -> secondChirho) -> firstChirho -> secondChirho
functionChirho $ argumentChirho = functionChirho argumentChirho
"#;
const APPLY_FIXTURES_CHIRHO: [ApplyFixtureChirho; 13] = [
    ApplyFixtureChirho {
        file_name_chirho: "T11339b.hs",
        source_chirho: r#"{-# LANGUAGE NoMonomorphismRestriction, RankNTypes, ScopedTypeVariables #-}

module T11339b where

import Control.Applicative ( Const(Const, getConst) )
import Data.Functor.Identity ( Identity(Identity) )

type Traversal s t a b = forall f. Applicative f => (a -> f b) -> s -> f t

failing :: forall s t a b . Traversal s t a b -> Traversal s t a b -> Traversal s t a b
failing left right afb s = case pins t of
  [] -> right afb s
  _  -> t afb
  where
    t :: Applicative f => (a -> f b) -> f t
    -- Works because of NoMonomorphismRestriction
    Bazaar { getBazaar = t } = left sell s

    sell :: a -> Bazaar a b b
    sell w   = Bazaar ($ w)

    pins :: ((a -> Const [Identity a] b) -> Const [Identity a] t) -> [Identity a]
    pins f   = getConst (f (\ra -> Const [Identity ra]))

newtype Bazaar a b t = Bazaar { getBazaar :: (forall f. Applicative f => (a -> f b) -> f t) }

instance Functor (Bazaar a b) where
  fmap f (Bazaar k) = Bazaar (fmap f . k)

instance Applicative (Bazaar a b) where
  pure a = Bazaar $ \_ -> pure a
  Bazaar mf <*> Bazaar ma = Bazaar $ \afb -> mf afb <*> ma afb
"#,
        providers_chirho: &[],
        options_chirho: &["-fno-warn-incomplete-patterns"],
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "ImplicitPreludeDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ImplicitPreludeDollarChirho where


data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho $ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "ExplicitPreludeDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module ExplicitPreludeDollarChirho where

import Prelude (($))

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho $ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "QualifiedPreludeDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module QualifiedPreludeDollarChirho where

import qualified Prelude as PreludeChirho

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho PreludeChirho.$ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "ReexportedDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module ReexportedDollarChirho where

import DollarReexportProviderChirho (($))

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho $ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[("DollarReexportProviderChirho.hs", APPLY_PROVIDER_0_CHIRHO)],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "QualifiedReexportDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module QualifiedReexportDollarChirho where

import qualified DollarReexportProviderChirho as ExportChirho

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho ExportChirho.$ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[("DollarReexportProviderChirho.hs", APPLY_PROVIDER_0_CHIRHO)],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "QualifiedSourceCloneDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module QualifiedSourceCloneDollarChirho where

import qualified DollarCloneProviderChirho as CloneChirho

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho CloneChirho.$ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[("DollarCloneProviderChirho.hs", APPLY_PROVIDER_1_CHIRHO)],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: Some("E0200"),
    },
    ApplyFixtureChirho {
        file_name_chirho: "OrdinaryAliasDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module OrdinaryAliasDollarChirho where

import qualified Prelude as PreludeChirho

applyChirho :: (firstChirho -> secondChirho) -> firstChirho -> secondChirho
applyChirho = (PreludeChirho.$)

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = applyChirho RankBoxChirho (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: Some("E0200"),
    },
    ApplyFixtureChirho {
        file_name_chirho: "HiddenDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module HiddenDollarChirho where

import Prelude hiding (($))

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho $ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: Some("unbound variable"),
    },
    ApplyFixtureChirho {
        file_name_chirho: "LicensedSourceCloneDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module LicensedSourceCloneDollarChirho where

import qualified DollarCloneProviderChirho as CloneChirho

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = RankBoxChirho CloneChirho.$ (\itemChirho -> itemChirho)
"#,
        providers_chirho: &[("DollarCloneProviderChirho.hs", APPLY_PROVIDER_1_CHIRHO)],
        options_chirho: IMPREDICATIVE_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "MonomorphicUnimportedDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module MonomorphicUnimportedDollarChirho where

import Prelude (Int, IO, print)

identityChirho :: Int -> Int
identityChirho itemChirho = itemChirho

witnessChirho :: Int
witnessChirho = identityChirho $ (2 :: Int)
"#,
        providers_chirho: &[],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: Some("unbound variable"),
    },
    ApplyFixtureChirho {
        file_name_chirho: "SourceOwnedMonomorphicDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module Main where

import Prelude (IO, print)
import qualified OwnDollarProviderChirho as OwnChirho

main :: IO ()
main = print ((OwnChirho.$) 2 3)
"#,
        providers_chirho: &[("OwnDollarProviderChirho.hs", APPLY_PROVIDER_2_CHIRHO)],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: None,
    },
    ApplyFixtureChirho {
        file_name_chirho: "NestedSourceHeadDollarChirho.hs",
        source_chirho: r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module NestedSourceHeadDollarChirho where

import qualified Prelude as PreludeChirho
import qualified DollarCloneProviderChirho as CloneChirho

data RankBoxChirho = RankBoxChirho (forall itemChirho. itemChirho -> itemChirho)

witnessChirho :: RankBoxChirho
witnessChirho = (PreludeChirho.$) PreludeChirho.id (RankBoxChirho CloneChirho.$ (\itemChirho -> itemChirho))
"#,
        providers_chirho: &[("DollarCloneProviderChirho.hs", APPLY_PROVIDER_3_CHIRHO)],
        options_chirho: RANK_APPLY_OPTIONS_CHIRHO,
        refusal_chirho: Some("E0200"),
    },
];

struct ApplySourceTreeChirho(PathBuf);

impl Drop for ApplySourceTreeChirho {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn assert_apply_fixture_chirho(index_chirho: usize) {
    let fixture_chirho = &APPLY_FIXTURES_CHIRHO[index_chirho];
    let nonce_chirho = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after epoch Chirho")
        .as_nanos();
    let tree_chirho = ApplySourceTreeChirho(std::env::temp_dir().join(format!(
        "haskelujah-selected-apply-{}-{nonce_chirho}-chirho",
        std::process::id()
    )));
    std::fs::create_dir(&tree_chirho.0).expect("create own source tree Chirho");
    for (file_name_chirho, source_chirho) in fixture_chirho.providers_chirho {
        std::fs::write(tree_chirho.0.join(file_name_chirho), source_chirho)
            .expect("write exact provider Chirho");
    }
    std::fs::write(
        tree_chirho.0.join(fixture_chirho.file_name_chirho),
        fixture_chirho.source_chirho,
    )
    .expect("write exact consumer Chirho");
    let options_chirho = FrontendOptionsChirho::from_ghc_options_chirho(
        fixture_chirho.options_chirho.iter().copied(),
    );
    let result_chirho = compile_source_with_search_path_and_options_chirho(
        fixture_chirho.source_chirho,
        &mut SourceMapChirho::new_chirho(),
        fixture_chirho.file_name_chirho,
        &tree_chirho.0,
        &options_chirho,
    );
    match fixture_chirho.refusal_chirho {
        None => assert!(
            result_chirho.is_ok(),
            "{}: expected acceptance, got {:?}",
            fixture_chirho.file_name_chirho,
            result_chirho.err()
        ),
        Some(reason_chirho) => {
            let error_chirho = match result_chirho {
                Ok(_) => panic!("{}: expected refusal", fixture_chirho.file_name_chirho),
                Err(error_chirho) => error_chirho.to_string(),
            };
            assert!(
                error_chirho.contains(reason_chirho),
                "{}: refusal must name {reason_chirho}: {error_chirho}",
                fixture_chirho.file_name_chirho
            );
        }
    }
}

#[test]
fn selected_apply_t11339b_original_chirho() {
    assert_apply_fixture_chirho(0);
}

#[test]
fn selected_apply_implicit_prelude_dollar_chirho() {
    assert_apply_fixture_chirho(1);
}

#[test]
fn selected_apply_explicit_prelude_dollar_chirho() {
    assert_apply_fixture_chirho(2);
}

#[test]
fn selected_apply_qualified_prelude_dollar_chirho() {
    assert_apply_fixture_chirho(3);
}

#[test]
fn selected_apply_reexported_dollar_chirho() {
    assert_apply_fixture_chirho(4);
}

#[test]
fn selected_apply_qualified_reexport_dollar_chirho() {
    assert_apply_fixture_chirho(5);
}

#[test]
fn selected_apply_qualified_source_clone_dollar_chirho() {
    assert_apply_fixture_chirho(6);
}

#[test]
fn selected_apply_ordinary_alias_dollar_chirho() {
    assert_apply_fixture_chirho(7);
}

#[test]
fn selected_apply_hidden_dollar_chirho() {
    assert_apply_fixture_chirho(8);
}

#[test]
fn selected_apply_licensed_source_clone_dollar_chirho() {
    assert_apply_fixture_chirho(9);
}

#[test]
fn selected_apply_monomorphic_unimported_dollar_chirho() {
    assert_apply_fixture_chirho(10);
}

#[test]
fn selected_apply_source_owned_monomorphic_dollar_chirho() {
    assert_apply_fixture_chirho(11);
}

#[test]
fn selected_apply_nested_source_head_dollar_chirho() {
    assert_apply_fixture_chirho(12);
}

/// This monomorphic body uses the real multi-module compiler/evaluator route.
/// Its bytes are the measured provider/consumer, without reconstructed ifaces.
#[test]
fn a_source_owned_monomorphic_dollar_runs_its_distinct_body_chirho() {
    let fixture_chirho = &APPLY_FIXTURES_CHIRHO[11];
    let mut sources_chirho = fixture_chirho.providers_chirho.to_vec();
    sources_chirho.push((
        fixture_chirho.file_name_chirho,
        fixture_chirho.source_chirho,
    ));
    let (_, machine_chirho) = eval_modules_with_machine_chirho(
        &sources_chirho,
        &mut SourceMapChirho::new_chirho(),
        Some("main"),
    )
    .expect("source-owned dollar must run its own body Chirho");
    assert_eq!(machine_chirho.io_output_chirho, "6\n");
}

#[test]
fn a_first_class_compiler_dollar_has_an_ordinary_body_chirho() {
    let source_chirho = "module Main where\nmain :: IO ()\nmain = print (($) (+ 1) (41 :: Int))\n";
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "FirstClassApplyChirho.hs",
        None,
    )
    .expect("first-class compiler dollar must have a body Chirho");
    assert_eq!(machine_chirho.io_output_chirho, "42\n");
}

#[test]
fn a_first_class_compiler_dollar_keeps_its_argument_lazy_chirho() {
    let source_chirho = "module Main where\nmain :: IO ()\nmain = print (($) (const (42 :: Int)) (error \"unused Chirho\"))\n";
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "LazyApplyChirho.hs",
        None,
    )
    .expect("compiler dollar must not evaluate an unused argument Chirho");
    assert_eq!(machine_chirho.io_output_chirho, "42\n");
}
