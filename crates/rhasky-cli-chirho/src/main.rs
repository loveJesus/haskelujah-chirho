// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

use std::env;
use std::process::ExitCode;

use rhasky_driver_chirho::{check_source_path_chirho, render_summary_chirho};
use rhasky_runtime_chirho::ExecutionModeChirho;

fn main() -> ExitCode {
    main_chirho()
}

fn main_chirho() -> ExitCode {
    let mut arguments_chirho = env::args();
    let program_name_chirho = arguments_chirho
        .next()
        .unwrap_or_else(|| "rhasky-cli-chirho".to_owned());
    let command_chirho = arguments_chirho.next();
    let path_chirho = arguments_chirho.next();

    let Some(command_chirho) = command_chirho else {
        print_usage_chirho(&program_name_chirho);
        return ExitCode::from(2);
    };

    match command_chirho.as_str() {
        "check" | "plan" | "script" => {
            let Some(path_chirho) = path_chirho else {
                eprintln!("missing path for `{command_chirho}`");
                print_usage_chirho(&program_name_chirho);
                return ExitCode::from(2);
            };

            let execution_mode_chirho = if command_chirho == "script" {
                ExecutionModeChirho::ScriptChirho
            } else {
                ExecutionModeChirho::BatchChirho
            };

            match check_source_path_chirho(&path_chirho, execution_mode_chirho) {
                Ok(check_summary_chirho) => {
                    println!("{}", render_summary_chirho(&check_summary_chirho));
                    ExitCode::SUCCESS
                }
                Err(diagnostic_bundle_chirho) => {
                    eprintln!("{diagnostic_bundle_chirho}");
                    ExitCode::from(1)
                }
            }
        }
        "repl" => {
            println!(
                "repl mode is not implemented yet, but the runtime plan reserves {:?} for it",
                ExecutionModeChirho::ReplChirho
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("unknown command `{command_chirho}`");
            print_usage_chirho(&program_name_chirho);
            ExitCode::from(2)
        }
    }
}

fn print_usage_chirho(program_name_chirho: &str) {
    eprintln!("usage: {program_name_chirho} <check|plan|script|repl> [path]");
}
