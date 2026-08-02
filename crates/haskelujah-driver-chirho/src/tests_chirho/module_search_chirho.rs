// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounds on the hierarchical module search.
//!
//! `haskelujah check <file>` searches the checked file's PARENT directory. For
//! a file outside a project that is an arbitrary directory, so the walk must be
//! bounded in depth, breadth and file count, and must terminate on a symlink
//! cycle. Measured before the bounds landed: a file placed directly in
//! `/private/tmp` (80418 directories, 4274 `.hs` files) did not finish within
//! 60 seconds.

use crate::{
    scan_hierarchical_modules_chirho, ModuleSearchBoundsChirho, MAX_MODULE_SEARCH_DIRS_CHIRHO,
    MAX_MODULE_SEARCH_FILES_CHIRHO,
};
use haskelujah_span_chirho::SourceMapChirho;
use std::path::{Path, PathBuf};

/// A scratch directory that removes itself when the test ends.
struct ScratchDirChirho {
    path_chirho: PathBuf,
}

impl ScratchDirChirho {
    fn new_chirho(tag_chirho: &str) -> Self {
        let path_chirho = std::env::temp_dir().join(format!(
            "haskelujah_module_search_{tag_chirho}_chirho_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path_chirho);
        std::fs::create_dir_all(&path_chirho).expect("create scratch dir");
        Self { path_chirho }
    }

    fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }
}

impl Drop for ScratchDirChirho {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path_chirho);
    }
}

fn scan_chirho(dir_chirho: &Path) -> Vec<String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut ifaces_chirho = Vec::new();
    scan_hierarchical_modules_chirho(
        dir_chirho,
        dir_chirho,
        &mut source_map_chirho,
        &mut ifaces_chirho,
        "Main.hs",
    );
    ifaces_chirho
        .into_iter()
        .map(|iface_chirho| iface_chirho.name_chirho)
        .collect()
}

#[test]
fn dir_budget_stops_descending_chirho() {
    let mut bounds_chirho = ModuleSearchBoundsChirho::new_chirho();
    for _ in 0..MAX_MODULE_SEARCH_DIRS_CHIRHO {
        assert!(bounds_chirho.may_enter_dir_chirho());
    }
    assert!(
        !bounds_chirho.may_enter_dir_chirho(),
        "the directory budget must stop the walk once it is spent"
    );
}

#[test]
fn file_budget_stops_parsing_chirho() {
    let mut bounds_chirho = ModuleSearchBoundsChirho::new_chirho();
    for _ in 0..MAX_MODULE_SEARCH_FILES_CHIRHO {
        assert!(bounds_chirho.may_read_file_chirho());
    }
    assert!(
        !bounds_chirho.may_read_file_chirho(),
        "the parse budget must stop the walk once it is spent"
    );
}

#[test]
fn truncation_is_reported_exactly_once_chirho() {
    // A truncated search can leave imports unresolvable. It must say so — but
    // once, not once per directory.
    let mut bounds_chirho = ModuleSearchBoundsChirho::new_chirho();
    assert!(!bounds_chirho.warned_chirho);
    bounds_chirho.dirs_entered_chirho = MAX_MODULE_SEARCH_DIRS_CHIRHO;
    assert!(!bounds_chirho.may_enter_dir_chirho());
    assert!(bounds_chirho.warned_chirho, "exhaustion must not be silent");
    // Second refusal keeps the flag set rather than reporting again.
    assert!(!bounds_chirho.may_enter_dir_chirho());
    assert!(bounds_chirho.warned_chirho);
}

#[test]
fn symlink_cycle_terminates_and_still_finds_siblings_chirho() {
    // `sub/up -> ..` used to be descended into, because `is_dir()` follows
    // symlinks; termination then depended on the OS `ELOOP` limit rather than
    // on us. Symlinked entries are now skipped outright.
    let scratch_chirho = ScratchDirChirho::new_chirho("cycle");
    let root_chirho = scratch_chirho.path_chirho();
    std::fs::write(
        root_chirho.join("SiblingChirho.hs"),
        "module SiblingChirho where\nvalue_chirho :: Int\nvalue_chirho = 1\n",
    )
    .expect("write sibling module");
    let sub_chirho = root_chirho.join("sub_chirho");
    std::fs::create_dir_all(&sub_chirho).expect("create subdirectory");

    #[cfg(unix)]
    std::os::unix::fs::symlink("..", sub_chirho.join("up_chirho")).expect("create symlink cycle");

    let names_chirho = scan_chirho(root_chirho);
    assert!(
        names_chirho.iter().any(|n_chirho| n_chirho == "SiblingChirho"),
        "the cycle guard must not cost us a real sibling module; found {names_chirho:?}"
    );
}

#[test]
fn nested_hierarchical_module_is_still_found_chirho() {
    // The bounds must not break the thing the search exists for: a module in a
    // subdirectory, named by its path (`Deep/NestedChirho.hs` -> Deep.NestedChirho).
    let scratch_chirho = ScratchDirChirho::new_chirho("nested");
    let root_chirho = scratch_chirho.path_chirho();
    let deep_chirho = root_chirho.join("Deep");
    std::fs::create_dir_all(&deep_chirho).expect("create nested directory");
    std::fs::write(
        deep_chirho.join("NestedChirho.hs"),
        "module Deep.NestedChirho where\ndeep_value_chirho :: Int\ndeep_value_chirho = 7\n",
    )
    .expect("write nested module");

    let names_chirho = scan_chirho(root_chirho);
    assert!(
        names_chirho
            .iter()
            .any(|n_chirho| n_chirho == "Deep.NestedChirho"),
        "a nested hierarchical module must still be discovered; found {names_chirho:?}"
    );
}
