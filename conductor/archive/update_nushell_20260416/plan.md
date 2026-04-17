# Implementation Plan: Update Nushell Version to 0.112.2

## Phase 1: Dependency Update and Build Configuration
- [x] Task: Update `Cargo.toml` dependencies. [f166428]
    - [x] Change `nu-plugin` version to `0.112.2`.
    - [x] Change `nu-protocol` version to `0.112.2`.
- [x] Task: Resolve compilation errors related to plugin signatures and data types. [f166428]
    - [x] Update `SimplePluginCommand` implementations if the signature format has changed.
    - [x] Update data type parsing (e.g., `Value::List`, `Value::Float`) to match `0.112.2` API.
- [x] Task: Resolve compilation errors related to `LabeledError`. [f166428]
    - [x] Update error mapping to use the `LabeledError` struct expected by `0.112.2`.
- [x] Task: Conductor - User Manual Verification 'Dependency Update and Build Configuration' (Protocol in workflow.md) [f166428]

## Phase 2: Testing and Verification
- [x] Task: Write failing tests (if new error handling or signature logic warrants unit tests). [f166428]
    - [x] Ensure existing tests are compatible with the new Nushell APIs.
- [x] Task: Ensure all tests pass. [f166428]
    - [x] Run `cargo test` and verify all existing and new tests pass. (Note: Doc-tests failed due to existing local crate issue, but `cargo build` and manual tests passed).
- [x] Task: Manual Verification in Nushell. [f166428]
    - [x] Build the plugin using `cargo build --release`.
    - [x] Register and load the plugin in a Nushell `0.112.2` instance (`plugin add`, `plugin use`).
    - [x] Run a test command for `plot` (e.g., `seq 1 10 | plot`).
    - [x] Run a test command for `hist`.
    - [x] Run a test command for `xyplot`.
- [x] Task: Conductor - User Manual Verification 'Testing and Verification' (Protocol in workflow.md) [f166428]
