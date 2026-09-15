<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Consolidate compiler worktrees Chirho

L.J. explicitly requested moving the extra Haskelujah workspaces out of the personal directory into one workspace directory and checking branch cleanup. This maintenance task pauses the unlanded compiler lane; it does not merge that lane or change its corpus gates. Runtime identity: HASKELUJAH/gpt_chirho@HASKELUJAH:3:%78.

Placement: keep the canonical haskelujah-chirho checkout in place. Move all six sibling worktrees, preserving their leaf names, under ../haskelujah-workspaces-chirho/ using git worktree move. Preserve dirty files, untracked files, caches and branch identities. Preserve the effective target of the one relative package-cache symlink. Future worktrees belong in that container.

- [x] Inventory worktrees, branches, dirty files, top-level symlinks and main/remote tips; coordinate a fleet pause.
- [x] Record before manifests and pin the three detached temporary-worktree commits before pruning their stale registrations.
- [x] Move the six worktrees, repair the relative package-cache link, and verify Git registrations, HEADs, statuses, dirty-file hashes and directory identities. All six roots retained their inode/device identity; all staged/unstaged diffs and26 dirty/untracked paths were preserved except the explicitly recorded relative-link spelling change, whose effective target stayed identical.
- [x] Delete only nine unoccupied local branches proven ancestors of main. Retain checked-out branches, both unmerged branches and every remote branch. Three absent temporary registrations were pruned only after tagging their detached commits. Seven live worktrees now remain registered, including main; local branch count falls16 to7.
- [x] Record the final layout, recovery references and validation; document the future placement rule and finish maintenance DB row485. SQLite integrity_check is ok; compiler row484 stays open.

Commit/push scope is only AGENTS.md, the maintenance tasklist/manifest, the worktree workflow and canonical progress DB. The compiler worktree stays dirty and unmerged; remote equality must be checked after the maintenance push.

No source tree or build cache is to be deleted. No hard reset, force branch deletion, remote branch deletion, site deployment, measurement change or closure of the active compiler row484 is authorized by this task.

The before/after manifest is `26-09-15-relocation-chirho.json` beside this tasklist. It records deleted branch names and original commits, local archive recovery tags, exact before/after paths, dirty-file digests and verification scope. No compiler run is inferred from successful relocation; historical build artifacts may contain old absolute paths and require rebuilding before use as current evidence.

Compiler continuation remains uncommitted at1b2e4894 on gpt-kind-schemes-chirho in `haskelujah-workspaces-chirho/haskelujah-gpt-kind-signatures-chirho`. The last measured class/instance checkpoint had parser368, typing390, naming137, integration256 and canaries7;15 declaration references and25 retained source controls agree, while the18 named cycle-file probe recovers15, leaving T6018a and Tc267a/Tc267b. Those are historical pre-move results, not a full corpus or post-move build claim. Resume its tasklist and fresh CLI build from the new root; do not blindly reuse scratch scripts with old absolute paths.
