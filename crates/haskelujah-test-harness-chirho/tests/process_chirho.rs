// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

#![cfg(unix)]

use haskelujah_test_harness::process_chirho::run_bounded_chirho;
use std::process::Command;
use std::time::{Duration, Instant};

#[test]
fn captures_both_pipes_and_preserves_nonzero_status_chirho() {
    let output_chirho = run_bounded_chirho(
        Command::new("sh").args([
            "-c",
            "printf 'output-chirho'; printf 'error-chirho' >&2; exit 7",
        ]),
        &[],
        Duration::from_secs(5),
    )
    .unwrap();
    assert_eq!(output_chirho.status.code(), Some(7));
    assert_eq!(output_chirho.stdout, b"output-chirho");
    assert_eq!(output_chirho.stderr, b"error-chirho");
}

#[test]
fn stdin_is_closed_and_an_echoing_child_finishes_chirho() {
    let output_chirho = run_bounded_chirho(
        &mut Command::new("cat"),
        b"input-chirho\n",
        Duration::from_secs(5),
    )
    .unwrap();
    assert!(output_chirho.status.success());
    assert_eq!(output_chirho.stdout, b"input-chirho\n");
}

#[test]
fn timeout_retains_partial_output_and_reaps_descendants_chirho() {
    let start_chirho = Instant::now();
    let error_chirho = run_bounded_chirho(
        Command::new("sh").args(["-c", "printf 'before-timeout-chirho'; sleep 30 & wait"]),
        &[],
        Duration::from_millis(100),
    )
    .unwrap_err();
    assert!(error_chirho.contains("timed out"), "{error_chirho}");
    assert!(
        error_chirho.contains("before-timeout-chirho"),
        "{error_chirho}"
    );
    assert!(start_chirho.elapsed() < Duration::from_secs(5));
}

#[test]
fn a_child_that_never_reads_stdin_cannot_block_the_timeout_chirho() {
    let start_chirho = Instant::now();
    let error_chirho = run_bounded_chirho(
        Command::new("sh").args(["-c", "exec sleep 30"]),
        &vec![b'x'; 1024 * 1024],
        Duration::from_millis(100),
    )
    .unwrap_err();
    assert!(error_chirho.contains("timed out"), "{error_chirho}");
    assert!(start_chirho.elapsed() < Duration::from_secs(5));
}

#[test]
fn unbounded_output_is_reported_instead_of_exhausting_memory_chirho() {
    let error_chirho =
        run_bounded_chirho(&mut Command::new("yes"), &[], Duration::from_secs(5)).unwrap_err();
    assert!(
        error_chirho.contains("output limit exceeded"),
        "{error_chirho}"
    );
    assert!(error_chirho.len() < 3 * 1024 * 1024);
}

#[cfg(target_os = "macos")]
#[test]
fn macos_resident_limit_is_measured_and_enforced_chirho() {
    let error_chirho = haskelujah_test_harness::process_chirho::run_bounded_with_memory_chirho(
        Command::new("sh").args(["-c", "exec sleep 30"]),
        &[],
        Duration::from_secs(5),
        Some(1024),
    )
    .unwrap_err();
    assert!(
        error_chirho.contains("resident/physical memory limit exceeded"),
        "{error_chirho}"
    );
}

#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn address_space_limit_rejects_allocation_in_the_child_chirho() {
    const CHILD_KEY_CHIRHO: &str = "PROCESS_MEMORY_CHILD_CHIRHO";
    if std::env::var_os(CHILD_KEY_CHIRHO).is_some() {
        let mut bytes_chirho = Vec::<u8>::new();
        assert!(
            bytes_chirho.try_reserve_exact(1024 * 1024 * 1024).is_err(),
            "one GiB reservation must exceed the child address-space limit"
        );
        eprintln!("ALLOCATION_REJECTED_CHIRHO");
        return;
    }
    let output_chirho = haskelujah_test_harness::process_chirho::run_bounded_with_memory_chirho(
        Command::new(std::env::current_exe().expect("current test executable"))
            .args([
                "--exact",
                "address_space_limit_rejects_allocation_in_the_child_chirho",
                "--nocapture",
            ])
            .env(CHILD_KEY_CHIRHO, "1"),
        &[],
        Duration::from_secs(5),
        Some(512 * 1024 * 1024),
    )
    .expect("bounded child finishes");
    assert!(
        output_chirho.status.success(),
        "{}",
        String::from_utf8_lossy(&output_chirho.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output_chirho.stderr).contains("ALLOCATION_REJECTED_CHIRHO"),
        "the child must reach the allocation test, not merely fail to start"
    );
}
