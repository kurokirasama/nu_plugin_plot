# Specification: Weather Plugin Bug Fix (Type Conversion & Crash)

## 1. Overview
This track addresses a bug reported in the `weather` command when using the `nu_plugin_plot` plugin. The error involves a Polars-related type conversion failure ("Error converting list to series with type Int64: Can't convert to int..") followed by a plugin core dump.

## 2. Functional Requirements
### 2.1 Bug Diagnosis & Reproduction
- Pinpoint the exact cause of the "Error converting list to series" message.
- Investigate if the issue is a bug in the plugin's core logic or an incompatibility with how it's being used in the `weather` command (e.g., data types, empty inputs).
- Identify the cause of the core dump (SIGSEGV/Panic) in the plugin binary.

### 2.2 Robust Input Handling
- **Empty List Protection:** Ensure `check_equality_of_list` and `plot_nested` handle empty lists gracefully instead of panicking.
- **Type Compatibility:** Ensure the plugin can handle `Int64` and other numerical types correctly, especially when coming from tables processed by Polars (`polars into-nu`).
- **Flag Parsing:** Update flag parsing to be more resilient (e.g., handle floats passed to `--width` or `--height` if Nushell allows it).

### 2.3 Error Reporting
- Replace potential panics with `LabeledError` to provide helpful feedback to Nushell instead of crashing.

## 3. Non-Functional Requirements
- **Stability:** The plugin must never core dump, regardless of the input data.
- **Performance:** Maintain the lightweight nature of the plugin.

## 4. Acceptance Criteria
- [ ] Minimal reproduction script (using standard data or mock weather data) no longer crashes the plugin.
- [ ] The "Error converting list to series" message is resolved or handled gracefully if it originates from data passed to the plugin.
- [ ] `cargo test` includes a regression test for the identified crash scenario.
- [ ] `weather` command works correctly without falling back to `gnu-plot` (if the plugin is installed).

## 5. Out of Scope
- Rewriting the `weather` command script itself (unless a minor change is needed for compatibility).
- Adding new plotting features.
