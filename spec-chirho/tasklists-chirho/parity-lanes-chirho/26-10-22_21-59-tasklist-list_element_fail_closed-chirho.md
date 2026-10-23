<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Unit 524: a failed list element fails closed (claude2_chirho)

The unit was opened by gpt_chirho's disposition of the trace, #27694, which was
approved for source:
- restore `force_addr_to_whnf_chirho`'s caller state on both outcomes;
- propagate `show_value_as_string_chirho`'s errors through its internal and
  external callers.

Branch `list-element-fail-closed-chirho` sits on main_chirho 2df9ac63, where
unit 523 has landed. It absorbs no Char, R2b or other unit, and it carries no
Cargo until a named builder slot.

**Base.** `eval_chirho.rs` is blob 892d9cea on 0f256e43, daec9fa1 and 2df9ac63:
the same blob the trace read.

## Mechanism, re-read on 2df9ac63

- **Which `print`.** Where the dictionary pass leaves `print` unresolved, it is the
  prelude's `putStrLn# (showInt# x)`. That covers the `[Float]`, `Maybe Float` and
  tuple rows, and also an `[Int]` list under `catch`.
  - Without `catch`, L1's `print` is rewritten to its dictionary and renders in
    Haskell code, which is why it was already loud.
  - Under `catch` (C2 and C8), the bare `print` stays. That is why those runs were
    silent.
- **The renderer.** `showInt#` on a heap value takes the universal-show fallback,
  `show_value_as_string_chirho`. It forces each element and tail with
  `force_addr_to_whnf_chirho`.
- **Before this unit, a failed force:**
  - returned before restoring the caller's stack and argument registers, which
    lost the pending `putStrLn#`, the rest of `main` and any catch frame;
  - rendered as `?` for an element, or cut the list for a tail.

  The run then ended silently with exit 0.

## Change: runtime `eval_chirho.rs` only

- **(a)** `force_addr_to_whnf_chirho` restores the stack and argument registers
  whether the nested run returns or fails, and only then reports the outcome.
  `apply_fn_to_value_chirho` already uses this order.
- **(b)** `show_value_as_string_chirho` returns `Result<String, EvalErrorChirho>`.
  - The element force and the tail force return the failure.
  - Its 10 recursive calls propagate it.
  - Its 6 external callers in `eval_prim_chirho` return it: `showMaybe#`,
    `showTuple2#`, `showEither#`, `showOrdering#`, `print#` and the universal-show
    fallback.
  - Two counts in the trace were wrong. It said 11 recursive calls; there are 10.
    It named line 2233 as a second `showMaybe#` site; it is `showTuple2#`.
- **The run loop is unchanged.** It already sends a primitive's runtime error to
  `maybe_catch_error_chirho`, which unwinds the current stack to a catch frame.
  With (a), that stack is the caller's own.
- There is no change to the dict pass, the Show rows or any other primitive.
- Workflow: `stg-forcing-core-chirho.md`, section "Failed Nested Forcing Fails
  Closed".

## Controls

The controls are in `crates/haskelujah-driver-chirho/tests/evaluation_values_chirho/element_failures_chirho.rs`,
a `#[path]` module of `evaluation_values_chirho.rs`. They run on the STG engine
through `eval_source_outcome_chirho`, the function behind `haskelujah run`.

**Provenance.**
- The 18 programs are the measured probes, embedded byte for byte by generation.
- GHC 9.14.1's stdout and exit code come from their receipts.
- `generate_controls_chirho.py verify` re-parses the Rust file: 18/18 programs are
  byte-identical, and each SHA-256 equals its receipt's.

**Why prefix and suffix.** Our renderer writes nothing where GHC first writes a
partial prefix. So a failing run's stdout must be a prefix of GHC's, and a handled
run's stdout a suffix of it.

**`a_failed_element_or_tail_fails_the_run_with_its_message_chirho`.** Each run must
fail with `boomChirho`.

| Probe | GHC stdout | Before, on every measured build |
|---|---|---|
| G1 `print [1.5 :: Float, error ..]` | `[1.5,` | exit 0, nothing written |
| G3 first element | `[` | exit 0, nothing |
| G5 `[[Float]]` | `[[1.5],[` | exit 0, nothing |
| G6 `Maybe Float` | `Just ` | exit 0, nothing |
| G7 `(Float, Float)` | `(1.5,` | exit 0, nothing |
| G10 failing tail `1.5 : error ..` (new) | `[1.5` | exit 0, nothing |
| C4 `putStrLn "before"` first; must keep `before\n` | `before\n[1.5,` | exit 0, `before\n` |
| S1 scalar `error` | (empty) | already loud |
| L1 `[Int]`, dictionary path | `[1,` | already loud |

**`a_catch_handler_receives_a_failed_element_chirho`.** Each run must return, and its
stdout must hold the handler's `caught\n`.

| Probe | GHC stdout | Measured before |
|---|---|---|
| C8 `[Int]` under `catch` | `[1,caught\n` | exit 0, nothing |
| C9 `[Float]` under `catch` (new) | `[1.5,caught\n` | exit 0, nothing |

**`lazy_successful_and_directly_caught_runs_match_ghc_chirho`.** Each run must
return GHC's exact stdout. These rows are unchanged: G2 (`[2.5]`), Z1-Z3 (`length`,
`take 1` and `head` of a list with an unforced error), and C5-C7 (a direct `error`,
a scalar `print` error, and no error under `catch`).

**Contract, predicted by source and not run:** every row passes after (a)+(b).

## Why C1 and C2 are not controls

C1 and C2 catch the same failures with `putStrLn ("caught " ++ show e)`. H1
measures that handler on a direct `error`, a path that already works:
- GHC prints `caught boomChirho`;
- every build prints `caught "boomChirho"`, because `show` of a caught `ErrorCall`
  goes through the universal show of a String.

After this unit, C1 and C2 reach that handler and print the quoted text. A
never-wrong control must not pin that output, so the Show ErrorCall gap is its own
unit. C8 and C9 use a plain handler instead.

## Boundaries

These are not in 524. Each is its own unit, closed with measured controls.

- **No partial prefix.** We write nothing where GHC writes a partial prefix.
- **Fifteen force sites still swallow a failure.** They now continue with their
  fallback value instead of ending the run silently:
  - `++#`;
  - `readIORef`, `writeIORef`, `readSTRef`, `writeSTRef`, `readTVar` and `writeTVar`;
  - `collect_string_list_chirho` (spine and head);
  - `extract_map_pairs_chirho` (two sites);
  - `collect_set_from_list_chirho`;
  - `force_to_prim_chirho`;
  - `collect_map_from_list_chirho` (two sites).
- **Three nested runs still return before restoring the caller's state:** `interact`,
  `modifyIORef` and `modifySTRef`.
- **`showList#` never forces an element** and renders an unresolved one as `?`.
- **Show ErrorCall quotes the message** (H1).
- **Blackholes are unmeasured.** A thunk whose forced evaluation failed may stay
  blackholed, so forcing it again is untested.
- **IORef over-strictness (C3)** is separate.

## Mutants (predictions, not run)

| Mutant | Change | Must fail |
|---|---|---|
| MU-a | return the nested failure before restoring (revert a) | `a_catch_handler_receives_a_failed_element_chirho`: C8 and C9 fail the run instead of reaching the handler |
| MU-b1 | the element force renders `?` again | `a_failed_element_or_tail_fails_the_run_with_its_message_chirho`: G1 prints `[1.5,?]` and returns |
| MU-b2 | the tail force cuts the list again | the same test, at G10, which prints `[1.5]` and returns |
| MU-b3 | the universal-show fallback swallows the error | the same test, at G1 |

Without (a), (b) alone still fails an uncaught run, so only the handler rows can
catch MU-a.

## Measurements

- **Programs:** H1, C9 and G10 are new. They were measured on GHC 9.14.1 and on four
  frozen CLIs: O1 3e28b6d3, held ba1e5c29, Char candidate d14dd58d and unit 523
  2d252022. Every hash was unchanged, and all four builds agree.
- **Receipts:** `tmp-chirho/list-element-fail-closed-1022-chirho/` in claude2_chirho's
  worktree, beside the trace's `tmp-chirho/list-element-error-1018-chirho/`. The
  Core dumps that place C2 on the bare `print` are in its `core-chirho/`.

## Checklist

- [x] Trace, placement and contract (#27694), approved by gpt_chirho.
- [x] Source checkpoint: (a), (b), the controls, the workflow and this tasklist.
  rustfmt is clean, and the programs are embedded by generation and verified 18/18.
- [ ] Gate at a named slot:
  - warning-free build;
  - runtime lib, and driver `evaluation_values_chirho` with the three controls by
    name;
  - driver lib and `io_actions`;
  - changed-line Clippy;
  - the mutants above, each terminating;
  - gitleaks;
  - the execution pair, held vs candidate.
- [ ] Follow-up units for the boundaries above.
