# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Normal developer Cargo entry. Frozen packets keep their original commands.

Workflow: README-chirho.md. Selection is per invocation and never a global env edit.
"""

import argparse as argparse_chirho
import fcntl as fcntl_chirho
import json as json_chirho
import os as os_chirho
from pathlib import Path as PathChirho
import shutil as shutil_chirho
import subprocess as subprocess_chirho
import sys as sys_chirho
import tempfile as tempfile_chirho

from monitor_chirho import OwnedSignalChirho, StorageGuardChirho, run_owned_chirho
from selection_chirho import CacheErrorChirho, choose_chirho


def profile_chirho(cargo_flags_chirho):
    value_chirho = "release" if "--release" in cargo_flags_chirho or "bench" in cargo_flags_chirho else "dev"
    for index_chirho, argument_chirho in enumerate(cargo_flags_chirho):
        if argument_chirho.startswith("--profile="):
            value_chirho = argument_chirho.split("=", 1)[1]
        elif argument_chirho == "--profile":
            if index_chirho + 1 == len(cargo_flags_chirho):
                raise CacheErrorChirho("Cargo --profile needs its value")
            value_chirho = cargo_flags_chirho[index_chirho + 1]
    return "cargo-" + value_chirho + "-chirho"


def main_chirho(arguments_chirho=None, selector_chirho=choose_chirho):
    parser_chirho = argparse_chirho.ArgumentParser(description="External-first isolated Cargo cache and TMP, with local fallback.")
    parser_chirho.add_argument("--worktree-chirho", type=PathChirho)
    parser_chirho.add_argument("--tool-chirho")
    parser_chirho.add_argument("--min-free-gib-chirho", type=int, default=8)
    parser_chirho.add_argument("--select-only-chirho", action="store_true")
    parser_chirho.add_argument("--private-temp-chirho", action="store_true", help="keep temporary output off the external cache")
    parser_chirho.add_argument("cargo_args_chirho", nargs=argparse_chirho.REMAINDER)
    options_chirho = parser_chirho.parse_args(arguments_chirho)
    try:
        if options_chirho.min_free_gib_chirho < 0 or (not options_chirho.select_only_chirho and options_chirho.min_free_gib_chirho < 2):
            raise CacheErrorChirho("at least 2 GiB reserve is required")
        if options_chirho.worktree_chirho is None:
            root_chirho = subprocess_chirho.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, timeout=5, check=True).stdout.decode().strip()
            worktree_chirho = PathChirho(root_chirho).resolve(strict=True)
        else:
            worktree_chirho = options_chirho.worktree_chirho.resolve(strict=True)
        if not (worktree_chirho / "Cargo.toml").is_file():
            raise CacheErrorChirho("selected worktree has no Cargo manifest")
        cargo_args_chirho = options_chirho.cargo_args_chirho
        if cargo_args_chirho[:1] == ["--"]:
            cargo_args_chirho = cargo_args_chirho[1:]
        cargo_flags_chirho = cargo_args_chirho[:cargo_args_chirho.index("--")] if "--" in cargo_args_chirho else cargo_args_chirho
        if any(value_chirho == "--target-dir" or value_chirho.startswith("--target-dir=") or value_chirho == "--config" or value_chirho.startswith("--config=") for value_chirho in cargo_flags_chirho):
            raise CacheErrorChirho("target/config overrides require an explicitly pinned direct-Cargo job")
        tool_chirho = options_chirho.tool_chirho or profile_chirho(cargo_flags_chirho)
        selected_chirho = selector_chirho(worktree_chirho, tool_chirho, options_chirho.min_free_gib_chirho << 30)
        guard_chirho = StorageGuardChirho(selected_chirho)
        target_chirho = guard_chirho.root_chirho / "target-chirho"
        result_chirho = {**selected_chirho, "target_directory_chirho": str(target_chirho), "worktree_chirho": str(worktree_chirho)}
        private_temp_chirho = options_chirho.private_temp_chirho or any(command_chirho in cargo_flags_chirho for command_chirho in ["test", "run", "bench", "install"])
        result_chirho["temporary_policy_chirho"] = "private_local_chirho" if private_temp_chirho else "selected_cache_chirho"
        if options_chirho.select_only_chirho:
            print(json_chirho.dumps(result_chirho))
            return 0
        if shutil_chirho.disk_usage(guard_chirho.root_chirho).free < options_chirho.min_free_gib_chirho << 30:
            raise CacheErrorChirho("selected cache does not meet the build space requirement")
        if shutil_chirho.disk_usage(PathChirho.home()).free < 2 << 30:
            raise CacheErrorChirho("internal disk is below the 2 GiB reserve; build not started")
        if not cargo_args_chirho:
            raise CacheErrorChirho("provide Cargo arguments after --")
        cargo_chirho = shutil_chirho.which("cargo")
        if cargo_chirho is None:
            raise CacheErrorChirho("Cargo is not installed")
        # Cargo also serializes its target internally; this lock keeps the TMP
        # lifetime and storage monitor under the same single invocation owner.
        if target_chirho.is_symlink():
            raise CacheErrorChirho("Cargo target must not be a symlink")
        lock_fd_chirho = os_chirho.open(guard_chirho.root_chirho / ".cargo-build-lock-chirho", os_chirho.O_WRONLY | os_chirho.O_CREAT | os_chirho.O_NOFOLLOW, 0o600)
        with os_chirho.fdopen(lock_fd_chirho, "w") as lock_chirho:
            try:
                fcntl_chirho.flock(lock_chirho, fcntl_chirho.LOCK_EX | fcntl_chirho.LOCK_NB)
            except BlockingIOError as error_chirho:
                raise CacheErrorChirho("selected worktree/profile already has an owned build") from error_chirho
            if not guard_chirho.valid_chirho():
                raise CacheErrorChirho("selected cache disappeared before build")
            temp_parent_chirho = guard_chirho.temp_chirho
            if private_temp_chirho:
                temp_parent_chirho = PathChirho(tempfile_chirho.gettempdir()).resolve()
                if temp_parent_chirho.is_relative_to(PathChirho("/Volumes")):
                    temp_parent_chirho = PathChirho("/private/tmp" if sys_chirho.platform == "darwin" else "/tmp").resolve()
                if temp_parent_chirho.is_relative_to(PathChirho("/Volumes")):
                    raise CacheErrorChirho("no internal private runtime TMP is available")
            temporary_chirho = PathChirho(tempfile_chirho.mkdtemp(prefix="cargo-run-chirho-", dir=temp_parent_chirho))
            environment_chirho = {**os_chirho.environ, "CARGO_TARGET_DIR": str(target_chirho), "TMPDIR": str(temporary_chirho), "TMP": str(temporary_chirho), "TEMP": str(temporary_chirho)}
            print("build-cache-chirho: " + json_chirho.dumps(result_chirho), file=sys_chirho.stderr, flush=True)
            try:
                code_chirho, reason_chirho = run_owned_chirho([cargo_chirho, *cargo_args_chirho], worktree_chirho, environment_chirho, guard_chirho)
            finally:
                # Never clean through a path that may now name a replacement drive.
                if private_temp_chirho or guard_chirho.valid_chirho():
                    shutil_chirho.rmtree(temporary_chirho)
            print("build-cache-result-chirho: " + json_chirho.dumps({"rc_chirho": code_chirho, "reason_chirho": reason_chirho, "backend_chirho": selected_chirho["backend_chirho"]}), file=sys_chirho.stderr)
            return code_chirho
    except KeyboardInterrupt:
        return 130
    except OwnedSignalChirho as stop_chirho:
        return 128 + stop_chirho.number_chirho
    except (CacheErrorChirho, OSError, ValueError, KeyError, subprocess_chirho.SubprocessError) as error_chirho:
        print("build-cache-chirho: " + (str(error_chirho) if isinstance(error_chirho, CacheErrorChirho) else "build storage or tool unavailable"), file=sys_chirho.stderr)
        return 74


if __name__ == "__main__":
    raise SystemExit(main_chirho())
