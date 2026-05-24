# Specification: Nushell 0.113.0 Compatibility Update

## 1. Overview
This track aims to update `nu_plugin_plot` to support Nushell version 0.113.0. This involves updating dependencies, addressing any breaking changes in the Nushell plugin API, and ensuring the plugin functions correctly with the latest release.

## 2. Functional Requirements
### 2.1 Dependency Updates
- Update `nu-plugin` dependency in `Cargo.toml` to `0.113.0`.
- Update `nu-protocol` dependency in `Cargo.toml` to `0.113.0`.

### 2.2 API Alignment
- Audit and update `src/lib.rs` to comply with any changes in the `Plugin` and `SimplePluginCommand` traits introduced in 0.113.0.
- Verify that `Value` and `Signature` usage aligns with the updated `nu-protocol`.

### 2.3 Verification
- Ensure the plugin can be registered with Nushell 0.113.0.
- Verify core commands (`plot`, `hist`, `xyplot`) are fully functional.

## 3. Non-Functional Requirements
### 3.1 Rust Toolchain
- Align with the Rust toolchain requirements for Nushell 0.113.0. 
- Ensure compatibility with the latest stable Rust (at least 1.80.0).

## 4. Acceptance Criteria
- [ ] `cargo build` succeeds without errors or significant new warnings.
- [ ] `cargo test` succeeds.
- [ ] Plugin successfully registers in Nushell 0.113.0 using `plugin add`.
- [ ] `seq 1 10 | plot` renders correctly.
- [ ] `seq 1 10 | hist` renders correctly.

## 5. Out of Scope
- Adding new plotting features or chart types.
- Refactoring the core `color_plot` logic unless required for compatibility.
