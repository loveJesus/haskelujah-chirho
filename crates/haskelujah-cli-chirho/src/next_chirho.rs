// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Explicit selection; an unsupported next command never delegates to legacy.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use super::FlagsChirho;
use std::{fs, process::ExitCode};

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
    if positional_chirho.first() != Some(&"check") || positional_chirho.len() != 2 {
        eprintln!(
            "next currently supports: check <library-module.hs> [--dump-core]; IO programs require the source-library slice"
        );
        return ExitCode::from(2);
    }
    if flags_chirho.emit_llvm_chirho
        || flags_chirho.emit_cranelift_chirho
        || flags_chirho.emit_wasm_chirho
        || flags_chirho.dump_llvm_chirho
        || flags_chirho.dump_stg_chirho
        || flags_chirho.output_path_chirho.is_some()
    {
        eprintln!("these backend flags are not supported by next check");
        return ExitCode::from(2);
    }
    let source_chirho = match fs::read_to_string(positional_chirho[1]) {
        Ok(source_chirho) => source_chirho,
        Err(error_chirho) => {
            eprintln!("{}: {error_chirho}", positional_chirho[1]);
            return ExitCode::from(1);
        }
    };
    let result_chirho = if flags_chirho.dump_core_chirho {
        haskelujah_driver_chirho::next_chirho::compile_source_chirho(&source_chirho).map(
            |module_chirho| {
                eprintln!("{module_chirho:#?}");
            },
        )
    } else {
        haskelujah_driver_chirho::next_chirho::check_source_chirho(&source_chirho).map(|_| ())
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
