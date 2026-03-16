// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Hackage package fetching
//!
//! Constructs URLs for fetching packages from Hackage, downloads `.tar.gz`
//! archives, extracts them, and parses their `.cabal` files.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::cabal_chirho::{parse_cabal_chirho, PackageDescChirho};
use crate::version_chirho::VersionChirho;

/// Hackage base URL.
const HACKAGE_BASE_CHIRHO: &str = "https://hackage.haskell.org/package";

/// Construct the URL for a package tarball on Hackage.
///
/// E.g. `https://hackage.haskell.org/package/base-4.19.0.0/base-4.19.0.0.tar.gz`
pub fn hackage_tarball_url_chirho(
    name_chirho: &str,
    version_chirho: &VersionChirho,
) -> String {
    let pkg_id_chirho = format!("{}-{}", name_chirho, version_chirho);
    format!(
        "{}/{}/{}.tar.gz",
        HACKAGE_BASE_CHIRHO, pkg_id_chirho, pkg_id_chirho
    )
}

/// Construct the URL for a package's `.cabal` file on Hackage.
///
/// E.g. `https://hackage.haskell.org/package/base-4.19.0.0/base.cabal`
pub fn hackage_cabal_url_chirho(
    name_chirho: &str,
    version_chirho: &VersionChirho,
) -> String {
    let pkg_id_chirho = format!("{}-{}", name_chirho, version_chirho);
    format!(
        "{}/{}/{}.cabal",
        HACKAGE_BASE_CHIRHO, pkg_id_chirho, name_chirho
    )
}

/// Construct the URL for the latest revision index of a package.
///
/// E.g. `https://hackage.haskell.org/package/base/preferred`
pub fn hackage_preferred_url_chirho(name_chirho: &str) -> String {
    format!("{}/{}/preferred", HACKAGE_BASE_CHIRHO, name_chirho)
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur during Hackage operations.
#[derive(Debug)]
pub enum HackageErrorChirho {
    /// HTTP request failed.
    HttpChirho(String),
    /// I/O error during extraction.
    IoChirho(std::io::Error),
    /// No `.cabal` file found in the archive.
    NoCabalFileChirho,
    /// Invalid archive format.
    InvalidArchiveChirho(String),
}

impl std::fmt::Display for HackageErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HttpChirho(msg_chirho) => write!(f_chirho, "HTTP error: {}", msg_chirho),
            Self::IoChirho(err_chirho) => write!(f_chirho, "I/O error: {}", err_chirho),
            Self::NoCabalFileChirho => write!(f_chirho, "no .cabal file found in archive"),
            Self::InvalidArchiveChirho(msg_chirho) => {
                write!(f_chirho, "invalid archive: {}", msg_chirho)
            }
        }
    }
}

impl std::error::Error for HackageErrorChirho {}

impl From<std::io::Error> for HackageErrorChirho {
    fn from(err_chirho: std::io::Error) -> Self {
        Self::IoChirho(err_chirho)
    }
}

// ---------------------------------------------------------------------------
// Download functions
// ---------------------------------------------------------------------------

/// Download a package tarball from Hackage into a byte vector.
pub fn download_tarball_chirho(
    name_chirho: &str,
    version_chirho: &VersionChirho,
) -> Result<Vec<u8>, HackageErrorChirho> {
    let url_chirho = hackage_tarball_url_chirho(name_chirho, version_chirho);
    let mut response_chirho = ureq::get(&url_chirho)
        .call()
        .map_err(|e_chirho| HackageErrorChirho::HttpChirho(e_chirho.to_string()))?;

    let mut body_chirho = Vec::new();
    response_chirho
        .body_mut()
        .as_reader()
        .read_to_end(&mut body_chirho)?;
    Ok(body_chirho)
}

/// Download only the `.cabal` file for a package from Hackage.
pub fn download_cabal_file_chirho(
    name_chirho: &str,
    version_chirho: &VersionChirho,
) -> Result<String, HackageErrorChirho> {
    let url_chirho = hackage_cabal_url_chirho(name_chirho, version_chirho);
    let mut response_chirho = ureq::get(&url_chirho)
        .call()
        .map_err(|e_chirho| HackageErrorChirho::HttpChirho(e_chirho.to_string()))?;

    let body_chirho = response_chirho
        .body_mut()
        .read_to_string()
        .map_err(|e_chirho| HackageErrorChirho::HttpChirho(e_chirho.to_string()))?;
    Ok(body_chirho)
}

/// Download and parse a package's `.cabal` file from Hackage.
pub fn fetch_cabal_desc_chirho(
    name_chirho: &str,
    version_chirho: &VersionChirho,
) -> Result<PackageDescChirho, HackageErrorChirho> {
    let cabal_text_chirho = download_cabal_file_chirho(name_chirho, version_chirho)?;
    Ok(parse_cabal_chirho(&cabal_text_chirho))
}

// ---------------------------------------------------------------------------
// Tar.gz extraction
// ---------------------------------------------------------------------------

/// Extract a `.tar.gz` byte buffer into a directory.
pub fn extract_tarball_chirho(
    tarball_chirho: &[u8],
    dest_chirho: &Path,
) -> Result<(), HackageErrorChirho> {
    let decoder_chirho = flate2::read::GzDecoder::new(tarball_chirho);
    let mut archive_chirho = tar::Archive::new(decoder_chirho);
    archive_chirho
        .unpack(dest_chirho)
        .map_err(HackageErrorChirho::IoChirho)?;
    Ok(())
}

/// Extract a `.tar.gz` byte buffer and find the `.cabal` file content.
pub fn extract_cabal_from_tarball_chirho(
    tarball_chirho: &[u8],
) -> Result<String, HackageErrorChirho> {
    let decoder_chirho = flate2::read::GzDecoder::new(tarball_chirho);
    let mut archive_chirho = tar::Archive::new(decoder_chirho);

    for entry_result_chirho in archive_chirho.entries()? {
        let mut entry_chirho = entry_result_chirho?;
        let path_chirho = entry_chirho.path()?.to_path_buf();

        if let Some(ext_chirho) = path_chirho.extension() {
            if ext_chirho == "cabal" {
                let mut content_chirho = String::new();
                entry_chirho.read_to_string(&mut content_chirho)?;
                return Ok(content_chirho);
            }
        }
    }

    Err(HackageErrorChirho::NoCabalFileChirho)
}

/// List all files in a `.tar.gz` archive.
pub fn list_tarball_files_chirho(
    tarball_chirho: &[u8],
) -> Result<Vec<PathBuf>, HackageErrorChirho> {
    let decoder_chirho = flate2::read::GzDecoder::new(tarball_chirho);
    let mut archive_chirho = tar::Archive::new(decoder_chirho);
    let mut files_chirho = Vec::new();

    for entry_result_chirho in archive_chirho.entries()? {
        let entry_chirho = entry_result_chirho?;
        let path_chirho = entry_chirho.path()?.to_path_buf();
        files_chirho.push(path_chirho);
    }

    Ok(files_chirho)
}

/// Download a package, extract it, and parse its `.cabal` file.
/// Returns the parsed description and the list of all files.
pub fn fetch_package_chirho(
    name_chirho: &str,
    version_chirho: &VersionChirho,
    dest_dir_chirho: &Path,
) -> Result<PackageDescChirho, HackageErrorChirho> {
    let tarball_chirho = download_tarball_chirho(name_chirho, version_chirho)?;
    extract_tarball_chirho(&tarball_chirho, dest_dir_chirho)?;

    // Find the .cabal file in the extracted directory.
    let pkg_dir_chirho = dest_dir_chirho.join(format!("{}-{}", name_chirho, version_chirho));
    let cabal_path_chirho = pkg_dir_chirho.join(format!("{}.cabal", name_chirho));

    if cabal_path_chirho.exists() {
        let content_chirho = std::fs::read_to_string(&cabal_path_chirho)?;
        Ok(parse_cabal_chirho(&content_chirho))
    } else {
        // Try to find any .cabal file in the directory.
        if let Ok(entries_chirho) = std::fs::read_dir(&pkg_dir_chirho) {
            for entry_chirho in entries_chirho.flatten() {
                if entry_chirho.path().extension().is_some_and(|e_chirho| e_chirho == "cabal") {
                    let content_chirho = std::fs::read_to_string(entry_chirho.path())?;
                    return Ok(parse_cabal_chirho(&content_chirho));
                }
            }
        }
        Err(HackageErrorChirho::NoCabalFileChirho)
    }
}

/// Create a minimal `.tar.gz` in memory containing one file.
/// Useful for testing without network access.
pub fn create_test_tarball_chirho(
    file_path_chirho: &str,
    content_chirho: &str,
) -> Vec<u8> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    let mut encoder_chirho = GzEncoder::new(Vec::new(), Compression::default());
    {
        let mut builder_chirho = tar::Builder::new(&mut encoder_chirho);
        let data_chirho = content_chirho.as_bytes();
        let mut header_chirho = tar::Header::new_gnu();
        header_chirho.set_path(file_path_chirho).unwrap();
        header_chirho.set_size(data_chirho.len() as u64);
        header_chirho.set_mode(0o644);
        header_chirho.set_cksum();
        builder_chirho
            .append(&header_chirho, data_chirho)
            .unwrap();
        builder_chirho.finish().unwrap();
    }
    encoder_chirho.finish().unwrap()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::version_chirho::parse_version_chirho;

    #[test]
    fn tarball_url_chirho() {
        let v_chirho = parse_version_chirho("4.19.0.0").unwrap();
        let url_chirho = hackage_tarball_url_chirho("base", &v_chirho);
        assert_eq!(
            url_chirho,
            "https://hackage.haskell.org/package/base-4.19.0.0/base-4.19.0.0.tar.gz"
        );
    }

    #[test]
    fn cabal_url_chirho() {
        let v_chirho = parse_version_chirho("1.4.2.0").unwrap();
        let url_chirho = hackage_cabal_url_chirho("text", &v_chirho);
        assert_eq!(
            url_chirho,
            "https://hackage.haskell.org/package/text-1.4.2.0/text.cabal"
        );
    }

    #[test]
    fn preferred_url_chirho() {
        let url_chirho = hackage_preferred_url_chirho("containers");
        assert_eq!(
            url_chirho,
            "https://hackage.haskell.org/package/containers/preferred"
        );
    }

    #[test]
    fn create_and_extract_cabal_from_tarball_chirho() {
        let cabal_content_chirho = r#"
name: test-pkg
version: 1.0.0
library
  exposed-modules: Lib
"#;
        let tarball_chirho = create_test_tarball_chirho(
            "test-pkg-1.0.0/test-pkg.cabal",
            cabal_content_chirho,
        );

        let extracted_chirho = extract_cabal_from_tarball_chirho(&tarball_chirho).unwrap();
        assert!(extracted_chirho.contains("test-pkg"));
        assert!(extracted_chirho.contains("1.0.0"));
    }

    #[test]
    fn list_tarball_files_chirho_test() {
        let tarball_chirho = create_test_tarball_chirho(
            "my-pkg-0.1/my-pkg.cabal",
            "name: my-pkg\nversion: 0.1\n",
        );

        let files_chirho = list_tarball_files_chirho(&tarball_chirho).unwrap();
        assert_eq!(files_chirho.len(), 1);
        assert!(files_chirho[0].to_string_lossy().contains("my-pkg.cabal"));
    }

    #[test]
    fn extract_tarball_to_dir_chirho() {
        let cabal_content_chirho = "name: foo\nversion: 2.0\n";
        let tarball_chirho = create_test_tarball_chirho(
            "foo-2.0/foo.cabal",
            cabal_content_chirho,
        );

        let dir_chirho = tempfile::tempdir().unwrap();
        extract_tarball_chirho(&tarball_chirho, dir_chirho.path()).unwrap();

        let cabal_path_chirho = dir_chirho.path().join("foo-2.0").join("foo.cabal");
        assert!(cabal_path_chirho.exists());

        let content_chirho = std::fs::read_to_string(&cabal_path_chirho).unwrap();
        assert!(content_chirho.contains("foo"));
    }

    #[test]
    fn extract_cabal_no_cabal_file_chirho() {
        let tarball_chirho = create_test_tarball_chirho(
            "README.md",
            "# Hello",
        );

        let result_chirho = extract_cabal_from_tarball_chirho(&tarball_chirho);
        assert!(result_chirho.is_err());
    }

    #[test]
    fn create_and_parse_cabal_from_tarball_chirho() {
        let cabal_content_chirho = r#"
name: hello-world
version: 0.1.0.0
synopsis: A simple package

library
  exposed-modules: Hello, Hello.World
  build-depends: base >=4.14
  default-language: Haskell2010

executable hello
  main-is: Main.hs
  build-depends: base, hello-world
"#;
        let tarball_chirho = create_test_tarball_chirho(
            "hello-world-0.1.0.0/hello-world.cabal",
            cabal_content_chirho,
        );

        let cabal_text_chirho = extract_cabal_from_tarball_chirho(&tarball_chirho).unwrap();
        let pkg_chirho = parse_cabal_chirho(&cabal_text_chirho);

        assert_eq!(pkg_chirho.name_chirho, "hello-world");
        let lib_chirho = pkg_chirho.library_chirho.as_ref().unwrap();
        assert_eq!(lib_chirho.exposed_modules_chirho, vec!["Hello", "Hello.World"]);
        assert_eq!(pkg_chirho.executables_chirho.len(), 1);
        assert_eq!(pkg_chirho.executables_chirho[0].name_chirho, "hello");
    }
}
