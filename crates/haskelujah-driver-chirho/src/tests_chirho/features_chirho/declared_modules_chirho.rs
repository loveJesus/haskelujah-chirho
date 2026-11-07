// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A build refuses a declared module that no source directory holds, by name (row 703).
//!
//! Before this, discovery skipped the missing file without a word, so an import could select a
//! builtin module of the same name instead. Each test lays out a scratch project and reads either
//! the refusal or what the build compiled. Test suites stay best effort, and a package's own
//! Cabal-generated `Paths_`/`PackageInfo_` modules are not source obligations.

#[cfg(test)]
mod tests_chirho {
    use crate::{
        build_cabal_project_chirho, compile_cabal_project_chirho, discover_modules_chirho,
    };
    use haskelujah_package_chirho::{
        PackageIndexChirho, SOURCE_STORE_DIR_NAME_CHIRHO, parse_cabal_chirho,
    };
    use std::fs;
    use std::path::Path;

    /// The refusal's own token, which no other build error carries.
    const REFUSAL_CHIRHO: &str = "declared source modules are missing";

    fn write_chirho(base_chirho: &Path, files_chirho: &[(&str, &str)]) {
        for (relative_chirho, text_chirho) in files_chirho {
            let path_chirho = base_chirho.join(relative_chirho);
            fs::create_dir_all(path_chirho.parent().unwrap()).unwrap();
            fs::write(path_chirho, text_chirho).unwrap();
        }
    }

    fn build_chirho(cabal_chirho: &Path) -> Result<crate::CabalBuildResultChirho, String> {
        build_cabal_project_chirho(cabal_chirho, &PackageIndexChirho::new_chirho())
    }

    /// A project-local module whose name a builtin module also has.
    const HOME_DATA_LIST_CHIRHO: &str =
        "module Data.List (homeValueChirho) where\n\nhomeValueChirho :: Int\nhomeValueChirho = 7\n";
    const USES_HOME_DATA_LIST_CHIRHO: &str = "module Main (main) where\n\nimport qualified Data.List as ListChirho\n\nmain :: IO ()\nmain = print ListChirho.homeValueChirho\n";
    const PLAIN_MAIN_CHIRHO: &str =
        "module Main (main) where\n\nmain :: IO ()\nmain = print (1 :: Int)\n";
    /// The executable `probe-chirho`, whose other module is `Data.List`, as row 709's helper wrote it.
    const DATA_LIST_PROBE_CABAL_CHIRHO: &str = "cabal-version: 2.4\nname: probe-chirho\nversion: 0.1\nbuild-type: Simple\n\nexecutable probe-chirho\n  main-is: Main.hs\n  other-modules: Data.List\n  hs-source-dirs: .\n  build-depends: base\n  default-language: Haskell2010\n";

    #[test]
    fn a_misplaced_declared_module_is_refused_by_name_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        write_chirho(
            scratch_chirho.path(),
            &[
                ("probe-chirho.cabal", DATA_LIST_PROBE_CABAL_CHIRHO),
                ("Main.hs", USES_HOME_DATA_LIST_CHIRHO),
                ("Data.List.hs", HOME_DATA_LIST_CHIRHO),
            ],
        );
        let error_chirho = build_chirho(&scratch_chirho.path().join("probe-chirho.cabal"))
            .expect_err("a declared module at the wrong path must not build against the builtin");
        for token_chirho in [
            REFUSAL_CHIRHO,
            "executable probe-chirho",
            "Data.List",
            "Data/List.hs",
        ] {
            assert!(
                error_chirho.contains(token_chirho),
                "{token_chirho}: {error_chirho}"
            );
        }
    }

    #[test]
    fn the_canonical_declared_module_builds_from_its_own_source_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        write_chirho(
            scratch_chirho.path(),
            &[
                ("probe-chirho.cabal", DATA_LIST_PROBE_CABAL_CHIRHO),
                ("Main.hs", USES_HOME_DATA_LIST_CHIRHO),
                ("Data/List.hs", HOME_DATA_LIST_CHIRHO),
            ],
        );
        let built_chirho = build_chirho(&scratch_chirho.path().join("probe-chirho.cabal"))
            .expect("the canonical Data/List.hs builds");
        assert_eq!(built_chirho.executables_chirho.len(), 1);
        let order_chirho = &built_chirho.executables_chirho[0].compilation_order_chirho;
        assert!(
            order_chirho
                .iter()
                .any(|module_chirho| module_chirho == "Data.List"),
            "the home Data.List is compiled: {order_chirho:?}"
        );
    }

    #[test]
    fn a_missing_library_module_is_refused_by_name_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        let library_cabal_chirho = "cabal-version: 2.4\nname: lib-chirho\nversion: 0.1\nbuild-type: Simple\n\nlibrary\n  exposed-modules: LibChirho\n  other-modules: LibChirho.InternalChirho\n  hs-source-dirs: src\n  build-depends: base\n  default-language: Haskell2010\n";
        write_chirho(
            scratch_chirho.path(),
            &[
                ("lib-chirho.cabal", library_cabal_chirho),
                (
                    "src/LibChirho.hs",
                    "module LibChirho (libValueChirho) where\n\nlibValueChirho :: Int\nlibValueChirho = 1\n",
                ),
            ],
        );
        let error_chirho = compile_cabal_project_chirho(
            scratch_chirho.path().join("lib-chirho.cabal"),
            &PackageIndexChirho::new_chirho(),
        )
        .expect_err("a library-only build requires every declared library module");
        for token_chirho in [
            REFUSAL_CHIRHO,
            "library lists module LibChirho.InternalChirho",
            "LibChirho/InternalChirho.hs",
            "[src]",
        ] {
            assert!(
                error_chirho.contains(token_chirho),
                "{token_chirho}: {error_chirho}"
            );
        }

        // The same library under an executable's build is refused the same way.
        let executable_cabal_chirho = format!(
            "{library_cabal_chirho}\nexecutable lib-chirho-exe\n  main-is: Main.hs\n  hs-source-dirs: app\n  build-depends: base\n  default-language: Haskell2010\n"
        );
        write_chirho(
            scratch_chirho.path(),
            &[
                ("lib-chirho.cabal", &executable_cabal_chirho),
                ("app/Main.hs", PLAIN_MAIN_CHIRHO),
            ],
        );
        let error_chirho = build_chirho(&scratch_chirho.path().join("lib-chirho.cabal"))
            .expect_err("an executable's build requires its package's library modules");
        assert!(
            error_chirho.contains("LibChirho.InternalChirho"),
            "{error_chirho}"
        );
    }

    #[test]
    fn a_missing_main_is_is_refused_by_name_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        // `main-is` is searched in hs-source-dirs only, so a root Main.hs does not satisfy it.
        write_chirho(
            scratch_chirho.path(),
            &[
                (
                    "main-chirho.cabal",
                    "cabal-version: 2.4\nname: main-chirho\nversion: 0.1\nbuild-type: Simple\n\nexecutable main-chirho\n  main-is: Main.hs\n  hs-source-dirs: app\n  build-depends: base\n  default-language: Haskell2010\n",
                ),
                ("Main.hs", PLAIN_MAIN_CHIRHO),
            ],
        );
        let error_chirho = build_chirho(&scratch_chirho.path().join("main-chirho.cabal"))
            .expect_err("an executable without its main-is source must not build");
        for token_chirho in [
            REFUSAL_CHIRHO,
            "executable main-chirho lists main-is Main.hs",
            "[app]",
        ] {
            assert!(
                error_chirho.contains(token_chirho),
                "{token_chirho}: {error_chirho}"
            );
        }
    }

    #[test]
    fn a_required_store_dependency_missing_a_module_is_refused_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        write_chirho(
            &scratch_chirho
                .path()
                .join(SOURCE_STORE_DIR_NAME_CHIRHO)
                .join("dep-gone-chirho-1.0"),
            &[
                (
                    "dep-gone-chirho.cabal",
                    "cabal-version: 2.4\nname: dep-gone-chirho\nversion: 1.0\nbuild-type: Simple\n\nlibrary\n  exposed-modules: DepGoneChirho\n  other-modules: DepGoneChirho.MissingChirho\n  hs-source-dirs: src\n  build-depends: base\n  default-language: Haskell2010\n",
                ),
                (
                    "src/DepGoneChirho.hs",
                    "module DepGoneChirho (goneValueChirho) where\n\ngoneValueChirho :: Int\ngoneValueChirho = 2\n",
                ),
            ],
        );
        let app_dir_chirho = scratch_chirho.path().join("app-chirho");
        write_chirho(
            &app_dir_chirho,
            &[
                (
                    "app-chirho.cabal",
                    "cabal-version: 2.4\nname: app-chirho\nversion: 0.1\nbuild-type: Simple\n\nexecutable app-chirho-exe\n  main-is: Main.hs\n  hs-source-dirs: app\n  build-depends: base, dep-gone-chirho\n  default-language: Haskell2010\n",
                ),
                (
                    "app/Main.hs",
                    "module Main (main) where\n\nimport DepGoneChirho (goneValueChirho)\n\nmain :: IO ()\nmain = print goneValueChirho\n",
                ),
            ],
        );
        let error_chirho = build_chirho(&app_dir_chirho.join("app-chirho.cabal"))
            .expect_err("a linked dependency must hold every declared library module");
        for token_chirho in [
            REFUSAL_CHIRHO,
            "package dep-gone-chirho",
            "DepGoneChirho.MissingChirho",
        ] {
            assert!(
                error_chirho.contains(token_chirho),
                "{token_chirho}: {error_chirho}"
            );
        }
    }

    #[test]
    fn found_only_discovery_still_skips_a_missing_declared_module_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        write_chirho(
            scratch_chirho.path(),
            &[
                ("probe-chirho.cabal", DATA_LIST_PROBE_CABAL_CHIRHO),
                ("Main.hs", USES_HOME_DATA_LIST_CHIRHO),
                ("Data.List.hs", HOME_DATA_LIST_CHIRHO),
            ],
        );
        let modules_chirho = discover_modules_chirho(
            &parse_cabal_chirho(DATA_LIST_PROBE_CABAL_CHIRHO),
            scratch_chirho.path(),
        );
        assert_eq!(
            modules_chirho,
            vec![("Main".to_string(), scratch_chirho.path().join("./Main.hs"))]
        );
    }

    #[test]
    fn the_packages_own_generated_modules_are_not_source_obligations_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        // As haskell-say does: both components list the package's own Paths_ module, unshipped.
        let generated_cabal_chirho = "cabal-version: 2.4\nname: say-chirho\nversion: 0.1\nbuild-type: Simple\n\nlibrary\n  exposed-modules: SayChirho\n  other-modules: Paths_say_chirho, PackageInfo_say_chirho\n  hs-source-dirs: src\n  build-depends: base\n  default-language: Haskell2010\n\nexecutable say-chirho-exe\n  main-is: Main.hs\n  other-modules: Paths_say_chirho\n  hs-source-dirs: app\n  build-depends: base, say-chirho\n  default-language: Haskell2010\n";
        write_chirho(
            scratch_chirho.path(),
            &[
                ("say-chirho.cabal", generated_cabal_chirho),
                (
                    "src/SayChirho.hs",
                    "module SayChirho (sayChirho) where\n\nsayChirho :: Int\nsayChirho = 3\n",
                ),
                ("app/Main.hs", PLAIN_MAIN_CHIRHO),
            ],
        );
        build_chirho(&scratch_chirho.path().join("say-chirho.cabal"))
            .expect("the package's own Cabal-generated modules are not missing sources");

        // Another package's Paths_ module is an ordinary declared module.
        write_chirho(
            scratch_chirho.path(),
            &[(
                "say-chirho.cabal",
                &generated_cabal_chirho.replace(
                    "other-modules: Paths_say_chirho\n  hs-source-dirs: app",
                    "other-modules: Paths_other_chirho\n  hs-source-dirs: app",
                ),
            )],
        );
        let error_chirho = build_chirho(&scratch_chirho.path().join("say-chirho.cabal"))
            .expect_err("another package's Paths_ module needs its source");
        assert!(
            error_chirho.contains(REFUSAL_CHIRHO) && error_chirho.contains("Paths_other_chirho"),
            "{error_chirho}"
        );
    }

    #[test]
    fn test_suite_modules_stay_best_effort_chirho() {
        let scratch_chirho = tempfile::tempdir().unwrap();
        write_chirho(
            scratch_chirho.path(),
            &[
                (
                    "suite-chirho.cabal",
                    "cabal-version: 2.4\nname: suite-chirho\nversion: 0.1\nbuild-type: Simple\n\nexecutable suite-chirho-exe\n  main-is: Main.hs\n  hs-source-dirs: app\n  build-depends: base\n  default-language: Haskell2010\n\ntest-suite suite-chirho-test\n  type: exitcode-stdio-1.0\n  main-is: TestMainChirho.hs\n  other-modules: TestGoneChirho\n  hs-source-dirs: test\n  build-depends: base\n  default-language: Haskell2010\n",
                ),
                ("app/Main.hs", PLAIN_MAIN_CHIRHO),
            ],
        );
        let built_chirho = build_chirho(&scratch_chirho.path().join("suite-chirho.cabal"))
            .expect("a default build does not select the test suite's sources");
        assert_eq!(built_chirho.executables_chirho.len(), 1);
    }
}
