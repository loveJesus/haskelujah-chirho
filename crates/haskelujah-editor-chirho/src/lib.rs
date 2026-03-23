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
    style::{self, Stylize},
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
            status_msg_chirho: "Haskelujah Editor — Ctrl-Q quit, Ctrl-S save, Ctrl-T typecheck"
                .to_string(),
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
        let file_name_chirho = self
            .file_path_chirho
            .as_deref()
            .unwrap_or("Buffer.hs");
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
                self.status_msg_chirho =
                    format!("{} error(s): {}", count_chirho, first_chirho);
            }
        }
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
            match key_chirho.code {
                event::KeyCode::Char('q') if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    editor_chirho.running_chirho = false;
                }
                event::KeyCode::Char('s') if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    editor_chirho.save_file_chirho()?;
                }
                event::KeyCode::Char('t') if key_chirho.modifiers.contains(event::KeyModifiers::CONTROL) => {
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
                        let len_chirho = editor_chirho.lines_chirho[editor_chirho.cursor_row_chirho].text_chirho.len();
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

    queue!(stdout_chirho, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;

    // Draw lines
    let visible_rows_chirho = rows_chirho.saturating_sub(2);
    for i_chirho in 0..visible_rows_chirho {
        let line_idx_chirho = editor_chirho.scroll_offset_chirho + i_chirho;
        queue!(stdout_chirho, cursor::MoveTo(0, i_chirho as u16))?;
        if line_idx_chirho < editor_chirho.lines_chirho.len() {
            let text_chirho = &editor_chirho.lines_chirho[line_idx_chirho].text_chirho;
            let display_chirho = if text_chirho.len() > cols_chirho {
                &text_chirho[..cols_chirho]
            } else {
                text_chirho
            };
            queue!(stdout_chirho, style::Print(display_chirho))?;
        } else {
            queue!(stdout_chirho, style::Print("~".dark_grey()))?;
        }
    }

    // Status bar
    let status_chirho = format!(
        " {} {} L{}/{}",
        editor_chirho
            .file_path_chirho
            .as_deref()
            .unwrap_or("[new]"),
        if editor_chirho.modified_chirho { "[+]" } else { "" },
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
        style::Print(&editor_chirho.status_msg_chirho),
    )?;

    // Cursor
    let cursor_row_chirho = editor_chirho.cursor_row_chirho - editor_chirho.scroll_offset_chirho;
    queue!(
        stdout_chirho,
        cursor::MoveTo(
            editor_chirho.cursor_col_chirho as u16,
            cursor_row_chirho as u16,
        ),
    )?;

    stdout_chirho.flush()?;
    Ok(())
}
