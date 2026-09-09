<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: Cranelift executable for a recursive `Describable [a]` instance never terminates

**Found:** 2026-09-03, by the landing gate of the rigid-type-variables lane (`cargo test --workspace`
hung); root-caused by gpt_chirho's read-only stack sample and confirmed against the untouched
HEAD worktree by claude_chirho.
**Status:** repair verified on the 2026-09-08 test-repairs worktree; landing pending.
The defect was PRE-EXISTING at `dd6d694a`. Not caused by the typing lane: the STG interpreter
(`haskelujah run`) prints `1:2:3:[]` for the program with both the HEAD binary and the lane's
binary, and the Cranelift executable produced by the HEAD binary loops exactly like the lane's.
**Severity:** a wrong program (infinite recursion) from a valid source; the unit test that pins the
behaviour hangs the whole workspace test run instead of failing.

## Reproduction

```haskell
module Main where
class Describable a where
  describe :: a -> String
instance Describable Int where
  describe x = show x
instance Describable a => Describable [a] where
  describe [] = "[]"
  describe (x:xs) = describe x ++ ":" ++ describe xs
main = putStrLn (describe [1,2,3])
```

Before the repair, `haskelujah compile Desc.hs --cranelift -o desc && ./desc` runs at ~95% CPU forever. A stack
sample (gpt_chirho, 2026-09-03 21:14 EDT) shows unbounded recursion inside
`$prim_Describable_describe_[a]`: the element call `describe x` inside the list instance is
dispatched to the list instance again instead of to `Describable Int`.

The driver unit test `tests_chirho::compile_chirho::cranelift_round_trip_recursive_custom_list_instance_output_chirho`
(`crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs`) exercises exactly this
program. It formerly blocked `cargo test --workspace` with the generated child at full CPU.
The repaired test runs without a skip and asserts the exact output `1:2:3:[]\n`.

## Verified repair

Dictionary simplification no longer erases arguments based on dictionary-like names.
`simplify_chirho/dictionaries_chirho.rs` projects a field only when the selector and
constructor structure prove it; unresolved evidence remains a runtime parameter.
Cranelift also enters a function thunk before interpreting its value as a callable
closure. The element call therefore receives the element dictionary, not the list
dictionary.

The native round-trip harness now fails compilation, linking, signals and deadlines
explicitly. This program completed with its exact output in the frozen 126/126 native
gate and again in the full driver-library run: 1766 passed, zero failed, zero ignored,
zero filtered. The second run took 3056 seconds; this is not a claim that the remaining
workspace integration targets or the two GHC typecheck corpora are all passing.
The mechanism and execution-boundary contract are documented in
`workflows-chirho/testing-chirho/execution-oracles-chirho.md`.

## Where to look

- `crates/haskelujah-core-chirho/src/dict_chirho/` — dictionary passing for an instance whose
  context names the element type (`Describable a => Describable [a]`): the element dictionary must
  be the instance's *argument*, not the instance itself.
- The Cranelift backend's handling of the dictionary argument for `describe x` inside the list
  instance (the interpreter gets it right, so the defect is at or after Core-to-STG for this
  backend).
