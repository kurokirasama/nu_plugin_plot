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
