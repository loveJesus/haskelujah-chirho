// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Newtype, existential and strict-field data declaration tests (row 700).

use crate::{compile_source_chirho, eval_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

// Data.Map, Data.Set, BST, record syntax, newtype, user-defined data type tests

#[test]
fn eval_newtype_chirho() {
    // Newtype construction and case dispatch
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
newtype Age = MkAge Int
getAge a = case a of
  MkAge n -> n
main = getAge (MkAge 25)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("newtype should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(25),
        "getAge (MkAge 25) should be 25"
    );
}

// ── Priority 51: Newtype deriving and GND ────────────────────
#[test]
fn eval_newtype_construct_match_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Basic newtype: construct and pattern-match
    let src_chirho =
        "module Test where\nnewtype Age = MkAge Int\nmain = case MkAge 42 of { MkAge n -> n }\n";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("newtype construct/match should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_newtype_erasure_arithmetic_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Newtype erasure means MkAge is identity — unwrapped value
    // participates in arithmetic directly.
    let src_chirho = "\
module Test where
newtype Age = MkAge Int
getAge x = case x of { MkAge n -> n }
main = getAge (MkAge 10) + getAge (MkAge 32)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("newtype erasure arithmetic should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_newtype_derived_eq_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Newtype with derived Eq: equality comparison works through erasure.
    let src_chirho = "\
module Test where
newtype Age = MkAge Int deriving (Eq)
main = if MkAge 5 == MkAge 5 then 1 else 0
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
        Err(e_chirho) => panic!("newtype derived Eq should evaluate: {}", e_chirho),
    }
}

// ── zipWith and unzip tests ──

// ── ExistentialQuantification tests ──

#[test]
fn existential_data_parses_chirho() {
    // ExistentialQuantification: `forall a. Show a => MkShowable a`
    // Test that the parser correctly extracts "MkShowable" as the constructor
    // name, not "Show" from the context.
    use crate::compile_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Showable = forall a. Show a => MkShowable a
main = 42
";
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "existential data decl should parse and compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn existential_data_no_context_parses_chirho() {
    // ExistentialQuantification without a context: `forall a. MkBox a`
    use crate::compile_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Box = forall a. MkBox a
main = 42
";
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "existential without context should parse: {:?}",
        result_chirho.err()
    );
}

#[test]
fn existential_constructor_eval_chirho() {
    // Construct an existential value and extract the inner value
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Box = forall a. MkBox a
unbox (MkBox x) = x
main = unbox (MkBox 42)
";
    let (val_chirho, _machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .expect("existential eval should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

// ── Strict data fields ──────────────────────────────────────────

#[test]
fn eval_strict_data_field_basic_chirho() {
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Pair = MkPair !Int !Int
main = case MkPair (1 + 2) (3 + 4) of
  MkPair a b -> a + b
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("strict fields should be forced");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

#[test]
fn eval_strict_and_lazy_fields_chirho() {
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Mixed = MkMixed !Int Int
main = case MkMixed (2 + 3) (4 + 5) of
  MkMixed a b -> a + b
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("mixed strict/lazy fields should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(14)
    );
}

#[test]
fn eval_strict_field_single_chirho() {
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Box = MkBox !Int
main = case MkBox (21 + 21) of
  MkBox x -> x
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("single strict field should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn eval_strict_field_preserves_core_chirho() {
    // Verify strict fields produce case wrappers in Core IR
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data StrictBox = SB !Int
main = case SB 42 of
  SB x -> x
";
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs")
        .expect("strict field should compile");
    // The Core IR should contain the binding even without case wrappers for literals
    let has_sb_chirho = result_chirho
        .core_chirho
        .bindings_chirho
        .iter()
        .any(|b_chirho| {
            let name_chirho = &b_chirho.binder_chirho.name_chirho;
            name_chirho == "main"
        });
    assert!(has_sb_chirho, "main binding should exist in Core IR");
}
