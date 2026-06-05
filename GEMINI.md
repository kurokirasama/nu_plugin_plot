# nu_plugin_plot

## Project Overview

`nu_plugin_plot` is a plugin for [Nushell](https://www.nushell.sh/) that enables rendering ASCII/Unicode plots directly in the terminal from numerical data lists. It provides commands to create line plots, histograms, and XY scatter plots.

The project is written in Rust and integrates with Nushell's plugin system. It includes custom implementations of plotting logic (based on `textplots` and `drawille`) to ensure compatibility and color support within the Nushell environment.

### Key Technologies

*   **Language:** Rust (Edition 2021)
*   **Framework:** `nu-plugin`, `nu-protocol` (Nushell Plugin API)
*   **Dependencies:**
    *   `owo-colors`: For terminal text coloring.
    *   `term_size`: For detecting terminal dimensions.
    *   `fnv`: Fast hashing (likely used internally by dependencies).

## Building and Installation

To build and use this plugin, you must have Rust installed and be running inside a Nushell instance.

### Build Command

```bash
cargo build --release
```

### Installation (Nushell)

After building, you need to register the plugin with Nushell:

```nushell
plugin add ./target/release/nu_plugin_plot
plugin use plot # or restart nu
```

### Usage Examples

Once installed, the following commands are available:

*   `plot`: Render a line plot from a list of values.
*   `hist`: Render a histogram from a list of values.
*   `xyplot`: Render an XY plot from a nested list of two lists (x and y coordinates).

**Example (Nushell):**

```nushell
let data = (seq 0 0.1 10 | math sin)
$data | plot
```

## Development Conventions

### Architecture

*   **Entry Point:** `src/main.rs` serves as the binary entry point, calling `serve_plugin`.
*   **Plugin Logic:** `src/lib.rs` contains the core plugin implementation.
    *   `PluginPlot`: The main struct implementing the `Plugin` trait.
    *   `CommandPlot`, `CommandHist`, `CommandXyplot`: Structs implementing `SimplePluginCommand` for each specific command.
*   **Plotting Engine:** `src/color_plot/` contains the plotting logic.
    *   `drawille.rs`: Handles Braille character generation for high-resolution terminal graphics.
    *   `textplots/`: Handles scaling, axes, and chart construction. *Note: These are local modified copies of existing crates to fix color rendering issues in the plugin environment.*

### Code Style

*   Follows standard Rust formatting (`rustfmt`).
*   Uses `LabeledError` for rich error reporting back to Nushell.

### Dependencies

*   The `Cargo.toml` defines dependencies. Note that `nu-plugin` and `nu-protocol` versions must match the version of Nushell being used.

## Key Files

*   `src/lib.rs`: Main library file containing command definitions and argument parsing.
*   `src/main.rs`: Binary entry point.
*   `src/color_plot/`: Custom plotting implementation directory.
*   `Cargo.toml`: Project configuration and dependencies.


# Nushell Activation
**ALWAYS** activate the `nushell-expert` skill before executing the first nushell command.

# context-mode — MANDATORY routing rules

You have context-mode MCP tools available. These rules are NOT optional — they protect your context window from flooding. A single unrouted command can dump 56 KB into context and waste the entire session.

## BLOCKED commands — do NOT attempt these

### curl / wget — BLOCKED
Any shell command containing `curl` or `wget` will be intercepted and blocked. Do NOT retry.
Instead use:
- `mcp__context-mode__ctx_fetch_and_index(url, source)` to fetch and index web pages
- `mcp__context-mode__ctx_execute(language: "javascript", code: "const r = await fetch(...)")` to run HTTP calls in sandbox

### Inline HTTP — BLOCKED
Any shell command containing `fetch('http`, `requests.get(`, `requests.post(`, `http.get(`, or `http.request(` will be intercepted and blocked. Do NOT retry with shell.
Instead use:
- `mcp__context-mode__ctx_execute(language, code)` to run HTTP calls in sandbox — only stdout enters context

### WebFetch / web browsing — BLOCKED
Direct web fetching is blocked. Use the sandbox equivalent.
Instead use:
- `mcp__context-mode__ctx_fetch_and_index(url, source)` then `mcp__context-mode__ctx_search(queries)` to query the indexed content

## REDIRECTED tools — use sandbox equivalents

### Shell (>20 lines output)
Shell is ONLY for: `git`, `mkdir`, `rm`, `mv`, `cd`, `ls`, `npm install`, `pip install`, and other short-output commands.
For everything else, use:
- `mcp__context-mode__ctx_batch_execute(commands, queries)` — run multiple commands + search in ONE call
- `mcp__context-mode__ctx_execute(language: "shell", code: "...")` — run in sandbox, only stdout enters context

### read_file (for analysis)
If you are reading a file to **edit** it → read_file is correct (edit needs content in context).
If you are reading to **analyze, explore, or summarize** → use `mcp__context-mode__ctx_execute_file(path, language, code)` instead. Only your printed summary enters context.

### grep / search (large results)
Search results can flood context. Use `mcp__context-mode__ctx_execute(language: "shell", code: "grep ...")` to run searches in sandbox. Only your printed summary enters context.

## Tool selection hierarchy

1. **GATHER**: `mcp__context-mode__ctx_batch_execute(commands, queries)` — Primary tool. Runs all commands, auto-indexes output, returns search results. ONE call replaces 30+ individual calls.
2. **FOLLOW-UP**: `mcp__context-mode__ctx_search(queries: ["q1", "q2", ...])` — Query indexed content. Pass ALL questions as array in ONE call.
3. **PROCESSING**: `mcp__context-mode__ctx_execute(language, code)` | `mcp__context-mode__ctx_execute_file(path, language, code)` — Sandbox execution. Only stdout enters context.
4. **WEB**: `mcp__context-mode__ctx_fetch_and_index(url, source)` then `mcp__context-mode__ctx_search(queries)` — Fetch, chunk, index, query. Raw HTML never enters context.
5. **INDEX**: `mcp__context-mode__ctx_index(content, source)` — Store content in FTS5 knowledge base for later search.

## Output constraints

- Keep responses under 500 words.
- Write artifacts (code, configs, PRDs) to FILES — never return them as inline text. Return only: file path + 1-line description.
- When indexing content, use descriptive source labels so others can `search(source: "label")` later.

## ctx commands

| Command | Action |
|---------|--------|
| `ctx stats` | Call the `stats` MCP tool and display the full output verbatim |
| `ctx doctor` | Call the `doctor` MCP tool, run the returned shell command, display as checklist |