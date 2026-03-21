// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Tests for the package install pipeline (§50 cabal-install compatibility).

#[cfg(test)]
mod tests_chirho {
    use crate::{InstallResultChirho, discover_modules_chirho, find_cabal_in_dir_chirho};
    use haskelujah_package_chirho::{
        InstalledPkgDbChirho, hackage_chirho::create_test_tarball_chirho,
        hackage_chirho::extract_tarball_chirho, parse_cabal_chirho, parse_version_chirho,
    };
    use std::fs;

    #[test]
    fn find_cabal_in_dir_chirho_test() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(
            tmp_chirho.path().join("my-pkg.cabal"),
            "name: my-pkg\nversion: 1.0\n",
        )
        .unwrap();

        let found_chirho = find_cabal_in_dir_chirho(tmp_chirho.path());
        assert!(found_chirho.is_some());
        assert!(
            found_chirho
                .unwrap()
                .to_string_lossy()
                .contains("my-pkg.cabal")
        );
    }

    #[test]
    fn find_cabal_in_dir_none_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        fs::write(tmp_chirho.path().join("README.md"), "# Hello").unwrap();

        let found_chirho = find_cabal_in_dir_chirho(tmp_chirho.path());
        assert!(found_chirho.is_none());
    }

    #[test]
    fn discover_modules_from_cabal_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        let src_dir_chirho = tmp_chirho.path().join("src");
        fs::create_dir_all(&src_dir_chirho).unwrap();
        fs::write(src_dir_chirho.join("Lib.hs"), "module Lib where\nfoo = 1\n").unwrap();
        let data_dir_chirho = src_dir_chirho.join("Data");
        fs::create_dir_all(&data_dir_chirho).unwrap();
        fs::write(
            data_dir_chirho.join("Utils.hs"),
            "module Data.Utils where\nbar = 2\n",
        )
        .unwrap();

        let cabal_text_chirho = "\
name: mypkg
version: 0.1
library
  exposed-modules: Lib, Data.Utils
  hs-source-dirs: src
";
        let pkg_chirho = parse_cabal_chirho(cabal_text_chirho);
        let modules_chirho = discover_modules_chirho(&pkg_chirho, tmp_chirho.path());

        assert_eq!(modules_chirho.len(), 2);
        let names_chirho: Vec<&str> = modules_chirho
            .iter()
            .map(|(n_chirho, _)| n_chirho.as_str())
            .collect();
        assert!(names_chirho.contains(&"Lib"));
        assert!(names_chirho.contains(&"Data.Utils"));
    }

    #[test]
    fn extract_and_find_cabal_chirho() {
        // Create a test tarball with a .cabal file inside
        let cabal_content_chirho = "\
name: test-install-pkg
version: 1.0.0
library
  exposed-modules: TestLib
";
        let tarball_chirho = create_test_tarball_chirho(
            "test-install-pkg-1.0.0/test-install-pkg.cabal",
            cabal_content_chirho,
        );

        let tmp_chirho = tempfile::tempdir().unwrap();
        extract_tarball_chirho(&tarball_chirho, tmp_chirho.path()).unwrap();

        let pkg_dir_chirho = tmp_chirho.path().join("test-install-pkg-1.0.0");
        let found_chirho = find_cabal_in_dir_chirho(&pkg_dir_chirho);
        assert!(found_chirho.is_some());

        let content_chirho = fs::read_to_string(found_chirho.unwrap()).unwrap();
        let pkg_chirho = parse_cabal_chirho(&content_chirho);
        assert_eq!(pkg_chirho.name_chirho, "test-install-pkg");
    }

    #[test]
    fn install_result_type_fields_chirho() {
        // Verify the InstallResultChirho struct has expected fields by constructing one
        let pkg_chirho = parse_cabal_chirho("name: foo\nversion: 1.0\n");
        let build_plan_chirho = haskelujah_package_chirho::BuildPlanChirho {
            steps_chirho: vec![],
        };
        let installed_chirho = haskelujah_package_chirho::InstalledPkgChirho {
            name_chirho: "foo".to_string(),
            version_chirho: parse_version_chirho("1.0").unwrap(),
            exposed_modules_chirho: vec![],
            depends_chirho: vec![],
            install_dir_chirho: std::path::PathBuf::from("/tmp/foo"),
        };

        let result_chirho = InstallResultChirho {
            package_chirho: pkg_chirho,
            build_plan_chirho,
            modules_compiled_chirho: 0,
            installed_pkg_chirho: installed_chirho,
        };

        assert_eq!(result_chirho.installed_pkg_chirho.name_chirho, "foo");
        assert_eq!(result_chirho.modules_compiled_chirho, 0);
    }

    #[test]
    fn pkgdb_register_after_install_chirho() {
        // Simulate the registration step of install
        let v_chirho = parse_version_chirho("2.0.0").unwrap();
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();

        let installed_chirho = haskelujah_package_chirho::InstalledPkgChirho {
            name_chirho: "containers".to_string(),
            version_chirho: v_chirho.clone(),
            exposed_modules_chirho: vec![
                haskelujah_package_chirho::InstalledModuleChirho {
                    module_name_chirho: "Data.Map".to_string(),
                    iface_path_chirho: std::path::PathBuf::from("Data/Map.rhi"),
                    object_path_chirho: None,
                },
                haskelujah_package_chirho::InstalledModuleChirho {
                    module_name_chirho: "Data.Set".to_string(),
                    iface_path_chirho: std::path::PathBuf::from("Data/Set.rhi"),
                    object_path_chirho: None,
                },
            ],
            depends_chirho: vec![("base".to_string(), v_chirho.clone())],
            install_dir_chirho: std::path::PathBuf::from("/tmp/containers-2.0.0"),
        };

        db_chirho.register_chirho(installed_chirho);

        // Verify lookup works
        let found_chirho = db_chirho.lookup_chirho("containers");
        assert!(found_chirho.is_some());
        assert_eq!(found_chirho.unwrap().exposed_modules_chirho.len(), 2);

        // Verify find_module works
        let mod_found_chirho = db_chirho.find_module_chirho("Data.Map");
        assert!(mod_found_chirho.is_some());
        assert_eq!(mod_found_chirho.unwrap().0.name_chirho, "containers");

        // Verify serialization roundtrip
        let serialized_chirho = db_chirho.to_string_chirho();
        let db2_chirho = InstalledPkgDbChirho::from_string_chirho(&serialized_chirho);
        assert_eq!(db2_chirho.total_versions_chirho(), 1);
    }

    #[test]
    fn pkgdb_persistence_roundtrip_chirho() {
        // Simulate the full persistence cycle: register → serialize → deserialize → verify
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();

        for (name_chirho, ver_chirho) in &[("text", "1.2.5"), ("bytestring", "0.11.4")] {
            let v_chirho = parse_version_chirho(ver_chirho).unwrap();
            db_chirho.register_chirho(haskelujah_package_chirho::InstalledPkgChirho {
                name_chirho: name_chirho.to_string(),
                version_chirho: v_chirho,
                exposed_modules_chirho: vec![],
                depends_chirho: vec![],
                install_dir_chirho: std::path::PathBuf::from(format!("/tmp/{}", name_chirho)),
            });
        }

        // Write to temp file
        let tmp_chirho = tempfile::tempdir().unwrap();
        let db_path_chirho = tmp_chirho.path().join("pkgdb-chirho.txt");
        fs::write(&db_path_chirho, db_chirho.to_string_chirho()).unwrap();

        // Read back
        let content_chirho = fs::read_to_string(&db_path_chirho).unwrap();
        let db2_chirho = InstalledPkgDbChirho::from_string_chirho(&content_chirho);
        assert_eq!(db2_chirho.total_versions_chirho(), 2);
        assert!(db2_chirho.lookup_chirho("text").is_some());
        assert!(db2_chirho.lookup_chirho("bytestring").is_some());
    }
}
