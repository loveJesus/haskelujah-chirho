// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! End-to-end RecursiveDo and MonadFix regressions.
//!
//! workflow: recursive-do-chirho

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn eval_output_chirho(source_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        "RecursiveDoChirho.hs",
        None,
    )
    .expect("RecursiveDo program should evaluate");
    machine_chirho.io_output_chirho
}

#[test]
fn eval_explicit_rec_ties_lazy_io_knot_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE RecursiveDo #-}\n",
        "module Test where\n",
        "main = do\n",
        "  rec\n",
        "    xsChirho <- pure (1 : ysChirho)\n",
        "    ysChirho <- pure (2 : xsChirho)\n",
        "  print (take 4 xsChirho)\n",
    );

    assert_eq!(eval_output_chirho(source_chirho), "[1,2,1,2]\n");
}

#[test]
fn eval_mdo_resolves_forward_reference_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE RecursiveDo #-}\n",
        "module Test where\n",
        "main = mdo\n",
        "  firstChirho <- pure secondChirho\n",
        "  secondChirho <- pure (42 :: Int)\n",
        "  print firstChirho\n",
    );

    assert_eq!(eval_output_chirho(source_chirho), "42\n");
}

#[test]
fn eval_maybe_and_list_mfix_use_backed_instance_bodies_chirho() {
    let source_chirho = concat!(
        "module Test where\n",
        "import Control.Monad.Fix (mfix)\n",
        "main = do\n",
        "  print (mfix (\\valueChirho -> Just 1) :: Maybe Int)\n",
        "  print (mfix (\\valueChirho -> Nothing) :: Maybe Int)\n",
        "  print (mfix (\\valueChirho -> [1, 2]) :: [Int])\n",
    );

    assert_eq!(
        eval_output_chirho(source_chirho),
        "Just 1\nNothing\n[1,2]\n"
    );
}

#[test]
fn eval_mdo_and_rec_remain_plain_identifiers_without_extension_chirho() {
    let source_chirho = concat!(
        "module Test where\n",
        "mdo = 20\n",
        "rec = 22\n",
        "main = print (mdo + rec)\n",
    );

    assert_eq!(eval_output_chirho(source_chirho), "42\n");
}
