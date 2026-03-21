// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Artifact store
//!
//! On-disk key→blob storage for cached compilation artifacts.
//!
//! The store layout is a flat directory of files named by the artifact's
//! fingerprint: `<cache-dir>/<hex>.blob`. This avoids nested paths and
//! makes cache eviction straightforward (delete old blobs).

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::fingerprint_chirho::FingerprintChirho;

/// Errors from the artifact store.
#[derive(Debug)]
pub enum ArtifactErrorChirho {
    IoChirho(io::Error),
    NotFoundChirho(FingerprintChirho),
}

impl From<io::Error> for ArtifactErrorChirho {
    fn from(e_chirho: io::Error) -> Self {
        Self::IoChirho(e_chirho)
    }
}

impl std::fmt::Display for ArtifactErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoChirho(e_chirho) => write!(f_chirho, "artifact I/O error: {}", e_chirho),
            Self::NotFoundChirho(fp_chirho) => {
                write!(f_chirho, "artifact not found: {}", fp_chirho)
            }
        }
    }
}

/// On-disk artifact store.
///
/// Can also operate in memory-only mode (for tests) when constructed
/// with [`ArtifactStoreChirho::in_memory_chirho`].
#[derive(Debug, Clone)]
pub struct ArtifactStoreChirho {
    /// Root directory for on-disk storage.
    dir_chirho: Option<PathBuf>,
    /// In-memory fallback (for tests or when disk is unavailable).
    mem_chirho: HashMap<FingerprintChirho, Vec<u8>>,
}

impl ArtifactStoreChirho {
    /// Create a disk-backed store at the given directory.
    /// The directory is created if it does not exist.
    pub fn open_chirho(dir_chirho: impl AsRef<Path>) -> Result<Self, ArtifactErrorChirho> {
        let dir_chirho = dir_chirho.as_ref().to_path_buf();
        fs::create_dir_all(&dir_chirho)?;
        Ok(Self {
            dir_chirho: Some(dir_chirho),
            mem_chirho: HashMap::new(),
        })
    }

    /// Create an in-memory-only store (no disk I/O).
    pub fn in_memory_chirho() -> Self {
        Self {
            dir_chirho: None,
            mem_chirho: HashMap::new(),
        }
    }

    /// Store a blob under the given fingerprint.
    pub fn put_chirho(
        &mut self,
        fp_chirho: FingerprintChirho,
        data_chirho: &[u8],
    ) -> Result<(), ArtifactErrorChirho> {
        if let Some(dir_chirho) = &self.dir_chirho {
            let path_chirho = dir_chirho.join(format!("{}.blob", fp_chirho.to_hex_chirho()));
            fs::write(&path_chirho, data_chirho)?;
        }
        self.mem_chirho.insert(fp_chirho, data_chirho.to_vec());
        Ok(())
    }

    /// Retrieve a blob by fingerprint.
    pub fn get_chirho(
        &mut self,
        fp_chirho: FingerprintChirho,
    ) -> Result<Vec<u8>, ArtifactErrorChirho> {
        // Check memory cache first.
        if let Some(data_chirho) = self.mem_chirho.get(&fp_chirho) {
            return Ok(data_chirho.clone());
        }
        // Try disk.
        if let Some(dir_chirho) = &self.dir_chirho {
            let path_chirho = dir_chirho.join(format!("{}.blob", fp_chirho.to_hex_chirho()));
            match fs::read(&path_chirho) {
                Ok(data_chirho) => {
                    self.mem_chirho.insert(fp_chirho, data_chirho.clone());
                    return Ok(data_chirho);
                }
                Err(e_chirho) if e_chirho.kind() == io::ErrorKind::NotFound => {}
                Err(e_chirho) => return Err(ArtifactErrorChirho::IoChirho(e_chirho)),
            }
        }
        Err(ArtifactErrorChirho::NotFoundChirho(fp_chirho))
    }

    /// Check whether an artifact exists (without reading it).
    pub fn contains_chirho(&self, fp_chirho: FingerprintChirho) -> bool {
        if self.mem_chirho.contains_key(&fp_chirho) {
            return true;
        }
        if let Some(dir_chirho) = &self.dir_chirho {
            let path_chirho = dir_chirho.join(format!("{}.blob", fp_chirho.to_hex_chirho()));
            return path_chirho.exists();
        }
        false
    }

    /// Remove an artifact.
    pub fn remove_chirho(
        &mut self,
        fp_chirho: FingerprintChirho,
    ) -> Result<(), ArtifactErrorChirho> {
        self.mem_chirho.remove(&fp_chirho);
        if let Some(dir_chirho) = &self.dir_chirho {
            let path_chirho = dir_chirho.join(format!("{}.blob", fp_chirho.to_hex_chirho()));
            if path_chirho.exists() {
                fs::remove_file(&path_chirho)?;
            }
        }
        Ok(())
    }

    /// Number of artifacts in the in-memory cache.
    pub fn cached_count_chirho(&self) -> usize {
        self.mem_chirho.len()
    }

    /// Clear the in-memory cache (disk artifacts remain).
    pub fn clear_mem_chirho(&mut self) {
        self.mem_chirho.clear();
    }

    /// Evict all artifacts from both memory and disk.
    pub fn evict_all_chirho(&mut self) -> Result<(), ArtifactErrorChirho> {
        self.mem_chirho.clear();
        if let Some(dir_chirho) = &self.dir_chirho {
            for entry_chirho in fs::read_dir(dir_chirho)? {
                let entry_chirho = entry_chirho?;
                let path_chirho = entry_chirho.path();
                if path_chirho.extension().and_then(|e| e.to_str()) == Some("blob") {
                    fs::remove_file(&path_chirho)?;
                }
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn fp_chirho(s_chirho: &str) -> FingerprintChirho {
        FingerprintChirho::from_str_chirho(s_chirho)
    }

    #[test]
    fn in_memory_put_get_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        let key_chirho = fp_chirho("test-key");
        store_chirho
            .put_chirho(key_chirho, b"hello artifact")
            .unwrap();
        let data_chirho = store_chirho.get_chirho(key_chirho).unwrap();
        assert_eq!(data_chirho, b"hello artifact");
    }

    #[test]
    fn not_found_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        let key_chirho = fp_chirho("missing");
        let result_chirho = store_chirho.get_chirho(key_chirho);
        assert!(matches!(
            result_chirho,
            Err(ArtifactErrorChirho::NotFoundChirho(_))
        ));
    }

    #[test]
    fn contains_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        let key_chirho = fp_chirho("exists");
        assert!(!store_chirho.contains_chirho(key_chirho));
        store_chirho.put_chirho(key_chirho, b"data").unwrap();
        assert!(store_chirho.contains_chirho(key_chirho));
    }

    #[test]
    fn remove_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        let key_chirho = fp_chirho("removeme");
        store_chirho.put_chirho(key_chirho, b"data").unwrap();
        assert!(store_chirho.contains_chirho(key_chirho));
        store_chirho.remove_chirho(key_chirho).unwrap();
        assert!(!store_chirho.contains_chirho(key_chirho));
    }

    #[test]
    fn cached_count_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        assert_eq!(store_chirho.cached_count_chirho(), 0);
        store_chirho.put_chirho(fp_chirho("a"), b"aaa").unwrap();
        store_chirho.put_chirho(fp_chirho("b"), b"bbb").unwrap();
        assert_eq!(store_chirho.cached_count_chirho(), 2);
    }

    #[test]
    fn clear_mem_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        store_chirho.put_chirho(fp_chirho("x"), b"xxx").unwrap();
        assert_eq!(store_chirho.cached_count_chirho(), 1);
        store_chirho.clear_mem_chirho();
        assert_eq!(store_chirho.cached_count_chirho(), 0);
    }

    #[test]
    fn disk_backed_round_trip_chirho() {
        let tmp_chirho = std::env::temp_dir().join("haskelujah_artifact_test_chirho");
        let _ = fs::remove_dir_all(&tmp_chirho); // clean up prior runs
        let mut store_chirho = ArtifactStoreChirho::open_chirho(&tmp_chirho).unwrap();

        let key_chirho = fp_chirho("disk-test");
        store_chirho.put_chirho(key_chirho, b"on disk").unwrap();

        // Clear memory, force disk read.
        store_chirho.clear_mem_chirho();
        let data_chirho = store_chirho.get_chirho(key_chirho).unwrap();
        assert_eq!(data_chirho, b"on disk");

        // Cleanup.
        store_chirho.evict_all_chirho().unwrap();
        let _ = fs::remove_dir_all(&tmp_chirho);
    }

    #[test]
    fn evict_all_chirho() {
        let mut store_chirho = ArtifactStoreChirho::in_memory_chirho();
        store_chirho.put_chirho(fp_chirho("a"), b"a").unwrap();
        store_chirho.put_chirho(fp_chirho("b"), b"b").unwrap();
        store_chirho.evict_all_chirho().unwrap();
        assert_eq!(store_chirho.cached_count_chirho(), 0);
    }
}
