# Implementation Plan: Bump Nushell and App version to 0.113.1

## Phase 1: Dependency Upgrade & Audit [checkpoint: 7620b78]
- [x] Task: Update dependencies and crate version in Cargo.toml [379cff0]
    - [x] Update package version to `0.113.1`
    - [x] Update `nu-plugin` dependency to `0.113.1`
    - [x] Update `nu-protocol` dependency to `0.113.1`
    - [x] Update `nu-plugin-test-support` dev-dependency to `0.113.1`
- [x] Task: SDK-First Compilation Audit [feca05c]
    - [x] Run `cargo build` to audit compilation
    - [x] Fix any compile/API errors arising from Nushell 0.113.1 SDK changes
- [x] Task: Verify test suite [bc1f1d7]
    - [x] Run `cargo test` to verify existing tests pass
- [x] Task: Conductor - User Manual Verification 'Phase 1: Dependency Upgrade & Audit' (Protocol in workflow.md) [7620b78]

## Phase 2: Local Integration Testing & Verification [checkpoint: 9eb2b05]
- [x] Task: Local registration [2c89b5a]
    - [x] Build release version: `cargo build --release`
    - [x] Add the plugin to Nushell: `plugin add ./target/release/nu_plugin_plot`
    - [x] Enable the plugin: `plugin use plot`
- [x] Task: Interactive test verification [1a2592b]
    - [x] Run test plotting command to verify output: `seq 0 0.1 10 | math sin | plot`
- [x] Task: Conductor - User Manual Verification 'Phase 2: Local Integration Testing & Verification' (Protocol in workflow.md) [9eb2b05]
