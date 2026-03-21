// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Package database
//!
//! An installed package registry that tracks compiled modules and their
//! interface files. The database is stored as a JSON file on disk and
//! provides lookup, registration, and listing of installed packages.
//!
//! ## Layout
//!
//! ```text
//! ~/.haskelujah/pkgdb/
//!   db.json              -- serialized InstalledPkgDbChirho
//!   <pkg>-<ver>/
//!     *.rhi              -- interface files
//!     *.o / *.bc         -- object files (optional)
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::version_chirho::VersionChirho;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// An entry for a single installed package version.
#[derive(Debug, Clone, PartialEq)]
pub struct InstalledPkgChirho {
    /// Package name (e.g. "containers").
    pub name_chirho: String,
    /// Installed version.
    pub version_chirho: VersionChirho,
    /// Exposed modules and their interface file paths.
    pub exposed_modules_chirho: Vec<InstalledModuleChirho>,
    /// Packages this one depends on (name + version).
    pub depends_chirho: Vec<(String, VersionChirho)>,
    /// Root directory of this package's installed artifacts.
    pub install_dir_chirho: PathBuf,
}

/// A single exposed module in an installed package.
#[derive(Debug, Clone, PartialEq)]
pub struct InstalledModuleChirho {
    /// Fully qualified module name (e.g. "Data.Map.Strict").
    pub module_name_chirho: String,
    /// Path to the interface file (`.rhi`), relative to the package install dir.
    pub iface_path_chirho: PathBuf,
    /// Optional path to the compiled object file.
    pub object_path_chirho: Option<PathBuf>,
}

/// The installed package database — maps `(name, version)` to package info.
#[derive(Debug, Clone, Default)]
pub struct InstalledPkgDbChirho {
    /// Packages keyed by name → vec of installed versions (newest first).
    pub packages_chirho: BTreeMap<String, Vec<InstalledPkgChirho>>,
}

// ---------------------------------------------------------------------------
// Package ID helpers
// ---------------------------------------------------------------------------

/// Construct a package-id string like "containers-0.6.7".
pub fn pkg_id_chirho(name_chirho: &str, version_chirho: &VersionChirho) -> String {
    format!("{}-{}", name_chirho, version_chirho)
}

// ---------------------------------------------------------------------------
// InstalledPkgDbChirho implementation
// ---------------------------------------------------------------------------

impl InstalledPkgDbChirho {
    /// Create an empty database.
    pub fn new_chirho() -> Self {
        Self {
            packages_chirho: BTreeMap::new(),
        }
    }

    /// Register an installed package.
    pub fn register_chirho(&mut self, pkg_chirho: InstalledPkgChirho) {
        let entry_chirho = self
            .packages_chirho
            .entry(pkg_chirho.name_chirho.clone())
            .or_default();

        // Remove any existing entry with the same version (re-install).
        entry_chirho.retain(|p_chirho| p_chirho.version_chirho != pkg_chirho.version_chirho);

        entry_chirho.push(pkg_chirho);

        // Sort newest first.
        entry_chirho
            .sort_by(|a_chirho, b_chirho| b_chirho.version_chirho.cmp(&a_chirho.version_chirho));
    }

    /// Unregister a package version.
    pub fn unregister_chirho(&mut self, name_chirho: &str, version_chirho: &VersionChirho) -> bool {
        if let Some(entry_chirho) = self.packages_chirho.get_mut(name_chirho) {
            let before_chirho = entry_chirho.len();
            entry_chirho.retain(|p_chirho| &p_chirho.version_chirho != version_chirho);
            let removed_chirho = entry_chirho.len() < before_chirho;
            if entry_chirho.is_empty() {
                self.packages_chirho.remove(name_chirho);
            }
            removed_chirho
        } else {
            false
        }
    }

    /// Look up the newest installed version of a package.
    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&InstalledPkgChirho> {
        self.packages_chirho
            .get(name_chirho)
            .and_then(|vs_chirho| vs_chirho.first())
    }

    /// Look up a specific version of a package.
    pub fn lookup_version_chirho(
        &self,
        name_chirho: &str,
        version_chirho: &VersionChirho,
    ) -> Option<&InstalledPkgChirho> {
        self.packages_chirho.get(name_chirho).and_then(|vs_chirho| {
            vs_chirho
                .iter()
                .find(|p_chirho| &p_chirho.version_chirho == version_chirho)
        })
    }

    /// All installed versions of a package, newest first.
    pub fn versions_of_chirho(&self, name_chirho: &str) -> &[InstalledPkgChirho] {
        self.packages_chirho
            .get(name_chirho)
            .map(|v_chirho| v_chirho.as_slice())
            .unwrap_or(&[])
    }

    /// List all installed packages (unique names).
    pub fn list_packages_chirho(&self) -> Vec<&str> {
        self.packages_chirho
            .keys()
            .map(|s_chirho| s_chirho.as_str())
            .collect()
    }

    /// Total number of installed package versions.
    pub fn total_versions_chirho(&self) -> usize {
        self.packages_chirho
            .values()
            .map(|v_chirho| v_chirho.len())
            .sum()
    }

    /// Find which installed package exposes a given module.
    pub fn find_module_chirho(
        &self,
        module_name_chirho: &str,
    ) -> Option<(&InstalledPkgChirho, &InstalledModuleChirho)> {
        for versions_chirho in self.packages_chirho.values() {
            // Check newest version first.
            if let Some(pkg_chirho) = versions_chirho.first() {
                for mod_chirho in &pkg_chirho.exposed_modules_chirho {
                    if mod_chirho.module_name_chirho == module_name_chirho {
                        return Some((pkg_chirho, mod_chirho));
                    }
                }
            }
        }
        None
    }

    /// Get the interface file path for a module from an installed package.
    pub fn iface_path_chirho(&self, module_name_chirho: &str) -> Option<PathBuf> {
        self.find_module_chirho(module_name_chirho)
            .map(|(pkg_chirho, mod_chirho)| {
                pkg_chirho
                    .install_dir_chirho
                    .join(&mod_chirho.iface_path_chirho)
            })
    }

    /// Compute the default database directory (`~/.haskelujah/pkgdb`).
    pub fn default_db_dir_chirho() -> Option<PathBuf> {
        dirs_path_chirho().map(|p_chirho| p_chirho.join("pkgdb"))
    }

    /// Compute the install directory for a specific package version.
    pub fn install_dir_for_chirho(
        db_dir_chirho: &Path,
        name_chirho: &str,
        version_chirho: &VersionChirho,
    ) -> PathBuf {
        db_dir_chirho.join(pkg_id_chirho(name_chirho, version_chirho))
    }
}

/// Get the haskelujah home directory (`~/.haskelujah`).
fn dirs_path_chirho() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|h_chirho| PathBuf::from(h_chirho).join(".haskelujah"))
}

// ---------------------------------------------------------------------------
// Serialization (simple line-based format)
// ---------------------------------------------------------------------------

impl InstalledPkgDbChirho {
    /// Serialize the database to a string (simple text format).
    pub fn to_string_chirho(&self) -> String {
        let mut out_chirho = String::new();
        for (name_chirho, versions_chirho) in &self.packages_chirho {
            for pkg_chirho in versions_chirho {
                out_chirho.push_str(&format!(
                    "pkg:{} {}\n",
                    name_chirho, pkg_chirho.version_chirho
                ));
                out_chirho.push_str(&format!(
                    "  dir:{}\n",
                    pkg_chirho.install_dir_chirho.display()
                ));
                for dep_chirho in &pkg_chirho.depends_chirho {
                    out_chirho.push_str(&format!("  dep:{} {}\n", dep_chirho.0, dep_chirho.1));
                }
                for mod_chirho in &pkg_chirho.exposed_modules_chirho {
                    out_chirho.push_str(&format!(
                        "  mod:{} {}\n",
                        mod_chirho.module_name_chirho,
                        mod_chirho.iface_path_chirho.display()
                    ));
                    if let Some(obj_chirho) = &mod_chirho.object_path_chirho {
                        out_chirho.push_str(&format!("  obj:{}\n", obj_chirho.display()));
                    }
                }
            }
        }
        out_chirho
    }

    /// Parse a database from the text format.
    pub fn from_string_chirho(input_chirho: &str) -> Self {
        let mut db_chirho = Self::new_chirho();
        let mut current_pkg_chirho: Option<InstalledPkgChirho> = None;

        for line_chirho in input_chirho.lines() {
            let trimmed_chirho = line_chirho.trim();

            if let Some(rest_chirho) = trimmed_chirho.strip_prefix("pkg:") {
                // Flush previous package.
                if let Some(pkg_chirho) = current_pkg_chirho.take() {
                    db_chirho.register_chirho(pkg_chirho);
                }

                let parts_chirho: Vec<&str> = rest_chirho.splitn(2, ' ').collect();
                if parts_chirho.len() == 2 {
                    let name_chirho = parts_chirho[0].to_string();
                    let version_chirho =
                        crate::version_chirho::parse_version_chirho(parts_chirho[1])
                            .unwrap_or_else(|| VersionChirho::new_chirho(vec![0]));
                    current_pkg_chirho = Some(InstalledPkgChirho {
                        name_chirho,
                        version_chirho,
                        exposed_modules_chirho: Vec::new(),
                        depends_chirho: Vec::new(),
                        install_dir_chirho: PathBuf::new(),
                    });
                }
            } else if let Some(rest_chirho) = trimmed_chirho.strip_prefix("dir:") {
                if let Some(pkg_chirho) = &mut current_pkg_chirho {
                    pkg_chirho.install_dir_chirho = PathBuf::from(rest_chirho);
                }
            } else if let Some(rest_chirho) = trimmed_chirho.strip_prefix("dep:") {
                if let Some(pkg_chirho) = &mut current_pkg_chirho {
                    let parts_chirho: Vec<&str> = rest_chirho.splitn(2, ' ').collect();
                    if parts_chirho.len() == 2 {
                        let dep_name_chirho = parts_chirho[0].to_string();
                        let dep_ver_chirho =
                            crate::version_chirho::parse_version_chirho(parts_chirho[1])
                                .unwrap_or_else(|| VersionChirho::new_chirho(vec![0]));
                        pkg_chirho
                            .depends_chirho
                            .push((dep_name_chirho, dep_ver_chirho));
                    }
                }
            } else if let Some(rest_chirho) = trimmed_chirho.strip_prefix("mod:") {
                if let Some(pkg_chirho) = &mut current_pkg_chirho {
                    let parts_chirho: Vec<&str> = rest_chirho.splitn(2, ' ').collect();
                    if parts_chirho.len() == 2 {
                        pkg_chirho
                            .exposed_modules_chirho
                            .push(InstalledModuleChirho {
                                module_name_chirho: parts_chirho[0].to_string(),
                                iface_path_chirho: PathBuf::from(parts_chirho[1]),
                                object_path_chirho: None,
                            });
                    }
                }
            } else if let Some(rest_chirho) = trimmed_chirho.strip_prefix("obj:") {
                if let Some(pkg_chirho) = &mut current_pkg_chirho {
                    if let Some(last_mod_chirho) = pkg_chirho.exposed_modules_chirho.last_mut() {
                        last_mod_chirho.object_path_chirho = Some(PathBuf::from(rest_chirho));
                    }
                }
            }
        }

        // Flush last package.
        if let Some(pkg_chirho) = current_pkg_chirho {
            db_chirho.register_chirho(pkg_chirho);
        }

        db_chirho
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn v_chirho(s_chirho: &str) -> VersionChirho {
        crate::version_chirho::parse_version_chirho(s_chirho).unwrap()
    }

    fn make_pkg_chirho(
        name_chirho: &str,
        ver_chirho: &str,
        modules_chirho: &[&str],
    ) -> InstalledPkgChirho {
        InstalledPkgChirho {
            name_chirho: name_chirho.to_string(),
            version_chirho: v_chirho(ver_chirho),
            exposed_modules_chirho: modules_chirho
                .iter()
                .map(|m_chirho| InstalledModuleChirho {
                    module_name_chirho: m_chirho.to_string(),
                    iface_path_chirho: PathBuf::from(format!("{}.rhi", m_chirho.replace('.', "/"))),
                    object_path_chirho: None,
                })
                .collect(),
            depends_chirho: Vec::new(),
            install_dir_chirho: PathBuf::from(format!("/lib/{}-{}", name_chirho, ver_chirho)),
        }
    }

    #[test]
    fn register_and_lookup_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho(
            "containers",
            "0.6.7",
            &["Data.Map", "Data.Set"],
        ));

        let pkg_chirho = db_chirho.lookup_chirho("containers").unwrap();
        assert_eq!(pkg_chirho.name_chirho, "containers");
        assert_eq!(pkg_chirho.version_chirho, v_chirho("0.6.7"));
        assert_eq!(pkg_chirho.exposed_modules_chirho.len(), 2);
    }

    #[test]
    fn lookup_version_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));
        db_chirho.register_chirho(make_pkg_chirho("text", "1.5", &["Data.Text"]));

        let pkg_chirho = db_chirho
            .lookup_version_chirho("text", &v_chirho("1.5"))
            .unwrap();
        assert_eq!(pkg_chirho.version_chirho, v_chirho("1.5"));

        // Newest first
        let newest_chirho = db_chirho.lookup_chirho("text").unwrap();
        assert_eq!(newest_chirho.version_chirho, v_chirho("2.0"));
    }

    #[test]
    fn unregister_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));
        db_chirho.register_chirho(make_pkg_chirho("text", "1.5", &["Data.Text"]));

        assert!(db_chirho.unregister_chirho("text", &v_chirho("1.5")));
        assert_eq!(db_chirho.versions_of_chirho("text").len(), 1);

        assert!(db_chirho.unregister_chirho("text", &v_chirho("2.0")));
        assert!(db_chirho.lookup_chirho("text").is_none());
        assert!(!db_chirho.unregister_chirho("text", &v_chirho("2.0")));
    }

    #[test]
    fn find_module_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho(
            "containers",
            "0.6.7",
            &["Data.Map", "Data.Set"],
        ));
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));

        let (pkg_chirho, mod_chirho) = db_chirho.find_module_chirho("Data.Map").unwrap();
        assert_eq!(pkg_chirho.name_chirho, "containers");
        assert_eq!(mod_chirho.module_name_chirho, "Data.Map");

        assert!(db_chirho.find_module_chirho("Data.ByteString").is_none());
    }

    #[test]
    fn iface_path_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho("containers", "0.6.7", &["Data.Map"]));

        let path_chirho = db_chirho.iface_path_chirho("Data.Map").unwrap();
        assert_eq!(
            path_chirho,
            PathBuf::from("/lib/containers-0.6.7/Data/Map.rhi")
        );
    }

    #[test]
    fn list_packages_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho("containers", "0.6.7", &["Data.Map"]));
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));

        let names_chirho = db_chirho.list_packages_chirho();
        assert_eq!(names_chirho, vec!["containers", "text"]);
    }

    #[test]
    fn total_versions_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));
        db_chirho.register_chirho(make_pkg_chirho("text", "1.5", &["Data.Text"]));
        db_chirho.register_chirho(make_pkg_chirho("containers", "0.6.7", &["Data.Map"]));

        assert_eq!(db_chirho.total_versions_chirho(), 3);
    }

    #[test]
    fn reinstall_replaces_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));
        db_chirho.register_chirho(make_pkg_chirho(
            "text",
            "2.0",
            &["Data.Text", "Data.Text.Lazy"],
        ));

        // Should replace, not duplicate.
        assert_eq!(db_chirho.versions_of_chirho("text").len(), 1);
        assert_eq!(
            db_chirho
                .lookup_chirho("text")
                .unwrap()
                .exposed_modules_chirho
                .len(),
            2
        );
    }

    #[test]
    fn serialize_roundtrip_chirho() {
        let mut db_chirho = InstalledPkgDbChirho::new_chirho();
        let mut pkg_chirho = make_pkg_chirho("containers", "0.6.7", &["Data.Map", "Data.Set"]);
        pkg_chirho.depends_chirho = vec![
            ("base".to_string(), v_chirho("4.19")),
            ("deepseq".to_string(), v_chirho("1.5")),
        ];
        db_chirho.register_chirho(pkg_chirho);
        db_chirho.register_chirho(make_pkg_chirho("text", "2.0", &["Data.Text"]));

        let serialized_chirho = db_chirho.to_string_chirho();
        let restored_chirho = InstalledPkgDbChirho::from_string_chirho(&serialized_chirho);

        assert_eq!(
            restored_chirho.list_packages_chirho(),
            vec!["containers", "text"]
        );
        assert_eq!(restored_chirho.total_versions_chirho(), 2);

        let containers_chirho = restored_chirho.lookup_chirho("containers").unwrap();
        assert_eq!(containers_chirho.exposed_modules_chirho.len(), 2);
        assert_eq!(containers_chirho.depends_chirho.len(), 2);
        assert_eq!(containers_chirho.depends_chirho[0].0, "base");
    }

    #[test]
    fn empty_db_chirho() {
        let db_chirho = InstalledPkgDbChirho::new_chirho();
        assert!(db_chirho.lookup_chirho("anything").is_none());
        assert!(db_chirho.list_packages_chirho().is_empty());
        assert_eq!(db_chirho.total_versions_chirho(), 0);
    }

    #[test]
    fn pkg_id_format_chirho() {
        assert_eq!(
            pkg_id_chirho("containers", &v_chirho("0.6.7")),
            "containers-0.6.7"
        );
    }

    #[test]
    fn install_dir_for_chirho() {
        let dir_chirho = InstalledPkgDbChirho::install_dir_for_chirho(
            Path::new("/home/user/.haskelujah/pkgdb"),
            "text",
            &v_chirho("2.0"),
        );
        assert_eq!(
            dir_chirho,
            PathBuf::from("/home/user/.haskelujah/pkgdb/text-2.0")
        );
    }
}
