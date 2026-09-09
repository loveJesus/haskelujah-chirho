// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded execution for generated programs: drain both pipes, retain diagnostics,
//! close stdin, and reap the owned process group on success or failure.

use std::io::{Read, Write};
use std::process::{Command, Output, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

const OUTPUT_LIMIT_CHIRHO: usize = 2 * 1024 * 1024;

fn drain_chirho(
    mut pipe_chirho: impl Read,
    overflow_chirho: Arc<AtomicBool>,
) -> std::io::Result<Vec<u8>> {
    let mut output_chirho = Vec::new();
    let mut buffer_chirho = [0; 8192];
    loop {
        let count_chirho = pipe_chirho.read(&mut buffer_chirho)?;
        if count_chirho == 0 {
            return Ok(output_chirho);
        }
        let remaining_chirho = OUTPUT_LIMIT_CHIRHO - output_chirho.len();
        output_chirho.extend_from_slice(&buffer_chirho[..count_chirho.min(remaining_chirho)]);
        if count_chirho > remaining_chirho {
            overflow_chirho.store(true, Ordering::Relaxed);
        }
    }
}

/// Run only commands owned by the current test. Output and elapsed time are
/// bounded independently; a child that never reads stdin cannot block the timer.
pub fn run_bounded_chirho(
    command_chirho: &mut Command,
    input_chirho: &[u8],
    timeout_chirho: Duration,
) -> Result<Output, String> {
    run_bounded_with_memory_chirho(command_chirho, input_chirho, timeout_chirho, None)
}

/// The resident/physical-footprint ceiling is sampled on macOS, whose setrlimit
/// rejects RLIMIT_AS. Footprint includes compressed pages: a swapped-out runaway
/// can have tiny RSS while still consuming tens of gigabytes of memory.
/// Other Unix platforms install a hard address-space limit at spawn. A requested
/// limit on an unsupported platform fails explicitly instead of running unbounded.
pub fn run_bounded_with_memory_chirho(
    command_chirho: &mut Command,
    input_chirho: &[u8],
    timeout_chirho: Duration,
    resident_limit_chirho: Option<u64>,
) -> Result<Output, String> {
    #[cfg(all(unix, not(target_os = "macos")))]
    if let Some(limit_chirho) = resident_limit_chirho {
        use std::os::unix::process::CommandExt;
        let limit_chirho = libc::rlim_t::try_from(limit_chirho)
            .map_err(|error_chirho| format!("invalid address-space limit: {error_chirho}"))?;
        // SAFETY: the post-fork hook only calls setrlimit and captures a scalar;
        // it neither allocates nor takes a lock before exec.
        unsafe {
            command_chirho.pre_exec(move || {
                let limit_chirho = libc::rlimit {
                    rlim_cur: limit_chirho,
                    rlim_max: limit_chirho,
                };
                if libc::setrlimit(libc::RLIMIT_AS, &limit_chirho) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    }
    #[cfg(not(unix))]
    if resident_limit_chirho.is_some() {
        return Err("child memory limits are not supported on this platform".to_string());
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command_chirho.process_group(0);
    }
    let description_chirho = format!("{command_chirho:?}");
    let mut child_chirho = command_chirho
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error_chirho| format!("{description_chirho}: {error_chirho}"))?;
    let mut stdin_chirho = child_chirho.stdin.take().expect("requested stdin pipe");
    let stdout_chirho = child_chirho.stdout.take().expect("requested stdout pipe");
    let stderr_chirho = child_chirho.stderr.take().expect("requested stderr pipe");
    let input_chirho = input_chirho.to_vec();
    let input_writer_chirho = thread::spawn(move || stdin_chirho.write_all(&input_chirho));
    let overflow_chirho = Arc::new(AtomicBool::new(false));
    let stdout_overflow_chirho = overflow_chirho.clone();
    let stderr_overflow_chirho = overflow_chirho.clone();
    let stdout_reader_chirho =
        thread::spawn(move || drain_chirho(stdout_chirho, stdout_overflow_chirho));
    let stderr_reader_chirho =
        thread::spawn(move || drain_chirho(stderr_chirho, stderr_overflow_chirho));
    let start_chirho = Instant::now();
    let mut failure_chirho = None;
    let status_chirho = loop {
        match child_chirho.try_wait() {
            Ok(Some(status_chirho)) => break Some(status_chirho),
            Err(error_chirho) => {
                failure_chirho = Some(error_chirho.to_string());
                break None;
            }
            Ok(None) => {}
        }
        if overflow_chirho.load(Ordering::Relaxed) {
            failure_chirho = Some("output limit exceeded".to_string());
            break None;
        }
        #[cfg(target_os = "macos")]
        if let Some(limit_chirho) = resident_limit_chirho {
            let mut usage_chirho = std::mem::MaybeUninit::<libc::rusage_info_v2>::zeroed();
            let status_chirho = unsafe {
                libc::proc_pid_rusage(
                    child_chirho.id() as i32,
                    libc::RUSAGE_INFO_V2,
                    usage_chirho.as_mut_ptr().cast(),
                )
            };
            if status_chirho == 0 {
                let usage_chirho = unsafe { usage_chirho.assume_init() };
                if usage_chirho
                    .ri_resident_size
                    .max(usage_chirho.ri_phys_footprint)
                    > limit_chirho
                {
                    failure_chirho = Some(format!(
                        "resident/physical memory limit exceeded ({limit_chirho} bytes)"
                    ));
                    break None;
                }
            }
            if status_chirho != 0
                && std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
            {
                failure_chirho = Some(format!(
                    "cannot measure child memory: {}",
                    std::io::Error::last_os_error()
                ));
                break None;
            }
        }
        if start_chirho.elapsed() >= timeout_chirho {
            failure_chirho = Some(format!("timed out after {timeout_chirho:?}"));
            break None;
        }
        thread::sleep(Duration::from_millis(10));
    };
    // Each command owns a new process group. Descendants must not retain pipes
    // after the leader exits, nor survive a timeout of their generated program.
    #[cfg(unix)]
    if let Ok(group_chirho) = i32::try_from(child_chirho.id())
        && group_chirho > 0
    {
        unsafe {
            libc::kill(-group_chirho, libc::SIGKILL);
        }
    }
    if status_chirho.is_none() {
        let _ = child_chirho.kill();
        let _ = child_chirho.wait();
    }
    let stdout_chirho = stdout_reader_chirho
        .join()
        .map_err(|_| "stdout reader panicked".to_string())?
        .map_err(|error_chirho| error_chirho.to_string())?;
    let stderr_chirho = stderr_reader_chirho
        .join()
        .map_err(|_| "stderr reader panicked".to_string())?
        .map_err(|error_chirho| error_chirho.to_string())?;
    let input_result_chirho = input_writer_chirho
        .join()
        .map_err(|_| "stdin writer panicked".to_string())?;
    if overflow_chirho.load(Ordering::Relaxed) && failure_chirho.is_none() {
        failure_chirho = Some("output limit exceeded".to_string());
    }
    if let Some(failure_chirho) = failure_chirho {
        return Err(format!(
            "{description_chirho}: {failure_chirho}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&stdout_chirho),
            String::from_utf8_lossy(&stderr_chirho)
        ));
    }
    if let Err(error_chirho) = input_result_chirho
        && error_chirho.kind() != std::io::ErrorKind::BrokenPipe
    {
        return Err(format!(
            "{description_chirho}: writing stdin: {error_chirho}"
        ));
    }
    Ok(Output {
        status: status_chirho.expect("completed child"),
        stdout: stdout_chirho,
        stderr: stderr_chirho,
    })
}
