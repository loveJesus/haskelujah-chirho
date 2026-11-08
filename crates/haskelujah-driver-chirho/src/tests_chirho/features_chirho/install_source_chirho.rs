// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source package install against real scratch stores (row 703).
//!
//! Every failure case snapshots the store's bytes first and proves they are unchanged after the
//! refused install, using actual filesystem conditions (a damaged database, a directory where the
//! database's temporary file goes, a held lock, a link) rather than mocks.

#[cfg(test)]
mod tests_chirho {
    use crate::dependency_packages_chirho::builtin_dependency_names_chirho;
    use crate::install_package_chirho;
    use haskelujah_package_chirho::hackage_chirho::create_test_tarball_files_chirho;
    use haskelujah_package_chirho::source_store_chirho::{
        SOURCE_DB_TEMP_FILE_NAME_CHIRHO, STAGING_DIR_NAME_CHIRHO, STORE_LOCK_FILE_NAME_CHIRHO,
        STORE_META_DIR_NAME_CHIRHO,
    };
    use haskelujah_package_chirho::{
        BUILTIN_PACKAGE_NAMES_CHIRHO, CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO,
        ResolvedDependencyChirho, SOURCE_STORE_DIR_NAME_CHIRHO, SourceInstallErrorChirho,
        SourceInstallInputChirho, SourceModuleOriginChirho, SourceStoreChirho,
        StoreStateAfterFailureChirho, parse_version_chirho,
    };
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    const HELLO_MODULE_CHIRHO: &str = "module GreetChirho.HelloChirho (greetingChirho) where\n\ngreetingChirho :: String -> String\ngreetingChirho nameChirho = \"Hello, \" ++ nameChirho ++ \"!\"\n";

    /// A scratch area holding package sources and a store builds would discover.
    struct ScratchChirho {
        _dir_chirho: tempfile::TempDir,
        root_chirho: PathBuf,
    }

    impl ScratchChirho {
        fn new_chirho() -> Self {
            let dir_chirho = tempfile::tempdir().unwrap();
            let root_chirho = dir_chirho.path().to_path_buf();
            Self {
                _dir_chirho: dir_chirho,
                root_chirho,
            }
        }

        fn store_chirho(&self) -> PathBuf {
            self.root_chirho.join(SOURCE_STORE_DIR_NAME_CHIRHO)
        }

        /// Write a package source tree under `sources-chirho/<dir_name_chirho>/`.
        fn package_chirho(&self, dir_name_chirho: &str, files_chirho: &[(&str, &str)]) -> PathBuf {
            let package_dir_chirho = self
                .root_chirho
                .join("sources-chirho")
                .join(dir_name_chirho);
            for (relative_chirho, text_chirho) in files_chirho {
                let path_chirho = package_dir_chirho.join(relative_chirho);
                fs::create_dir_all(path_chirho.parent().unwrap()).unwrap();
                fs::write(path_chirho, text_chirho).unwrap();
            }
            package_dir_chirho
        }

        fn install_dir_chirho(
            &self,
            package_dir_chirho: &Path,
        ) -> Result<crate::InstallResultChirho, SourceInstallErrorChirho> {
            install_package_chirho(
                &SourceInstallInputChirho::DirectoryChirho(package_dir_chirho.to_path_buf()),
                &self.store_chirho(),
            )
        }
    }

    fn cabal_chirho(name_chirho: &str, version_chirho: &str, library_chirho: &str) -> String {
        format!(
            "cabal-version: 2.4\nname: {name_chirho}\nversion: {version_chirho}\nbuild-type: Simple\n\nlibrary\n{library_chirho}  hs-source-dirs: src\n  default-language: Haskell2010\n"
        )
    }

    fn greet_package_chirho(scratch_chirho: &ScratchChirho) -> PathBuf {
        scratch_chirho.package_chirho(
            "greet-chirho",
            &[
                (
                    "greet-chirho.cabal",
                    &cabal_chirho(
                        "greet-chirho",
                        "1.0.0",
                        "  exposed-modules: GreetChirho.HelloChirho\n  build-depends: base\n",
                    ),
                ),
                ("src/GreetChirho/HelloChirho.hs", HELLO_MODULE_CHIRHO),
                ("README.md", "greet\n"),
                ("dist-chirho/build/stale-exe-chirho", "stale build output\n"),
            ],
        )
    }

    /// Every path in the store with its bytes (`None` for a directory), leaving out only what
    /// the unchanged-store contract allows an install to create: the two bookkeeping
    /// directories, the empty lock file and staging contents. The database and anything else
    /// inside the bookkeeping directory are compared.
    fn store_snapshot_chirho(store_chirho: &Path) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
        let lock_path_chirho =
            Path::new(STORE_META_DIR_NAME_CHIRHO).join(STORE_LOCK_FILE_NAME_CHIRHO);
        let mut snapshot_chirho = BTreeMap::new();
        let mut pending_chirho = vec![store_chirho.to_path_buf()];
        while let Some(dir_chirho) = pending_chirho.pop() {
            for entry_chirho in fs::read_dir(&dir_chirho).unwrap() {
                let path_chirho = entry_chirho.unwrap().path();
                let relative_chirho = path_chirho
                    .strip_prefix(store_chirho)
                    .unwrap()
                    .to_path_buf();
                if relative_chirho.starts_with(STAGING_DIR_NAME_CHIRHO)
                    || relative_chirho == lock_path_chirho
                {
                    continue;
                }
                if relative_chirho == Path::new(STORE_META_DIR_NAME_CHIRHO) {
                    pending_chirho.push(path_chirho);
                    continue;
                }
                if path_chirho.is_dir() {
                    snapshot_chirho.insert(relative_chirho, None);
                    pending_chirho.push(path_chirho);
                } else {
                    snapshot_chirho.insert(relative_chirho, Some(fs::read(&path_chirho).unwrap()));
                }
            }
        }
        snapshot_chirho
    }

    fn staging_entry_count_chirho(store_chirho: &Path) -> usize {
        fs::read_dir(store_chirho.join(STAGING_DIR_NAME_CHIRHO))
            .map(|entries_chirho| entries_chirho.count())
            .unwrap_or(0)
    }

    /// Refused, store bytes unchanged, staging empty; returns the message.
    fn assert_refused_unchanged_chirho(
        outcome_chirho: Result<crate::InstallResultChirho, SourceInstallErrorChirho>,
        store_chirho: &Path,
        before_chirho: &BTreeMap<PathBuf, Option<Vec<u8>>>,
    ) -> String {
        let error_chirho = outcome_chirho.expect_err("install should be refused");
        assert_eq!(
            error_chirho.store_state_chirho,
            StoreStateAfterFailureChirho::UnchangedChirho,
            "{}",
            error_chirho.message_chirho
        );
        assert_eq!(&store_snapshot_chirho(store_chirho), before_chirho);
        assert_eq!(staging_entry_count_chirho(store_chirho), 0);
        error_chirho.message_chirho
    }

    #[test]
    fn directory_install_publishes_checked_source_without_build_output_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let source_chirho = greet_package_chirho(&scratch_chirho);
        let result_chirho = scratch_chirho.install_dir_chirho(&source_chirho).unwrap();

        let published_chirho = scratch_chirho.store_chirho().join("greet-chirho-1.0.0");
        assert_eq!(result_chirho.package_dir_chirho, published_chirho);
        for relative_chirho in [
            "src/GreetChirho/HelloChirho.hs",
            "README.md",
            "greet-chirho.cabal",
        ] {
            assert_eq!(
                fs::read(published_chirho.join(relative_chirho)).unwrap(),
                fs::read(source_chirho.join(relative_chirho)).unwrap(),
                "{relative_chirho} must be published unchanged"
            );
        }
        assert!(!published_chirho.join("dist-chirho").exists());
        assert_eq!(
            result_chirho.excluded_paths_chirho,
            vec![PathBuf::from("dist-chirho")]
        );
        assert_eq!(result_chirho.checked_module_count_chirho, 1);

        let stored_chirho = SourceStoreChirho::open_chirho(&scratch_chirho.store_chirho())
            .unwrap()
            .load_db_chirho()
            .unwrap();
        let record_chirho = stored_chirho.get_chirho("greet-chirho").unwrap();
        assert_eq!(record_chirho, &result_chirho.record_chirho);
        assert_eq!(
            record_chirho.capability_chirho,
            CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO
        );
        assert_eq!(record_chirho.modules_chirho.len(), 1);
        assert_eq!(
            record_chirho.modules_chirho[0].relative_path_chirho,
            PathBuf::from("src/GreetChirho/HelloChirho.hs")
        );
        assert_eq!(
            record_chirho.dependencies_chirho,
            vec![ResolvedDependencyChirho::BuiltinChirho("base".to_string())]
        );
        assert_eq!(
            staging_entry_count_chirho(&scratch_chirho.store_chirho()),
            0
        );
        let snapshot_chirho = store_snapshot_chirho(&scratch_chirho.store_chirho());
        assert!(
            snapshot_chirho
                .keys()
                .all(
                    |path_chirho| path_chirho.extension().is_none_or(|extension_chirho| {
                        extension_chirho != "rhi" && extension_chirho != "o"
                    })
                ),
            "a source install writes no interface or object file"
        );
    }

    #[test]
    fn listed_paths_module_is_generated_and_its_importer_compiles_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let source_chirho = scratch_chirho.package_chirho(
            "paths-chirho",
            &[
                (
                    "paths-chirho.cabal",
                    &cabal_chirho(
                        "paths-chirho",
                        "1.0.0",
                        "  exposed-modules: PathsUserChirho\n  other-modules: Paths_paths_chirho\n  autogen-modules: Paths_paths_chirho\n  build-depends: base\n",
                    ),
                ),
                (
                    "src/PathsUserChirho.hs",
                    "module PathsUserChirho (packageVersionChirho) where\n\nimport Data.Version (Version)\nimport Paths_paths_chirho (version)\n\npackageVersionChirho :: Version\npackageVersionChirho = version\n",
                ),
            ],
        );
        let result_chirho = scratch_chirho.install_dir_chirho(&source_chirho).unwrap();
        assert_eq!(result_chirho.checked_module_count_chirho, 2);
        let generated_chirho = &result_chirho.record_chirho.modules_chirho[1];
        assert_eq!(generated_chirho.module_name_chirho, "Paths_paths_chirho");
        assert_eq!(
            generated_chirho.origin_chirho,
            SourceModuleOriginChirho::GeneratedChirho
        );
        assert!(!generated_chirho.exposed_chirho);
        let text_chirho = fs::read_to_string(
            result_chirho
                .package_dir_chirho
                .join(&generated_chirho.relative_path_chirho),
        )
        .unwrap();
        assert!(text_chirho.contains("module Paths_paths_chirho ("));
        assert!(text_chirho.contains("version = Version [1,0,0] []"));
        assert!(text_chirho.contains("lookupEnv (\"paths_chirho_\" ++ varChirho)"));
        for directory_chirho in [
            "bindir",
            "libdir",
            "dynlibdir",
            "datadir",
            "libexecdir",
            "sysconfdir",
        ] {
            assert!(
                text_chirho.contains(&format!("\"{directory_chirho}\"")),
                "{directory_chirho}"
            );
        }
        assert!(!source_chirho.join("src/Paths_paths_chirho.hs").exists());
    }

    #[test]
    fn missing_module_failed_compile_and_package_info_leave_the_store_unchanged_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        scratch_chirho
            .install_dir_chirho(&greet_package_chirho(&scratch_chirho))
            .unwrap();
        let store_chirho = scratch_chirho.store_chirho();
        let before_chirho = store_snapshot_chirho(&store_chirho);

        let missing_chirho = scratch_chirho.package_chirho(
            "missing-chirho",
            &[(
                "missing-chirho.cabal",
                &cabal_chirho(
                    "missing-chirho",
                    "0.1",
                    "  exposed-modules: AbsentChirho.ModuleChirho\n",
                ),
            )],
        );
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&missing_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("AbsentChirho.ModuleChirho"),
            "{message_chirho}"
        );

        let broken_chirho = scratch_chirho.package_chirho(
            "broken-chirho",
            &[
                (
                    "broken-chirho.cabal",
                    &cabal_chirho("broken-chirho", "0.1", "  exposed-modules: BrokenChirho\n  build-depends: base\n"),
                ),
                ("src/BrokenChirho.hs", "module BrokenChirho where\n\nwrongChirho :: Int\nwrongChirho = \"not an Int\"\n"),
            ],
        );
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&broken_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("does not compile"),
            "{message_chirho}"
        );

        let info_chirho = scratch_chirho.package_chirho(
            "info-chirho",
            &[(
                "info-chirho.cabal",
                &cabal_chirho(
                    "info-chirho",
                    "0.1",
                    "  exposed-modules: PackageInfo_info_chirho\n",
                ),
            )],
        );
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&info_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(message_chirho.contains("PackageInfo_"), "{message_chirho}");
    }

    #[test]
    fn damaged_database_is_an_error_and_keeps_its_bytes_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        scratch_chirho
            .install_dir_chirho(&greet_package_chirho(&scratch_chirho))
            .unwrap();
        let store_chirho = scratch_chirho.store_chirho();
        let db_path_chirho = SourceStoreChirho::open_chirho(&store_chirho)
            .unwrap()
            .db_path_chirho();
        fs::write(
            &db_path_chirho,
            "haskelujah-source-package-db-chirho 1\nfrobnicate\n",
        )
        .unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let other_chirho = scratch_chirho.package_chirho(
            "other-chirho",
            &[
                (
                    "other-chirho.cabal",
                    &cabal_chirho(
                        "other-chirho",
                        "0.1",
                        "  exposed-modules: GreetChirho.HelloChirho\n",
                    ),
                ),
                ("src/GreetChirho/HelloChirho.hs", HELLO_MODULE_CHIRHO),
            ],
        );
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&other_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("damaged at line 2"),
            "{message_chirho}"
        );
    }

    #[test]
    fn failed_database_save_moves_the_published_directory_back_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        scratch_chirho
            .install_dir_chirho(&greet_package_chirho(&scratch_chirho))
            .unwrap();
        let store_chirho = scratch_chirho.store_chirho();
        // A non-empty directory where the temporary database file goes: removing or creating a
        // file there fails for every user, root included.
        let blocker_chirho = store_chirho
            .join(STORE_META_DIR_NAME_CHIRHO)
            .join(SOURCE_DB_TEMP_FILE_NAME_CHIRHO);
        fs::create_dir_all(&blocker_chirho).unwrap();
        fs::write(blocker_chirho.join("keep-chirho"), "x").unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let second_chirho = scratch_chirho.package_chirho(
            "second-chirho",
            &[
                (
                    "second-chirho.cabal",
                    &cabal_chirho(
                        "second-chirho",
                        "0.2",
                        "  exposed-modules: GreetChirho.HelloChirho\n",
                    ),
                ),
                ("src/GreetChirho/HelloChirho.hs", HELLO_MODULE_CHIRHO),
            ],
        );
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&second_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("cannot save the source package database"),
            "{message_chirho}"
        );
        assert!(message_chirho.contains("moved back"), "{message_chirho}");
        assert!(!store_chirho.join("second-chirho-0.2").exists());
    }

    #[test]
    fn existing_directory_held_lock_and_links_are_refused_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        fs::create_dir_all(store_chirho.join("greet-chirho-0.9")).unwrap();
        fs::write(
            store_chirho.join("greet-chirho-0.9/marker-chirho"),
            "legacy",
        )
        .unwrap();
        let source_chirho = greet_package_chirho(&scratch_chirho);
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&source_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("already exists without a record"),
            "{message_chirho}"
        );
        fs::remove_dir_all(store_chirho.join("greet-chirho-0.9")).unwrap();

        let store_handle_chirho = SourceStoreChirho::open_chirho(&store_chirho).unwrap();
        let held_chirho = store_handle_chirho.lock_chirho().unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&source_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("another install holds the lock"),
            "{message_chirho}"
        );
        drop(held_chirho);

        #[cfg(unix)]
        {
            let linked_chirho = scratch_chirho.package_chirho(
                "linked-chirho",
                &[(
                    "linked-chirho.cabal",
                    &cabal_chirho(
                        "linked-chirho",
                        "0.1",
                        "  exposed-modules: GreetChirho.HelloChirho\n",
                    ),
                )],
            );
            fs::create_dir_all(linked_chirho.join("src/GreetChirho")).unwrap();
            std::os::unix::fs::symlink(
                source_chirho.join("src/GreetChirho/HelloChirho.hs"),
                linked_chirho.join("src/GreetChirho/HelloChirho.hs"),
            )
            .unwrap();
            let message_chirho = assert_refused_unchanged_chirho(
                scratch_chirho.install_dir_chirho(&linked_chirho),
                &store_chirho,
                &before_chirho,
            );
            assert!(message_chirho.contains("link"), "{message_chirho}");
        }

        scratch_chirho.install_dir_chirho(&source_chirho).unwrap();
    }

    #[test]
    fn dependencies_resolve_only_to_recorded_satisfying_store_packages_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        let user_library_chirho = |constraint_chirho: &str| {
            format!(
                "  exposed-modules: UseChirho\n  build-depends: base, dep-chirho {constraint_chirho}\n"
            )
        };
        let user_files_chirho = |name_chirho: &str, constraint_chirho: &str| {
            scratch_chirho.package_chirho(
                name_chirho,
                &[
                    (&format!("{name_chirho}.cabal"), &cabal_chirho(name_chirho, "0.1", &user_library_chirho(constraint_chirho))),
                    ("src/UseChirho.hs", "module UseChirho (useValueChirho) where\n\nimport DepChirho.ValueChirho (depValueChirho)\n\nuseValueChirho :: Int\nuseValueChirho = depValueChirho + 1\n"),
                ],
            )
        };
        fs::create_dir_all(&store_chirho).unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&user_files_chirho("early-chirho", ">= 2")),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("not a source package recorded in this store"),
            "{message_chirho}"
        );

        let dep_chirho = scratch_chirho.package_chirho(
            "dep-chirho",
            &[
                ("dep-chirho.cabal", &cabal_chirho("dep-chirho", "2.1", "  exposed-modules: DepChirho.ValueChirho\n  build-depends: base\n")),
                ("src/DepChirho/ValueChirho.hs", "module DepChirho.ValueChirho (depValueChirho) where\n\ndepValueChirho :: Int\ndepValueChirho = 42\n"),
            ],
        );
        scratch_chirho.install_dir_chirho(&dep_chirho).unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&user_files_chirho("newer-chirho", ">= 3")),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("does not satisfy"),
            "{message_chirho}"
        );

        let result_chirho = scratch_chirho
            .install_dir_chirho(&user_files_chirho("user-chirho", ">= 2"))
            .unwrap();
        assert_eq!(
            result_chirho.record_chirho.dependencies_chirho,
            vec![
                ResolvedDependencyChirho::BuiltinChirho("base".to_string()),
                ResolvedDependencyChirho::StoreChirho(
                    "dep-chirho".to_string(),
                    parse_version_chirho("2.1").unwrap()
                ),
            ]
        );
    }

    #[test]
    fn release_archive_installs_like_its_directory_and_must_be_named_for_it_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let cabal_text_chirho = cabal_chirho(
            "tar-chirho",
            "0.3",
            "  exposed-modules: GreetChirho.HelloChirho\n",
        );
        let archive_chirho = |root_chirho: &str| {
            create_test_tarball_files_chirho(&[
                (
                    &format!("{root_chirho}/tar-chirho.cabal"),
                    &cabal_text_chirho,
                ),
                (
                    &format!("{root_chirho}/src/GreetChirho/HelloChirho.hs"),
                    HELLO_MODULE_CHIRHO,
                ),
            ])
        };
        let store_chirho = scratch_chirho.store_chirho();
        fs::create_dir_all(&store_chirho).unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            install_package_chirho(
                &SourceInstallInputChirho::TarballChirho(archive_chirho("other-name-0.3")),
                &store_chirho,
            ),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("top-level directory is other-name-0.3"),
            "{message_chirho}"
        );

        let result_chirho = install_package_chirho(
            &SourceInstallInputChirho::TarballChirho(archive_chirho("tar-chirho-0.3")),
            &store_chirho,
        )
        .unwrap();
        assert_eq!(
            fs::read_to_string(
                result_chirho
                    .package_dir_chirho
                    .join("src/GreetChirho/HelloChirho.hs")
            )
            .unwrap(),
            HELLO_MODULE_CHIRHO
        );
        assert_eq!(result_chirho.checked_module_count_chirho, 1);
    }

    #[test]
    fn setup_code_is_refused_unless_it_is_the_default_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        let package_chirho =
            |name_chirho: &str, build_type_chirho: &str, setup_chirho: Option<&str>| {
                let cabal_text_chirho = cabal_chirho(
                    name_chirho,
                    "0.1",
                    "  exposed-modules: GreetChirho.HelloChirho\n",
                )
                .replace(
                    "build-type: Simple",
                    &format!("build-type: {build_type_chirho}"),
                );
                let cabal_name_chirho = format!("{name_chirho}.cabal");
                let mut files_chirho = vec![
                    (cabal_name_chirho.as_str(), cabal_text_chirho.as_str()),
                    ("src/GreetChirho/HelloChirho.hs", HELLO_MODULE_CHIRHO),
                ];
                if let Some(setup_text_chirho) = setup_chirho {
                    files_chirho.push(("Setup.hs", setup_text_chirho));
                }
                scratch_chirho.package_chirho(name_chirho, &files_chirho)
            };
        fs::create_dir_all(&store_chirho).unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&package_chirho(
                "configure-chirho",
                "Configure",
                None,
            )),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("configure or make"),
            "{message_chirho}"
        );
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&package_chirho(
                "custom-chirho",
                "Custom",
                Some(
                    "import Distribution.Simple\nmain = putStrLn \"side effect\" >> defaultMain\n",
                ),
            )),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("custom Setup script"),
            "{message_chirho}"
        );
        scratch_chirho
            .install_dir_chirho(&package_chirho(
                "default-chirho",
                "Custom",
                Some("#!/usr/bin/env runhaskell\n-- the default\nimport Distribution.Simple\nmain = defaultMain\n"),
            ))
            .unwrap();
    }

    #[test]
    fn stale_staging_is_swept_and_the_store_must_carry_its_discoverable_name_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        let stale_chirho = store_chirho
            .join(STAGING_DIR_NAME_CHIRHO)
            .join("install-stale-chirho/package-chirho");
        fs::create_dir_all(&stale_chirho).unwrap();
        fs::write(stale_chirho.join("left-by-a-crash-chirho"), "x").unwrap();
        scratch_chirho
            .install_dir_chirho(&greet_package_chirho(&scratch_chirho))
            .unwrap();
        assert_eq!(staging_entry_count_chirho(&store_chirho), 0);

        let misnamed_chirho = scratch_chirho.root_chirho.join("my-store-chirho");
        let error_chirho = install_package_chirho(
            &SourceInstallInputChirho::DirectoryChirho(greet_package_chirho(&scratch_chirho)),
            &misnamed_chirho,
        )
        .expect_err("a store builds cannot find must be refused");
        assert!(
            error_chirho
                .message_chirho
                .contains(SOURCE_STORE_DIR_NAME_CHIRHO)
        );
        assert!(!misnamed_chirho.exists());
    }

    #[test]
    fn nontrivial_bounds_on_compiler_provided_packages_are_refused_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        fs::create_dir_all(&store_chirho).unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        for (name_chirho, bound_chirho) in [
            ("future-base-chirho", ">=999"),
            ("bounded-base-chirho", ">=4.7 && <5"),
        ] {
            let package_chirho = scratch_chirho.package_chirho(
                name_chirho,
                &[
                    (
                        &format!("{name_chirho}.cabal"),
                        &cabal_chirho(
                            name_chirho,
                            "0.1",
                            &format!("  exposed-modules: GreetChirho.HelloChirho\n  build-depends: base {bound_chirho}\n"),
                        ),
                    ),
                    ("src/GreetChirho/HelloChirho.hs", HELLO_MODULE_CHIRHO),
                ],
            );
            let message_chirho = assert_refused_unchanged_chirho(
                scratch_chirho.install_dir_chirho(&package_chirho),
                &store_chirho,
                &before_chirho,
            );
            assert!(
                message_chirho.contains("declares no base version"),
                "{message_chirho}"
            );
        }
    }

    #[test]
    fn every_library_bound_on_a_shared_dependency_is_kept_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        let dep_chirho = scratch_chirho.package_chirho(
            "dep-chirho",
            &[
                ("dep-chirho.cabal", &cabal_chirho("dep-chirho", "2.1", "  exposed-modules: DepChirho.ValueChirho\n  build-depends: base\n")),
                ("src/DepChirho/ValueChirho.hs", "module DepChirho.ValueChirho (depValueChirho) where\n\ndepValueChirho :: Int\ndepValueChirho = 42\n"),
            ],
        );
        scratch_chirho.install_dir_chirho(&dep_chirho).unwrap();
        // Neither library imports dep-chirho: the bounds alone must decide.
        let two_libraries_chirho = |name_chirho: &str, second_bound_chirho: &str| {
            let cabal_text_chirho = format!(
                "cabal-version: 2.4\nname: {name_chirho}\nversion: 0.1\nbuild-type: Simple\n\nlibrary\n  exposed-modules: TwoMainChirho\n  build-depends: base, dep-chirho >=2\n  hs-source-dirs: src\n  default-language: Haskell2010\n\nlibrary second-chirho\n  exposed-modules: TwoSecondChirho\n  build-depends: base, dep-chirho {second_bound_chirho}\n  hs-source-dirs: src\n  default-language: Haskell2010\n"
            );
            scratch_chirho.package_chirho(
                name_chirho,
                &[
                    (&format!("{name_chirho}.cabal"), &cabal_text_chirho),
                    ("src/TwoMainChirho.hs", "module TwoMainChirho (mainValueChirho) where\n\nmainValueChirho :: Int\nmainValueChirho = 1\n"),
                    ("src/TwoSecondChirho.hs", "module TwoSecondChirho (secondValueChirho) where\n\nsecondValueChirho :: Int\nsecondValueChirho = 2\n"),
                ],
            )
        };
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&two_libraries_chirho("strict-two-chirho", ">=999")),
            &store_chirho,
            &before_chirho,
        );
        assert!(
            message_chirho.contains("does not satisfy"),
            "{message_chirho}"
        );
        assert!(
            message_chirho.contains(">=999"),
            "the second library's bound is kept: {message_chirho}"
        );

        let result_chirho = scratch_chirho
            .install_dir_chirho(&two_libraries_chirho("loose-two-chirho", ">=2.1"))
            .unwrap();
        assert_eq!(result_chirho.checked_module_count_chirho, 2);
        assert!(result_chirho.record_chirho.dependencies_chirho.contains(
            &ResolvedDependencyChirho::StoreChirho(
                "dep-chirho".to_string(),
                parse_version_chirho("2.1").unwrap()
            )
        ));
    }

    #[test]
    fn a_demanded_dependency_that_no_longer_compiles_refuses_the_install_chirho() {
        let scratch_chirho = ScratchChirho::new_chirho();
        let store_chirho = scratch_chirho.store_chirho();
        let library_package_chirho = |name_chirho: &str,
                                      depends_chirho: &str,
                                      module_chirho: &str,
                                      text_chirho: &str| {
            scratch_chirho.package_chirho(
                name_chirho,
                &[
                    (&format!("{name_chirho}.cabal"), &cabal_chirho(name_chirho, "1.0", &format!("  exposed-modules: {module_chirho}\n  build-depends: {depends_chirho}\n"))),
                    (&format!("src/{module_chirho}.hs"), text_chirho),
                ],
            )
        };
        scratch_chirho
            .install_dir_chirho(&library_package_chirho(
                "dep-b-chirho",
                "base",
                "DepBChirho",
                "module DepBChirho (bValueChirho) where\n\nbValueChirho :: Int\nbValueChirho = 1\n",
            ))
            .unwrap();
        scratch_chirho
            .install_dir_chirho(&library_package_chirho("dep-a-chirho", "base, dep-b-chirho", "DepAChirho", "module DepAChirho (aValueChirho) where\n\nimport DepBChirho (bValueChirho)\n\naValueChirho :: Int\naValueChirho = bValueChirho + 1\n"))
            .unwrap();
        let user_chirho = library_package_chirho(
            "user-chirho",
            "base, dep-a-chirho",
            "UserChirho",
            "module UserChirho (userValueChirho) where\n\nimport DepAChirho (aValueChirho)\n\nuserValueChirho :: Int\nuserValueChirho = aValueChirho + 1\n",
        );
        // The transitive dependency stops compiling; the strict collector must not skip it.
        fs::write(
            store_chirho.join("dep-b-chirho-1.0/src/DepBChirho.hs"),
            "module DepBChirho (bValueChirho) where\n\nbValueChirho :: Int\nbValueChirho = \"broken\"\n",
        )
        .unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&user_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(message_chirho.contains("dep-b-chirho"), "{message_chirho}");

        // So does a direct dependency whose module no longer compiles.
        fs::write(
            store_chirho.join("dep-b-chirho-1.0/src/DepBChirho.hs"),
            "module DepBChirho (bValueChirho) where\n\nbValueChirho :: Int\nbValueChirho = 1\n",
        )
        .unwrap();
        fs::write(
            store_chirho.join("dep-a-chirho-1.0/src/DepAChirho.hs"),
            "module DepAChirho (aValueChirho) where\n\naValueChirho :: Int\naValueChirho = True\n",
        )
        .unwrap();
        let before_chirho = store_snapshot_chirho(&store_chirho);
        let message_chirho = assert_refused_unchanged_chirho(
            scratch_chirho.install_dir_chirho(&user_chirho),
            &store_chirho,
            &before_chirho,
        );
        assert!(message_chirho.contains("dep-a-chirho"), "{message_chirho}");
    }

    #[test]
    fn builtin_package_list_matches_the_build_path_chirho() {
        let shared_chirho: std::collections::HashSet<String> = BUILTIN_PACKAGE_NAMES_CHIRHO
            .iter()
            .map(|name_chirho| name_chirho.to_string())
            .collect();
        assert_eq!(shared_chirho, builtin_dependency_names_chirho());
        assert_eq!(shared_chirho.len(), BUILTIN_PACKAGE_NAMES_CHIRHO.len());
    }
}
