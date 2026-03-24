<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Getting Started with Haskelujah

## Install

```bash
# From crates.io
cargo install haskelujah

# From source
git clone https://github.com/loveJesus/haskelujah-chirho
cd haskelujah-chirho
cargo install --path crates/haskelujah-cli-chirho
```

## Create Your First Project

```bash
haskelujah init my-app
cd my-app
```

This creates:
- `my-app.cabal` — project metadata
- `Main.hs` — entry point with batteries-included imports
- `tests/Test.hs` — test file using Haskelujah.Test

## Build and Run

```bash
# Type-check your code
haskelujah check Main.hs

# Build the project
haskelujah build .

# Run tests
haskelujah test .
```

## Edit Your Code

```bash
# Terminal editor (works everywhere)
haskelujah edit Main.hs

# GUI editor (macOS/Linux/Windows)
haskelujah edit --gui Main.hs
```

The editor has:
- Haskell syntax highlighting (keywords gold, types cyan, comments grey)
- **Ctrl+T** — typecheck the current file
- **Ctrl+E** — evaluate a Haskell expression
- **Ctrl+G** — go to line
- **Ctrl+S** — save

## Use the Standard Library

No `haskelujah install` needed — these modules are built in:

```haskell
import Haskelujah.JSON      -- encode/decode JSON
import Haskelujah.Test      -- assertEqual, runTests
import Haskelujah.Text      -- toLower, splitOn, contains
import Haskelujah.Map       -- insert, lookup, fromList
import Haskelujah.Set       -- union, intersection, member
import Haskelujah.Pretty    -- nest, hsep, render
import Haskelujah.Random    -- nextInt, shuffle, choice
import Haskelujah.Args      -- getFlag, getOption
import Haskelujah.HTTP      -- get, post
import Haskelujah.Debug     -- trace, traceIO, assert, todo
import Haskelujah.File      -- readFileText, writeFileText
import Haskelujah.Process   -- shell, exec
import Haskelujah.Time      -- now, sleep, measure
import Haskelujah.Concurrent -- fork, Chan, MVar
import Haskelujah.Prelude   -- trim, chunksOf, nub'
```

## Use as a Script Runner

```bash
cat > hello.hs << 'EOF'
#!/usr/bin/env haskelujah
import Haskelujah.JSON
main = putStrLn (encode (object ["msg" .= String "hello"]))
EOF
chmod +x hello.hs
./hello.hs
```

## Format Your Code

```bash
haskelujah fmt .
```

Trims trailing whitespace, normalizes indentation, removes excess blank lines.

## AI Integration

Haskelujah includes a built-in MCP server for AI assistants:

```bash
haskelujah mcp
```

Add to your Claude Code settings:
```json
{
  "mcpServers": {
    "haskelujah": {
      "command": "haskelujah",
      "args": ["mcp"]
    }
  }
}
```

## LSP for Editors

```bash
haskelujah lsp
```

Add to VS Code `settings.json`:
```json
{
  "haskelujah.serverPath": "haskelujah",
  "haskelujah.serverArgs": ["lsp"]
}
```

## Install Hackage Packages

```bash
haskelujah install aeson
haskelujah install lens
```

17 packages compile out of the box including transformers, mtl, and parsec.

## All Commands

| Command | What it does |
|---|---|
| `haskelujah build <dir>` | Compile a project |
| `haskelujah check <file>` | Type-check a file |
| `haskelujah edit <file>` | Open the editor (`--gui` / `--cli`) |
| `haskelujah test <dir>` | Run test suite |
| `haskelujah fmt <dir>` | Format source files |
| `haskelujah script <file>` | Run a Haskell script |
| `haskelujah repl` | Interactive REPL |
| `haskelujah mcp` | Start MCP server for AI |
| `haskelujah lsp` | Start LSP server for editors |
| `haskelujah install <pkg>` | Install from Hackage |
| `haskelujah init <name>` | Create a new project |
| `haskelujah clean <dir>` | Remove build artifacts |
