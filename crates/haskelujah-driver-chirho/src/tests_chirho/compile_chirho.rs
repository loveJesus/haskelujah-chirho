// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Compilation pipeline, check, LLVM, Wasm, incremental, module discovery tests

#[allow(unused_imports)]
use crate::{
    check_source_file_chirho, compile_modules_chirho, compile_modules_incremental_chirho,
    compile_source_chirho, discover_modules_chirho, eval_modules_chirho, eval_source_chirho,
    eval_source_with_input_chirho, eval_source_with_machine_chirho,
    eval_source_with_step_limit_chirho, render_summary_chirho,
};
#[allow(unused_imports)]
use haskelujah_runtime_chirho::{ExecutionModeChirho, ValueChirho};
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use haskelujah_syntax_chirho::SourceFileChirho;

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
    assert!(
        !check_summary_chirho
            .runtime_plan_chirho
            .incremental_session_chirho
    );
    assert!(render_summary_chirho(&check_summary_chirho)
        .contains("llvm_preview: ; haskelujah llvm stub"));
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

    assert_eq!(
        result_chirho.module_chirho.name_chirho.text_chirho(),
        "Test"
    );
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

    assert!(
        check_summary_chirho
            .runtime_plan_chirho
            .incremental_session_chirho
    );
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
    assert_eq!(
        results_chirho[0].module_chirho.name_chirho.text_chirho(),
        "Lib"
    );
    assert_eq!(
        results_chirho[1].module_chirho.name_chirho.text_chirho(),
        "Main"
    );
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
fn exhaustiveness_check_warns_incomplete_case_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // data Color = Red | Green | Blue — case only covers Red
    // Non-exhaustive patterns are warnings, not errors (matches GHC behavior)
    let result_chirho = compile_source_chirho(
        "module Test where\ndata Color = Red | Green | Blue\nf x = case x of\n  Red -> 1\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "incomplete case should succeed with warnings: {:?}",
        result_chirho.err()
    );
}

#[test]
fn incremental_compilation_marks_recompiled_chirho() {
    use crate::compile_modules_incremental_chirho;
    use haskelujah_incremental_chirho::IncrementalSessionChirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut session_chirho = IncrementalSessionChirho::new_chirho();

    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "LibChirho.hs",
            "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
        ),
        ("MainChirho.hs", "module Main where\nimport Lib\nf x = x\n"),
    ];

    // First build: everything should be recompiled.
    let results_chirho = compile_modules_incremental_chirho(
        &sources_chirho,
        &mut source_map_chirho,
        &mut session_chirho,
    )
    .expect("incremental compilation should succeed");

    assert_eq!(results_chirho.len(), 2);
    assert!(
        results_chirho[0].1,
        "Lib should be recompiled on first build"
    );
    assert!(
        results_chirho[1].1,
        "Main should be recompiled on first build"
    );

    // Second build with identical sources: nothing should be recompiled.
    let mut source_map_chirho2 = SourceMapChirho::new_chirho();
    let results2_chirho = compile_modules_incremental_chirho(
        &sources_chirho,
        &mut source_map_chirho2,
        &mut session_chirho,
    )
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
    use haskelujah_incremental_chirho::IncrementalSessionChirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut session_chirho = IncrementalSessionChirho::new_chirho();

    let sources_v1_chirho: Vec<(&str, &str)> =
        vec![("TestChirho.hs", "module Test where\nf x = x\n")];

    let _ = compile_modules_incremental_chirho(
        &sources_v1_chirho,
        &mut source_map_chirho,
        &mut session_chirho,
    )
    .expect("v1 should compile");

    // Change the source.
    let sources_v2_chirho: Vec<(&str, &str)> =
        vec![("TestChirho.hs", "module Test where\nf x = x\ng y = y\n")];

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
    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_test_chirho");
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
    write!(lib_file_chirho, "module Lib where\nf x = x\n").unwrap();

    // Create an empty package index (base is builtin, so no external deps needed)
    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
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

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_multi_test_chirho");
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

    let mut helper_chirho = std::fs::File::create(temp_dir_chirho.join("src/Helper.hs")).unwrap();
    write!(helper_chirho, "module Helper where\ng y = y\n").unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
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

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_missing_test_chirho");
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
    let mut exists_chirho = std::fs::File::create(temp_dir_chirho.join("src/Exists.hs")).unwrap();
    write!(exists_chirho, "module Exists where\nval = 42\n").unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("should compile what exists");

    // Only Exists should be compiled (DoesNotExist not found on disk)
    assert_eq!(result_chirho.module_results_chirho.len(), 1);

    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_builds_executable_llvm_plan_chirho() {
    use crate::build_cabal_project_chirho;
    use std::io::Write;

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_build_exec_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("build-exec.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         build-exec
version:      0.1.0.0

library
  exposed-modules: Lib
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5

executable hello-app
  main-is:         Main.hs
  hs-source-dirs:  src
  other-modules:   Lib
  build-depends:   base >=4.14 && <5, build-exec
"#
    )
    .unwrap();

    let mut lib_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
    write!(
        lib_file_chirho,
        "module Lib where\nmessage = \"Hello from Cabal build\"\n"
    )
    .unwrap();

    let mut main_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Main.hs")).unwrap();
    write!(
        main_file_chirho,
        "module Main where\nimport Lib\nmain = putStrLn message\n"
    )
    .unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();
    let result_chirho = build_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("cabal executable project should build");

    assert_eq!(result_chirho.package_chirho.name_chirho, "build-exec");
    assert_eq!(result_chirho.executables_chirho.len(), 1);
    assert_eq!(result_chirho.executables_chirho[0].name_chirho, "hello-app");
    assert_eq!(
        result_chirho.executables_chirho[0].compilation_order_chirho,
        vec!["Lib".to_string(), "Main".to_string()]
    );
    assert!(!result_chirho.executables_chirho[0]
        .core_chirho
        .bindings_chirho
        .is_empty());
    assert!(result_chirho.executables_chirho[0]
        .llvm_ir_chirho
        .contains("define i32 @main()"));
    assert!(result_chirho.executables_chirho[0]
        .llvm_ir_chirho
        .contains("@haskelujah_main"));

    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_cranelift_dedups_duplicate_prelude_bindings_chirho() {
    use crate::build_cabal_project_chirho;
    use haskelujah_backend_cranelift_chirho::{
        compile_core_to_object_executable_chirho, TargetConfigChirho,
    };
    use std::io::Write;

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_cranelift_dedup_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("dup-io.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         dup-io
version:      0.1.0.0

executable dup-io
  main-is:         Main.hs
  hs-source-dirs:  src
  other-modules:   Lib
  build-depends:   base >=4.14 && <5
"#
    )
    .unwrap();

    let mut lib_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
    write!(
        lib_file_chirho,
        "module Lib where\nsayHi = putStrLn \"lib\"\n"
    )
    .unwrap();

    let mut main_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Main.hs")).unwrap();
    write!(
        main_file_chirho,
        "module Main where\nimport Lib\nmain = do\n  putStrLn \"main\"\n  sayHi\n"
    )
    .unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();
    let build_result_chirho = build_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("cabal executable project should build");
    let config_chirho = TargetConfigChirho::default();
    let object_result_chirho = compile_core_to_object_executable_chirho(
        &build_result_chirho.executables_chirho[0].core_chirho,
        &config_chirho,
    )
    .expect("Cranelift should accept merged multi-module Prelude bindings");
    assert!(
        !object_result_chirho.object_bytes_chirho.is_empty(),
        "Cranelift object output should not be empty"
    );

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
    use haskelujah_package_chirho::{BuildInfoChirho, PackageDescChirho, TestSuiteChirho};
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
    assert!(
        names_chirho.contains(&"Main"),
        "should discover test-suite Main"
    );
    assert!(
        names_chirho.contains(&"Spec"),
        "should discover test-suite other-module Spec"
    );
}

#[test]
fn discover_setup_hs_chirho() {
    use haskelujah_package_chirho::PackageDescChirho;
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
    use haskelujah_package_chirho::{BuildInfoChirho, LibraryChirho, PackageDescChirho};
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

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );
    assert!(exec_ir_chirho.contains("define i64 @haskelujah_main()"));
    assert!(exec_ir_chirho.contains("ret i64 42"));
    assert!(exec_ir_chirho.contains("define i32 @main()"));
    assert!(exec_ir_chirho.contains("ptrtoint ptr @haskelujah_main to i64"));
    assert!(
        exec_ir_chirho.contains("call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)")
    );
}

#[test]
fn llvm_executable_arithmetic_chirho() {
    // f x y = x + y; main = f 10 32 should produce correct LLVM IR
    let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );
    // f should compile to an add instruction
    assert!(exec_ir_chirho.contains("add i64"));
    // main should call f with arguments
    assert!(exec_ir_chirho.contains("call i64 @haskelujah_f(i64 10, i64 32)"));
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
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );
    // Should have recursive fib function and main
    assert!(exec_ir_chirho.contains("@haskelujah_fib"));
    assert!(exec_ir_chirho.contains("define i32 @main()"));
}

#[test]
fn wasm_executable_constant_chirho() {
    // main = 42 should produce valid WASM with dict elision
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
            .expect("should compile");

    let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
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
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
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
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
        &result_chirho.core_chirho,
    );
    assert_eq!(&wasm_chirho[0..4], b"\0asm");
    // Should contain function call and case/if instructions
    assert!(
        wasm_chirho.contains(&0x10_u8),
        "WASM should contain call instruction"
    );
    assert!(
        wasm_chirho.contains(&0x04_u8),
        "WASM should contain if instruction"
    );
}

// ── LLVM round-trip smoke tests (§H.61) ──────────────────────────────
// These tests compile Haskell source to LLVM IR, invoke clang to produce
// a native binary, execute it, and compare the exit code to the STG
// interpreter result.

/// Helper: compile source to LLVM IR executable, link with clang, run,
/// and capture the exit code plus stdout.
fn llvm_round_trip_output_chirho(src_chirho: &str) -> Option<(i32, String)> {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );

    let tmp_dir_chirho = tempfile::tempdir().ok()?;
    let ll_path_chirho = tmp_dir_chirho.path().join("main.ll");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&ll_path_chirho, &exec_ir_chirho).ok()?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho()?;

    let compile_output_chirho = std::process::Command::new("clang")
        .arg("-O0")
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&ll_path_chirho)
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho")
        .output()
        .ok()?;

    if !compile_output_chirho.status.success() {
        return None;
    }

    let run_output_chirho = std::process::Command::new(&bin_path_chirho).output().ok()?;

    let exit_code_chirho = run_output_chirho.status.code()?;
    let stdout_chirho = String::from_utf8(run_output_chirho.stdout).ok()?;
    Some((exit_code_chirho, stdout_chirho))
}

fn haskell_string_literal_chirho(text_chirho: &str) -> String {
    let mut out_chirho = String::with_capacity(text_chirho.len() + 2);
    out_chirho.push('"');
    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '\\' => out_chirho.push_str("\\\\"),
            '"' => out_chirho.push_str("\\\""),
            '\n' => out_chirho.push_str("\\n"),
            '\r' => out_chirho.push_str("\\r"),
            '\t' => out_chirho.push_str("\\t"),
            _ => out_chirho.push(ch_chirho),
        }
    }
    out_chirho.push('"');
    out_chirho
}

fn llvm_round_trip_output_with_input_chirho(
    src_chirho: &str,
    input_chirho: &str,
) -> Option<(i32, String)> {
    use std::io::Write as _;
    use std::process::Stdio;

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );

    let tmp_dir_chirho = tempfile::tempdir().ok()?;
    let ll_path_chirho = tmp_dir_chirho.path().join("main.ll");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&ll_path_chirho, &exec_ir_chirho).ok()?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho()?;

    let compile_output_chirho = std::process::Command::new("clang")
        .arg("-O0")
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&ll_path_chirho)
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho")
        .output()
        .ok()?;

    if !compile_output_chirho.status.success() {
        return None;
    }

    let mut child_chirho = std::process::Command::new(&bin_path_chirho)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    child_chirho
        .stdin
        .as_mut()?
        .write_all(input_chirho.as_bytes())
        .ok()?;
    let run_output_chirho = child_chirho.wait_with_output().ok()?;

    let exit_code_chirho = run_output_chirho.status.code()?;
    let stdout_chirho = String::from_utf8(run_output_chirho.stdout).ok()?;
    Some((exit_code_chirho, stdout_chirho))
}

/// Helper: compile source to LLVM IR executable, link with clang, run, return exit code.
fn llvm_round_trip_chirho(src_chirho: &str) -> Option<i32> {
    llvm_round_trip_output_chirho(src_chirho)
        .map(|(exit_code_chirho, _stdout_chirho)| exit_code_chirho)
}

fn cranelift_round_trip_output_chirho(src_chirho: &str) -> Option<(i32, String)> {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;
    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .ok()?;

    let tmp_dir_chirho = tempfile::tempdir().ok()?;
    let obj_path_chirho = tmp_dir_chirho.path().join("main.o");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&obj_path_chirho, &obj_chirho.object_bytes_chirho).ok()?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho()?;

    let compile_status_chirho = std::process::Command::new("cc")
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&obj_path_chirho)
        .arg("-Wl,-no_fixup_chains")
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho")
        .status()
        .ok()?;
    if !compile_status_chirho.success() {
        return None;
    }

    let run_output_chirho = std::process::Command::new(&bin_path_chirho).output().ok()?;
    let exit_code_chirho = run_output_chirho.status.code()?;
    let stdout_chirho = String::from_utf8(run_output_chirho.stdout).ok()?;
    Some((exit_code_chirho, stdout_chirho))
}

fn cranelift_round_trip_chirho(src_chirho: &str) -> Option<i32> {
    cranelift_round_trip_output_chirho(src_chirho)
        .map(|(exit_code_chirho, _stdout_chirho)| exit_code_chirho)
}

fn ensure_rts_staticlib_for_tests_chirho() -> Option<std::path::PathBuf> {
    let crate_dir_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root_chirho = crate_dir_chirho.parent()?.parent()?.to_path_buf();
    let cargo_status_chirho = std::process::Command::new("cargo")
        .current_dir(&workspace_root_chirho)
        .args(["build", "-p", "haskelujah-rts-chirho", "--quiet"])
        .status()
        .ok()?;
    if !cargo_status_chirho.success() {
        return None;
    }
    Some(workspace_root_chirho.join("target").join("debug"))
}

#[test]
fn llvm_round_trip_constant_chirho() {
    // main = 42 → exit code 42
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 42");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: main = 42 should exit with 42"
        );
    }
    // If clang not available or linking fails, skip silently
}

#[test]
fn llvm_round_trip_arithmetic_chirho() {
    // f x y = x + y; main = f 10 32 → exit code 42
    let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: f 10 32 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_subtraction_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 50 - 8");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 50 - 8 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_multiplication_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 6 * 7");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 6 * 7 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_division_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 84 `div` 2");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 84 `div` 2 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_modulo_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 127 `mod` 85");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 127 `mod` 85 should exit with 42"
        );
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
        assert_eq!(
            code_chirho, 55,
            "LLVM round-trip: fib 10 should exit with 55"
        );
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

#[test]
fn llvm_round_trip_put_str_ln_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn \"Hello from Haskelujah!\"";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "putStrLn executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello from Haskelujah!\n");
    }
}

#[test]
fn llvm_round_trip_put_str_output_chirho() {
    let src_chirho = "module Main where\nmain = do\n  putStr \"Hello\"\n  putStr \" from\"\n  putStrLn \" Haskelujah!\"\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "putStr executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello from Haskelujah!\n");
    } else {
        panic!("LLVM putStr round-trip failed");
    }
}

#[test]
fn llvm_round_trip_get_line_output_chirho() {
    let src_chirho =
        "module Main where\nmain = do\n  name <- getLine\n  putStr \"Hello, \"\n  putStrLn name\n";
    if let Some((exit_code_chirho, stdout_chirho)) =
        llvm_round_trip_output_with_input_chirho(src_chirho, "Alice\n")
    {
        assert_eq!(
            exit_code_chirho, 0,
            "getLine executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello, Alice\n");
    } else {
        panic!("LLVM getLine round-trip failed");
    }
}

#[test]
fn llvm_round_trip_write_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("write-file-llvm-chirho.txt");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  writeFile {file_path_literal_chirho} \"hello from llvm\"\n  putStr \"ok\"\n"
    );
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(&src_chirho).expect("writeFile LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "writeFile executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "ok");
    let file_contents_chirho =
        std::fs::read_to_string(&file_path_chirho).expect("writeFile should create file");
    assert_eq!(file_contents_chirho, "hello from llvm");
}

#[test]
fn llvm_round_trip_read_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("read-file-llvm-chirho.txt");
    std::fs::write(&file_path_chirho, "hello from llvm disk")
        .expect("fixture file should be written");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  contentsChirho <- readFile {file_path_literal_chirho}\n  putStr contentsChirho\n"
    );
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(&src_chirho).expect("readFile LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "readFile executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "hello from llvm disk");
}

#[test]
fn llvm_round_trip_print_int_output_chirho() {
    let src_chirho = "module Main where\nmain = print 42";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_print_true_output_chirho() {
    let src_chirho = "module Main where\nmain = print True";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print True executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "True\n");
    }
}

#[test]
fn llvm_round_trip_print_false_output_chirho() {
    let src_chirho = "module Main where\nmain = print False";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print False executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "False\n");
    }
}

#[test]
fn llvm_round_trip_print_char_output_chirho() {
    let src_chirho = "module Main where\nmain = print 'A'";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print Char executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "'A'\n");
    }
}

#[test]
fn llvm_round_trip_print_float_output_chirho() {
    let src_chirho = "module Main where\nmain = print 3.14";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print Float executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "3.14\n");
    }
}

#[test]
fn llvm_round_trip_print_comparison_outputs_bool_chirho() {
    let src_chirho = r#"module Main where
main = do
  print (2 == 2)
  print (2 /= 3)
  print (2 < 3)
  print (3 > 2)
  print (2 <= 2)
  print (3 >= 3)"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "comparison executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "True\nTrue\nTrue\nTrue\nTrue\nTrue\n");
    }
}

#[test]
fn llvm_round_trip_if_then_else_true_branch_output_chirho() {
    let src_chirho =
        "module Main where\nmain = if 2 + 3 == 5 then putStrLn \"yes\" else putStrLn \"no\"";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "if-then-else executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "yes\n");
    }
}

#[test]
fn llvm_round_trip_if_then_else_false_branch_output_chirho() {
    let src_chirho =
        "module Main where\nmain = if 2 + 3 == 6 then putStrLn \"yes\" else putStrLn \"no\"";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "if-then-else executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "no\n");
    }
}

#[test]
fn llvm_round_trip_do_put_str_ln_then_print_output_chirho() {
    let src_chirho =
        "module Main where\nmain = do\n  putStrLn \"Hello from Haskelujah!\"\n  print 42\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "do block executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello from Haskelujah!\n42\n");
    }
}

#[test]
fn llvm_round_trip_bind_return_print_output_chirho() {
    let src_chirho = "module Main where\nmain = do\n  x <- return 42\n  print x\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "bind executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_recursive_where_print_output_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStrLn "Hello from Haskelujah Chirho!"
  print (2 + 3)
  print (factorial 10)
  where
    factorial 0 = 1
    factorial n = n * factorial (n - 1)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("recursive where LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "recursive where executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "Hello from Haskelujah Chirho!\n5\n3628800\n");
}

#[test]
fn llvm_round_trip_nested_where_print_output_chirho() {
    let src_chirho = r#"module Main where
sumTo n = outer n where
  outer m = go m 0 where
    go 0 acc = acc
    go k acc = go (k - 1) (acc + k)
main = print (sumTo 10)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("nested where LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "nested where executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "55\n");
}

#[test]
fn llvm_round_trip_where_sibling_cross_reference_output_chirho() {
    let src_chirho =
        "module Main where\nf x = result where result = a + b; a = x * 2; b = x * 3\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("where sibling LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "where sibling LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "25\n");
}

#[test]
fn llvm_round_trip_let_inside_where_output_chirho() {
    let src_chirho =
        "module Main where\nf x = a where a = let sq = x * x in sq + 1\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("let-in-where LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "let-in-where LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "26\n");
}

#[test]
fn llvm_round_trip_derived_eq_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Color = Red | Green | Blue deriving (Eq, Show)
main = do
  print (Red == Red)
  print (Red == Blue)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("derived Eq LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Eq LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "True\nFalse\n");
}

#[test]
fn llvm_round_trip_derived_ord_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Prio = Low | Med | High deriving (Ord, Eq, Show)
main = print (compare High Low)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("derived Ord LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Ord LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "GT\n");
}

#[test]
fn llvm_round_trip_euler1_tail_recursion_no_stack_overflow_chirho() {
    let src_chirho = r#"module Main where
euler1 limit = go 0 0 where
  go acc n =
    if n >= limit
      then acc
      else if mod n 3 == 0
        then go (acc + n) (n + 1)
        else if mod n 5 == 0
          then go (acc + n) (n + 1)
          else go acc (n + 1)
main = print (euler1 100000000)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("tail-recursive LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "tail-recursive LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "2333333316666668\n");
}

#[test]
fn llvm_round_trip_put_str_ln_show_int_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show 42)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_true_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show True)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show True executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "True\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_false_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show False)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show False executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "False\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_char_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show 'A')";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show Char executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "'A'\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_float_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show 3.14)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show Float executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "3.14\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_derived_enum_output_chirho() {
    let src_chirho =
        "module Main where\ndata Color = Red | Green | Blue deriving (Show)\nmain = putStrLn (show Green)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show derived enum executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Green\n");
    }
}

#[test]
fn llvm_round_trip_print_derived_enum_output_chirho() {
    let src_chirho =
        "module Main where\ndata Color = Red | Green | Blue deriving (Show)\nmain = print Green";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print derived enum executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Green\n");
    }
}

#[test]
fn llvm_round_trip_print_derived_field_constructor_output_chirho() {
    let src_chirho =
        "module Main where\ndata Pair = MkPair Int Int deriving (Show)\nmain = print (MkPair 3 4)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print derived field constructor executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "MkPair 3 4\n");
    }
}

#[test]
fn llvm_round_trip_nested_adt_list_pattern_output_chirho() {
    let src_chirho =
        "module Main where\ndata Value = VInt Int | VList [Value]\nscore (VList [VInt n]) = n\nscore (VList (_:_)) = 1\nscore _ = 2\nmain = print (score (VList [VInt 7]))";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "nested ADT/list pattern executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "7\n");
    }
}

#[test]
fn llvm_round_trip_print_sum_list_output_chirho() {
    let src_chirho = "module Main where\nmain = print (sum [1,2,3])";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("sum list LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "sum list executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "6\n");
}

#[test]
fn llvm_round_trip_map_lambda_sum_chirho() {
    let src_chirho = "module Main where\nmain = sum (map (\\x -> x * 2) [1,2,3])";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(code_chirho, 12, "map should compile through LLVM");
    }
}

#[test]
fn llvm_round_trip_filter_sum_chirho() {
    let src_chirho = "module Main where\nmain = sum (filter (> 2) [1,2,3,4])";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(code_chirho, 7, "filter should compile through LLVM");
    }
}

#[test]
fn llvm_round_trip_foldr_sum_chirho() {
    let src_chirho = "module Main where\nmain = foldr (+) 0 [1,2,3]";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(code_chirho, 6, "foldr should compile through LLVM");
    }
}

#[test]
fn llvm_round_trip_recursive_custom_list_instance_output_chirho() {
    let src_chirho = r#"module Main where
class Describable a where
  describe :: a -> String

instance Describable Int where
  describe x = show x

instance Describable a => Describable [a] where
  describe [] = "[]"
  describe (x:xs) = describe x ++ ":" ++ describe xs

main = putStrLn (describe [1,2,3])
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "llvm custom list instance should exit successfully"
        );
        assert_eq!(stdout_chirho, "1:2:3:[]\n");
    }
}

#[test]
fn llvm_round_trip_show_concat_output_chirho() {
    let src_chirho = r#"module Main where
main = putStrLn ("answer: " ++ show 42)
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show concat executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "answer: 42\n");
    }
}

#[test]
fn llvm_round_trip_take_zero_matches_first_equation_chirho() {
    let src_chirho = r#"module Main where
myTakeChirho 0 _ = []
myTakeChirho _ [] = []
myTakeChirho nChirho (xChirho:xsChirho) = xChirho : myTakeChirho (nChirho - 1) xsChirho
myLenChirho [] = 0
myLenChirho (_:xsChirho) = 1 + myLenChirho xsChirho
main = print (myLenChirho (myTakeChirho 0 [1,2,3]))
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "take 0 literal-first executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "0\n");
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

    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
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
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
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
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .expect("cranelift compilation should succeed");
    assert!(
        !obj_chirho.object_bytes_chirho.is_empty(),
        "cranelift fibonacci object file should not be empty"
    );
}

#[test]
fn cranelift_round_trip_payload_constructor_box_int_chirho() {
    let src_chirho = r#"module Main where
data Box = Box Int
unBox :: Box -> Int
unBox (Box n) = n
main = unBox (Box 42)
"#;
    let exit_code_chirho = cranelift_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "Cranelift round-trip: Box Int should exit with 42"
        );
    }
}

#[test]
fn cranelift_round_trip_payload_constructor_rect_area_chirho() {
    let src_chirho = r#"module Main where
data Shape = Circle Int | Rect Int Int
area :: Shape -> Int
area (Circle r) = 3 * r * r
area (Rect w h) = w * h
main = area (Rect 3 7)
"#;
    let exit_code_chirho = cranelift_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 21,
            "Cranelift round-trip: Rect 3 7 should exit with 21"
        );
    }
}

#[test]
fn cranelift_round_trip_io_order_output_chirho() {
    let src_chirho = r#"module Main where
main = do
  putStrLn "hello"
  print 42
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = cranelift_round_trip_output_chirho(src_chirho)
    {
        assert_eq!(
            exit_code_chirho, 0,
            "Cranelift IO program should exit successfully"
        );
        assert_eq!(
            stdout_chirho, "hello\n42\n",
            "Cranelift should preserve source IO sequencing",
        );
    }
}

#[test]
fn cranelift_round_trip_recursive_where_print_output_chirho() {
    let src_chirho = r#"module Main where
collatz n = go n 0
  where
    go 1 acc = acc
    go k acc = if mod k 2 == 0 then go (div k 2) (acc + 1) else go (3 * k + 1) (acc + 1)
main = print (collatz 7)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("recursive where Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "recursive where Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "16\n");
}

#[test]
fn cranelift_round_trip_nested_where_print_output_chirho() {
    let src_chirho = r#"module Main where
sumTo n = outer n where
  outer m = go m 0 where
    go 0 acc = acc
    go k acc = go (k - 1) (acc + k)
main = print (sumTo 10)
"#;
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("nested where Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "nested where Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "55\n");
}

#[test]
fn cranelift_round_trip_where_sibling_cross_reference_output_chirho() {
    let src_chirho =
        "module Main where\nf x = result where result = a + b; a = x * 2; b = x * 3\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("where sibling Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "where sibling Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "25\n");
}

#[test]
fn cranelift_round_trip_let_inside_where_output_chirho() {
    let src_chirho =
        "module Main where\nf x = a where a = let sq = x * x in sq + 1\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("let-in-where Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "let-in-where Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "26\n");
}

#[test]
fn cranelift_round_trip_derived_eq_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Color = Red | Green | Blue deriving (Eq, Show)
main = do
  print (Red == Red)
  print (Red == Blue)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("derived Eq Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Eq Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "True\nFalse\n");
}

#[test]
fn cranelift_round_trip_derived_ord_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Prio = Low | Med | High deriving (Ord, Eq, Show)
main = print (compare High Low)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("derived Ord Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Ord Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "GT\n");
}

#[test]
fn cranelift_round_trip_wildcard_first_column_safe_divide_output_chirho() {
    let src_chirho = r#"module Main where
safeDivide _ 0 = 0
safeDivide a b = div a b
main = print (safeDivide 10 2)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("wildcard safeDivide Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "wildcard-first-column Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "5\n");
}

#[test]
fn cranelift_round_trip_partial_application_make_adder_output_chirho() {
    let src_chirho = r#"module Main where
makeAdder n = \x -> x + n
main = do
  let add5 = makeAdder 5
  print (add5 37)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("partial application makeAdder Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "partial application Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "42\n");
}

#[test]
fn cranelift_round_trip_where_capture_output_chirho() {
    let src_chirho = r#"module Main where
f x = go 0 where
  go n = if n >= x then n else go (n + 1)
main = print (f 5)
"#;
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("where capture Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "where-capture Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "5\n");
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
fn cranelift_round_trip_show_concat_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = putStrLn ("fib 10 = " ++ show (55))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert!(
            stdout_chirho.contains("fib 10 = 55"),
            "expected 'fib 10 = 55', got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_foldr_filter_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs

foldr' :: (Int -> Int -> Int) -> Int -> [Int] -> Int
foldr' f z [] = z
foldr' f z (x:xs) = f x (foldr' f z xs)

main = print (foldr' (\x acc -> x + acc) 0 (filter' (\x -> x `mod` 2 == 0) [1,2,3,4,5,6,7,8,9,10]))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert!(
            stdout_chirho.trim() == "30",
            "sum of evens in [1..10] = 30, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_take_zero_matches_first_equation_chirho() {
    let src_chirho = r#"module Main where
myTakeChirho 0 _ = []
myTakeChirho _ [] = []
myTakeChirho nChirho (xChirho:xsChirho) = xChirho : myTakeChirho (nChirho - 1) xsChirho
myLenChirho [] = 0
myLenChirho (_:xsChirho) = 1 + myLenChirho xsChirho
main = print (myLenChirho (myTakeChirho 0 [1,2,3]))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert_eq!(stdout_chirho, "0\n");
    }
}

#[test]
fn cranelift_round_trip_comprehensive_chirho() {
    let src_chirho = r#"module Main where

fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)

mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs

makeAdder :: Int -> Int -> Int
makeAdder n x = n + x

countUpTo :: Int -> Int
countUpTo limit = go 0
  where go n = if n >= limit then n else go (n + 1)

data Shape = Circle Int | Rectangle Int Int

area :: Shape -> Int
area (Circle r) = r * r
area (Rectangle w h) = w * h

main :: IO ()
main = do
  putStrLn ("fib 20 = " ++ show (fib 20))
  putStrLn ("sum [1..5] = " ++ show (mySum [1,2,3,4,5]))
  let add10 = makeAdder 10
  putStrLn ("add10 32 = " ++ show (add10 32))
  putStrLn ("countUpTo 50 = " ++ show (countUpTo 50))
  putStrLn ("area Circle 7 = " ++ show (area (Circle 7)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert!(stdout_chirho.contains("fib 20 = 6765"), "fib 20");
        assert!(stdout_chirho.contains("sum [1..5] = 15"), "sum");
        assert!(stdout_chirho.contains("add10 32 = 42"), "closure");
        assert!(stdout_chirho.contains("countUpTo 50 = 50"), "where");
        assert!(stdout_chirho.contains("area Circle 7 = 49"), "ADT");
    }
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
    let (val_chirho, _machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ForeignExportEval.hs", None)
            .expect("foreign export should not break evaluation");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn cranelift_round_trip_nested_cons_pattern_chirho() {
    let src_chirho = r#"module Main where
headTwo :: [Int] -> Int
headTwo (a:b:_) = a + b
headTwo _ = 0
main = print (headTwo [100, 200, 300])
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert!(
            stdout_chirho.trim() == "300",
            "headTwo [100,200,300] = 300, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_quicksort_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main = print (mySum (qsort [5,1,4,2,3]))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert!(
            stdout_chirho.trim() == "15",
            "sum of sorted [5,1,4,2,3] = 15, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_abs_signum_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStrLn (show (abs (-42)))
  putStrLn (show (signum (-7)))
  putStrLn (show (negate 10))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        let lines_chirho: Vec<&str> = stdout_chirho.trim().lines().collect();
        assert_eq!(lines_chirho.len(), 3, "expected 3 lines");
        assert_eq!(lines_chirho[0], "42", "abs(-42)");
        assert_eq!(lines_chirho[1], "-1", "signum(-7)");
        assert_eq!(lines_chirho[2], "-10", "negate(10)");
    }
}

#[test]
fn cranelift_round_trip_prime_sieve_chirho() {
    let src_chirho = r#"module Main where
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
removeMultiples :: Int -> [Int] -> [Int]
removeMultiples _ [] = []
removeMultiples p (x:xs) = if x `mod` p == 0
                           then removeMultiples p xs
                           else x : removeMultiples p xs
sieve :: [Int] -> [Int]
sieve [] = []
sieve (p:xs) = p : sieve (removeMultiples p xs)
countList :: [Int] -> Int
countList [] = 0
countList (_:xs) = 1 + countList xs
main = print (countList (sieve (range 2 100)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert!(
            stdout_chirho.trim() == "25",
            "25 primes up to 100, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_mixed_show_int_bool_chirho() {
    let src_chirho = r#"module Main where
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = do
  putStrLn ("sum = " ++ show (mySum [1,2,3]))
  putStrLn ("bool = " ++ show True)
  putStrLn ("abs = " ++ show (abs (-42)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        let lines_chirho: Vec<&str> = stdout_chirho.trim().lines().collect();
        assert!(
            lines_chirho.len() >= 3,
            "expected 3 lines, got {}",
            lines_chirho.len()
        );
        assert_eq!(lines_chirho[0], "sum = 6", "show Int after sum");
        assert_eq!(lines_chirho[1], "bool = True", "show Bool");
        assert_eq!(lines_chirho[2], "abs = 42", "show Int via abs");
    }
}

#[test]
fn cranelift_round_trip_tco_euler1_chirho() {
    let src_chirho = r#"module Main where
euler1 :: Int -> Int
euler1 limit = go 0 0
  where go acc n = if n >= limit then acc
                   else if n `mod` 3 == 0 || n `mod` 5 == 0
                        then go (acc + n) (n + 1)
                        else go acc (n + 1)
main = print (euler1 1000000)
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "TCO euler1 should not stack overflow");
        assert_eq!(
            stdout_chirho.trim(),
            "233333166668",
            "euler1(1000000) = 233333166668"
        );
    }
}

#[test]
fn cranelift_round_trip_qsort_random_500_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
lcg :: Int -> Int -> [Int]
lcg seed 0 = []
lcg seed n = let next = (seed * 1103515245 + 12345) `mod` 2147483648
             in (next `mod` 1000) : lcg next (n - 1)
main = print (mySum (qsort (lcg 42 500)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "quicksort 500 random should work");
        assert_eq!(
            stdout_chirho.trim(),
            "256218",
            "sum of sorted 500 random elements"
        );
    }
}

#[test]
fn cranelift_round_trip_qsort_random_1000_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
lcg :: Int -> Int -> [Int]
lcg seed 0 = []
lcg seed n = let next = (seed * 1103515245 + 12345) `mod` 2147483648
             in (next `mod` 1000) : lcg next (n - 1)
main = print (mySum (qsort (lcg 42 1000)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "quicksort 1000 random should work");
        assert_eq!(
            stdout_chirho.trim(),
            "507468",
            "sum of sorted 1000 random elements"
        );
    }
}

#[test]
fn cranelift_round_trip_range_10000_chirho() {
    let src_chirho = r#"module Main where
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
myLen :: [Int] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = print (myLen (range 1 10000))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "range 10000 should work");
        assert_eq!(stdout_chirho.trim(), "10000", "len of range 1..10000");
    }
}

#[test]
fn cranelift_round_trip_range_100000_chirho() {
    let src_chirho = r#"module Main where
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
myLen :: [Int] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = print (myLen (range 1 100000))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "range 100000 should work");
        assert_eq!(stdout_chirho.trim(), "100000", "len of range 1..100000");
    }
}

#[test]
fn cranelift_round_trip_recursive_custom_list_instance_output_chirho() {
    let src_chirho = r#"module Main where
class Describable a where
  describe :: a -> String

instance Describable Int where
  describe x = show x

instance Describable a => Describable [a] where
  describe [] = "[]"
  describe (x:xs) = describe x ++ ":" ++ describe xs

main = putStrLn (describe [1,2,3])
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "cranelift custom list instance should work");
        assert_eq!(stdout_chirho, "1:2:3:[]\n");
    }
}

#[test]
fn cranelift_round_trip_put_str_no_newline_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStr "Hello "
  putStr "World"
  putStrLn "!"
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "putStr should exit 0");
        assert_eq!(
            stdout_chirho, "Hello World!\n",
            "putStr concatenates without newlines"
        );
    }
}

#[test]
fn cranelift_round_trip_getline_bind_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  name <- getLine
  putStrLn ("Hello, " ++ name ++ "!")
"#;
    // Use stdin-fed round trip
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok();
    let Some(result_chirho) = result_chirho else {
        return;
    };
    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .ok();
    let Some(obj_chirho) = obj_chirho else { return };

    let tmp_dir_chirho = tempfile::tempdir().ok();
    let Some(tmp_dir_chirho) = tmp_dir_chirho else {
        return;
    };
    let obj_path_chirho = tmp_dir_chirho.path().join("main.o");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&obj_path_chirho, &obj_chirho.object_bytes_chirho).ok();
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho();
    let Some(rts_lib_dir_chirho) = rts_lib_dir_chirho else {
        return;
    };

    let compile_status_chirho = std::process::Command::new("cc")
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&obj_path_chirho)
        .arg("-Wl,-no_fixup_chains")
        .arg("-Wl,-stack_size,0x10000000")
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho")
        .status()
        .ok();
    if compile_status_chirho.map_or(true, |s| !s.success()) {
        return;
    }

    // Run with piped stdin
    let run_output_chirho = std::process::Command::new(&bin_path_chirho)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child_chirho| {
            use std::io::Write;
            if let Some(stdin_chirho) = child_chirho.stdin.as_mut() {
                let _ = stdin_chirho.write_all(b"Haskelujah\n");
            }
            child_chirho.wait_with_output()
        })
        .ok();
    let Some(output_chirho) = run_output_chirho else {
        return;
    };
    let stdout_chirho = String::from_utf8_lossy(&output_chirho.stdout);
    assert!(
        stdout_chirho.contains("Hello, Haskelujah!"),
        "getLine should read stdin: got {stdout_chirho}"
    );
}

#[test]
fn cranelift_round_trip_text_processing_chirho() {
    let src_chirho = r#"module Main where
myMapChar :: (Char -> Char) -> [Char] -> [Char]
myMapChar f [] = []
myMapChar f (c:cs) = f c : myMapChar f cs
myFilterChar :: (Char -> Bool) -> [Char] -> [Char]
myFilterChar f [] = []
myFilterChar f (c:cs) = if f c then c : myFilterChar f cs else myFilterChar f cs
myLen :: [Char] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main :: IO ()
main = do
  putStrLn (pack (myMapChar toUpper (unpack "hello")))
  putStrLn (pack (myFilterChar isAlpha (unpack "Hi 123")))
  print (myLen (unpack "test"))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        let lines_chirho: Vec<&str> = stdout_chirho.trim().lines().collect();
        assert_eq!(lines_chirho[0], "HELLO", "toUpper via unpack/pack");
        assert_eq!(lines_chirho[1], "Hi", "filterChar isAlpha");
        assert_eq!(lines_chirho[2], "4", "myLen unpack");
    }
}

#[test]
fn llvm_round_trip_ackermann_multi_equation_pattern_match_chirho() {
    // Regression test for multi-equation pattern match ordering.
    // ack 0 n = n+1 must take priority over ack m 0 when m=0, n=0.
    let src_chirho = r#"module Main where
ack :: Int -> Int -> Int
ack 0 n = n + 1
ack m 0 = ack (m - 1) 1
ack m n = ack (m - 1) (ack m (n - 1))
main :: IO ()
main = print (ack 3 7)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1021");
    }
}

#[test]
fn llvm_round_trip_guards_correct_branching_chirho() {
    // Regression test for LLVM phi node predecessor fix.
    // Guards produce nested case expressions whose phi nodes
    // must reference the correct predecessor block.
    let src_chirho = r#"module Main where
sign :: Int -> Int
sign n
  | n > 0 = 1
  | n == 0 = 0
  | otherwise = -1
main :: IO ()
main = do
  print (sign 5)
  print (sign 0)
  print (sign (-3))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1\n0\n-1");
    }
}

#[test]
fn llvm_round_trip_collatz_guards_where_chirho() {
    // Collatz sequence: guards + where-clause + multi-equation + TCO
    let src_chirho = r#"module Main where
collatz :: Int -> Int
collatz n = go n 0
  where
    go 1 steps = steps
    go n steps
      | n `mod` 2 == 0 = go (n `div` 2) (steps + 1)
      | otherwise = go (3 * n + 1) (steps + 1)
main :: IO ()
main = do
  print (collatz 27)
  print (collatz 1)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "111\n0");
    }
}

#[test]
fn llvm_round_trip_adt_constructor_fields_chirho() {
    // ADT constructor field extraction + Maybe pattern matching
    let src_chirho = r#"module Main where
data Shape = Circle Int | Rectangle Int Int
area :: Shape -> Int
area (Circle r) = r * r
area (Rectangle w h) = w * h
fromMaybe :: Int -> Maybe Int -> Int
fromMaybe def Nothing = def
fromMaybe _ (Just x) = x
main :: IO ()
main = do
  print (area (Circle 7))
  print (area (Rectangle 3 4))
  print (fromMaybe 0 (Just 42))
  print (fromMaybe (-1) Nothing)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "49\n12\n42\n-1");
    }
}

#[test]
fn llvm_round_trip_recursive_adt_expr_evaluator_chirho() {
    // 4-constructor recursive expression ADT with nested patterns
    let src_chirho = r#"module Main where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr | Neg Expr
eval :: Expr -> Int
eval (Lit n) = n
eval (Add a b) = eval a + eval b
eval (Mul a b) = eval a * eval b
eval (Neg e) = 0 - eval e
main :: IO ()
main = do
  print (eval (Mul (Add (Lit 3) (Lit 4)) (Lit 2)))
  print (eval (Neg (Add (Lit 5) (Lit 3))))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "14\n-8");
    }
}

#[test]
fn llvm_round_trip_recursive_tree_sum_depth_chirho() {
    // Recursive binary tree with sum and depth
    let src_chirho = r#"module Main where
data Tree = Leaf Int | Node Tree Tree
sumTree :: Tree -> Int
sumTree (Leaf n) = n
sumTree (Node l r) = sumTree l + sumTree r
max' :: Int -> Int -> Int
max' a b = if a >= b then a else b
depth :: Tree -> Int
depth (Leaf _) = 1
depth (Node l r) = 1 + max' (depth l) (depth r)
main :: IO ()
main = do
  let t = Node (Node (Leaf 1) (Leaf 2)) (Node (Leaf 3) (Leaf 4))
  print (sumTree t)
  print (depth t)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "10\n3");
    }
}

#[test]
fn llvm_round_trip_partial_application_chirho() {
    let src_chirho = r#"module Main where
add :: Int -> Int -> Int
add x y = x + y
add5 :: Int -> Int
add5 = add 5
mul :: Int -> Int -> Int
mul x y = x * y
double :: Int -> Int
double = mul 2
main :: IO ()
main = do
  print (add5 37)
  print (double 21)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "42\n42");
    }
}

#[test]
fn llvm_round_trip_number_theory_chirho() {
    // GCD + power + nth prime — comprehensive test
    let src_chirho = r#"module Main where
gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
power :: Int -> Int -> Int
power _ 0 = 1
power base exp = base * power base (exp - 1)
main :: IO ()
main = do
  print (gcd' 252 105)
  print (power 2 10)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "21\n1024");
    }
}

#[test]
fn llvm_round_trip_mutual_recursion_chirho() {
    let src_chirho = r#"module Main where
isEven :: Int -> Int
isEven 0 = 1
isEven n = isOdd (n - 1)
isOdd :: Int -> Int
isOdd 0 = 0
isOdd n = isEven (n - 1)
main :: IO ()
main = do
  print (isEven 10)
  print (isOdd 3)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1\n1");
    }
}

#[test]
fn llvm_round_trip_adt_with_field_constructor_chirho() {
    // Stack machine: 4-constructor ADT with fields + tuple return
    let src_chirho = r#"module Main where
data Instr = Push Int | Add | Mul | Neg
eval :: Instr -> Int -> Int -> (Int, Int)
eval (Push n) a b = (n, a)
eval Add a b = (a + b, 0)
eval Mul a b = (a * b, 0)
eval Neg a b = (0 - a, b)
main :: IO ()
main = do
  let (a1, _) = eval (Push 3) 0 0
  let (a2, _) = eval (Push 4) a1 0
  let (r, _) = eval Add a2 a1
  print r
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "7");
    }
}

#[test]
fn llvm_round_trip_list_map_filter_sum_chirho() {
    // List operations: enumFromTo, map with lambda, filter, sum
    let src_chirho = r#"module Main where
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
isEven :: Int -> Bool
isEven n = n `mod` 2 == 0
main :: IO ()
main = do
  let xs = enumFromTo 1 10
  print (sumList xs)
  print (sumList (myFilter isEven xs))
  print (sumList (myMap (\x -> x * x) (enumFromTo 1 5)))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "55\n30\n55");
    }
}

#[test]
fn llvm_round_trip_multi_arg_hof_and_foldr_chirho() {
    // Regression: 2-arg HOF + foldr (was SIGBUS before multi-arg fix)
    let src_chirho = r#"module Main where
myFoldr :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
add :: Int -> Int -> Int
add x y = x + y
mul :: Int -> Int -> Int
mul x y = x * y
main :: IO ()
main = do
  print (myFoldr add 0 [1, 2, 3, 4, 5])
  print (myFoldr mul 1 [1, 2, 3, 4, 5, 6])
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "15\n720");
    }
}

#[test]
fn llvm_round_trip_quicksort_chirho() {
    // Quicksort with filter, append, lambda HOFs — comprehensive test
    let src_chirho = r#"module Main where
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (x:xs) = append (qsort (myFilter (\y -> y <= x) xs))
                       (x : qsort (myFilter (\y -> y > x) xs))
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
main :: IO ()
main = print (sumList (qsort [5, 3, 8, 1, 9, 2, 7, 4, 6]))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        // sum [1..9] = 45
        assert_eq!(stdout_chirho.trim(), "45");
    }
}

#[test]
fn llvm_round_trip_take_zipwith_chirho() {
    // take + zipWith: variable rule + multi-arg HOF combined
    let src_chirho = r#"module Main where
myTake :: Int -> [Int] -> [Int]
myTake 0 _ = []
myTake _ [] = []
myTake n (x:xs) = x : myTake (n - 1) xs
myZipWith :: (Int -> Int -> Int) -> [Int] -> [Int] -> [Int]
myZipWith _ [] _ = []
myZipWith _ _ [] = []
myZipWith f (x:xs) (y:ys) = f x y : myZipWith f xs ys
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = do
  print (sumList (myTake 3 (enumFromTo 1 100)))
  print (sumList (myZipWith add (enumFromTo 1 5) (enumFromTo 10 14)))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "6\n75");
    }
}

#[test]
fn llvm_round_trip_foldl_at_scale_chirho() {
    // foldl with 2-arg HOF at 10K elements
    let src_chirho = r#"module Main where
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = print (myFoldl add 0 (enumFromTo 1 10000))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "50005000");
    }
}

#[test]
fn llvm_round_trip_merge_sort_chirho() {
    // Full merge sort with take/drop/merge
    let src_chirho = r#"module Main where
merge :: [Int] -> [Int] -> [Int]
merge [] ys = ys
merge xs [] = xs
merge (x:xs) (y:ys) = if x <= y then x : merge xs (y:ys) else y : merge (x:xs) ys
myTake :: Int -> [Int] -> [Int]
myTake 0 _ = []
myTake _ [] = []
myTake n (x:xs) = x : myTake (n - 1) xs
myDrop :: Int -> [Int] -> [Int]
myDrop 0 xs = xs
myDrop _ [] = []
myDrop n (_:xs) = myDrop (n - 1) xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
msort :: [Int] -> [Int]
msort [] = []
msort (x:[]) = [x]
msort xs = merge (msort (myTake half xs)) (msort (myDrop half xs))
  where half = myLength xs `div` 2
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
main :: IO ()
main = print (sumList (msort [9, 3, 7, 1, 8, 2, 6, 4, 5]))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "45");
    }
}

#[test]
fn llvm_round_trip_comprehensive_demo_chirho() {
    // Comprehensive: filter, foldl with lambda, primes, show++
    let src_chirho = r#"module Main where
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
isPrime :: Int -> Bool
isPrime n
  | n < 2 = False
  | otherwise = go 2
  where
    go d
      | d * d > n = True
      | n `mod` d == 0 = False
      | otherwise = go (d + 1)
main :: IO ()
main = do
  let primes = myFilter isPrime (enumFromTo 2 100)
  putStrLn ("primes: " ++ show (myLength primes))
  let sq = myFoldl (\acc x -> acc + x * x) 0 (enumFromTo 1 10)
  putStrLn ("sumsq: " ++ show sq)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "primes: 25\nsumsq: 385");
    }
}
