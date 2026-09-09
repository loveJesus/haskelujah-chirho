<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# AICEO report — HASKELUJAH — 2026-09-09

Reporter: HASKELUJAH/gpt_chirho. L.J. directed deep repairs until the tests pass,
with appropriate commits and pushes. No public deployment is authorized here.
Validated compiler/test source: `2f74126d`; execution evidence: `77b55f5f`.
Local factual review: HASKELUJAH/claude2_chirho, room message 21712; no disagreement.

## Completed correctness gate

- Full workspace: **3318 passed, zero failed, zero ignored, zero filtered**,
  cargo exit 0 on macOS arm64, with the documented 16 MiB Rust test-thread stack.
- Driver library: 1773/1773, including all 126 bounded native round trips.
- Curated suite: 537/537, comprising 514 actually compared execution oracles
  and 23 compile-only inputs. All 514 execution sources were independently rerun
  under GHC 9.14.1; their reference outputs and source SHA-256 values are committed.
- The two upstream typecheck corpora did not move: two full passes each,
  byte-identical sets and fixed CLI digest, zero timeouts/unexpected exits.

```text
# QUOTE-AS: ~93% (877 of 938) — do not quote a third significant figure
# QUOTE-AS: ~29% (222 of 767) — do not quote a third significant figure
```

Those percentages are typecheck verdicts, not execution compatibility. Neither
upstream corpus is fully passing. The eight bulk tracking tests being green
does not change that distinction.

## Why unchanged counts do not mean unchanged behavior

The old curated harness ignored 500 declared answer headers. The headline was
already 537/537, but it now carries 514 compared execution oracles rather than 14.
The old native
helper silently accepted four crash/link failures. Both instruments now require
the result they claim, with controls shown to fail for the wrong evidence.
The implementation fixes include dictionary evidence and projection, IO action
reuse, lazy/strict demand boundaries, full-width thunk memoization, GC roots,
bounded shared-case lowering and race-free temporary ownership. Required fixture
slices and their upstream licenses are now worktree-portable.

## Limits and next work

The separate clippy run still emits 535 warning messages, including duplicates;
oversized files/directories remain structural debt. Optional missing-package
early returns are not package-compilation proof, and macOS results do not prove
Linux execution. Wasm has not adopted the repaired IO-action representation.
Remaining upstream compatibility work continues from the unchanged red lists.
No website deployment or production change was made.

Durable evidence: `spec-chirho/workflows-chirho/testing-chirho/execution-measurement-chirho.md`,
its 75-target JSONL companion, and `test-data-chirho/curated-oracles-chirho/`.
Detailed reductions and failed intermediate gates are preserved in
`spec-chirho/tasklists-chirho/26-09-08_17-09-tasklist-test_repairs-chirho.md`.
