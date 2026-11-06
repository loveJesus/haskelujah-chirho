// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing extension controls, grouped by topic.

use crate::{compile_source_chirho, eval_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn builtin_real_to_frac_chirho() {
    // realToFrac should type-check without error (type scheme present)
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nimport Prelude (realToFrac)\nf x = realToFrac x\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "realToFrac.hs");
    assert!(
        result_chirho.is_ok(),
        "realToFrac should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn builtin_from_integral_chirho() {
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nimport Prelude (fromIntegral)\nf x = fromIntegral x\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "fromIntegral.hs");
    assert!(
        result_chirho.is_ok(),
        "fromIntegral should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn builtin_div_mod_chirho() {
    let src_chirho = "module Test where\nmain = div 10 3\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "divmod.hs", None);
    assert!(
        result_chirho.is_ok(),
        "div should eval: {:?}",
        result_chirho.err()
    );
}

#[test]
fn builtin_error_without_stack_trace_chirho() {
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nimport Prelude (errorWithoutStackTrace)\nf = errorWithoutStackTrace \"oops\"\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "errWST.hs");
    assert!(
        result_chirho.is_ok(),
        "errorWithoutStackTrace should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn parametric_show_list_bool_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
main = putStrLn (show [True, False, True])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ShowListBool.hs", None)
            .unwrap_or_else(|e_chirho| panic!("Show [Bool] should work: {}", e_chirho));
    assert!(
        m_chirho.io_output_chirho.contains("[True"),
        "expected list show output, got: {}",
        m_chirho.io_output_chirho
    );
}

#[test]
fn parametric_eq_maybe_chirho() {
    let src_chirho = r#"module Test where
main = print (Just 1 == Just 1)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "EqMaybe.hs");
    assert!(
        result_chirho.is_ok(),
        "Eq (Maybe Int) should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn parametric_show_maybe_chirho() {
    let src_chirho = r#"module Test where
main = putStrLn (show (Just 42))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ShowMaybe.hs");
    assert!(
        result_chirho.is_ok(),
        "Show (Maybe Int) should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn parametric_show_tuple_chirho() {
    let src_chirho = r#"module Test where
main = putStrLn (show (1, True))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ShowTuple.hs");
    assert!(
        result_chirho.is_ok(),
        "Show (Int, Bool) should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn parametric_eq_list_chirho() {
    let src_chirho = r#"module Test where
main = print ([1,2,3] == [1,2,3])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "EqList.hs");
    assert!(
        result_chirho.is_ok(),
        "Eq [Int] should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn parametric_ord_list_chirho() {
    let src_chirho = r#"module Test where
main = print (compare [1] [2])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "OrdList.hs");
    assert!(
        result_chirho.is_ok(),
        "Ord [Int] should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn parametric_eq_triple_chirho() {
    let src_chirho = r#"module Test where
main = print ((1, True, 'a') == (1, True, 'a'))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "EqTriple.hs");
    assert!(
        result_chirho.is_ok(),
        "Eq (Int, Bool, Char) should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_as_type_of_chirho() {
    let src_chirho = r#"
module PreludeAsTypeOf where

val :: Int
val = 42 `asTypeOf` (undefined :: Int)

main :: IO ()
main = print val
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PreludeAsTypeOf.hs", None);
    assert!(
        result_chirho.is_ok(),
        "asTypeOf failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_math_functions_chirho() {
    let src_chirho = r#"
module PreludeMath where

main :: IO ()
main = do
  print (sqrt 4.0)
  print (divMod 17 5)
  print (splitAt 2 [1,2,3,4])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PreludeMath.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Prelude math failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_list_expanded_chirho() {
    let src_chirho = r#"
module PreludeListExp where

main :: IO ()
main = do
  print (splitAt 3 [1,2,3,4,5])
  print (break (> 3) [1,2,3,4,5])
  print (cycle [1,2,3] !! 7)
  print (replicate 3 'x')
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PreludeListExp.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Prelude list expanded failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn real_float_monadio_instances_chirho() {
    let src_chirho = r#"
module RfMioChirho where

foo :: RealFloat a => a -> Bool
foo x = isNaN x || isInfinite x

bar :: Double -> Bool
bar = foo
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RfMio.hs");
    assert!(
        result_chirho.is_ok(),
        "RealFloat constraint failed: {:?}",
        result_chirho.err()
    );
}
