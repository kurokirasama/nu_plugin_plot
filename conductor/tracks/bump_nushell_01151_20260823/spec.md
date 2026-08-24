# Specification: Bump Nushell Dependencies to v0.115.1

## 1. Overview
This track updates `nu_plugin_plot` dependencies from Nushell v0.115.0 to v0.115.1 (released on 2026-08-23). Nushell plugins require exact protocol version alignment with the host Nushell binary. This track handles Cargo dependency updates, compilation verification against `nu-plugin` / `nu-protocol` 0.115.1, test suite validation, and documentation updates.

**Note:** A previous track `bump_nushell_01150_20260815` exists in the archive for the 0.115.0 migration. This is a new patch-level track for 0.115.1.

## 2. Scope & Dependency Updates
- `Cargo.toml`:
  - `[package].version`: `0.115.0` -> `0.115.1`
  - `[dependencies].nu-plugin`: `"0.115.0"` -> `"0.115.1"`
  - `[dependencies].nu-protocol`: `{ version = "0.115.0", features = ["plugin"] }` -> `{ version = "0.115.1", features = ["plugin"] }`
  - `[dev-dependencies].nu-plugin-test-support`: `"0.115.0"` -> `"0.115.1"`
  - `[dev-dependencies].nu-plugin-engine`: `{ version = "0.115.0", features = ["local-socket"] }` -> `{ version = "0.115.1", features = ["local-socket"] }`

## 3. Compatibility & Breaking Changes Audit

### 3.1 Plugin API Changes in 0.115.1
Nushell 0.115.1 is a patch release with **no breaking changes to the plugin API**. The changes are:
- Bug fixes (semver comparisons, `$ans`, SQLite `update`, keybinding merging)
- Completion improvements (Helix keybindings, background-completions opt-out)
- No trait signature changes, struct field changes, or macro changes affecting plugins

### 3.2 Adaptation Mapping
- `src/lib.rs`: Verify `Plugin` and `SimplePluginCommand` trait implementations compile under 0.115.1
- `src/main.rs`: Verify `serve_plugin` entrypoint compatibility
- `src/color_plot/`: Vendored `drawille` and `textplots` modules (unchanged)
- `tests/`: Verify test suite compiles with updated test dependencies

### 3.3 Version Delta Classification
**Patch-level only** — no breaking changes, no deprecations. Dependency bump required for protocol alignment.

## 4. Acceptance Criteria & BDD Scenarios

### Scenario 1: Clean Compilation with Nushell 0.115.1
- **Given** `Cargo.toml` dependencies are updated to `0.115.1`
- **When** `cargo build --release` is executed
- **Then** the plugin compiles successfully without breaking errors

### Scenario 2: Test Suite Verification
- **Given** the updated plugin binary and test suite
- **When** `cargo test` is executed
- **Then** all unit and functional tests pass

### Scenario 3: Nushell Integration & Visual Plotting
- **Given** a Nushell 0.115.1 environment with `nu_plugin_plot` registered
- **When** executing `[1 2 3 4 5] | plot`, `[1 2 2 3 3 3 4] | hist`, and `[ [0 1 2] [0 1 4] ] | xyplot`
- **Then** terminal Braille/ASCII plots render correctly without errors

## 5. Out of Scope
- Rewriting vendored `textplots` / `drawille` plotting engine logic
- Adding new plotting commands or non-version-bump features
- Major refactoring beyond version alignment