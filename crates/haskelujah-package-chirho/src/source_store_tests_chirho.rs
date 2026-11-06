// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The source package database format, the store's lock and atomic save, the generated `Paths_`
//! module and bounded archive extraction (row 703).

use crate::generated_modules_chirho::{paths_module_name_chirho, render_paths_module_chirho};
use crate::hackage_chirho::extract_tarball_bounded_chirho;
use crate::source_db_chirho::{
    CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO, ResolvedDependencyChirho, SourceModuleChirho,
    SourceModuleOriginChirho, SourcePackageDbChirho, SourcePackageRecordChirho,
};
use crate::source_store_chirho::{
    SOURCE_DB_TEMP_FILE_NAME_CHIRHO, SOURCE_STORE_DIR_NAME_CHIRHO, SourceStoreChirho,
};
use crate::version_chirho::parse_version_chirho;
use std::fs;
use std::path::PathBuf;

fn record_chirho(name_chirho: &str, version_text_chirho: &str) -> SourcePackageRecordChirho {
    let version_chirho = parse_version_chirho(version_text_chirho).unwrap();
    SourcePackageRecordChirho {
        name_chirho: name_chirho.to_string(),
        directory_name_chirho: format!("{name_chirho}-{version_chirho}"),
        version_chirho,
        cabal_file_chirho: PathBuf::from(format!("{name_chirho}.cabal")),
        capability_chirho: CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO.to_string(),
        modules_chirho: vec![
            SourceModuleChirho {
                module_name_chirho: "Data.ThingChirho".to_string(),
                exposed_chirho: true,
                origin_chirho: SourceModuleOriginChirho::ShippedChirho,
                relative_path_chirho: PathBuf::from("src dir/Data/ThingChirho.hs"),
            },
            SourceModuleChirho {
                module_name_chirho: "Paths_thing_chirho".to_string(),
                exposed_chirho: false,
                origin_chirho: SourceModuleOriginChirho::GeneratedChirho,
                relative_path_chirho: PathBuf::from("src dir/Paths_thing_chirho.hs"),
            },
        ],
        dependencies_chirho: vec![
            ResolvedDependencyChirho::BuiltinChirho("base".to_string()),
            ResolvedDependencyChirho::StoreChirho(
                "dep-chirho".to_string(),
                parse_version_chirho("0.2.1").unwrap(),
            ),
        ],
    }
}

fn db_with_chirho(records_chirho: &[SourcePackageRecordChirho]) -> SourcePackageDbChirho {
    let mut db_chirho = SourcePackageDbChirho::new_chirho();
    for record_chirho in records_chirho {
        db_chirho.insert_new_chirho(record_chirho.clone()).unwrap();
    }
    db_chirho
}

#[test]
fn database_text_round_trips_exactly_chirho() {
    let db_chirho = db_with_chirho(&[
        record_chirho("thing-chirho", "1.2.0"),
        record_chirho("other-chirho", "0.2.1"),
    ]);
    let text_chirho = db_chirho.render_chirho();
    assert!(
        text_chirho
            .starts_with("haskelujah-source-package-db-chirho 1\npackage other-chirho 0.2.1\n")
    );
    assert!(
        text_chirho
            .contains("module other generated Paths_thing_chirho src dir/Paths_thing_chirho.hs\n")
    );
    assert!(text_chirho.contains("depends store dep-chirho 0.2.1\n"));
    let parsed_chirho = SourcePackageDbChirho::parse_chirho(&text_chirho).unwrap();
    assert_eq!(parsed_chirho, db_chirho);
    assert_eq!(parsed_chirho.render_chirho(), text_chirho);
}

#[test]
fn damaged_database_text_is_refused_with_its_line_chirho() {
    let good_chirho = db_with_chirho(&[record_chirho("thing-chirho", "1.2.0")]).render_chirho();
    let cases_chirho: Vec<(String, usize)> = vec![
        (String::new(), 1),
        ("haskelujah-source-package-db-chirho 2\n".to_string(), 1),
        (
            format!("{good_chirho}stray line\n"),
            good_chirho.lines().count() + 1,
        ),
        (good_chirho.replace("end\n", ""), 2),
        (
            good_chirho.replace("directory thing-chirho-1.2.0", "directory thing-chirho-9"),
            2,
        ),
        (
            good_chirho.replace("capability core-checked-source", "capability installed"),
            2,
        ),
        (
            good_chirho.replace("src dir/Data/ThingChirho.hs", "../escape.hs"),
            2,
        ),
        (
            good_chirho.replace("module exposed shipped", "module exposed fabricated"),
            6,
        ),
        (
            good_chirho.replace("depends builtin base", "depends builtin"),
            8,
        ),
        (
            format!(
                "{good_chirho}{}",
                good_chirho.lines().skip(1).collect::<Vec<_>>().join("\n") + "\n"
            ),
            good_chirho.lines().count() + 1,
        ),
    ];
    for (text_chirho, line_chirho) in cases_chirho {
        let error_chirho = SourcePackageDbChirho::parse_chirho(&text_chirho)
            .expect_err(&format!("must refuse:\n{text_chirho}"));
        assert_eq!(
            error_chirho.line_chirho, line_chirho,
            "{error_chirho} in\n{text_chirho}"
        );
    }
}

#[test]
fn generated_paths_module_follows_cabals_contract_chirho() {
    let version_chirho = parse_version_chirho("1.0.0.0").unwrap();
    assert_eq!(paths_module_name_chirho("haskell-say"), "Paths_haskell_say");
    let text_chirho = render_paths_module_chirho("haskell-say", &version_chirho).unwrap();
    assert_eq!(
        text_chirho,
        render_paths_module_chirho("haskell-say", &version_chirho).unwrap()
    );
    assert!(text_chirho.contains("module Paths_haskell_say (\n"));
    assert!(text_chirho.contains("version = Version [1,0,0,0] []\n"));
    for export_chirho in [
        "version,",
        "getBinDir,",
        "getLibDir,",
        "getDynLibDir,",
        "getDataDir,",
        "getLibexecDir,",
        "getDataFileName,",
        "getSysconfDir",
    ] {
        assert!(text_chirho.contains(export_chirho), "{export_chirho}");
    }
    for directory_chirho in [
        "bindir",
        "libdir",
        "dynlibdir",
        "datadir",
        "libexecdir",
        "sysconfdir",
    ] {
        assert!(
            text_chirho.contains(&format!("overrideDirChirho \"{directory_chirho}\"")),
            "{directory_chirho}"
        );
    }
    assert!(text_chirho.contains("lookupEnv (\"haskell_say_\" ++ varChirho)"));
    for token_chirho in ["@MODULE@", "@PKGID@", "@PKGVAR@", "@DIGITS@"] {
        assert!(
            !text_chirho.contains(token_chirho),
            "{token_chirho} was not replaced"
        );
    }
    assert!(render_paths_module_chirho("bad name", &version_chirho).is_err());
}

#[test]
fn store_needs_its_discoverable_name_and_saves_atomically_chirho() {
    let dir_chirho = tempfile::tempdir().unwrap();
    assert!(SourceStoreChirho::open_chirho(&dir_chirho.path().join("store-chirho")).is_err());
    let store_chirho =
        SourceStoreChirho::open_chirho(&dir_chirho.path().join(SOURCE_STORE_DIR_NAME_CHIRHO))
            .unwrap();
    assert_eq!(
        store_chirho.load_db_chirho().unwrap(),
        SourcePackageDbChirho::new_chirho()
    );

    let lock_chirho = store_chirho.lock_chirho().unwrap();
    let second_chirho = store_chirho
        .lock_chirho()
        .expect_err("the lock is exclusive");
    assert!(
        second_chirho.contains("another install holds the lock"),
        "{second_chirho}"
    );
    let first_db_chirho = db_with_chirho(&[record_chirho("thing-chirho", "1.2.0")]);
    store_chirho
        .save_db_chirho(&first_db_chirho, &lock_chirho)
        .unwrap();
    assert_eq!(store_chirho.load_db_chirho().unwrap(), first_db_chirho);
    let saved_bytes_chirho = fs::read(store_chirho.db_path_chirho()).unwrap();

    let blocker_chirho = store_chirho
        .meta_dir_chirho()
        .join(SOURCE_DB_TEMP_FILE_NAME_CHIRHO);
    fs::create_dir(&blocker_chirho).unwrap();
    fs::write(blocker_chirho.join("keep-chirho"), "x").unwrap();
    let second_db_chirho = db_with_chirho(&[
        record_chirho("thing-chirho", "1.2.0"),
        record_chirho("other-chirho", "0.2.1"),
    ]);
    assert!(
        store_chirho
            .save_db_chirho(&second_db_chirho, &lock_chirho)
            .is_err()
    );
    assert_eq!(
        fs::read(store_chirho.db_path_chirho()).unwrap(),
        saved_bytes_chirho
    );

    fs::remove_dir_all(&blocker_chirho).unwrap();
    fs::write(&blocker_chirho, "stale temporary file").unwrap();
    store_chirho
        .save_db_chirho(&second_db_chirho, &lock_chirho)
        .unwrap();
    assert_eq!(store_chirho.load_db_chirho().unwrap(), second_db_chirho);
    assert!(!blocker_chirho.exists());

    fs::create_dir_all(
        store_chirho
            .staging_root_chirho()
            .join("old-chirho/nested-chirho"),
    )
    .unwrap();
    assert_eq!(store_chirho.sweep_staging_chirho(&lock_chirho).unwrap(), 1);
    assert_eq!(
        fs::read_dir(store_chirho.staging_root_chirho())
            .unwrap()
            .count(),
        0
    );
    drop(lock_chirho);
    store_chirho.lock_chirho().unwrap();
}

fn archive_with_entry_chirho(path_chirho: &str, entry_type_chirho: tar::EntryType) -> Vec<u8> {
    let mut encoder_chirho =
        flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    {
        let mut builder_chirho = tar::Builder::new(&mut encoder_chirho);
        let mut header_chirho = tar::Header::new_gnu();
        header_chirho
            .set_path("pkg-chirho-0.1/pkg-chirho.cabal")
            .unwrap();
        header_chirho.set_size(4);
        header_chirho.set_mode(0o644);
        header_chirho.set_cksum();
        builder_chirho.append(&header_chirho, &b"name"[..]).unwrap();
        let mut link_header_chirho = tar::Header::new_gnu();
        link_header_chirho.set_path(path_chirho).unwrap();
        link_header_chirho.set_entry_type(entry_type_chirho);
        link_header_chirho.set_link_name("/etc/passwd").unwrap();
        link_header_chirho.set_size(0);
        link_header_chirho.set_mode(0o777);
        link_header_chirho.set_cksum();
        builder_chirho
            .append(&link_header_chirho, std::io::empty())
            .unwrap();
        builder_chirho.finish().unwrap();
    }
    encoder_chirho.finish().unwrap()
}

#[test]
fn bounded_extraction_refuses_links_chirho() {
    for entry_type_chirho in [tar::EntryType::Symlink, tar::EntryType::Link] {
        let dir_chirho = tempfile::tempdir().unwrap();
        let archive_chirho =
            archive_with_entry_chirho("pkg-chirho-0.1/escape-chirho", entry_type_chirho);
        let error_chirho = extract_tarball_bounded_chirho(&archive_chirho, dir_chirho.path())
            .expect_err("links are refused");
        assert!(
            error_chirho
                .to_string()
                .contains("only files and directories"),
            "{error_chirho}"
        );
        assert!(
            !dir_chirho
                .path()
                .join("pkg-chirho-0.1/escape-chirho")
                .exists()
        );
    }
    let dir_chirho = tempfile::tempdir().unwrap();
    let archive_chirho = crate::hackage_chirho::create_test_tarball_files_chirho(&[
        ("pkg-chirho-0.1/pkg-chirho.cabal", "name: pkg-chirho\n"),
        ("pkg-chirho-0.1/src/A.hs", "module A where\n"),
    ]);
    extract_tarball_bounded_chirho(&archive_chirho, dir_chirho.path()).unwrap();
    assert_eq!(
        fs::read_to_string(dir_chirho.path().join("pkg-chirho-0.1/src/A.hs")).unwrap(),
        "module A where\n"
    );
}
