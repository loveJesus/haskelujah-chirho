// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Annotated pattern bindings retain their binder and check the RHS against the annotation
//! (row 675). Function-argument and lambda annotations retain their existing route.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::{
    assert_compile_success_chirho, assert_prints_everywhere_chirho, rejected_like_ghc_chirho,
};

const EXTENSION_BINDING_CHIRHO: &str = r#"module Main where
valueChirho :: Int = 7
main :: IO ()
main = print valueChirho
"#;

const EXTENSION_CASES_CHIRHO: [(&str, &str, bool); 7] = [
    ("DefaultAnnotationChirho.hs", "", true),
    (
        "LegacyAnnotationChirho.hs",
        "{-# LANGUAGE Haskell2010 #-}\n",
        false,
    ),
    (
        "DisabledAnnotationChirho.hs",
        "{-# LANGUAGE GHC2021, NoScopedTypeVariables #-}\n",
        false,
    ),
    (
        "EnabledLegacyAnnotationChirho.hs",
        "{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}\n",
        true,
    ),
    (
        "ModernAnnotationChirho.hs",
        "{-# LANGUAGE GHC2021 #-}\n",
        true,
    ),
    (
        "AliasAnnotationChirho.hs",
        "{-# LANGUAGE Haskell2010, PatternSignatures #-}\n",
        true,
    ),
    (
        "DisabledAliasAnnotationChirho.hs",
        "{-# LANGUAGE GHC2021, NoPatternSignatures #-}\n",
        false,
    ),
];

#[test]
fn a_binding_signature_requires_its_language_extension_chirho() {
    for (file_chirho, pragmas_chirho, enabled_chirho) in EXTENSION_CASES_CHIRHO {
        let source_chirho = format!("{pragmas_chirho}{EXTENSION_BINDING_CHIRHO}");
        if enabled_chirho {
            assert_compile_success_chirho(file_chirho, &source_chirho);
        } else {
            rejected_like_ghc_chirho(&source_chirho, file_chirho, &["ScopedTypeVariables"]);
        }
    }
}

const ANNOTATED_BINDINGS_CHIRHO: &str = r#"{-# LANGUAGE ScopedTypeVariables #-}
module Main where
valueChirho :: Int = 7
(leftChirho, rightChirho) :: (Int, Int) = (2, 3)
[itemChirho] :: [Int] = [5]
nextChirho :: Int -> Int
nextChirho valueChirho = localChirho
  where localChirho :: Int = valueChirho + 1
main :: IO ()
main = do
  print valueChirho
  print (leftChirho + rightChirho)
  print itemChirho
  print (nextChirho 10)
"#;

const SCOPED_BINDING_CHIRHO: &str = r#"{-# LANGUAGE ScopedTypeVariables #-}
module Main where
copyChirho :: forall a. a -> a
copyChirho valueChirho = localChirho
  where localChirho :: a = valueChirho
main :: IO ()
main = do
  print (copyChirho True)
  print (copyChirho (7 :: Int))
"#;

const UNANNOTATED_BINDINGS_CHIRHO: &str = r#"module Main where
valueChirho :: Int
valueChirho = 7
(leftChirho, rightChirho) = (2 :: Int, 3 :: Int)
[itemChirho] = [5 :: Int]
main :: IO ()
main = do
  print valueChirho
  print (leftChirho + rightChirho)
  print itemChirho
"#;

const VIEW_BINDING_CHIRHO: &str = r#"{-# LANGUAGE ScopedTypeVariables, ViewPatterns #-}
module Main where
((\(inputChirho :: Int) -> inputChirho + 1) -> viewedChirho) :: Int = 4
main :: IO ()
main = print viewedChirho
"#;

#[test]
fn annotated_pattern_bindings_keep_their_values_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "AnnotatedBindingsChirho.hs",
        ANNOTATED_BINDINGS_CHIRHO,
        "7\n5\n5\n11\n",
    );
}

#[test]
fn a_binding_annotation_uses_the_enclosing_signature_variable_chirho() {
    assert_prints_everywhere_chirho("ScopedBindingChirho.hs", SCOPED_BINDING_CHIRHO, "True\n7\n");
}

#[test]
fn unannotated_pattern_bindings_keep_their_values_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "UnannotatedBindingsChirho.hs",
        UNANNOTATED_BINDINGS_CHIRHO,
        "7\n5\n5\n",
    );
}

#[test]
fn a_view_expression_keeps_ordinary_argument_inference_chirho() {
    assert_prints_everywhere_chirho("ViewBindingChirho.hs", VIEW_BINDING_CHIRHO, "5\n");
}

#[test]
fn tc135_keeps_its_existential_where_binding_chirho() {
    assert_compile_success_chirho(
        "tc135.hs",
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc135.hs"),
    );
}

#[test]
fn binding_annotations_reject_conflicting_rhs_types_chirho() {
    for (file_chirho, source_chirho) in [
        (
            "BadVariableAnnotationChirho.hs",
            "{-# LANGUAGE ScopedTypeVariables #-}\nmodule BadVariableAnnotationChirho where\nvalueChirho :: Bool = 3\n",
        ),
        (
            "BadTupleAnnotationChirho.hs",
            "{-# LANGUAGE ScopedTypeVariables #-}\nmodule BadTupleAnnotationChirho where\n(leftChirho, rightChirho) :: (Int, Bool) = (1, 'a')\n",
        ),
        (
            "BadListAnnotationChirho.hs",
            "{-# LANGUAGE ScopedTypeVariables #-}\nmodule BadListAnnotationChirho where\n[itemChirho] :: [Bool] = [3]\n",
        ),
        (
            "BadNestedAnnotationChirho.hs",
            "{-# LANGUAGE ScopedTypeVariables #-}\nmodule BadNestedAnnotationChirho where\n((leftChirho :: Bool), rightChirho) :: (Int, Bool) = (3, True)\n",
        ),
        (
            "BadLocalAnnotationChirho.hs",
            "{-# LANGUAGE ScopedTypeVariables #-}\nmodule BadLocalAnnotationChirho where\nvalueChirho = localChirho\n  where localChirho :: Char = True\n",
        ),
        (
            "BadScopedAnnotationChirho.hs",
            "{-# LANGUAGE ScopedTypeVariables #-}\nmodule BadScopedAnnotationChirho where\nvalueChirho :: forall a. a -> a\nvalueChirho inputChirho = localChirho\n  where localChirho :: a = True\n",
        ),
    ] {
        rejected_like_ghc_chirho(source_chirho, file_chirho, &[]);
    }
}
