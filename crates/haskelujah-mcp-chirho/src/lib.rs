// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-mcp-chirho
//!
//! Model Context Protocol (MCP) server for the Haskelujah compiler.
//! Exposes Haskell type information, project structure, and compiler
//! diagnostics to AI assistants (Claude, GPT, Gemini).
//!
//! ## Protocol
//!
//! Implements the MCP specification over stdio (JSON-RPC 2.0):
//! - `tools/list` — available compiler tools
//! - `tools/call` — invoke a tool (typecheck, hover, diagnostics)

use serde::{Deserialize, Serialize};

/// An MCP JSON-RPC request.
#[derive(Debug, Deserialize)]
pub struct McpRequestChirho {
    #[serde(rename = "jsonrpc")]
    pub jsonrpc_chirho: String,
    #[serde(rename = "id")]
    pub id_chirho: serde_json::Value,
    #[serde(rename = "method")]
    pub method_chirho: String,
    #[serde(rename = "params", default)]
    pub params_chirho: serde_json::Value,
}

/// An MCP JSON-RPC response.
#[derive(Debug, Serialize)]
pub struct McpResponseChirho {
    #[serde(rename = "jsonrpc")]
    pub jsonrpc_chirho: String,
    #[serde(rename = "id")]
    pub id_chirho: serde_json::Value,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result_chirho: Option<serde_json::Value>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error_chirho: Option<McpErrorChirho>,
}

/// An MCP error object.
#[derive(Debug, Serialize)]
pub struct McpErrorChirho {
    #[serde(rename = "code")]
    pub code_chirho: i64,
    #[serde(rename = "message")]
    pub message_chirho: String,
}

/// MCP tool definition.
#[derive(Debug, Serialize)]
pub struct McpToolChirho {
    #[serde(rename = "name")]
    pub name_chirho: String,
    #[serde(rename = "description")]
    pub description_chirho: String,
    #[serde(rename = "inputSchema")]
    pub input_schema_chirho: serde_json::Value,
}

/// List the tools this MCP server exposes.
pub fn list_tools_chirho() -> Vec<McpToolChirho> {
    vec![
        McpToolChirho {
            name_chirho: "typecheck".to_string(),
            description_chirho: "Type-check a Haskell source file and return diagnostics"
                .to_string(),
            input_schema_chirho: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": { "type": "string", "description": "Path to the .hs file" }
                },
                "required": ["file_path"]
            }),
        },
        McpToolChirho {
            name_chirho: "hover".to_string(),
            description_chirho: "Get the type of an expression at a source position".to_string(),
            input_schema_chirho: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": { "type": "string", "description": "Path to the .hs file" },
                    "line": { "type": "integer", "description": "1-based line number" },
                    "column": { "type": "integer", "description": "1-based column number" }
                },
                "required": ["file_path", "line", "column"]
            }),
        },
        McpToolChirho {
            name_chirho: "module_info".to_string(),
            description_chirho: "Get exported types and functions from a module".to_string(),
            input_schema_chirho: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": { "type": "string", "description": "Path to the .hs file or .cabal project" }
                },
                "required": ["file_path"]
            }),
        },
        McpToolChirho {
            name_chirho: "build".to_string(),
            description_chirho: "Build a Cabal project and return compilation results".to_string(),
            input_schema_chirho: serde_json::json!({
                "type": "object",
                "properties": {
                    "project_path": { "type": "string", "description": "Path to the project directory" }
                },
                "required": ["project_path"]
            }),
        },
    ]
}

/// Handle an MCP request and produce a response.
pub fn handle_request_chirho(request_chirho: &McpRequestChirho) -> McpResponseChirho {
    let result_chirho = match request_chirho.method_chirho.as_str() {
        "initialize" => Ok(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": {} },
            "serverInfo": {
                "name": "haskelujah-mcp",
                "version": env!("CARGO_PKG_VERSION")
            }
        })),
        "tools/list" => {
            let tools_chirho = list_tools_chirho();
            Ok(serde_json::json!({ "tools": tools_chirho }))
        }
        "tools/call" => {
            let tool_name_chirho = request_chirho
                .params_chirho
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match tool_name_chirho {
                "typecheck" => handle_typecheck_chirho(&request_chirho.params_chirho),
                _ => Err(McpErrorChirho {
                    code_chirho: -32601,
                    message_chirho: format!("unknown tool: {}", tool_name_chirho),
                }),
            }
        }
        _ => Err(McpErrorChirho {
            code_chirho: -32601,
            message_chirho: format!("unknown method: {}", request_chirho.method_chirho),
        }),
    };

    match result_chirho {
        Ok(val_chirho) => McpResponseChirho {
            jsonrpc_chirho: "2.0".to_string(),
            id_chirho: request_chirho.id_chirho.clone(),
            result_chirho: Some(val_chirho),
            error_chirho: None,
        },
        Err(err_chirho) => McpResponseChirho {
            jsonrpc_chirho: "2.0".to_string(),
            id_chirho: request_chirho.id_chirho.clone(),
            result_chirho: None,
            error_chirho: Some(err_chirho),
        },
    }
}

fn handle_typecheck_chirho(
    params_chirho: &serde_json::Value,
) -> Result<serde_json::Value, McpErrorChirho> {
    let file_path_chirho = params_chirho
        .get("arguments")
        .and_then(|a| a.get("file_path"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpErrorChirho {
            code_chirho: -32602,
            message_chirho: "missing file_path argument".to_string(),
        })?;

    let source_chirho =
        std::fs::read_to_string(file_path_chirho).map_err(|e_chirho| McpErrorChirho {
            code_chirho: -32602,
            message_chirho: format!("cannot read {}: {}", file_path_chirho, e_chirho),
        })?;

    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    match haskelujah_driver_chirho::compile_source_chirho(
        &source_chirho,
        &mut sm_chirho,
        file_path_chirho,
    ) {
        Ok(result_chirho) => Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": format!("Typecheck OK: module {}", result_chirho.module_chirho.name_chirho.text_chirho())
            }]
        })),
        Err(diagnostics_chirho) => {
            let msgs_chirho: Vec<String> = diagnostics_chirho
                .diagnostics_chirho()
                .iter()
                .map(|d_chirho| d_chirho.message_chirho.clone())
                .collect();
            Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Errors:\n{}", msgs_chirho.join("\n"))
                }]
            }))
        }
    }
}
