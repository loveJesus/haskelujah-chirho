// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Tuples, Maybe, Either, user data types, patterns, deriving and type synonyms (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_tuple_swap_chirho() {
    // Swap a tuple and extract first element (tests construction + pattern match)
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
swap p = case p of
  (a, b) -> (b, a)
fst p = case p of
  (a, b) -> a
main = fst (swap (1, 2))
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("tuple swap should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2),
        "fst (swap (1, 2)) should be 2"
    );
}

#[test]
fn eval_triple_chirho() {
    // 3-tuple construction and pattern match
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
third t = case t of
  (a, b, c) -> c
main = third (1, 2, 42)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("triple should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
        "third (1, 2, 42) should be 42"
    );
}

#[test]
fn eval_multi_constructor_data_chirho() {
    // Data type with multiple constructors and case dispatch
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Shape = Circle Int | Rectangle Int Int
area s = case s of
  Circle r -> r * r
  Rectangle w h -> w * h
main = area (Rectangle 3 4)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("multi-constructor data should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(12),
        "area (Rectangle 3 4) should be 12"
    );
}

#[test]
fn eval_maybe_just_chirho() {
    // Maybe with Just: fromMaybe 0 (Just 42) = 42
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
fromMaybe def m = case m of
  Nothing -> def
  Just x  -> x
main = fromMaybe 0 (Just 42)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("Maybe Just should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
        "fromMaybe 0 (Just 42) should be 42"
    );
}

#[test]
fn eval_maybe_nothing_chirho() {
    // Maybe with Nothing: fromMaybe 0 Nothing = 0
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
fromMaybe def m = case m of
  Nothing -> def
  Just x  -> x
main = fromMaybe 99 Nothing
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("Maybe Nothing should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(99),
        "fromMaybe 99 Nothing should be 99"
    );
}

#[test]
fn eval_either_chirho() {
    // Either type with Left/Right dispatch
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Either a b = Left a | Right b
fromRight def e = case e of
  Left _  -> def
  Right x -> x
main = fromRight 0 (Right 42)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("Either should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
        "fromRight 0 (Right 42) should be 42"
    );
}

#[test]
fn eval_binary_tree_chirho() {
    // Binary tree with recursive sum
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Tree = Leaf Int | Node Tree Tree
treeSum t = case t of
  Leaf n -> n
  Node l r -> treeSum l + treeSum r
main = treeSum (Node (Node (Leaf 1) (Leaf 2)) (Leaf 3))
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("binary tree should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6),
        "treeSum (Node (Node (Leaf 1) (Leaf 2)) (Leaf 3)) should be 6"
    );
}

// Rows 429 and 611: these five helper tests once declared their own `data Maybe` in module
// `Test` and applied the builtin helpers to it, a program GHC 9.14.1 rejects (GHC-87543). Each now
// uses the standard `Maybe` and imports the helper from Data.Maybe; GHC agrees on every value.
#[test]
fn eval_is_just_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             import Data.Maybe (isJust)\n\
             main = if isJust (Just 42) then 1 else 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => panic!("isJust should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_is_nothing_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             import Data.Maybe (isNothing)\n\
             main = if isNothing Nothing then 1 else 0\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => panic!("isNothing should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_from_maybe_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // fromMaybe 0 (Just 42) should give 42
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             import Data.Maybe (fromMaybe)\n\
             main = fromMaybe 0 (Just 42)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("fromMaybe Just should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_from_maybe_nothing_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // fromMaybe 0 Nothing should give 0
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             import Data.Maybe (fromMaybe)\n\
             main = fromMaybe 0 Nothing\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
            );
        }
        Err(e_chirho) => panic!("fromMaybe Nothing should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_maybe_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // maybe 0 (\x -> x + 1) (Just 41) should give 42
    let result_chirho = eval_source_chirho(
        "module Test where\n\
             inc x = x + 1\n\
             main = maybe 0 inc (Just 41)\n",
        &mut source_map_chirho,
        "TestChirho.hs",
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("maybe Just should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_user_data_case_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Use a single-equation case-based function to avoid exhaustiveness per-equation issue
    let src_chirho = "\
module Test where
data Color = Red | Green | Blue
showColor c = case c of
  Red -> 1
  Green -> 2
  Blue -> 3
main = showColor Red
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => panic!("user data case should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_bool_and_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if True && False then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("&& should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
        "True && False → 0"
    );
}

#[test]
fn eval_bool_and_true_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if True && True then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("&& true should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
        "True && True → 1"
    );
}

#[test]
fn eval_bool_or_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if False || True then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("|| should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1),
        "False || True → 1"
    );
}

#[test]
fn eval_bool_or_false_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if False || False then 1 else 0
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("|| false should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0),
        "False || False → 0"
    );
}

#[test]
fn eval_derived_ord_chirho() {
    use crate::eval_source_chirho;
    // compare Red Green = LT for nullary enum with derived Ord
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Color = Red | Green | Blue deriving (Eq, Ord)
main = case compare Red Blue of
  LT -> 1
  EQ -> 0
  GT -> 2
";
    let r1_chirho = eval_source_chirho(src_chirho, &mut sm1_chirho, "TestChirho.hs", None)
        .expect("derived Ord compare should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_maybe_bind_chirho() {
    use crate::eval_source_chirho;
    // Just 10 >>= \x -> Just (x * 2) = Just 20
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
safeDivide a b = if b == 0 then Nothing else Just (a `div` b)
main = case safeDivide 20 2 of
  Just y -> y
  Nothing -> 0
";
    let r1_chirho = eval_source_chirho(src_chirho, &mut sm1_chirho, "TestChirho.hs", None)
        .expect("Maybe safeDivide should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

#[test]
fn eval_neg_lit_pattern_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // case (-1) of { (-1) -> 10; _ -> 20 } should be 10
    let result_chirho = eval_source_chirho(
        "module Test where\nmain = case (-1) of\n  (-1) -> 10\n  _ -> 20\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
            );
        }
        Err(e_chirho) => panic!("negative literal pattern should work: {}", e_chirho),
    }
}

#[test]
fn eval_string_pattern_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // case "hello" of { "hello" -> "yes"; _ -> "no" } → "yes"
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn (case \"hello\" of\n  \"hello\" -> \"yes\"\n  _ -> \"no\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "yes\n");
        }
        Err(e_chirho) => panic!("string pattern should work: {}", e_chirho),
    }
}

#[test]
fn eval_as_pattern_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // f xs@(x:_) = x + length xs; f [] = 0
    // main = f [10, 20, 30]
    let result_chirho = eval_source_chirho(
        "module Test where\nf xs@(x:_) = x + length xs\nf [] = 0\nmain = f [10, 20, 30]\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(13)
            );
        }
        Err(e_chirho) => panic!("as-pattern should work: {}", e_chirho),
    }
}

#[test]
fn eval_char_pattern_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // case 'a' of { 'a' -> "yes"; _ -> "no" }
    let result_chirho = eval_source_with_machine_chirho(
        "module Test where\nmain = putStrLn (case 'a' of\n  'a' -> \"yes\"\n  _ -> \"no\")\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "yes\n");
        }
        Err(e_chirho) => panic!("char pattern should work: {}", e_chirho),
    }
}

#[test]
fn eval_type_synonym_string_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Test that user-defined type synonym `String` annotation unifies with [Char]
    let result_chirho = eval_source_chirho(
        "module Test where\ngreet :: String -> Int\ngreet s = length s\nmain = greet \"hello\"\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
            );
        }
        Err(e_chirho) => panic!("type synonym String should work: {}", e_chirho),
    }
}

#[test]
fn eval_type_synonym_user_defined_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // User-defined type synonym: type MyInt = Int
    let result_chirho = eval_source_chirho(
        "module Test where\ntype MyInt = Int\naddOne :: MyInt -> MyInt\naddOne x = x + 1\nmain = addOne 41\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("user type synonym should work: {}", e_chirho),
    }
}

#[test]
fn eval_type_synonym_chained_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Chained synonyms: type MyList = [Int], type FilePath = String
    let result_chirho = eval_source_chirho(
        "module Test where\ntype FilePath = String\npathLen :: FilePath -> Int\npathLen p = length p\nmain = pathLen \"/usr/bin\"\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(8)
            );
        }
        Err(e_chirho) => panic!("chained type synonym should work: {}", e_chirho),
    }
}

#[test]
fn eval_data_with_synonym_field_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Data type using a type synonym in its field
    let result_chirho = eval_source_chirho(
        "module Test where\ntype Name = String\ndata Person = MkPerson Name Int\ngetName (MkPerson n _) = n\nmain = length (getName (MkPerson \"Alice\" 30))\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
            );
        }
        Err(e_chirho) => panic!("data with synonym field should work: {}", e_chirho),
    }
}

#[test]
fn eval_catmaybes_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // catMaybes filters out Nothing and unwraps Just values
    let result_chirho = eval_source_chirho(
        "module Test where\ncatMaybes xs = case xs of\n  [] -> []\n  (y:ys) -> case y of\n    Nothing -> catMaybes ys\n    Just v -> v : catMaybes ys\nmain = sum (catMaybes [Just 1, Nothing, Just 3, Nothing, Just 5])\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(9)
        ),
        Err(e_chirho) => panic!("catMaybes should work: {}", e_chirho),
    }
}

// ── Nested pattern matching in case ─────────────────────────────

#[test]
fn eval_case_nested_tuple_chirho() {
    // case (1, 2) of { (a, b) -> a + b }
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = case (1, 2) of { (a, b) -> a + b }\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("case nested tuple: {}", e_chirho),
    }
}

// ── Data constructor as function ────────────────────────────────

#[test]
fn eval_map_just_chirho() {
    // map Just [1,2,3] → [Just 1, Just 2, Just 3], length → 3
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = length (map Just [1,2,3])\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("map Just: {}", e_chirho),
    }
}

// ── Nested data types ───────────────────────────────────────────

#[test]
fn eval_nested_maybe_case_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
safe_head xs = case xs of
  [] -> Nothing
  (x:_) -> Just x
main = case safe_head [42, 1, 2] of
  Nothing -> 0
  Just x -> x
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
        ),
        Err(e_chirho) => panic!("nested maybe case: {}", e_chirho),
    }
}

#[test]
fn eval_data_maybe_chain_chirho() {
    // safeDivide with guard instead of literal pattern match
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Data.Maybe (fromMaybe)\nsafeDivide x y\n  | y == 0 = Nothing\n  | otherwise = Just (x `div` y)\nmain = fromMaybe 0 (safeDivide 10 3) + fromMaybe 0 (safeDivide 10 0)\n";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
        ),
        Err(e_chirho) => panic!("data maybe chain: {}", e_chirho),
    }
}

#[test]
fn eval_first_tuple_chirho() {
    // first (+10) (5, 99) → (15, 99) → fst → 15
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Control.Arrow (first)\nadd10 x = x + 10\nmain = fst (first add10 (5, 99))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15)
    );
}

#[test]
fn eval_second_tuple_chirho() {
    // second (*2) (10, 7) → (10, 14) → snd → 14
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nimport Control.Arrow (second)\ndbl x = x * 2\nmain = snd (second dbl (10, 7))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(14)
    );
}

#[test]
fn eval_both_tuple_chirho() {
    // both (+1) (10, 20) → (11, 21) → fst + snd → 32
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho =
        "module Test where\ninc x = x + 1\nmain = case both inc (10, 20) of (a, b) -> a + b\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(32)
    );
}
