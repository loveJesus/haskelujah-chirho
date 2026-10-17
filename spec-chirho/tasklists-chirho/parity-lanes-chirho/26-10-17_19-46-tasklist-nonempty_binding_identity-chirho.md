<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# NonEmpty binding identity and representation Chirho

Owner: HASKELUJAH/claude2_chirho is the source writer; GPT owns integration, canonical main/DB and the current builder. This is the bounded unit assigned in #27401 from design #27400, not permission for concurrent Cargo. Preserve case 77b91eb3 and pragma 95ec65db; new source branches from off-main pattern checkpoint 4cfb9584. No compiler source from this unit is on main.

Problem: real constructor-operator values now reach Data.List.NonEmpty names that incorrectly fall through to same-named global identities/list functions. toList can return a real NonEmpty value unchanged and finish with a wrong answer. fromList's old identity representation made existing toList/head/length examples correct by accident, so replacing one function alone loses valid behavior. Naming currently does not resolve value references; occurrence origins identify uses but are not connected to bindings. A raw spelling replacement would intercept local shadows.

## Placement and contract

- The naming import producer applies its existing import-list/hiding/alias rules and records exposures. Table entries relevant to the six NonEmpty exports distinguish one defining origin from ambiguity, counting ALL imports exposing each spelling in either order. Two exposures of the same origin remain unique. Do not invent omitted names or a full qualifier when an alias replaced it.
- Remove entries shadowed by any module value in the final naming environment, including accessors, class methods and pattern-bound values. Local scope lookup precedes canonical exposure lookup in typing and desugar. A shared exposure data type lives below both consumers and naming, with no typing-to-naming dependency.
- Pass the producer's one table through the driver to both consumers. No general value-resolution walk or bare string rewrite. Absent and ambiguous exposures retain their stated current support boundary.
- Six canonical bindings and matching schemes: toList, fromList (error on empty), head, tail, map and nonEmpty, all over the real :| representation. No global Prelude or IsList override. Put implementations and exposure producer in focused modules; oversized driver/infer/prelude files get dispatch/registration only.
- Preserve length and existing G4 programs with real NonEmpty representation. Before enabling the required Foldable seed, enumerate affected list-only consumers and every wildcard/default branch. Newly reachable wrong-success output blocks, even if other consumers fail loudly. No Map repair, generic dictionary functions, Show renderer or character work.

## Checklist

- [x] Re-read the qualified binding/design routes and independently confirm absence of a resolved value-reference map.
- [x] Assign one source writer, preserve earlier branches and record builder/main/DB ownership.
- [ ] Surface physical module homes/dependency direction before source edits; continue within this contract without another approval ceremony.
- [ ] Implement exposure producer/handoff and both priority-aware consumers, six real bindings/schemes and coherent length compatibility.
- [ ] Audit default/wildcard Foldable paths and retain explicit loud boundaries.
- [ ] Publish source checkpoint with exact GHC/main controls; no candidate behavior claims before the named build.
- [ ] Gate aliases, selective/hiding imports, both competitor orders, duplicate-origin imports and lambda/let/where/module shadows. Preserve ordinary list/IsList and G4 behavior.
- [ ] Named builder gate, meaningful mutations, paired corpus/replay/execution and final combined-source checks.
- [ ] Review source/proof at actual scope, record a local landing decision and close the ledger unit only on its declared evidence. No push/release.

Evidence/design roots in Claude2's worktree: tmp-chirho/output-gaps-1017-chirho/ and tmp-chirho/nonempty-identity-1017-chirho/. Earlier invalid-scope acceptances I2/I3/I6/I8 remain baseline evidence, not desired accepted behavior or proof of correct resolution. GPT's pattern builder is still active; queued case then ground gates retain named handoffs.
