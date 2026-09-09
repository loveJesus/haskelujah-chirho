// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Cabal conditional selection and layout-preserving flattening.

use super::{ConditionChirho, eval_condition_chirho, parse_condition_chirho, strip_comment_chirho};
use std::collections::HashMap;

#[cfg(test)]
mod tests_chirho;

/// Pre-process cabal file to resolve `if os(...)` / `else` blocks inline.
///
/// On Linux/Unix, `if os(windows)` blocks are removed and `else` blocks are
/// kept (with indentation reduced).  On Windows, the reverse applies.
/// `if impl(ghc ...)` and `if flag(...)` are treated as true by default.
pub(super) fn preprocess_cabal_conditionals_chirho(input_chirho: &str) -> String {
    let flag_defaults_chirho = collect_flag_defaults_chirho(input_chirho);
    preprocess_cabal_conditionals_with_flags_chirho(input_chirho, &flag_defaults_chirho)
}

fn preprocess_cabal_conditionals_with_flags_chirho(
    input_chirho: &str,
    flag_defaults_chirho: &HashMap<String, bool>,
) -> String {
    let os_chirho = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "osx"
    } else {
        "linux"
    };
    preprocess_cabal_conditionals_with_env_chirho(
        input_chirho,
        flag_defaults_chirho,
        os_chirho,
        std::env::consts::ARCH,
    )
}

fn preprocess_cabal_conditionals_with_env_chirho(
    input_chirho: &str,
    flag_defaults_chirho: &HashMap<String, bool>,
    os_chirho: &str,
    arch_chirho: &str,
) -> String {
    let lines_chirho: Vec<&str> = input_chirho.lines().collect();
    let mut result_chirho: Vec<String> = Vec::with_capacity(lines_chirho.len());
    let mut i_chirho = 0;

    while i_chirho < lines_chirho.len() {
        let line_chirho = lines_chirho[i_chirho];
        let trimmed_chirho = line_chirho.trim();

        // Detect `if <condition>` at any indentation
        if let Some(cond_text_chirho) = trimmed_chirho.strip_prefix("if ") {
            let if_indent_chirho = line_chirho.len() - line_chirho.trim_start().len();
            let mut branches_chirho: Vec<(Option<ConditionChirho>, Vec<&str>)> = vec![(
                Some(parse_condition_chirho(cond_text_chirho.trim())),
                Vec::new(),
            )];

            i_chirho += 1;

            loop {
                while i_chirho < lines_chirho.len() {
                    let branch_line_chirho = lines_chirho[i_chirho];
                    let branch_trimmed_chirho = branch_line_chirho.trim();
                    let branch_indent_chirho =
                        branch_line_chirho.len() - branch_line_chirho.trim_start().len();
                    let is_same_level_else_chirho = branch_indent_chirho == if_indent_chirho
                        && branch_trimmed_chirho.eq_ignore_ascii_case("else");
                    let is_same_level_elif_chirho = branch_indent_chirho == if_indent_chirho
                        && branch_trimmed_chirho.starts_with("elif ");
                    if is_same_level_else_chirho || is_same_level_elif_chirho {
                        break;
                    }
                    if !branch_trimmed_chirho.is_empty() && branch_indent_chirho <= if_indent_chirho
                    {
                        break;
                    }
                    branches_chirho
                        .last_mut()
                        .unwrap()
                        .1
                        .push(branch_line_chirho);
                    i_chirho += 1;
                }

                if i_chirho >= lines_chirho.len() {
                    break;
                }

                let branch_header_chirho = lines_chirho[i_chirho];
                let branch_header_trimmed_chirho = branch_header_chirho.trim();
                let branch_header_indent_chirho =
                    branch_header_chirho.len() - branch_header_chirho.trim_start().len();
                if branch_header_indent_chirho != if_indent_chirho {
                    break;
                }

                if let Some(elif_cond_text_chirho) =
                    branch_header_trimmed_chirho.strip_prefix("elif ")
                {
                    branches_chirho.push((
                        Some(parse_condition_chirho(elif_cond_text_chirho.trim())),
                        Vec::new(),
                    ));
                    i_chirho += 1;
                    continue;
                }
                if branch_header_trimmed_chirho.eq_ignore_ascii_case("else") {
                    branches_chirho.push((None, Vec::new()));
                    i_chirho += 1;
                    continue;
                }
                break;
            }

            // Include the first matching branch, recursively preprocessing
            // nested conditionals and dedenting it back to the surrounding
            // stanza level so fields are not swallowed as continuations.
            let Some(chosen_chirho) =
                branches_chirho
                    .iter()
                    .find_map(|(cond_chirho, branch_lines_chirho)| match cond_chirho {
                        Some(cond_chirho)
                            if eval_condition_chirho(
                                cond_chirho,
                                flag_defaults_chirho,
                                os_chirho,
                                arch_chirho,
                            ) =>
                        {
                            Some(branch_lines_chirho)
                        }
                        None => Some(branch_lines_chirho),
                        _ => None,
                    })
            else {
                // A false if with no matching elif/else contributes no fields.
                // The scanner already points at the following sibling line.
                continue;
            };
            let min_block_indent_chirho = chosen_chirho
                .iter()
                .filter_map(|line_chirho| {
                    let trimmed_line_chirho = line_chirho.trim();
                    if trimmed_line_chirho.is_empty() {
                        None
                    } else {
                        Some(line_chirho.len() - line_chirho.trim_start().len())
                    }
                })
                .min()
                .unwrap_or(if_indent_chirho);
            let dedent_width_chirho = min_block_indent_chirho.saturating_sub(if_indent_chirho);
            let dedented_block_chirho = chosen_chirho
                .iter()
                .map(|line_chirho| {
                    if line_chirho.trim().is_empty() {
                        String::new()
                    } else {
                        line_chirho
                            .get(dedent_width_chirho..)
                            .unwrap_or(line_chirho)
                            .to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let nested_preprocessed_chirho = preprocess_cabal_conditionals_with_env_chirho(
                &dedented_block_chirho,
                flag_defaults_chirho,
                os_chirho,
                arch_chirho,
            );
            result_chirho.extend(
                nested_preprocessed_chirho
                    .lines()
                    .map(|line_chirho| line_chirho.to_string()),
            );
        } else {
            result_chirho.push(line_chirho.to_string());
            i_chirho += 1;
        }
    }

    result_chirho.join("\n")
}

fn collect_flag_defaults_chirho(input_chirho: &str) -> HashMap<String, bool> {
    let mut flag_defaults_chirho = HashMap::new();
    let mut current_flag_name_chirho: Option<String> = None;

    for raw_line_chirho in input_chirho.lines() {
        let line_chirho = strip_comment_chirho(raw_line_chirho);
        if line_chirho.trim().is_empty() {
            continue;
        }

        let indent_chirho = line_chirho.len() - line_chirho.trim_start().len();
        let trimmed_chirho = line_chirho.trim();
        let lower_trimmed_chirho = trimmed_chirho.to_ascii_lowercase();

        if indent_chirho == 0 {
            current_flag_name_chirho = lower_trimmed_chirho
                .strip_prefix("flag ")
                .map(|name_chirho| name_chirho.trim().to_string());
            continue;
        }

        let Some(flag_name_chirho) = current_flag_name_chirho.as_ref() else {
            continue;
        };

        if let Some(default_value_chirho) = lower_trimmed_chirho.strip_prefix("default:") {
            let parsed_default_chirho = match default_value_chirho.trim() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
            if let Some(parsed_default_chirho) = parsed_default_chirho {
                flag_defaults_chirho.insert(flag_name_chirho.clone(), parsed_default_chirho);
            }
        }
    }

    flag_defaults_chirho
}
