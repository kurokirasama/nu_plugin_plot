# Specification: Update Nushell Version to 0.112.2

## Overview
This track updates the `nu_plugin_plot` dependencies (`nu-plugin`, `nu-protocol`) to version `0.112.2` to ensure compatibility with the latest Nushell release.

## Functional Requirements
- **Dependency Update:** Update `nu-plugin` and `nu-protocol` versions in `Cargo.toml` to `0.112.2`.
- **Plugin Signatures:** Adopt any new plugin signature features introduced in `0.112.2`.
- **Error Handling:** Update the error handling to use the new `LabeledError` format if necessary.
- **Data Types:** Ensure compatibility with new data types introduced or modified in `0.112.2`.
- **Compilation:** Ensure the project compiles successfully with the new dependencies using `cargo build`.
- **Functionality:** Ensure `plot`, `hist`, and `xyplot` commands continue to function correctly.

## Non-Functional Requirements
- Only Nushell-related dependencies will be updated. Other dependencies (e.g., `owo-colors`, `term_size`) should remain unchanged unless required for compilation.
- No changes to the core plotting logic or external dependencies are planned.

## Acceptance Criteria
- [ ] `Cargo.toml` reflects `nu-plugin` and `nu-protocol` version `0.112.2`.
- [ ] The project builds successfully with `cargo build --release`.
- [ ] The plugin loads successfully into Nushell 0.112.2 (`plugin add`, `plugin use`).
- [ ] Basic manual testing in Nushell confirms plots render correctly without panicking or type errors.

## Out of Scope
- Updating non-Nushell dependencies.
- Refactoring the core `color_plot` engine.
- Adding new plotting features (e.g., new plot types).
