// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Explicit selection; an unsupported next command never delegates to legacy.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use super::FlagsChirho;
use std::{path::Path, process::ExitCode};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum PipelineChirho {
    LegacyChirho,
    NextChirho,
}

impl PipelineChirho {
    pub(super) fn parse_chirho(value_chirho: &str) -> Option<Self> {
        match value_chirho {
            "legacy" => Some(Self::LegacyChirho),
            "next" => Some(Self::NextChirho),
            _ => None,
        }
    }
}

pub(super) fn command_chirho(positional_chirho: &[&str], flags_chirho: &FlagsChirho) -> ExitCode {
    let command_chirho = positional_chirho.first().copied();
    if !matches!(command_chirho, Some("check" | "run")) || positional_chirho.len() != 2 {
        eprintln!("next supports: check <module.hs> [--dump-core], run <Main.hs>");
        return ExitCode::from(2);
    }
    if flags_chirho.emit_llvm_chirho
        || flags_chirho.emit_cranelift_chirho
        || flags_chirho.emit_wasm_chirho
        || flags_chirho.dump_llvm_chirho
        || flags_chirho.dump_stg_chirho
        || flags_chirho.output_path_chirho.is_some()
    {
        eprintln!("these backend flags are not supported by next check/run");
        return ExitCode::from(2);
    }
    if command_chirho == Some("run") {
        if flags_chirho.dump_core_chirho {
            eprintln!("use next check --dump-core before run");
            return ExitCode::from(2);
        }
        return run_chirho(Path::new(positional_chirho[1]));
    }
    let result_chirho = if flags_chirho.dump_core_chirho {
        haskelujah_driver_chirho::next_chirho::compile_source_file_chirho(Path::new(
            positional_chirho[1],
        ))
        .map(|module_chirho| {
            eprintln!("{module_chirho:#?}");
        })
    } else {
        haskelujah_driver_chirho::next_chirho::check_source_file_chirho(Path::new(
            positional_chirho[1],
        ))
        .map(|_| ())
    };
    match result_chirho {
        Ok(()) => {
            println!("next: checked {}", positional_chirho[1]);
            ExitCode::SUCCESS
        }
        Err(error_chirho) => {
            eprintln!("{}: {error_chirho}", positional_chirho[1]);
            ExitCode::from(1)
        }
    }
}

fn run_chirho(path_chirho: &Path) -> ExitCode {
    use std::io::Write;
    match haskelujah_driver_chirho::next_chirho::run_main_file_chirho(
        path_chirho,
        Default::default(),
    ) {
        Err(error_chirho) => {
            eprintln!("{}: {error_chirho}", path_chirho.display());
            run_failure_code_chirho(&error_chirho)
        }
        Ok(outcome_chirho) => {
            let mut stdout_chirho = std::io::stdout().lock();
            let output_chirho = stdout_chirho
                .write_all(outcome_chirho.machine_chirho.io_output_chirho.as_bytes())
                .and_then(|()| stdout_chirho.flush());
            // A handler's failed output flush cannot replace the program's
            // original exception. Successful actions may exit quietly on EPIPE.
            if let Err(error_chirho) = outcome_chirho.result_chirho {
                eprintln!("{}: {error_chirho}", path_chirho.display());
                return run_failure_code_chirho(&error_chirho);
            }
            if let Err(error_chirho) = output_chirho
                && error_chirho.kind() != std::io::ErrorKind::BrokenPipe
            {
                eprintln!("{}: output error: {error_chirho}", path_chirho.display());
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
    }
}

fn run_failure_code_chirho(
    error_chirho: &haskelujah_driver_chirho::next_chirho::NextErrorChirho,
) -> ExitCode {
    use haskelujah_driver_chirho::next_chirho::NextErrorKindChirho;
    ExitCode::from(match error_chirho.kind_chirho {
        NextErrorKindChirho::ResourceChirho => 75,
        NextErrorKindChirho::UnsupportedChirho => 64,
        _ => 1,
    })
}
