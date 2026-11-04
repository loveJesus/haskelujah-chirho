// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `.hsc` preprocessing and source reading, moved from the driver root (row 691, L1): hsc
//! include stripping and sanitizing around cpp, the lowering of Maybe-like unboxed sums, and
//! reading a Haskell source file through cpp with a directive-stripping fallback.

use super::*;

pub(crate) fn path_is_hsc_chirho(path_chirho: &Path) -> bool {
    path_chirho
        .extension()
        .and_then(|ext_chirho| ext_chirho.to_str())
        .is_some_and(|ext_chirho| ext_chirho.eq_ignore_ascii_case("hsc"))
}

fn sanitize_hsc_source_chirho(source_chirho: &str) -> String {
    let mut sanitized_chirho = String::with_capacity(source_chirho.len());
    let mut cursor_chirho = 0usize;

    while let Some(rel_start_chirho) = source_chirho[cursor_chirho..].find("#{") {
        let start_chirho = cursor_chirho + rel_start_chirho;
        sanitized_chirho.push_str(&source_chirho[cursor_chirho..start_chirho]);
        let body_start_chirho = start_chirho + 2;
        let Some(rel_end_chirho) = source_chirho[body_start_chirho..].find('}') else {
            sanitized_chirho.push_str(&source_chirho[start_chirho..]);
            return sanitized_chirho;
        };
        let end_chirho = body_start_chirho + rel_end_chirho;
        let body_chirho = source_chirho[body_start_chirho..end_chirho].trim();
        let replacement_chirho = if body_chirho.starts_with("type ") {
            "CInt"
        } else if body_chirho.starts_with("size ")
            || body_chirho.starts_with("alignment ")
            || body_chirho.starts_with("const ")
        {
            "0"
        } else if body_chirho.starts_with("peek ") {
            "peek"
        } else if body_chirho.starts_with("poke ") {
            "poke"
        } else if body_chirho.starts_with("ptr ") {
            "nullPtr"
        } else {
            "undefined"
        };
        sanitized_chirho.push_str(replacement_chirho);
        cursor_chirho = end_chirho + 1;
    }

    sanitized_chirho.push_str(&source_chirho[cursor_chirho..]);
    sanitized_chirho
}

fn strip_hsc_include_directives_chirho(source_chirho: &str) -> String {
    source_chirho
        .lines()
        .filter(|line_chirho| {
            let trimmed_chirho = line_chirho.trim_start();
            let directive_chirho = trimmed_chirho
                .strip_prefix('#')
                .map(|rest_chirho| rest_chirho.trim_start())
                .unwrap_or_default();
            !(directive_chirho.starts_with("include") || directive_chirho.starts_with("let"))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn preprocess_hsc_source_with_options_chirho(
    path_chirho: &Path,
    source_chirho: &str,
    extra_cpp_options_chirho: &[String],
) -> io::Result<String> {
    let stripped_source_chirho = strip_hsc_include_directives_chirho(source_chirho);
    let sanitized_source_chirho = sanitize_hsc_source_chirho(&stripped_source_chirho);
    let temporary_chirho =
        temporary_source_file_chirho(&sanitized_source_chirho, path_chirho.parent(), ".hsc")?;
    let output_chirho =
        configure_cpp_command_chirho(temporary_chirho.path(), true, extra_cpp_options_chirho)?
            .output_chirho()?;
    if !output_chirho.status.success() {
        let stderr_chirho = String::from_utf8_lossy(&output_chirho.stderr)
            .trim()
            .to_string();
        return Err(io::Error::other(format!(
            "hsc preprocessing failed for {}: {}",
            path_chirho.display(),
            stderr_chirho
        )));
    }
    let preprocessed_chirho = decode_cpp_stdout_chirho(path_chirho, output_chirho.stdout)?;
    Ok(sanitize_hsc_source_chirho(&preprocessed_chirho))
}

fn is_maybe_absent_unboxed_sum_alt_chirho(alt_chirho: &str) -> bool {
    matches!(alt_chirho.trim(), "" | "(# #)" | "(##)")
}

fn maybe_like_unboxed_sum_parts_chirho(
    line_chirho: &str,
    start_chirho: usize,
) -> Option<(usize, &str, &str)> {
    if !line_chirho[start_chirho..].starts_with("(#") {
        return None;
    }

    let bytes_chirho = line_chirho.as_bytes();
    let mut idx_chirho = start_chirho + 2;
    let mut depth_chirho = 1usize;
    let mut pipe_idx_chirho = None;

    while idx_chirho + 1 < bytes_chirho.len() {
        if line_chirho[idx_chirho..].starts_with("(#") {
            depth_chirho += 1;
            idx_chirho += 2;
            continue;
        }
        if line_chirho[idx_chirho..].starts_with("#)") {
            depth_chirho = depth_chirho.saturating_sub(1);
            if depth_chirho == 0 {
                let pipe_idx_chirho = pipe_idx_chirho?;
                let body_start_chirho = start_chirho + 2;
                let body_end_chirho = idx_chirho;
                let left_chirho = &line_chirho[body_start_chirho..pipe_idx_chirho];
                let right_chirho = &line_chirho[pipe_idx_chirho + 1..body_end_chirho];
                return Some((idx_chirho + 2, left_chirho, right_chirho));
            }
            idx_chirho += 2;
            continue;
        }
        if depth_chirho == 1 && bytes_chirho[idx_chirho] == b'|' {
            pipe_idx_chirho = Some(idx_chirho);
        }
        idx_chirho += 1;
    }

    None
}

fn rewrite_maybe_like_unboxed_sum_line_chirho(
    line_chirho: &str,
    type_context_chirho: bool,
) -> String {
    if !line_chirho.contains("(#") || !line_chirho.contains('|') {
        return line_chirho.to_string();
    }

    let mut rewritten_chirho = String::with_capacity(line_chirho.len());
    let mut cursor_chirho = 0usize;

    while let Some(rel_start_chirho) = line_chirho[cursor_chirho..].find("(#") {
        let start_chirho = cursor_chirho + rel_start_chirho;
        rewritten_chirho.push_str(&line_chirho[cursor_chirho..start_chirho]);

        let Some((end_chirho, left_alt_chirho, right_alt_chirho)) =
            maybe_like_unboxed_sum_parts_chirho(line_chirho, start_chirho)
        else {
            rewritten_chirho.push_str("(#");
            cursor_chirho = start_chirho + 2;
            continue;
        };

        let left_trimmed_chirho = left_alt_chirho.trim();
        let right_trimmed_chirho = right_alt_chirho.trim();

        let replacement_chirho = if type_context_chirho {
            if is_maybe_absent_unboxed_sum_alt_chirho(left_trimmed_chirho)
                && !right_trimmed_chirho.is_empty()
            {
                Some(format!("Maybe {right_trimmed_chirho}"))
            } else {
                None
            }
        } else if is_maybe_absent_unboxed_sum_alt_chirho(left_trimmed_chirho) {
            if right_trimmed_chirho.is_empty() {
                Some("(Nothing)".to_string())
            } else {
                Some(format!("(Just {right_trimmed_chirho})"))
            }
        } else {
            None
        };

        if let Some(replacement_chirho) = replacement_chirho {
            rewritten_chirho.push_str(&replacement_chirho);
            cursor_chirho = end_chirho;
        } else {
            rewritten_chirho.push_str(&line_chirho[start_chirho..end_chirho]);
            cursor_chirho = end_chirho;
        }
    }

    rewritten_chirho.push_str(&line_chirho[cursor_chirho..]);
    rewritten_chirho
}

pub(crate) fn lower_maybe_like_unboxed_sums_chirho(source_chirho: &str) -> String {
    if !source_chirho.contains("(#") || !source_chirho.contains('|') {
        return source_chirho.to_string();
    }

    source_chirho
        .lines()
        .map(|line_chirho| {
            let type_rewritten_chirho = if line_chirho.contains("::") {
                rewrite_maybe_like_unboxed_sum_line_chirho(line_chirho, true)
            } else {
                line_chirho.to_string()
            };
            rewrite_maybe_like_unboxed_sum_line_chirho(&type_rewritten_chirho, false)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn read_haskell_source_file_chirho(path_chirho: impl AsRef<Path>) -> io::Result<String> {
    let path_ref_chirho = path_chirho.as_ref();
    let source_chirho = std::fs::read_to_string(path_ref_chirho)?;
    // Try CPP preprocessing; fall back to raw source (with directives
    // stripped) if cpp fails (e.g., tick characters in Haskell identifiers).
    match preprocess_cpp_source_chirho(path_ref_chirho, &source_chirho) {
        Ok(processed_chirho) => {
            let cleaned_chirho = if has_cpp_directive_residue_chirho(&processed_chirho) {
                strip_cpp_directives_chirho(&processed_chirho)
            } else {
                processed_chirho
            };
            Ok(lower_maybe_like_unboxed_sums_chirho(&cleaned_chirho))
        }
        Err(_) if path_is_hsc_chirho(path_ref_chirho) => Ok(sanitize_hsc_source_chirho(
            &lower_maybe_like_unboxed_sums_chirho(&strip_cpp_directives_chirho(&source_chirho)),
        )),
        Err(_) => Ok(lower_maybe_like_unboxed_sums_chirho(
            &strip_cpp_directives_chirho(&source_chirho),
        )),
    }
}
