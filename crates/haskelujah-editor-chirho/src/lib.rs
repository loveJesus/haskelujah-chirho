// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-editor-chirho
//!
//! A self-programmable terminal editor for Haskell, extensible via the
//! language it compiles — like Emacs uses Lisp, Haskelujah uses Haskell.
//!
//! ## Architecture
//!
//! - Rust core: buffer management, rendering, input handling (crossterm)
//! - Haskell plugin API: commands, keybindings, syntax rules evaluated
//!   by the Haskelujah compiler at runtime
//! - Built-in LSP client: connects to `haskelujah lsp` for diagnostics

use crossterm::{
    cursor, event, execute, queue,
    style::{self, Color, Stylize},
    terminal,
};
use std::io::{self, Write};

/// A text buffer line.
#[derive(Clone, Debug)]
pub struct LineChirho {
    pub text_chirho: String,
}

/// The editor state.
pub struct EditorChirho {
    pub lines_chirho: Vec<LineChirho>,
    pub cursor_row_chirho: usize,
    pub cursor_col_chirho: usize,
    pub scroll_offset_chirho: usize,
    pub file_path_chirho: Option<String>,
    pub modified_chirho: bool,
    pub running_chirho: bool,
    pub status_msg_chirho: String,
    prompt_mode_chirho: Option<PromptModeChirho>,
}

impl EditorChirho {
    pub fn new_chirho() -> Self {
        Self {
            lines_chirho: vec![LineChirho {
                text_chirho: String::new(),
            }],
            cursor_row_chirho: 0,
            cursor_col_chirho: 0,
            scroll_offset_chirho: 0,
            file_path_chirho: None,
            modified_chirho: false,
            running_chirho: true,
            status_msg_chirho:
                "Haskelujah Editor — Ctrl-Q quit, Ctrl-S save, Ctrl-T typecheck, Ctrl-E eval, Ctrl-G goto line"
                    .to_string(),
            prompt_mode_chirho: None,
        }
    }

    pub fn open_file_chirho(&mut self, path_chirho: &str) -> io::Result<()> {
        let content_chirho = std::fs::read_to_string(path_chirho)?;
        self.lines_chirho = content_chirho
            .lines()
            .map(|l_chirho| LineChirho {
                text_chirho: l_chirho.to_string(),
            })
            .collect();
        if self.lines_chirho.is_empty() {
            self.lines_chirho.push(LineChirho {
                text_chirho: String::new(),
            });
        }
        self.file_path_chirho = Some(path_chirho.to_string());
        self.cursor_row_chirho = 0;
        self.cursor_col_chirho = 0;
        self.modified_chirho = false;
        self.status_msg_chirho = format!("Opened {}", path_chirho);
        Ok(())
    }

    pub fn save_file_chirho(&mut self) -> io::Result<()> {
        if let Some(path_chirho) = &self.file_path_chirho {
            let content_chirho: String = self
                .lines_chirho
                .iter()
                .map(|l_chirho| l_chirho.text_chirho.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write(path_chirho, &content_chirho)?;
            self.modified_chirho = false;
            self.status_msg_chirho = format!("Saved {}", path_chirho);
        }
        Ok(())
    }

    pub fn typecheck_chirho(&mut self) {
        let source_chirho: String = self
            .lines_chirho
            .iter()
            .map(|l_chirho| l_chirho.text_chirho.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let file_name_chirho = self.file_path_chirho.as_deref().unwrap_or("Buffer.hs");
        let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
        match haskelujah_driver_chirho::compile_source_chirho(
            &source_chirho,
            &mut sm_chirho,
            file_name_chirho,
        ) {
            Ok(result_chirho) => {
                self.status_msg_chirho = format!(
                    "Typecheck OK: {}",
                    result_chirho.module_chirho.name_chirho.text_chirho()
                );
            }
            Err(diagnostics_chirho) => {
                let count_chirho = diagnostics_chirho.diagnostics_chirho().len();
                let first_chirho = diagnostics_chirho
                    .diagnostics_chirho()
                    .first()
                    .map(|d_chirho| d_chirho.message_chirho.as_str())
                    .unwrap_or("unknown error");
                self.status_msg_chirho = format!("{} error(s): {}", count_chirho, first_chirho);
            }
        }
    }

    pub fn start_eval_prompt_chirho(&mut self) {
        self.prompt_mode_chirho = Some(PromptModeChirho::EvalChirho {
            input_chirho: String::new(),
        });
        self.status_msg_chirho = "Eval Haskell expression:".to_string();
    }

    pub fn start_goto_line_prompt_chirho(&mut self) {
        self.prompt_mode_chirho = Some(PromptModeChirho::GotoLineChirho {
            input_chirho: String::new(),
        });
        self.status_msg_chirho = "Go to line:".to_string();
    }

    pub fn handle_prompt_key_chirho(&mut self, key_chirho: event::KeyEvent) -> io::Result<bool> {
        let Some(prompt_mode_chirho) = self.prompt_mode_chirho.as_mut() else {
            return Ok(false);
        };

        match key_chirho.code {
            event::KeyCode::Esc => {
                self.prompt_mode_chirho = None;
                self.status_msg_chirho = "Cancelled prompt".to_string();
                Ok(true)
            }
            event::KeyCode::Enter => {
                let prompt_result_chirho = match prompt_mode_chirho {
                    PromptModeChirho::EvalChirho { input_chirho } => {
                        PromptSubmitChirho::EvalChirho(input_chirho.clone())
                    }
                    PromptModeChirho::GotoLineChirho { input_chirho } => {
                        PromptSubmitChirho::GotoLineChirho(input_chirho.clone())
                    }
                };
                self.prompt_mode_chirho = None;
                self.finish_prompt_chirho(prompt_result_chirho);
                Ok(true)
            }
            event::KeyCode::Backspace => {
                match prompt_mode_chirho {
                    PromptModeChirho::EvalChirho { input_chirho }
                    | PromptModeChirho::GotoLineChirho { input_chirho } => {
                        input_chirho.pop();
                    }
                }
                Ok(true)
            }
            event::KeyCode::Char(c_chirho) => {
                match prompt_mode_chirho {
                    PromptModeChirho::EvalChirho { input_chirho } => {
                        input_chirho.push(c_chirho);
                    }
                    PromptModeChirho::GotoLineChirho { input_chirho } => {
                        if c_chirho.is_ascii_digit() {
                            input_chirho.push(c_chirho);
                        }
                    }
                }
                Ok(true)
            }
            _ => Ok(true),
        }
    }

    fn finish_prompt_chirho(&mut self, prompt_submit_chirho: PromptSubmitChirho) {
        match prompt_submit_chirho {
            PromptSubmitChirho::EvalChirho(expression_chirho) => {
                self.eval_expression_chirho(&expression_chirho);
            }
            PromptSubmitChirho::GotoLineChirho(line_text_chirho) => {
                self.jump_to_line_chirho(&line_text_chirho);
            }
        }
    }

    pub fn eval_expression_chirho(&mut self, expression_chirho: &str) {
        let expression_chirho = expression_chirho.trim();
        if expression_chirho.is_empty() {
            self.status_msg_chirho = "Eval skipped: empty expression".to_string();
            return;
        }

        match eval_haskell_expression_chirho(expression_chirho) {
            Ok(output_chirho) => {
                self.status_msg_chirho = if output_chirho.is_empty() {
                    "Eval complete (no stdout)".to_string()
                } else {
                    format!("Eval => {}", output_chirho)
                };
            }
            Err(error_chirho) => {
                self.status_msg_chirho = compact_status_msg_chirho(&error_chirho);
            }
        }
    }

    pub fn jump_to_line_chirho(&mut self, line_text_chirho: &str) {
        let Ok(line_number_chirho) = line_text_chirho.trim().parse::<usize>() else {
            self.status_msg_chirho = "Goto line failed: invalid line number".to_string();
            return;
        };
        if self.lines_chirho.is_empty() {
            self.status_msg_chirho = "Goto line failed: buffer is empty".to_string();
            return;
        }
        let target_row_chirho = line_number_chirho
            .saturating_sub(1)
            .min(self.lines_chirho.len().saturating_sub(1));
        self.cursor_row_chirho = target_row_chirho;
        let max_col_chirho = self.lines_chirho[target_row_chirho].text_chirho.len();
        self.cursor_col_chirho = self.cursor_col_chirho.min(max_col_chirho);
        self.scroll_offset_chirho = target_row_chirho;
        self.status_msg_chirho = format!("Moved to line {}", target_row_chirho + 1);
    }

    pub fn insert_char_chirho(&mut self, c_chirho: char) {
        if self.cursor_row_chirho < self.lines_chirho.len() {
            let line_chirho = &mut self.lines_chirho[self.cursor_row_chirho];
            let col_chirho = self.cursor_col_chirho.min(line_chirho.text_chirho.len());
            line_chirho.text_chirho.insert(col_chirho, c_chirho);
            self.cursor_col_chirho = col_chirho + 1;
            self.modified_chirho = true;
        }
    }

    pub fn insert_newline_chirho(&mut self) {
        if self.cursor_row_chirho < self.lines_chirho.len() {
            let line_chirho = &self.lines_chirho[self.cursor_row_chirho];
            let col_chirho = self.cursor_col_chirho.min(line_chirho.text_chirho.len());
            let rest_chirho = line_chirho.text_chirho[col_chirho..].to_string();
            self.lines_chirho[self.cursor_row_chirho].text_chirho =
                line_chirho.text_chirho[..col_chirho].to_string();
            self.cursor_row_chirho += 1;
            self.lines_chirho.insert(
                self.cursor_row_chirho,
                LineChirho {
                    text_chirho: rest_chirho,
                },
            );
            self.cursor_col_chirho = 0;
            self.modified_chirho = true;
        }
    }

    pub fn delete_char_chirho(&mut self) {
        if self.cursor_col_chirho > 0 && self.cursor_row_chirho < self.lines_chirho.len() {
            self.cursor_col_chirho -= 1;
            self.lines_chirho[self.cursor_row_chirho]
                .text_chirho
                .remove(self.cursor_col_chirho);
            self.modified_chirho = true;
        } else if self.cursor_col_chirho == 0 && self.cursor_row_chirho > 0 {
            let current_chirho = self.lines_chirho.remove(self.cursor_row_chirho);
            self.cursor_row_chirho -= 1;
            self.cursor_col_chirho = self.lines_chirho[self.cursor_row_chirho].text_chirho.len();
            self.lines_chirho[self.cursor_row_chirho]
                .text_chirho
                .push_str(&current_chirho.text_chirho);
            self.modified_chirho = true;
        }
    }
}

/// Run the editor TUI.
pub fn run_editor_chirho(file_path_chirho: Option<&str>) -> io::Result<()> {
    let mut editor_chirho = EditorChirho::new_chirho();
    if let Some(path_chirho) = file_path_chirho {
        editor_chirho.open_file_chirho(path_chirho)?;
    }

    terminal::enable_raw_mode()?;
    let mut stdout_chirho = io::stdout();
    execute!(stdout_chirho, terminal::EnterAlternateScreen, cursor::Show)?;

    while editor_chirho.running_chirho {
        draw_screen_chirho(&mut stdout_chirho, &editor_chirho)?;

        if let event::Event::Key(key_chirho) = event::read()? {
            if editor_chirho.prompt_mode_chirho.is_some()
                && editor_chirho.handle_prompt_key_chirho(key_chirho)?
            {
                continue;
            }
            match key_chirho.code {
                event::KeyCode::Char('q')
                    if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    editor_chirho.running_chirho = false;
                }
                event::KeyCode::Char('s')
                    if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    editor_chirho.save_file_chirho()?;
                }
                event::KeyCode::Char('e')
                    if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    editor_chirho.start_eval_prompt_chirho();
                }
                event::KeyCode::Char('g')
                    if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    editor_chirho.start_goto_line_prompt_chirho();
                }
                event::KeyCode::Char('t')
                    if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    editor_chirho.typecheck_chirho();
                }
                event::KeyCode::Char(c_chirho) => editor_chirho.insert_char_chirho(c_chirho),
                event::KeyCode::Enter => editor_chirho.insert_newline_chirho(),
                event::KeyCode::Backspace => editor_chirho.delete_char_chirho(),
                event::KeyCode::Left => {
                    if editor_chirho.cursor_col_chirho > 0 {
                        editor_chirho.cursor_col_chirho -= 1;
                    }
                }
                event::KeyCode::Right => {
                    if editor_chirho.cursor_row_chirho < editor_chirho.lines_chirho.len() {
                        let len_chirho = editor_chirho.lines_chirho
                            [editor_chirho.cursor_row_chirho]
                            .text_chirho
                            .len();
                        if editor_chirho.cursor_col_chirho < len_chirho {
                            editor_chirho.cursor_col_chirho += 1;
                        }
                    }
                }
                event::KeyCode::Up => {
                    if editor_chirho.cursor_row_chirho > 0 {
                        editor_chirho.cursor_row_chirho -= 1;
                    }
                }
                event::KeyCode::Down => {
                    if editor_chirho.cursor_row_chirho + 1 < editor_chirho.lines_chirho.len() {
                        editor_chirho.cursor_row_chirho += 1;
                    }
                }
                _ => {}
            }
        }
    }

    execute!(stdout_chirho, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_screen_chirho(
    stdout_chirho: &mut io::Stdout,
    editor_chirho: &EditorChirho,
) -> io::Result<()> {
    let (cols_chirho, rows_chirho) = terminal::size()?;
    let rows_chirho = rows_chirho as usize;
    let cols_chirho = cols_chirho as usize;
    let gutter_width_chirho = line_number_gutter_width_chirho(editor_chirho.lines_chirho.len());
    let text_cols_chirho = cols_chirho.saturating_sub(gutter_width_chirho);

    queue!(
        stdout_chirho,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )?;

    // Draw lines
    let visible_rows_chirho = rows_chirho.saturating_sub(2);
    for i_chirho in 0..visible_rows_chirho {
        let line_idx_chirho = editor_chirho.scroll_offset_chirho + i_chirho;
        queue!(stdout_chirho, cursor::MoveTo(0, i_chirho as u16))?;
        if line_idx_chirho < editor_chirho.lines_chirho.len() {
            let line_number_chirho = format!(
                "{:>width$} ",
                line_idx_chirho + 1,
                width = gutter_width_chirho - 1
            );
            queue!(
                stdout_chirho,
                style::SetForegroundColor(Color::DarkGrey),
                style::Print(line_number_chirho),
                style::SetForegroundColor(Color::Reset),
            )?;
            let text_chirho = &editor_chirho.lines_chirho[line_idx_chirho].text_chirho;
            draw_highlighted_line_chirho(stdout_chirho, text_chirho, text_cols_chirho)?;
        } else {
            let blank_gutter_chirho = " ".repeat(gutter_width_chirho.saturating_sub(1));
            queue!(
                stdout_chirho,
                style::SetForegroundColor(Color::DarkGrey),
                style::Print(format!("{}~", blank_gutter_chirho)),
                style::SetForegroundColor(Color::Reset),
            )?;
        }
    }

    // Status bar
    let status_chirho = format!(
        " {} {} L{}/{}",
        editor_chirho.file_path_chirho.as_deref().unwrap_or("[new]"),
        if editor_chirho.modified_chirho {
            "[+]"
        } else {
            ""
        },
        editor_chirho.cursor_row_chirho + 1,
        editor_chirho.lines_chirho.len(),
    );
    queue!(
        stdout_chirho,
        cursor::MoveTo(0, (rows_chirho - 2) as u16),
        style::Print(status_chirho.clone().on_dark_grey().white()),
    )?;

    // Message bar
    queue!(
        stdout_chirho,
        cursor::MoveTo(0, (rows_chirho - 1) as u16),
        style::Print(prompt_or_status_line_chirho(editor_chirho, cols_chirho)),
    )?;

    // Cursor
    let cursor_row_chirho = editor_chirho.cursor_row_chirho - editor_chirho.scroll_offset_chirho;
    queue!(
        stdout_chirho,
        cursor::MoveTo(
            (gutter_width_chirho + editor_chirho.cursor_col_chirho) as u16,
            cursor_row_chirho as u16,
        ),
    )?;

    stdout_chirho.flush()?;
    Ok(())
}

enum PromptModeChirho {
    EvalChirho { input_chirho: String },
    GotoLineChirho { input_chirho: String },
}

enum PromptSubmitChirho {
    EvalChirho(String),
    GotoLineChirho(String),
}

fn draw_highlighted_line_chirho(
    stdout_chirho: &mut io::Stdout,
    text_chirho: &str,
    cols_chirho: usize,
) -> io::Result<()> {
    let mut used_cols_chirho = 0usize;
    for (segment_chirho, color_chirho) in highlight_segments_chirho(text_chirho) {
        if used_cols_chirho >= cols_chirho {
            break;
        }
        let mut clipped_segment_chirho = String::new();
        for ch_chirho in segment_chirho.chars() {
            if used_cols_chirho >= cols_chirho {
                break;
            }
            clipped_segment_chirho.push(ch_chirho);
            used_cols_chirho += 1;
        }
        if clipped_segment_chirho.is_empty() {
            continue;
        }
        queue!(
            stdout_chirho,
            style::SetForegroundColor(color_chirho),
            style::Print(clipped_segment_chirho),
            style::SetForegroundColor(Color::Reset),
        )?;
    }
    Ok(())
}

fn prompt_or_status_line_chirho(editor_chirho: &EditorChirho, cols_chirho: usize) -> String {
    let base_text_chirho = match &editor_chirho.prompt_mode_chirho {
        Some(PromptModeChirho::EvalChirho { input_chirho }) => {
            format!("Eval Haskell> {}", input_chirho)
        }
        Some(PromptModeChirho::GotoLineChirho { input_chirho }) => {
            format!("Go to line> {}", input_chirho)
        }
        None => editor_chirho.status_msg_chirho.clone(),
    };
    if base_text_chirho.chars().count() <= cols_chirho {
        return base_text_chirho;
    }
    base_text_chirho.chars().take(cols_chirho).collect()
}

fn compact_status_msg_chirho(text_chirho: &str) -> String {
    let compacted_chirho = text_chirho
        .lines()
        .map(str::trim)
        .filter(|line_chirho| !line_chirho.is_empty())
        .take(2)
        .collect::<Vec<_>>()
        .join(" | ");
    if compacted_chirho.is_empty() {
        "Operation failed".to_string()
    } else {
        compacted_chirho
    }
}

fn line_number_gutter_width_chirho(line_count_chirho: usize) -> usize {
    line_count_chirho.max(1).to_string().len() + 1
}

fn eval_haskell_expression_chirho(expression_chirho: &str) -> Result<String, String> {
    let source_chirho = format!(
        "mainChirho :: IO ()\nmainChirho = print (({}))\n",
        expression_chirho
    );
    let mut source_map_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    match haskelujah_driver_chirho::compile_source_chirho(
        &source_chirho,
        &mut source_map_chirho,
        "editor-eval-chirho.hs",
    ) {
        Ok(_) => match haskelujah_driver_chirho::eval_source_with_machine_chirho(
            &source_chirho,
            &mut source_map_chirho,
            "editor-eval-chirho.hs",
            Some("mainChirho"),
        ) {
            Ok((_value_chirho, machine_chirho)) => {
                Ok(machine_chirho.io_output_chirho.trim().to_string())
            }
            Err(error_chirho) => Err(error_chirho),
        },
        Err(diagnostics_chirho) => Err(haskelujah_driver_chirho::render_diagnostics_chirho(
            &diagnostics_chirho,
            &source_map_chirho,
            false,
        )),
    }
}

fn highlight_segments_chirho(text_chirho: &str) -> Vec<(String, Color)> {
    let chars_chirho: Vec<char> = text_chirho.chars().collect();
    let mut segments_chirho = Vec::new();
    let mut idx_chirho = 0usize;

    while idx_chirho < chars_chirho.len() {
        if chars_chirho[idx_chirho] == '-' && chars_chirho.get(idx_chirho + 1) == Some(&'-') {
            segments_chirho.push((chars_chirho[idx_chirho..].iter().collect(), Color::DarkGrey));
            break;
        }

        if chars_chirho[idx_chirho] == '{' && chars_chirho.get(idx_chirho + 1) == Some(&'-') {
            let start_chirho = idx_chirho;
            idx_chirho += 2;
            while idx_chirho + 1 < chars_chirho.len() {
                if chars_chirho[idx_chirho] == '-' && chars_chirho[idx_chirho + 1] == '}' {
                    idx_chirho += 2;
                    break;
                }
                idx_chirho += 1;
            }
            segments_chirho.push((
                chars_chirho[start_chirho..idx_chirho].iter().collect(),
                Color::DarkGrey,
            ));
            continue;
        }

        if chars_chirho[idx_chirho] == '"' {
            let start_chirho = idx_chirho;
            idx_chirho += 1;
            while idx_chirho < chars_chirho.len() {
                let current_chirho = chars_chirho[idx_chirho];
                idx_chirho += 1;
                if current_chirho == '"'
                    && chars_chirho.get(idx_chirho.wrapping_sub(2)) != Some(&'\\')
                {
                    break;
                }
            }
            segments_chirho.push((
                chars_chirho[start_chirho..idx_chirho].iter().collect(),
                Color::Green,
            ));
            continue;
        }

        if chars_chirho[idx_chirho].is_ascii_alphabetic() || chars_chirho[idx_chirho] == '_' {
            let start_chirho = idx_chirho;
            idx_chirho += 1;
            while idx_chirho < chars_chirho.len()
                && (chars_chirho[idx_chirho].is_ascii_alphanumeric()
                    || chars_chirho[idx_chirho] == '_'
                    || chars_chirho[idx_chirho] == '\'')
            {
                idx_chirho += 1;
            }
            let token_chirho: String = chars_chirho[start_chirho..idx_chirho].iter().collect();
            let color_chirho = if is_haskell_keyword_chirho(&token_chirho) {
                Color::Rgb {
                    r: 212,
                    g: 175,
                    b: 55,
                }
            } else if token_chirho
                .chars()
                .next()
                .is_some_and(|first_chirho| first_chirho.is_ascii_uppercase())
            {
                Color::Cyan
            } else {
                Color::White
            };
            segments_chirho.push((token_chirho, color_chirho));
            continue;
        }

        segments_chirho.push((chars_chirho[idx_chirho].to_string(), Color::White));
        idx_chirho += 1;
    }

    segments_chirho
}

fn is_haskell_keyword_chirho(token_chirho: &str) -> bool {
    matches!(
        token_chirho,
        "module"
            | "where"
            | "let"
            | "in"
            | "do"
            | "case"
            | "of"
            | "if"
            | "then"
            | "else"
            | "class"
            | "instance"
            | "data"
            | "type"
            | "newtype"
            | "import"
    )
}

#[cfg(test)]
mod tests_chirho {
    use super::{
        Color, EditorChirho, eval_haskell_expression_chirho, highlight_segments_chirho,
        is_haskell_keyword_chirho, line_number_gutter_width_chirho, prompt_or_status_line_chirho,
    };

    #[test]
    fn highlight_keywords_types_comments_and_strings_chirho() {
        let segments_chirho = highlight_segments_chirho("module Demo where -- comment \"ignored\"");
        assert!(segments_chirho.iter().any(|(text_chirho, color_chirho)| {
            text_chirho == "module"
                && *color_chirho
                    == Color::Rgb {
                        r: 212,
                        g: 175,
                        b: 55,
                    }
        }));
        assert!(segments_chirho.iter().any(|(text_chirho, color_chirho)| {
            text_chirho == "Demo" && *color_chirho == Color::Cyan
        }));
        assert!(segments_chirho.iter().any(|(text_chirho, color_chirho)| {
            text_chirho.starts_with("--") && *color_chirho == Color::DarkGrey
        }));
    }

    #[test]
    fn haskell_keyword_table_chirho() {
        assert!(is_haskell_keyword_chirho("let"));
        assert!(!is_haskell_keyword_chirho("map"));
    }

    #[test]
    fn eval_haskell_expression_returns_rendered_result_chirho() {
        let output_chirho =
            eval_haskell_expression_chirho("map (+1) [1,2,3]").expect("eval should succeed");
        assert_eq!(output_chirho, "[2,3,4]");
    }

    #[test]
    fn editor_eval_updates_status_bar_chirho() {
        let mut editor_chirho = EditorChirho::new_chirho();
        editor_chirho.eval_expression_chirho("sum [1,2,3]");
        assert_eq!(editor_chirho.status_msg_chirho, "Eval => 6");
    }

    #[test]
    fn goto_line_clamps_and_updates_status_chirho() {
        let mut editor_chirho = EditorChirho::new_chirho();
        editor_chirho.lines_chirho = vec![
            super::LineChirho {
                text_chirho: "one".to_string(),
            },
            super::LineChirho {
                text_chirho: "two".to_string(),
            },
        ];
        editor_chirho.jump_to_line_chirho("99");
        assert_eq!(editor_chirho.cursor_row_chirho, 1);
        assert_eq!(editor_chirho.scroll_offset_chirho, 1);
        assert_eq!(editor_chirho.status_msg_chirho, "Moved to line 2");
    }

    #[test]
    fn goto_prompt_renders_input_chirho() {
        let mut editor_chirho = EditorChirho::new_chirho();
        editor_chirho.start_goto_line_prompt_chirho();
        if let Some(super::PromptModeChirho::GotoLineChirho { input_chirho }) =
            editor_chirho.prompt_mode_chirho.as_mut()
        {
            input_chirho.push_str("12");
        }
        assert_eq!(
            prompt_or_status_line_chirho(&editor_chirho, 80),
            "Go to line> 12"
        );
    }

    #[test]
    fn gutter_width_tracks_line_count_chirho() {
        assert_eq!(line_number_gutter_width_chirho(1), 2);
        assert_eq!(line_number_gutter_width_chirho(99), 3);
        assert_eq!(line_number_gutter_width_chirho(100), 4);
    }
}
