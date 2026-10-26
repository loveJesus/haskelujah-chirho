<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native Map Size Chirho

Provisional Row 545 is stacked after tested Row 544 at `5dc19cb6` while #28786 holds `main_chirho` and the progress database. It owns saved native oracle P05:

```haskell
print (M.size (M.fromList [(1 :: Int, "a")]))
```

GHC and the interpreter print `1`. LLVM and Cranelift currently refuse `mapFromList#/1, mapSize#/1`.

## Placement and boundary

- A native Map uses a sorted association list behind the opaque Map primops. Empty and singleton maps need no comparison, so their representation is correct for every key type.
- `mapFromList#` lives in one bounded RTS module. It validates and demands the list spine, accepts zero or one tuple, returns a fresh one-cell map for a singleton, and fails named for a second pair.
- `mapSize#` validates and counts that opaque association-list representation.
- LLVM and Cranelift force the one argument and call the same two RTS entries. P05 is the end-to-end contract.
- P04 remains a named compile refusal because `mapToList#` stays unsupported. Its two-element `fromList` requires real ordering.
- Root cause for that remaining boundary: typing records `Ord k`, but the generated `mapFromList` Core binding calls unary `mapFromList#` with no dictionary or key-type witness. Native `i64` values cannot distinguish an `Int` key from a `Double` bit pattern, so sorting them as signed integers would silently miscompile other legal maps. A later unit must carry comparator evidence to the primitive instead of guessing.

## Checklist

- [x] Reproduce P05's refusal and trace the interpreter and generated-Core representations.
- [x] Reject the unsound generic signed-integer sort after proving the `Ord` evidence is erased.
- [x] Add shared RTS singleton construction and size controls.
- [x] Lower `mapFromList#` and `mapSize#` on LLVM and Cranelift while retaining `mapToList#` refusal.
- [x] Turn P05 into an exact all-engine control and keep P04 refused.
- [x] Update the native execution workflow at the changed seam.
- [x] Run the warm 2–5 minute gate at the actual behavior scope.
- [ ] Rebase onto released main, land, and close progress row 545 GREEN.

## Source checkpoint

- The shared RTS accepts only empty or singleton `fromList` inputs, validates list and tuple allocations, copies the singleton spine, and reports malformed, cyclic, overflowing, or comparator-dependent inputs by a stable internal error.
- Generated Core spells pair constructors as `$tuple2`. The first gate exposed that globally normalizing every `$tupleN` tag to zero changed an existing tuple case from a named failure into the wrong success `()`. The corrected route preserves each backend's existing tuple tags and passes only its `$tuple2` tag into the shared RTS pair validator.
- GHC 9.14.1, the interpreter, LLVM, and Cranelift each print `1` for P05. Both native backends still refuse P04 at compile time because `mapToList#/1` is unsupported. A two-pair size program reaches the shared `mapFromList# native maps currently require at most one pair` failure on both backends.
- Warm focused proof: RTS 55/55, LLVM tuple-tag control 1/1, Cranelift tuple-tag control 1/1, and the driver `map` filter 4/4. The concurrent focused wall time was 26.2 seconds. Rustfmt and diff-check are clean; Clippy reports no diagnostic in the new RTS module, with only pre-existing diagnostics elsewhere.
- The combined-main re-gate caught the overbroad tuple-tag normalization because both native no-match controls changed to `()`. This correction is not accepted until those controls fail named again beside P05's exact `1`.
- The corrected focused proof is GREEN: RTS 55/55, LLVM 38/38, Cranelift 42/42, and native error helpers 29/29 in 21.1 seconds after the build. P05 remains exact, the two-pair Map boundary remains named, and both pre-existing no-match controls again fail named. Clippy reports no diagnostic on a changed line; rustfmt and diff-check are clean.
