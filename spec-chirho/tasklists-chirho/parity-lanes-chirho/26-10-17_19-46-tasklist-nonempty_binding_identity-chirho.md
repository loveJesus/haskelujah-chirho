<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# NonEmpty binding identity and representation Chirho

Owner: HASKELUJAH/claude2_chirho is the source writer; GPT owns integration and canonical main/DB. Claude2 holds the builder only for the separate case gate under #27430/#27431; this unit remains source-only. This is the bounded unit assigned in #27401 from design #27400 and staged by #27427/#27428. Preserve case 77b91eb3 and pragma 95ec65db; the NonEmpty branch starts from off-main pattern checkpoint 4cfb9584. No compiler source from this unit is on main.

Problem: real constructor-operator values now reach Data.List.NonEmpty names that incorrectly fall through to same-named global identities/list functions. toList can return a real NonEmpty value unchanged and finish with a wrong answer. fromList's old identity representation made existing toList/head/length examples correct by accident, so replacing one function alone loses valid behavior. Naming currently does not resolve value references; occurrence origins identify uses but are not connected to bindings. A raw spelling replacement would intercept local shadows.

## Placement and contract

- The naming import producer applies its existing import-list/hiding/alias rules and records exposures. Table entries relevant to the stage's owned NonEmpty exports distinguish one defining origin from ambiguity, counting ALL imports exposing each spelling in either order. Two exposures of the same origin remain unique. Do not invent omitted names or a full qualifier when an alias replaced it. One shared constant owns the actual set: toList, head and tail for stage B.
- Remove bare entries shadowed by any module value in the final naming environment, including accessors, class methods and pattern-bound values. A shared exposure data type lives below the driver/desugar consumers and naming, with no typing-to-naming dependency. Exact qualified scope lookup stays first in desugar; bare local names cannot capture a consumed qualified import.
- After every import has merged, the driver consumes only a Unique owned QUALIFIED spelling whose final merged scheme passes the existing placeholder predicate. Forward that exact filtered table to desugar. Typing keeps its current exact-qualified placeholder lookup, predicates and wanted judgment. Bare, ambiguous and non-placeholder spellings retain today's route. No general value-resolution walk or bare string rewrite.
- Stage B owns dual-shape toList, head and tail, with existing list arms first and :| arms appended, plus the necessary :| arms in length, mapM_, forM_ and sequence_. Keep fromList, map and nonEmpty on their current type/representation routes. The original six-function representation change is deferred until coherent Foldable typing can preserve the working programs below. No global Prelude or IsList override and no six-function completeness claim.
- #27432/#27433 settle the typing/evidence route with no new schemes: the driver placeholder is forall a. a with one matching variable numbered at least 9000 and no predicates. The exact qualified typing lookup finds it and empty-predicate capture records no class-method evidence. Preserve this current route rather than adding NonEmpty-only, unconstrained or Foldable schemes. No new union-type or solver mechanism. Any InferResult transport-field edit must be described as metadata, not byte-identical typing source.
- Placement #27420/#27432/#27433: shared AST table/tests, focused naming producer, driver filter/carrier, desugar reference consumer and focused Core prelude module. No new typing module is needed. Leave pure move 7a87f118 out of this unit and preserve it on its existing branch; the directory-cap adjustment was conditional on adding the now-unneeded typing module. No typing-to-naming dependency and no new Foldable seed.
- Keep canonical module/export identity in the table, but route consumed origins to private Core implementation bindings whose final names do not collide with class methods. A module-qualified binding ending in toList is insufficient: the dictionary pass recognizes class methods by qualified-name suffix even without captured predicates. No general dictionary rewrite or global IsList change belongs to this unit.
- Preserve length and existing G4 programs. Audit all list-only/default consumers before changing representation. Newly reachable wrong-success output blocks, even if other consumers fail loudly. No Map repair, generic dictionary functions, Show renderer or character work.

## Checklist

- [x] Re-read the qualified binding/design routes and independently confirm absence of a resolved value-reference map.
- [x] Assign one source writer, preserve earlier branches and record builder/main/DB ownership.
- [x] Surface physical module homes/dependency direction before source edits; preserve the conversion-move branch and omit the unneeded move from this stage.
- [x] Audit default/wildcard Foldable paths and measure working fromList programs before a representation change.
- [x] Publish and review the stage-B placeholder-only typing/evidence design; no typing seeds are added.
- [ ] Implement exposure producer/handoff, both priority-aware consumers and coherent dual-shape bindings/length/traversal behavior within that design.
- [ ] Publish source checkpoint with exact GHC/main controls; no candidate behavior claims before the named build.
- [ ] Gate aliases, selective/hiding imports, both competitor orders, duplicate-origin imports and lambda/let/where/module shadows. Preserve ordinary list/IsList and G4 behavior.
- [ ] Named builder gate, meaningful mutations, paired corpus/replay/execution and final combined-source checks.
- [ ] Review source/proof at actual scope, record a local landing decision and close the ledger unit only on its declared evidence. No push/release.

Evidence/design roots in Claude2's worktree: tmp-chirho/output-gaps-1017-chirho/ and tmp-chirho/nonempty-identity-1017-chirho/. Earlier invalid-scope acceptances I2/I3/I6/I8 remain baseline evidence, not desired accepted behavior or proof of correct resolution. Case is active under Claude2's named take #27431, then ground; NonEmpty has no Cargo lease.

## Measured representation traps and staged disposition

Claude2 #27427 measures sum, elem, maximum, foldr, any and concatMap on NE.fromList as GHC-exact on frozen main. GPT #27425 independently measures mapM_, forM_ and sequence_ on NE.fromList: all three print one then two on separate lines on GHC, main and 4cfb9584. A real fromList result with the current list-only schemes would regress these valid programs. #27428 chooses staged B, preserving their routes until a coherent Foldable-typing unit. Required composition controls include NE.head (NE.map (+1) (NE.fromList [1,2])) and NE.toList of that same NE.map expression.

#27431 corrects #27428's assumed negative baseline: NE.head and NE.tail currently check even on Just; the measured head use fails loudly at runtime, while NE.toList (Just 1) wrongly succeeds with Just 1. Record the actual baseline instead of claiming these are current check rejections. GHC-negative container inputs remain invalid-language controls; stage B must not turn a current loud miss into a wrong success, and its ordinary-export binding must not silently return a non-list value. #27432/#27433 settle the design checkpoint through the existing placeholder, not new seeds; candidate outputs remain unmeasured.

NonEmpty branch checkpoint c9283dac preserves pure move 7a87f118 and the measured stage-B tasklist with sixteen probes. It is source-only and explicitly not ready. Its source writer switches to the separate case gate only after preserving that checkpoint.

GPT's nine-command baseline receipt, with source and before/after binary identities:

/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-chirho/tmp-chirho/known-type-patterns-1017-chirho/gate-chirho/field-ownership-chirho/nonempty-foldable-baseline-chirho/receipt-chirho.json

Preserved boundaries for this stage include fromList [] returning [], fromList-built values not matching :|, the full Foldable representation transition and the unmeasured WebAssembly constructor-tag collision. Appending :| after existing list alternatives preserves their priority; it does not certify WebAssembly NonEmpty behavior.

## Placeholder-only seam disposition, 20:28 update

Claude2 #27432 supplies the source route and additional measured reductions. GPT reads the pinned 4cfb9584 driver predicate/Phase 3.9, exact qualified typing lookup, empty-predicate capture, and the Core name-dispatch helpers; this is a source review, not a fresh runtime rerun. B22/B23 preserve valid head/toList after NE.reverse, while the typed NonEmpty wrapper for B22 is rejected by the current list reverse route. B20's final imported scheme is IsList under a non-qualified import with an alias, so the filtered consumer excludes it; its measured wrong output remains a material named boundary. Test both import orders because filtering happens after the full merge. Existing precise schemes must not be replaced or canonicalized.

GPT #27433 accepts source work within this narrower boundary and omits the unneeded conversion move. It also identifies a separate downstream name seam: class_method_selector_for_name_chirho strips the module prefix before checking method_selectors_chirho, while the top-level ordinary-shadow guard checks exact method names. A canonical Data.List.NonEmpty.toList binding can therefore still be classified as IsList. The private implementation binding route avoids that suffix collision. Required controls include first-class uses and a valid ambient IsList function that calls NE.toList on an independently constructed :| value; measure its GHC oracle before claiming candidate behavior. No feature source or candidate output is approved merely by this design disposition.

Case keeps Claude2's named builder reservation, then ground. NonEmpty remains source-only and not ready until its exact source checkpoint and separately named gate. The whole NonEmpty integration dependency is not closed by a qualified-placeholder-only repair.
