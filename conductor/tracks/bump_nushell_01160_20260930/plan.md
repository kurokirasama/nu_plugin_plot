# Implementation Plan: Bump Nushell Dependencies to v0.116.0

## Phase 1: Cargo.toml Version Updates & Clean Build
- [x] Task: Read howto.md before starting this phase
- [x] Task: Update package and dependency versions in Cargo.toml
    - [x] Set `version = "0.116.0"`
    - [x] Update `nu-plugin = "0.116.0"`
    - [x] Update `nu-protocol = { version = "0.116.0", features = ["plugin"] }`
    - [x] Update `nu-plugin-test-support = "0.116.0"`
    - [x] Update `nu-plugin-engine = { version = "0.116.0", features = ["local-socket"] }`
- [x] Task: Execute clean build
    - [x] Run `cargo clean`
    - [x] Run `cargo build --release`
    - [x] Inspect any compiler warnings
- [x] Task: Conductor - User Manual Verification 'Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

## Phase 2: Breaking Fix Implementation
- [x] Task: Audit and resolve source-level API changes
    - [x] Inspect `src/main.rs`, `src/lib.rs`, and commands for trait/signature adaptations
    - [x] Compile and verify zero compilation errors
- [x] Task: Conductor - User Manual Verification 'Breaking Fix Implementation' (Protocol in workflow.md)

## Phase 3: Test Suite Updates & Verification
- [x] Task: Run and adapt test suite
    - [x] Run `cargo test`
    - [x] Update any test assertions or test fixtures in `tests/`
- [x] Task: Conductor - User Manual Verification 'Test Suite Updates & Verification' (Protocol in workflow.md)

## Phase 4: Integration Testing with Nushell
- [x] Task: Verify live plugin registration in Nushell
    - [x] Run `plugin add ./target/release/nu_plugin_plot`
    - [x] Verify `seq 1 10 | plot` produces valid ANSI graphs
- [x] Task: Conductor - User Manual Verification 'Integration Testing with Nushell' (Protocol in workflow.md)

## Phase 5: Documentation Updates & Remote Push
- [x] Task: Update documentation and tech-stack references
    - [x] Update `conductor/tech-stack.md` versions to 0.116.0
    - [x] Update README.md if version numbers are referenced
- [x] Task: Stage, commit, push, and clean
    - [x] Stage and commit changes with `feat: Bump Nushell dependencies to 0.116.0`
    - [x] Push to GitHub remote `origin/main`
    - [x] Run `cargo clean`
- [x] Task: Conductor - User Manual Verification 'Documentation Updates & Remote Push' (Protocol in workflow.md)

