// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC-verified first-class Prelude composition, distinct from infix syntax sugar.
//! Workflow: language-features-chirho/dictionary-evidence-chirho.
use super::assert_execution_chirho;

#[test]
fn first_class_composition_executes_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
main = print ((.) (+2) (*2) (20 :: Int))
"###;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn first_class_composition_does_not_force_unused_function_or_value_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
main = print ((.) (const (42 :: Int)) (error "unused function chirho") (error "unused argument chirho"))
"###;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn composition_alias_executes_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
composeChirho = (.)
main = print (composeChirho (+2) (*2) (20 :: Int))
"###;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn source_composition_body_is_not_replaced_by_prelude_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import Prelude hiding ((.))
(.) :: (Int -> Int) -> (Int -> Int) -> Int -> Int
(.) firstChirho secondChirho valueChirho = firstChirho (secondChirho valueChirho) + 1
main = print ((.) (+2) (*2) (20 :: Int))
"###;
    assert_execution_chirho(source_chirho, "43\n");
}
