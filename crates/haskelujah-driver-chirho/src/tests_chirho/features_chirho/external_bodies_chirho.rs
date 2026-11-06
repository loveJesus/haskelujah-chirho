// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Executables link the bodies of their store dependencies (row 703).
//!
//! Before this, a build merged only the project's own modules, so a dependency's functions were
//! referenced but never defined in the executable's Core. Each test builds against a scratch
//! store and reads the merged Core's bindings.

#[cfg(test)]
mod tests_chirho {
    use crate::build_cabal_project_chirho;
    use haskelujah_package_chirho::{PackageIndexChirho, SOURCE_STORE_DIR_NAME_CHIRHO};
    use std::fs;
    use std::path::{Path, PathBuf};

    fn write_chirho(base_chirho: &Path, files_chirho: &[(&str, &str)]) {
        for (relative_chirho, text_chirho) in files_chirho {
            let path_chirho = base_chirho.join(relative_chirho);
            fs::create_dir_all(path_chirho.parent().unwrap()).unwrap();
            fs::write(path_chirho, text_chirho).unwrap();
        }
    }

    /// A library package `<name>-1.0` in the scratch store with one module.
    fn store_library_chirho(
        root_chirho: &Path,
        name_chirho: &str,
        depends_chirho: &str,
        module_chirho: &str,
        text_chirho: &str,
    ) {
        let package_dir_chirho = root_chirho
            .join(SOURCE_STORE_DIR_NAME_CHIRHO)
            .join(format!("{name_chirho}-1.0"));
        write_chirho(
            &package_dir_chirho,
            &[
                (
                    &format!("{name_chirho}.cabal"),
                    &format!(
                        "cabal-version: 2.4\nname: {name_chirho}\nversion: 1.0\nbuild-type: Simple\n\nlibrary\n  exposed-modules: {module_chirho}\n  build-depends: {depends_chirho}\n  hs-source-dirs: src\n  default-language: Haskell2010\n"
                    ),
                ),
                (&format!("src/{module_chirho}.hs"), text_chirho),
            ],
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
            "module Main (main) where\n\nmain :: IO ()\nmain = putStrLn \"unreachable\"\n",
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
}
