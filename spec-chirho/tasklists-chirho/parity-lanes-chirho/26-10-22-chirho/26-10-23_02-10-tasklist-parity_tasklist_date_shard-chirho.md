<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Parity tasklist date shard Chirho

- [x] Move every existing parity-lane tasklist into its matching date directory without changing its bytes.
- [x] Update the three verified absolute cross-references to the moved paths.
- [x] Prove there are only three root entries, each date directory stays below fifteen entries, and no stale tracked old path remains.
- [ ] Run `git diff --check`, verify moved-file hashes, scan the docs-only range with gitleaks, and close progress row 525.
