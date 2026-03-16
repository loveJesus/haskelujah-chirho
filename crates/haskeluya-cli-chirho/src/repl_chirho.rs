// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use haskeluya_driver_chirho::{eval_source_with_machine_chirho, run_frontend_chirho};
use haskeluya_naming_chirho::builtin_module_ifaces_chirho;
use haskeluya_runtime_chirho::ValueChirho;
use haskeluya_span_chirho::SourceMapChirho;
use haskeluya_typing_chirho::SchemeChirho;

/// REPL state persists across evaluations.
struct ReplStateChirho {
    /// Definitions accumulated from user input and :load.
    definitions_chirho: Vec<String>,
    /// Last loaded file path (for :reload).
    loaded_file_chirho: Option<String>,
    /// Imported type schemes from loaded modules.
    imported_types_chirho: HashMap<String, SchemeChirho>,
    /// Line counter for fresh module names.
    eval_counter_chirho: u64,
}

impl ReplStateChirho {
    fn new_chirho() -> Self {
        Self {
            definitions_chirho: Vec::new(),
            loaded_file_chirho: None,
            imported_types_chirho: HashMap::new(),
            eval_counter_chirho: 0,
        }
    }
}

/// Run the interactive REPL. Returns the exit code.
pub fn repl_command_chirho() -> std::process::ExitCode {
    println!("Haskeluya REPL — Haskell compiler in Rust");
    println!("Type :help for available commands, :quit to exit.\n");

    let stdin_chirho = io::stdin();
    let mut reader_chirho = stdin_chirho.lock();
    let mut state_chirho = ReplStateChirho::new_chirho();
    let mut multiline_buffer_chirho: Option<String> = None;

    loop {
        // Print prompt.
        let prompt_chirho = if multiline_buffer_chirho.is_some() {
            "| "
        } else {
            "haskeluya> "
        };
        print!("{prompt_chirho}");
        let _ = io::stdout().flush();

        let mut line_chirho = String::new();
        match reader_chirho.read_line(&mut line_chirho) {
            Ok(0) => {
                // EOF
                println!();
                break;
            }
            Ok(_) => {}
            Err(e_chirho) => {
                eprintln!("error reading input: {e_chirho}");
                break;
            }
        }

        let trimmed_chirho = line_chirho.trim();

        // Multi-line mode.
        if let Some(ref mut buf_chirho) = multiline_buffer_chirho {
            if trimmed_chirho == ":}" {
                let complete_chirho = buf_chirho.clone();
                multiline_buffer_chirho = None;
                eval_input_chirho(&complete_chirho, &mut state_chirho);
            } else {
                buf_chirho.push_str(&line_chirho);
            }
            continue;
        }

        if trimmed_chirho == ":{" {
            multiline_buffer_chirho = Some(String::new());
            continue;
        }

        if trimmed_chirho.is_empty() {
            continue;
        }

        // REPL commands.
        if trimmed_chirho.starts_with(':') {
            if handle_command_chirho(trimmed_chirho, &mut state_chirho) {
                break; // :quit
            }
            continue;
        }

        // Regular expression/declaration.
        eval_input_chirho(trimmed_chirho, &mut state_chirho);
    }

    std::process::ExitCode::SUCCESS
}

/// Handle a REPL command. Returns true if the REPL should exit.
fn handle_command_chirho(input_chirho: &str, state_chirho: &mut ReplStateChirho) -> bool {
    let parts_chirho: Vec<&str> = input_chirho.splitn(2, char::is_whitespace).collect();
    let cmd_chirho = parts_chirho[0];
    let arg_chirho = parts_chirho.get(1).map(|s_chirho| s_chirho.trim());

    match cmd_chirho {
        ":quit" | ":q" => return true,

        ":help" | ":h" | ":?" => {
            println!("Available commands:");
            println!("  :load <file>    Load a Haskell source file");
            println!("  :reload         Reload the last loaded file");
            println!("  :type <expr>    Show the type of an expression");
            println!("  :info <name>    Show info about a name");
            println!("  :let <decl>     Add a definition to scope");
            println!("  :{{             Begin multi-line input");
            println!("  :}}             End multi-line input");
            println!("  :clear          Clear accumulated definitions");
            println!("  :quit           Exit the REPL");
        }

        ":load" | ":l" => {
            if let Some(path_chirho) = arg_chirho {
                load_file_chirho(path_chirho, state_chirho);
            } else {
                eprintln!("usage: :load <file.hs>");
            }
        }

        ":reload" | ":r" => {
            if let Some(path_chirho) = state_chirho.loaded_file_chirho.clone() {
                load_file_chirho(&path_chirho, state_chirho);
            } else {
                eprintln!("no file loaded; use :load <file> first");
            }
        }

        ":type" | ":t" => {
            if let Some(expr_chirho) = arg_chirho {
                type_of_chirho(expr_chirho, state_chirho);
            } else {
                eprintln!("usage: :type <expr>");
            }
        }

        ":info" | ":i" => {
            if let Some(name_chirho) = arg_chirho {
                info_chirho(name_chirho, state_chirho);
            } else {
                eprintln!("usage: :info <name>");
            }
        }

        ":let" => {
            if let Some(decl_chirho) = arg_chirho {
                state_chirho
                    .definitions_chirho
                    .push(decl_chirho.to_string());
            } else {
                eprintln!("usage: :let <name> = <expr>");
            }
        }

        ":clear" => {
            state_chirho.definitions_chirho.clear();
            state_chirho.imported_types_chirho.clear();
            println!("Cleared all definitions.");
        }

        other_chirho => {
            eprintln!("unknown command: {other_chirho}");
            eprintln!("use :help for available commands");
        }
    }

    false
}

/// Load a Haskell file, parsing its declarations into REPL state.
fn load_file_chirho(path_chirho: &str, state_chirho: &mut ReplStateChirho) {
    match std::fs::read_to_string(path_chirho) {
        Ok(source_chirho) => {
            state_chirho.loaded_file_chirho = Some(path_chirho.to_string());
            // Extract declarations (skip the module header).
            let lines_chirho: Vec<&str> = source_chirho.lines().collect();
            let mut decls_chirho = Vec::new();
            let mut past_header_chirho = false;
            for line_chirho in &lines_chirho {
                let trimmed_chirho = line_chirho.trim();
                if !past_header_chirho {
                    if trimmed_chirho.starts_with("module ") && trimmed_chirho.contains("where") {
                        past_header_chirho = true;
                        continue;
                    }
                    if trimmed_chirho.starts_with("import ") {
                        past_header_chirho = true;
                        decls_chirho.push(line_chirho.to_string());
                        continue;
                    }
                    // Skip pragmas and comments at top.
                    if trimmed_chirho.starts_with("{-#")
                        || trimmed_chirho.starts_with("--")
                        || trimmed_chirho.is_empty()
                    {
                        continue;
                    }
                    past_header_chirho = true;
                }
                decls_chirho.push(line_chirho.to_string());
            }

            // Also run the full frontend to extract type environment.
            let mut sm_chirho = SourceMapChirho::new_chirho();
            let file_id_chirho = sm_chirho.add_file_chirho(path_chirho, &source_chirho);
            match run_frontend_chirho(
                &source_chirho,
                file_id_chirho,
                &builtin_module_ifaces_chirho(),
                &HashMap::new(),
            ) {
                Ok(result_chirho) => {
                    // Extract exported type schemes.
                    let env_chirho = &result_chirho.infer_result_chirho.env_chirho;
                    for (name_chirho, scheme_chirho) in env_chirho.all_bindings_chirho() {
                        state_chirho
                            .imported_types_chirho
                            .insert(name_chirho.clone(), scheme_chirho.clone());
                    }
                    state_chirho.definitions_chirho = decls_chirho;
                    println!(
                        "Loaded: {} ({} definitions)",
                        path_chirho,
                        state_chirho.definitions_chirho.len()
                    );
                    for w_chirho in &result_chirho.warnings_chirho {
                        eprintln!("warning: {w_chirho}");
                    }
                }
                Err(diag_chirho) => {
                    state_chirho.definitions_chirho = decls_chirho;
                    eprintln!("type-checking warnings while loading:");
                    eprintln!("{diag_chirho}");
                    println!("Loaded: {} (with errors)", path_chirho);
                }
            }
        }
        Err(e_chirho) => {
            eprintln!("cannot read `{path_chirho}`: {e_chirho}");
        }
    }
}

/// Evaluate an expression or declaration in the REPL.
fn eval_input_chirho(input_chirho: &str, state_chirho: &mut ReplStateChirho) {
    state_chirho.eval_counter_chirho += 1;

    // Determine if input looks like a declaration or an expression.
    // Declarations start with a lowercase name followed by patterns/= or
    // a keyword (data, type, class, instance, newtype).
    // We check for a top-level `=` that is NOT inside quotes or parens.
    let is_decl_chirho = if input_chirho.starts_with("data ")
        || input_chirho.starts_with("type ")
        || input_chirho.starts_with("class ")
        || input_chirho.starts_with("instance ")
        || input_chirho.starts_with("newtype ")
    {
        true
    } else if input_chirho.starts_with(|c: char| c.is_lowercase() || c == '_') {
        // Check for a top-level `=` (not `==`, not inside quotes/parens).
        has_toplevel_equals_chirho(input_chirho)
    } else {
        false
    };

    let source_chirho = if is_decl_chirho {
        // It's a declaration — add it to definitions and evaluate.
        state_chirho
            .definitions_chirho
            .push(input_chirho.to_string());
        // Build a module with all accumulated definitions.
        let mut src_chirho = String::from("module ReplMain where\n");
        for def_chirho in &state_chirho.definitions_chirho {
            src_chirho.push_str(def_chirho);
            src_chirho.push('\n');
        }
        // No main to evaluate for bare declarations.
        println!("defined.");
        return;
    } else {
        // It's an expression — wrap it as `main`.
        let mut src_chirho = String::from("module ReplMain where\n");
        for def_chirho in &state_chirho.definitions_chirho {
            src_chirho.push_str(def_chirho);
            src_chirho.push('\n');
        }
        // Try as IO action first (putStrLn, print, etc.), fall back to pure expr.
        src_chirho.push_str(&format!("main = {input_chirho}\n"));
        src_chirho
    };

    let mut sm_chirho = SourceMapChirho::new_chirho();
    match eval_source_with_machine_chirho(&source_chirho, &mut sm_chirho, "REPL", None) {
        Ok((value_chirho, machine_chirho)) => {
            // Print IO output if any.
            if !machine_chirho.io_output_chirho.is_empty() {
                print!("{}", machine_chirho.io_output_chirho);
            } else {
                // Print the returned value for pure expressions.
                println!("{}", display_value_chirho(&value_chirho));
            }
        }
        Err(err_chirho) => {
            // If eval fails, try wrapping in `print`.
            let print_src_chirho = {
                let mut src_chirho = String::from("module ReplMain where\n");
                for def_chirho in &state_chirho.definitions_chirho {
                    src_chirho.push_str(def_chirho);
                    src_chirho.push('\n');
                }
                src_chirho.push_str(&format!("main = print ({input_chirho})\n"));
                src_chirho
            };

            let mut sm2_chirho = SourceMapChirho::new_chirho();
            match eval_source_with_machine_chirho(&print_src_chirho, &mut sm2_chirho, "REPL", None)
            {
                Ok((_val_chirho, machine_chirho)) => {
                    if !machine_chirho.io_output_chirho.is_empty() {
                        print!("{}", machine_chirho.io_output_chirho);
                    }
                }
                Err(_) => {
                    eprintln!("error: {err_chirho}");
                }
            }
        }
    }
}

/// Show the type of an expression.
fn type_of_chirho(expr_chirho: &str, state_chirho: &ReplStateChirho) {
    // Build a module with the expression as a binding.
    let binding_name_chirho = "_repl_expr_chirho";
    let mut src_chirho = String::from("module ReplType where\n");
    for def_chirho in &state_chirho.definitions_chirho {
        src_chirho.push_str(def_chirho);
        src_chirho.push('\n');
    }
    src_chirho.push_str(&format!("{binding_name_chirho} = {expr_chirho}\n"));

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let file_id_chirho = sm_chirho.add_file_chirho("REPL", &src_chirho);
    match run_frontend_chirho(
        &src_chirho,
        file_id_chirho,
        &builtin_module_ifaces_chirho(),
        &state_chirho.imported_types_chirho,
    ) {
        Ok(result_chirho) => {
            let env_chirho = &result_chirho.infer_result_chirho.env_chirho;
            let subst_chirho = &result_chirho.infer_result_chirho.subst_chirho;
            if let Some(scheme_chirho) = env_chirho.lookup_chirho(binding_name_chirho) {
                let applied_chirho = subst_chirho.apply_scheme_chirho(scheme_chirho);
                println!("{expr_chirho} :: {applied_chirho}");
            } else {
                eprintln!("could not determine type");
            }
        }
        Err(diag_chirho) => {
            eprintln!("{diag_chirho}");
        }
    }
}

/// Check if input has a top-level `=` that looks like a binding (not `==`,
/// not inside string literals or parentheses).
fn has_toplevel_equals_chirho(input_chirho: &str) -> bool {
    let mut depth_chirho: i32 = 0;
    let mut in_string_chirho = false;
    let mut in_char_chirho = false;
    let mut prev_chirho = '\0';
    let chars_chirho: Vec<char> = input_chirho.chars().collect();

    for (i_chirho, &c_chirho) in chars_chirho.iter().enumerate() {
        if in_string_chirho {
            if c_chirho == '"' && prev_chirho != '\\' {
                in_string_chirho = false;
            }
            prev_chirho = c_chirho;
            continue;
        }
        if in_char_chirho {
            if c_chirho == '\'' && prev_chirho != '\\' {
                in_char_chirho = false;
            }
            prev_chirho = c_chirho;
            continue;
        }

        match c_chirho {
            '"' => in_string_chirho = true,
            '\'' => in_char_chirho = true,
            '(' | '[' | '{' => depth_chirho += 1,
            ')' | ']' | '}' => depth_chirho -= 1,
            '=' if depth_chirho == 0 => {
                // Check it's not `==`
                let next_chirho = chars_chirho.get(i_chirho + 1).copied().unwrap_or('\0');
                if next_chirho != '=' && prev_chirho != '/' && prev_chirho != '!' && prev_chirho != '<' && prev_chirho != '>' {
                    return true;
                }
            }
            _ => {}
        }
        prev_chirho = c_chirho;
    }
    false
}

/// Display a runtime value in a user-friendly way (without internal # suffixes).
fn display_value_chirho(val_chirho: &ValueChirho) -> String {
    match val_chirho {
        ValueChirho::IntChirho(n_chirho) => n_chirho.to_string(),
        ValueChirho::FloatChirho(f_chirho) => {
            let s_chirho = f_chirho.to_string();
            if s_chirho.contains('.') {
                s_chirho
            } else {
                format!("{s_chirho}.0")
            }
        }
        ValueChirho::CharChirho(c_chirho) => format!("'{c_chirho}'"),
        ValueChirho::BoolChirho(b_chirho) => {
            if *b_chirho {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        ValueChirho::StringChirho(s_chirho) => format!("\"{s_chirho}\""),
        ValueChirho::HeapPtrChirho(addr_chirho) => format!("<<heap@{}>>", addr_chirho.0),
        ValueChirho::MapChirho(pairs_chirho) => {
            let entries_chirho: Vec<String> = pairs_chirho
                .iter()
                .map(|(k_chirho, v_chirho)| {
                    format!(
                        "({},{})",
                        display_value_chirho(k_chirho),
                        display_value_chirho(v_chirho)
                    )
                })
                .collect();
            format!("fromList [{}]", entries_chirho.join(","))
        }
        ValueChirho::SetChirho(elems_chirho) => {
            let items_chirho: Vec<String> = elems_chirho
                .iter()
                .map(|e_chirho| display_value_chirho(e_chirho))
                .collect();
            format!("fromList [{}]", items_chirho.join(","))
        }
    }
}

/// Show info about a name (type, class, etc.).
fn info_chirho(name_chirho: &str, state_chirho: &ReplStateChirho) {
    // Check imported types first.
    if let Some(scheme_chirho) = state_chirho.imported_types_chirho.get(name_chirho) {
        println!("{name_chirho} :: {scheme_chirho}");
        return;
    }

    // Build a module with existing definitions and look up.
    let mut src_chirho = String::from("module ReplInfo where\n");
    for def_chirho in &state_chirho.definitions_chirho {
        src_chirho.push_str(def_chirho);
        src_chirho.push('\n');
    }
    // Add a dummy binding so the module compiles.
    src_chirho.push_str("_dummy_chirho = 0\n");

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let file_id_chirho = sm_chirho.add_file_chirho("REPL", &src_chirho);
    match run_frontend_chirho(
        &src_chirho,
        file_id_chirho,
        &builtin_module_ifaces_chirho(),
        &state_chirho.imported_types_chirho,
    ) {
        Ok(result_chirho) => {
            let env_chirho = &result_chirho.infer_result_chirho.env_chirho;
            let subst_chirho = &result_chirho.infer_result_chirho.subst_chirho;
            if let Some(scheme_chirho) = env_chirho.lookup_chirho(name_chirho) {
                let applied_chirho = subst_chirho.apply_scheme_chirho(scheme_chirho);
                println!("{name_chirho} :: {applied_chirho}");
            } else {
                eprintln!("not in scope: {name_chirho}");
            }
        }
        Err(diag_chirho) => {
            eprintln!("{diag_chirho}");
        }
    }
}
