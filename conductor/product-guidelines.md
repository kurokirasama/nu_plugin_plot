# Product Guidelines: nu_plugin_plot

## 1. Tone and Voice
- **Senior Engineer Tone:** Documentation and command help should be technical, clear, and direct. Avoid conversational filler or unnecessary explanations. Focus on high-signal information and technical rationale.
- **Clarity and Precision:** Every sentence in the documentation should serve a specific purpose.
- **Discoverability:** Use consistent naming and thorough documentation to ensure commands are easy to find and use.

## 2. Visual Style and Rendering
- **Braille Plots (Default):** Use high-density Unicode characters for rendering plots to maximize information density in the terminal.
- **Color-Aware Design:** Support terminal colors where appropriate to distinguish between different data series or to highlight key information.
- **Density Over Simplicity:** Prioritize capturing data trends accurately over simplistic ASCII-only output.
- **Aesthetic Consistency:** Use 2-space indentation, 80-100 characters max line length, and spacing around pipes and inside braces.

## 3. Error Handling and Reporting
- **Native Nushell Errors:** Utilize Nushell's `LabeledError` system to provide rich, formatted error reporting. Errors should clearly point to the source of the issue (e.g., malformed input lists or unsupported data types) and provide actionable feedback.
- **Fail Fast:** The plugin should fail immediately upon encountering an invalid state or malformed input.
- **Meaningful Feedback:** Provide error messages that suggest a potential fix or clarify the expected input format.

## 4. Naming Conventions
- **Kebab-Case Names:** Adhere to Nushell's standard for command names and flags using kebab-case (e.g., `plot`, `xyplot`, `--line-color`, `--plot-width`).
- **Standard Abbreviations:** Use short, intuitive abbreviations for common flags (e.g., `-w` for `--width`, `-h` for `--height`).
- **Variable and Parameter Naming:** Use `snake_case` for internal variables and parameters, and `SCREAMING_SNAKE_CASE` for environment variables.
- **Nushell Integration:** Command names and behavior should feel like a native extension of the Nushell environment.

## 5. Architectural Principles
- **Data > Text:** Prioritize working with structured data (lists, records, tables) over raw string manipulation.
- **Modularity:** Organize core plotting logic into clean, modular components (e.g., `color_plot` module) for easier maintenance.
- **Composability:** Commands should return data in a way that is easily piped to other Nushell commands.
