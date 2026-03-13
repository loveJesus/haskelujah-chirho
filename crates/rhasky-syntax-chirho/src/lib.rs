// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-syntax-chirho
//!
//! Core syntax types for the Rhasky compiler: source files, module headers,
//! and (soon) token kinds and CST node kinds.

pub mod cst_chirho;
pub mod green_chirho;
pub mod token_chirho;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use rhasky_span_chirho::{FileIdChirho, SourceMapChirho, SpanChirho};

// ---------------------------------------------------------------------------
// SourceFileChirho
// ---------------------------------------------------------------------------

/// A source file loaded into memory, identified by a [`FileIdChirho`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFileChirho {
    path_chirho: PathBuf,
    contents_chirho: String,
    file_id_chirho: FileIdChirho,
}

impl SourceFileChirho {
    /// Create a source file with a pre-assigned file ID.
    pub fn new_chirho(
        path_chirho: impl Into<PathBuf>,
        contents_chirho: impl Into<String>,
        file_id_chirho: FileIdChirho,
    ) -> Self {
        Self {
            path_chirho: path_chirho.into(),
            contents_chirho: contents_chirho.into(),
            file_id_chirho,
        }
    }

    /// Register a source file in a [`SourceMapChirho`] and return the
    /// `SourceFileChirho` with a proper file ID.
    pub fn from_source_map_chirho(
        source_map_chirho: &mut SourceMapChirho,
        path_chirho: impl Into<PathBuf>,
        contents_chirho: impl Into<String>,
    ) -> Self {
        let path_buf_chirho: PathBuf = path_chirho.into();
        let contents_str_chirho: String = contents_chirho.into();
        let file_id_chirho = source_map_chirho.add_file_chirho(
            path_buf_chirho.to_string_lossy().to_string(),
            contents_str_chirho.clone(),
        );
        Self {
            path_chirho: path_buf_chirho,
            contents_chirho: contents_str_chirho,
            file_id_chirho,
        }
    }

    /// Load a source file from disk and register it in a [`SourceMapChirho`].
    pub fn from_path_with_map_chirho(
        source_map_chirho: &mut SourceMapChirho,
        path_chirho: impl AsRef<Path>,
    ) -> io::Result<Self> {
        let path_buf_chirho = path_chirho.as_ref().to_path_buf();
        let contents_chirho = fs::read_to_string(&path_buf_chirho)?;
        Ok(Self::from_source_map_chirho(
            source_map_chirho,
            path_buf_chirho,
            contents_chirho,
        ))
    }

    /// Load from disk with a synthetic file ID (for backwards compatibility
    /// during the transition — prefer `from_path_with_map_chirho`).
    pub fn from_path_chirho(path_chirho: impl AsRef<Path>) -> io::Result<Self> {
        let path_buf_chirho = path_chirho.as_ref().to_path_buf();
        let contents_chirho = fs::read_to_string(&path_buf_chirho)?;
        Ok(Self::new_chirho(
            path_buf_chirho,
            contents_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
        ))
    }

    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }

    pub fn contents_chirho(&self) -> &str {
        &self.contents_chirho
    }

    pub fn file_id_chirho(&self) -> FileIdChirho {
        self.file_id_chirho
    }
}

// ---------------------------------------------------------------------------
// ModuleHeaderChirho
// ---------------------------------------------------------------------------

/// A parsed module header (`module Foo where`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleHeaderChirho {
    pub module_name_chirho: String,
    pub span_chirho: SpanChirho,
}
