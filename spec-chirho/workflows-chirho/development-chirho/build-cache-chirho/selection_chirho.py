# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Shared-selector adaptation and a portable LOCAL-only missing-helper fallback.

Workflow: ../build-cache-chirho/README-chirho.md. No copying or retirement.
"""

import hashlib as hashlib_chirho
import json as json_chirho
import os as os_chirho
from pathlib import Path as PathChirho
import re as re_chirho
import shutil as shutil_chirho
import stat as stat_chirho
import subprocess as subprocess_chirho
import sys as sys_chirho

PROJECT_CHIRHO = "haskelujah-chirho"
MARKER_CHIRHO = ".build-cache-namespace-chirho.json"
VOLUME_CHIRHO = PathChirho("/Volumes/OPENALEX_M2_CHIRHO")


class CacheErrorChirho(RuntimeError):
    """Fixed diagnostics never echo credentials or a subprocess environment."""


def identity_chirho(worktree_chirho, profile_chirho):
    if not re_chirho.fullmatch(r"[a-z0-9][a-z0-9_-]{0,95}chirho", profile_chirho):
        raise CacheErrorChirho("invalid Cargo cache profile")
    return {
        "schema_chirho": "build-cache-v1-chirho",
        "project_chirho": PROJECT_CHIRHO,
        "worktree_key_chirho": hashlib_chirho.sha256(os_chirho.fsencode(worktree_chirho)).hexdigest(),
        "tool_chirho": profile_chirho,
    }


def local_cache_chirho(worktree_chirho, profile_chirho, minimum_chirho):
    # Only used when the shared selector is not installed. Never probe or create
    # anything below /Volumes in this path. Namespace/marker match the shared API.
    identity_value_chirho = identity_chirho(worktree_chirho, profile_chirho)
    key_chirho = identity_value_chirho["worktree_key_chirho"]
    project_key_chirho = hashlib_chirho.sha256(PROJECT_CHIRHO.encode()).hexdigest()
    home_chirho = PathChirho.home().resolve()
    prefix_chirho = ["Library", "Caches"] if sys_chirho.platform == "darwin" else [".cache"]
    parts_chirho = [*prefix_chirho, "chirho-builds-chirho", f"group-{int(project_key_chirho[:2], 16) % 8:02d}-chirho", PROJECT_CHIRHO, f"worktrees-{int(key_chirho[:2], 16) % 8:02d}-chirho", f"worktree-{key_chirho[:16]}-chirho", profile_chirho]
    flags_chirho = os_chirho.O_RDONLY | os_chirho.O_DIRECTORY | os_chirho.O_NOFOLLOW
    descriptor_chirho = os_chirho.open(home_chirho, flags_chirho)
    try:
        for part_chirho in parts_chirho:
            try:
                os_chirho.mkdir(part_chirho, 0o700, dir_fd=descriptor_chirho)
            except FileExistsError:
                pass
            next_chirho = os_chirho.open(part_chirho, flags_chirho, dir_fd=descriptor_chirho)
            os_chirho.close(descriptor_chirho)
            descriptor_chirho = next_chirho
        marker_body_chirho = (json_chirho.dumps(identity_value_chirho, sort_keys=True) + "\n").encode()
        try:
            os_chirho.stat(MARKER_CHIRHO, dir_fd=descriptor_chirho, follow_symlinks=False)
        except FileNotFoundError:
            with os_chirho.scandir(descriptor_chirho) as entries_chirho:
                if next(entries_chirho, None) is not None:
                    raise CacheErrorChirho("nonempty local cache has no owner marker")
        try:
            marker_fd_chirho = os_chirho.open(MARKER_CHIRHO, os_chirho.O_WRONLY | os_chirho.O_CREAT | os_chirho.O_EXCL | os_chirho.O_NOFOLLOW, 0o600, dir_fd=descriptor_chirho)
        except FileExistsError:
            marker_fd_chirho = os_chirho.open(MARKER_CHIRHO, os_chirho.O_RDONLY | os_chirho.O_NOFOLLOW | os_chirho.O_NONBLOCK, dir_fd=descriptor_chirho)
            try:
                info_chirho = os_chirho.fstat(marker_fd_chirho)
                if not stat_chirho.S_ISREG(info_chirho.st_mode) or info_chirho.st_size > 4096 or os_chirho.read(marker_fd_chirho, 4097) != marker_body_chirho:
                    raise CacheErrorChirho("local cache namespace mismatch")
            finally:
                os_chirho.close(marker_fd_chirho)
        else:
            try:
                if os_chirho.write(marker_fd_chirho, marker_body_chirho) != len(marker_body_chirho):
                    raise CacheErrorChirho("local cache marker write failed")
                os_chirho.fsync(marker_fd_chirho)
            finally:
                os_chirho.close(marker_fd_chirho)
        try:
            os_chirho.mkdir("tmp-chirho", 0o700, dir_fd=descriptor_chirho)
        except FileExistsError:
            pass
        temp_fd_chirho = os_chirho.open("tmp-chirho", flags_chirho, dir_fd=descriptor_chirho)
        os_chirho.close(temp_fd_chirho)
    finally:
        os_chirho.close(descriptor_chirho)
    root_chirho = home_chirho.joinpath(*parts_chirho)
    if shutil_chirho.disk_usage(root_chirho).free < minimum_chirho:
        raise CacheErrorChirho("local cache has insufficient free space")
    return {**identity_value_chirho, "backend_chirho": "local_chirho", "cache_root_chirho": str(root_chirho), "temp_root_chirho": str(root_chirho / "tmp-chirho"), "device_chirho": root_chirho.stat().st_dev, "available_bytes_chirho": shutil_chirho.disk_usage(root_chirho).free, "reason_chirho": "shared_selector_not_installed_chirho"}


def choose_chirho(worktree_chirho, profile_chirho, minimum_chirho):
    identity_chirho(worktree_chirho, profile_chirho)
    helper_chirho = shutil_chirho.which("build-cache-chirho")
    if helper_chirho is None:
        candidate_chirho = PathChirho.home() / "bin-chirho/build-cache-chirho"
        helper_chirho = str(candidate_chirho) if candidate_chirho.is_file() else None
    if helper_chirho is None:
        return local_cache_chirho(worktree_chirho, profile_chirho, minimum_chirho)
    try:
        completed_chirho = subprocess_chirho.run([helper_chirho, "--project-chirho", PROJECT_CHIRHO, "--worktree-chirho", str(worktree_chirho), "--tool-chirho", profile_chirho, "--min-free-bytes-chirho", str(minimum_chirho)], capture_output=True, timeout=15, check=False)
    except (FileNotFoundError, PermissionError):
        return local_cache_chirho(worktree_chirho, profile_chirho, minimum_chirho)
    if completed_chirho.returncode or len(completed_chirho.stdout) > 16384:
        raise CacheErrorChirho("shared selector found no safe suitable cache")
    try:
        result_chirho = json_chirho.loads(completed_chirho.stdout)
    except ValueError as error_chirho:
        raise CacheErrorChirho("invalid shared-selector response") from error_chirho
    if not isinstance(result_chirho, dict) or any(not isinstance(result_chirho.get(key_chirho), str) for key_chirho in ["cache_root_chirho", "temp_root_chirho", "backend_chirho"]) or not isinstance(result_chirho.get("device_chirho"), int):
        raise CacheErrorChirho("invalid shared-selector response fields")
    expected_chirho = identity_chirho(worktree_chirho, profile_chirho)
    if any(result_chirho.get(key_chirho) != value_chirho for key_chirho, value_chirho in expected_chirho.items()):
        raise CacheErrorChirho("shared-selector namespace mismatch")
    return result_chirho
