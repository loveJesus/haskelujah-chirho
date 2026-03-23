// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-lsp-chirho
//!
//! Language Server Protocol implementation for the Haskelujah compiler.

use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::*;

/// Run the LSP server on stdio.
pub fn run_lsp_chirho() -> Result<(), Box<dyn std::error::Error>> {
    let (connection_chirho, io_threads_chirho) = Connection::stdio();

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
                handle_request_chirho(&connection_chirho, req_chirho);
            }
            Message::Notification(notif_chirho) => {
                handle_notification_chirho(&connection_chirho, notif_chirho);
            }
            Message::Response(_) => {}
        }
    }

    io_threads_chirho.join()?;
    Ok(())
}

fn handle_request_chirho(conn_chirho: &Connection, req_chirho: Request) {
    if req_chirho.method == "textDocument/hover" {
        let hover_chirho = Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "Haskelujah: hover types coming soon".to_string(),
            )),
            range: None,
        };
        let resp_chirho = Response::new_ok(
            req_chirho.id,
            Some(serde_json::to_value(hover_chirho).unwrap_or_default()),
        );
        conn_chirho.sender.send(Message::Response(resp_chirho)).ok();
    }
}

fn handle_notification_chirho(conn_chirho: &Connection, notif_chirho: Notification) {
    if notif_chirho.method == "textDocument/didOpen" {
        if let Ok(params_chirho) =
            serde_json::from_value::<DidOpenTextDocumentParams>(notif_chirho.params)
        {
            publish_diagnostics_chirho(
                conn_chirho,
                params_chirho.text_document.uri,
                &params_chirho.text_document.text,
            );
        }
    } else if notif_chirho.method == "textDocument/didChange" {
        if let Ok(params_chirho) =
            serde_json::from_value::<DidChangeTextDocumentParams>(notif_chirho.params)
        {
            if let Some(change_chirho) = params_chirho.content_changes.last() {
                publish_diagnostics_chirho(
                    conn_chirho,
                    params_chirho.text_document.uri,
                    &change_chirho.text,
                );
            }
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
        Err(bundle_chirho) => bundle_chirho
            .diagnostics_chirho()
            .iter()
            .map(|d_chirho| Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                message: d_chirho.message_chirho.clone(),
                ..Default::default()
            })
            .collect(),
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
