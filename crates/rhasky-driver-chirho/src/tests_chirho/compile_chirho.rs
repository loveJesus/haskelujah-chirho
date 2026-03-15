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
                },
            }],
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
                },
            }),
            executables_chirho: vec![],
            test_suites_chirho: vec![],
        };

        let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
        assert_eq!(modules_chirho.len(), 1);
        assert_eq!(modules_chirho[0].0, "Data.Map.Internal");
        assert!(modules_chirho[0].1.ends_with("Data/Map/Internal.hs"));
    }

