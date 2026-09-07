<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Type namespace and lexical type-variable resolution

Directive: advance the GHC typecheck corpus through reusable root-cause mechanisms rather than corpus-specific guards.

## Brick 1 — architecture and ownership

- [x] Keep this lane in `haskelujah-naming-chirho`: the pass already owns `NameEnvChirho`, imported interfaces, namespaces, and undefined-name diagnostics.
- [x] Add a focused `type_scope_chirho.rs` module because `resolve_chirho.rs` already exceeds the project file-size limit.
- [x] Keep its unit tests in `type_scope_tests_chirho.rs` so the implementation stays below 1,000 lines and the naming directory remains within its entry budget.
- [x] Begin with the naming module and its narrow resolver/lib hooks; expand only at a measured trust-boundary failure after the other agents' typing/kind lanes land.
- [x] Isolate this lane on branch `gpt-type-scope-chirho` in a sibling worktree so Claude's corpus binaries contain only his typing lane.
- [x] Respect Claude's builder and progress-DB locks: no Cargo build/test/corpus gate and no SQLite write until `SLOT-FREE` / `DB-FREE`.
- [x] Put interface export extraction in `type_exports_chirho.rs` and class-member lowering in `type_member_lowering_chirho.rs`, rather than adding more responsibility to the 18K-line interface/lowering files. Something is wrong with both giant files; these extractions are the bounded structural repair owned by this lane, not a claim that either split is complete.

## Brick 2 — source type-scope walker

- [x] Resolve type constructors and classes against wired-in syntax or the populated local/imported `NameEnvChirho`, including qualified imports.
- [x] Track lexical type-variable binders through declaration parameters and explicit `forall` scopes.
- [x] Diagnose free type variables where Haskell does not implicitly quantify them: type-synonym right-hand sides, outermost invisible-forall signatures, and positional/reliably-lowered data fields.
- [x] Preserve legitimate implicit quantification in signatures and GADT signatures without an outermost invisible forall, existential forms, PolyKinds declaration binders, instance heads, and family equations until kind-annotated family-pattern scopes are represented completely.
- [x] Defer only invisible-forall/quantified record fields whose current flat parser path can manufacture the wrong scope; continue checking ordinary and required-forall fields.
- [x] Keep the successful-resolution path linear in the AST with hash-backed scope lookup and no filesystem or module scanning; keep typo suggestions on the diagnostic-only path over the already-populated environment.
- [x] Canonicalize operator/type spellings at the interface boundary and retain associated-family parent relationships through selected imports, hiding, explicit exports, and module re-exports.
- [x] Preserve parser shapes the scope pass needs: data-family headers, associated type/data members, operator class/family names, explicit empty family bodies, datatype-context heads, top-level equality constraints, nested rank-N constructor fields, and `TYPE representation` kind applications.
- [x] Bound the two known lossy-AST cases rather than guessing: rank-N record fields and existential constructor prefixes defer only the checks their lowering cannot faithfully represent. Record the latter in `bug-existential-constructor-scope-dropped-chirho.md`.

## Brick 3 — behavioral proof

- [x] Add focused naming-crate tests for local, imported, qualified, shadowed, explicitly quantified, free-variable, unknown-tycon, and unknown-class behavior.
- [x] Add interface tests proving associated types survive all/selected exports, hiding, module re-exports, and both unqualified and qualified-alias resolution.
- [x] Add focused parser tests whose assertions depend on each recovered AST shape surviving.
- [x] Add/adjust the name-resolution workflow DAG and comment participating entry points in code.
- [ ] After builder release, run formatting and the touched-crate tests with zero warnings.
- [ ] Rebuild the compiler, run the complete 938-file accept subset gate and 767-file reject gate, and inspect every delta at source level.
- [ ] Update both committed measurement artifacts only if the full two-pass measurements change, preserving their `# QUOTE-AS:` contract.
- [ ] Log one progress row after the DB lock is released, commit only named owned paths, and report the landed result through Metropoliluya.

## Measured AST blockers

The fresh-binary regression probe now fixes 16 of the 19 originally new accept-axis failures. Three remain intentionally unguarded because lowering cannot represent what their source declares:

- `T13915a`: `data instance T Int = MkT` is dropped, so the imported constructor `MkT` cannot enter an interface.
- `T16141`: `newtype instance` is dropped and its deriving clause is fabricated as class `Unknown`.
- `T22141g`: `type data Letter = A | B | C` is fabricated as the alias `Letter = A`; `B` and `C` disappear.

The room's compiler experiment established that a new `DeclChirho` sibling is caught by only one exhaustive match, while an added field is ignored by most `..` patterns. These forms therefore need a representation chosen on meaning plus an explicit consumer sweep—not namespace metadata, filename guards, or invented exports. This lane will not land a three-file regression or bless the fabricated AST; the representation decision remains with L.J.

## Explored and reverted: inferred forall specificity

An end-to-end probe represented `forall {a}` binders in the AST and carried their inferred specificity into scheme instantiation. It recovered `ExplicitSpecificityA1`, but making the parser honest also exposed separate family-solver gaps in `T15079` and `T19535`. The AST, parser, and inference experiment was fully reverted rather than landing a parser-only half-fix or widening this naming lane into family solving. The durable design note from the probe is that specificity belongs to each quantified scheme—not the globally reused numeric type-variable id—and visible type application must skip inferred scheme variables.
