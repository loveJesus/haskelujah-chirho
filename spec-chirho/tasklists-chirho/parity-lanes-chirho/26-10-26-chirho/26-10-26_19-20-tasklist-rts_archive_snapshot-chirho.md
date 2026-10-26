<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Native links read a private RTS snapshot Chirho

## Origin and scope

Row 568 closes finding 6a from row 565 (#29126). I claimed it in #29129, and gpt assigned it in #29131: one cross-process freshness-build and snapshot helper, a private same-volume hard link, bounded cleanup on drop, both production call sites, a survival and cleanup unit control, a parallel stress measurement, and the warm gate.

It starts from main_chirho `1cfb1db7`.

## The race

Every native compile runs `cargo build -p haskelujah-rts` as a freshness check, then links `-L target/debug -lhaskelujah_rts`. Even a no-op build replaces `target/debug/libhaskelujah_rts.a`: measured inodes 40489291, then 40502374, then 40502436, with the same mtime and one link.

The hashed `deps/` archive keeps its inode, but cargo's JSON reports only the uplifted path. So in parallel compiles from one tree, one compile's check can remove the archive while another links it. Row 565's gate r2 lost B06's Cranelift cell this way, with `ld: ... errno=2`, and the same cell passed alone.

## Placement

- **The helper.** `rts_snapshot_chirho` is in `haskelujah-driver-chirho/src/native_link_chirho/rts_snapshot_chirho.rs`, a new group directory. The driver's `src/` holds 10 entries, and its `tests_chirho/` was already full at 25.
  - It takes an exclusive `std::fs::File::lock` on `target/debug/.rts-snapshot-lock-chirho`.
  - Under the lock, it runs the caller's freshness build and hard-links the archive into `target/debug/rts-snapshots-chirho/<pid>-<n>/`.
  - The snapshot's directory is removed when it drops. Directories older than ten minutes are retired under the lock.
  - A hard link that finds the path missing waits up to five seconds. That covers a build that took no lock.
- **Callers:**
  - The CLI's LLVM and Cranelift links: `ensure_rts_staticlib_chirho` returns the snapshot, and both link paths hold it until their linker has run.
  - The test harness: `runtime_snapshot_chirho` builds once per process inside the lock, and every link takes its own snapshot.
  - The driver's in-crate Cranelift stdin test.
- **Unit controls** are in `native_link_chirho/rts_snapshot_tests_chirho.rs`:
  - a snapshot outlives an archive replacement and its owner;
  - a failed freshness check takes no snapshot;
  - a briefly missing archive is waited for;
  - stale directories are retired.
- **Workflow:** `testing-chirho/native-fail-loud-chirho.md`, section "The native link's RTS archive".

## Measured

Parallel CLI compiles of one small program alternate LLVM and Cranelift. The scripts are in the session scratchpad; the logs are in `tmp-chirho/rts-archive-race-chirho/`.

| condition | main `14d6099a` CLI | candidate CLI |
|---|---|---|
| no provocation, 160 compiles at 16 parallel | 160 OK | |
| a raw no-op RTS build loop running, 96 at 12 parallel | 96 OK | 96 OK |
| a loop replacing the archive as cargo does, with the gap held open 10 ms in every 30, 96 at 16 parallel | 68 OK, 28 `ld: library 'haskelujah_rts' not found` | 96 OK |

- The natural gap is an APFS clone that takes microseconds. That is why gate r2's hit was rare.
- Afterwards, no snapshot directory was left, and the archive was restored by cargo.

## Boundaries

- **No change** to codegen, the RTS, or what gets linked.
- **A build outside the lock** is covered only by the five-second wait for a missing path.

## Checklist

- [x] Measure the inode replacement and cargo's reported paths. Claimed (#29129); gpt assigned row 568 (#29131).
- [x] Write the helper and its unit controls (4/4), and move all three link sites to it.
- [x] Measure the stress: main against the candidate.
- [ ] Commit, rebase onto current main, run the warm gate, and record it.
- [ ] Land on main and close progress row 568.
