// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Hackage URL construction
//!
//! Constructs URLs for fetching packages and cabal files from Hackage.

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
}
