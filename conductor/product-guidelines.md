# Product Guidelines: nu_plugin_plot

## 1. Tone and Voice
- **Senior Engineer Tone:** Documentation and command help should be technical, clear, and direct. Avoid conversational filler or unnecessary explanations. Focus on high-signal information and technical rationale.

## 2. Visual Style and Rendering
- **Braille Plots (Default):** Use high-density Unicode characters for rendering plots to maximize information density in the terminal.
- **Color-Aware Design:** Support terminal colors where appropriate to distinguish between different data series or to highlight key information.
- **Density Over Simplicity:** Prioritize capturing data trends accurately over simplistic ASCII-only output.

## 3. Error Handling and Reporting
- **Native Nushell Errors:** Utilize Nushell's `LabeledError` system to provide rich, formatted error reporting. Errors should clearly point to the source of the issue (e.g., malformed input lists or unsupported data types) and provide actionable feedback.

## 4. Naming Conventions
- **Kebab-Case Names:** Adhere to Nushell's standard for command names and flags using kebab-case (e.g., `plot`, `xyplot`, `--line-color`, `--plot-width`).
- **Standard Abbreviations:** Use short, intuitive abbreviations for common flags (e.g., `-w` for `--width`, `-h` for `--height`).
- **Nushell Integration:** Command names and behavior should feel like a native extension of the Nushell environment.
