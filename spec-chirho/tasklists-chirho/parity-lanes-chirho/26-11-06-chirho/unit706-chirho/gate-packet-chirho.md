<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Unit 706 pending gate Chirho

Status: SOURCE ONLY. Base `7075a7df`; the candidate is the commit carrying this packet. No Cargo/compiler/backend verifier/linker/native/GHC command has run for unit 706. Standalone rustfmt and Git whitespace checks are source preparation only. The sole kind scheduler owns the next compiler-slot grant; this packet grants none.

Candidate worktree:

`/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-gpt-native-entry-role-chirho`

After the scheduler records the exact candidate and grants the slot, its focused command is:

```sh
cargo test --locked --offline -j 2 -p haskelujah-driver --test native_entry_role_chirho -- --nocapture --test-threads=1
```

The package name is `haskelujah-driver`, not its suffixed directory name. The nested RTS freshness build uses the same locked, offline, two-job policy: `cargo build --locked --offline -j 2 -p haskelujah-rts --quiet`. The target contains six named tests and executes both LLVM and Cranelift independently; it compiles real source through the frontend, emits artifacts and uses each run's own RTS snapshot. It checks:

- Program `IO Int` returning 7: status 0, empty stdout.
- Program printing 1 then returning 5: status 0, stdout `1\n`.
- Program returning an unused failing payload: status 0, empty stdout, no payload demand.
- Pure evaluator `Int` 7: retain the existing backend value observation (LLVM stdout `7\n` and status 7; Cranelift empty stdout and status 7).
- Program action failing after printing: named runtime error, status 1, prior effect retained.
- Effectful generic NoMonomorphismRestriction entry: status 0 and stdout `7\n`; this is a required open-seam control, never replaced with an empty-output generic `return` case.

Record the generic effect control separately if it remains red. Program launcher discard is not entry-dictionary application. No typeclass/evidence guard changes belong to this candidate, and the entry-dictionary consumer seam is not closed by a vacuous empty-output success.

After the focused target, rerun the original row 692 restriction controls at the assembled candidate, preserving their sources/expected results and old `7075a7df`/w08 receipts. The row 704 owner separately gates root-lifetime behavior; unit 706 changes neither the RTS worker nor generated frame ownership. Preserve `99898c26`'s independent RTS build policy when assembling the harness/CLI hunks.

Cabal executable IR emission receives Program role at its existing producer, not by regenerating IR only in the CLI. Cabal frontend entry-obligation propagation remains open. The launcher regions extracted from both backends are byte-identical in `7075a7df` and `39a5a806`; preserve row 704 module imports and all frame/handoff hunks when assembling.

Source implementation: `e8b931f1ebaf8965cbc4ec38db033ad9dbb3d0ff`, pushed on `native-entry-role-706-chirho`. A following documentation-only receipt marks that source preparation complete. The implementation's formatter, whitespace and exact outgoing gitleaks checks passed; behavior remains unrun.
