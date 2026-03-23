<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Haskelujah MCP Chirho

`haskelujah mcp` starts a stdio MCP server for AI assistants. It speaks JSON-RPC 2.0 over stdin/stdout and exposes compiler-aware tools so an agent can ask Haskelujah to inspect Haskell code directly.

Current reference state in this repo:
- command: `haskelujah mcp`
- transport: `stdio`
- protocol style: line-delimited JSON-RPC 2.0
- current HEAD documented here: `377185b6`

## What It Does

The MCP server is intended to expose:
- type checking
- hover/type-at-position
- module export information
- project build results

On current `377185b6`, the server advertises four tools in `tools/list`:
- `typecheck`
- `hover`
- `module_info`
- `build`

But only `typecheck` is currently implemented in `tools/call`. The other three names are reserved and will currently return `unknown tool`.

## How To Start It

From the repo:

```bash
cargo run -p haskelujah -- mcp
```

With an installed binary:

```bash
haskelujah mcp
```

When it starts, it writes this startup line to `stderr`:

```text
haskelujah mcp server starting (stdio)...
```

Requests must be written to `stdin`, one JSON object per line. Responses come back on `stdout`, also one JSON object per line.

## Protocol

The server currently handles these JSON-RPC methods:
- `initialize`
- `tools/list`
- `tools/call`

Unknown methods return JSON-RPC error code `-32601`.

### Request Shape

All requests follow standard JSON-RPC 2.0:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
```

### Response Shape

Successful responses:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": { "...": "..." }
}
```

Error responses:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32601,
    "message": "unknown method: something"
  }
}
```

## Initialize

Example request:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {}
}
```

Example response:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "haskelujah-mcp",
      "version": "0.1.1"
    }
  }
}
```

## Listing Tools

Example request:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

Example response shape:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name_chirho": "typecheck",
        "description_chirho": "Type-check a Haskell source file and return diagnostics",
        "input_schema_chirho": {
          "type": "object",
          "properties": {
            "file_path": {
              "type": "string",
              "description": "Path to the .hs file"
            }
          },
          "required": ["file_path"]
        }
      }
    ]
  }
}
```

Important current behavior:
- the server advertises `hover`, `module_info`, and `build`
- but only `typecheck` is implemented in `tools/call`

## Calling Tools

### Important Parameter Detail

The current `tools/call` implementation expects arguments under:

```json
"params": {
  "name": "typecheck",
  "arguments": {
    "file_path": "Main.hs"
  }
}
```

That `arguments` wrapper matters. The schema shown by `tools/list` describes the inner object, but the actual `tools/call` payload still needs `params.arguments`.

## Tool: `typecheck`

Checks a single Haskell source file using `compile_source_chirho`.

### Request

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "typecheck",
    "arguments": {
      "file_path": "/absolute/path/to/Main.hs"
    }
  }
}
```

### Success Response

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Typecheck OK: module Main"
      }
    ]
  }
}
```

### Error Response From Compiler Diagnostics

The tool itself still returns a successful JSON-RPC result, but the result text contains the compiler errors:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Errors:\nno instance for `Num Bool`\n..."
      }
    ]
  }
}
```

### Error Response From Bad Arguments

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32602,
    "message": "missing file_path argument"
  }
}
```

## Tool: `hover`

Advertised input:
- `file_path`
- `line`
- `column`

Intended purpose:
- return the inferred type of the expression at a source position

Current status on `377185b6`:
- advertised in `tools/list`
- not implemented in `tools/call`

Current result if called:

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "error": {
    "code": -32601,
    "message": "unknown tool: hover"
  }
}
```

## Tool: `module_info`

Advertised input:
- `file_path`

Intended purpose:
- return exported values, constructors, and types for a module or project entrypoint

Current status on `377185b6`:
- advertised in `tools/list`
- not implemented in `tools/call`

## Tool: `build`

Advertised input:
- `project_path`

Intended purpose:
- compile a Cabal project and return module/build diagnostics

Current status on `377185b6`:
- advertised in `tools/list`
- not implemented in `tools/call`

## Example Session

### 1. Initialize

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
```

### 2. List tools

```json
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
```

### 3. Typecheck a file

```json
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"typecheck","arguments":{"file_path":"/home/user/project/Main.hs"}}}
```

## Claude Code / Generic MCP Client Configuration

Most MCP clients that support stdio servers want:
- a command
- an argument list
- optional environment variables

Representative config shape:

```json
{
  "mcpServers": {
    "haskelujah-chirho": {
      "command": "/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/target/debug/haskelujah",
      "args": ["mcp"]
    }
  }
}
```

If you installed the binary globally:

```json
{
  "mcpServers": {
    "haskelujah-chirho": {
      "command": "haskelujah",
      "args": ["mcp"]
    }
  }
}
```

Notes:
- exact config file location depends on the client
- the transport is stdio, not HTTP
- the server expects one complete JSON object per line
- startup text goes to `stderr`, responses go to `stdout`

## Practical Recommendations For AI Tools

If you wire this into an assistant today:
- use `initialize`
- use `tools/list`
- use `typecheck`
- do not rely on `hover`, `module_info`, or `build` yet unless you also implement them

For robust clients:
- treat `unknown tool` as a normal capability miss
- keep requests single-line JSON
- pass absolute paths when possible

## Implementation Notes

Relevant source files:
- [main.rs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/crates/haskelujah-cli-chirho/src/main.rs)
- [lib.rs](/home/hallelujah/dev-aleluya/personal-aleluya/haskelujah-chirho/crates/haskelujah-mcp-chirho/src/lib.rs)

Current behavior summary:
- `initialize` is implemented
- `tools/list` is implemented
- `tools/call(typecheck)` is implemented
- `tools/call(hover)` is not implemented yet
- `tools/call(module_info)` is not implemented yet
- `tools/call(build)` is not implemented yet

## Future Improvements

The next obvious MCP upgrades are:
- implement `hover`
- implement `module_info`
- implement `build`
- return structured diagnostics instead of one flat text blob
- add position-aware diagnostics and type spans
- expose project/module graph information directly
