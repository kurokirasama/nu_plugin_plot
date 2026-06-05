# Implementation Plan: Bump Nushell and App version to 0.113.1

## Phase 1: Dependency Upgrade & Audit
- [x] Task: Update dependencies and crate version in Cargo.toml [379cff0]
    - [x] Update package version to `0.113.1`
    - [x] Update `nu-plugin` dependency to `0.113.1`
    - [x] Update `nu-protocol` dependency to `0.113.1`
    - [x] Update `nu-plugin-test-support` dev-dependency to `0.113.1`
- [ ] Task: SDK-First Compilation Audit
    - [ ] Run `cargo build` to audit compilation
    - [ ] Fix any compile/API errors arising from Nushell 0.113.1 SDK changes
- [ ] Task: Verify test suite
    - [ ] Run `cargo test` to verify existing tests pass
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Dependency Upgrade & Audit' (Protocol in workflow.md)

## Phase 2: Local Integration Testing & Verification
- [ ] Task: Local registration
    - [ ] Build release version: `cargo build --release`
    - [ ] Add the plugin to Nushell: `plugin add ./target/release/nu_plugin_plot`
    - [ ] Enable the plugin: `plugin use plot`
- [ ] Task: Interactive test verification
    - [ ] Run test plotting command to verify output: `seq 0 0.1 10 | math sin | plot`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Local Integration Testing & Verification' (Protocol in workflow.md)
