<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Package source install (row 703)

`haskelujah install` publishes a checked source package into a store that builds discover. It
never installs or records an interface or object file: builds compile the published source.

- Store: a directory named `.haskelujah-packages-chirho`, the only name
  `find_dependency_packages_dir_chirho` looks for in a project's ancestors.
- Package directory: `<store>/<name>-<version>/`, one per package name, because builds select a
  single directory per name (`find_local_dependency_package_dir_chirho`).
- Record: `<store>/.source-db-chirho/source-packages-chirho.txt`, a strict, versioned text
  database (`SourcePackageDbChirho`) saying which modules were compiled to Core and which
  builtin or exact store version each dependency was checked against.

```mermaid
flowchart TD
  cli["CLI install_command_chirho"] --> choice{"--from given?"}
  choice -- no --> fetch["fetch_install_input_chirho: Hackage release archive, the only network step"]
  choice -- yes --> local["local_install_input_chirho: directory or archive file"]
  fetch --> install["install_package_chirho"]
  local --> install
  install --> open["SourceStoreChirho::open_chirho: discoverable name, no nested store"]
  open --> lock["lock_chirho: exclusive non-blocking advisory lock"]
  lock --> load["load_db_chirho: missing is empty, damaged is an error"]
  load --> sweep["sweep_staging_chirho: leftovers of crashed installs"]
  sweep --> stage["stage_source_chirho: bounded, link-free copy without dist-chirho"]
  stage --> cabal["read_single_cabal_file_chirho: exactly one .cabal"]
  cabal --> setup["ensure_runs_no_setup_code_chirho: Simple, or Cabal's default Setup"]
  setup --> name["ensure_new_package_name_chirho: no record, no directory for the name"]
  name --> deps["resolve_recorded_dependencies_chirho: builtin, or recorded, satisfying and selected"]
  deps --> inventory["inventory_library_modules_chirho: every listed module; Paths_ written; PackageInfo_ refused"]
  inventory --> check["check_library_compiles_chirho: whole library to Core with store seeds"]
  check --> publish["rename staged directory to store/name-version"]
  publish --> save["save_db_chirho: temp file, sync, rename"]
  save -- saved --> done["InstallResultChirho"]
  save -- failed --> back["move_publication_back_chirho"]
  back -- moved back --> unchanged["SourceInstallErrorChirho: UnchangedChirho"]
  back -- move failed --> unrecorded["SourceInstallErrorChirho: UnrecordedDirectoryChirho"]
  open -. error .-> unchanged
  load -. error .-> unchanged
  stage -. error .-> unchanged
  deps -. error .-> unchanged
  inventory -. error .-> unchanged
  check -. error .-> unchanged
```

## Recovery boundary

Until the directory rename, nothing visible changes. At most the bookkeeping directories
`.source-db-chirho/` and `.staging-chirho/` and the empty lock file are created. The next install
sweeps any staging a crash left, which is safe because only the lock holder sweeps. If the
database save fails after the rename, the directory is moved back. A crash between the two
renames leaves a package directory without a record. Builds would compile it, since it passed
every check, and install refuses to replace it until someone removes it. The operating system
releases the advisory lock when its holder exits, so a crash never leaves the store locked.

## Generated modules

Cabal generates `Paths_<pkg>`, with `-` mangled to `_`. Install writes it into the listing
library's first source directory, where builds discover it too.

- `version` is exact.
- Each directory getter returns the documented `<pkg>_<dir>` override. Without the override it
  fails with an explicit `IOError`, because a source package installs no binaries, libraries,
  data or configuration.
- `PackageInfo_<pkg>` has no generator, so a package listing it is refused.

## Consumers still to change (handed off, row 703)

- The build path's `builtin_dependency_names_chirho` should read
  `haskelujah_package_chirho::BUILTIN_PACKAGE_NAMES_CHIRHO`. Until then a test pins the two
  lists equal.
- `collect_local_dependency_frontend_artifacts_chirho` warns and skips a dependency that fails
  to compile instead of returning the error.
- Builds link only the project's own module bodies (Unit B).
