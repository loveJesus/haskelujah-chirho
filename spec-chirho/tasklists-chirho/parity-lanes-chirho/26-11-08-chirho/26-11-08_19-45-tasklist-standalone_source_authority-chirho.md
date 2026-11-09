<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# P1 standalone source authority (row 692, gpt #33306, #33308, #33309)

Branch `mr-fundep-chirho` from 784022ac, worktree `haskelujah-workspaces-chirho/haskelujah-mr-fundep-chirho`. The authoring GO is source only: no Cargo, GHC, CLI or native run, and no runner was created.

- [x] Companion controls (test-only): the byte-exact r2 fixtures plus `fundep_improvement_chirho.rs`. Committed d0b50297.
- [x] AST carrier `SourceUnitAuthorityChirho`: it holds a driver-produced unit and its admitted home modules, never mints, and uses exact membership. Re-exported from `name_chirho`.
- [x] Typing carrier: `ElaborationImportsChirho.source_unit_chirho` feeds `ElaborationPolicyChirho.source_unit_chirho`. Every other builder passes `None`:
  - family_discovery, source_family_registration and surface prefix;
  - 8 test literals.
- [x] Join: the `constraint_head_proof_chirho` Source branch, under an admitted home module only.
- [x] Class, method predicates and instance head share `declared_source_class_proof_chirho`. A non-source result keeps each site's existing path.
- [x] Driver producer:
  - `FrontendPolicyChirho.standalone_home_chirho`, set only by the single-source route and `run_standalone_frontend_chirho`, which the two CLI check functions call;
  - `standalone_home_authority_chirho` mints `main` at the pipeline boundary, over this module plus `HomeChirho` admissions.
- [x] Consumer (#33309): `superclass_search_chirho.rs` routes proven identities through the arena's shared `class_record_by_identity_chirho`. A source proof never meets an unknown side by text.
- [x] Controls:
  - ast carrier exactness;
  - driver: the public route's local class, raw library and project entries, home admission versus package exposure, and a home sibling class;
  - superclass: no same-text source expansion, and no discharge of a MissingUnit wanted by spelling.
- [x] Static checks: rustfmt via stdin, `git diff --check`, a single minting site, the standalone call graph, and every imports literal adapted.
- [x] Workflow: the `dictionary-evidence-chirho.md` section "Standalone source class authority (P1, row 692)".
- [x] #33315 correction: a recognized source instance head under the standalone authority keeps its own join result (Source or the join's unknown) and never reaches the spelling fallback. That fallback never carries a source proof (a selected source record gives the legacy unknown). Without the authority every site keeps its pre-P1 path. Focused negative: unadmitted, opaque and unaccounted heads beside an active same-text source class (typing `class_instance_tests_chirho.rs`).
- [x] #33316 provenance: `unit_free_definers_chirho` withholds home names that another unit could also define. These are names under a non-home exposure, modules that non-home accounts define in (hidden package modules behind re-exports), and modules that home interfaces re-export from. Controls: an end-to-end same-definer-name Home/Packages collision through `PackageImports` stays MissingUnit, and a membership control covers the hidden and re-exported definers.
- [x] #33339 correction (controls #33340, producer #33343). It replaces module membership in the carrier, join and producer items above, and the #33316 withholding, with exact accounts.
  - The authority carries the checked module's own class declaration accounts, read from the pipeline's existing type scope and checked against the declaration's name and module.
  - Every current account whose identity a selected import's full export set also names is dropped. An import with no selected interface declines the authority.
  - `unit_free_definers_chirho` and the module membership API are gone.
  - Home siblings now stay MissingUnit. Their earlier Source expectation was never measured, and the driver test records the change.
  - Controls:
    - carrier exactness;
    - an actual-driver home module that forwards a same-named package class, with the package interface absent;
    - the current module's own class colliding with a relayed package class: local and relayed are both MissingUnit and compiler Num is unchanged, while the same module alone is still proven;
    - membership over home, relay, hidden and package interfaces;
    - the #33316 collision.
  - The typing `class_instance_tests_chirho.rs` carrier call was adapted (cleared in #33355).
- [ ] Stock zero-message first check at the #33339 leaf (cargo check, workspace, all targets, JSON) on a fresh GO. The 7f0c9ae7 and 08e0943c packets stay immutable and UNRUN, and the target is absent.
- [ ] Measured preservation on its own GO: N1, N2, T, compiler identity controls, the standard MR controls, and these new controls.
- [ ] Later dependent bricks: P2 (kinded instance binder catalogue) and P3 (one checked fundep primitive, also used by the legacy caller), then the seven-witness cohort with its runtime dictionary handoff.
