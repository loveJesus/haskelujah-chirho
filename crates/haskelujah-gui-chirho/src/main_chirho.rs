// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-gui
//!
//! Cross-platform GUI editor for Haskell — like Zed, powered by Haskelujah.
//! Runs on macOS, Linux, Windows, and Web (via WebAssembly).

fn main() -> eframe::Result {
    let options_chirho = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Haskelujah Editor")
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Haskelujah Editor",
        options_chirho,
        Box::new(|cc_chirho| {
            Ok(Box::new(HaskelujahAppChirho::new_chirho(cc_chirho)))
        }),
    )
}

use eframe::egui;

struct HaskelujahAppChirho {
    source_chirho: String,
    file_path_chirho: Option<String>,
    diagnostics_chirho: Vec<String>,
    status_chirho: String,
    show_file_browser_chirho: bool,
    font_size_chirho: f32,
    line_numbers_chirho: bool,
}

impl HaskelujahAppChirho {
    fn new_chirho(_cc_chirho: &eframe::CreationContext) -> Self {
        Self {
            source_chirho: "-- For God so loved the world that he gave his only begotten Son,\n\
                -- that whoever believes in him should not perish but have eternal life.\n\
                -- John 3:16\n\
                \n\
                module Main where\n\
                \n\
                import Haskelujah.JSON\n\
                \n\
                main :: IO ()\n\
                main = putStrLn (encode (object [\"hello\" .= String \"world\"]))\n"
                .to_string(),
            file_path_chirho: None,
            diagnostics_chirho: Vec::new(),
            status_chirho: "Ready — Ctrl+Enter to typecheck".to_string(),
            show_file_browser_chirho: false,
            font_size_chirho: 16.0,
            line_numbers_chirho: true,
        }
    }

    fn typecheck_chirho(&mut self) {
        let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
        let file_name_chirho = self
            .file_path_chirho
            .as_deref()
            .unwrap_or("Main.hs");

        match haskelujah_driver_chirho::compile_source_chirho(
            &self.source_chirho,
            &mut sm_chirho,
            file_name_chirho,
        ) {
            Ok(result_chirho) => {
                self.diagnostics_chirho.clear();
                self.status_chirho = format!(
                    "Typecheck OK: module {}",
                    result_chirho.module_chirho.name_chirho.text_chirho()
                );
            }
            Err(bundle_chirho) => {
                self.diagnostics_chirho = bundle_chirho
                    .diagnostics_chirho()
                    .iter()
                    .map(|d_chirho| d_chirho.message_chirho.clone())
                    .collect();
                self.status_chirho = format!(
                    "{} error(s)",
                    self.diagnostics_chirho.len()
                );
            }
        }
    }

    fn open_file_chirho(&mut self, path_chirho: &str) {
        match std::fs::read_to_string(path_chirho) {
            Ok(content_chirho) => {
                self.source_chirho = content_chirho;
                self.file_path_chirho = Some(path_chirho.to_string());
                self.status_chirho = format!("Opened {}", path_chirho);
                self.diagnostics_chirho.clear();
            }
            Err(e_chirho) => {
                self.status_chirho = format!("Error: {}", e_chirho);
            }
        }
    }

    fn save_file_chirho(&mut self) {
        if let Some(path_chirho) = &self.file_path_chirho {
            match std::fs::write(path_chirho, &self.source_chirho) {
                Ok(()) => self.status_chirho = format!("Saved {}", path_chirho),
                Err(e_chirho) => self.status_chirho = format!("Save error: {}", e_chirho),
            }
        }
    }
}

impl eframe::App for HaskelujahAppChirho {
    fn update(&mut self, ctx_chirho: &egui::Context, _frame_chirho: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_chirho").show(ctx_chirho, |ui_chirho| {
            egui::menu::bar(ui_chirho, |ui_chirho| {
                ui_chirho.menu_button("File", |ui_chirho| {
                    if ui_chirho.button("Open...").clicked() {
                        if let Some(path_chirho) = rfd_open_chirho() {
                            self.open_file_chirho(&path_chirho);
                        }
                        ui_chirho.close_menu();
                    }
                    if ui_chirho.button("Save").clicked() {
                        self.save_file_chirho();
                        ui_chirho.close_menu();
                    }
                });
                ui_chirho.menu_button("Build", |ui_chirho| {
                    if ui_chirho.button("Typecheck (Ctrl+Enter)").clicked() {
                        self.typecheck_chirho();
                        ui_chirho.close_menu();
                    }
                });
                ui_chirho.menu_button("View", |ui_chirho| {
                    ui_chirho.checkbox(&mut self.line_numbers_chirho, "Line Numbers");
                    ui_chirho.add(egui::Slider::new(&mut self.font_size_chirho, 10.0..=32.0).text("Font Size"));
                });
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_chirho").show(ctx_chirho, |ui_chirho| {
            ui_chirho.horizontal(|ui_chirho| {
                let line_count_chirho = self.source_chirho.lines().count();
                ui_chirho.label(format!(
                    "{} | {} lines | {}",
                    self.file_path_chirho.as_deref().unwrap_or("[untitled]"),
                    line_count_chirho,
                    self.status_chirho,
                ));
            });
        });

        // Diagnostics panel
        if !self.diagnostics_chirho.is_empty() {
            egui::TopBottomPanel::bottom("diagnostics_chirho")
                .resizable(true)
                .min_height(60.0)
                .show(ctx_chirho, |ui_chirho| {
                    ui_chirho.heading("Diagnostics");
                    egui::ScrollArea::vertical().show(ui_chirho, |ui_chirho| {
                        for diag_chirho in &self.diagnostics_chirho {
                            ui_chirho.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                diag_chirho,
                            );
                        }
                    });
                });
        }

        // Main editor area
        egui::CentralPanel::default().show(ctx_chirho, |ui_chirho| {
            // Keyboard shortcuts
            if ctx_chirho.input(|i_chirho| {
                i_chirho.key_pressed(egui::Key::Enter)
                    && i_chirho.modifiers.ctrl
            }) {
                self.typecheck_chirho();
            }
            if ctx_chirho.input(|i_chirho| {
                i_chirho.key_pressed(egui::Key::S)
                    && i_chirho.modifiers.ctrl
            }) {
                self.save_file_chirho();
            }

            egui::ScrollArea::both().show(ui_chirho, |ui_chirho| {
                let font_id_chirho = egui::FontId::monospace(self.font_size_chirho);

                if self.line_numbers_chirho {
                    ui_chirho.horizontal_top(|ui_chirho| {
                        // Line numbers gutter
                        let line_count_chirho = self.source_chirho.lines().count().max(1);
                        let gutter_chirho: String = (1..=line_count_chirho)
                            .map(|n_chirho| format!("{:>4} ", n_chirho))
                            .collect::<Vec<_>>()
                            .join("\n");
                        ui_chirho.label(
                            egui::RichText::new(gutter_chirho)
                                .font(font_id_chirho.clone())
                                .color(egui::Color32::from_gray(100)),
                        );

                        // Source editor
                        let editor_chirho = egui::TextEdit::multiline(&mut self.source_chirho)
                            .font(font_id_chirho)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(40);
                        ui_chirho.add(editor_chirho);
                    });
                } else {
                    let editor_chirho = egui::TextEdit::multiline(&mut self.source_chirho)
                        .font(font_id_chirho)
                        .code_editor()
                        .desired_width(f32::INFINITY)
                        .desired_rows(40);
                    ui_chirho.add(editor_chirho);
                }
            });
        });
    }
}

/// Simple file open dialog (returns path or None).
fn rfd_open_chirho() -> Option<String> {
    // TODO: use rfd crate for native file dialogs
    // For now, return None (menu item exists but doesn't open dialog)
    None
}
