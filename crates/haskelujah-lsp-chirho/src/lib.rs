// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-lsp-chirho
//!
//! Language Server Protocol implementation for the Haskelujah compiler.

use std::collections::HashMap;

use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, SeverityChirho};
use haskelujah_driver_chirho::{
    compile_source_chirho, preprocess_cpp_chirho, run_frontend_chirho,
};
use haskelujah_naming_chirho::builtin_module_ifaces_chirho;
use haskelujah_span_chirho::{LineColChirho, SourceMapChirho, SpanChirho};
use lsp_server::{Connection, Message, Notification, Request, Response, ResponseError};
use lsp_types::{
    Diagnostic, DiagnosticSeverity, DidOpenTextDocumentParams, Hover, HoverContents,
    HoverParams, HoverProviderCapability, InitializeParams, MarkedString, Position,
    PublishDiagnosticsParams, Range, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, Uri,
};

#[derive(Debug, Clone)]
struct OpenDocumentChirho {
    uri_chirho: Uri,
    source_chirho: String,
}

/// Run the LSP server on stdio.
pub fn run_lsp_chirho() -> Result<(), Box<dyn std::error::Error>> {
    let (connection_chirho, io_threads_chirho) = Connection::stdio();
    let mut open_documents_chirho: HashMap<String, OpenDocumentChirho> = HashMap::new();

    let capabilities_chirho = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })?;

    let init_params_chirho = connection_chirho.initialize(capabilities_chirho)?;
    let _params_chirho: InitializeParams = serde_json::from_value(init_params_chirho)?;

    eprintln!("haskelujah LSP server initialized");

    for msg_chirho in &connection_chirho.receiver {
        match msg_chirho {
            Message::Request(req_chirho) => {
                if connection_chirho.handle_shutdown(&req_chirho)? {
                    break;
                }
                handle_request_chirho(&connection_chirho, req_chirho, &open_documents_chirho);
            }
            Message::Notification(notif_chirho) => {
                handle_notification_chirho(
                    &connection_chirho,
                    notif_chirho,
                    &mut open_documents_chirho,
                );
            }
            Message::Response(_) => {}
        }
    }

    io_threads_chirho.join()?;
    Ok(())
}

fn handle_request_chirho(
    conn_chirho: &Connection,
    req_chirho: Request,
    open_documents_chirho: &HashMap<String, OpenDocumentChirho>,
) {
    if req_chirho.method == "textDocument/hover" {
        let hover_result_chirho = serde_json::from_value::<HoverParams>(req_chirho.params.clone())
            .ok()
            .and_then(|params_chirho| hover_result_for_params_chirho(&params_chirho, open_documents_chirho).ok())
            .flatten();
        let resp_chirho = Response::new_ok(req_chirho.id, hover_result_chirho);
        conn_chirho
            .sender
            .send(Message::Response(resp_chirho))
            .ok();
    } else {
        let resp_chirho = Response::new_err(
            req_chirho.id,
            -32601,
            format!("unsupported request: {}", req_chirho.method),
        );
        conn_chirho
            .sender
            .send(Message::Response(resp_chirho))
            .ok();
    }
}

fn handle_notification_chirho(
    conn_chirho: &Connection,
    notif_chirho: Notification,
    open_documents_chirho: &mut HashMap<String, OpenDocumentChirho>,
) {
    if notif_chirho.method == "textDocument/didOpen" {
        if let Ok(params_chirho) =
            serde_json::from_value::<DidOpenTextDocumentParams>(notif_chirho.params)
        {
            open_documents_chirho.insert(
                params_chirho.text_document.uri.to_string(),
                OpenDocumentChirho {
                    uri_chirho: params_chirho.text_document.uri.clone(),
                    source_chirho: params_chirho.text_document.text.clone(),
                },
            );
            publish_diagnostics_chirho(
                conn_chirho,
                params_chirho.text_document.uri,
                &params_chirho.text_document.text,
            );
        }
    }
}

fn publish_diagnostics_chirho(conn_chirho: &Connection, uri_chirho: Uri, source_chirho: &str) {
    let file_name_chirho = uri_chirho
        .as_str()
        .rsplit('/')
        .next()
        .unwrap_or("Main.hs");

    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let diagnostics_chirho = match haskelujah_driver_chirho::compile_source_chirho(
        source_chirho,
        &mut sm_chirho,
        file_name_chirho,
    ) {
        Ok(_) => vec![],
        Err(bundle_chirho) => diagnostics_to_lsp_chirho(&bundle_chirho, &sm_chirho),
    };

    let params_chirho = PublishDiagnosticsParams {
        uri: uri_chirho,
        diagnostics: diagnostics_chirho,
        version: None,
    };

    let notif_chirho = Notification::new(
        "textDocument/publishDiagnostics".to_string(),
        serde_json::to_value(params_chirho).unwrap(),
    );
    conn_chirho
        .sender
        .send(Message::Notification(notif_chirho))
        .ok();
}

fn hover_result_for_params_chirho(
    params_chirho: &HoverParams,
    open_documents_chirho: &HashMap<String, OpenDocumentChirho>,
) -> Result<Option<Hover>, Box<dyn std::error::Error>> {
    let uri_key_chirho = params_chirho
        .text_document_position_params
        .text_document
        .uri
        .to_string();
    let Some(document_chirho) = open_documents_chirho.get(&uri_key_chirho) else {
        return Ok(None);
    };

    let preprocessed_source_chirho = preprocess_cpp_chirho(&document_chirho.source_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let file_id_chirho = source_map_chirho.add_file_chirho(
        document_chirho.uri_chirho.to_string(),
        preprocessed_source_chirho.clone(),
    );
    let builtin_ifaces_chirho = builtin_module_ifaces_chirho();
    let imported_types_chirho = HashMap::new();
    let frontend_result_chirho = match run_frontend_chirho(
        &preprocessed_source_chirho,
        file_id_chirho,
        &builtin_ifaces_chirho,
        &imported_types_chirho,
    ) {
        Ok(frontend_result_chirho) => frontend_result_chirho,
        Err(_) => return Ok(None),
    };

    let byte_offset_chirho = match position_to_byte_offset_chirho(
        &preprocessed_source_chirho,
        params_chirho.text_document_position_params.position,
    ) {
        Some(byte_offset_chirho) => byte_offset_chirho,
        None => return Ok(None),
    };
    let Some((token_chirho, range_chirho)) =
        extract_hover_token_chirho(&preprocessed_source_chirho, byte_offset_chirho)
    else {
        return Ok(None);
    };

    let scheme_chirho = frontend_result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho(&token_chirho)
        .or_else(|| {
            token_chirho.rsplit('.').next().and_then(|name_chirho| {
                frontend_result_chirho
                    .infer_result_chirho
                    .env_chirho
                    .lookup_chirho(name_chirho)
            })
        });
    let Some(scheme_chirho) = scheme_chirho else {
        return Ok(None);
    };

    Ok(Some(Hover {
        contents: HoverContents::Scalar(MarkedString::String(format!("{}", scheme_chirho))),
        range: Some(range_chirho),
    }))
}

fn diagnostics_to_lsp_chirho(
    bundle_chirho: &DiagnosticBundleChirho,
    source_map_chirho: &SourceMapChirho,
) -> Vec<Diagnostic> {
    bundle_chirho
        .diagnostics_chirho()
        .iter()
        .map(|diagnostic_chirho| diagnostic_to_lsp_chirho(diagnostic_chirho, source_map_chirho))
        .collect()
}

fn diagnostic_to_lsp_chirho(
    diagnostic_chirho: &DiagnosticChirho,
    source_map_chirho: &SourceMapChirho,
) -> Diagnostic {
    Diagnostic {
        range: span_to_range_chirho(
            diagnostic_chirho
                .primary_span_chirho()
                .unwrap_or(SpanChirho::DUMMY_CHIRHO),
            source_map_chirho,
        ),
        severity: Some(match diagnostic_chirho.severity_chirho {
            SeverityChirho::ErrorChirho => DiagnosticSeverity::ERROR,
            SeverityChirho::WarningChirho => DiagnosticSeverity::WARNING,
            SeverityChirho::InfoChirho => DiagnosticSeverity::INFORMATION,
            SeverityChirho::HintChirho => DiagnosticSeverity::HINT,
        }),
        code: diagnostic_chirho
            .code_chirho
            .map(|code_chirho| lsp_types::NumberOrString::String(code_chirho.to_string())),
        message: diagnostic_chirho.message_chirho.clone(),
        source: Some("haskelujah".to_string()),
        ..Default::default()
    }
}

fn span_to_range_chirho(span_chirho: SpanChirho, source_map_chirho: &SourceMapChirho) -> Range {
    if span_chirho == SpanChirho::DUMMY_CHIRHO {
        return Range::new(Position::new(0, 0), Position::new(0, 0));
    }

    let Some(line_index_chirho) = source_map_chirho.line_index_chirho(span_chirho.file_id_chirho())
    else {
        return Range::new(Position::new(0, 0), Position::new(0, 0));
    };
    let source_len_chirho = source_map_chirho
        .file_source_chirho(span_chirho.file_id_chirho())
        .map(str::len)
        .unwrap_or(0);
    let clamped_span_chirho = span_chirho.clamp_to_source_chirho(source_len_chirho);
    let start_line_col_chirho = line_index_chirho.line_col_chirho(clamped_span_chirho.start_chirho());
    let end_line_col_chirho = line_index_chirho.line_col_chirho(clamped_span_chirho.end_chirho());
    Range::new(
        line_col_to_position_chirho(start_line_col_chirho),
        line_col_to_position_chirho(end_line_col_chirho),
    )
}

fn line_col_to_position_chirho(line_col_chirho: LineColChirho) -> Position {
    Position::new(
        line_col_chirho.line_chirho.saturating_sub(1),
        line_col_chirho.col_chirho.saturating_sub(1),
    )
}

fn position_to_byte_offset_chirho(source_chirho: &str, position_chirho: Position) -> Option<usize> {
    let target_line_chirho = position_chirho.line as usize;
    let target_char_chirho = position_chirho.character as usize;
    let mut byte_index_chirho = 0usize;

    for (line_index_chirho, line_chirho) in source_chirho.split_inclusive('\n').enumerate() {
        if line_index_chirho == target_line_chirho {
            let line_body_chirho = line_chirho.strip_suffix('\n').unwrap_or(line_chirho);
            let byte_in_line_chirho = line_body_chirho
                .char_indices()
                .nth(target_char_chirho)
                .map(|(idx_chirho, _)| idx_chirho)
                .unwrap_or_else(|| line_body_chirho.len());
            return Some(byte_index_chirho + byte_in_line_chirho);
        }
        byte_index_chirho += line_chirho.len();
    }

    if target_line_chirho == source_chirho.lines().count() {
        Some(source_chirho.len())
    } else {
        None
    }
}

fn extract_hover_token_chirho(
    source_chirho: &str,
    byte_offset_chirho: usize,
) -> Option<(String, Range)> {
    let (start_chirho, end_chirho) =
        token_bounds_at_offset_chirho(source_chirho, byte_offset_chirho)?;
    let token_chirho = source_chirho.get(start_chirho..end_chirho)?.to_string();
    Some((
        token_chirho,
        Range::new(
            byte_offset_to_position_chirho(source_chirho, start_chirho),
            byte_offset_to_position_chirho(source_chirho, end_chirho),
        ),
    ))
}

fn token_bounds_at_offset_chirho(
    source_chirho: &str,
    byte_offset_chirho: usize,
) -> Option<(usize, usize)> {
    if source_chirho.is_empty() {
        return None;
    }
    let max_offset_chirho = source_chirho.len().saturating_sub(1);
    let clamped_offset_chirho = byte_offset_chirho.min(max_offset_chirho);
    scan_token_bounds_chirho(source_chirho, clamped_offset_chirho, is_identifier_char_chirho)
        .or_else(|| {
            clamped_offset_chirho.checked_sub(1).and_then(|prev_offset_chirho| {
                scan_token_bounds_chirho(source_chirho, prev_offset_chirho, is_identifier_char_chirho)
            })
        })
        .or_else(|| scan_token_bounds_chirho(source_chirho, clamped_offset_chirho, is_operator_char_chirho))
        .or_else(|| {
            clamped_offset_chirho.checked_sub(1).and_then(|prev_offset_chirho| {
                scan_token_bounds_chirho(source_chirho, prev_offset_chirho, is_operator_char_chirho)
            })
        })
}

fn scan_token_bounds_chirho(
    source_chirho: &str,
    byte_offset_chirho: usize,
    predicate_chirho: fn(char) -> bool,
) -> Option<(usize, usize)> {
    if !source_chirho.is_char_boundary(byte_offset_chirho) {
        return None;
    }
    let current_char_chirho = source_chirho[byte_offset_chirho..].chars().next()?;
    if !predicate_chirho(current_char_chirho) {
        return None;
    }

    let mut start_chirho = byte_offset_chirho;
    while start_chirho > 0 {
        let prev_start_chirho = source_chirho[..start_chirho].char_indices().last()?.0;
        let prev_char_chirho = source_chirho[prev_start_chirho..].chars().next()?;
        if !predicate_chirho(prev_char_chirho) {
            break;
        }
        start_chirho = prev_start_chirho;
    }

    let mut end_chirho = byte_offset_chirho + current_char_chirho.len_utf8();
    while end_chirho < source_chirho.len() {
        let next_char_chirho = source_chirho[end_chirho..].chars().next()?;
        if !predicate_chirho(next_char_chirho) {
            break;
        }
        end_chirho += next_char_chirho.len_utf8();
    }
    Some((start_chirho, end_chirho))
}

fn byte_offset_to_position_chirho(source_chirho: &str, byte_offset_chirho: usize) -> Position {
    let mut line_chirho = 0u32;
    let mut character_chirho = 0u32;
    let mut seen_bytes_chirho = 0usize;
    for char_chirho in source_chirho.chars() {
        if seen_bytes_chirho >= byte_offset_chirho {
            break;
        }
        if char_chirho == '\n' {
            line_chirho += 1;
            character_chirho = 0;
        } else {
            character_chirho += 1;
        }
        seen_bytes_chirho += char_chirho.len_utf8();
    }
    Position::new(line_chirho, character_chirho)
}

fn is_identifier_char_chirho(char_chirho: char) -> bool {
    char_chirho.is_alphanumeric() || matches!(char_chirho, '_' | '\'' | '.')
}

fn is_operator_char_chirho(char_chirho: char) -> bool {
    matches!(
        char_chirho,
        '!' | '#' | '$' | '%' | '&' | '*' | '+' | '.' | '/' | '<' | '=' | '>' | '?' | '@'
            | '\\' | '^' | '|' | '-' | '~' | ':'
    )
}
