# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Pin one backend for an owned process group; stop rather than switch or retry.

Workflow: README-chirho.md. Monitor work is constant per poll, not a tree scan.
"""

import json as json_chirho
import os as os_chirho
from pathlib import Path as PathChirho
import shutil as shutil_chirho
import signal as signal_chirho
import stat as stat_chirho
import subprocess as subprocess_chirho
import time as time_chirho

from selection_chirho import CacheErrorChirho, MARKER_CHIRHO, VOLUME_CHIRHO


class OwnedSignalChirho(BaseException):
    def __init__(self, number_chirho):
        self.number_chirho = number_chirho


def read_marker_chirho(root_chirho):
    descriptor_chirho = os_chirho.open(root_chirho / MARKER_CHIRHO, os_chirho.O_RDONLY | os_chirho.O_NOFOLLOW | os_chirho.O_NONBLOCK)
    try:
        info_chirho = os_chirho.fstat(descriptor_chirho)
        if not stat_chirho.S_ISREG(info_chirho.st_mode) or info_chirho.st_size > 4096:
            raise CacheErrorChirho("invalid cache marker")
        return os_chirho.read(descriptor_chirho, 4097)
    finally:
        os_chirho.close(descriptor_chirho)


class StorageGuardChirho:
    def __init__(self, selected_chirho):
        self.selected_chirho = selected_chirho
        self.root_chirho = PathChirho(selected_chirho["cache_root_chirho"])
        self.temp_chirho = PathChirho(selected_chirho["temp_root_chirho"])
        if not self.root_chirho.is_absolute() or self.root_chirho.is_symlink() or self.temp_chirho != self.root_chirho / "tmp-chirho" or self.temp_chirho.is_symlink():
            raise CacheErrorChirho("unsafe selected cache paths")
        self.stat_chirho = self.root_chirho.stat()
        self.marker_chirho = read_marker_chirho(self.root_chirho)
        if len(self.marker_chirho) > 4096:
            raise CacheErrorChirho("invalid cache marker")
        marker_chirho = json_chirho.loads(self.marker_chirho)
        if any(marker_chirho.get(key_chirho) != selected_chirho.get(key_chirho) for key_chirho in ["schema_chirho", "project_chirho", "worktree_key_chirho", "tool_chirho"]):
            raise CacheErrorChirho("cache marker identity mismatch")
        if not self.valid_chirho():
            raise CacheErrorChirho("selected cache changed before build")

    def valid_chirho(self):
        try:
            stat_chirho = self.root_chirho.stat()
            if self.root_chirho.is_symlink() or stat_chirho.st_dev != self.selected_chirho["device_chirho"] or (stat_chirho.st_dev, stat_chirho.st_ino) != (self.stat_chirho.st_dev, self.stat_chirho.st_ino):
                return False
            if self.selected_chirho["backend_chirho"] == "external_chirho":
                if not self.root_chirho.is_relative_to(VOLUME_CHIRHO) or not os_chirho.path.ismount(VOLUME_CHIRHO) or VOLUME_CHIRHO.stat().st_dev != stat_chirho.st_dev:
                    return False
            elif self.selected_chirho["backend_chirho"] != "local_chirho" or self.root_chirho.is_relative_to(PathChirho("/Volumes")):
                return False
            return not self.temp_chirho.is_symlink() and read_marker_chirho(self.root_chirho) == self.marker_chirho
        except (OSError, ValueError, KeyError, CacheErrorChirho):
            return False


def stop_group_chirho(process_chirho):
    try:
        os_chirho.killpg(process_chirho.pid, signal_chirho.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        process_chirho.wait(timeout=3)
    except subprocess_chirho.TimeoutExpired:
        pass
    # The group may still contain a child after its leader exits. Clean only this
    # newly created session/group, never a matching executable in another fleet.
    try:
        os_chirho.killpg(process_chirho.pid, signal_chirho.SIGKILL)
    except ProcessLookupError:
        pass
    process_chirho.wait()


def run_owned_chirho(argv_chirho, worktree_chirho, env_chirho, guard_chirho, reserve_chirho=2 << 30):
    previous_chirho = {}
    process_chirho = None

    def interrupt_chirho(number_chirho, frame_chirho):
        raise OwnedSignalChirho(number_chirho)

    reason_chirho = "completed_chirho"
    try:
        for number_chirho in [signal_chirho.SIGTERM, signal_chirho.SIGHUP]:
            previous_chirho[number_chirho] = signal_chirho.signal(number_chirho, interrupt_chirho)
        process_chirho = subprocess_chirho.Popen(argv_chirho, cwd=worktree_chirho, env=env_chirho, start_new_session=True)
        while process_chirho.poll() is None:
            if not guard_chirho.valid_chirho():
                reason_chirho = "selected_backend_disappeared_chirho"
                stop_group_chirho(process_chirho)
                return 74, reason_chirho
            if min(shutil_chirho.disk_usage(guard_chirho.root_chirho).free, shutil_chirho.disk_usage(PathChirho.home()).free) < reserve_chirho:
                reason_chirho = "cache_reserve_exhausted_chirho"
                stop_group_chirho(process_chirho)
                return 74, reason_chirho
            time_chirho.sleep(0.2)
        if not guard_chirho.valid_chirho():
            return 74, "selected_backend_disappeared_chirho"
        code_chirho = process_chirho.returncode
        return code_chirho if code_chirho >= 0 else 128 - code_chirho, reason_chirho
    finally:
        # A second ordinary termination must not interrupt owned-group teardown.
        for number_chirho in previous_chirho:
            signal_chirho.signal(number_chirho, signal_chirho.SIG_IGN)
        try:
            if process_chirho is not None:
                stop_group_chirho(process_chirho)
        finally:
            for number_chirho, handler_chirho in previous_chirho.items():
                signal_chirho.signal(number_chirho, handler_chirho)
