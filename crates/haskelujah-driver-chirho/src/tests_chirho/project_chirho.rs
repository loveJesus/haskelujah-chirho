// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Tests for hierarchical project compilation (compile_project_dir_chirho).

#[cfg(test)]
mod tests_chirho {
    use crate::{
        collect_frontend_artifacts_from_module_sources_chirho,
        collect_local_dependency_frontend_artifacts_chirho,
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
    fn extract_imports_source_pragma_chirho() {
        let src_chirho = "module Foo where\nimport {-# SOURCE #-} qualified Text.Megaparsec.Error as ErrorChirho\n";
        let imports_chirho = extract_imports_chirho(src_chirho);
        assert_eq!(imports_chirho, vec!["Text.Megaparsec.Error"]);
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
    fn extend_imported_type_families_preserves_existing_equations_chirho() {
        use crate::extend_imported_type_families_chirho;

        let mut target_families_chirho = std::collections::HashMap::from([(
            "PrimState".to_string(),
            vec![(
                vec![TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_string())),
                    Box::new(TyChirho::ForallVarChirho("s".to_string())),
                )],
                TyChirho::ForallVarChirho("s".to_string()),
            )],
        )]);
        let source_families_chirho =
            std::collections::HashMap::from([("PrimState".to_string(), Vec::new())]);

        extend_imported_type_families_chirho(&mut target_families_chirho, source_families_chirho);

        let prim_state_equations_chirho = target_families_chirho
            .get("PrimState")
            .expect("PrimState family should remain present after merge");
        assert_eq!(
            prim_state_equations_chirho.len(),
            1,
            "merging an empty family definition should not erase existing equations"
        );
    }

    #[test]
    fn filter_seeded_imported_types_keeps_unique_bare_local_export_names_chirho() {
        let source_chirho = "module DownstreamMultiMapSeedMiniChirho where\nimport LocalMultiMapSeedMiniChirho as MM\nvalueChirho = empty\n";
        let imported_types_chirho = std::collections::HashMap::from([
            (
                "LocalMultiMapSeedMiniChirho.empty".to_string(),
                SchemeChirho::mono_chirho(TyChirho::ConChirho(
                    "LocalEmptySchemeChirho".to_string(),
                )),
            ),
            (
                "empty".to_string(),
                SchemeChirho::mono_chirho(TyChirho::ConChirho(
                    "LocalEmptySchemeChirho".to_string(),
                )),
            ),
            (
                "Text.Parsec.empty".to_string(),
                SchemeChirho::mono_chirho(TyChirho::ConChirho(
                    "OtherEmptySchemeChirho".to_string(),
                )),
            ),
        ]);

        let filtered_types_chirho =
            filter_seeded_imported_types_for_source_chirho(source_chirho, &imported_types_chirho);

        assert!(filtered_types_chirho.contains_key("LocalMultiMapSeedMiniChirho.empty"));
        assert!(filtered_types_chirho.contains_key("empty"));
        assert!(!filtered_types_chirho.contains_key("Text.Parsec.empty"));
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
            std::collections::HashMap::new(),
            true,
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
    fn builtin_ghc_event_timeout_helpers_typecheck_chirho() {
        let module_sources_chirho = vec![(
            "Main".to_string(),
            "/virtual/Main.hs".to_string(),
            "module Main where\nimport GHC.Event\nmain = do\n  mgr <- getSystemTimerManager\n  key <- registerTimeout mgr 1000 (pure ())\n  unregisterTimeout mgr key\n".to_string(),
        )];
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let compile_result_chirho = compile_module_sources_with_extra_ifaces_chirho(
            module_sources_chirho,
            &mut source_map_chirho,
            vec![],
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        );

        assert!(
            compile_result_chirho.is_ok(),
            "GHC.Event timer helper builtins should typecheck, got: {compile_result_chirho:?}"
        );
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
    fn compile_cabal_project_preserves_exposed_modules_after_blank_lines_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(src_dir_chirho.join("Demo")).unwrap();

        fs::write(
            tmp_chirho.path().join("blank-line-modules.cabal"),
            "\
name: blank-line-modules
version: 0.1.0.0
library
  exposed-modules:
    Demo.PublicChirho

    -- keep parsing this field
    Demo.InternalChirho
  other-modules:
    Demo.UserChirho
  hs-source-dirs: src
",
        )
        .unwrap();

        fs::write(
            src_dir_chirho.join("Demo/InternalChirho.hs"),
            "module Demo.InternalChirho (helperValueChirho) where\n\
helperValueChirho :: Int\n\
helperValueChirho = 7\n",
        )
        .unwrap();
        fs::write(
            src_dir_chirho.join("Demo/PublicChirho.hs"),
            "module Demo.PublicChirho (publicValueChirho) where\n\
import Demo.InternalChirho (helperValueChirho)\n\
\n\
publicValueChirho :: Int\n\
publicValueChirho = helperValueChirho\n",
        )
        .unwrap();
        fs::write(
            src_dir_chirho.join("Demo/UserChirho.hs"),
            "module Demo.UserChirho (userValueChirho) where\n\
import Demo.InternalChirho (helperValueChirho)\n\
\n\
userValueChirho :: Int\n\
userValueChirho = helperValueChirho\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = tmp_chirho.path().join("blank-line-modules.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        assert!(
            result_chirho.is_ok(),
            "cabal build should keep exposed-modules entries after blank lines/comments: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_cabal_project_applies_cpp_options_to_package_modules_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(src_dir_chirho.join("Demo")).unwrap();

        fs::write(
            tmp_chirho.path().join("cpp-options-package.cabal"),
            "\
name: cpp-options-package
version: 0.1.0.0
library
  exposed-modules: Demo.CppBranchChirho
  hs-source-dirs: src
  cpp-options: -DKEEP_PRIMARY_CHIRHO=1
",
        )
        .unwrap();

        fs::write(
            src_dir_chirho.join("Demo/CppBranchChirho.hs"),
            "{-# LANGUAGE CPP #-}\n\
module Demo.CppBranchChirho (valueChirho) where\n\
\n\
#if KEEP_PRIMARY_CHIRHO\n\
valueChirho :: Int\n\
valueChirho = 1\n\
#else\n\
import Demo.MissingChirho (missingValueChirho)\n\
\n\
valueChirho :: Int\n\
valueChirho = missingValueChirho\n\
#endif\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = tmp_chirho.path().join("cpp-options-package.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        assert!(
            result_chirho.is_ok(),
            "cabal build should apply cpp-options before frontend compilation: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_cabal_project_discovers_min_version_macros_from_local_packages_dir_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(src_dir_chirho.join("Demo")).unwrap();
        let packages_dir_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let filepath_dir_chirho = packages_dir_chirho.join("filepath-1.5.5.0");
        fs::create_dir_all(&filepath_dir_chirho).unwrap();

        fs::write(
            tmp_chirho.path().join("min-version-package.cabal"),
            "\
name: min-version-package
version: 0.1.0.0
library
  exposed-modules: Demo.VersionBranchChirho
  hs-source-dirs: src
  build-depends: base, filepath
",
        )
        .unwrap();

        fs::write(
            filepath_dir_chirho.join("filepath.cabal"),
            "\
name: filepath
version: 1.5.5.0
library
  exposed-modules: System.FilePath
  hs-source-dirs: src
",
        )
        .unwrap();

        fs::write(
            src_dir_chirho.join("Demo/VersionBranchChirho.hs"),
            "{-# LANGUAGE CPP #-}\n\
module Demo.VersionBranchChirho (valueChirho) where\n\
\n\
#if MIN_VERSION_filepath(1,5,0)\n\
valueChirho :: Int\n\
valueChirho = 1\n\
#else\n\
import Demo.MissingChirho (missingValueChirho)\n\
\n\
valueChirho :: Int\n\
valueChirho = missingValueChirho\n\
#endif\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let cabal_path_chirho = tmp_chirho.path().join("min-version-package.cabal");
        let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
        assert!(
            result_chirho.is_ok(),
            "cabal build should synthesize MIN_VERSION macros from local package versions: {:?}",
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
            exported_type_synonyms_from_module_chirho, extract_imports_chirho,
            read_haskell_source_file_chirho, run_frontend_with_type_synonyms_chirho,
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
        let mut imported_synonyms_chirho = ImportedTypeSynonymsChirho::new();

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
                let frontend_result_chirho = match frontend_result_chirho {
                    Ok(frontend_result_chirho) => frontend_result_chirho,
                    Err(error_chirho) => {
                        let error_text_chirho = format!("{error_chirho:?}");
                        assert!(
                            !error_text_chirho.contains(
                                "could not find module Utils.Containers.Internal.BitUtil"
                            ),
                            "containers should stay past the old BitUtil frontier, got: {error_text_chirho}",
                        );
                        assert!(
                            module_name_chirho != "Data.IntSet.Internal",
                            "Data.IntSet.Internal should stay past the old Key alias frontier: {error_text_chirho}",
                        );
                        return;
                    }
                };
                let iface_chirho = build_iface_with_imports_chirho(
                    &frontend_result_chirho.module_chirho,
                    &ifaces_chirho,
                );
                imported_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
                    &frontend_result_chirho.module_chirho,
                    &iface_chirho,
                    &imported_synonyms_chirho,
                ));
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
            collect_package_deps_chirho, discover_modules_chirho,
            exported_type_synonyms_from_module_chirho, extract_imports_chirho,
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
        let mut imported_synonyms_chirho =
            dep_frontend_artifacts_chirho.imported_type_synonyms_chirho;

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
                let frontend_result_chirho = match frontend_result_chirho {
                    Ok(frontend_result_chirho) => frontend_result_chirho,
                    Err(error_chirho) => {
                        let error_text_chirho = format!("{error_chirho:?}");
                        assert!(
                            !error_text_chirho.contains(
                                "could not find module Utils.Containers.Internal.BitUtil"
                            ),
                            "containers should stay past the old BitUtil frontier with package seed, got: {error_text_chirho}",
                        );
                        assert!(
                            module_name_chirho != "Data.IntSet.Internal",
                            "Data.IntSet.Internal should stay past the old Key alias frontier with package seed: {error_text_chirho}",
                        );
                        return;
                    }
                };
                let iface_chirho = build_iface_with_imports_chirho(
                    &frontend_result_chirho.module_chirho,
                    &ifaces_chirho,
                );
                imported_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
                    &frontend_result_chirho.module_chirho,
                    &iface_chirho,
                    &imported_synonyms_chirho,
                ));
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
    fn compile_real_random_cabal_project_moves_past_state_alias_mismatch_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho =
            workspace_root_chirho().join(".haskelujah-packages-chirho/random-1.3.1/random.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real random compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !(error_text_chirho.contains("Error compiling System.Random.Internal")
                        && error_text_chirho.contains("expected `(StateT")
                        && error_text_chirho.contains("found `State`")),
                    "random should move past the old State/StateT alias mismatch, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn compile_real_quickcheck_cabal_project_moves_past_random_state_alias_mismatch_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/QuickCheck-2.18.0.0/QuickCheck.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real QuickCheck compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !(error_text_chirho.contains(
                        "dependency package 'random': Error compiling System.Random.Internal"
                    ) && error_text_chirho.contains("expected `(StateT")
                        && error_text_chirho.contains("found `State`")),
                    "QuickCheck should move past the old random State/StateT dependency mismatch, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn compile_real_quickcheck_cabal_project_moves_past_property_result_shadow_mismatch_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/QuickCheck-2.18.0.0/QuickCheck.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real QuickCheck compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !(error_text_chirho.contains("Error compiling Test.QuickCheck.Test")
                        && error_text_chirho.contains("expected `P.Result`")),
                    "QuickCheck should move past the old Test.QuickCheck.Test P.Result shadow mismatch, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn compile_real_quickcheck_cabal_project_moves_past_qualified_state_record_label_mismatches_chirho()
     {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/QuickCheck-2.18.0.0/QuickCheck.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real QuickCheck compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !(error_text_chirho.contains("Error compiling Test.QuickCheck.Test")
                        && error_text_chirho.contains("expected `(Maybe Int)`, found `Int`")),
                    "QuickCheck should move past the old qualified State record-label field-order mismatch, got: {error_text_chirho}",
                );
                assert!(
                    !(error_text_chirho.contains("Error compiling Test.QuickCheck.Test")
                        && error_text_chirho
                            .contains("expected `Confidence`, found `(Maybe Confidence)`")),
                    "QuickCheck should move past the old qualified State coverageConfidence mismatch, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn compile_real_quickcheck_cabal_project_moves_past_template_haskell_runio_scope_gap_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/QuickCheck-2.18.0.0/QuickCheck.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real QuickCheck compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !(error_text_chirho.contains("Error compiling Test.QuickCheck.All")
                        && error_text_chirho.contains("unbound variable: `runIO`")),
                    "QuickCheck should move past the old Test.QuickCheck.All runIO scope gap, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn compile_real_th_abstraction_cabal_project_stays_past_missing_module_stubs_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/th-abstraction-0.7.2.0/th-abstraction.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real th-abstraction compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !error_text_chirho.contains("could not find module"),
                    "th-abstraction should stay past missing module-stub failures, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn compile_real_bifunctors_cabal_project_stays_past_missing_module_stubs_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/bifunctors-5.6.3/bifunctors.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let result_chirho =
            compile_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho());
        match result_chirho {
            Ok(result_chirho) => {
                assert!(
                    !result_chirho.module_results_chirho.is_empty(),
                    "real bifunctors compile_cabal_project should compile modules",
                );
            }
            Err(error_chirho) => {
                let error_text_chirho = format!("{error_chirho}");
                assert!(
                    !error_text_chirho.contains("could not find module"),
                    "bifunctors should stay past missing module-stub failures, got: {error_text_chirho}",
                );
            }
        }
    }

    #[test]
    fn collect_local_dependency_frontend_artifacts_skips_unrelated_builtin_only_packages_chirho() {
        use haskelujah_package_chirho::{DependencyChirho, VersionConstraintChirho};

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let unrelated_pkg_root_chirho = packages_root_chirho.join("unrelated-dep-0.1.0.0");
        let unrelated_src_dir_chirho = unrelated_pkg_root_chirho.join("src/Unrelated");
        let project_dir_chirho = tmp_chirho.path().join("project-chirho");

        fs::create_dir_all(&unrelated_src_dir_chirho).unwrap();
        fs::create_dir_all(&project_dir_chirho).unwrap();
        fs::write(
            unrelated_pkg_root_chirho.join("unrelated-dep.cabal"),
            "name: unrelated-dep\nversion: 0.1.0.0\nlibrary\n  exposed-modules: Unrelated.Dep\n  hs-source-dirs: src\n",
        )
        .unwrap();
        fs::write(
            unrelated_src_dir_chirho.join("Dep.hs"),
            "module Unrelated.Dep where\nunrelatedValueChirho :: Int\nunrelatedValueChirho = 1\n",
        )
        .unwrap();

        let artifacts_chirho = collect_local_dependency_frontend_artifacts_chirho(
            &project_dir_chirho,
            &[DependencyChirho {
                package_chirho: "base".to_string(),
                constraint_chirho: VersionConstraintChirho::AnyChirho,
            }],
        )
        .expect("builtin-only dependency collection should succeed");

        assert!(
            !artifacts_chirho
                .ifaces_chirho
                .iter()
                .any(|iface_chirho| iface_chirho.name_chirho == "Unrelated.Dep"),
            "builtin-only dependency collection should not seed unrelated package ifaces",
        );
    }

    #[test]
    fn compile_cabal_project_ignores_local_dependency_executables_for_frontend_seed_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let project_dir_chirho = tmp_chirho.path().join("frontend-root-chirho");
        let project_src_dir_chirho = project_dir_chirho.join("src");
        let dep_dir_chirho = packages_root_chirho.join("dep-pkg-0.1.0.0");
        let dep_src_dir_chirho = dep_dir_chirho.join("src/Dep");
        let dep_app_dir_chirho = dep_dir_chirho.join("app");

        fs::create_dir_all(&project_src_dir_chirho).unwrap();
        fs::create_dir_all(&dep_src_dir_chirho).unwrap();
        fs::create_dir_all(&dep_app_dir_chirho).unwrap();

        fs::write(
            project_dir_chirho.join("frontend-root-chirho.cabal"),
            "name: frontend-root-chirho\n\
version: 0.1.0.0\n\
library\n\
  exposed-modules: LibChirho\n\
  hs-source-dirs: src\n\
  build-depends: base, dep-pkg >= 0.1\n",
        )
        .unwrap();
        fs::write(
            project_src_dir_chirho.join("LibChirho.hs"),
            "module LibChirho where\n\
import Dep.Lib\n\
valueChirho :: DepMarkerChirho\n\
valueChirho = DepMarkerChirho\n",
        )
        .unwrap();

        fs::write(
            dep_dir_chirho.join("dep-pkg.cabal"),
            "name: dep-pkg\n\
version: 0.1.0.0\n\
library\n\
  exposed-modules: Dep.Lib\n\
  hs-source-dirs: src\n\
  build-depends: base\n\
executable dep-tool\n\
  main-is: Main.hs\n\
  hs-source-dirs: app\n\
  build-depends: base, missing-exe-dep\n",
        )
        .unwrap();
        fs::write(
            dep_src_dir_chirho.join("Lib.hs"),
            "module Dep.Lib where\n\
data DepMarkerChirho = DepMarkerChirho\n",
        )
        .unwrap();
        fs::write(
            dep_app_dir_chirho.join("Main.hs"),
            "module Main where\n\
import Missing.Executable.Dep\n\
mainChirho :: IO ()\n\
mainChirho = pure ()\n",
        )
        .unwrap();

        let result_chirho = compile_cabal_project_chirho(
            project_dir_chirho.join("frontend-root-chirho.cabal"),
            &PackageIndexChirho::new_chirho(),
        )
        .expect("dependency frontend seeding should ignore broken dependency executables");

        let build_plan_packages_chirho: Vec<&str> = result_chirho
            .build_plan_chirho
            .steps_chirho
            .iter()
            .map(|step_chirho| step_chirho.package_chirho.as_str())
            .collect();

        assert_eq!(build_plan_packages_chirho, vec!["dep-pkg"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }

    #[test]
    fn compile_cabal_project_resolves_transitive_local_dependency_index_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let project_dir_chirho = tmp_chirho.path().join("local-root-chirho");
        let project_src_dir_chirho = project_dir_chirho.join("src");
        let transformers_dir_chirho = packages_root_chirho.join("transformers-0.6.3.0");
        let transformers_src_dir_chirho = transformers_dir_chirho.join("src/Control/Monad/Trans");
        let mtl_dir_chirho = packages_root_chirho.join("mtl-2.3.2");
        let mtl_src_dir_chirho = mtl_dir_chirho.join("src/Control/Monad/State");

        fs::create_dir_all(&project_src_dir_chirho).unwrap();
        fs::create_dir_all(&transformers_src_dir_chirho).unwrap();
        fs::create_dir_all(&mtl_src_dir_chirho).unwrap();

        fs::write(
            project_dir_chirho.join("local-root-chirho.cabal"),
            "name: local-root-chirho\n\
version: 0.1.0.0\n\
library\n\
  exposed-modules: LibChirho\n\
  hs-source-dirs: src\n\
  build-depends: base, mtl >= 2.3\n",
        )
        .unwrap();
        fs::write(
            project_src_dir_chirho.join("LibChirho.hs"),
            "module LibChirho where\n\
import Control.Monad.State.Class\n\
valueChirho :: StateMarkerChirho Int -> Int\n\
valueChirho _ = 1\n",
        )
        .unwrap();

        fs::write(
            transformers_dir_chirho.join("transformers.cabal"),
            "name: transformers\n\
version: 0.6.3.0\n\
library\n\
  exposed-modules: Control.Monad.Trans.Class\n\
  hs-source-dirs: src\n\
  build-depends: base\n",
        )
        .unwrap();
        fs::write(
            transformers_src_dir_chirho.join("Class.hs"),
            "module Control.Monad.Trans.Class where\n\
newtype TransIdentityChirho aChirho = TransIdentityChirho aChirho\n",
        )
        .unwrap();

        fs::write(
            mtl_dir_chirho.join("mtl.cabal"),
            "name: mtl\n\
version: 2.3.2\n\
library\n\
  exposed-modules: Control.Monad.State.Class\n\
  hs-source-dirs: src\n\
  build-depends: base, transformers >= 0.6\n",
        )
        .unwrap();
        fs::write(
            mtl_src_dir_chirho.join("Class.hs"),
            "module Control.Monad.State.Class where\n\
import Control.Monad.Trans.Class\n\
newtype StateMarkerChirho aChirho = StateMarkerChirho (TransIdentityChirho aChirho)\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let result_chirho = compile_cabal_project_chirho(
            project_dir_chirho.join("local-root-chirho.cabal"),
            &index_chirho,
        )
        .expect("cabal project should resolve transitive local package versions");

        let build_plan_packages_chirho: Vec<&str> = result_chirho
            .build_plan_chirho
            .steps_chirho
            .iter()
            .map(|step_chirho| step_chirho.package_chirho.as_str())
            .collect();

        assert_eq!(build_plan_packages_chirho, vec!["transformers", "mtl"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }

    #[test]
    fn compile_cabal_project_ignores_test_only_transitive_cycle_in_local_index_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let project_dir_chirho = tmp_chirho.path().join("vault-smoke-chirho");
        let project_src_dir_chirho = project_dir_chirho.join("src");
        let hashable_dir_chirho = packages_root_chirho.join("hashable-1.5.0.0");
        let hashable_src_dir_chirho = hashable_dir_chirho.join("src/Data");
        let hashable_test_dir_chirho = hashable_dir_chirho.join("test");
        let unordered_dir_chirho = packages_root_chirho.join("unordered-containers-0.2.21");
        let unordered_src_dir_chirho = unordered_dir_chirho.join("src/Data/HashMap");

        fs::create_dir_all(&project_src_dir_chirho).unwrap();
        fs::create_dir_all(&hashable_src_dir_chirho).unwrap();
        fs::create_dir_all(&hashable_test_dir_chirho).unwrap();
        fs::create_dir_all(&unordered_src_dir_chirho).unwrap();

        fs::write(
            project_dir_chirho.join("vault-smoke-chirho.cabal"),
            "name: vault-smoke-chirho\n\
version: 0.1.0.0\n\
library\n\
  exposed-modules: LibChirho\n\
  hs-source-dirs: src\n\
  build-depends: base, hashable >= 1.5\n",
        )
        .unwrap();
        fs::write(
            project_src_dir_chirho.join("LibChirho.hs"),
            "module LibChirho where\n\
import Data.Hashable\n\
valueChirho :: HashableMarkerChirho\n\
valueChirho = HashableMarkerChirho\n",
        )
        .unwrap();

        fs::write(
            hashable_dir_chirho.join("hashable.cabal"),
            "name: hashable\n\
version: 1.5.0.0\n\
library\n\
  exposed-modules: Data.Hashable\n\
  hs-source-dirs: src\n\
  build-depends: base\n\
test-suite hashable-test\n\
  type: exitcode-stdio-1.0\n\
  main-is: Spec.hs\n\
  hs-source-dirs: test\n\
  build-depends: base, unordered-containers >= 0.2\n",
        )
        .unwrap();
        fs::write(
            hashable_src_dir_chirho.join("Hashable.hs"),
            "module Data.Hashable where\n\
data HashableMarkerChirho = HashableMarkerChirho\n",
        )
        .unwrap();
        fs::write(
            hashable_test_dir_chirho.join("Spec.hs"),
            "module Main where\n\
mainChirho :: IO ()\n\
mainChirho = pure ()\n",
        )
        .unwrap();

        fs::write(
            unordered_dir_chirho.join("unordered-containers.cabal"),
            "name: unordered-containers\n\
version: 0.2.21\n\
library\n\
  exposed-modules: Data.HashMap.Strict\n\
  hs-source-dirs: src\n\
  build-depends: base, hashable >= 1.5\n",
        )
        .unwrap();
        fs::write(
            unordered_src_dir_chirho.join("Strict.hs"),
            "module Data.HashMap.Strict where\n\
data HashMapMarkerChirho = HashMapMarkerChirho\n",
        )
        .unwrap();

        let index_chirho = PackageIndexChirho::new_chirho();
        let result_chirho = compile_cabal_project_chirho(
            project_dir_chirho.join("vault-smoke-chirho.cabal"),
            &index_chirho,
        )
        .expect("test-only dependency cycles should not pollute the recursive local index");

        let build_plan_packages_chirho: Vec<&str> = result_chirho
            .build_plan_chirho
            .steps_chirho
            .iter()
            .map(|step_chirho| step_chirho.package_chirho.as_str())
            .collect();

        assert_eq!(build_plan_packages_chirho, vec!["hashable"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }

    #[test]
    fn merged_local_dependency_package_index_discovers_all_local_packages_chirho() {
        use crate::merge_local_dependency_package_index_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let hashable_dir_chirho = packages_root_chirho.join("hashable-1.5.0.0");
        let unordered_dir_chirho = packages_root_chirho.join("unordered-containers-0.2.21");
        let malformed_dir_chirho = packages_root_chirho.join("wcwidth-0.0.2");

        fs::create_dir_all(&hashable_dir_chirho).unwrap();
        fs::create_dir_all(&unordered_dir_chirho).unwrap();
        fs::create_dir_all(&malformed_dir_chirho).unwrap();

        fs::write(
            hashable_dir_chirho.join("hashable.cabal"),
            "name: hashable\n\
version: 1.5.0.0\n\
library\n\
  exposed-modules: Data.Hashable\n\
  hs-source-dirs: src\n\
  build-depends: base, os-string\n",
        )
        .unwrap();
        fs::write(
            unordered_dir_chirho.join("unordered-containers.cabal"),
            "name: unordered-containers\n\
version: 0.2.21\n\
library\n\
  exposed-modules: Data.HashMap.Strict\n\
  hs-source-dirs: src\n\
  build-depends: base, hashable >= 1.5\n",
        )
        .unwrap();
        fs::write(
            malformed_dir_chirho.join("wcwidth.cabal"),
            "name                          : wcwidth\n\
version                       : 0.0.2\n\
library\n\
  exposed-modules             : Data.Char.WCWidth\n\
  hs-source-dirs              : src\n\
  build-depends               : base\n",
        )
        .unwrap();

        let merged_index_chirho = merge_local_dependency_package_index_chirho(
            tmp_chirho.path(),
            &[],
            &PackageIndexChirho::new_chirho(),
        )
        .expect("all local package directories should be indexed");

        assert!(
            merged_index_chirho.packages_chirho.contains_key("hashable"),
            "merged local index should include hashable from the packages directory"
        );
        assert!(
            merged_index_chirho
                .packages_chirho
                .contains_key("unordered-containers"),
            "merged local index should include sibling packages even when they are not root-transitive"
        );
        assert!(
            !merged_index_chirho.packages_chirho.contains_key("wcwidth"),
            "malformed unrelated local packages should be skipped instead of breaking the whole merge"
        );
    }

    #[test]
    fn compile_cabal_project_treats_os_string_as_builtin_for_local_hashable_chirho() {
        use crate::compile_cabal_project_chirho;
        use haskelujah_package_chirho::PackageIndexChirho;

        let tmp_chirho = tempfile::tempdir().unwrap();
        let packages_root_chirho = tmp_chirho.path().join(".haskelujah-packages-chirho");
        let project_dir_chirho = tmp_chirho.path().join("hashable-smoke-chirho");
        let project_src_dir_chirho = project_dir_chirho.join("src");
        let hashable_dir_chirho = packages_root_chirho.join("hashable-1.5.0.0");
        let hashable_src_dir_chirho = hashable_dir_chirho.join("src/Data");

        fs::create_dir_all(&project_src_dir_chirho).unwrap();
        fs::create_dir_all(&hashable_src_dir_chirho).unwrap();

        fs::write(
            project_dir_chirho.join("hashable-smoke-chirho.cabal"),
            "name: hashable-smoke-chirho\n\
version: 0.1.0.0\n\
library\n\
  exposed-modules: LibChirho\n\
  hs-source-dirs: src\n\
  build-depends: base, hashable >= 1.5\n",
        )
        .unwrap();
        fs::write(
            project_src_dir_chirho.join("LibChirho.hs"),
            "module LibChirho where\n\
import Data.Hashable\n\
valueChirho :: HashableMarkerChirho\n\
valueChirho = HashableMarkerChirho\n",
        )
        .unwrap();

        fs::write(
            hashable_dir_chirho.join("hashable.cabal"),
            "name: hashable\n\
version: 1.5.0.0\n\
library\n\
  exposed-modules: Data.Hashable\n\
  hs-source-dirs: src\n\
  build-depends: base, os-string\n",
        )
        .unwrap();
        fs::write(
            hashable_src_dir_chirho.join("Hashable.hs"),
            "module Data.Hashable where\n\
data HashableMarkerChirho = HashableMarkerChirho\n",
        )
        .unwrap();

        let result_chirho = compile_cabal_project_chirho(
            project_dir_chirho.join("hashable-smoke-chirho.cabal"),
            &PackageIndexChirho::new_chirho(),
        )
        .expect("os-string should be treated as builtin for local hashable resolution");

        let build_plan_packages_chirho: Vec<&str> = result_chirho
            .build_plan_chirho
            .steps_chirho
            .iter()
            .map(|step_chirho| step_chirho.package_chirho.as_str())
            .collect();

        assert_eq!(build_plan_packages_chirho, vec!["hashable"]);
        assert_eq!(result_chirho.module_results_chirho.len(), 1);
    }

    #[test]
    fn merged_local_dependency_index_uses_library_edges_for_parsers_chain_chirho() {
        use crate::{collect_package_deps_chirho, merge_local_dependency_package_index_chirho};
        use haskelujah_package_chirho::{PackageIndexChirho, parse_cabal_chirho};

        let cabal_path_chirho = workspace_root_chirho()
            .join(".haskelujah-packages-chirho/parsers-0.12.12/parsers.cabal");
        if !cabal_path_chirho.exists() {
            return;
        }

        let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).unwrap();
        let package_chirho = parse_cabal_chirho(&cabal_content_chirho);
        let project_dir_chirho = cabal_path_chirho.parent().unwrap();
        let root_deps_chirho = collect_package_deps_chirho(&package_chirho);
        let merged_index_chirho = merge_local_dependency_package_index_chirho(
            project_dir_chirho,
            &root_deps_chirho,
            &PackageIndexChirho::new_chirho(),
        )
        .expect("real parsers local dependency index should merge");

        let hashable_deps_chirho = merged_index_chirho
            .packages_chirho
            .get("hashable")
            .and_then(|versions_chirho| versions_chirho.first())
            .map(|meta_chirho| {
                meta_chirho
                    .dependencies_chirho
                    .iter()
                    .map(|dep_chirho| dep_chirho.package_chirho.clone())
                    .collect::<Vec<_>>()
            })
            .expect("hashable metadata should be present");
        let unordered_deps_chirho = merged_index_chirho
            .packages_chirho
            .get("unordered-containers")
            .and_then(|versions_chirho| versions_chirho.first())
            .map(|meta_chirho| {
                meta_chirho
                    .dependencies_chirho
                    .iter()
                    .map(|dep_chirho| dep_chirho.package_chirho.clone())
                    .collect::<Vec<_>>()
            })
            .expect("unordered-containers metadata should be present");
        let scientific_deps_chirho = merged_index_chirho
            .packages_chirho
            .get("scientific")
            .and_then(|versions_chirho| versions_chirho.first())
            .map(|meta_chirho| {
                meta_chirho
                    .dependencies_chirho
                    .iter()
                    .map(|dep_chirho| dep_chirho.package_chirho.clone())
                    .collect::<Vec<_>>()
            })
            .expect("scientific metadata should be present");

        assert!(
            !hashable_deps_chirho.contains(&"unordered-containers".to_string()),
            "hashable library metadata should not depend on unordered-containers, got {:?}",
            hashable_deps_chirho
        );
        assert!(
            !unordered_deps_chirho.contains(&"scientific".to_string()),
            "unordered-containers library metadata should not depend on scientific, got {:?}",
            unordered_deps_chirho
        );
        assert!(
            !scientific_deps_chirho.contains(&"charset".to_string()),
            "scientific library metadata should not depend on charset, got {:?}",
            scientific_deps_chirho
        );
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
    fn compile_project_imported_record_construction_uses_field_labels_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { terminalMiniChirho :: Int\n\
  , maxSuccessTestsMiniChirho :: Int\n\
  , maxDiscardedRatioMiniChirho :: Int\n\
  , coverageConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  , expectedMiniChirho :: Bool\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import StateMiniChirho\n\
\n\
buildStateMiniChirho :: StateMiniChirho\n\
buildStateMiniChirho = MkStateMiniChirho\n\
  { coverageConfidenceMiniChirho = Just (ConfidenceMiniChirho { certaintyMiniChirho = 1 })\n\
  , replayStartSizeMiniChirho = Nothing\n\
  , maxDiscardedRatioMiniChirho = 10\n\
  , terminalMiniChirho = 0\n\
  , maxSuccessTestsMiniChirho = 100\n\
  , maxTestSizeMiniChirho = 25\n\
  , expectedMiniChirho = True\n\
  }\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "project compilation should order imported record construction by field labels, not source order: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_project_imported_record_construction_skips_omitted_maybe_fields_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { maxSuccessTestsMiniChirho :: Int\n\
  , maxDiscardedRatioMiniChirho :: Int\n\
  , coverageConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , numTotMaxShrinksMiniChirho :: Int\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import StateMiniChirho\n\
\n\
buildStateMiniChirho :: StateMiniChirho\n\
buildStateMiniChirho = MkStateMiniChirho\n\
  { replayStartSizeMiniChirho = Nothing\n\
  , maxSuccessTestsMiniChirho = 100\n\
  , maxTestSizeMiniChirho = 25\n\
  , maxDiscardedRatioMiniChirho = 10\n\
  , numTotMaxShrinksMiniChirho = 5\n\
  }\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "imported record construction should preserve field alignment when an intermediate Maybe field is omitted: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_project_imported_record_construction_mixes_qualified_labels_after_omitted_maybe_field_chirho()
     {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { maxSuccessTestsMiniChirho :: Int\n\
  , maxDiscardedRatioMiniChirho :: Int\n\
  , coverageConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , numTotMaxShrinksMiniChirho :: Int\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  , labelsMiniChirho :: [(String, Int)]\n\
  , classesMiniChirho :: [(String, Int)]\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import StateMiniChirho\n\
import qualified StateMiniChirho as S\n\
\n\
buildStateMiniChirho :: StateMiniChirho\n\
buildStateMiniChirho = MkStateMiniChirho\n\
  { maxSuccessTestsMiniChirho = 100\n\
  , maxDiscardedRatioMiniChirho = 10\n\
  , replayStartSizeMiniChirho = Nothing\n\
  , maxTestSizeMiniChirho = 25\n\
  , numTotMaxShrinksMiniChirho = 5\n\
  , S.labelsMiniChirho = []\n\
  , S.classesMiniChirho = []\n\
  }\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "imported record construction should still match later qualified field labels after omitting an intermediate Maybe field: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_project_imported_record_selectors_preserve_maybe_field_types_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { maybeConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import Data.Maybe (fromMaybe)\n\
import StateMiniChirho\n\
\n\
confidenceLevelMiniChirho :: StateMiniChirho -> Integer\n\
confidenceLevelMiniChirho stateMiniChirho =\n\
  case maybeConfidenceMiniChirho stateMiniChirho of\n\
    Just confidenceMiniChirho -> certaintyMiniChirho confidenceMiniChirho\n\
    Nothing -> 0\n\
\n\
sizeOrDefaultMiniChirho :: StateMiniChirho -> Maybe Int -> Int\n\
sizeOrDefaultMiniChirho stateMiniChirho maybeSizeMiniChirho =\n\
  fromMaybe (maxTestSizeMiniChirho stateMiniChirho) maybeSizeMiniChirho\n\
\n\
replayOrSizeMiniChirho :: StateMiniChirho -> Int\n\
replayOrSizeMiniChirho stateMiniChirho =\n\
  fromMaybe (maxTestSizeMiniChirho stateMiniChirho) (replayStartSizeMiniChirho stateMiniChirho)\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "project compilation should preserve imported record selector field types, especially Maybe-valued fields: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_project_imported_record_patterns_preserve_maybe_field_types_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { coverageConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import StateMiniChirho\n\
\n\
confidenceLevelMiniChirho :: StateMiniChirho -> Integer\n\
confidenceLevelMiniChirho MkStateMiniChirho{coverageConfidenceMiniChirho = Just confidenceMiniChirho} =\n\
  certaintyMiniChirho confidenceMiniChirho\n\
confidenceLevelMiniChirho _ = 0\n\
\n\
combinedSizeMiniChirho :: StateMiniChirho -> Int\n\
combinedSizeMiniChirho MkStateMiniChirho{replayStartSizeMiniChirho = Just startMiniChirho, maxTestSizeMiniChirho = maxSizeMiniChirho} =\n\
  startMiniChirho + maxSizeMiniChirho\n\
combinedSizeMiniChirho MkStateMiniChirho{maxTestSizeMiniChirho = maxSizeMiniChirho} = maxSizeMiniChirho\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "project compilation should preserve imported record-pattern field types, especially Maybe-valued fields: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn compile_project_imported_quickcheck_style_state_patterns_preserve_field_types_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { maxSuccessTestsMiniChirho :: Int\n\
  , maxDiscardedRatioMiniChirho :: Int\n\
  , coverageConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  , numSuccessTestsMiniChirho :: Int\n\
  , numRecentlyDiscardedTestsMiniChirho :: Int\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import StateMiniChirho\n\
\n\
computeSizeMiniChirho :: StateMiniChirho -> Int\n\
computeSizeMiniChirho MkStateMiniChirho{replayStartSizeMiniChirho = Just sMiniChirho,numSuccessTestsMiniChirho = 0,numRecentlyDiscardedTestsMiniChirho=0} = sMiniChirho\n\
computeSizeMiniChirho MkStateMiniChirho{maxSuccessTestsMiniChirho = msMiniChirho, maxTestSizeMiniChirho = mtsMiniChirho, maxDiscardedRatioMiniChirho = mdMiniChirho,numSuccessTestsMiniChirho=nMiniChirho,numRecentlyDiscardedTestsMiniChirho=dMiniChirho} =\n\
  msMiniChirho + mtsMiniChirho + mdMiniChirho + nMiniChirho + dMiniChirho\n\
\n\
coverageKnownSufficientMiniChirho :: StateMiniChirho -> Integer\n\
coverageKnownSufficientMiniChirho stateMiniChirho@MkStateMiniChirho{coverageConfidenceMiniChirho=Just confidenceMiniChirho} =\n\
  certaintyMiniChirho confidenceMiniChirho + fromIntegral (maxTestSizeMiniChirho stateMiniChirho)\n\
coverageKnownSufficientMiniChirho _ = 0\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "QuickCheck-style imported State patterns should preserve Maybe and Int field types: {:?}",
            result_chirho.err()
        );
    }

    #[test]
    fn collect_frontend_artifacts_exports_record_selectors_for_imports_chirho() {
        use crate::{
            ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
            collect_frontend_artifacts_from_module_sources_chirho,
        };
        use haskelujah_naming_chirho::builtin_module_ifaces_chirho;

        let module_sources_chirho = vec![(
            "StateMiniChirho".to_string(),
            "StateMiniChirho.hs".to_string(),
            "module StateMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { maybeConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  , replayStartSizeMiniChirho :: Maybe Int\n\
  , maxTestSizeMiniChirho :: Int\n\
  }\n"
            .to_string(),
        )];

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
            module_sources_chirho,
            &mut source_map_chirho,
            builtin_module_ifaces_chirho(),
            std::collections::HashMap::new(),
            ImportedTypeSynonymsChirho::new(),
            ImportedTypeFamiliesChirho::new(),
            true,
        )
        .expect("frontend artifact collection should succeed for the defining module");

        assert!(
            artifacts_chirho
                .imported_types_chirho
                .contains_key("maybeConfidenceMiniChirho"),
            "record selector should be re-exported into imported type seeds: {:?}",
            artifacts_chirho
                .imported_types_chirho
                .keys()
                .filter(|name_chirho| name_chirho.contains("MiniChirho"))
                .cloned()
                .collect::<Vec<_>>()
        );
        assert!(
            artifacts_chirho
                .imported_types_chirho
                .contains_key("maxTestSizeMiniChirho"),
            "plain record selector should be available to downstream modules: {:?}",
            artifacts_chirho
                .imported_types_chirho
                .keys()
                .filter(|name_chirho| name_chirho.contains("MiniChirho"))
                .cloned()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn compile_project_qualified_record_update_fields_preserve_nested_binders_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("StateMiniChirho.hs"),
            "module StateMiniChirho where\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { labelsMiniChirho :: [(String, Int)]\n\
  , tablesMiniChirho :: [(String, String)]\n\
  }\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("UseMiniChirho.hs"),
            "module UseMiniChirho where\n\
import qualified StateMiniChirho as S\n\
\n\
updateStateMiniChirho :: S.StateMiniChirho -> [(String, Bool)] -> [(String, String)] -> S.StateMiniChirho\n\
updateStateMiniChirho stateMiniChirho classPairsMiniChirho tablePairsMiniChirho =\n\
  stateMiniChirho\n\
    { S.labelsMiniChirho = [ (labelMiniChirho, if flagMiniChirho then 1 else 0)\n\
                           | (labelMiniChirho, flagMiniChirho) <- classPairsMiniChirho\n\
                           ]\n\
    , S.tablesMiniChirho =\n\
        foldr\n\
          (\\(tableMiniChirho, valueMiniChirho) accMiniChirho -> (tableMiniChirho, valueMiniChirho) : accMiniChirho)\n\
          []\n\
          tablePairsMiniChirho\n\
    }\n",
        )
        .unwrap();

        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let result_chirho = compile_project_dir_chirho(tmp_chirho.path(), &mut source_map_chirho);
        assert!(
            result_chirho.is_ok(),
            "qualified record-update fields should keep their nested tuple binders and list-comprehension binders intact: {:?}",
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
