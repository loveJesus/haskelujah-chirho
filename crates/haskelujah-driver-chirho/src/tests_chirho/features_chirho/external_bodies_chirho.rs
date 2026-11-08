// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Executables link the bodies of their store dependencies, at exactly the versions the strict
//! build plan selects (row 703).
//!
//! Before this, a build merged only the project's own modules, so a dependency's functions were
//! referenced but never defined in the executable's Core. Then the bodies were linked by package
//! name alone, so a bound could exclude the version that was linked. Each test builds against a
//! scratch store and reads either the refusal or the merged Core's bindings.

#[cfg(test)]
mod tests_chirho {
    use crate::build_cabal_project_chirho;
    use haskelujah_package_chirho::{
        BuildStepChirho, PackageIndexChirho, SOURCE_STORE_DIR_NAME_CHIRHO, parse_version_chirho,
    };
    use std::fs;
    use std::path::{Path, PathBuf};

    fn write_chirho(base_chirho: &Path, files_chirho: &[(&str, &str)]) {
        for (relative_chirho, text_chirho) in files_chirho {
            let path_chirho = base_chirho.join(relative_chirho);
            fs::create_dir_all(path_chirho.parent().unwrap()).unwrap();
            fs::write(path_chirho, text_chirho).unwrap();
        }
    }

    const P_ONE_CHIRHO: &str =
        "module DepPChirho (pValueChirho) where\n\npValueChirho :: Int\npValueChirho = 1\n";
    const USES_P_CHIRHO: &str = "module Main (main) where\n\nimport DepPChirho (pValueChirho)\n\nmain :: IO ()\nmain = print pValueChirho\n";
    const C_CHIRHO: &str =
        "module DepCChirho (cValueChirho) where\n\ncValueChirho :: Int\ncValueChirho = 1\n";
    const A_USES_C_CHIRHO: &str = "module DepAChirho (aValueChirho) where\n\nimport DepCChirho (cValueChirho)\n\naValueChirho :: Int\naValueChirho = cValueChirho\n";
    const UNREACHABLE_MAIN_CHIRHO: &str =
        "module Main (main) where\n\nmain :: IO ()\nmain = putStrLn \"unreachable\"\n";

    /// The .cabal text of a one-module library package.
    fn library_cabal_chirho(
        name_chirho: &str,
        version_chirho: &str,
        depends_chirho: &str,
        module_chirho: &str,
    ) -> String {
        format!(
            "cabal-version: 2.4\nname: {name_chirho}\nversion: {version_chirho}\nbuild-type: Simple\n\nlibrary\n  exposed-modules: {module_chirho}\n  build-depends: {depends_chirho}\n  hs-source-dirs: src\n  default-language: Haskell2010\n"
        )
    }

    /// A library package `<name>-<version>` in the scratch store with one module.
    fn store_library_version_chirho(
        root_chirho: &Path,
        name_chirho: &str,
        version_chirho: &str,
        depends_chirho: &str,
        module_chirho: &str,
        text_chirho: &str,
    ) {
        let package_dir_chirho = root_chirho
            .join(SOURCE_STORE_DIR_NAME_CHIRHO)
            .join(format!("{name_chirho}-{version_chirho}"));
        write_chirho(
            &package_dir_chirho,
            &[
                (
                    &format!("{name_chirho}.cabal"),
                    &library_cabal_chirho(
                        name_chirho,
                        version_chirho,
                        depends_chirho,
                        module_chirho,
                    ),
                ),
                (&format!("src/{module_chirho}.hs"), text_chirho),
            ],
        );
    }

    /// A library package `<name>-1.0` in the scratch store with one module.
    fn store_library_chirho(
        root_chirho: &Path,
        name_chirho: &str,
        depends_chirho: &str,
        module_chirho: &str,
        text_chirho: &str,
    ) {
        store_library_version_chirho(
            root_chirho,
            name_chirho,
            "1.0",
            depends_chirho,
            module_chirho,
            text_chirho,
        );
    }

    /// An executable project `app-chirho` depending on `depends_chirho`; returns its .cabal path.
    fn app_chirho(root_chirho: &Path, depends_chirho: &str, main_chirho: &str) -> PathBuf {
        let app_dir_chirho = root_chirho.join("app-chirho");
        write_chirho(
            &app_dir_chirho,
            &[
                (
                    "app-chirho.cabal",
                    &format!(
                        "cabal-version: 2.4\nname: app-chirho\nversion: 0.1\nbuild-type: Simple\n\nexecutable app-chirho-exe\n  main-is: Main.hs\n  hs-source-dirs: app\n  build-depends: {depends_chirho}\n  default-language: Haskell2010\n"
                    ),
                ),
                ("app/Main.hs", main_chirho),
            ],
        );
        app_dir_chirho.join("app-chirho.cabal")
    }

    /// The project `app-chirho` with a library and an executable, each with its own bounds; its
    /// executable uses `DepPChirho`. Returns its .cabal path.
    fn app_with_library_chirho(
        root_chirho: &Path,
        library_depends_chirho: &str,
        executable_depends_chirho: &str,
    ) -> PathBuf {
        let app_dir_chirho = root_chirho.join("app-chirho");
        write_chirho(
            &app_dir_chirho,
            &[
                (
                    "app-chirho.cabal",
                    &format!(
                        "cabal-version: 2.4\nname: app-chirho\nversion: 0.1\nbuild-type: Simple\n\nlibrary\n  exposed-modules: AppLibChirho\n  hs-source-dirs: src\n  build-depends: {library_depends_chirho}\n  default-language: Haskell2010\n\nexecutable app-chirho-exe\n  main-is: Main.hs\n  hs-source-dirs: app\n  build-depends: {executable_depends_chirho}\n  default-language: Haskell2010\n"
                    ),
                ),
                (
                    "src/AppLibChirho.hs",
                    "module AppLibChirho (appLibChirho) where\n\nappLibChirho :: Int\nappLibChirho = 3\n",
                ),
                ("app/Main.hs", USES_P_CHIRHO),
            ],
        );
        app_dir_chirho.join("app-chirho.cabal")
    }

    /// Build `cabal_path_chirho` and return the refusal, which must come from resolution, before
    /// any package is compiled.
    fn resolution_refusal_chirho(cabal_path_chirho: &Path, why_chirho: &str) -> String {
        let error_chirho =
            build_cabal_project_chirho(cabal_path_chirho, &PackageIndexChirho::new_chirho())
                .expect_err(why_chirho);
        assert!(
            error_chirho.starts_with("dependency resolution failed"),
            "{error_chirho}"
        );
        error_chirho
    }

    fn binding_names_chirho(core_chirho: &haskelujah_core_chirho::CoreModuleChirho) -> Vec<String> {
        core_chirho
            .bindings_chirho
            .iter()
            .map(|binding_chirho| binding_chirho.binder_chirho.name_chirho.clone())
            .collect()
    }

    fn defines_chirho(names_chirho: &[String], function_chirho: &str) -> bool {
        names_chirho.iter().any(|name_chirho| {
            name_chirho == function_chirho || name_chirho.ends_with(&format!(".{function_chirho}"))
        })
    }

    #[test]
    fn an_executable_links_its_dependency_packages_bodies_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(
            root_chirho,
            "dep-b-chirho",
            "base",
            "DepBChirho",
            "module DepBChirho (bValueChirho) where\n\nbValueChirho :: Int\nbValueChirho = 41\n",
        );
        store_library_chirho(
            root_chirho,
            "dep-a-chirho",
            "base, dep-b-chirho",
            "DepAChirho",
            "module DepAChirho (greetChirho, aValueChirho) where\n\nimport DepBChirho (bValueChirho)\n\ngreetChirho :: String -> String\ngreetChirho nameChirho = \"dependency greets \" ++ nameChirho\n\naValueChirho :: Int\naValueChirho = bValueChirho + 1\n",
        );
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-a-chirho",
            "module Main (main) where\n\nimport DepAChirho (aValueChirho, greetChirho)\n\nmain :: IO ()\nmain = do\n  putStrLn (greetChirho \"you\")\n  print aValueChirho\n",
        );
        let built_chirho =
            build_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho())
                .unwrap();
        assert_eq!(built_chirho.executables_chirho.len(), 1);
        let names_chirho = binding_names_chirho(&built_chirho.executables_chirho[0].core_chirho);
        for function_chirho in ["greetChirho", "aValueChirho", "bValueChirho", "main"] {
            assert!(
                defines_chirho(&names_chirho, function_chirho),
                "the merged Core must define {function_chirho}; it defines {names_chirho:?}"
            );
        }
        assert!(!built_chirho.executables_chirho[0].llvm_ir_chirho.is_empty());
    }

    #[test]
    fn two_dependency_packages_defining_one_module_are_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        for (name_chirho, value_chirho) in [("dep-one-chirho", "1"), ("dep-two-chirho", "2")] {
            store_library_chirho(
                root_chirho,
                name_chirho,
                "base",
                "SharedChirho",
                &format!(
                    "module SharedChirho (sharedValueChirho) where\n\nsharedValueChirho :: Int\nsharedValueChirho = {value_chirho}\n"
                ),
            );
        }
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-one-chirho, dep-two-chirho",
            UNREACHABLE_MAIN_CHIRHO,
        );
        let error_chirho =
            build_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho())
                .expect_err("one module from two packages must not be linked");
        assert!(error_chirho.contains("SharedChirho"), "{error_chirho}");
        assert!(error_chirho.contains("defined by both"), "{error_chirho}");
    }

    #[test]
    fn a_dependency_that_fails_to_compile_fails_the_build_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(
            root_chirho,
            "dep-broken-chirho",
            "base",
            "DepBrokenChirho",
            "module DepBrokenChirho (brokenChirho) where\n\nbrokenChirho :: Int\nbrokenChirho = \"not an Int\"\n",
        );
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-broken-chirho",
            "module Main (main) where\n\nimport DepBrokenChirho (brokenChirho)\n\nmain :: IO ()\nmain = print brokenChirho\n",
        );
        let error_chirho =
            build_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho())
                .expect_err("a body that does not compile cannot be linked");
        assert!(error_chirho.contains("dep-broken-chirho"), "{error_chirho}");
    }

    #[test]
    fn an_excluded_store_version_is_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(
            root_chirho,
            "dep-p-chirho",
            "base",
            "DepPChirho",
            P_ONE_CHIRHO,
        );
        let cabal_path_chirho = app_chirho(root_chirho, "base, dep-p-chirho >=2", USES_P_CHIRHO);
        let error_chirho = resolution_refusal_chirho(
            &cabal_path_chirho,
            "the store holds only dep-p-chirho 1.0, outside >=2",
        );
        assert!(error_chirho.contains("dep-p-chirho"), "{error_chirho}");
    }

    #[test]
    fn a_dependency_missing_from_the_store_is_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(
            root_chirho,
            "dep-p-chirho",
            "base",
            "DepPChirho",
            P_ONE_CHIRHO,
        );
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-absent-chirho",
            UNREACHABLE_MAIN_CHIRHO,
        );
        let error_chirho = resolution_refusal_chirho(
            &cabal_path_chirho,
            "no store directory holds dep-absent-chirho",
        );
        assert!(error_chirho.contains("dep-absent-chirho"), "{error_chirho}");
    }

    #[test]
    fn incompatible_diamond_bounds_are_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        for version_chirho in ["1.0", "2.0"] {
            store_library_version_chirho(
                root_chirho,
                "dep-c-chirho",
                version_chirho,
                "base",
                "DepCChirho",
                C_CHIRHO,
            );
        }
        store_library_chirho(
            root_chirho,
            "dep-a-chirho",
            "base, dep-c-chirho >=2",
            "DepAChirho",
            A_USES_C_CHIRHO,
        );
        store_library_chirho(
            root_chirho,
            "dep-b-chirho",
            "base, dep-c-chirho <2",
            "DepBChirho",
            "module DepBChirho (bValueChirho) where\n\nimport DepCChirho (cValueChirho)\n\nbValueChirho :: Int\nbValueChirho = cValueChirho\n",
        );
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-a-chirho, dep-b-chirho",
            "module Main (main) where\n\nimport DepAChirho (aValueChirho)\nimport DepBChirho (bValueChirho)\n\nmain :: IO ()\nmain = print (aValueChirho + bValueChirho)\n",
        );
        let error_chirho = resolution_refusal_chirho(
            &cabal_path_chirho,
            "no dep-c-chirho satisfies both >=2 and <2",
        );
        assert!(
            error_chirho.contains("dep-c-chirho") && error_chirho.contains("dep-b-chirho"),
            "{error_chirho}"
        );
    }

    #[test]
    fn an_unsatisfiable_transitive_bound_is_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(root_chirho, "dep-c-chirho", "base", "DepCChirho", C_CHIRHO);
        store_library_chirho(
            root_chirho,
            "dep-a-chirho",
            "base, dep-c-chirho >=2",
            "DepAChirho",
            A_USES_C_CHIRHO,
        );
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-a-chirho",
            "module Main (main) where\n\nimport DepAChirho (aValueChirho)\n\nmain :: IO ()\nmain = print aValueChirho\n",
        );
        let error_chirho = resolution_refusal_chirho(
            &cabal_path_chirho,
            "dep-a-chirho needs dep-c-chirho >=2, and the store holds only 1.0",
        );
        assert!(error_chirho.contains("dep-c-chirho"), "{error_chirho}");
    }

    #[test]
    fn a_later_executable_bound_is_conjoined_with_the_library_bound_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(
            root_chirho,
            "dep-p-chirho",
            "base",
            "DepPChirho",
            P_ONE_CHIRHO,
        );
        let cabal_path_chirho = app_with_library_chirho(
            root_chirho,
            "base, dep-p-chirho >=1",
            "base, dep-p-chirho >=2",
        );
        let error_chirho = resolution_refusal_chirho(
            &cabal_path_chirho,
            "1.0 meets the library's >=1 but not the executable's >=2",
        );
        assert!(error_chirho.contains("dep-p-chirho"), "{error_chirho}");
    }

    #[test]
    fn the_build_links_the_planned_version_not_the_newest_directory_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        for (version_chirho, only_chirho, text_chirho) in [
            ("1.0", "oneOnlyChirho", "one"),
            ("2.0", "twoOnlyChirho", "two"),
        ] {
            store_library_version_chirho(
                root_chirho,
                "dep-p-chirho",
                version_chirho,
                "base",
                "DepPChirho",
                &format!(
                    "module DepPChirho (labelChirho, {only_chirho}) where\n\nlabelChirho :: String\nlabelChirho = {only_chirho}\n\n{only_chirho} :: String\n{only_chirho} = \"{text_chirho}\"\n"
                ),
            );
        }
        let cabal_path_chirho = app_chirho(
            root_chirho,
            "base, dep-p-chirho <2",
            "module Main (main) where\n\nimport DepPChirho (labelChirho)\n\nmain :: IO ()\nmain = putStrLn labelChirho\n",
        );
        let built_chirho =
            build_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho())
                .unwrap();
        assert_eq!(
            built_chirho.build_plan_chirho.steps_chirho,
            vec![BuildStepChirho {
                package_chirho: "dep-p-chirho".to_string(),
                version_chirho: parse_version_chirho("1.0").unwrap(),
            }]
        );
        let names_chirho = binding_names_chirho(&built_chirho.executables_chirho[0].core_chirho);
        assert!(
            defines_chirho(&names_chirho, "oneOnlyChirho"),
            "{names_chirho:?}"
        );
        assert!(
            !defines_chirho(&names_chirho, "twoOnlyChirho"),
            "{names_chirho:?}"
        );
    }

    #[test]
    fn two_store_directories_holding_the_planned_version_are_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let root_chirho = scratch_chirho.path();
        store_library_chirho(
            root_chirho,
            "dep-p-chirho",
            "base",
            "DepPChirho",
            P_ONE_CHIRHO,
        );
        write_chirho(
            &root_chirho
                .join(SOURCE_STORE_DIR_NAME_CHIRHO)
                .join("dep-p-chirho"),
            &[
                (
                    "dep-p-chirho.cabal",
                    &library_cabal_chirho("dep-p-chirho", "1.0", "base", "DepPChirho"),
                ),
                ("src/DepPChirho.hs", P_ONE_CHIRHO),
            ],
        );
        let cabal_path_chirho = app_chirho(root_chirho, "base, dep-p-chirho", USES_P_CHIRHO);
        let error_chirho =
            build_cabal_project_chirho(&cabal_path_chirho, &PackageIndexChirho::new_chirho())
                .expect_err("two directories hold dep-p-chirho 1.0, so its body is ambiguous");
        assert!(
            error_chirho.contains("store directories hold that version"),
            "{error_chirho}"
        );
    }
}
