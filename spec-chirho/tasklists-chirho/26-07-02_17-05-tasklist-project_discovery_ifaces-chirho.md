<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Project Discovery And Iface Seeding

- [x] Claim builder slot for targeted project-discovery representatives only.
- [x] Reproduce `discover_setup_hs_chirho` failure.
- [x] Include test-suite `other-modules` and `main-is` in `discover_modules_chirho`.
- [x] Include `Setup.hs`/`Setup.lhs` only for setup-only packages, avoiding normal library package compile loops.
- [x] Seed current-package stub interfaces before sibling dependency package stubs in `scan_dependency_package_ifaces_chirho`.
- [x] Verify `discover_setup_hs_chirho`.
- [x] Verify `discover_test_suite_modules_chirho`.
- [x] Verify `real_containers_dependency_seed_includes_bitutil_iface_chirho`.
- [x] Check `real_containers_package_seed_still_resolves_bitutil_before_inttreecommons_chirho`.
- [x] Record that the remaining containers package-seed failure is now a separate `Key`/`Int` typing frontier with `has_bitutil=true`.
- [x] Update `spec-chirho/prd_chirho.json` to v2.2.61 with the honest remaining count.
