<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Bug: Cranelift executable for a recursive `Describable [a]` instance never terminates

**Found:** 2026-09-03, by the landing gate of the rigid-type-variables lane (`cargo test --workspace`
hung); root-caused by gpt_chirho's read-only stack sample and confirmed against the untouched
HEAD worktree by claude_chirho.
**Status:** open, PRE-EXISTING at `dd6d694a`. Not caused by the typing lane: the STG interpreter
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

`haskelujah compile Desc.hs --cranelift -o desc && ./desc` runs at ~95% CPU forever. A stack
sample (gpt_chirho, 2026-09-03 21:14 EDT) shows unbounded recursion inside
`$prim_Describable_describe_[a]`: the element call `describe x` inside the list instance is
dispatched to the list instance again instead of to `Describable Int`.

The driver unit test `tests_chirho::compile_chirho::cranelift_round_trip_recursive_custom_list_instance_output_chirho`
(`crates/haskelujah-driver-chirho/src/tests_chirho/compile_chirho.rs:8912`) exercises exactly this
program and therefore never returns; `cargo test --workspace` blocks on it with the generated
child at full CPU. Until fixed, run the suite with
`-- --skip cranelift_round_trip_recursive_custom_list_instance_output_chirho`, and give the
round-trip helper a wall-clock budget so a runaway program fails the test instead of hanging it.

## Where to look

- `crates/haskelujah-core-chirho/src/dict_chirho/` — dictionary passing for an instance whose
  context names the element type (`Describable a => Describable [a]`): the element dictionary must
  be the instance's *argument*, not the instance itself.
- The Cranelift backend's handling of the dictionary argument for `describe x` inside the list
  instance (the interpreter gets it right, so the defect is at or after Core-to-STG for this
  backend).
