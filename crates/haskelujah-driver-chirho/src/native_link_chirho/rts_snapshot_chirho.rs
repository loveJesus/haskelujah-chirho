// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A private snapshot of the RTS archive for each native link (row 568). Every native
//! compile first runs `cargo build -p haskelujah-rts` as a freshness check, and even a
//! no-op build replaces `target/debug/libhaskelujah_rts.a`: a new inode, the same mtime.
//! Compiles running in parallel from one tree therefore raced. One compile's check could
//! remove the archive while another's linker opened it, and the link failed with
//! `ld: file cannot be open()ed, errno=2`. Each link now reads a hard link to the archive,
//! taken under an exclusive lock that every snapshot's freshness check also holds. A later
//! build replaces the shared path but never that inode. The link itself runs unlocked,
//! and the snapshot's directory is removed when its owner drops it.
//! workflow: testing-chirho/native-fail-loud-chirho

use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// The archive `cargo build -p haskelujah-rts` leaves in a target profile directory.
pub const RTS_ARCHIVE_CHIRHO: &str = "libhaskelujah_rts.a";
const LOCK_FILE_CHIRHO: &str = ".rts-snapshot-lock-chirho";
pub(crate) const SNAPSHOTS_DIR_CHIRHO: &str = "rts-snapshots-chirho";
/// No link takes this long, so an older snapshot directory was left by a killed process.
const STALE_AFTER_CHIRHO: Duration = Duration::from_secs(600);
/// How long a snapshot waits for an archive that a build outside the lock is replacing.
const ARCHIVE_WAIT_CHIRHO: Duration = Duration::from_secs(5);

static NEXT_SNAPSHOT_CHIRHO: AtomicU64 = AtomicU64::new(0);

/// The RTS archive as it stood when the snapshot was taken, alone in a private directory
/// that a linker's `-L` can name. The directory is removed on drop.
#[derive(Debug)]
pub struct RtsSnapshotChirho {
    dir_chirho: PathBuf,
}

impl RtsSnapshotChirho {
    /// The directory holding this snapshot's `libhaskelujah_rts.a`.
    pub fn link_dir_chirho(&self) -> &Path {
        &self.dir_chirho
    }
}

impl Drop for RtsSnapshotChirho {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir_chirho);
    }
}

/// Run `freshness_chirho`, the RTS build, and hard-link the archive in `profile_dir_chirho`
/// into a private snapshot. Both happen under one exclusive lock that every snapshot
/// takes, so no other snapshot's build can replace the archive in between.
/// workflow: testing-chirho/native-fail-loud-chirho
pub fn rts_snapshot_chirho(
    profile_dir_chirho: &Path,
    freshness_chirho: impl FnOnce() -> Result<(), String>,
) -> Result<RtsSnapshotChirho, String> {
    let snapshots_chirho = profile_dir_chirho.join(SNAPSHOTS_DIR_CHIRHO);
    fs::create_dir_all(&snapshots_chirho).map_err(|error_chirho| {
        format!(
            "could not create {}: {error_chirho}",
            snapshots_chirho.display()
        )
    })?;
    let lock_path_chirho = profile_dir_chirho.join(LOCK_FILE_CHIRHO);
    let lock_chirho = File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&lock_path_chirho)
        .map_err(|error_chirho| {
            format!(
                "could not open {}: {error_chirho}",
                lock_path_chirho.display()
            )
        })?;
    lock_chirho.lock().map_err(|error_chirho| {
        format!(
            "could not lock {}: {error_chirho}",
            lock_path_chirho.display()
        )
    })?;
    freshness_chirho()?;
    remove_stale_snapshots_chirho(&snapshots_chirho, STALE_AFTER_CHIRHO);
    let snapshot_chirho = RtsSnapshotChirho {
        dir_chirho: snapshots_chirho.join(format!(
            "{}-{}",
            std::process::id(),
            NEXT_SNAPSHOT_CHIRHO.fetch_add(1, Ordering::Relaxed)
        )),
    };
    fs::create_dir_all(&snapshot_chirho.dir_chirho).map_err(|error_chirho| {
        format!(
            "could not create {}: {error_chirho}",
            snapshot_chirho.dir_chirho.display()
        )
    })?;
    link_archive_chirho(
        &profile_dir_chirho.join(RTS_ARCHIVE_CHIRHO),
        &snapshot_chirho.dir_chirho.join(RTS_ARCHIVE_CHIRHO),
    )?;
    Ok(snapshot_chirho)
}

/// Hard-link the archive, waiting out its replacement by a build that took no lock, such
/// as a plain `cargo build` in the same tree.
fn link_archive_chirho(archive_chirho: &Path, snapshot_chirho: &Path) -> Result<(), String> {
    let deadline_chirho = Instant::now() + ARCHIVE_WAIT_CHIRHO;
    loop {
        match fs::hard_link(archive_chirho, snapshot_chirho) {
            Ok(()) => return Ok(()),
            Err(error_chirho)
                if error_chirho.kind() == ErrorKind::NotFound
                    && Instant::now() < deadline_chirho =>
            {
                thread::sleep(Duration::from_millis(5));
            }
            Err(error_chirho) => {
                return Err(format!(
                    "could not snapshot {}: {error_chirho}",
                    archive_chirho.display()
                ));
            }
        }
    }
}

/// Retire snapshot directories older than `max_age_chirho`, which a killed process left.
pub(crate) fn remove_stale_snapshots_chirho(snapshots_chirho: &Path, max_age_chirho: Duration) {
    let Ok(entries_chirho) = fs::read_dir(snapshots_chirho) else {
        return;
    };
    for entry_chirho in entries_chirho.flatten() {
        let stale_chirho = entry_chirho
            .metadata()
            .and_then(|metadata_chirho| metadata_chirho.modified())
            .ok()
            .and_then(|modified_chirho| modified_chirho.elapsed().ok())
            .is_some_and(|age_chirho| age_chirho > max_age_chirho);
        if stale_chirho {
            let _ = fs::remove_dir_all(entry_chirho.path());
        }
    }
}
