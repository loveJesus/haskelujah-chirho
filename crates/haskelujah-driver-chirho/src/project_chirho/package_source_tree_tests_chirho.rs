// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Staging bounds count every entry, directories included, as it is listed (row 703).

use super::{SourceTreeLimitsChirho, copy_source_tree_within_chirho};
use std::fs;

const SMALL_LIMITS_CHIRHO: SourceTreeLimitsChirho = SourceTreeLimitsChirho {
    entries_chirho: 3,
    bytes_chirho: 1024,
};

#[test]
fn directories_count_toward_the_entry_bound_chirho() {
    let scratch_chirho = tempfile::tempdir().unwrap();
    let source_chirho = scratch_chirho.path().join("source-chirho");
    for name_chirho in ["a-chirho", "b-chirho", "c-chirho", "d-chirho"] {
        fs::create_dir_all(source_chirho.join(name_chirho)).unwrap();
    }
    let error_chirho = copy_source_tree_within_chirho(
        &source_chirho,
        &scratch_chirho.path().join("dest-chirho"),
        SMALL_LIMITS_CHIRHO,
    )
    .expect_err("four empty directories exceed three entries");
    assert!(error_chirho.contains("exceeds 3 entries"), "{error_chirho}");
}

#[test]
fn a_wide_directory_is_refused_while_it_is_listed_chirho() {
    let scratch_chirho = tempfile::tempdir().unwrap();
    let source_chirho = scratch_chirho.path().join("source-chirho");
    fs::create_dir_all(source_chirho.join("wide-chirho")).unwrap();
    for index_chirho in 0..5 {
        fs::write(
            source_chirho.join(format!("wide-chirho/f{index_chirho}.hs")),
            "x",
        )
        .unwrap();
    }
    let dest_chirho = scratch_chirho.path().join("dest-chirho");
    assert!(
        copy_source_tree_within_chirho(&source_chirho, &dest_chirho, SMALL_LIMITS_CHIRHO).is_err()
    );
    let copied_chirho = fs::read_dir(dest_chirho.join("wide-chirho"))
        .map_or(0, |entries_chirho| entries_chirho.count());
    assert_eq!(
        copied_chirho, 0,
        "no file of the over-wide directory was copied"
    );
}

#[test]
fn trees_within_the_bounds_copy_exactly_and_bytes_are_bounded_chirho() {
    let scratch_chirho = tempfile::tempdir().unwrap();
    let source_chirho = scratch_chirho.path().join("source-chirho");
    fs::create_dir_all(source_chirho.join("src-chirho")).unwrap();
    fs::write(source_chirho.join("src-chirho/A.hs"), "module A where\n").unwrap();
    fs::write(source_chirho.join("pkg-chirho.cabal"), "name: pkg-chirho\n").unwrap();
    let dest_chirho = scratch_chirho.path().join("dest-chirho");
    let excluded_chirho =
        copy_source_tree_within_chirho(&source_chirho, &dest_chirho, SMALL_LIMITS_CHIRHO).unwrap();
    assert!(excluded_chirho.is_empty());
    assert_eq!(
        fs::read_to_string(dest_chirho.join("src-chirho/A.hs")).unwrap(),
        "module A where\n"
    );

    let large_chirho = scratch_chirho.path().join("large-chirho");
    fs::create_dir_all(&large_chirho).unwrap();
    fs::write(large_chirho.join("big.hs"), vec![b'x'; 2048]).unwrap();
    let error_chirho = copy_source_tree_within_chirho(
        &large_chirho,
        &scratch_chirho.path().join("large-dest-chirho"),
        SMALL_LIMITS_CHIRHO,
    )
    .expect_err("2048 bytes exceed the 1024-byte bound");
    assert!(error_chirho.contains("1024 bytes"), "{error_chirho}");
}
