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
- [x] After builder release, remove formatter-only drift, run the touched-crate tests, and rebuild the CLI without compiler warnings.
- [x] Rebuild the compiler, run the complete 938-file accept subset gate and 767-file reject gate twice, and inspect every delta at source level.
- [x] Update both committed measurement artifacts from the byte-identical two-pass sets, preserving their `# QUOTE-AS:` contract.
- [x] Log progress row 474 after the DB lock is released and commit only named owned paths. The Metropoliluya landing report is the final post-push step.

## Final measured result

- `should_compile`: 876 -> 877 of 938. `T14010` and `T4355` recover for the represented kind/newtype-GADT reasons; `T16188` now fails because the repaired data-instance boundary no longer swallows its following declaration. The old pass was not evidence.
- `should_fail`: 206 -> 218 of 767. Eighteen files become rejections and six spurious name-error rejections become accepts, for net +12. The companion artifact classifies the eight exact namespace gains, ten right-verdict/incomplete-reason gains, and six exposed downstream rule gaps individually.
- Both axes completed twice against the fresh debug CLI at parallelism four with zero timeouts; the two failing/accepted sets are byte-identical.
- Naming: 132/132. Typing: 314 passed, one ignored. Parser: 314 passed, two known fixture/environment failures and three ignored; both failures reproduce at untouched `dd6d694a`.
- Driver library: 1757 passed, exactly four recorded pre-existing checker/runtime failures, and the filed Cranelift custom-list runaway skipped. Dictionary-evidence 7/7, given-equality 10/10, rigid-variable 14/14, and typing integration 8/8 passed.
- Curated executable corpus: 537/537. Cranelift integration: 42/42 after excluding its separately filed recursive-IO test.
- The final CLI build completed without Rust compiler warnings. The Cranelift execution test still exposes the existing macOS linker warning about object-file platform load commands; this lane does not alter backend object emission.

## Measured AST blockers

The fresh-binary regression probe recovered 16 of the 19 initially exposed accept-axis files through faithful parser/interface fixes. Three keep their pre-lane accept verdict only because the scope pass deliberately stands down at an AST boundary it cannot represent:

- `T13915a`: `data instance T Int = MkT` is dropped, so the imported constructor `MkT` cannot enter an interface.
- `T16141`: `newtype instance` is dropped and its deriving clause is fabricated as class `Unknown`.
- `T22141g`: `type data Letter = A | B | C` is fabricated as the alias `Letter = A`; `B` and `C` disappear.

The room's compiler experiment established that a new `DeclChirho` sibling is caught by only one exhaustive match, while an added field is ignored by most `..` patterns. These forms therefore need a representation chosen on meaning plus an explicit consumer sweep—not namespace metadata, filename guards, or invented exports. Their current green `check` verdicts are compatibility holds, not claims that the constructs work; the representation decision remains with L.J.

`T16188` proves why the boundary must preserve the rest of the module even while the construct itself is deferred: the old data-instance skip consumed a later `%&&` declaration and returned green without checking it. `eat_until_unrepresented_instance_end_chirho` now stops at the instance's own layout boundary, so the sibling is checked and the file fails honestly until the data-family-instance shape exists.

## Root-cause additions discovered during the lane

- Mixed explicit/implicit layout now unwinds nested implicit contexts before an explicit `}`, restoring the enclosing module's declaration separator. This repairs scope at token normalization rather than adding declaration-specific recovery.
- Data/newtype GADT constructor blocks share one parser path, and newtype standalone kind signatures keep their full source type rather than fabricating head binders.
- Flat standalone kind signatures use the shared type reconstruction path, including symbolic operators; required foralls contribute their body kind.
- Imported closed Boolean families (`If`, `Not`, `&&`, `||`) are registered as families and reduce only when their leading argument selects an equation; otherwise they remain stuck.
- Qualified class instances expose an associated-family member unqualified only when that member is genuinely visible through the parent class import. The `GHC.Exts.IsList` / `Item` project regression fixed the interface relation rather than globally leaking `Item`.
- Driver fixtures that used explicit or qualified import lists now import every type they actually reference, so the tests remain valid Haskell under real namespace enforcement.

## Explored and reverted: inferred forall specificity

An end-to-end probe represented `forall {a}` binders in the AST and carried their inferred specificity into scheme instantiation. It recovered `ExplicitSpecificityA1`, but making the parser honest also exposed separate family-solver gaps in `T15079` and `T19535`. The AST, parser, and inference experiment was fully reverted rather than landing a parser-only half-fix or widening this naming lane into family solving. The durable design note from the probe is that specificity belongs to each quantified scheme—not the globally reused numeric type-variable id—and visible type application must skip inferred scheme variables.
