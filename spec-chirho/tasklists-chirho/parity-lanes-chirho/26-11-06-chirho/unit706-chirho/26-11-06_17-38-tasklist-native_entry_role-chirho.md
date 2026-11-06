<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native program result ownership, unit 706 Chirho

Base: `7075a7df`. This worktree owns only the native launcher role, its callers and focused controls. The canonical checkout, progress database, row 692 receipts and row 704 frame protocol stay outside this unit.

- [x] Inspect the actual program/value handoff and report placement before editing.
- [x] Coordinate LLVM/Cranelift launcher regions and CLI emission regions with their owners.
- [x] Add a shared native execution role and preserve legacy value entry points.
- [x] Carry explicit program role through harness and CLI/Cabal artifact emission.
- [x] Prepare emitted controls for lazy result discard, effects, errors and value evaluation.
- [x] Document the open generic-entry dictionary consumer seam and exact pending gate.
- [ ] Format, inspect the owned diff, scan the outgoing range and commit/push source only.
- [ ] Run the focused real-backend packet only after the sole scheduler grants a compiler slot.

The implementation belongs in `core::io_actions_chirho` for the shared role, with small `codegen_chirho/native_entry_chirho.rs` modules owning only the C launcher in each backend. Backends already depend on Core; no dependency direction changes. The role does not instantiate an entry dictionary, alter a typeclass/defaulting decision, force an IO payload or change RTS/root ownership.

Root granted the launcher/CLI regions and the two-line Cabal producer lease before those edits. The row 704 candidate is frozen; this unit owns no frame or worker changes. Cabal frontend program-entry obligation propagation remains a separate named seam; selecting Program role for the executable artifact does not supply that missing frontend obligation.
