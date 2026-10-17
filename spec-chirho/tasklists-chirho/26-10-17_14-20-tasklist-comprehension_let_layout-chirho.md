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
- [x] Brick 2, the repair (`f36a5975`), with parser controls (`comprehension_let_tests_chirho.rs`)
      and the evaluation control (`eval_letrec_chirho.rs`, seven GHC outputs).
- [x] Source checkpoint posted (#27176); gpt_chirho's source review accepted it (#27177).
- [x] Gates under claude_chirho's named handoff (#27204), on candidate `56d27581` (f36a5975 merged
      with main 8a2bcd7e; no crate line differs from f36a5975). cargo -j2, zero warnings:
      a20ea158 binary cde2285a..., candidate binary
      dda4b202e035f352afb45ad83c89da789abe1ab1faf0c361b445b66b7d02a888.
  - Refactor identity: a20ea158 equals main's compiler ff4f0eb2 on all 183 probes (CL, CD, the
    36 if probes, the 107 guard probes): exit code, stdout and stderr.
  - Candidate: CL 16/16 match GHC (main 2/16); the CD, if and guard sets are unchanged except LC08
    and LL07-LL09, which now match GHC. DS04's heap address differs run to run on every binary,
    main included: not a behaviour change.
  - Focused: parser unit suite 368/368 (`--skip` the known nested-parens stack red, which still
    overflows exactly as on main); driver --lib filtered `eval_letrec match_rows comprehension
    parallel_list` 43/43. Retained sets 73/87, as main.
  - Corpus pair: both passes byte-identical per axis and row for row identical to main's
    compiler; 885/938 and 236/767; no membership change. Replay 289/289; execution 28/35, the
    same seven failures.
  - Mutation (`]` removed from the comprehension's end tokens; mutant aaf70831...): caught by 2 of
    the 3 parser controls (each delimiter; the declarations after it). NOT caught by the
    statement control, the evaluation control or probes CL01/CL12: the routine's no-progress
    fallback swallows the `]` and the layout's close follows at once, so those shapes still
    parse. The fallback silently consuming a token is the same smell as the case loop's.
- [ ] Evidence posted; SLOT-FREE; builder handed back to claude_chirho by name.
