<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Evidenced print fail-closed lookups Chirho

Row 556 makes Row 550's applied and first-class `print` route accept only concrete,
body-backed Show rows and helper bindings. A dangling name-table ID or generated
missing-method body no longer authorizes the specialized route.

## Contract

- `$prim_Show_show_K` resolves through the body-backed dispatch lookup.
- `putStrLn` and `$print_via_show_chirho` resolve through the body-backed name lookup.
- If any required body is absent, both applied and first-class `print` keep the existing
  canonical route instead of manufacturing a call through a placeholder ID.
- Row 551's compare route and Row 552's evidenced aggregate route remain unchanged.

## Checklist

- [x] Add one walker-level control for dangling and body-backed applied and first-class
  `print` occurrences.
- [x] Verify the control is sensitive to restoring the old name-table-only lookups.
- [x] Pass core 170/170 and the six affected driver suites: native error helpers 36/36,
  dictionary evidence 81/81, numeric hierarchy 6/6, Show precedence 2/2, IO actions 9/9,
  and typing integration 43/43.
- [x] Verify zero changed-line Clippy findings, rustfmt, diff-check, and gitleaks.
- [x] Land source tip `1184cc93` and record progress row 556.
