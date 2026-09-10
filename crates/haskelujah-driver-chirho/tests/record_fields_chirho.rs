// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Record construction and update go by the constructor's field set, never
//! its argument arity (GHC's rules): an undeclared field is an error, an
//! omitted lazy field is a named thunk forced only on use, an omitted strict
//! field is an error. A `check`-only gate cannot see the run-time half of
//! this (trap 17); the programs here read fields back.
//! workflow: language-features-chirho/rigid-type-variables-chirho (records)

use haskelujah_driver::{compile_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;

// GHC 9.14.1 prints (3,'x') then 7. Both record construction and the
// type-changing update must check their lambda against the rank-n field.
const RANK_RECORD_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE RankNTypes #-}
module Main where
data RChirho bChirho = MkRChirho
  { fChirho :: (forall aChirho. aChirho -> aChirho) -> (Int, bChirho)
  , cChirho :: Int
  }
changeChirho :: RChirho Bool -> RChirho Char
changeChirho rChirho = rChirho { fChirho = \kChirho -> (kChirho 3, kChirho 'x') }
main :: IO ()
main = do
  let beforeChirho = MkRChirho { fChirho = \kChirho -> (kChirho 1, kChirho True), cChirho = 7 }
  let afterChirho = changeChirho beforeChirho
  print (fChirho afterChirho id)
  print (cChirho afterChirho)
"#;

#[test]
fn rank_n_record_fields_are_checked_before_their_lambdas_are_inferred_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        RANK_RECORD_SOURCE_CHIRHO,
        &mut source_map_chirho,
        "RankRecordChirho.hs",
        None,
    )
    .expect("GHC accepts the independent Int and Char instantiations of the field argument");
    assert_eq!(machine_chirho.io_output_chirho, "(3,'x')\n7\n");
}

#[test]
fn rank_n_record_update_still_rejects_a_wrong_field_result_chirho() {
    let source_chirho = RANK_RECORD_SOURCE_CHIRHO
        .replace("(kChirho 3, kChirho 'x')", "(kChirho True, kChirho 'x')");
    let error_chirho = check_chirho("BadRankRecordChirho.hs", &source_chirho)
        .expect_err("a polymorphic argument does not permit Bool in the field's Int result");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn update_on_a_field_no_constructor_declares_is_rejected_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module Main where\ndata P = P { px :: Int, py :: Int }\nmain :: IO ()\nmain = do\n  let p = P { px = 1, py = 2 }\n  print (px (p { pz_chirho = 9 }))\n",
        &mut source_map_chirho,
        "BogusFieldChirho.hs",
    );
    assert!(
        result_chirho.is_err(),
        "GHC rejects an update on a field no constructor of `P` declares; we accepted it"
    );
}

#[test]
fn update_on_a_declared_field_runs_and_reads_back_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Main where\ndata P = P { px :: Int, py :: Int }\nmain :: IO ()\nmain = do\n  let p = P { px = 1, py = 2 }\n  print (px (p { px = 9 }))\n  print (py (p { px = 9 }))\n",
        &mut source_map_chirho,
        "RealFieldChirho.hs",
        None,
    )
    .expect("the control program runs");
    assert_eq!(machine_chirho.io_output_chirho, "9\n2\n");
}

fn check_chirho(file_name_chirho: &str, source_chirho: &str) -> Result<(), String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho)
        .map(|_| ())
        .map_err(|err_chirho| format!("{err_chirho}"))
}

#[test]
fn omitted_lazy_field_is_allowed_and_not_forced_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Main where\ndata P = P { px :: Int, py :: Int }\np :: P\np = P { px = 1 }\nmain :: IO ()\nmain = print (px p)\n",
        &mut source_map_chirho,
        "PartialLazyChirho.hs",
        None,
    )
    .expect("an omitted lazy field is legal Haskell and must not be forced by a read of another field");
    assert_eq!(machine_chirho.io_output_chirho, "1\n");
}

#[test]
fn forcing_an_omitted_lazy_field_names_it_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_machine_chirho(
        "module Main where\ndata P = P { px :: Int, py :: Int }\np :: P\np = P { px = 1 }\nmain :: IO ()\nmain = print (py p)\n",
        &mut source_map_chirho,
        "PartialForcedChirho.hs",
        None,
    );
    let message_chirho = match result_chirho {
        Err(err_chirho) => format!("{err_chirho}"),
        Ok((_value_chirho, machine_chirho)) => panic!(
            "forcing an omitted field must fail; it printed {:?}",
            machine_chirho.io_output_chirho
        ),
    };
    assert!(
        message_chirho.contains("py"),
        "the missing-field error must name the field: {message_chirho}"
    );
}

#[test]
fn empty_braces_on_a_lazy_field_construct_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Main where\ndata T = MkT { f :: Int }\nt :: T\nt = MkT {}\nmain :: IO ()\nmain = case t of\n  MkT {} -> putStrLn \"built\"\n",
        &mut source_map_chirho,
        "EmptyBracesChirho.hs",
        None,
    )
    .expect("`MkT {}` is a record construction with no fields, legal for a lazy field");
    assert_eq!(machine_chirho.io_output_chirho, "built\n");
}

#[test]
fn omitted_strict_field_is_rejected_chirho() {
    let result_chirho = check_chirho(
        "StrictOmittedChirho.hs",
        "module Main where\ndata T = MkT { f :: !Int }\nt :: T\nt = MkT {}\nmain :: IO ()\nmain = putStrLn \"never\"\n",
    );
    match result_chirho {
        Err(err_chirho) => assert!(
            err_chirho.contains("required strict field"),
            "GHC-95909 names the strict field: {err_chirho}"
        ),
        Ok(()) => panic!("GHC rejects a construction that omits a strict field; we accepted it"),
    }
}

#[test]
fn undeclared_field_in_a_construction_is_rejected_chirho() {
    assert!(
        check_chirho(
            "BogusConFieldChirho.hs",
            "module Main where\ndata P = P { px :: Int, py :: Int }\np :: P\np = P { px = 1, pq_chirho = 2 }\nmain :: IO ()\nmain = print (px p)\n",
        )
        .is_err(),
        "GHC rejects a construction naming a field the constructor does not have; we accepted it"
    );
}

#[test]
fn empty_braces_on_a_positional_constructor_construct_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Main where\ndata T = MkT Int\nmain :: IO ()\nmain = case MkT {} of\n  MkT _ -> putStrLn \"built\"\n",
        &mut source_map_chirho,
        "EmptyBracesPositionalChirho.hs",
        None,
    )
    .expect("`MkT {}` is legal for any constructor; its arguments are missing fields");
    assert_eq!(machine_chirho.io_output_chirho, "built\n");
}

#[test]
fn named_field_on_a_positional_constructor_is_rejected_chirho() {
    assert!(
        check_chirho(
            "PositionalFieldChirho.hs",
            "module Main where\ndata T = MkT Int\nt :: T\nt = MkT { tx_chirho = 1 }\nmain :: IO ()\nmain = putStrLn \"never\"\n",
        )
        .is_err(),
        "GHC rejects a named field on a constructor that declares none; we accepted it"
    );
}

#[test]
fn empty_braces_on_a_positional_constructor_with_a_strict_field_is_rejected_chirho() {
    let result_chirho = check_chirho(
        "StrictPositionalChirho.hs",
        "module Main where\ndata T = T Int !Int\nt :: T\nt = T {}\nmain :: IO ()\nmain = putStrLn \"never\"\n",
    );
    match result_chirho {
        Err(err_chirho) => assert!(
            err_chirho.contains("required strict field"),
            "tcfail112: `T {{}}` omits a strict argument: {err_chirho}"
        ),
        Ok(()) => {
            panic!("GHC rejects `T {{}}` when T has a strict argument (tcfail112); we accepted it")
        }
    }
}

#[test]
fn giving_the_strict_field_and_omitting_the_lazy_one_is_legal_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        "module Main where\ndata S = S { x :: Int, y :: !Int }\ns :: S\ns = S { y = 3 }\nmain :: IO ()\nmain = print (y s)\n",
        &mut source_map_chirho,
        "StrictGivenLazyOmittedChirho.hs",
        None,
    )
    .expect("tcfail112's `s3 = S { y=3 }` is legal: the omitted field is the lazy one");
    assert_eq!(machine_chirho.io_output_chirho, "3\n");
}

#[test]
fn construction_with_a_constructor_that_does_not_exist_is_rejected_chirho() {
    let result_chirho = check_chirho(
        "NoSuchConstructorChirho.hs",
        "module Main where\ng :: Int\ng = Int{}\nmain :: IO ()\nmain = print g\n",
    );
    match result_chirho {
        Err(err_chirho) => assert!(
            err_chirho.contains("unbound constructor"),
            "T23739c: `Int` is a type, not a data constructor: {err_chirho}"
        ),
        Ok(()) => panic!("GHC rejects `Int{{}}` (T23739c); we accepted it with an invented type"),
    }
}
