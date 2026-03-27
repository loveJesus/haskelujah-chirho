// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Tests for hierarchical project compilation (compile_project_dir_chirho).

#[cfg(test)]
mod tests_chirho {
    use crate::{
        collect_frontend_artifacts_from_module_sources_chirho,
        compile_module_sources_with_extra_ifaces_chirho, compile_project_dir_chirho,
        discover_hs_files_chirho, extract_imports_chirho, extract_module_name_chirho,
        filter_seeded_imported_types_for_source_chirho,
        filter_seeded_type_synonyms_for_source_chirho,
    };
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    use haskelujah_span_chirho::SourceMapChirho;
    use haskelujah_typing_chirho::SchemeChirho;
    use haskelujah_typing_chirho::TyChirho;
    use std::fs;

    fn workspace_root_chirho() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path_chirho| path_chirho.parent())
            .unwrap()
            .to_path_buf()
    }

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
        let src_chirho = "module Foo where\nimport qualified Data.Map as Map\nimport Bar\n";
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
    fn filter_seeded_type_synonyms_only_keeps_explicit_modules_chirho() {
        let type_var_name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "a".to_string(),
            haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        ));
        let source_chirho = "module Main where\nimport Text.Parsec.String\nmain = 0\n";
        let imported_type_synonyms_chirho = std::collections::HashMap::from([
            (
                "Text.Parsec.String.GenParser".to_string(),
                (
                    vec!["tok".to_string(), "st".to_string()],
                    TypeChirho::VarChirho(type_var_name_chirho.clone()),
                ),
            ),
            (
                "Text.Parsec.Text.GenParser".to_string(),
                (
                    vec!["st".to_string()],
                    TypeChirho::VarChirho(type_var_name_chirho),
                ),
            ),
        ]);

        let filtered_synonyms_chirho = filter_seeded_type_synonyms_for_source_chirho(
            source_chirho,
            &imported_type_synonyms_chirho,
        );

        assert!(filtered_synonyms_chirho.contains_key("Text.Parsec.String.GenParser"));
        assert!(!filtered_synonyms_chirho.contains_key("Text.Parsec.Text.GenParser"));
    }

    #[test]
    fn filter_seeded_imported_types_only_keeps_explicit_modules_chirho() {
        let source_chirho = "module Main where\nimport Text.Parsec\nmain = 0\n";
        let imported_types_chirho = std::collections::HashMap::from([
            (
                "Text.Parsec.choice".to_string(),
                SchemeChirho::mono_chirho(TyChirho::ConChirho("ParsecChoiceChirho".to_string())),
            ),
            (
                "Text.Parsec.Text.choice".to_string(),
                SchemeChirho::mono_chirho(TyChirho::ConChirho("TextChoiceChirho".to_string())),
            ),
            (
                "choice".to_string(),
                SchemeChirho::mono_chirho(TyChirho::ConChirho(
                    "UnqualifiedChoiceChirho".to_string(),
                )),
            ),
        ]);

        let filtered_types_chirho =
            filter_seeded_imported_types_for_source_chirho(source_chirho, &imported_types_chirho);

        assert!(filtered_types_chirho.contains_key("Text.Parsec.choice"));
        assert!(!filtered_types_chirho.contains_key("Text.Parsec.Text.choice"));
        assert!(!filtered_types_chirho.contains_key("choice"));
    }

    #[test]
    fn discover_hs_files_in_temp_dir_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "module Main where\nmain = 42\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Lib.hs"),
            "module Lib where\nfoo = 1\n",
        )
        .unwrap();
        let sub_chirho = tmp_chirho.path().join("Data");
        fs::create_dir_all(&sub_chirho).unwrap();
        fs::write(
            sub_chirho.join("Utils.hs"),
            "module Data.Utils where\nbar = 2\n",
        )
        .unwrap();

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
    fn real_parsec_reexport_module_seeds_choice_scheme_chirho() {
        let package_root_chirho =
            workspace_root_chirho().join(".haskelujah-packages-chirho/parsec-3.1.18.0/src");
        let module_names_chirho = [
            "Text.Parsec.Pos",
            "Text.Parsec.Error",
            "Text.Parsec.Prim",
            "Text.Parsec.Char",
            "Text.Parsec.Combinator",
            "Text.Parsec",
        ];
        let module_sources_chirho: Vec<(String, String, String)> = module_names_chirho
            .iter()
            .map(|module_name_chirho| {
                let relative_path_chirho = module_name_chirho.replace('.', "/") + ".hs";
                let path_chirho = package_root_chirho.join(relative_path_chirho);
                let source_chirho = fs::read_to_string(&path_chirho).unwrap();
                (
                    (*module_name_chirho).to_string(),
                    path_chirho.to_string_lossy().to_string(),
                    source_chirho,
                )
            })
            .collect();
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
            module_sources_chirho,
            &mut source_map_chirho,
            vec![],
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        )
        .expect("real parsec frontend artifacts should build for the core module slice");

        assert!(
            artifacts_chirho
                .imported_types_chirho
                .contains_key("Text.Parsec.choice"),
            "Text.Parsec should re-export choice into downstream seeded schemes"
        );
        assert!(
            artifacts_chirho
                .imported_type_synonyms_chirho
                .contains_key("Text.Parsec.Parsec"),
            "Text.Parsec should re-export the Parsec type synonym into downstream seeded synonyms"
        );
    }

    #[test]
    fn real_parsec_core_slice_orders_text_parsec_before_perm_chirho() {
        let package_root_chirho =
            workspace_root_chirho().join(".haskelujah-packages-chirho/parsec-3.1.18.0/src");
        let module_names_chirho = [
            "Text.Parsec.Pos",
            "Text.Parsec.Error",
            "Text.Parsec.Prim",
            "Text.Parsec.Char",
            "Text.Parsec.Combinator",
            "Text.Parsec",
            "Text.Parsec.Perm",
        ];
        let module_sources_chirho: Vec<(String, String, String)> = module_names_chirho
            .iter()
            .map(|module_name_chirho| {
                let relative_path_chirho = module_name_chirho.replace('.', "/") + ".hs";
                let path_chirho = package_root_chirho.join(relative_path_chirho);
                let source_chirho = fs::read_to_string(&path_chirho).unwrap();
                (
                    (*module_name_chirho).to_string(),
                    path_chirho.to_string_lossy().to_string(),
                    source_chirho,
                )
            })
            .collect();
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let compile_result_chirho = compile_module_sources_with_extra_ifaces_chirho(
            module_sources_chirho,
            &mut source_map_chirho,
            vec![],
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        );

        match compile_result_chirho {
            Ok(project_result_chirho) => {
                let parsec_index_chirho = project_result_chirho
                    .compilation_order_chirho
                    .iter()
                    .position(|name_chirho| name_chirho == "Text.Parsec")
                    .unwrap();
                let perm_index_chirho = project_result_chirho
                    .compilation_order_chirho
                    .iter()
                    .position(|name_chirho| name_chirho == "Text.Parsec.Perm")
                    .unwrap();
                assert!(
                    parsec_index_chirho < perm_index_chirho,
                    "Text.Parsec should compile before Text.Parsec.Perm in the core parsec slice"
                );
            }
            Err(error_chirho) => panic!(
                "real parsec core slice should compile or at least expose an ordering error, got: {error_chirho}"
            ),
        }
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
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
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
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
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
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
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
    fn compile_project_circular_import_succeeds_chirho() {
        // A imports B, B imports A — circular import handled via SCC compilation.
        // Neither module uses names from the other, so empty boot interfaces suffice.
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
        assert!(
            result_chirho.is_ok(),
            "Circular import with no cross-references should succeed, got: {:?}",
            result_chirho.err()
        );
        let proj_chirho = result_chirho.unwrap();
        assert_eq!(proj_chirho.module_results_chirho.len(), 2);
    }

    #[test]
    fn compile_project_circular_with_boot_chirho() {
        // A imports B (uses b_val), B imports A (uses a_val).
        // .hs-boot files provide the needed interfaces.
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("A.hs"),
            "module A where\nimport B\na_val = 10\nmain_a = a_val\n",
        )
        .unwrap();
        // A.hs-boot declares a_val for B to import
        fs::write(
            tmp_chirho.path().join("A.hs-boot"),
            "module A where\na_val = 10\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("B.hs"),
            "module B where\nimport A\nb_val = 20\n",
        )
        .unwrap();
        // B.hs-boot declares b_val for A to import
        fs::write(
            tmp_chirho.path().join("B.hs-boot"),
            "module B where\nb_val = 20\n",
        )
        .unwrap();

        let mut sm_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho);
        assert!(
            result_chirho.is_ok(),
            "Circular import with boot files should succeed, got: {:?}",
            result_chirho.err()
        );
        let proj_chirho = result_chirho.unwrap();
        assert_eq!(proj_chirho.module_results_chirho.len(), 2);
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
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut sm_chirho)
            .expect("single module should compile");
        assert_eq!(result_chirho.compilation_order_chirho, vec!["Main"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }

    #[test]
    fn compile_project_cabal_based_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(&src_dir_chirho).unwrap();

        // Write a minimal .cabal file
        fs::write(
            tmp_chirho.path().join("myproject.cabal"),
            "\
name: myproject
version: 0.1.0.0
library
  exposed-modules: Lib
  hs-source-dirs: src
executable myproject
  main-is: Main.hs
  other-modules: Lib
  hs-source-dirs: src
",
        )
        .unwrap();

        fs::write(
            src_dir_chirho.join("Lib.hs"),
            "module Lib where\nadd1 x = x + 1\n",
        )
        .unwrap();
        fs::write(
            src_dir_chirho.join("Main.hs"),
            "module Main where\nimport Lib\nmain = add1 41\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = tmp_chirho.path().join("myproject.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
            .expect("cabal project should compile");
        assert_eq!(result_chirho.package_chirho.name_chirho, "myproject");
        assert!(!result_chirho.module_results_chirho.is_empty());
    }

    #[test]
    fn compile_cabal_project_orders_hierarchical_other_modules_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(src_dir_chirho.join("Utils/Containers/Internal")).unwrap();

        fs::write(
            tmp_chirho.path().join("mini-containers.cabal"),
            "\
name: mini-containers
version: 0.1.0.0
library
  exposed-modules: Utils.Containers.Internal.BitQueue
  other-modules: Utils.Containers.Internal.BitUtil
  hs-source-dirs: src
",
        )
        .unwrap();

        fs::write(
            src_dir_chirho.join("Utils/Containers/Internal/BitUtil.hs"),
            "module Utils.Containers.Internal.BitUtil (wordSize) where\n\
wordSize :: Int\n\
wordSize = 64\n",
        )
        .unwrap();
        fs::write(
            src_dir_chirho.join("Utils/Containers/Internal/BitQueue.hs"),
            "module Utils.Containers.Internal.BitQueue (queueSizeChirho) where\n\
import Utils.Containers.Internal.BitUtil (wordSize)\n\
\n\
queueSizeChirho :: Int\n\
queueSizeChirho = wordSize\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = tmp_chirho.path().join("mini-containers.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        assert!(
            result_chirho.is_ok(),
            "cabal build should topo-sort hierarchical other-modules by imports: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_cabal_project_nested_under_packages_dir_orders_cpp_hierarchical_modules_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let package_dir_chirho = packages_root_chirho.join("mini-containers-0.1.0.0");
        let src_dir_chirho = package_dir_chirho.join("src");
        let include_dir_chirho = package_dir_chirho.join("include");
        fs::create_dir_all(src_dir_chirho.join("Utils/Containers/Internal")).unwrap();
        fs::create_dir_all(src_dir_chirho.join("Data/IntSet/Internal")).unwrap();
        fs::create_dir_all(&include_dir_chirho).unwrap();

        fs::write(
            include_dir_chirho.join("containers.h"),
            "#include \"MachDeps.h\"\n",
        )
        .unwrap();
        fs::write(
            package_dir_chirho.join("mini-containers.cabal"),
            "\
name: mini-containers
version: 0.1.0.0
library
  build-depends: base
  if impl(ghc)
    build-depends: template-haskell
  hs-source-dirs: src
  other-extensions: CPP
  exposed-modules: Data.IntSet.Internal.IntTreeCommons
  other-modules:
    Utils.Containers.Internal.BitUtil
  include-dirs: include
",
        )
        .unwrap();
        fs::write(
            src_dir_chirho.join("Utils/Containers/Internal/BitUtil.hs"),
            "{-# LANGUAGE CPP #-}\n\
#ifdef __GLASGOW_HASKELL__\n\
{-# LANGUAGE MagicHash #-}\n\
#endif\n\
#include \"containers.h\"\n\
module Utils.Containers.Internal.BitUtil (wordSize) where\n\
\n\
wordSize :: Int\n\
wordSize = 64\n",
        )
        .unwrap();
        fs::write(
            src_dir_chirho.join("Data/IntSet/Internal/IntTreeCommons.hs"),
            "module Data.IntSet.Internal.IntTreeCommons where\n\
import Utils.Containers.Internal.BitUtil (wordSize)\n\
\n\
treeWordSizeChirho :: Int\n\
treeWordSizeChirho = wordSize\n",
        )
        .unwrap();
        fs::create_dir_all(packages_root_chirho.join("dummy-dep-0.1.0.0/src")).unwrap();
        fs::write(
            packages_root_chirho.join("dummy-dep-0.1.0.0/dummy-dep.cabal"),
            "name: dummy-dep\nversion: 0.1.0.0\nlibrary\n  exposed-modules: Dummy\n  hs-source-dirs: src\n",
        )
        .unwrap();
        fs::write(
            packages_root_chirho.join("dummy-dep-0.1.0.0/src/Dummy.hs"),
            "module Dummy where\ndummyValueChirho :: Int\ndummyValueChirho = 1\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = package_dir_chirho.join("mini-containers.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        assert!(
            result_chirho.is_ok(),
            "nested package builds should compile BitUtil before IntTreeCommons even under .haskelujah-packages-chirho scanning: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn discover_real_containers_bitutil_module_chirho() {
        use crate::discover_modules_chirho;
        use haskelujah_package_chirho::parse_cabal_chirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/containers-0.8/containers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).unwrap();
        let package_chirho = parse_cabal_chirho(&cabal_content_chirho);
        let project_dir_chirho = cabal_path_chirho.parent().unwrap();
        let discovered_modules_chirho =
            discover_modules_chirho(&package_chirho, project_dir_chirho);

        assert!(
            discovered_modules_chirho
                .iter()
                .any(|(module_name_chirho, _)| module_name_chirho
                    == "Utils.Containers.Internal.BitUtil"),
            "expected real containers discovery to include Utils.Containers.Internal.BitUtil; sample={:?}",
            discovered_modules_chirho
                .iter()
                .map(|(module_name_chirho, _)| module_name_chirho.clone())
                .take(20)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn real_containers_dep_graph_orders_bitutil_before_inttreecommons_chirho() {
        use crate::{
            discover_modules_chirho, extract_imports_chirho, read_haskell_source_file_chirho,
        };
        use haskelujah_incremental_chirho::{DepGraphChirho, FingerprintChirho};
        use haskelujah_package_chirho::parse_cabal_chirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/containers-0.8/containers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).unwrap();
        let package_chirho = parse_cabal_chirho(&cabal_content_chirho);
        let project_dir_chirho = cabal_path_chirho.parent().unwrap();
        let discovered_modules_chirho =
            discover_modules_chirho(&package_chirho, project_dir_chirho);

        let mut dep_graph_chirho = DepGraphChirho::new_chirho();
        let known_modules_chirho: std::collections::HashSet<String> = discovered_modules_chirho
            .iter()
            .map(|(module_name_chirho, _)| module_name_chirho.clone())
            .collect();

        for (module_name_chirho, path_chirho) in &discovered_modules_chirho {
            let source_chirho = read_haskell_source_file_chirho(path_chirho).unwrap();
            dep_graph_chirho.add_module_chirho(
                module_name_chirho,
                FingerprintChirho::from_str_chirho(&source_chirho),
            );
            for imported_module_chirho in extract_imports_chirho(&source_chirho) {
                if known_modules_chirho.contains(&imported_module_chirho) {
                    dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_module_chirho);
                }
            }
        }

        let compilation_order_chirho: Vec<String> = dep_graph_chirho
            .topo_sort_sccs_chirho()
            .into_iter()
            .flatten()
            .collect();
        let bitutil_idx_chirho = compilation_order_chirho
            .iter()
            .position(|module_name_chirho| {
                module_name_chirho == "Utils.Containers.Internal.BitUtil"
            })
            .unwrap();
        let inttreecommons_idx_chirho = compilation_order_chirho
            .iter()
            .position(|module_name_chirho| {
                module_name_chirho == "Data.IntSet.Internal.IntTreeCommons"
            })
            .unwrap();

        assert!(
            bitutil_idx_chirho < inttreecommons_idx_chirho,
            "expected BitUtil before IntTreeCommons, got {:?}",
            compilation_order_chirho
        );
    }

    #[test]
    fn real_containers_bitutil_cpp_keeps_module_header_chirho() {
        use crate::read_haskell_source_file_chirho;
        use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
        use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
        use haskelujah_span_chirho::FileIdChirho;

        let bitutil_path_chirho = workspace_root_chirho().join(
            ".haskelujah-packages-chirho/containers-0.8/src/Utils/Containers/Internal/BitUtil.hs",
        );
        if !bitutil_path_chirho.exists() {
            return;
        }

        let source_chirho = read_haskell_source_file_chirho(bitutil_path_chirho).unwrap();
        let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
        let green_chirho = ParserChirho::new_chirho(&source_chirho, file_id_chirho).parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

        assert_eq!(
            module_chirho.name_chirho.full_name_chirho(),
            "Utils.Containers.Internal.BitUtil",
            "expected CPP-read BitUtil source to keep its hierarchical module header",
        );
    }

    #[test]
    fn real_containers_inttreecommons_frontend_accepts_bitutil_iface_chirho() {
        use crate::{
            ImportedTypeSynonymsChirho, SourceMapChirho, read_haskell_source_file_chirho,
            run_frontend_with_type_synonyms_chirho,
        };
        use haskelujah_naming_chirho::builtin_module_ifaces_chirho;
        use haskelujah_naming_chirho::iface_chirho::build_iface_with_imports_chirho;
        use haskelujah_syntax_chirho::SourceFileChirho;

        let bitutil_path_chirho = workspace_root_chirho().join(
            ".haskelujah-packages-chirho/containers-0.8/src/Utils/Containers/Internal/BitUtil.hs",
        );
        let inttreecommons_path_chirho = workspace_root_chirho().join(
            ".haskelujah-packages-chirho/containers-0.8/src/Data/IntSet/Internal/IntTreeCommons.hs",
        );
        if !bitutil_path_chirho.exists() || !inttreecommons_path_chirho.exists() {
            return;
        }

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let bitutil_source_chirho = read_haskell_source_file_chirho(&bitutil_path_chirho).unwrap();
        let bitutil_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            bitutil_path_chirho,
            &bitutil_source_chirho,
        );
        let builtin_ifaces_chirho = builtin_module_ifaces_chirho();
        let empty_imported_types_chirho = std::collections::HashMap::new();
        let empty_imported_synonyms_chirho = ImportedTypeSynonymsChirho::new();

        let bitutil_frontend_chirho = run_frontend_with_type_synonyms_chirho(
            &bitutil_source_chirho,
            bitutil_file_chirho.file_id_chirho(),
            &builtin_ifaces_chirho,
            &empty_imported_types_chirho,
            &empty_imported_synonyms_chirho,
        )
        .expect("BitUtil frontend should succeed");
        let bitutil_iface_chirho = build_iface_with_imports_chirho(
            &bitutil_frontend_chirho.module_chirho,
            &builtin_ifaces_chirho,
        );

        let inttreecommons_source_chirho =
            read_haskell_source_file_chirho(&inttreecommons_path_chirho).unwrap();
        let inttreecommons_file_chirho = SourceFileChirho::from_source_map_chirho(
            &mut source_map_chirho,
            inttreecommons_path_chirho,
            &inttreecommons_source_chirho,
        );
        let mut available_ifaces_chirho = builtin_ifaces_chirho.clone();
        available_ifaces_chirho.push(bitutil_iface_chirho);

        let inttreecommons_result_chirho = run_frontend_with_type_synonyms_chirho(
            &inttreecommons_source_chirho,
            inttreecommons_file_chirho.file_id_chirho(),
            &available_ifaces_chirho,
            &empty_imported_types_chirho,
            &empty_imported_synonyms_chirho,
        );
        assert!(
            inttreecommons_result_chirho.is_ok(),
            "expected IntTreeCommons to resolve BitUtil from a freshly-built local iface: {:?}",
            inttreecommons_result_chirho.err()
        );
    }

    #[test]
    fn real_containers_loop_has_bitutil_iface_before_inttreecommons_chirho() {
        use crate::{
            ImportedTypeSynonymsChirho, SourceMapChirho, discover_modules_chirho,
            extract_imports_chirho, read_haskell_source_file_chirho,
            run_frontend_with_type_synonyms_chirho,
        };
        use haskelujah_incremental_chirho::{DepGraphChirho, FingerprintChirho};
        use haskelujah_naming_chirho::builtin_module_ifaces_chirho;
        use haskelujah_naming_chirho::iface_chirho::build_iface_with_imports_chirho;
        use haskelujah_package_chirho::parse_cabal_chirho;
        use haskelujah_syntax_chirho::SourceFileChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/containers-0.8/containers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).unwrap();
        let package_chirho = parse_cabal_chirho(&cabal_content_chirho);
        let project_dir_chirho = cabal_path_chirho.parent().unwrap();
        let source_files_chirho = discover_modules_chirho(&package_chirho, project_dir_chirho);
        let mut module_sources_chirho = Vec::new();
        for (module_name_chirho, path_chirho) in &source_files_chirho {
            module_sources_chirho.push((
                module_name_chirho.clone(),
                path_chirho.to_string_lossy().to_string(),
                read_haskell_source_file_chirho(path_chirho).unwrap(),
            ));
        }

        let mut dep_graph_chirho = DepGraphChirho::new_chirho();
        let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
            .iter()
            .map(|(module_name_chirho, _, _)| module_name_chirho.clone())
            .collect();
        for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
            dep_graph_chirho.add_module_chirho(
                module_name_chirho,
                FingerprintChirho::from_str_chirho(source_chirho),
            );
            for imported_chirho in extract_imports_chirho(source_chirho) {
                if known_modules_chirho.contains(&imported_chirho) {
                    dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
                }
            }
        }

        let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let mut ifaces_chirho = builtin_module_ifaces_chirho();
        let imported_types_chirho = std::collections::HashMap::new();
        let imported_synonyms_chirho = ImportedTypeSynonymsChirho::new();

        for scc_chirho in sccs_chirho {
            for module_name_chirho in scc_chirho {
                if module_name_chirho == "Data.IntSet.Internal.IntTreeCommons" {
                    assert!(
                        ifaces_chirho.iter().any(|iface_chirho| {
                            iface_chirho.name_chirho == "Utils.Containers.Internal.BitUtil"
                        }),
                        "BitUtil iface missing before IntTreeCommons; sample={:?}",
                        ifaces_chirho
                            .iter()
                            .map(|iface_chirho| iface_chirho.name_chirho.clone())
                            .take(40)
                            .collect::<Vec<_>>()
                    );
                }
                let (_, file_name_chirho, source_chirho) = module_sources_chirho
                    .iter()
                    .find(|(candidate_name_chirho, _, _)| {
                        candidate_name_chirho == &module_name_chirho
                    })
                    .unwrap();
                let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                    &mut source_map_chirho,
                    file_name_chirho,
                    source_chirho,
                );
                let frontend_result_chirho = run_frontend_with_type_synonyms_chirho(
                    source_chirho,
                    source_file_chirho.file_id_chirho(),
                    &ifaces_chirho,
                    &imported_types_chirho,
                    &imported_synonyms_chirho,
                );
                let frontend_result_chirho =
                    frontend_result_chirho.unwrap_or_else(|error_chirho| {
                        panic!(
                            "module {} failed with ifaces {:?}: {:?}",
                            module_name_chirho,
                            ifaces_chirho
                                .iter()
                                .map(|iface_chirho| iface_chirho.name_chirho.clone())
                                .take(50)
                                .collect::<Vec<_>>(),
                            error_chirho
                        )
                    });
                let iface_chirho = build_iface_with_imports_chirho(
                    &frontend_result_chirho.module_chirho,
                    &ifaces_chirho,
                );
                ifaces_chirho.push(iface_chirho);
            }
        }
    }

    #[test]
    fn real_containers_dependency_seed_includes_bitutil_iface_chirho() {
        let project_dir_chirho =
            workspace_root_chirho().join(".haskelujah-packages-chirho/containers-0.8");
        if !project_dir_chirho.exists() {
            return;
        }

        let dependency_ifaces_chirho =
            crate::scan_dependency_package_ifaces_chirho(&project_dir_chirho);
        assert!(
            dependency_ifaces_chirho
                .iter()
                .any(|iface_chirho| iface_chirho.name_chirho == "Utils.Containers.Internal.BitUtil"),
            "expected dependency seed to include BitUtil; sample={:?}",
            dependency_ifaces_chirho
                .iter()
                .map(|iface_chirho| iface_chirho.name_chirho.clone())
                .take(60)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn real_containers_package_seed_still_resolves_bitutil_before_inttreecommons_chirho() {
        use crate::{
            SourceMapChirho, collect_local_dependency_frontend_artifacts_chirho,
            collect_package_deps_chirho, discover_modules_chirho, extract_imports_chirho,
            read_haskell_source_file_chirho, run_frontend_with_type_synonyms_chirho,
            scan_dependency_package_ifaces_chirho,
        };
        use haskelujah_incremental_chirho::{DepGraphChirho, FingerprintChirho};
        use haskelujah_naming_chirho::builtin_module_ifaces_chirho;
        use haskelujah_naming_chirho::iface_chirho::{
            build_iface_with_imports_chirho, merge_module_ifaces_chirho,
        };
        use haskelujah_package_chirho::parse_cabal_chirho;
        use haskelujah_syntax_chirho::SourceFileChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/containers-0.8/containers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).unwrap();
        let package_chirho = parse_cabal_chirho(&cabal_content_chirho);
        let project_dir_chirho = cabal_path_chirho.parent().unwrap();
        let all_deps_chirho = collect_package_deps_chirho(&package_chirho);
        let dep_ifaces_chirho = scan_dependency_package_ifaces_chirho(project_dir_chirho);
        let dep_frontend_artifacts_chirho = collect_local_dependency_frontend_artifacts_chirho(
            project_dir_chirho,
            &all_deps_chirho,
        )
        .unwrap();
        let mut extra_ifaces_chirho = dep_ifaces_chirho;
        extra_ifaces_chirho.extend(dep_frontend_artifacts_chirho.ifaces_chirho.clone());

        let source_files_chirho = discover_modules_chirho(&package_chirho, project_dir_chirho);
        let mut module_sources_chirho = Vec::new();
        for (module_name_chirho, path_chirho) in &source_files_chirho {
            module_sources_chirho.push((
                module_name_chirho.clone(),
                path_chirho.to_string_lossy().to_string(),
                read_haskell_source_file_chirho(path_chirho).unwrap(),
            ));
        }

        let mut dep_graph_chirho = DepGraphChirho::new_chirho();
        let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
            .iter()
            .map(|(module_name_chirho, _, _)| module_name_chirho.clone())
            .collect();
        for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
            dep_graph_chirho.add_module_chirho(
                module_name_chirho,
                FingerprintChirho::from_str_chirho(source_chirho),
            );
            for imported_chirho in extract_imports_chirho(source_chirho) {
                if known_modules_chirho.contains(&imported_chirho) {
                    dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
                }
            }
        }

        let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let mut ifaces_chirho = builtin_module_ifaces_chirho();
        ifaces_chirho.extend(extra_ifaces_chirho);
        let mut ifaces_chirho = merge_module_ifaces_chirho(ifaces_chirho);
        let imported_types_chirho = dep_frontend_artifacts_chirho.imported_types_chirho;
        let imported_synonyms_chirho = dep_frontend_artifacts_chirho.imported_type_synonyms_chirho;

        for scc_chirho in sccs_chirho {
            for module_name_chirho in scc_chirho {
                if module_name_chirho == "Data.IntSet.Internal.IntTreeCommons" {
                    assert!(
                        ifaces_chirho.iter().any(|iface_chirho| {
                            iface_chirho.name_chirho == "Utils.Containers.Internal.BitUtil"
                        }),
                        "BitUtil missing from package-seeded ifaces before IntTreeCommons; sample={:?}",
                        ifaces_chirho
                            .iter()
                            .map(|iface_chirho| iface_chirho.name_chirho.clone())
                            .take(80)
                            .collect::<Vec<_>>()
                    );
                }
                let (_, file_name_chirho, source_chirho) = module_sources_chirho
                    .iter()
                    .find(|(candidate_name_chirho, _, _)| {
                        candidate_name_chirho == &module_name_chirho
                    })
                    .unwrap();
                let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                    &mut source_map_chirho,
                    file_name_chirho,
                    source_chirho,
                );
                let frontend_result_chirho = run_frontend_with_type_synonyms_chirho(
                    source_chirho,
                    source_file_chirho.file_id_chirho(),
                    &ifaces_chirho,
                    &imported_types_chirho,
                    &imported_synonyms_chirho,
                );
                let frontend_result_chirho = frontend_result_chirho.unwrap_or_else(|error_chirho| {
                    panic!(
                        "module {} failed with package seed; has_bitutil={} sample_ifaces={:?} error={:?}",
                        module_name_chirho,
                        ifaces_chirho.iter().any(|iface_chirho| {
                            iface_chirho.name_chirho == "Utils.Containers.Internal.BitUtil"
                        }),
                        ifaces_chirho
                            .iter()
                            .map(|iface_chirho| iface_chirho.name_chirho.clone())
                            .take(80)
                            .collect::<Vec<_>>(),
                        error_chirho
                    )
                });
                let iface_chirho = build_iface_with_imports_chirho(
                    &frontend_result_chirho.module_chirho,
                    &ifaces_chirho,
                );
                ifaces_chirho.push(iface_chirho);
            }
        }
    }

    #[test]
    fn compile_real_containers_cabal_project_moves_past_bitutil_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/containers-0.8/containers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real containers compile_cabal_project should compile modules",
                );
                let bitutil_idx_chirho = result_chirho
                    .compilation_order_chirho
                    .iter()
                    .position(|module_name_chirho| {
                        module_name_chirho == "Utils.Containers.Internal.BitUtil"
                    })
                    .expect("compilation order should include BitUtil");
                let inttreecommons_idx_chirho = result_chirho
                    .compilation_order_chirho
                    .iter()
                    .position(|module_name_chirho| {
                        module_name_chirho == "Data.IntSet.Internal.IntTreeCommons"
                    })
                    .expect("compilation order should include IntTreeCommons");
                assert!(bitutil_idx_chirho < inttreecommons_idx_chirho);
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !error_text_chirho
                        .contains("could not find module Utils.Containers.Internal.BitUtil"),
                    "containers should move past the old BitUtil failure, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn build_then_compile_real_containers_stays_past_bitutil_chirho() {
        use crate::{build_cabal_project_chirho, compile_cabal_project_chirho};
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/containers-0.8/containers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let index_chirho = PackageIndexChirho::new_chirho();
        let build_result_chirho = build_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        if let Err(error_chirho) = build_result_chirho {
            let error_text_chirho = format!("{error_chirho}");
            assert!(
                !error_text_chirho
                    .contains("could not find module Utils.Containers.Internal.BitUtil"),
                "build should stay past the old BitUtil frontier, got: {error_text_chirho}",
            );
        }

        let compile_result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        if let Err(error_chirho) = compile_result_chirho {
            let error_text_chirho = format!("{error_chirho}");
            assert!(
                !error_text_chirho
                    .contains("could not find module Utils.Containers.Internal.BitUtil"),
                "build-then-compile should stay past the old BitUtil frontier, got: {error_text_chirho}",
            );
        }
    }

    #[test]
    fn compile_project_skips_hidden_dirs_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "module Main where\nmain = 42\n",
        )
        .unwrap();

        // Create a hidden directory with an .hs file that should be skipped
        let hidden_dir_chirho = tmp_chirho.path().join(".hidden");
        fs::create_dir_all(&hidden_dir_chirho).unwrap();
        fs::write(
            hidden_dir_chirho.join("Secret.hs"),
            "module Secret where\nsecret = 99\n",
        )
        .unwrap();

        let files_chirho = discover_hs_files_chirho(tmp_chirho.path());
        assert_eq!(files_chirho.len(), 1); // Only Main.hs, not Secret.hs
    }

    #[test]
    fn compile_project_cpp_module_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("Main.hs"),
            "\
{-# LANGUAGE CPP #-}
module Main where
#if __GLASGOW_HASKELL__ >= 810
main = 42
#else
main = 0
#endif
",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho)
            .expect("project compilation should preprocess CPP modules");
        assert_eq!(result_chirho.compilation_order_chirho, vec!["Main"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }

    #[test]
    fn compile_project_imported_record_wildcards_bind_fields_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data PosStateChirho sChirho = PosStateChirho\n\
  { pstateInputChirho :: sChirho\n\
  , pstateOffsetChirho :: Int\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "{-# LANGUAGE RecordWildCards #-}\n\
module UseMiniChirho where\n\
import StateMiniChirho\n\
reachOffsetMiniChirho :: PosStateChirho [Int] -> Int\n\
reachOffsetMiniChirho PosStateChirho {..} = pstateOffsetChirho\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "project compilation should bind imported RecordWildCards fields from iface metadata: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_cabal_project_imported_infix_constructor_member_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(&src_dir_chirho).unwrap();

        fs::write(
            tmp_chirho.path().join("containers-mini.cabal"),
            "\
name: containers-mini
version: 0.1.0.0
library
  exposed-modules: Utils.Containers.Internal.StrictPair, Utils.Containers.Internal.EqOrdUtil
  hs-source-dirs: src
",
        )
        .unwrap();

        let utils_dir_chirho = src_dir_chirho.join("Utils/Containers/Internal");
        fs::create_dir_all(&utils_dir_chirho).unwrap();
        fs::write(
            utils_dir_chirho.join("StrictPair.hs"),
            "module Utils.Containers.Internal.StrictPair (StrictPair(..), toPair) where\n\
data StrictPair a b = !a :*: !b\n\
infixr 1 :*:\n\
toPair :: StrictPair a b -> (a, b)\n\
toPair (x :*: y) = (x, y)\n",
        )
        .unwrap();
        fs::write(
            utils_dir_chirho.join("EqOrdUtil.hs"),
            "module Utils.Containers.Internal.EqOrdUtil where\n\
import Utils.Containers.Internal.StrictPair\n\
\n\
data EqMiniChirho a = EqMiniChirho { runEqMiniChirho :: a -> StrictPair Bool a }\n\
\n\
unwrapEqMiniChirho :: EqMiniChirho a -> a -> StrictPair Bool a\n\
unwrapEqMiniChirho fChirho xChirho = case runEqMiniChirho fChirho xChirho of\n\
  rChirho@(eChirho :*: xPrimeChirho) -> if eChirho then True :*: xPrimeChirho else rChirho\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = tmp_chirho.path().join("containers-mini.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        assert!(
            result_chirho.is_ok(),
            "cabal build should import infix constructor members from same-package ifaces: {:?}",
            result_chirho.err()
        );
    }
}
