# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

"""Lay out a fresh package-ladder scratch area from these committed inputs (row 703).

    python3 -I make_ladder_scratch_chirho.py --out <new dir> [--copy-store] [--shared-cache <dir>]

It runs no compiler and writes only under the new directory:
- `l0-chirho/`, `l0-bad-chirho/`, `l1-chirho/`, `l1-unsat-chirho/`: the rung projects;
- `haskell-say-1.0.0.0/`: the unchanged package, which is the L0.5 project;
- `haskell-say-1.0.0.0-src/`: a second copy, the input `haskelujah install --from` consumes, so
  an install cannot change the L0.5 project;
- `.haskelujah-packages-chirho/`: an empty store, or with `--copy-store` a plain copy of the package
  as a control that separates install behavior from build behavior;
- `manifest-chirho.json`: every file's sha256 and a tree hash per copy.

With `--shared-cache`, each package copy must also equal that cache's copy, leaving out its top-level
`dist-chirho` build output. That cache hash is the offline provenance claim, not verified Hackage
provenance; the cache is only read. Links and special files are refused rather than followed."""

import argparse
import hashlib
import json
import os
import shutil
import sys
from pathlib import Path

HERE_CHIRHO = Path(__file__).resolve().parent
PACKAGES_DIR_CHIRHO = HERE_CHIRHO / "packages-chirho"
RUNG_PROJECTS_CHIRHO = ["l0-chirho", "l0-bad-chirho", "l1-chirho", "l1-unsat-chirho"]
EXCLUDED_TOP_LEVEL_CHIRHO = {"dist-chirho"}
STORE_NAME_CHIRHO = ".haskelujah-packages-chirho"


def tree_files_chirho(root_chirho: Path) -> tuple[dict[str, str], list[str]]:
    """Map each regular file's relative path to its sha256, leaving out top-level build output."""
    hashes_chirho: dict[str, str] = {}
    excluded_chirho: list[str] = []
    for current_chirho, dirs_chirho, files_chirho in os.walk(root_chirho):
        current_path_chirho = Path(current_chirho)
        for name_chirho in sorted(dirs_chirho):
            if (current_path_chirho / name_chirho).is_symlink():
                raise SystemExit(f"refusing link {current_path_chirho / name_chirho}")
            if current_path_chirho == root_chirho and name_chirho in EXCLUDED_TOP_LEVEL_CHIRHO:
                excluded_chirho.append(name_chirho)
                dirs_chirho.remove(name_chirho)
        for name_chirho in sorted(files_chirho):
            path_chirho = current_path_chirho / name_chirho
            if path_chirho.is_symlink() or not path_chirho.is_file():
                raise SystemExit(f"refusing link or special file {path_chirho}")
            relative_chirho = path_chirho.relative_to(root_chirho).as_posix()
            hashes_chirho[relative_chirho] = hashlib.sha256(path_chirho.read_bytes()).hexdigest()
    return hashes_chirho, excluded_chirho


def tree_hash_chirho(hashes_chirho: dict[str, str]) -> str:
    digest_chirho = hashlib.sha256()
    for relative_chirho in sorted(hashes_chirho):
        digest_chirho.update(f"{relative_chirho}\0{hashes_chirho[relative_chirho]}\n".encode())
    return digest_chirho.hexdigest()


def copy_tree_chirho(source_chirho: Path, dest_chirho: Path) -> dict:
    hashes_chirho, excluded_chirho = tree_files_chirho(source_chirho)
    for relative_chirho in hashes_chirho:
        target_chirho = dest_chirho / relative_chirho
        target_chirho.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source_chirho / relative_chirho, target_chirho)
    if tree_files_chirho(dest_chirho)[0] != hashes_chirho:
        raise SystemExit(f"the copy of {source_chirho} differs from its source")
    return {
        "source_chirho": str(source_chirho),
        "files_chirho": hashes_chirho,
        "tree_sha256_chirho": tree_hash_chirho(hashes_chirho),
        "excluded_chirho": excluded_chirho,
    }


def main_chirho() -> int:
    parser_chirho = argparse.ArgumentParser()
    parser_chirho.add_argument("--out", required=True, type=Path)
    parser_chirho.add_argument("--copy-store", action="store_true")
    parser_chirho.add_argument("--shared-cache", type=Path)
    args_chirho = parser_chirho.parse_args()
    out_chirho: Path = args_chirho.out.resolve()
    if out_chirho.exists():
        raise SystemExit(f"{out_chirho} already exists; the scratch area must be fresh")
    out_chirho.mkdir(parents=True)
    manifest_chirho: dict = {"projects_chirho": {}, "packages_chirho": {}, "store_chirho": None}
    for project_chirho in RUNG_PROJECTS_CHIRHO:
        manifest_chirho["projects_chirho"][project_chirho] = copy_tree_chirho(
            HERE_CHIRHO / project_chirho, out_chirho / project_chirho
        )
    for package_dir_chirho in sorted(path_chirho for path_chirho in PACKAGES_DIR_CHIRHO.iterdir() if path_chirho.is_dir()):
        name_chirho = package_dir_chirho.name
        entry_chirho = copy_tree_chirho(package_dir_chirho, out_chirho / name_chirho)
        install_input_chirho = copy_tree_chirho(package_dir_chirho, out_chirho / f"{name_chirho}-src")
        if install_input_chirho["tree_sha256_chirho"] != entry_chirho["tree_sha256_chirho"]:
            raise SystemExit(f"two copies of {name_chirho} disagree")
        if args_chirho.shared_cache is not None:
            cache_hashes_chirho, cache_excluded_chirho = tree_files_chirho(args_chirho.shared_cache / name_chirho)
            if tree_hash_chirho(cache_hashes_chirho) != entry_chirho["tree_sha256_chirho"]:
                raise SystemExit(f"the committed {name_chirho} differs from {args_chirho.shared_cache / name_chirho}")
            entry_chirho["equals_shared_cache_chirho"] = str(args_chirho.shared_cache / name_chirho)
            entry_chirho["shared_cache_excluded_chirho"] = cache_excluded_chirho
        manifest_chirho["packages_chirho"][name_chirho] = entry_chirho
    store_chirho = out_chirho / STORE_NAME_CHIRHO
    store_chirho.mkdir()
    if args_chirho.copy_store:
        manifest_chirho["store_chirho"] = {
            name_chirho: copy_tree_chirho(PACKAGES_DIR_CHIRHO / name_chirho, store_chirho / name_chirho)["tree_sha256_chirho"]
            for name_chirho in manifest_chirho["packages_chirho"]
        }
    (out_chirho / "manifest-chirho.json").write_text(json.dumps(manifest_chirho, indent=2, sort_keys=True) + "\n")
    for section_chirho in ("packages_chirho", "projects_chirho"):
        for name_chirho, entry_chirho in manifest_chirho[section_chirho].items():
            print(f"{name_chirho}: {len(entry_chirho['files_chirho'])} files, tree {entry_chirho['tree_sha256_chirho'][:16]}")
    print(f"store: {'plain copy' if args_chirho.copy_store else 'empty'} at {store_chirho}")
    return 0


if __name__ == "__main__":
    sys.exit(main_chirho())
