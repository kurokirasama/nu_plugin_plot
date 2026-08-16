# Specification: Bump Nushell Dependencies to v0.115.0

## 1. Overview
This track updates `nu_plugin_plot` dependencies from Nushell v0.114.0 to v0.115.0 (released on 2026-08-15). Nushell plugins require exact protocol version alignment with the host Nushell binary. This track handles Cargo dependency updates, compilation verification against `nu-plugin` / `nu-protocol` 0.115.0, test suite validation, and documentation updates.

## 2. Scope & Dependency Updates
- `Cargo.toml`:
  - `[package].version`: `0.114.0` -> `0.115.0`
  - `[dependencies].nu-plugin`: `"0.114.0"` -> `"0.115.0"`
  - `[dependencies].nu-protocol`: `{ version = "0.114.0", features = ["plugin"] }` -> `{ version = "0.115.0", features = ["plugin"] }`
  - `[dev-dependencies].nu-plugin-test-support`: `"0.114.0"` -> `"0.115.0"`
  - `[dev-dependencies].nu-plugin-engine`: `{ version = "0.114.0", features = ["local-socket"] }` -> `{ version = "0.115.0", features = ["local-socket"] }`

## 3. Compatibility & Breaking Changes Audit
- **Plugin API Changes in 0.115.0**:
  - `PluginCommand::get_dynamic_completion()` now takes `EngineInterface` allowing config access (PR #18587). `nu_plugin_plot` commands (`plot`, `hist`, `xyplot`) implement `SimplePluginCommand` without dynamic completion overrides.
  - Test framework updates in `nu-plugin-test-support` / `nu-plugin-engine`.
- **Adaptation Mapping**:
  - `src/lib.rs`: Verify `Plugin` and `SimplePluginCommand` trait implementations compile under 0.115.0.
  - `src/main.rs`: Verify `serve_plugin` entrypoint compatibility.
  - `src/color_plot/`: Vendored `drawille` and `textplots` modules.
  - `tests/`: Verify test suite compiles with updated test dependencies.

## 4. Acceptance Criteria & BDD Scenarios
### Scenario 1: Clean Compilation with Nushell 0.115.0
- **Given** `Cargo.toml` dependencies are updated to `0.115.0`
- **When** `cargo build --release` is executed
- **Then** the plugin compiles successfully without breaking errors

### Scenario 2: Test Suite Verification
- **Given** the updated plugin binary and test suite
- **When** `cargo test` is executed
- **Then** all unit and functional tests pass

### Scenario 3: Nushell Integration & Visual Plotting
- **Given** a Nushell 0.115.0 environment with `nu_plugin_plot` registered
- **When** executing `[1 2 3 4 5] | plot`, `[1 2 2 3 3 3 4] | hist`, and `[ [0 1 2] [0 1 4] ] | xyplot`
- **Then** terminal Braille/ASCII plots render correctly without errors

## 5. Out of Scope
- Rewriting vendored `textplots` / `drawille` plotting engine logic unless breaking changes require it.
- Adding new plotting commands or non-version-bump features.
