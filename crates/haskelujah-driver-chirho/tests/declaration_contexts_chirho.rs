// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A declaration's context reaches the dictionary pass whole. Before the
//! contexts were lowered with the type grammar, `(Show a, Pretty a) =>` kept
//! only its first member and `instance forall a. C a =>` kept nothing, so each
//! of these valid programs either was rejected ("could not deduce") or died at
//! run time ("missing STG binding"). Every expected output is GHC 9.14.1's own.
//!
//! Instance heads are lists on purpose: an instance at an applied data type
//! (`Box a`, `Maybe a`) does not dispatch at run time on main for a reason
//! unrelated to contexts (its dictionary and its methods are named from
//! different renderings of the head), so such a head could not tell this
//! repair from that defect.
//! workflow: language-features-chirho/flat-type-syntax-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn output_chirho(name_chirho: &str, source_chirho: &str) -> String {
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        name_chirho,
        None,
    )
    .unwrap_or_else(|err_chirho| panic!("{name_chirho} must run: {err_chirho}"));
    machine_chirho.io_output_chirho
}

#[test]
fn the_second_superclass_is_a_superclass_chirho() {
    let source_chirho = r##"module Main where
class Named a where
  name :: a -> String
class Sized a where
  size :: a -> Int
class (Named a, Sized a) => Shape a where
  area :: a -> Int
data Sq = Sq Int
instance Named Sq where
  name _ = "square"
instance Sized Sq where
  size (Sq n) = n
instance Shape Sq where
  area (Sq n) = n * n
data Rect = Rect Int Int
instance Named Rect where
  name _ = "rect"
instance Sized Rect where
  size (Rect w h) = w + h
instance Shape Rect where
  area (Rect w h) = w * h
report :: Shape a => a -> String
report x = name x ++ " " ++ show (size x) ++ " " ++ show (area x)
sizeOnly :: Shape a => a -> Int
sizeOnly x = size x
main :: IO ()
main = do
  putStrLn (report (Sq 4))
  putStrLn (report (Rect 2 5))
  print (sizeOnly (Rect 10 20))
"##;
    assert_eq!(
        output_chirho("TwoSuperclassesChirho.hs", source_chirho),
        "square 4 16\nrect 7 10\n30\n"
    );
}

#[test]
fn both_members_of_an_instance_context_carry_their_dictionaries_chirho() {
    let source_chirho = r##"module Main where
class Pretty a where
  pretty :: a -> String
instance Pretty Int where
  pretty n = "#" ++ show n
instance Pretty Bool where
  pretty b = if b then "yes" else "no"
class Describe a where
  describe :: a -> String
instance (Show a, Pretty a) => Describe [a] where
  describe xs = concatMap (\x -> show x ++ "/" ++ pretty x ++ ";") xs
main :: IO ()
main = do
  putStrLn (describe [3 :: Int, 4])
  putStrLn (describe [True, False])
"##;
    assert_eq!(
        output_chirho("TwoConstraintsChirho.hs", source_chirho),
        "3/#3;4/#4;\nTrue/yes;False/no;\n"
    );
}

#[test]
fn two_user_classes_in_one_context_are_both_passed_chirho() {
    // No built-in class here: nothing can stand in for a dictionary that the
    // lowering dropped.
    let source_chirho = r##"module Main where
class Pretty a where
  pretty :: a -> String
instance Pretty Int where
  pretty n = "#" ++ show n
class Loud a where
  loud :: a -> String
instance Loud Int where
  loud n = show n ++ "!"
class Describe a where
  describe :: a -> String
instance (Loud a, Pretty a) => Describe [a] where
  describe xs = concatMap pretty xs ++ concatMap loud xs
main :: IO ()
main = putStrLn (describe [3 :: Int, 4])
"##;
    assert_eq!(
        output_chirho("TwoUserClassesChirho.hs", source_chirho),
        "#3#43!4!\n"
    );
}

#[test]
fn an_explicit_instance_forall_keeps_the_context_chirho() {
    let source_chirho = r##"module Main where
class Pretty a where
  pretty :: a -> String
instance Pretty Int where
  pretty n = "#" ++ show n
class Tag a where
  tag :: a -> String
instance forall a. Pretty a => Tag [a] where
  tag xs = concatMap pretty xs
main :: IO ()
main = putStrLn (tag [1 :: Int, 2])
"##;
    assert_eq!(
        output_chirho("InstanceForallChirho.hs", source_chirho),
        "#1#2\n"
    );
}
