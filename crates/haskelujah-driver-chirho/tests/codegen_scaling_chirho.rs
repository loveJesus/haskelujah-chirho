// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded pattern compilation plus observable ordering and laziness. The child
//! boundary makes a growth regression a named failure rather than a host OOM.

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

#[test]
fn ghc_t2045_compiles_with_bounded_memory_chirho() {
    if std::env::var_os("CODEGEN_SCALING_CHILD_CHIRHO").is_some() {
        let source_chirho =
            include_str!("../../../ghc-tests-chirho/typecheck-chirho/should_compile/T2045.hs");
        eprintln!("BEGIN_CHIRHO T2045.hs full compilation");
        let result_chirho = haskelujah_driver::compile_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "T2045.hs",
        )
        .expect("T2045 must finish frontend, desugaring and backend generation");
        assert!(!result_chirho.llvm_ir_chirho.is_empty());
        assert!(!result_chirho.wasm_bytes_chirho.is_empty());
        eprintln!("END_CHIRHO T2045.hs full compilation");
        return;
    }
    let output_chirho =
        haskelujah_test_harness_chirho::process_chirho::run_bounded_with_memory_chirho(
            std::process::Command::new(std::env::current_exe().expect("test executable"))
                .args([
                    "--exact",
                    "ghc_t2045_compiles_with_bounded_memory_chirho",
                    "--nocapture",
                ])
                .env("CODEGEN_SCALING_CHILD_CHIRHO", "1")
                .env("RUST_MIN_STACK", "16777216"),
            &[],
            std::time::Duration::from_secs(90),
            Some(1024 * 1024 * 1024),
        )
        .expect("T2045 stays within the one GiB platform memory bound and 90 seconds");
    assert!(
        output_chirho.status.success(),
        "{}",
        String::from_utf8_lossy(&output_chirho.stderr)
    );
    assert!(String::from_utf8_lossy(&output_chirho.stderr).contains("END_CHIRHO T2045.hs"));
}

#[test]
fn pattern_compilation_grows_with_input_not_alternative_suffixes_chirho() {
    for count_chirho in [8, 16, 32] {
        let mut source_chirho = String::from(
            "module ScalingChirho where\ndata BoxChirho = BoxChirho [Int]\nmatchChirho :: BoxChirho -> Int\nmatchChirho valueChirho = case valueChirho of\n",
        );
        for length_chirho in 0..count_chirho {
            let fields_chirho = vec!["_"; length_chirho].join(",");
            source_chirho.push_str(&format!(
                "  BoxChirho [{fields_chirho}] -> {length_chirho}\n"
            ));
        }
        source_chirho.push_str("  _ -> -1\n");
        let frontend_chirho = haskelujah_driver::typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ScalingChirho.hs",
        )
        .expect("well-typed list-length pattern family");
        let desugared_chirho = haskelujah_core_chirho::desugar_chirho::desugar_module_chirho(
            &frontend_chirho.module_chirho,
        );
        let size_chirho: usize = desugared_chirho
            .module_chirho
            .bindings_chirho
            .iter()
            .map(|binding_chirho| {
                haskelujah_core_chirho::simplify_chirho::expr_size_chirho(
                    &binding_chirho.rhs_chirho,
                )
            })
            .sum();
        assert!(
            size_chirho < 32 * source_chirho.len(),
            "{count_chirho} rows: {size_chirho} Core nodes for {} source bytes",
            source_chirho.len()
        );
    }
}

#[test]
fn nested_pattern_fallthrough_preserves_order_evidence_and_laziness_chirho() {
    let source_chirho = r#"module Main where
data BoxChirho = BoxChirho [Int]
chooseChirho :: BoxChirho -> String
chooseChirho valueChirho = case valueChirho of
  BoxChirho [_] -> show True
  BoxChirho [_, _] -> show (2.5 :: Double)
  BoxChirho (firstChirho : _) -> show (firstChirho + 100)
  _ -> "empty"
data PairChirho = PairChirho (Maybe Int) (Maybe Int)
selectChirho :: PairChirho -> Int
selectChirho pairChirho = case pairChirho of
  PairChirho Nothing _ -> 1
  PairChirho (Just valueChirho) Nothing -> valueChirho
  PairChirho _ (Just valueChirho) -> valueChirho + 10
main = do
  putStrLn (chooseChirho (BoxChirho []))
  putStrLn (chooseChirho (BoxChirho [5]))
  putStrLn (chooseChirho (BoxChirho [6, 7]))
  putStrLn (chooseChirho (BoxChirho [8, 9, 10]))
  print (selectChirho (PairChirho Nothing (error "unusedChirho")))
  print (selectChirho (PairChirho (Just 7) Nothing))
  print (selectChirho (PairChirho (Just 3) (Just 9)))
"#;
    assert_all_engines_chirho(source_chirho, "empty\nTrue\n2.5\n108\n1\n7\n19\n");
}

fn assert_all_engines_chirho(source_chirho: &str, expected_chirho: &str) {
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "PatternsChirho.hs",
        None,
    )
    .expect("pattern program evaluates");
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn nested_singleton_failure_tries_the_following_alternative_chirho() {
    assert_all_engines_chirho(
        r#"module Main where
singletonChirho :: [Int] -> Int
singletonChirho xsChirho = case xsChirho of
  valueChirho : [] -> 1
  _ -> 2
main = do
  print (singletonChirho [])
  print (singletonChirho [99])
  print (singletonChirho [88, 77])
  print (singletonChirho [error "unused-head-chirho"])
"#,
        "2\n1\n2\n1\n",
    );
}

#[test]
fn nil_then_tuple_cons_uses_the_head_without_forcing_the_tail_chirho() {
    assert_all_engines_chirho(
        r#"module Main where
headValueChirho :: [(Int, Int)] -> Int
headValueChirho xsChirho = case xsChirho of
  [] -> 0
  (keyChirho, valueChirho) : restChirho -> valueChirho
main = do
  print (headValueChirho [])
  print (headValueChirho [(17, 42), (33, error "unused-tail-field-chirho")])
  print (headValueChirho ((17, 42) : error "unused-tail-chirho"))
"#,
        "0\n42\n42\n",
    );
}
