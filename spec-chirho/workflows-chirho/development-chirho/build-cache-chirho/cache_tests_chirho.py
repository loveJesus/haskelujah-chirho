# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""Small storage/process controls. No compiler, drive disconnect or bulk output.

Workflow: README-chirho.md. Subprocess fixtures are owned by each test only.
"""

import contextlib as contextlib_chirho
import fcntl as fcntl_chirho
import io as io_chirho
import json as json_chirho
import os as os_chirho
from pathlib import Path as PathChirho
import signal as signal_chirho
import subprocess as subprocess_chirho
import sys as sys_chirho
import tempfile as tempfile_chirho
import time as time_chirho
import unittest as unittest_chirho
from unittest import mock as mock_chirho

import cargo_build_chirho
import monitor_chirho
import selection_chirho


class CacheTestsChirho(unittest_chirho.TestCase):
    def setUp(self):
        self.temporary_chirho = tempfile_chirho.TemporaryDirectory(prefix="haskelujah-cache-test-chirho-")
        self.addCleanup(self.temporary_chirho.cleanup)
        self.root_chirho = PathChirho(self.temporary_chirho.name)
        self.worktree_chirho = self.root_chirho / "source-chirho"
        self.worktree_chirho.mkdir()
        (self.worktree_chirho / "Cargo.toml").write_text("[workspace]\n")
        self.bin_chirho = self.root_chirho / "bin-chirho"
        self.bin_chirho.mkdir()
        self.cargo_chirho = self.bin_chirho / "cargo"
        self.received_chirho = self.root_chirho / "received-chirho.txt"
        self.cargo_chirho.write_text("#!/bin/sh\nprintf '%s\\n' \"$CARGO_TARGET_DIR\" \"$TMPDIR\" > '" + str(self.received_chirho) + "'\nexit 7\n")
        self.cargo_chirho.chmod(0o755)

    def select_chirho(self, worktree_chirho=None, profile_chirho="cargo-dev-chirho", minimum_chirho=0):
        with mock_chirho.patch.object(selection_chirho.PathChirho, "home", return_value=self.root_chirho):
            return selection_chirho.local_cache_chirho(worktree_chirho or self.worktree_chirho, profile_chirho, minimum_chirho)

    def invoke_chirho(self, arguments_chirho, selection_chirho_value=None):
        stdout_chirho, stderr_chirho = io_chirho.StringIO(), io_chirho.StringIO()
        chosen_chirho = selection_chirho_value or self.select_chirho()
        with contextlib_chirho.redirect_stdout(stdout_chirho), contextlib_chirho.redirect_stderr(stderr_chirho), mock_chirho.patch.dict(os_chirho.environ, {"PATH": str(self.bin_chirho)}, clear=False):
            code_chirho = cargo_build_chirho.main_chirho(["--worktree-chirho", str(self.worktree_chirho), *arguments_chirho], lambda *values_chirho: chosen_chirho)
        return code_chirho, stdout_chirho.getvalue(), stderr_chirho.getvalue()

    def test_worktrees_and_profiles_are_distinct_and_selection_is_stable_chirho(self):
        first_chirho = self.select_chirho()
        self.assertEqual(first_chirho["cache_root_chirho"], self.select_chirho()["cache_root_chirho"])
        self.assertNotEqual(first_chirho["cache_root_chirho"], self.select_chirho(profile_chirho="cargo-release-chirho")["cache_root_chirho"])
        other_chirho = self.root_chirho / "other-source-chirho"
        other_chirho.mkdir()
        self.assertNotEqual(first_chirho["cache_root_chirho"], self.select_chirho(worktree_chirho=other_chirho)["cache_root_chirho"])

    def test_missing_shared_helper_selects_local_without_touching_a_volume_chirho(self):
        with mock_chirho.patch.object(selection_chirho.shutil_chirho, "which", return_value=None), mock_chirho.patch.object(selection_chirho.PathChirho, "home", return_value=self.root_chirho), mock_chirho.patch.object(selection_chirho.PathChirho, "is_file", return_value=False):
            chosen_chirho = selection_chirho.choose_chirho(self.worktree_chirho, "cargo-dev-chirho", 0)
        self.assertEqual(chosen_chirho["backend_chirho"], "local_chirho")
        self.assertEqual(chosen_chirho["reason_chirho"], "shared_selector_not_installed_chirho")
        self.assertFalse(PathChirho(chosen_chirho["cache_root_chirho"]).is_relative_to(PathChirho("/Volumes")))

    def test_malformed_shared_response_refuses_before_paths_are_used_chirho(self):
        response_chirho = subprocess_chirho.CompletedProcess([], 0, stdout=b"[]", stderr=b"")
        with mock_chirho.patch.object(selection_chirho.shutil_chirho, "which", return_value="shared-selector-chirho"), mock_chirho.patch.object(selection_chirho.subprocess_chirho, "run", return_value=response_chirho):
            with self.assertRaises(selection_chirho.CacheErrorChirho):
                selection_chirho.choose_chirho(self.worktree_chirho, "cargo-dev-chirho", 0)

    def test_unmarked_cache_is_never_adopted_and_path_traversal_declines_chirho(self):
        chosen_chirho = self.select_chirho()
        root_chirho = PathChirho(chosen_chirho["cache_root_chirho"])
        (root_chirho / selection_chirho.MARKER_CHIRHO).unlink()
        (root_chirho / "kept-chirho").write_text("unique-chirho")
        with self.assertRaises(selection_chirho.CacheErrorChirho):
            self.select_chirho()
        self.assertFalse((root_chirho / selection_chirho.MARKER_CHIRHO).exists())
        self.assertEqual((root_chirho / "kept-chirho").read_text(), "unique-chirho")
        with self.assertRaises(selection_chirho.CacheErrorChirho):
            self.select_chirho(profile_chirho="../other-chirho")

    def test_marker_drift_and_symlinks_refuse_chirho(self):
        chosen_chirho = self.select_chirho()
        guard_chirho = monitor_chirho.StorageGuardChirho(chosen_chirho)
        marker_chirho = guard_chirho.root_chirho / selection_chirho.MARKER_CHIRHO
        marker_chirho.write_text("{}")
        self.assertFalse(guard_chirho.valid_chirho())
        marker_chirho.unlink()
        marker_chirho.symlink_to(self.worktree_chirho / "Cargo.toml")
        with self.assertRaises(OSError):
            monitor_chirho.StorageGuardChirho(chosen_chirho)

    def test_selection_only_needs_no_compiler_or_build_floor_chirho(self):
        code_chirho, stdout_chirho, _ignored_chirho = self.invoke_chirho(["--select-only-chirho", "--min-free-gib-chirho", "0"])
        self.assertEqual(code_chirho, 0)
        result_chirho = json_chirho.loads(stdout_chirho)
        self.assertEqual(result_chirho["backend_chirho"], "local_chirho")
        self.assertFalse(PathChirho(result_chirho["target_directory_chirho"]).exists())

    def test_target_overrides_refuse_but_runtime_flags_are_not_cargo_flags_chirho(self):
        for flag_chirho in ["--target-dir=other-chirho", "--config=build.target-dir='other-chirho'"]:
            code_chirho, _ignored_chirho, _ignored_chirho = self.invoke_chirho(["--", "check", flag_chirho])
            self.assertEqual(code_chirho, 74)
        self.assertEqual(cargo_build_chirho.profile_chirho(["test", "--release"]), "cargo-release-chirho")
        self.assertEqual(cargo_build_chirho.profile_chirho(["build", "--profile", "release"]), "cargo-release-chirho")

    def test_initial_capacity_refuses_before_any_child_chirho(self):
        with mock_chirho.patch.object(cargo_build_chirho.shutil_chirho, "disk_usage", return_value=type("SpaceChirho", (), {"free": 0})()), mock_chirho.patch.object(cargo_build_chirho.subprocess_chirho, "Popen") as spawn_chirho:
            code_chirho, _ignored_chirho, _ignored_chirho = self.invoke_chirho(["--", "check"])
        self.assertEqual(code_chirho, 74)
        spawn_chirho.assert_not_called()

    def test_single_writer_refuses_a_second_owner_chirho(self):
        chosen_chirho = self.select_chirho()
        with (PathChirho(chosen_chirho["cache_root_chirho"]) / ".cargo-build-lock-chirho").open("w") as lock_chirho, mock_chirho.patch.object(cargo_build_chirho.shutil_chirho, "disk_usage", return_value=type("SpaceChirho", (), {"free": 1 << 40})()):
            fcntl_chirho.flock(lock_chirho, fcntl_chirho.LOCK_EX | fcntl_chirho.LOCK_NB)
            code_chirho, _ignored_chirho, stderr_chirho = self.invoke_chirho(["--", "check"], chosen_chirho)
        self.assertEqual(code_chirho, 74)
        self.assertIn("already has an owned build", stderr_chirho)

    def test_child_receives_selected_paths_and_its_exit_is_preserved_chirho(self):
        chosen_chirho = self.select_chirho()
        with mock_chirho.patch.object(cargo_build_chirho.shutil_chirho, "disk_usage", return_value=type("SpaceChirho", (), {"free": 1 << 40})()):
            code_chirho, _ignored_chirho, stderr_chirho = self.invoke_chirho(["--", "check"], chosen_chirho)
        self.assertEqual(code_chirho, 7)
        self.assertIn('"reason_chirho": "completed_chirho"', stderr_chirho)
        target_chirho, temporary_chirho = self.received_chirho.read_text().splitlines()
        self.assertEqual(target_chirho, str(PathChirho(chosen_chirho["cache_root_chirho"]) / "target-chirho"))
        self.assertTrue(PathChirho(temporary_chirho).is_relative_to(PathChirho(chosen_chirho["temp_root_chirho"])))
        self.assertFalse(PathChirho(temporary_chirho).exists())
        self.assertEqual(list(PathChirho(chosen_chirho["temp_root_chirho"]).iterdir()), [])

    def test_runtime_tests_keep_private_local_temp_chirho(self):
        chosen_chirho = self.select_chirho()
        recorded_chirho = {}

        def owned_chirho(argv_chirho, worktree_chirho, environment_chirho, guard_chirho):
            recorded_chirho.update(environment_chirho)
            self.assertTrue(guard_chirho.valid_chirho())
            return 0, "completed_chirho"

        with mock_chirho.patch.object(cargo_build_chirho.shutil_chirho, "disk_usage", return_value=type("SpaceChirho", (), {"free": 1 << 40})()), mock_chirho.patch.object(cargo_build_chirho, "run_owned_chirho", side_effect=owned_chirho):
            code_chirho, _ignored_chirho, _ignored_chirho = self.invoke_chirho(["--", "test"])
        self.assertEqual(code_chirho, 0)
        self.assertFalse(PathChirho(recorded_chirho["TMPDIR"]).is_relative_to(PathChirho(chosen_chirho["temp_root_chirho"])))
        self.assertFalse(PathChirho(recorded_chirho["TMPDIR"]).is_relative_to(PathChirho("/Volumes")))

    def test_disappearance_stops_only_the_owned_group_without_retry_chirho(self):
        chosen_chirho = self.select_chirho()
        guard_chirho = monitor_chirho.StorageGuardChirho(chosen_chirho)
        counter_chirho = self.root_chirho / "started-chirho.txt"
        unrelated_chirho = subprocess_chirho.Popen([sys_chirho.executable, "-c", "import time; time.sleep(20)"], start_new_session=True)
        checks_chirho = 0

        def valid_chirho():
            nonlocal checks_chirho
            checks_chirho += 1
            return checks_chirho < 4

        source_chirho = "import os,time,pathlib; pathlib.Path(" + repr(str(counter_chirho)) + ").write_text(str(os.getpgrp())); time.sleep(20)"
        try:
            with mock_chirho.patch.object(guard_chirho, "valid_chirho", side_effect=valid_chirho), mock_chirho.patch.object(monitor_chirho.shutil_chirho, "disk_usage", return_value=type("SpaceChirho", (), {"free": 1 << 40})()):
                code_chirho, reason_chirho = monitor_chirho.run_owned_chirho([sys_chirho.executable, "-c", source_chirho], self.worktree_chirho, dict(os_chirho.environ), guard_chirho)
            self.assertEqual((code_chirho, reason_chirho), (74, "selected_backend_disappeared_chirho"))
            self.assertIsNone(unrelated_chirho.poll())
            with self.assertRaises(ProcessLookupError):
                os_chirho.killpg(int(counter_chirho.read_text()), 0)
        finally:
            os_chirho.killpg(unrelated_chirho.pid, signal_chirho.SIGKILL)
            unrelated_chirho.wait()

    def test_runtime_reserve_loss_stops_the_owned_child_chirho(self):
        guard_chirho = monitor_chirho.StorageGuardChirho(self.select_chirho())
        high_chirho = type("SpaceChirho", (), {"free": 1 << 40})()
        low_chirho = type("SpaceChirho", (), {"free": 0})()
        with mock_chirho.patch.object(monitor_chirho.shutil_chirho, "disk_usage", side_effect=[high_chirho, high_chirho, low_chirho, low_chirho]):
            code_chirho, reason_chirho = monitor_chirho.run_owned_chirho([sys_chirho.executable, "-c", "import time; time.sleep(20)"], self.worktree_chirho, dict(os_chirho.environ), guard_chirho)
        self.assertEqual((code_chirho, reason_chirho), (74, "cache_reserve_exhausted_chirho"))

    def test_termination_of_the_wrapper_cleans_its_child_group_chirho(self):
        marker_chirho = self.root_chirho / "signal-child-chirho.txt"
        probe_chirho = self.root_chirho / "signal-probe-chirho.py"
        modules_chirho = PathChirho(__file__).resolve().parent
        child_chirho = "import os as os_chirho,time as time_chirho,pathlib as pathlib_chirho; pathlib_chirho.Path(" + repr(str(marker_chirho)) + ").write_text(str(os_chirho.getpgrp())); time_chirho.sleep(20)"
        probe_chirho.write_text(
            "import sys as sys_chirho,os as os_chirho,types as types_chirho,pathlib as pathlib_chirho\n"
            + "sys_chirho.path.insert(0," + repr(str(modules_chirho)) + ")\n"
            + "import monitor_chirho\n"
            + "monitor_chirho.shutil_chirho.disk_usage=lambda path_chirho: types_chirho.SimpleNamespace(free=1<<40)\n"
            + "guard_chirho=types_chirho.SimpleNamespace(root_chirho=pathlib_chirho.Path('.'),valid_chirho=lambda: True)\n"
            + "try:\n monitor_chirho.run_owned_chirho([sys_chirho.executable,'-c'," + repr(child_chirho) + "],'.',dict(os_chirho.environ),guard_chirho)\n"
            + "except monitor_chirho.OwnedSignalChirho as stop_chirho:\n sys_chirho.exit(128+stop_chirho.number_chirho)\n"
        )
        process_chirho = subprocess_chirho.Popen([sys_chirho.executable, "-B", str(probe_chirho)], cwd=self.root_chirho, start_new_session=True)
        try:
            for _attempt_chirho in range(40):
                if marker_chirho.exists():
                    break
                time_chirho.sleep(0.05)
            self.assertTrue(marker_chirho.exists(), "owned child never started")
            os_chirho.kill(process_chirho.pid, signal_chirho.SIGTERM)
            self.assertEqual(process_chirho.wait(timeout=5), 143)
            with self.assertRaises(ProcessLookupError):
                os_chirho.killpg(int(marker_chirho.read_text()), 0)
        finally:
            if marker_chirho.exists():
                try:
                    os_chirho.killpg(int(marker_chirho.read_text()), signal_chirho.SIGKILL)
                except ProcessLookupError:
                    pass
            monitor_chirho.stop_group_chirho(process_chirho)


if __name__ == "__main__":
    unittest_chirho.main()
