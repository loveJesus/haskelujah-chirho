// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A source package store: its layout, install lock and atomic database publication (row 703).
//!
//! ```text
//! <store>/                     named .haskelujah-packages-chirho, the only name builds discover
//!   <name>-<version>/          a published source package, which builds compile
//!   .source-db-chirho/
//!     source-packages-chirho.txt          the source package database
//!     source-packages-chirho.txt.tmp-chirho   written and synced, then renamed over it
//!     install-lock-chirho      held under an exclusive advisory lock for a whole install
//!   .staging-chirho/<install>/ an install in progress; the next install sweeps leftovers
//! ```
//!
//! Builds select package directories by name (`<name>` or `<name>-<digit>…`), so neither
//! bookkeeping directory is ever read as a package. The lock is advisory and released by the
//! operating system when its holder exits, so a crashed install never leaves the store locked.

use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::source_db_chirho::SourcePackageDbChirho;

/// The directory name builds look for in a project's ancestors.
pub const SOURCE_STORE_DIR_NAME_CHIRHO: &str = ".haskelujah-packages-chirho";
/// The store's bookkeeping directory.
pub const STORE_META_DIR_NAME_CHIRHO: &str = ".source-db-chirho";
/// The source package database inside the bookkeeping directory.
pub const SOURCE_DB_FILE_NAME_CHIRHO: &str = "source-packages-chirho.txt";
/// The file a new database is written to before it is renamed into place.
pub const SOURCE_DB_TEMP_FILE_NAME_CHIRHO: &str = "source-packages-chirho.txt.tmp-chirho";
/// The file whose advisory lock serializes installs into one store.
pub const STORE_LOCK_FILE_NAME_CHIRHO: &str = "install-lock-chirho";
/// The directory that holds installs in progress.
pub const STAGING_DIR_NAME_CHIRHO: &str = ".staging-chirho";

/// What install consumes: a local source. Fetching a release is a separate, earlier step.
#[derive(Debug, Clone)]
pub enum SourceInstallInputChirho {
    /// A package source directory; its top-level `dist-chirho` build output is left out.
    DirectoryChirho(PathBuf),
    /// The bytes of a `.tar.gz` release archive holding one `<name>-<version>/` directory.
    TarballChirho(Vec<u8>),
}

/// What a failed install left in the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreStateAfterFailureChirho {
    /// No package directory was published and the source database is byte-identical; at most
    /// the store's own bookkeeping directories were created.
    UnchangedChirho,
    /// The package directory was published, its record could not be saved, and moving the
    /// directory back failed as well: the store holds this directory without a record.
    UnrecordedDirectoryChirho(PathBuf),
}

/// Why an install failed, and what it left behind.
#[derive(Debug, Clone)]
pub struct SourceInstallErrorChirho {
    pub message_chirho: String,
    pub store_state_chirho: StoreStateAfterFailureChirho,
}

impl SourceInstallErrorChirho {
    /// A failure that left the store unchanged.
    pub fn unchanged_chirho(message_chirho: impl Into<String>) -> Self {
        Self {
            message_chirho: message_chirho.into(),
            store_state_chirho: StoreStateAfterFailureChirho::UnchangedChirho,
        }
    }
}

impl std::fmt::Display for SourceInstallErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho.write_str(&self.message_chirho)
    }
}

impl std::error::Error for SourceInstallErrorChirho {}

/// An open source package store.
#[derive(Debug, Clone)]
pub struct SourceStoreChirho {
    root_chirho: PathBuf,
    /// The canonical root, which names this store whatever spelling opened it.
    identity_chirho: PathBuf,
}

/// Proof that this process holds one store's install lock; dropping it releases the lock. A
/// store refuses a lock taken on another store before it changes anything.
#[derive(Debug)]
pub struct StoreLockChirho {
    _file_chirho: File,
    store_identity_chirho: PathBuf,
}

fn io_message_chirho(action_chirho: &str, path_chirho: &Path, error_chirho: io::Error) -> String {
    format!(
        "cannot {action_chirho} {}: {error_chirho}",
        path_chirho.display()
    )
}

/// Create `path_chirho` if needed and require a real directory, never a link that could send
/// sweeps or writes somewhere else.
fn ensure_real_dir_chirho(path_chirho: &Path) -> Result<(), String> {
    fs::create_dir_all(path_chirho)
        .map_err(|error_chirho| io_message_chirho("create", path_chirho, error_chirho))?;
    let metadata_chirho = fs::symlink_metadata(path_chirho)
        .map_err(|error_chirho| io_message_chirho("inspect", path_chirho, error_chirho))?;
    if metadata_chirho.file_type().is_symlink() || !metadata_chirho.is_dir() {
        return Err(format!(
            "{} must be a directory, not a link or a file",
            path_chirho.display()
        ));
    }
    Ok(())
}

impl SourceStoreChirho {
    /// Open the store at `root_chirho`, creating it when absent. Its directory must be named
    /// `.haskelujah-packages-chirho`: builds find no other name, so packages installed under a
    /// different name could never be used.
    pub fn open_chirho(root_chirho: &Path) -> Result<Self, String> {
        if root_chirho.file_name() != Some(OsStr::new(SOURCE_STORE_DIR_NAME_CHIRHO)) {
            return Err(format!(
                "a package store must be a directory named {SOURCE_STORE_DIR_NAME_CHIRHO}, the only name builds find; got {}",
                root_chirho.display()
            ));
        }
        fs::create_dir_all(root_chirho)
            .map_err(|error_chirho| io_message_chirho("create", root_chirho, error_chirho))?;
        if !root_chirho.is_dir() {
            return Err(format!("{} is not a directory", root_chirho.display()));
        }
        let nested_chirho = root_chirho.join(SOURCE_STORE_DIR_NAME_CHIRHO);
        if fs::symlink_metadata(&nested_chirho).is_ok() {
            return Err(format!(
                "{} holds a nested {SOURCE_STORE_DIR_NAME_CHIRHO}, which would capture dependency lookup for its packages",
                root_chirho.display()
            ));
        }
        let identity_chirho = fs::canonicalize(root_chirho)
            .map_err(|error_chirho| io_message_chirho("resolve", root_chirho, error_chirho))?;
        Ok(Self {
            root_chirho: root_chirho.to_path_buf(),
            identity_chirho,
        })
    }

    /// Refuse a lock that another store handed out.
    fn ensure_own_lock_chirho(&self, lock_chirho: &StoreLockChirho) -> Result<(), String> {
        if lock_chirho.store_identity_chirho == self.identity_chirho {
            Ok(())
        } else {
            Err(format!(
                "the install lock given belongs to {}, not to {}",
                lock_chirho.store_identity_chirho.display(),
                self.identity_chirho.display()
            ))
        }
    }

    pub fn root_chirho(&self) -> &Path {
        &self.root_chirho
    }

    pub fn meta_dir_chirho(&self) -> PathBuf {
        self.root_chirho.join(STORE_META_DIR_NAME_CHIRHO)
    }

    pub fn db_path_chirho(&self) -> PathBuf {
        self.meta_dir_chirho().join(SOURCE_DB_FILE_NAME_CHIRHO)
    }

    pub fn staging_root_chirho(&self) -> PathBuf {
        self.root_chirho.join(STAGING_DIR_NAME_CHIRHO)
    }

    /// Where the package directory `directory_name_chirho` is published.
    pub fn package_dir_chirho(&self, directory_name_chirho: &str) -> PathBuf {
        self.root_chirho.join(directory_name_chirho)
    }

    /// Take the store's install lock without waiting; another holder is an error.
    pub fn lock_chirho(&self) -> Result<StoreLockChirho, String> {
        let meta_dir_chirho = self.meta_dir_chirho();
        ensure_real_dir_chirho(&meta_dir_chirho)?;
        let lock_path_chirho = meta_dir_chirho.join(STORE_LOCK_FILE_NAME_CHIRHO);
        let file_chirho = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path_chirho)
            .map_err(|error_chirho| io_message_chirho("open", &lock_path_chirho, error_chirho))?;
        match file_chirho.try_lock() {
            Ok(()) => Ok(StoreLockChirho {
                _file_chirho: file_chirho,
                store_identity_chirho: self.identity_chirho.clone(),
            }),
            Err(TryLockError::WouldBlock) => Err(format!(
                "another install holds the lock {}; wait for it to finish",
                lock_path_chirho.display()
            )),
            Err(TryLockError::Error(error_chirho)) => {
                Err(io_message_chirho("lock", &lock_path_chirho, error_chirho))
            }
        }
    }

    /// Read the source package database. A store without one is empty; an unreadable or
    /// damaged one is an error, never an empty database.
    pub fn load_db_chirho(&self) -> Result<SourcePackageDbChirho, String> {
        let db_path_chirho = self.db_path_chirho();
        match fs::read_to_string(&db_path_chirho) {
            Ok(text_chirho) => {
                SourcePackageDbChirho::parse_chirho(&text_chirho).map_err(|error_chirho| {
                    format!(
                        "the source package database {} is damaged at {error_chirho}",
                        db_path_chirho.display()
                    )
                })
            }
            Err(error_chirho) if error_chirho.kind() == io::ErrorKind::NotFound => {
                Ok(SourcePackageDbChirho::new_chirho())
            }
            Err(error_chirho) => Err(io_message_chirho("read", &db_path_chirho, error_chirho)),
        }
    }

    /// Publish `db_chirho` atomically: write and sync a temporary file, then rename it over the
    /// database. Readers see the old bytes or the new bytes, never a mixture. On error the
    /// database file is unchanged.
    pub fn save_db_chirho(
        &self,
        db_chirho: &SourcePackageDbChirho,
        lock_chirho: &StoreLockChirho,
    ) -> Result<(), String> {
        self.ensure_own_lock_chirho(lock_chirho)?;
        let meta_dir_chirho = self.meta_dir_chirho();
        ensure_real_dir_chirho(&meta_dir_chirho)?;
        let temp_path_chirho = meta_dir_chirho.join(SOURCE_DB_TEMP_FILE_NAME_CHIRHO);
        match fs::symlink_metadata(&temp_path_chirho) {
            Ok(_) => fs::remove_file(&temp_path_chirho).map_err(|error_chirho| {
                io_message_chirho("remove the stale", &temp_path_chirho, error_chirho)
            })?,
            Err(error_chirho) if error_chirho.kind() == io::ErrorKind::NotFound => {}
            Err(error_chirho) => {
                return Err(io_message_chirho(
                    "inspect",
                    &temp_path_chirho,
                    error_chirho,
                ));
            }
        }
        let written_chirho = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path_chirho)
            .and_then(|mut file_chirho| {
                file_chirho.write_all(db_chirho.render_chirho().as_bytes())?;
                file_chirho.sync_all()
            });
        if let Err(error_chirho) = written_chirho {
            let _ = fs::remove_file(&temp_path_chirho);
            return Err(io_message_chirho("write", &temp_path_chirho, error_chirho));
        }
        let db_path_chirho = self.db_path_chirho();
        if let Err(error_chirho) = fs::rename(&temp_path_chirho, &db_path_chirho) {
            let _ = fs::remove_file(&temp_path_chirho);
            return Err(io_message_chirho("replace", &db_path_chirho, error_chirho));
        }
        // Make the rename itself durable where the platform allows it; the new database is
        // already the visible one, so a failure here is not an install failure.
        if let Ok(dir_chirho) = File::open(&meta_dir_chirho) {
            let _ = dir_chirho.sync_all();
        }
        Ok(())
    }

    /// Remove every leftover of earlier installs from the staging directory. Only an install
    /// holding the lock may sweep, so no live install loses its staging.
    pub fn sweep_staging_chirho(&self, lock_chirho: &StoreLockChirho) -> Result<usize, String> {
        self.ensure_own_lock_chirho(lock_chirho)?;
        let staging_root_chirho = self.staging_root_chirho();
        ensure_real_dir_chirho(&staging_root_chirho)?;
        let entries_chirho = fs::read_dir(&staging_root_chirho).map_err(|error_chirho| {
            io_message_chirho("list", &staging_root_chirho, error_chirho)
        })?;
        let mut removed_chirho = 0;
        for entry_chirho in entries_chirho {
            let path_chirho = entry_chirho
                .map_err(|error_chirho| {
                    io_message_chirho("list", &staging_root_chirho, error_chirho)
                })?
                .path();
            let metadata_chirho = fs::symlink_metadata(&path_chirho)
                .map_err(|error_chirho| io_message_chirho("inspect", &path_chirho, error_chirho))?;
            let removal_chirho = if metadata_chirho.is_dir() {
                fs::remove_dir_all(&path_chirho)
            } else {
                fs::remove_file(&path_chirho)
            };
            removal_chirho
                .map_err(|error_chirho| io_message_chirho("remove", &path_chirho, error_chirho))?;
            removed_chirho += 1;
        }
        Ok(removed_chirho)
    }

    /// Create an empty staging directory for this install, on the store's own filesystem so
    /// that publishing is a single rename.
    pub fn new_staging_dir_chirho(&self, lock_chirho: &StoreLockChirho) -> Result<PathBuf, String> {
        self.ensure_own_lock_chirho(lock_chirho)?;
        let staging_root_chirho = self.staging_root_chirho();
        ensure_real_dir_chirho(&staging_root_chirho)?;
        let nanos_chirho = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |elapsed_chirho| elapsed_chirho.as_nanos());
        let path_chirho =
            staging_root_chirho.join(format!("install-{}-{nanos_chirho}", std::process::id()));
        fs::create_dir(&path_chirho)
            .map_err(|error_chirho| io_message_chirho("create", &path_chirho, error_chirho))?;
        Ok(path_chirho)
    }
}
