// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Tests for hierarchical project compilation (compile_project_dir_chirho).

#[cfg(test)]
mod tests_chirho {
    use crate::{
        compile_project_dir_chirho, discover_hs_files_chirho,
        extract_imports_chirho, extract_module_name_chirho,
    };
    use rhasky_span_chirho::SourceMapChirho;
    use std::fs;

    #[test]
    fn extract_module_name_simple_chirho() {
        assert_eq!(
            extract_module_name_chirho("module Foo where\nmain = 42\n"),
            "Foo"
        );
    }

    #[test]
    fn extract_module_name_hierarchical_chirho() {
        assert_eq!(
            extract_module_name_chirho("module Data.List.Utils where\n"),
            "Data.List.Utils"
        );
    }

    #[test]
    fn extract_module_name_default_main_chirho() {
        assert_eq!(
            extract_module_name_chirho("main = putStrLn \"hello\"\n"),
            "Main"
        );
    }

    #[test]
    fn extract_module_name_with_exports_chirho() {
        assert_eq!(
            extract_module_name_chirho("module Foo (bar, baz) where\n"),
            "Foo"
        );
    }

    #[test]
    fn extract_imports_basic_chirho() {
        let src_chirho = "module Foo where\nimport Data.Map\nimport Data.List\nmain = 42\n";
        let imports_chirho = extract_imports_chirho(src_chirho);
        assert_eq!(imports_chirho, vec!["Data.Map", "Data.List"]);
    }

    #[test]
    fn extract_imports_qualified_chirho() {
        let src_chirho =
            "module Foo where\nimport qualified Data.Map as Map\nimport Bar\n";
        let imports_chirho = extract_imports_chirho(src_chirho);
        assert_eq!(imports_chirho, vec!["Data.Map", "Bar"]);
    }

    #[test]
    fn extract_imports_with_spec_chirho() {
        let src_chirho = "module Foo where\nimport Bar (baz, quux)\n";
        let imports_chirho = extract_imports_chirho(src_chirho);
        assert_eq!(imports_chirho, vec!["Bar"]);
    }

    #[test]
    fn discover_hs_files_in_temp_dir_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(tmp_chirho.path().join("Main.hs"), "module Main where\nmain = 42\n").unwrap();
        fs::write(tmp_chirho.path().join("Lib.hs"), "module Lib where\nfoo = 1\n").unwrap();
        let sub_chirho = tmp_chirho.path().join("Data");
        fs::create_dir_all(&sub_chirho).unwrap();
        fs::write(sub_chirho.join("Utils.hs"), "module Data.Utils where\nbar = 2\n").unwrap();

        let files_chirho = discover_hs_files_chirho(tmp_chirho.path());
        assert_eq!(files_chirho.len(), 3);
        // Files should be sorted
        let names_chirho: Vec<String> = files_chirho
            .iter()
            .map(|p_chirho| p_chirho.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert!(names_chirho.contains(&"Main.hs".to_string()));
        assert!(names_chirho.contains(&"Lib.hs".to_string()));
        assert!(names_chirho.contains(&"Utils.hs".to_string()));
    }

    #[test]
    fn compile_project_two_modules_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Lib.hs"),
            "module Lib where\nadd1 x = x + 1\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "module Main where\nimport Lib\nmain = add1 41\n",
        )
        .unwrap();

        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
                .expect("project compilation should succeed");
        assert_eq!(result_chirho.compilation_order_chirho.len(), 2);
        // Lib should be compiled before Main
        assert_eq!(result_chirho.compilation_order_chirho[0], "Lib");
        assert_eq!(result_chirho.compilation_order_chirho[1], "Main");
        assert_eq!(result_chirho.module_results_chirho.len(), 2);
    }

    #[test]
    fn compile_project_three_modules_chain_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Base.hs"),
            "module Base where\nval = 10\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Middle.hs"),
            "module Middle where\nimport Base\nbump x = x + val\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "module Main where\nimport Middle\nmain = bump 32\n",
        )
        .unwrap();

        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
                .expect("3-module chain should compile");
        assert_eq!(result_chirho.compilation_order_chirho.len(), 3);
        // Base → Middle → Main
        let base_idx_chirho = result_chirho
            .compilation_order_chirho
            .iter()
            .position(|n_chirho| n_chirho == "Base")
            .unwrap();
        let mid_idx_chirho = result_chirho
            .compilation_order_chirho
            .iter()
            .position(|n_chirho| n_chirho == "Middle")
            .unwrap();
        let main_idx_chirho = result_chirho
            .compilation_order_chirho
            .iter()
            .position(|n_chirho| n_chirho == "Main")
            .unwrap();
        assert!(base_idx_chirho < mid_idx_chirho);
        assert!(mid_idx_chirho < main_idx_chirho);
    }

    #[test]
    fn compile_project_diamond_deps_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Core.hs"),
            "module Core where\nbaseVal = 1\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Left.hs"),
            "module Left where\nimport Core\nleftVal = baseVal + 10\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Right.hs"),
            "module Right where\nimport Core\nrightVal = baseVal + 20\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "module Main where\nimport Left\nimport Right\nmain = leftVal + rightVal\n",
        )
        .unwrap();

        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
                .expect("diamond deps should compile");
        assert_eq!(result_chirho.compilation_order_chirho.len(), 4);
        // Core must come before Left and Right, which must come before Main
        let core_idx_chirho = result_chirho
            .compilation_order_chirho
            .iter()
            .position(|n_chirho| n_chirho == "Core")
            .unwrap();
        let main_idx_chirho = result_chirho
            .compilation_order_chirho
            .iter()
            .position(|n_chirho| n_chirho == "Main")
            .unwrap();
        assert!(core_idx_chirho < main_idx_chirho);
    }

    #[test]
    fn compile_project_circular_import_detected_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("A.hs"),
            "module A where\nimport B\na = 1\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("B.hs"),
            "module B where\nimport A\nb = 2\n",
        )
        .unwrap();

        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho);
        assert!(result_chirho.is_err());
        let err_chirho = result_chirho.unwrap_err();
        assert!(
            err_chirho.contains("Circular"),
            "Expected circular import error, got: {}",
            err_chirho
        );
    }

    #[test]
    fn compile_project_no_hs_files_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho);
        assert!(result_chirho.is_err());
        assert!(result_chirho.unwrap_err().contains("No .hs files"));
    }

    #[test]
    fn compile_project_single_module_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "module Main where\nmain = 42\n",
        )
        .unwrap();

        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho =
            compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
                .expect("single module should compile");
        assert_eq!(result_chirho.compilation_order_chirho, vec!["Main"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }
}
