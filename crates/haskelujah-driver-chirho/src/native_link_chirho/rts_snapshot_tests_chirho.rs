// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The RTS snapshot's contract (row 568).
//! - It outlives a replacement of the shared archive.
//! - It waits out a briefly missing archive.
//! - It takes nothing when the freshness check fails.
//! - It leaves no directory behind.
//!
//! workflow: testing-chirho/native-fail-loud-chirho

use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use crate::rts_snapshot_chirho::{
    RTS_ARCHIVE_CHIRHO, SNAPSHOTS_DIR_CHIRHO, remove_stale_snapshots_chirho, rts_snapshot_chirho,
};

/// A fresh profile directory under the system temporary directory, holding an archive
/// that reads `contents_chirho`.
fn profile_dir_chirho(name_chirho: &str, contents_chirho: &str) -> PathBuf {
    let dir_chirho = std::env::temp_dir().join(format!(
        "haskelujah-rts-snapshot-{name_chirho}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir_chirho);
    fs::create_dir_all(&dir_chirho).unwrap();
    fs::write(dir_chirho.join(RTS_ARCHIVE_CHIRHO), contents_chirho).unwrap();
    dir_chirho
}

/// Replace the archive as cargo's uplift does: remove the path, then write a new file there.
fn replace_archive_chirho(profile_dir_chirho: &Path, contents_chirho: &str) {
    let archive_chirho = profile_dir_chirho.join(RTS_ARCHIVE_CHIRHO);
    fs::remove_file(&archive_chirho).unwrap();
    fs::write(&archive_chirho, contents_chirho).unwrap();
}

fn snapshot_contents_chirho(
    snapshot_chirho: &crate::rts_snapshot_chirho::RtsSnapshotChirho,
) -> String {
    fs::read_to_string(snapshot_chirho.link_dir_chirho().join(RTS_ARCHIVE_CHIRHO)).unwrap()
}

#[test]
fn a_snapshot_outlives_an_archive_replacement_and_its_owner_chirho() {
    let profile_chirho = profile_dir_chirho("outlives", "first");
    let snapshot_chirho = rts_snapshot_chirho(&profile_chirho, || Ok(())).unwrap();
    replace_archive_chirho(&profile_chirho, "second");
    assert_eq!(snapshot_contents_chirho(&snapshot_chirho), "first");
    let later_chirho = rts_snapshot_chirho(&profile_chirho, || Ok(())).unwrap();
    assert_eq!(snapshot_contents_chirho(&later_chirho), "second");
    let dir_chirho = snapshot_chirho.link_dir_chirho().to_path_buf();
    drop(snapshot_chirho);
    assert!(
        !dir_chirho.exists(),
        "a dropped snapshot left its directory"
    );
    assert!(later_chirho.link_dir_chirho().exists());
    drop(later_chirho);
    fs::remove_dir_all(&profile_chirho).unwrap();
}

#[test]
fn a_failed_freshness_check_takes_no_snapshot_chirho() {
    let profile_chirho = profile_dir_chirho("failed", "first");
    let error_chirho =
        rts_snapshot_chirho(&profile_chirho, || Err("build failed".to_string())).unwrap_err();
    assert_eq!(error_chirho, "build failed");
    let snapshots_chirho = fs::read_dir(profile_chirho.join(SNAPSHOTS_DIR_CHIRHO))
        .unwrap()
        .count();
    assert_eq!(snapshots_chirho, 0);
    fs::remove_dir_all(&profile_chirho).unwrap();
}

#[test]
fn a_briefly_missing_archive_is_waited_for_chirho() {
    let profile_chirho = profile_dir_chirho("missing", "first");
    let archive_chirho = profile_chirho.join(RTS_ARCHIVE_CHIRHO);
    fs::remove_file(&archive_chirho).unwrap();
    let writer_chirho = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        fs::write(&archive_chirho, "rebuilt").unwrap();
    });
    let snapshot_chirho = rts_snapshot_chirho(&profile_chirho, || Ok(())).unwrap();
    writer_chirho.join().unwrap();
    assert_eq!(snapshot_contents_chirho(&snapshot_chirho), "rebuilt");
    drop(snapshot_chirho);
    fs::remove_dir_all(&profile_chirho).unwrap();
}

#[test]
fn snapshot_directories_left_by_a_killed_process_are_retired_chirho() {
    let profile_chirho = profile_dir_chirho("stale", "first");
    let snapshots_chirho = profile_chirho.join(SNAPSHOTS_DIR_CHIRHO);
    let left_chirho = snapshots_chirho.join("left-by-a-killed-process");
    fs::create_dir_all(&left_chirho).unwrap();
    thread::sleep(Duration::from_millis(20));
    remove_stale_snapshots_chirho(&snapshots_chirho, Duration::from_secs(600));
    assert!(left_chirho.exists(), "a recent snapshot was retired");
    remove_stale_snapshots_chirho(&snapshots_chirho, Duration::from_millis(10));
    assert!(!left_chirho.exists(), "a stale snapshot survived");
    fs::remove_dir_all(&profile_chirho).unwrap();
}
