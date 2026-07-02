<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# WI-005 MapFoldl Strict And FromEnum Regression

- [x] Wait for Claude's full-probe acceptance attempt and avoid concurrent builds.
- [x] Claim the slot after no probe/build process remained.
- [x] Wire `mapFoldlWithKey'` as a Core wrapper over existing `mapFoldlWithKey#`.
- [x] Restore `mapFoldlWithKey'` to the real `Map k v` seeded type scheme.
- [x] Add a targeted strict-fold evaluation test.
- [x] Fix the v2.2.27 `fromEnum (chr 97)` regression by excluding `Enum` methods from numeric-under-print default rewriting.
- [x] Add a targeted `print (fromEnum (chr 97))` regression pin.
- [x] Run targeted gates only under the resource guard.
- [x] Update PRD/progress and hand off full-probe acceptance to Claude.
