# Implementation Plan: Bump Nushell Dependencies to v0.115.0

This plan guides the upgrade of `nu_plugin_plot` to Nushell v0.115.0.

> **CRITICAL INSTRUCTION:** The implementer MUST read [`howto.md`](./howto.md) before starting implementation.

## Phase 1: Cargo.toml Version Updates & Clean Build
- [ ] Task: Update package and dependency versions in Cargo.toml
    - [ ] Update `[package].version` to `0.115.0` in `Cargo.toml`
    - [ ] Update `nu-plugin` dependency to `0.115.0`
    - [ ] Update `nu-protocol` dependency to `0.115.0` with feature `["plugin"]`
    - [ ] Update dev-dependencies `nu-plugin-test-support` and `nu-plugin-engine` to `0.115.0`
- [ ] Task: Execute clean build and check compiler diagnostics
    - [ ] Run `cargo clean` to ensure no stale artifacts
    - [ ] Run `cargo check --release` and capture compiler errors or warnings
- [ ] Task: Conductor - User Manual Verification 'Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

## Phase 2: Breaking Fix Implementation & Source Code Adaptation
- [ ] Task: Audit and adapt plugin source code to Nushell 0.115.0 API
    - [ ] Verify `Plugin` trait implementation in `src/lib.rs` for compatibility
    - [ ] Verify `SimplePluginCommand` implementations (`CommandPlot`, `CommandHist`, `CommandXyplot`) in `src/lib.rs`
    - [ ] Verify `serve_plugin` call in `src/main.rs`
    - [ ] Verify internal color plotting modules in `src/color_plot/`
- [ ] Task: Build release binary
    - [ ] Run `cargo build --release` and confirm zero errors
- [ ] Task: Conductor - User Manual Verification 'Breaking Fix Implementation & Source Code Adaptation' (Protocol in workflow.md)

## Phase 3: Test Suite Updates & Verification (TDD/Red-Green)
- [ ] Task: Update and execute plugin functional tests
    - [ ] Check `tests/` suite for compatibility with `nu-plugin-test-support` 0.115.0
    - [ ] Update any test harnesses or assertions as required by the 0.115.0 engine interface
    - [ ] Run `cargo test` and verify 100% pass rate
- [ ] Task: Conductor - User Manual Verification 'Test Suite Updates & Verification' (Protocol in workflow.md)

## Phase 4: Integration Testing with Nushell 0.115.0
- [ ] Task: Register plugin and verify in Nushell runtime
    - [ ] Register plugin via `plugin add ./target/release/nu_plugin_plot` and `plugin use plot`
    - [ ] Verify line plotting: `(seq 0 0.1 10 | math sin) | plot`
    - [ ] Verify histogram plotting: `(seq 1 100 | math random) | hist`
    - [ ] Verify XY plotting: `[ (seq 0 0.1 10) ((seq 0 0.1 10) | math sin) ] | xyplot`
    - [ ] Verify edge cases: empty list `[] | plot`, single item `[5] | plot`, negative numbers `[-5 0 5] | plot`
- [ ] Task: Conductor - User Manual Verification 'Integration Testing with Nushell 0.115.0' (Protocol in workflow.md)

## Phase 5: Documentation & Styleguide Updates
- [ ] Task: Update project documentation and Conductor tech stack
    - [ ] Update `conductor/tech-stack.md` with Nushell 0.115.0 versions and minimum Rust version
    - [ ] Update `README.md` / `GEMINI.md` / `AGENTS.md` version references if applicable
- [ ] Task: Conductor - User Manual Verification 'Documentation & Styleguide Updates' (Protocol in workflow.md)

## Phase 6: Remote Push & Repository Cleanup
- [ ] Task: Commit changes, push to remote repository, and clean build artifacts
    - [ ] Stage all modified files (`Cargo.toml`, `src/`, `tests/`, `conductor/`) and create a git commit: `feat: Bump Nushell dependencies to 0.115.0`
    - [ ] Push commit to remote origin (`git push origin <branch>`)
    - [ ] Execute `cargo clean` to reclaim disk space
- [ ] Task: Conductor - User Manual Verification 'Remote Push & Repository Cleanup' (Protocol in workflow.md)
