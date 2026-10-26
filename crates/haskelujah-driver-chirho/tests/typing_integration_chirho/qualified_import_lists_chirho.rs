// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Explicit import lists bind their qualifier (row 536, gpt_chirho #28483 from the I2 trace
//! #28482). `import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))` checked
//! `NE.toList`, which GHC 9.14.1 rejects with GHC-76037: naming never checked a qualified value
//! against the list, and typing's fallback read the bare `toList`.
//!
//! Each program here ran under GHC 9.14.1, and its behavior is recorded beside it. Name
//! resolution is called directly on the lowered program, with the builtin interfaces and the
//! implicit Prelude import the driver adds, so each control measures this unit's rule rather
//! than a later phase. I2 also runs through the whole driver.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_driver::compile_source_chirho;
use haskelujah_naming_chirho::{builtin_module_ifaces_chirho, resolve_module_with_imports_chirho};
use haskelujah_parser_chirho::cst_parser_chirho::parse_to_cst_chirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{FileIdChirho, SourceMapChirho, SpanChirho};

/// Every message name resolution gives `source_chirho`. As the driver does, a module that
/// does not import the Prelude itself imports all of it.
fn naming_messages_chirho(source_chirho: &str) -> Vec<String> {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let mut module_chirho = lower_module_chirho(
        &parse_to_cst_chirho(source_chirho, file_chirho),
        file_chirho,
    );
    if !module_chirho
        .imports_chirho
        .iter()
        .any(|import_chirho| import_chirho.module_chirho.text_chirho() == "Prelude")
    {
        module_chirho.imports_chirho.push(ImportDeclChirho {
            module_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
                "Prelude",
                SpanChirho::DUMMY_CHIRHO,
            )),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });
    }
    resolve_module_with_imports_chirho(&module_chirho, &builtin_module_ifaces_chirho())
        .diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .map(|diagnostic_chirho| diagnostic_chirho.message_chirho.clone())
        .collect()
}

/// GHC 9.14.1 rejects each with GHC-76037, naming exactly these references: the import lists
/// behind their qualifiers do not supply them.
const REJECTED_CHIRHO: &[(&str, &str, &[&str])] = &[
    (
        // GHC: Not in scope: ‘NE.toList’.
        "I2_import_list_only_head_chirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))\n\
         main :: IO ()\n\
         main = print (NE.toList (1 NE.:| [2 :: Int]))\n",
        &["variable not in scope: `NE.toList`"],
    ),
    (
        // GHC rename/should_fail/T19843i's shape. GHC: Not in scope: ‘M.getSum’.
        "AliasListChirho.hs",
        "module Main where\n\
         import Data.Monoid as M (getAll)\n\
         main :: IO ()\n\
         main = M.getSum `seq` return ()\n",
        &["variable not in scope: `M.getSum`"],
    ),
    (
        // GHC module/mod88's shape, a constructor expression. GHC: Not in scope: data
        // constructor ‘Prelude.Left’.
        "ConstructorChirho.hs",
        "module ConstructorChirho where\n\
         import qualified Prelude (map)\n\
         x = Prelude.Left 'a'\n",
        &["data constructor not in scope: `Prelude.Left`"],
    ),
    (
        // An empty list supplies nothing. GHC: Not in scope: ‘M.size’ and ‘M.empty’.
        "EmptyListChirho.hs",
        "module Main where\n\
         import qualified Data.Map as M ()\n\
         main :: IO ()\n\
         main = print (M.size M.empty)\n",
        &[
            "variable not in scope: `M.size`",
            "variable not in scope: `M.empty`",
        ],
    ),
    (
        // An operator. GHC: Not in scope: ‘M.!’.
        "OperatorChirho.hs",
        "module Main where\n\
         import qualified Data.Map as M (fromList)\n\
         main :: IO ()\n\
         main = print (M.fromList [(1 :: Int, 'a')] M.! 1)\n",
        &["variable not in scope: `M.!`"],
    ),
    (
        // Two lists under one alias supply their union only. GHC: Not in scope: ‘L.head’.
        "SharedAliasChirho.hs",
        "module Main where\n\
         import qualified Data.List as L (sortOn)\n\
         import qualified Data.List.NonEmpty as L (toList)\n\
         main :: IO ()\n\
         main = print (L.head (L.sortOn negate [1, 2 :: Int]))\n",
        &["variable not in scope: `L.head`"],
    ),
    (
        // An unqualified list closes the full module name. GHC: Not in scope: ‘Data.List.nub’.
        "FullQualifierChirho.hs",
        "module Main where\n\
         import Data.List (sort)\n\
         main :: IO ()\n\
         main = print (Data.List.nub (sort [2, 1, 2 :: Int]))\n",
        &["variable not in scope: `Data.List.nub`"],
    ),
];

/// GHC 9.14.1 accepts each; the recorded value is what it printed. Each keeps its qualifier's
/// supply, or leaves the qualifier open, so name resolution reports nothing.
const ADMITTED_CHIRHO: &[(&str, &str)] = &[
    (
        // A listed variable, and `:|` from `NonEmpty(..)`. GHC: 1.
        "ListedChirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))\n\
         main :: IO ()\n\
         main = print (NE.head (1 NE.:| [2 :: Int]))\n",
    ),
    (
        // `NonEmpty(..)` alone. GHC: [2].
        "TypeMembersChirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty as NE (NonEmpty(..))\n\
         main :: IO ()\n\
         main = case 1 NE.:| [2 :: Int] of\n\
         \x20 _ NE.:| rest -> print rest\n",
    ),
    (
        // An open alias. GHC: [1,2].
        "OpenAliasChirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty as NE\n\
         main :: IO ()\n\
         main = print (NE.toList (NE.fromList [1, 2 :: Int]))\n",
    ),
    (
        // `hiding`. GHC: [1,2].
        "HidingChirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty as NE hiding (head)\n\
         main :: IO ()\n\
         main = print (NE.toList (NE.fromList [1, 2 :: Int]))\n",
    ),
    (
        // An open full import. GHC: [1,2].
        "OpenFullChirho.hs",
        "module Main where\n\
         import qualified Data.List\n\
         main :: IO ()\n\
         main = print (Data.List.sort [2, 1 :: Int])\n",
    ),
    (
        // A listed name under the full module name. GHC: [1,2].
        "ListedFullChirho.hs",
        "module Main where\n\
         import Data.List (sort)\n\
         main :: IO ()\n\
         main = print (Data.List.sort [2, 1 :: Int])\n",
    ),
    (
        // An explicit list beside an open import of the same qualifier. GHC: Just 'a'.
        "ListBesideOpenChirho.hs",
        "module Main where\n\
         import qualified Data.Map as M (insert)\n\
         import qualified Data.Map as M\n\
         main :: IO ()\n\
         main = print (M.lookup (1 :: Int) (M.insert 1 'a' M.empty))\n",
    ),
    (
        // Two lists under one alias, each supplying its own names. GHC: [2,1].
        "SharedAliasUnionChirho.hs",
        "module Main where\n\
         import qualified Data.List as L (sortOn)\n\
         import qualified Data.List.NonEmpty as L (toList, fromList)\n\
         main :: IO ()\n\
         main = print (L.sortOn negate (L.toList (L.fromList [1, 2 :: Int])))\n",
    ),
    (
        // Listed, though the builtin interface omits `reverse`. GHC: 2.
        "ListedOmittedChirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty as NE (reverse, NonEmpty(..))\n\
         main :: IO ()\n\
         main = case NE.reverse (1 NE.:| [2 :: Int]) of\n\
         \x20 x NE.:| _ -> print x\n",
    ),
    (
        // A builtin class whose method list is partial leaves `S` open. GHC: [1,2].
        "PartialClassChirho.hs",
        "module Main where\n\
         import qualified Data.List.NonEmpty\n\
         import qualified Data.Semigroup as S (Semigroup(..))\n\
         main :: IO ()\n\
         main = print (S.sconcat (Data.List.NonEmpty.fromList [[1 :: Int], [2]]))\n",
    ),
    (
        // A builtin record type whose field list is partial leaves `Lazy` open.
        // GHC: ('x',(),[]).
        "PartialRecordChirho.hs",
        "module Main where\n\
         import qualified Control.Monad.Trans.RWS.Lazy as Lazy (RWST(..))\n\
         main :: IO ()\n\
         main = do\n\
         \x20 (a, s, w) <- Lazy.runRWST (return 'x') () ()\n\
         \x20 print (a, s, w :: [Int])\n",
    ),
    (
        // A listed operator. GHC: 6.
        "ListedOperatorChirho.hs",
        "module Main where\n\
         import qualified Data.Function as DF ((&))\n\
         main :: IO ()\n\
         main = print ((5 :: Int) DF.& (+ 1))\n",
    ),
    (
        // The implicit Prelude is open. GHC: [2,3].
        "ImplicitPreludeChirho.hs",
        "module Main where\n\
         main :: IO ()\n\
         main = Prelude.print (Prelude.map (+ 1) [1, 2 :: Int])\n",
    ),
    (
        // The module's own name qualifies its own top-level value. GHC: 7.
        "OwnNameChirho.hs",
        "module Main where\n\
         foo :: Int\n\
         foo = 7\n\
         main :: IO ()\n\
         main = print Main.foo\n",
    ),
    (
        // A `pattern` item lowering cannot represent leaves `Seq` open, so the pattern
        // synonym it imports stays usable. GHC: (2,True).
        "PatternItemChirho.hs",
        "{-# LANGUAGE PatternSynonyms #-}\n\
         module Main where\n\
         import qualified Data.Sequence as Seq (pattern Empty, fromList)\n\
         main :: IO ()\n\
         main = print (length (Seq.fromList [1, 2 :: Int]), null Seq.Empty)\n",
    ),
];

#[test]
fn an_explicit_list_supplies_only_what_it_lists_chirho() {
    for (file_name_chirho, source_chirho, expected_chirho) in REJECTED_CHIRHO {
        assert_eq!(
            naming_messages_chirho(source_chirho),
            *expected_chirho,
            "{file_name_chirho}: GHC rejects exactly these references"
        );
    }
}

#[test]
fn every_supplied_or_open_qualifier_still_resolves_chirho() {
    for (file_name_chirho, source_chirho) in ADMITTED_CHIRHO {
        let messages_chirho = naming_messages_chirho(source_chirho);
        assert!(
            messages_chirho.is_empty(),
            "{file_name_chirho}: GHC accepts it; name resolution reported {messages_chirho:?}"
        );
    }
}

#[test]
fn i2_stops_the_driver_at_name_resolution_chirho() {
    // Before row 536 this checked, and the frozen replay printed `:| 1 [2]`.
    let (file_name_chirho, source_chirho, _expected_chirho) = REJECTED_CHIRHO[0];
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let Err(err_chirho) =
        compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho)
    else {
        panic!("{file_name_chirho}: GHC rejects this program; we accepted it");
    };
    let rendered_chirho = err_chirho.to_string();
    assert!(
        rendered_chirho.contains("error[E0100]: variable not in scope: `NE.toList`"),
        "{file_name_chirho}: rejected, but not by the import list: {rendered_chirho}"
    );
}
