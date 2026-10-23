<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Own-class method occurrences workflow

A method of a class the program itself declares gets one occurrence id per reference, so the
checker's proof for that one use reaches the dictionary pass. Before, every use of such a method
shared one free-variable id, and the pass guessed its dictionary from sibling arguments:
`map neededChirho [[1 :: Int]]` ran `NeedsChirho Int`'s method on each list. Trace 2
(2026-10-23, frozen base 926ccd4b and Ord dea2a0b7 CLIs, GHC 9.14.1) measured `[[1]]` where
GHC fails with "No instance nor default method", `[[1]]` where GHC prints `[99]`, and a crash
where GHC prints `[1005]`. The proof already existed: the checker captures every constrained
reference, with no name filter. Only an occurrence to carry it was missing.

This document stands beside [the dictionary evidence workflow](dictionary-evidence-chirho.md),
which describes the join and the pass. That file is one of the Ord unit's preserved paths, so
this unit leaves it unchanged (gpt_chirho #28092).

```mermaid
flowchart TD
    module_chirho["Lowered module: each DeclChirho::ClassDeclChirho with one type variable"]
    module_chirho --> names_chirho["method_occurrence_names_chirho in evidence_join_chirho.rs: the standard set plus those classes' methods, minus the INV-001 names"]
    names_chirho --> inputs_chirho["compile_backend_chirho: DesugarInputsChirho.method_names_chirho, a thin call"]
    inputs_chirho --> resolve_chirho["resolve_var_chirho: a name in scope is that binding; an opted-in free name gets a fresh occurrence id beside its canonical id"]
    resolve_chirho --> site_chirho["Variable site: the occurrence's span and provenance are recorded"]
    capture_chirho["Checker: capture_occurrence_predicates_chirho for every constrained reference"] --> records_chirho["finalize_occurrence_records_chirho: NeedsChirho at [Int] for the map use"]
    site_chirho --> join_chirho["join_occurrence_evidence_chirho: by provenance, then by span; a record counts only if its class declares the name"]
    records_chirho --> join_chirho
    join_chirho --> self_chirho["Head-only proof whose caller key names a local self dictionary: the class selector applied to it"]
    self_chirho --> row_chirho["Else the row named $prim_C_m_key: as it is, or with its instance's ground context dictionaries applied first"]
    row_chirho --> keyed_chirho["Else the keyed selector with that key's dictionary; a foreign proof never reaches the context dictionary"]
    keyed_chirho --> restore_chirho["Unresolved, or no proof: the canonical id and the caller's own route"]
```

## Rule (gpt_chirho #28091)

- Opted in: the standard set (`EVIDENCE_METHOD_NAMES_CHIRHO`) and every method of a
  single-parameter class the compiling module declares.
- Never opted in: `>>=`, `>>`, `return`, `pure` and `fail` (INV-001), even when the module's own
  class declares one. They keep their dedicated dispatch machinery.

## Boundaries

Each of these keeps its shared id and today's verdict:

- A seeded standard class's methods outside the standard set (`fmap`, `<>`, `toList`,
  `traverse`). Switching about forty seeded classes at once is its own measured brick.
- A class imported from another module. Its rows live in that module's Core, and finding them
  from the importing module is unmeasured.
- A multi-parameter class.
- The spelling of an operator method (`(<+>)` or `<+>`) is not pinned: the unit test checks
  alphanumeric methods only.

## Shadows

- A lambda, case, let or where binder is found in scope before any occurrence is minted
  (`resolve_var_chirho`), and the dictionary pass skips local shadow ids.
- Top-level functions and record fields are bound in scope before any body (desugar Pass 1 and
  1b). In one module, neither can share a name with a method of that module's own class.
- A qualified reference resolves by its text unless the qualified name itself is bound, and the
  checker records the same text.

## Dependency on Ord brick 2

A proven occurrence calls the row named for its key directly. Before Ord brick 2 (f93ca45f), the
conditional list stub under `$prim_C_m_[T]` took leading `$Dict_Missing` parameters that only
the dictionary applied, so a caller that found the row by name bound its first argument as the
dictionary. On main alone this unit would turn cd95568b's direct missing-method control into a
partial application. The unit therefore sits on the exact Ord replay dea2a0b7 (gpt_chirho
#28096), and the combined gate is its only landing authority.

## Where a proof lands (gpt_chirho #28111)

The first combined gate (combined-03a337d4-chirho) measured two regressions this unit made. Both
come from proofs the old route never had. The rules live in
`dict_chirho/rewrite_chirho/occurrence_rows_chirho.rs`, and both dispatch entries call them
(`occurrence_head_replacement_chirho`, `try_rewrite_method_var_chirho`).

- Self agreement. Inside `instance DescribableChirho a => DescribableChirho [a]`, the checker
  proves `describeChirho xs` only as `[]`, a list over a variable. The instance's self
  dictionary is registered under `[a]`. The proof is served by that dictionary when the
  caller's key (`[a]`, read off `xs`) has the proof as its head and names a dictionary this
  binding holds. Never a global `[]` alias: a `[b]` or ground occurrence in the same body is
  not a local dictionary here.
- No ambient answer. A proof that is not the binding's own dictionary key is never served by
  the context dictionary. Before, it was, and Int's method ran on the tail: `1:[2,3]` on the
  interpreter, a pointer on LLVM and Cranelift. Unresolved, the occurrence returns to its
  canonical method and the caller's own route.
- Context-leading rows. A `$prim` row whose instance has a context takes those dictionaries
  first (`\$dShow -> \_ -> 7`). The rows are recorded from the class environment before any
  binding is rewritten, by the same rule that gives them their parameters. A proof is served
  by such a row only with every context dictionary applied (each predicate one-parameter,
  ground and registered); otherwise the keyed route decides. The rows the pass generates for
  ground keys (`$prim_C_m_[Int]`) take no dictionary.

Self agreement and no ambient answer serve the same recursive calls: the restored route finds
the self dictionary through the same `[a]` key. Each masks the other in every behavioral
control, so one terminating mutant reverts both together (gpt_chirho #28112). Measured
boundaries, before and after: a recursive `TreeChirho a` instance fails loudly on main too
("missing STG binding"), and a bare self occurrence (`goChirho = describeChirho`) is wrong on
main too. Neither is claimed here.

## Controls

- `tests/dictionary_evidence_chirho/higher_order_occurrences_chirho.rs`:
  - GHC-exact programs: the defined list method under `map`, two types in one program, the
    element method inside an instance, a constrained binding's own dictionary, the direct call,
    and the self-qualified `Main.neededChirho`;
  - missing-method programs that must fail by name: a constant element method, the eta form, and
    a context-free `[a]` head;
  - four lexical shadows: lambda, let, where and case binders.
- `src/tests_chirho/evidence_chirho/occurrence_names_chirho.rs`: the name set's boundaries.
- Recursive calls on the interpreter (`1:2:3:[]`, direct and through a `where`), plus the
  LLVM and Cranelift recursive custom-list round trips in `compile_chirho.rs`. An occurrence
  at `[Bool]` inside the `[a]` instance (`1:2:[]bb`). Context-leading rows: one predicate,
  under `map`, and two predicates (`7`, `[7,7]`, `2`).
- `occurrence_rows_tests_chirho.rs`: agreement, context-leading rows and their record.
- Mutant o1 drops only the opt-in (`.chain(own_methods_chirho.take(0))`). The higher-order
  controls must catch it.
- Mutant o2 reverts self agreement and no ambient answer together, and the recursive and
  agreement rows catch it. Mutant o4 calls a context-leading row bare, and the
  context-leading rows catch it.
