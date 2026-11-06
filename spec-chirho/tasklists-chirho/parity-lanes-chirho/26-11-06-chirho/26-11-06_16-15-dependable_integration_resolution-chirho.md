<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Dependable GHC source integration resolution Chirho

SOURCE ONLY. Isolated root-owned assembly on `dependable-integration-chirho`; main, progress SQLite, shared caches and remotes remain untouched by this unit. Root owns acceptance and landing. No compiler, GHC, product, native or RTS execution is authorized for this assembly. Numeric MR, runtime measurement and package-source changes remain separate.

- Kind parent: `29a5b2b272df9624ff03cbd38e47f88462373e4a`; crates `6f327fa57ca7dab79bb4cdb33ca26bacd3ce7a58`.
- Reader parent: `54dd469f34a59265b605f8812c1f7c4a8584f0e1`; crates `1b3071281cde458f6dbb022af2d8a620f973017f`.
- Resolved crates tree: `9348cc73b535576664c27256d2d266db581a085a`.
- Merge uses both parents without rewriting either source history. Incoming Reader source includes the rejected fallback and its removal in history; no fallback is reintroduced.
- Root instructions: critical-path checklist at canonical `spec-chirho/tasklists-chirho/parity-lanes-chirho/26-11-06-chirho/26-11-06_15-05-tasklist-dependable_ghc-chirho.md`; Reader handoff names D/K/E/G/S invariants. Root explicitly cleared reached-set return metadata and complete-capture migration during this assembly.

## Plan and source check status

- [x] Verify exact source parents, clean canonical checkout and worktree placement; create isolated worktree.
- [x] Resolve the eight actual conflict paths, retaining both source contracts.
- [x] Migrate auto-merged Reader APIs to kind arena/views/factories and carry returned admission metadata.
- [x] Standalone rustfmt with `skip_children=true` over incoming/modified Rust paths; source parse/format only.
- [x] `git diff --check`; no unresolved conflict markers.
- [x] Commit isolated merge and scan outgoing `51b27ed1c219fce68427089b81d3d433fc126a2c..HEAD` with gitleaks: 145 commits, approximately 6.18 MB, no leaks. Staged scan also found no leaks.
- [ ] Root and both implementation writers review combined source before any test lease, push or landing.

## Actual conflicts

Each row records the two immutable input blobs and the resolved working blob. Hashes identify source, not executable behavior.

| Path | Kind blob | Reader blob | Resolved blob | Resolution |
| --- | --- | --- | --- | --- |
| `crates/haskelujah-core-chirho/src/dict_chirho/instance_chirho/generic_heads_chirho.rs` | `0a51164c1286ba609b0d6f4b13ee2fdc735e99b4` | `6d0a08287811e2987a4ba3529f31c82ef48d48bd` | `c5d0508a49c87ba4aa5da424b680fb643e09fa1a` | Reader shared matching/provider logic; kind arena views, complete superclass telescope and instance records. |
| `crates/haskelujah-core-chirho/src/dict_chirho/mod.rs` | `a8968ecaa17b8fe12c4654212d511ef259ffeef6` | `f2096512b3cb642dd0137ab2dde6b56eda18320d` | `f14073531b5adbdde75d891e7b7575459b1f0280` | Both context payloads; kind structured inputs and S1/S2/S3 retained; per-module K01 carried in evidence. |
| `crates/haskelujah-driver-chirho/src/backend_chirho/compile_backend_chirho.rs` | `2a242b5ddf7f9a9ebc636a121d38a537feb30b90` | `2a37afa379f84ce22c51ce7df2cd3b4990231ada` | `4156b55a9e93b935e93d046056bc441cbfd69c25` | Kind structured dictionary call, Reader declaration inventory and fresh naming K01 scope. |
| `crates/haskelujah-driver-chirho/src/backend_chirho/evidence_join_chirho.rs` | `e323d91bbce2e8310c0f6d686f768e91aea4882f` | `db040c9e0c5cfbf3a3df811fa1efd791af101382` | `c512a9352f274ebce6361ad5c9980faa23421d64` | Kind JoinedProof map and ground/applied payload; Reader local declarations and concrete admission; complete capture handling below. |
| `crates/haskelujah-driver-chirho/src/frontend_chirho/module_search_chirho.rs` | `3d523c30a33ebf78b5e08fa355c89336b8e9f179` | `20d3ef993728908478b02c9bc44ace6fc2d309ab` | `64e4add22e158c6fa8e2a247ed38e6588c345bf3` | Kind authoritative AST carrier and bounded ModuleSearchWalk; parsed-success test admissions populate Reader inventory. |
| `crates/haskelujah-driver-chirho/src/lib.rs` | `4adaffb16378461a4ca278784189af10c0c7d115` | `4dc55bd2dfe6a7946133064ff8426b807a6ea339` | `5787054a098803fe99ab74bf32a6bfd3de2eab53` | Kind search-context frontend; successful source supplier return passed to backend without linking dependency Core. |
| `crates/haskelujah-typing-chirho/src/class_chirho.rs` | `absent` | `9599eb6dc6290d9fc4e7cc7525d578dbaa14b6c4` | `deleted` | Keep kind deletion; move Reader registration/default into extracted owning children. |
| `spec-chirho/workflows-chirho/compiler-pipeline-chirho/builtin-class-identity-chirho.md` | `a81b0606ac507a7812b4a0cc3a3838191c77f406` | `26ab3bbc8df3aeca755671100da3725f9093ec2e` | `6801e43f69f40114b7f0b6eef8c79f96381069a2` | Kind identity/schema/elaboration DAG retained; Reader per-module declared-default authority edge added. |

## Auto-merged API and metadata adaptations

These were not textual conflicts. No type inference implementation region is edited: all files under `crates/haskelujah-typing-chirho/src/infer_chirho` retain the kind parent's blobs, including the leased module, schemes, deferred-defaulting and tracking-context regions.

| Path | Resolved blob | Adaptation |
| --- | --- | --- |
| `crates/haskelujah-core-chirho/src/dict_chirho/instance_chirho/defaults_chirho/monad_defaults_chirho.rs` | `64ffabe0530e9ad883e6f03b719d72a30162373f` | Arena class views; authority checks and evaluation order retained. |
| `crates/haskelujah-core-chirho/src/dict_chirho/instance_chirho/defaults_chirho/monad_defaults_tests_chirho.rs` | `39cbe42e3ee05d78dc4e4c029697b8a17578ba17` | Arena views, authentic legacy factories, owned declaration clone; preserve every assertion. |
| `crates/haskelujah-core-chirho/src/dict_chirho/instance_chirho/generic_heads_tests_chirho.rs` | `6acd1924533a5b8f5f397c3d6be4cc34f2d089db` | Arena views, legacy declaration/predicate factories and quantified-variable conversion. |
| `crates/haskelujah-driver-chirho/src/frontend_chirho/module_search_chirho/family_discovery_chirho.rs` | `623ed178459f7846377dac6fc43279823c55c91b` | Return existing reached set after final atomic publication, minus checked root; no new registry. |
| `crates/haskelujah-driver-chirho/src/frontend_chirho/module_search_chirho/boot_contract_tests_chirho.rs` | `9281e33d241856db4a7d66a9c3982cd95168fde3` | Frontend-only helper maps returned metadata to unit; original assertions retained. |
| `crates/haskelujah-driver-chirho/src/tests_chirho/evidence_chirho/concrete_heads_chirho.rs` | `4aa66858a446c67314137d5bf4f70bbd7ec47920` | Arena/factory migration; retain assertions; add unexecuted complete/inconsistent capture contrast. |
| `crates/haskelujah-driver-chirho/src/tests_chirho/evidence_chirho/occurrence_provenance_join_chirho.rs` | `e21963cea604c3ea22e46d65c4b1e157d6783a68` | Pass default declaration inventory to two auto-merged calls; preserve original evidence assertions. |
| `crates/haskelujah-typing-chirho/src/class_chirho/mod.rs` | `ec6d915c75c20896063e814eca76152f689cb69e` | Re-export existing canonical Monad default builder, required by Reader Core unit tests. |
| `crates/haskelujah-typing-chirho/src/class_chirho/standard_chirho/mod.rs` | `01e26a1a5946d6a5210232c871b416a4b5f985c8` | Register/re-export Reader existing monad_defaults child; directory becomes 8 entries. |
| `crates/haskelujah-typing-chirho/src/class_chirho/standard_chirho/higher_kinded_classes_chirho.rs` | `8ee5104ddefda05b63571904a1708744347b6547` | Seeded Monad owns its existing canonical >> arms; every other seed field retained. |
| `spec-chirho/workflows-chirho/compiler-pipeline-chirho/module-search-authority-chirho.md` | `80b2b72b6217ecdb1371f13c5a343b201bd6745d` | Document exact successful admission return and supplier transport. |


## Retained authority contracts

- D/K: a fresh context starts with an empty builtin scope. The backend clones this module's naming-derived `BuiltinClassScopeChirho` into `DictPassEvidenceChirho.builtin_classes_chirho`; the structured entry moves it into its own context. The compatibility wrapper clones its explicit last argument into the same carrier. The declared Monad default needs the BARE K01 identity, its own exact structural arms, the supported schema checked last, and this instance's body-backed donor checked first. No missing-method stub or namesake donates; no module inherits another module's admission. Kind arena entries retain each class's own defaults.
- E: method occurrences keep kind's `JoinedProofChirho` with ground predicates, applied types and existing consensus suppression. Reader concrete admission retains origin, declaring class/method, exact local data/newtype head, exact applied type, and unique context-free instance matching. A present `ground_pred_chirho` must agree with the record's class and applied type and is passed COMPLETE to matching, retaining proof, hidden kind prefix and extra types. A conflicting present capture refuses without fallback. With no structural capture, Reader's original legacy class/key admission remains; its temporary matching predicate never enters `JoinedProofChirho.ground_preds_chirho` and grants no S1 structural support. An unexecuted contrast covers matching full arguments and inconsistent class/type/prefix/extra captures.
- G: Reader superclass matching/provider logic retains its shared 32-step Cell budget, one context-free generic provider, no GND provider and pass-1 `existing_id_named_chirho` lookup. Four inherited diagnostic strings retain Reader's wording. Kind's complete superclass predicate/telescope transport and arena instance records remain. No Core RD3/RD4 refusal is added.
- S: kind's existing `DiscoveredSourceModuleChirho` stores AST/path/syntax/interface errors, so membership alone is not authority. `reachable_chirho` traverses retained AST imports through `selects_home_source_chirho`. `register_discovered_type_families_chirho` refuses every reached syntax/interface error, strictly rebuilds each reached interface, calls `admit_home_iface_chirho`, and atomically replaces the caller's snapshots only after final publication. Its return changes from `Result<(), DiagnosticBundleChirho>` to `Result<BTreeSet<String>, DiagnosticBundleChirho>` by returning that EXISTING reached set with the checked root removed AFTER publication. No membership registry, admission decision, bound or lookup changes. `compile_source_with_search_context_chirho` consumes that return into the backend supplier input; frontend-only checking discards it. The boot-test helper explicitly maps it to unit. Test-only scanner wrappers add names at actual successful parsed-interface admission, preserving replacement and excluding recovery stubs. Checked-root override, package-qualified edges, path authority, symlink/depth/directory/file bounds and the retained kind walker are unchanged.
- Reader STG binding ownership, imported-reference exclusion, builtin-only linker and interpreter provenance offset/remap remain as auto-merged source. No source-supplier name implies linked sibling Core. Scalar/field source identity, row-698 occurrence identity and kind predicate/proof transports remain inherited.

## Open validation and retained gaps

No compilation, Clippy, unit/integration test, corpus, GHC reference, native, installed CLI or RTS claim is made by this assembly. Combined workspace/ordinary preservation and both corpus axes remain pending their owner's scheduler. The new contrast is written, not run.

RD3/RD4 remain RED typing-obligation false accepts. LLVM boxed-Show remains RED. The inherited five eval-basic HeapPtr failures, unattributed eval-advanced numeric failures, missing `groupWith` body and recorded mandatory Chirho lint policy conflict remain open at their original scopes. No expectation, Core refusal, runtime measurement, MR repair, package publication behavior, prototype activation or lookup fallback is added here. Reader's held v5/v6 and NOT-GREEN v8 receipts stay historical and unchanged.
