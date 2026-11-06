// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Enum and Bounded methods and arithmetic sequences (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn eval_arith_sequence_from_to_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1..3]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("arith sequence should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6),
        "sum [1..3] should be 6"
    );
}

#[test]
fn eval_arith_sequence_sum_1_to_5_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum [1..5]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("arith sequence [1..5] should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15),
        "sum [1..5] should be 15"
    );
}

#[test]
fn eval_arith_sequence_length_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
len xs = case xs of
  [] -> 0
  (x:rest) -> 1 + len rest
main = len [10..15]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("length of [10..15] should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6),
        "len [10..15] should be 6"
    );
}

#[test]
fn eval_map_with_arith_sequence_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
myMap f xs = case xs of
  [] -> []
  (x:rest) -> f x : myMap f rest
sum xs = case xs of
  [] -> 0
  (x:rest) -> x + sum rest
main = sum (myMap (\\x -> x + 10) [1..3])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("map with lambda over arith sequence should evaluate");
    // [1,2,3] mapped with (+10) gives [11,12,13], sum = 36
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(36),
        "sum (map (+10) [1..3]) should be 36"
    );
}

// ---------------------------------------------------------------
// Priority 65: Arithmetic sequences with step
// ---------------------------------------------------------------

#[test]
fn eval_enum_from_then_to_chirho() {
    // [1,3..10] should produce [1,3,5,7,9]
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum [1, 3 .. 10]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("enumFromThenTo should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(25),
        "sum [1,3..10] = 1+3+5+7+9 = 25"
    );
}

#[test]
fn eval_enum_from_then_to_down_chirho() {
    // [10,8..1] should produce [10,8,6,4,2]
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum [10, 8 .. 1]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("enumFromThenTo descending should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(30),
        "sum [10,8..1] = 10+8+6+4+2 = 30"
    );
}

#[test]
fn eval_enum_from_then_to_length_chirho() {
    // [2,5..20] → [2,5,8,11,14,17,20] has length 7
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = length [2, 5 .. 20]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("enumFromThenTo length should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7),
        "length [2,5..20] = 7"
    );
}

#[test]
fn eval_enum_from_chirho() {
    // head [10..] should be 10
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
x = 10
main = head [x ..]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("enumFrom head should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10),
        "head [10..] = 10"
    );
}

#[test]
fn eval_take_from_enum_chirho() {
    // take 5 [1..100] → [1,2,3,4,5], sum = 15
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = sum (take 5 [1..100])
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("take on enum range should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(15),
        "sum (take 5 [1..100]) = 15"
    );
}

#[test]
fn eval_succ_pred_chirho() {
    use crate::eval_source_chirho;
    // succ 5 = 6
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = succ 5\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("succ should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
    );

    // pred 10 = 9
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = pred 10\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("pred should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(9)
    );

    // succ (pred 7) = 7
    let mut sm3_chirho = SourceMapChirho::new_chirho();
    let r3_chirho = eval_source_chirho(
        "module Test where\nmain = succ (pred 7)\n",
        &mut sm3_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("succ pred should evaluate");
    assert_eq!(
        r3_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
    );
}

#[test]
fn eval_to_from_enum_chirho() {
    use crate::eval_source_chirho;
    // toEnum 42 :: Int = 42
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = toEnum 42\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("toEnum should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );

    // fromEnum 99 :: Int = 99
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
        "module Test where\nmain = fromEnum 99\n",
        &mut sm2_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("fromEnum should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
    );

    // fromEnum (99 :: Integer) = 99 in the current runtime representation
    let mut sm3_chirho = SourceMapChirho::new_chirho();
    let r3_chirho = eval_source_chirho(
        "module Test where\nmain = fromEnum (99 :: Integer)\n",
        &mut sm3_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("fromEnum Integer should evaluate");
    assert_eq!(
        r3_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
    );

    let mut sm4_chirho = SourceMapChirho::new_chirho();
    let r4_chirho = crate::eval_source_with_machine_chirho(
        "module Test where\nimport Data.Char (chr)\nmain = print (fromEnum (chr 97))\n",
        &mut sm4_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("fromEnum . chr should print the ordinal");
    assert_eq!(r4_chirho.1.io_output_chirho, "97\n");
}

#[test]
fn eval_bounded_int_chirho() {
    use crate::eval_source_chirho;
    // maxBound > minBound for Int
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
        "module Test where\nmain = if maxBound > minBound then 1 else 0\n",
        &mut sm1_chirho,
        "TestChirho.hs",
        None,
    )
    .expect("bounded comparison should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_derived_enum_chirho() {
    use crate::eval_source_chirho;
    // fromEnum Green = 1
    let mut sm1_chirho = SourceMapChirho::new_chirho();
    let r1_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Green\n",
            &mut sm1_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum should evaluate");
    assert_eq!(
        r1_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );

    // fromEnum Blue = 2
    let mut sm2_chirho = SourceMapChirho::new_chirho();
    let r2_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Blue\n",
            &mut sm2_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum Blue should evaluate");
    assert_eq!(
        r2_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );

    // fromEnum Red + fromEnum Blue = 0 + 2 = 2
    let mut sm3_chirho = SourceMapChirho::new_chirho();
    let r3_chirho = eval_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue deriving (Enum)\nmain = fromEnum Red + fromEnum Blue\n",
            &mut sm3_chirho, "TestChirho.hs", None,
        ).expect("derived Enum fromEnum arithmetic should evaluate");
    assert_eq!(
        r3_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
    );
}
