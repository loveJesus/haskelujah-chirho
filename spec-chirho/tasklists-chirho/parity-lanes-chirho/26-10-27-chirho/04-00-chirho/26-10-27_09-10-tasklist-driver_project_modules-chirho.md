<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Driver project modules Chirho

Row 634 (gpt_chirho #30194; placement approved #30200) is behavior-neutral. It moves the
multi-module Core merge, the multi-module compile and eval, and the project and Cabal build flow
out of the driver's 6,472-line `lib.rs`, into bounded modules. It adds no row 629 semantics.

## Placement (approved)

- `native_link_chirho/core_merge_chirho.rs`: the merge, beside the native provenance it maintains.
- `project_chirho/`:
  - `multi_module_chirho.rs`: compile and eval of in-memory sources;
  - `project_dir_chirho.rs`: directory projects;
  - `cabal_build_chirho.rs`: a package's compile and executable build;
  - `dependency_packages_chirho.rs`: local dependency packages;
  - `module_discovery_chirho.rs`: which files a package's components name;
  - `package_install_chirho.rs`: install.
- `FrontendSeedArtifactsChirho` stays in `lib.rs`, beside the stdlib seed cache that also builds it.
  Its fields stay private, which child modules can still read.
- Public items keep their crate-root paths through `pub use`. Moved helpers that another module
  calls are `pub(crate)`.

## Checklist

- [x] Placement checkpoint (#30198), approved with multi-module compile and eval (#30200).
- [x] Commit 1: repair the 17 pre-existing Clippy diagnostics in place. The workspace check is
  warning-free, and the driver library's project, install, eval_* and extensions modules pass
  1,330 tests.
- [x] Commit 2: the move.
  - `lib.rs` goes from 6,463 to 3,133 lines; the new files hold 210 to 851 lines each.
  - Each moved body equals rustfmt of its parent regions, with `pub(crate)` on 24 listed
    signatures; shasums are in the post.
  - The crate-root public set is 40 names before and 40 after, exactly equal.
- [x] Gate on the replayed tip:
  - the driver library's compile, project, install, eval_* and extensions modules (row 620's
    fixture slice): 1,719 passed;
  - `native_symbol_identity` 12 and `native_error_helpers` 60;
  - Cabal and native receipts, main 34c77f71 against the tip: all 67 cells identical, among them
    40 Cabal build-run cells on LLVM and true Cranelift, with no fallback;
  - the warning-free check, changed-line Clippy 0, `git diff --check`, and gitleaks.
