<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Type namespace and lexical type-variable resolution

Directive: advance the GHC typecheck corpus through reusable root-cause mechanisms rather than corpus-specific guards.

## Brick 1 — architecture and ownership

- [x] Keep this lane in `haskelujah-naming-chirho`: the pass already owns `NameEnvChirho`, imported interfaces, namespaces, and undefined-name diagnostics.
- [x] Add a focused `type_scope_chirho.rs` module because `resolve_chirho.rs` already exceeds the project file-size limit.
- [x] Keep its unit tests in `type_scope_tests_chirho.rs` so the implementation stays below 1,000 lines and the naming directory remains within its entry budget.
- [x] Reserve only the naming module, its narrow resolver/lib hooks, this tasklist, and the corresponding workflow document; do not touch Claude's inference/kind/parser lanes.
- [x] Isolate this lane on branch `gpt-type-scope-chirho` in a sibling worktree so Claude's corpus binaries contain only his typing lane.
- [x] Respect Claude's builder and progress-DB locks: no Cargo build/test/corpus gate and no SQLite write until `SLOT-FREE` / `DB-FREE`.

## Brick 2 — source type-scope walker

- [ ] Resolve type constructors and classes against wired-in syntax or the populated local/imported `NameEnvChirho`, including qualified imports.
- [ ] Track lexical type-variable binders through declaration parameters and explicit `forall` scopes.
- [ ] Diagnose free type variables where Haskell does not implicitly quantify them: type-synonym right-hand sides, outermost invisible-forall signatures, and positional/reliably-lowered data fields.
- [ ] Preserve legitimate implicit quantification in signatures and GADT signatures without an outermost invisible forall, existential forms, PolyKinds declaration binders, instance heads, and family equations until kind-annotated family-pattern scopes are represented completely.
- [ ] Defer only invisible-forall/quantified record fields whose current flat parser path can manufacture the wrong scope; continue checking ordinary and required-forall fields.
- [ ] Keep the successful-resolution path linear in the AST with hash-backed scope lookup and no filesystem or module scanning; keep typo suggestions on the diagnostic-only path over the already-populated environment.

## Brick 3 — behavioral proof

- [ ] Add focused naming-crate tests for local, imported, qualified, shadowed, explicitly quantified, free-variable, unknown-tycon, and unknown-class behavior.
- [ ] Add/adjust the name-resolution workflow DAG and comment participating entry points in code.
- [ ] After builder release, run formatting and the touched-crate tests with zero warnings.
- [ ] Rebuild the compiler, run the complete 938-file accept subset gate and 767-file reject gate, and inspect every delta at source level.
- [ ] Update both committed measurement artifacts only if the full two-pass measurements change, preserving their `# QUOTE-AS:` contract.
- [ ] Log one progress row after the DB lock is released, commit only named owned paths, and report the landed result through Metropoliluya.
