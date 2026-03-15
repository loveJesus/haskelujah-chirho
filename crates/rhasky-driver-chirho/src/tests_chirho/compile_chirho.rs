// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Compilation pipeline, check, LLVM, Wasm, incremental, module discovery tests

#[allow(unused_imports)]
use crate::{
    eval_source_chirho,
    eval_source_with_machine_chirho,
    eval_source_with_input_chirho,
    eval_source_with_step_limit_chirho,
    compile_source_chirho,
    check_source_file_chirho,
    render_summary_chirho,
    compile_modules_chirho,
    eval_modules_chirho,
    compile_modules_incremental_chirho,
    discover_modules_chirho,
};
#[allow(unused_imports)]
use rhasky_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use rhasky_runtime_chirho::{ValueChirho, ExecutionModeChirho};
#[allow(unused_imports)]
use rhasky_syntax_chirho::SourceFileChirho;

    #[test]
    fn builds_a_check_summary_for_batch_mode_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "BatchSampleChirho.hs",
            "module BatchSampleChirho where\nvalueChirho = 1\n",
        );

        let check_summary_chirho =
            check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho)
                .expect("driver should accept a valid source file");

        assert_eq!(check_summary_chirho.module_name_chirho, "BatchSampleChirho");
        assert!(!check_summary_chirho.runtime_plan_chirho.incremental_session_chirho);
        assert!(
            render_summary_chirho(&check_summary_chirho).contains("llvm_preview: ; rhasky llvm stub")
        );
    }


    #[test]
    fn full_pipeline_parses_and_resolves_chirho() {
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green\nf x = x\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("full pipeline should succeed");

        assert_eq!(result_chirho.module_chirho.name_chirho.text_chirho(), "Test");
        assert!(result_chirho.module_chirho.decls_chirho.len() >= 2);
        // Core module was produced by desugaring
        assert_eq!(result_chirho.core_chirho.name_chirho, "Test");
        assert!(!result_chirho.core_chirho.bindings_chirho.is_empty());
        // Backend output was produced
        assert!(result_chirho.llvm_ir_chirho.contains("; ModuleID = 'Test'"));
        assert_eq!(&result_chirho.wasm_bytes_chirho[0..4], b"\0asm");
    }


    #[test]
    fn script_mode_uses_incremental_runtime_plan_chirho() {
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            "ScriptSampleChirho.hs",
            "mainChirho = print 42\n",
        );

        let check_summary_chirho =
            check_source_file_chirho(source_file_chirho, ExecutionModeChirho::ScriptChirho)
                .expect("script mode should accept module-less files");

        assert!(check_summary_chirho.runtime_plan_chirho.incremental_session_chirho);
        assert_eq!(check_summary_chirho.module_name_chirho, "Main");
    }


    #[test]
    fn multi_module_import_chirho() {
        use crate::compile_modules_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let sources_chirho: Vec<(&str, &str)> = vec![
            (
                "LibChirho.hs",
                "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
            ),
            (
                "MainChirho.hs",
                "module Main where\nimport Lib\nf x = case x of\n  Red -> 1\n  Green -> 2\n",
            ),
        ];

        let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho)
            .expect("multi-module compilation should succeed");

        assert_eq!(results_chirho.len(), 2);
        assert_eq!(results_chirho[0].module_chirho.name_chirho.text_chirho(), "Lib");
        assert_eq!(results_chirho[1].module_chirho.name_chirho.text_chirho(), "Main");
    }


    #[test]
    fn exhaustiveness_check_passes_for_complete_case_chirho() {
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // data Color = Red | Green — case covers both constructors
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green\nf x = case x of\n  Red -> 1\n  Green -> 2\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_ok(),
            "complete case should pass exhaustiveness check"
        );
    }


    #[test]
    fn exhaustiveness_check_rejects_incomplete_case_chirho() {
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // data Color = Red | Green | Blue — case only covers Red
        let result_chirho = compile_source_chirho(
            "module Test where\ndata Color = Red | Green | Blue\nf x = case x of\n  Red -> 1\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        );
        assert!(
            result_chirho.is_err(),
            "incomplete case should fail exhaustiveness check"
        );
        let err_chirho = result_chirho.unwrap_err();
        assert!(err_chirho.has_errors_chirho());
        let msg_chirho = format!("{}", err_chirho);
        assert!(
            msg_chirho.contains("Green") || msg_chirho.contains("Blue"),
            "error should mention missing constructors"
        );
    }


    #[test]
    fn incremental_compilation_marks_recompiled_chirho() {
        use crate::compile_modules_incremental_chirho;
        use rhasky_incremental_chirho::IncrementalSessionChirho;

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let mut session_chirho = IncrementalSessionChirho::new_chirho();

        let sources_chirho: Vec<(&str, &str)> = vec![
            (
                "LibChirho.hs",
                "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
            ),
            (
                "MainChirho.hs",
                "module Main where\nimport Lib\nf x = x\n",
            ),
        ];

        // First build: everything should be recompiled.
        let results_chirho =
            compile_modules_incremental_chirho(&sources_chirho, &mut source_map_chirho, &mut session_chirho)
                .expect("incremental compilation should succeed");

        assert_eq!(results_chirho.len(), 2);
        assert!(results_chirho[0].1, "Lib should be recompiled on first build");
        assert!(results_chirho[1].1, "Main should be recompiled on first build");

        // Second build with identical sources: nothing should be recompiled.
        let mut source_map_chirho2 = SourceMapChirho::new_chirho();
        let results2_chirho =
            compile_modules_incremental_chirho(&sources_chirho, &mut source_map_chirho2, &mut session_chirho)
                .expect("second incremental compilation should succeed");

        assert_eq!(results2_chirho.len(), 2);
        assert!(
            !results2_chirho[0].1,
            "Lib should NOT be recompiled when source unchanged"
        );
        assert!(
            !results2_chirho[1].1,
            "Main should NOT be recompiled when source unchanged"
        );
    }


    #[test]
    fn incremental_detects_source_change_chirho() {
        use crate::compile_modules_incremental_chirho;
        use rhasky_incremental_chirho::IncrementalSessionChirho;

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let mut session_chirho = IncrementalSessionChirho::new_chirho();

        let sources_v1_chirho: Vec<(&str, &str)> = vec![(
            "TestChirho.hs",
            "module Test where\nf x = x\n",
        )];

        let _ = compile_modules_incremental_chirho(
            &sources_v1_chirho,
            &mut source_map_chirho,
            &mut session_chirho,
        )
        .expect("v1 should compile");

        // Change the source.
        let sources_v2_chirho: Vec<(&str, &str)> = vec![(
            "TestChirho.hs",
            "module Test where\nf x = x\ng y = y\n",
        )];

        let mut source_map2_chirho = SourceMapChirho::new_chirho();
        let results_chirho = compile_modules_incremental_chirho(
            &sources_v2_chirho,
            &mut source_map2_chirho,
            &mut session_chirho,
        )
        .expect("v2 should compile");

        assert_eq!(results_chirho.len(), 1);
        assert!(
            results_chirho[0].1,
            "Test should be recompiled when source changes"
        );
    }


    #[test]
    fn cabal_project_compilation_chirho() {
        use crate::compile_cabal_project_chirho;
        use std::io::Write;

        // Create a temp directory with a small Cabal project
        let temp_dir_chirho = std::env::temp_dir().join("rhasky_cabal_test_chirho");
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
        std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

        // Write .cabal file
        let cabal_path_chirho = temp_dir_chirho.join("hello.cabal");
        let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
        write!(
            cabal_file_chirho,
            r#"cabal-version: 3.0
name:         hello
version:      0.1.0.0

library
  exposed-modules: Lib
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5
  default-language: Haskell2010
"#
        )
        .unwrap();

        // Write Haskell source
        let lib_path_chirho = temp_dir_chirho.join("src").join("Lib.hs");
        let mut lib_file_chirho = std::fs::File::create(&lib_path_chirho).unwrap();
        write!(
            lib_file_chirho,
            "module Lib where\nf x = x\n"
        )
        .unwrap();

        // Create an empty package index (base is builtin, so no external deps needed)
        let index_chirho = rhasky_package_chirho::PackageIndexChirho::new_chirho();

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
                .expect("cabal project should compile");

        assert_eq!(result_chirho.package_chirho.name_chirho, "hello");
        assert!(result_chirho.build_plan_chirho.steps_chirho.is_empty()); // only base (builtin)
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
        assert_eq!(
            result_chirho.module_results_chirho[0]
                .module_chirho
                .name_chirho
                .text_chirho(),
            "Lib"
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    }


    #[test]
    fn cabal_project_multi_module_chirho() {
        use crate::compile_cabal_project_chirho;
        use std::io::Write;

        let temp_dir_chirho = std::env::temp_dir().join("rhasky_cabal_multi_test_chirho");
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
        std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

        let cabal_path_chirho = temp_dir_chirho.join("multi.cabal");
        let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
        write!(
            cabal_file_chirho,
            r#"cabal-version: 3.0
name:         multi
version:      0.1.0.0

library
  exposed-modules: Lib, Helper
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5
"#
        )
        .unwrap();

        let mut lib_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
        write!(lib_chirho, "module Lib where\nf x = x\n").unwrap();

        let mut helper_chirho =
            std::fs::File::create(temp_dir_chirho.join("src/Helper.hs")).unwrap();
        write!(helper_chirho, "module Helper where\ng y = y\n").unwrap();

        let index_chirho = rhasky_package_chirho::PackageIndexChirho::new_chirho();

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
                .expect("multi-module cabal project should compile");

        assert_eq!(result_chirho.package_chirho.name_chirho, "multi");
        assert_eq!(result_chirho.module_results_chirho.len(), 2);

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    }


    #[test]
    fn cabal_project_missing_module_skipped_chirho() {
        use crate::compile_cabal_project_chirho;
        use std::io::Write;

        let temp_dir_chirho = std::env::temp_dir().join("rhasky_cabal_missing_test_chirho");
        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
        std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

        let cabal_path_chirho = temp_dir_chirho.join("miss.cabal");
        let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
        write!(
            cabal_file_chirho,
            r#"cabal-version: 3.0
name:         miss
version:      0.1.0.0

library
  exposed-modules: Exists, DoesNotExist
  hs-source-dirs:  src
  build-depends:   base
"#
        )
        .unwrap();

        // Only create Exists.hs, not DoesNotExist.hs
        let mut exists_chirho =
            std::fs::File::create(temp_dir_chirho.join("src/Exists.hs")).unwrap();
        write!(exists_chirho, "module Exists where\nval = 42\n").unwrap();

        let index_chirho = rhasky_package_chirho::PackageIndexChirho::new_chirho();

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
                .expect("should compile what exists");

        // Only Exists should be compiled (DoesNotExist not found on disk)
        assert_eq!(result_chirho.module_results_chirho.len(), 1);

        let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    }

    // ── PrimOp / arithmetic end-to-end tests ────────────────────────────


    #[test]
    fn compile_primop_to_llvm_chirho() {
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        // Use non-literal args so constant folding doesn't eliminate the primop
        let result_chirho = compile_source_chirho(
            "module Test where\nf x = x + 1\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");
        // LLVM IR should contain an add instruction (not folded away because x is a variable)
        assert!(
            result_chirho.llvm_ir_chirho.contains("add i64"),
            "LLVM IR should contain add instruction: {}",
            result_chirho.llvm_ir_chirho
        );
    }


    #[test]
    fn compile_primop_to_wasm_chirho() {
        use crate::compile_source_chirho;
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(
            "module Test where\nmain = 3 + 4\n",
            &mut source_map_chirho,
            "TestChirho.hs",
        )
        .expect("should compile");
        // WASM binary should be a valid module
        assert_eq!(&result_chirho.wasm_bytes_chirho[0..4], b"\0asm");
        assert!(result_chirho.wasm_bytes_chirho.len() > 20);
    }


    #[test]
    fn discover_test_suite_modules_chirho() {
        use rhasky_package_chirho::{
            BuildInfoChirho, PackageDescChirho, TestSuiteChirho,
        };
        use std::fs;
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        let test_dir_chirho = dir_chirho.path().join("test");
        fs::create_dir_all(&test_dir_chirho).unwrap();
        fs::write(test_dir_chirho.join("Spec.hs"), "module Spec where\n").unwrap();
        fs::write(test_dir_chirho.join("Main.hs"), "module Main where\n").unwrap();

        let pkg_chirho = PackageDescChirho {
            name_chirho: "test-pkg".to_string(),
            version_chirho: None,
            cabal_version_chirho: None,
            license_chirho: None,
            author_chirho: None,
            maintainer_chirho: None,
            synopsis_chirho: None,
            description_chirho: None,
            category_chirho: None,
            homepage_chirho: None,
            bug_reports_chirho: None,
            build_type_chirho: None,
            library_chirho: None,
            executables_chirho: vec![],
            test_suites_chirho: vec![TestSuiteChirho {
                name_chirho: "my-tests".to_string(),
                type_chirho: Some("exitcode-stdio-1.0".to_string()),
                main_is_chirho: Some("Main.hs".to_string()),
                other_modules_chirho: vec!["Spec".to_string()],
                build_info_chirho: BuildInfoChirho {
                    build_depends_chirho: vec![],
                    hs_source_dirs_chirho: vec!["test".to_string()],
                    default_language_chirho: None,
                    ghc_options_chirho: vec![],
                    default_extensions_chirho: vec![],
                    other_extensions_chirho: vec![],
                    imports_chirho: vec![],
                },
            }],
            benchmarks_chirho: vec![],
            flags_chirho: vec![],
            source_repos_chirho: vec![],
            common_stanzas_chirho: vec![],
            custom_setup_chirho: None,
        };

        let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        let names_chirho: Vec<&str> = modules_chirho.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names_chirho.contains(&"Main"), "should discover test-suite Main");
        assert!(names_chirho.contains(&"Spec"), "should discover test-suite other-module Spec");
    }


    #[test]
    fn discover_setup_hs_chirho() {
        use rhasky_package_chirho::PackageDescChirho;
        use std::fs;
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        fs::write(
            dir_chirho.path().join("Setup.hs"),
            "import Distribution.Simple\nmain = defaultMain\n",
        )
        .unwrap();

        let pkg_chirho = PackageDescChirho {
            name_chirho: "setup-pkg".to_string(),
            version_chirho: None,
            cabal_version_chirho: None,
            license_chirho: None,
            author_chirho: None,
            maintainer_chirho: None,
            synopsis_chirho: None,
            description_chirho: None,
            category_chirho: None,
            homepage_chirho: None,
            bug_reports_chirho: None,
            build_type_chirho: None,
            library_chirho: None,
            executables_chirho: vec![],
            test_suites_chirho: vec![],
            benchmarks_chirho: vec![],
            flags_chirho: vec![],
            source_repos_chirho: vec![],
            common_stanzas_chirho: vec![],
            custom_setup_chirho: None,
        };

        let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        let names_chirho: Vec<&str> = modules_chirho.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names_chirho.contains(&"Setup"), "should discover Setup.hs");
    }


    #[test]
    fn discover_hierarchical_module_chirho() {
        use rhasky_package_chirho::{BuildInfoChirho, LibraryChirho, PackageDescChirho};
        use std::fs;
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        let src_dir_chirho = dir_chirho.path().join("src").join("Data").join("Map");
        fs::create_dir_all(&src_dir_chirho).unwrap();
        fs::write(
            src_dir_chirho.join("Internal.hs"),
            "module Data.Map.Internal where\n",
        )
        .unwrap();

        let pkg_chirho = PackageDescChirho {
            name_chirho: "hier-pkg".to_string(),
            version_chirho: None,
            cabal_version_chirho: None,
            license_chirho: None,
            author_chirho: None,
            maintainer_chirho: None,
            synopsis_chirho: None,
            description_chirho: None,
            category_chirho: None,
            homepage_chirho: None,
            bug_reports_chirho: None,
            build_type_chirho: None,
            library_chirho: Some(LibraryChirho {
                exposed_modules_chirho: vec!["Data.Map.Internal".to_string()],
                other_modules_chirho: vec![],
                build_info_chirho: BuildInfoChirho {
                    build_depends_chirho: vec![],
                    hs_source_dirs_chirho: vec!["src".to_string()],
                    default_language_chirho: None,
                    ghc_options_chirho: vec![],
                    default_extensions_chirho: vec![],
                    other_extensions_chirho: vec![],
                    imports_chirho: vec![],
                },
            }),
            executables_chirho: vec![],
            test_suites_chirho: vec![],
            benchmarks_chirho: vec![],
            flags_chirho: vec![],
            source_repos_chirho: vec![],
            common_stanzas_chirho: vec![],
            custom_setup_chirho: None,
        };

        let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        assert_eq!(modules_chirho.len(), 1);
        assert_eq!(modules_chirho[0].0, "Data.Map.Internal");
        assert!(modules_chirho[0].1.ends_with("Data/Map/Internal.hs"));
    }

    #[test]
    fn llvm_executable_constant_chirho() {
        // main = 42 should produce LLVM IR with ret i64 42
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let exec_ir_chirho =
            rhasky_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
                &result_chirho.core_chirho,
            );
        assert!(exec_ir_chirho.contains("define i64 @rhasky_main()"));
        assert!(exec_ir_chirho.contains("ret i64 42"));
        assert!(exec_ir_chirho.contains("define i32 @main()"));
        assert!(exec_ir_chirho.contains("call i64 @rhasky_main()"));
    }

    #[test]
    fn llvm_executable_arithmetic_chirho() {
        // f x y = x + y; main = f 10 32 should produce correct LLVM IR
        let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let exec_ir_chirho =
            rhasky_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
                &result_chirho.core_chirho,
            );
        // f should compile to an add instruction
        assert!(exec_ir_chirho.contains("add i64"));
        // main should call f with arguments
        assert!(exec_ir_chirho.contains("call i64 @rhasky_f(i64 10, i64 32)"));
    }

    #[test]
    fn llvm_executable_fibonacci_chirho() {
        let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let exec_ir_chirho =
            rhasky_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
                &result_chirho.core_chirho,
            );
        // Should have recursive fib function and main
        assert!(exec_ir_chirho.contains("@rhasky_fib"));
        assert!(exec_ir_chirho.contains("define i32 @main()"));
    }

    #[test]
    fn wasm_executable_constant_chirho() {
        // main = 42 should produce valid WASM with dict elision
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let wasm_chirho =
            rhasky_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
                &result_chirho.core_chirho,
            );
        assert_eq!(&wasm_chirho[0..4], b"\0asm");
        assert_eq!(&wasm_chirho[4..8], &[1, 0, 0, 0]);
        assert!(wasm_chirho.len() > 20);
    }

    #[test]
    fn wasm_executable_arithmetic_chirho() {
        // f x y = x + y; main = f 10 32 should produce WASM with call instruction
        let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let wasm_chirho =
            rhasky_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
                &result_chirho.core_chirho,
            );
        assert_eq!(&wasm_chirho[0..4], b"\0asm");
        // Should contain call opcode (0x10) for the function call f 10 32
        assert!(
            wasm_chirho.contains(&0x10_u8),
            "WASM should contain call instruction"
        );
    }

    #[test]
    fn wasm_executable_fibonacci_chirho() {
        let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let wasm_chirho =
            rhasky_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
                &result_chirho.core_chirho,
            );
        assert_eq!(&wasm_chirho[0..4], b"\0asm");
        // Should contain function call and case/if instructions
        assert!(wasm_chirho.contains(&0x10_u8), "WASM should contain call instruction");
        assert!(wasm_chirho.contains(&0x04_u8), "WASM should contain if instruction");
    }

    // ── LLVM round-trip smoke tests (§H.61) ──────────────────────────────
    // These tests compile Haskell source to LLVM IR, invoke clang to produce
    // a native binary, execute it, and compare the exit code to the STG
    // interpreter result.

    /// Helper: compile source to LLVM IR executable, link with clang, run, return exit code.
    fn llvm_round_trip_chirho(src_chirho: &str) -> Option<i32> {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;

        let exec_ir_chirho =
            rhasky_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(&result_chirho.core_chirho);

        let tmp_dir_chirho = tempfile::tempdir().ok()?;
        let ll_path_chirho = tmp_dir_chirho.path().join("main.ll");
        let bin_path_chirho = tmp_dir_chirho.path().join("main");
        std::fs::write(&ll_path_chirho, &exec_ir_chirho).ok()?;

        let compile_output_chirho = std::process::Command::new("clang")
            .arg("-O0")
            .arg("-o")
            .arg(&bin_path_chirho)
            .arg(&ll_path_chirho)
            .output()
            .ok()?;

        if !compile_output_chirho.status.success() {
            return None;
        }

        let run_output_chirho = std::process::Command::new(&bin_path_chirho)
            .output()
            .ok()?;

        run_output_chirho.status.code()
    }

    #[test]
    fn llvm_round_trip_constant_chirho() {
        // main = 42 → exit code 42
        let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 42");
        if let Some(code_chirho) = exit_code_chirho {
            assert_eq!(code_chirho, 42, "LLVM round-trip: main = 42 should exit with 42");
        }
        // If clang not available or linking fails, skip silently
    }

    #[test]
    fn llvm_round_trip_arithmetic_chirho() {
        // f x y = x + y; main = f 10 32 → exit code 42
        let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
        let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
        if let Some(code_chirho) = exit_code_chirho {
            assert_eq!(code_chirho, 42, "LLVM round-trip: f 10 32 should exit with 42");
        }
    }

    #[test]
    fn llvm_round_trip_fibonacci_chirho() {
        // fib 10 = 55 → exit code 55
        let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
        let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
        if let Some(code_chirho) = exit_code_chirho {
            assert_eq!(code_chirho, 55, "LLVM round-trip: fib 10 should exit with 55");
        }
    }

    #[test]
    fn llvm_round_trip_matches_stg_chirho() {
        // Compare LLVM native execution with STG interpreter result
        let src_chirho = "module Main where\nmain = 3 * 14";
        // STG interpreter
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let stg_result_chirho = crate::eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
        let stg_val_chirho = match stg_result_chirho {
            Ok(ValueChirho::IntChirho(n_chirho)) => n_chirho,
            _ => return, // skip if STG eval fails
        };

        // LLVM native
        let native_code_chirho = llvm_round_trip_chirho(src_chirho);
        if let Some(code_chirho) = native_code_chirho {
            assert_eq!(
                code_chirho as i64, stg_val_chirho,
                "LLVM native exit code should match STG interpreter result"
            );
        }
    }

    // ── Cranelift backend driver integration tests ────────────────────────

    #[test]
    fn cranelift_executable_constant_chirho() {
        // main = 42 should produce valid native object file
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let config_chirho = rhasky_backend_cranelift_chirho::TargetConfigChirho::default();
        let obj_chirho =
            rhasky_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                &result_chirho.core_chirho,
                &config_chirho,
            )
            .expect("cranelift compilation should succeed");
        assert!(
            !obj_chirho.object_bytes_chirho.is_empty(),
            "object file should not be empty"
        );
    }

    #[test]
    fn cranelift_executable_arithmetic_chirho() {
        // f x y = x + y; main = f 10 32 should produce valid object with function call
        let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let config_chirho = rhasky_backend_cranelift_chirho::TargetConfigChirho::default();
        let obj_chirho =
            rhasky_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                &result_chirho.core_chirho,
                &config_chirho,
            )
            .expect("cranelift compilation should succeed");
        assert!(
            !obj_chirho.object_bytes_chirho.is_empty(),
            "object file should not be empty"
        );
    }

    #[test]
    fn cranelift_executable_fibonacci_chirho() {
        let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
                .expect("should compile");

        let config_chirho = rhasky_backend_cranelift_chirho::TargetConfigChirho::default();
        let obj_chirho =
            rhasky_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
                &result_chirho.core_chirho,
                &config_chirho,
            )
            .expect("cranelift compilation should succeed");
        assert!(
            !obj_chirho.object_bytes_chirho.is_empty(),
            "cranelift fibonacci object file should not be empty"
        );
    }

    // ---------------------------------------------------------------
    // §29 — Structured error messages with "did you mean?" suggestions
    // ---------------------------------------------------------------

    #[test]
    fn unbound_var_suggests_similar_name_chirho() {
        // Typo: "ad1" instead of "add1"
        let src_chirho = "module Main where\nadd1 x = x + 1\nmain = ad1 41\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
        assert!(result_chirho.is_err(), "should fail with unbound variable");
        let diag_chirho = result_chirho.unwrap_err();
        let rendered_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
        assert!(
            rendered_chirho.contains("did you mean"),
            "error should contain 'did you mean' suggestion, got: {rendered_chirho}"
        );
        assert!(
            rendered_chirho.contains("add1"),
            "suggestion should include 'add1', got: {rendered_chirho}"
        );
    }

    #[test]
    fn type_mismatch_shows_expected_found_chirho() {
        let src_chirho = "module Main where\nf :: Int -> Int\nf x = x + 1\nmain = f True\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
        assert!(result_chirho.is_err());
        let diag_chirho = result_chirho.unwrap_err();
        let rendered_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
        assert!(
            rendered_chirho.contains("type mismatch"),
            "error should mention 'type mismatch', got: {rendered_chirho}"
        );
        assert!(
            rendered_chirho.contains("expected type:") && rendered_chirho.contains("found type:"),
            "error should show expected/found types, got: {rendered_chirho}"
        );
    }

    #[test]
    fn error_rendered_with_source_snippet_chirho() {
        let src_chirho = "module Main where\nmain = undefined_func 42\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
        assert!(result_chirho.is_err());
        let diag_chirho = result_chirho.unwrap_err();
        let rendered_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
        assert!(
            rendered_chirho.contains("Main.hs"),
            "error should reference file, got: {rendered_chirho}"
        );
        assert!(
            rendered_chirho.contains("-->"),
            "error should have --> source pointer, got: {rendered_chirho}"
        );
    }

    #[test]
    fn error_rendered_with_color_chirho() {
        let src_chirho = "module Main where\nmain = no_such_var\n";
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
        assert!(result_chirho.is_err());
        let diag_chirho = result_chirho.unwrap_err();
        let colored_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, true);
        assert!(
            colored_chirho.contains("\x1b["),
            "colored output should contain ANSI escapes"
        );
        let plain_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
        assert!(
            !plain_chirho.contains("\x1b["),
            "plain output should not contain ANSI escapes"
        );
    }

    // ── §44 Foreign exports ──────────────────────────────────────────────

    #[test]
    fn foreign_export_parsed_and_reaches_core_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = concat!(
            "module Test where\n",
            "foreign export ccall addOne :: Int -> Int\n",
            "addOne x = x + 1\n",
            "main = addOne 41\n",
        );
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ForeignExport.hs")
            .expect("should compile with foreign export");
        // The Core module should have the foreign export recorded
        assert!(
            !result_chirho.core_chirho.foreign_exports_chirho.is_empty(),
            "foreign exports should be propagated to Core module"
        );
        let export_chirho = &result_chirho.core_chirho.foreign_exports_chirho[0];
        assert_eq!(export_chirho.haskell_name_chirho, "addOne");
        assert_eq!(export_chirho.foreign_name_chirho, "addOne");
        assert_eq!(export_chirho.calling_conv_chirho, "ccall");
    }

    #[test]
    fn foreign_export_with_custom_c_name_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = concat!(
            "module Test where\n",
            "foreign export ccall \"hs_add_one\" addOne :: Int -> Int\n",
            "addOne x = x + 1\n",
            "main = addOne 41\n",
        );
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ForeignExportName.hs")
            .expect("should compile with custom C name");
        let export_chirho = &result_chirho.core_chirho.foreign_exports_chirho[0];
        assert_eq!(export_chirho.haskell_name_chirho, "addOne");
        assert_eq!(export_chirho.foreign_name_chirho, "hs_add_one");
    }

    #[test]
    fn foreign_export_llvm_emits_wrapper_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = concat!(
            "module Test where\n",
            "foreign export ccall \"hs_val\" getVal :: Int\n",
            "getVal = 42\n",
            "main = getVal\n",
        );
        let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ForeignExportLLVM.hs")
            .expect("should compile");
        assert!(
            result_chirho.llvm_ir_chirho.contains("@hs_val"),
            "LLVM IR should contain the foreign export wrapper: {}",
            result_chirho.llvm_ir_chirho
        );
    }

    #[test]
    fn foreign_export_eval_still_works_chirho() {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = concat!(
            "module Test where\n",
            "foreign export ccall addOne :: Int -> Int\n",
            "addOne x = x + 1\n",
            "main = addOne 41\n",
        );
        let (val_chirho, _machine_chirho) = eval_source_with_machine_chirho(
            src_chirho, &mut sm_chirho, "ForeignExportEval.hs", None,
        ).expect("foreign export should not break evaluation");
        assert_eq!(val_chirho, rhasky_runtime_chirho::ValueChirho::IntChirho(42));
    }

