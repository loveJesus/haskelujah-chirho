// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The native failure route, observed from outside the process: each case runs
//! in a child copy of this test binary and must exit 1, not by a signal, with
//! exactly one prefixed report on stderr after everything it wrote to stdout.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

use haskelujah_rts::ffi_chirho::{
    haskelujah_alloc_chirho, haskelujah_alloc_thunk_chirho, haskelujah_enter_thunk_chirho,
    haskelujah_error_chirho, haskelujah_gc_collect_chirho, haskelujah_is_heap_ptr_chirho,
    haskelujah_no_match_chirho, haskelujah_print_int_chirho, haskelujah_show_int_chirho,
    haskelujah_unbound_thunk_chirho,
};
use std::ffi::{CStr, c_char, c_int};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

const CHILD_CASE_ENV_CHIRHO: &str = "HASKELUJAH_FAILURE_CASE_CHIRHO";
const UNBOUND_NAME_CHIRHO: &[u8] = b"scanl1";

unsafe extern "C" {
    fn printf(format_chirho: *const c_char, ...) -> c_int;
}

static SELF_THUNK_CHIRHO: AtomicU64 = AtomicU64::new(0);
// In-process cases share the global heap; a collection must not free another
// case's unrooted result.
static IN_PROCESS_LOCK_CHIRHO: Mutex<()> = Mutex::new(());

unsafe extern "C" fn return_seven_chirho(_environment_chirho: *const u64) -> u64 {
    7
}

unsafe extern "C" fn enter_self_chirho(_environment_chirho: *const u64) -> u64 {
    haskelujah_enter_thunk_chirho(SELF_THUNK_CHIRHO.load(Ordering::SeqCst))
}

fn new_thunk_chirho(entry_chirho: unsafe extern "C" fn(*const u64) -> u64) -> u64 {
    // SAFETY: the entry has the thunk signature and reads no environment.
    unsafe { haskelujah_alloc_thunk_chirho(entry_chirho as *const () as u64, 0, std::ptr::null()) }
}

/// Runs only inside a child started by `run_case_chirho`; a no-op otherwise.
#[test]
fn failure_child_chirho() {
    let Ok(case_chirho) = std::env::var(CHILD_CASE_ENV_CHIRHO) else {
        return;
    };
    match case_chirho.as_str() {
        "user-error" => {
            haskelujah_print_int_chirho(7);
            // C stdio is fully buffered on a file: only the route's flush
            // puts this before the report.
            // SAFETY: a NUL-terminated format with no conversions.
            unsafe {
                printf(c"partial".as_ptr());
            }
            let message_chirho = b"boom";
            // SAFETY: four readable bytes.
            unsafe { haskelujah_error_chirho(message_chirho.as_ptr(), 4) }
        }
        "internal" => {
            let thunk_chirho = new_thunk_chirho(return_seven_chirho);
            // SAFETY: the thunk's state word, which no valid state uses as 7.
            unsafe { ((thunk_chirho & !1) as usize as *mut u64).write(7) };
            haskelujah_enter_thunk_chirho(thunk_chirho);
        }
        "unbound" => {
            // SAFETY: a static name.
            let thunk_chirho = unsafe {
                haskelujah_unbound_thunk_chirho(
                    UNBOUND_NAME_CHIRHO.as_ptr(),
                    UNBOUND_NAME_CHIRHO.len() as u64,
                )
            };
            haskelujah_enter_thunk_chirho(thunk_chirho);
        }
        "boxed-show" => {
            let boxed_chirho = (haskelujah_alloc_chirho(16) as u64) | 1;
            haskelujah_show_int_chirho(boxed_chirho as i64);
        }
        "no-match" => haskelujah_no_match_chirho(),
        "loop" => {
            let thunk_chirho = new_thunk_chirho(enter_self_chirho);
            SELF_THUNK_CHIRHO.store(thunk_chirho, Ordering::SeqCst);
            haskelujah_enter_thunk_chirho(thunk_chirho);
        }
        other_chirho => panic!("unknown failure case {other_chirho}"),
    }
    unreachable!("case {case_chirho} returned instead of failing");
}

struct ChildOutcomeChirho {
    code_chirho: Option<i32>,
    stdout_chirho: String,
    stderr_chirho: String,
}

fn child_command_chirho(case_chirho: &str) -> Command {
    let mut command_chirho = Command::new(std::env::current_exe().expect("test binary path"));
    command_chirho
        .args([
            "--exact",
            "failure_child_chirho",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD_CASE_ENV_CHIRHO, case_chirho);
    command_chirho
}

fn run_case_chirho(case_chirho: &str) -> ChildOutcomeChirho {
    let output_chirho = child_command_chirho(case_chirho)
        .output()
        .expect("spawn the failure child");
    ChildOutcomeChirho {
        code_chirho: output_chirho.status.code(),
        stdout_chirho: String::from_utf8_lossy(&output_chirho.stdout).into_owned(),
        stderr_chirho: String::from_utf8_lossy(&output_chirho.stderr).into_owned(),
    }
}

fn assert_reports_chirho(case_chirho: &str, report_chirho: &str) -> ChildOutcomeChirho {
    let outcome_chirho = run_case_chirho(case_chirho);
    // `code()` is None for a signal, so this also rules out abort().
    assert_eq!(outcome_chirho.code_chirho, Some(1), "case {case_chirho}");
    assert_eq!(
        outcome_chirho.stderr_chirho, report_chirho,
        "case {case_chirho}"
    );
    outcome_chirho
}

#[test]
fn program_failure_flushes_stdout_and_reports_the_exact_payload_chirho() {
    let outcome_chirho = assert_reports_chirho("user-error", "haskelujah: error: boom\n");
    assert!(
        outcome_chirho.stdout_chirho.ends_with("7\npartial"),
        "{:?}",
        outcome_chirho.stdout_chirho
    );
}

#[test]
fn the_report_follows_buffered_c_output_on_one_stream_chirho() {
    let merged_path_chirho = std::env::temp_dir().join(format!(
        "haskelujah-failure-merged-{}-chirho.txt",
        std::process::id()
    ));
    let merged_file_chirho = std::fs::File::create(&merged_path_chirho).expect("merged file");
    let status_chirho = child_command_chirho("user-error")
        .stdout(Stdio::from(merged_file_chirho.try_clone().expect("clone")))
        .stderr(Stdio::from(merged_file_chirho))
        .status()
        .expect("spawn the failure child");
    let merged_chirho = std::fs::read_to_string(&merged_path_chirho).expect("read merged");
    let _ = std::fs::remove_file(&merged_path_chirho);
    assert_eq!(status_chirho.code(), Some(1));
    assert!(
        merged_chirho.ends_with("7\npartialhaskelujah: error: boom\n"),
        "{merged_chirho:?}"
    );
}

#[test]
fn a_violated_runtime_invariant_uses_the_internal_prefix_chirho() {
    assert_reports_chirho(
        "internal",
        "haskelujah: internal error: corrupt native thunk state\n",
    );
}

#[test]
fn an_unbound_reference_fails_named_when_demanded_chirho() {
    assert_reports_chirho(
        "unbound",
        "haskelujah: error: missing native binding `scanl1`\n",
    );
}

#[test]
fn an_integer_formatter_refuses_a_live_heap_value_chirho() {
    assert_reports_chirho(
        "boxed-show",
        "haskelujah: error: show: no native Show instance for a boxed value\n",
    );
}

#[test]
fn a_case_default_fails_named_chirho() {
    assert_reports_chirho("no-match", "haskelujah: error: no matching alternative\n");
}

#[test]
fn a_thunk_that_demands_itself_reports_loop_chirho() {
    assert_reports_chirho("loop", "haskelujah: error: <<loop>>\n");
}

#[test]
fn unbound_thunks_are_shared_per_name_and_survive_collection_chirho() {
    let _guard_chirho = IN_PROCESS_LOCK_CHIRHO
        .lock()
        .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
    // SAFETY: a static name.
    let first_chirho = unsafe {
        haskelujah_unbound_thunk_chirho(
            UNBOUND_NAME_CHIRHO.as_ptr(),
            UNBOUND_NAME_CHIRHO.len() as u64,
        )
    };
    haskelujah_gc_collect_chirho();
    // SAFETY: the same static name.
    let second_chirho = unsafe {
        haskelujah_unbound_thunk_chirho(
            UNBOUND_NAME_CHIRHO.as_ptr(),
            UNBOUND_NAME_CHIRHO.len() as u64,
        )
    };
    assert_eq!(first_chirho, second_chirho);
    assert_eq!(haskelujah_is_heap_ptr_chirho(first_chirho), 1);
}

#[test]
fn odd_immediates_still_format_as_integers_chirho() {
    let _guard_chirho = IN_PROCESS_LOCK_CHIRHO
        .lock()
        .unwrap_or_else(|poisoned_chirho| poisoned_chirho.into_inner());
    for (value_chirho, text_chirho) in [
        (3_i64, "3"),
        (-1, "-1"),
        (i64::MIN + 1, "-9223372036854775807"),
    ] {
        let shown_chirho = haskelujah_show_int_chirho(value_chirho);
        // SAFETY: show returns a NUL-terminated heap string.
        let shown_text_chirho = unsafe { CStr::from_ptr(shown_chirho as usize as *const c_char) };
        assert_eq!(shown_text_chirho.to_bytes(), text_chirho.as_bytes());
    }
}
