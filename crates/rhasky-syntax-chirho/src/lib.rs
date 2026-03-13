// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFileChirho {
    path_chirho: PathBuf,
    contents_chirho: String,
}

impl SourceFileChirho {
    pub fn new_chirho(
        path_chirho: impl Into<PathBuf>,
        contents_chirho: impl Into<String>,
    ) -> Self {
        Self {
            path_chirho: path_chirho.into(),
            contents_chirho: contents_chirho.into(),
        }
    }

    pub fn from_path_chirho(path_chirho: impl AsRef<Path>) -> io::Result<Self> {
        let path_buf_chirho = path_chirho.as_ref().to_path_buf();
        let contents_chirho = fs::read_to_string(&path_buf_chirho)?;
        Ok(Self::new_chirho(path_buf_chirho, contents_chirho))
    }

    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }

    pub fn contents_chirho(&self) -> &str {
        &self.contents_chirho
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleHeaderChirho {
    pub module_name_chirho: String,
    pub line_number_chirho: usize,
}

