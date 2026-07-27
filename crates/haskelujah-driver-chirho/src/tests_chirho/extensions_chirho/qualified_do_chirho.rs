// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! End-to-end QualifiedDo regressions.
//!
//! workflow: language-features-chirho/qualified-do-chirho

use crate::{compile_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;

const T21206_SOURCE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T21206.hs"
));

fn eval_output_chirho(source_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        "QualifiedDoChirho.hs",
        None,
    )
    .expect("QualifiedDo program should evaluate");
    machine_chirho.io_output_chirho
}

#[test]
fn eval_self_qualified_do_uses_local_bind_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE QualifiedDo #-}\n",
        "module QualifiedDoRuntimeChirho where\n",
        "valueChirho >>= nextChirho = nextChirho valueChirho\n",
        "main = print (QualifiedDoRuntimeChirho.do\n",
        "  valueChirho <- 4\n",
        "  valueChirho + 1)\n",
    );

    assert_eq!(eval_output_chirho(source_chirho), "5\n");
}

#[test]
fn eval_import_qualified_do_uses_imported_then_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE QualifiedDo #-}\n",
        "module QualifiedDoPreludeChirho where\n",
        "import qualified Prelude as P\n",
        "main = P.do\n",
        "  P.print (1 :: Int)\n",
        "  P.print (2 :: Int)\n",
    );

    assert_eq!(eval_output_chirho(source_chirho), "1\n2\n");
}

#[test]
fn eval_unknown_qualified_do_method_stays_loud_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE QualifiedDo #-}\n",
        "module QualifiedDoMissingChirho where\n",
        "import qualified Data.List as MissingChirho\n",
        "main = print (MissingChirho.do\n",
        "  valueChirho <- 4\n",
        "  valueChirho + 1)\n",
    );
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let error_chirho = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        "QualifiedDoMissingChirho.hs",
        None,
    )
    .expect_err("unknown QualifiedDo methods must not fall back to Prelude");

    assert!(
        error_chirho.contains("MissingChirho.>>="),
        "expected the unresolved qualified bind name, got {error_chirho}"
    );
}

#[test]
fn typecheck_t21206_rank_n_qualified_do_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(T21206_SOURCE_CHIRHO, &mut source_map_chirho, "T21206.hs");

    assert!(
        result_chirho.is_ok(),
        "T21206 QualifiedDo regression should type-check: {:?}",
        result_chirho.err()
    );
}
