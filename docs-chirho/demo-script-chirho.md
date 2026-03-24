<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Haskelujah Demo Script

## Recording Setup

Use [asciinema](https://asciinema.org/) for terminal recording, then convert to MP4/GIF:

```bash
# Install asciinema
pip install asciinema

# Record
asciinema rec haskelujah-demo.cast

# Convert to GIF (using agg)
agg haskelujah-demo.cast haskelujah-demo.gif
```

## Demo Script (copy-paste into terminal)

### Part 1: Install and Create (30 seconds)

```bash
# Show version
haskelujah --version

# Create a new project
haskelujah init hello-app
cd hello-app

# Show what was generated
cat Main.hs
```

**Narration:** "Haskelujah generates a project with batteries-included imports — JSON, Text, and Debug are built in, no install needed."

### Part 2: Type-Check and Test (30 seconds)

```bash
# Type-check
haskelujah check Main.hs

# Run tests
haskelujah test .
```

**Narration:** "Type-checking and testing work out of the box. The test file uses our built-in test framework."

### Part 3: Edit with Built-in Editor (45 seconds)

```bash
# Open editor
haskelujah edit Main.hs
```

In the editor:
1. Show syntax highlighting
2. Press **Ctrl+T** to typecheck — show "Typecheck OK" in status bar
3. Press **Ctrl+E**, type `2 + 2`, press Enter — show "4" in status bar
4. Press **Ctrl+Q** to quit

**Narration:** "The built-in editor has syntax highlighting, inline typechecking, and a Haskell eval prompt. It evaluates expressions using the same compiler."

### Part 4: Format (15 seconds)

```bash
# Add some messy formatting
echo -e "module  Main where\n\n\n\nmain =   putStrLn \"hello\"  \t" > Messy.hs
cat Messy.hs

# Format it
haskelujah fmt Messy.hs
cat Messy.hs
```

**Narration:** "haskelujah fmt cleans up whitespace, normalizes indentation, and removes excess blank lines."

### Part 5: Script Runner (20 seconds)

```bash
cat > script.hs << 'EOF'
#!/usr/bin/env haskelujah
import Haskelujah.JSON
main = putStrLn (encode (object ["status" .= String "running"]))
EOF
chmod +x script.hs
./script.hs
```

**Narration:** "Run Haskell files directly as scripts — no compilation step needed."

### Part 6: AI Integration (20 seconds)

```bash
# Start MCP server
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | haskelujah mcp 2>/dev/null | python3 -m json.tool
```

**Narration:** "The built-in MCP server lets AI assistants like Claude typecheck and analyze your Haskell code."

### Part 7: Hackage Packages (30 seconds)

```bash
# Show what compiles
haskelujah install transformers
haskelujah install mtl
haskelujah install parsec
```

**Narration:** "17 Hackage packages compile including the transformers, mtl, and parsec cascade. The package manager resolves dependencies automatically."

## Total Demo Time: ~3 minutes

## Key Messages

1. **One binary, everything included** — compiler, editor, test runner, formatter, REPL, AI server
2. **Batteries included** — 15 standard library modules, no install needed
3. **Haskell-extensible** — the editor evaluates the language it compiles
4. **Cross-platform** — terminal editor everywhere, GUI on macOS/Linux/Windows
5. **GHC-compatible** — 88.8% test suite compatibility, 40+ extensions
