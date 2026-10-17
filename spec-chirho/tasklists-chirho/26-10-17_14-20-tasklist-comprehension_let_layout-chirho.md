<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Comprehension `let` layout (claude2_chirho)

Lane assigned by gpt_chirho in #27174 (L.J. 2026-10-17: "push dev forward"). Branch
`comprehension-let-layout-chirho` in claude2's worktree, on main `e7b0edc7`. Source checkpoint
first; Cargo only under the next named builder handoff (-j2, nice/timeout, at most two test
threads, focused gates; no full driver --lib, no ghc_bulk). No corpus gain is promised: a scan
found an implicit comprehension `let` in none of the 53 should_compile files main rejects.

## Trace: who owns the close and the separator

- The layout pass (`layout_chirho.rs`) closes an implicit block by indentation, and closes a `do`
  or `case` block opened inside parentheses at the matching `)` (it tracks paren depth only). It
  has no bracket depth, and it never closes a `let` block at a delimiter.
- So in `[y | x <- xs, let y = x + 1]` written on one line, the layout pass places the `let`
  block's virtual `}` at the start of the NEXT line, followed by that line's separator for the
  enclosing block (a do block's `;`, a `where` block's `;`, the module's `;`).
- The parser's own routine, `parse_list_comp_let_binds_chirho`, consumed the block's virtual `{`
  and stopped at the `,`, `|` or `]`, but consumed the virtual `}` only when indentation had
  already put it there (the multi-line form). On one line the `}` stayed after the `]`; the
  enclosing block's parser took it as its own close, and the enclosing separator after it then
  belonged to the block outside that one. Measured on main's compiler: the next do statement
  merged into the previous one (compile errors), or vanished while the program exited 0 (CL05:
  `putStrLn "next"` never ran); at top level the following declarations vanished ("no binding
  named 'main'"). Explicit braces avoid it; the multi-line form works.
- Guards had the same shape and already had the Report's parse-error(t) close (6g,
  `guard_let_chirho.rs`): move the block's virtual `}` to the delimiter and drop the block's
  virtual `;` after it. That routine is general; the comprehension's partial copy is not.

## Design

- Brick 1 (`a20ea158`): `guard_let_chirho.rs` becomes `let_qualifier_chirho.rs`, its loop taking
  the end tokens as a parameter; guard behaviour unchanged.
- Brick 2: a comprehension's `let` goes through the same routine with `,`, `|` and `]`; the
  comprehension's own routine is deleted (cst_parser_chirho.rs shrinks by 47 lines).
- Not in this unit, reported separately: a one-line `case` closed by `,` or `|` inside brackets
  or parentheses (S01-S04). `print (case 1 of 1 -> 0; _ -> 9, True)` prints `0` where GHC prints
  `(0,True)`, and `print [case 1 of 1 -> 0; _ -> 9, 5]` prints `[0]` where GHC prints `[0,5]`:
  silent wrong answers, the same parse-error(t) family for a `case` block, which the layout pass
  closes only at `)`. Nested multi-way-if layout also stays separate (gpt_chirho, #27174).

## Checklist

- [x] Trace, and controls CL01-CL16 on GHC 9.14.1 and main's compiler ff4f0eb2: main 2/16 (the
      explicit-brace and multi-line controls); 13 fail loudly, CL05 silently drops a statement.
- [x] Brick 1, the routine generalized and renamed (`a20ea158`).
- [ ] Brick 2, the repair, with parser controls (`comprehension_let_tests_chirho.rs`: the next
      statement, declarations, a `where` block, every delimiter, a binding's own guards,
      explicit braces) and evaluation controls (`eval_letrec_chirho.rs`, seven GHC outputs).
- [ ] Source checkpoint posted.
- [ ] Under the next builder handoff: build both bricks; brick 1 identical to main on every probe;
      the parser unit suite (with its known nested-parens stack red run separately), the guard,
      comprehension and letrec controls; CL01-CL16, the 107-probe and the retained sets; the corpus
      pair (no membership change expected); one mutation; SLOT-FREE.
