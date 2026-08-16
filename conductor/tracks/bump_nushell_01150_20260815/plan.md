# Implementation Plan: Bump Nushell Dependencies to v0.115.0

This plan guides the upgrade of `nu_plugin_plot` to Nushell v0.115.0.

> **CRITICAL INSTRUCTION:** The implementer MUST read [`howto.md`](./howto.md) before starting implementation.

## Phase 1: Cargo.toml Version Updates & Clean Build [checkpoint: 6654559]
- [x] Task: Update package and dependency versions in Cargo.toml (31eb433)
    - [x] Update `[package].version` to `0.115.0` in `Cargo.toml`
    - [x] Update `nu-plugin` dependency to `0.115.0`
    - [x] Update `nu-protocol` dependency to `0.115.0` with feature `["plugin"]`
    - [x] Update dev-dependencies `nu-plugin-test-support` and `nu-plugin-engine` to `0.115.0`
- [x] Task: Execute clean build and check compiler diagnostics (d3191a9)
    - [x] Run `cargo clean` to ensure no stale artifacts
    - [x] Run `cargo check --release` and capture compiler errors or warnings
- [x] Task: Conductor - User Manual Verification 'Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

## Phase 2: Breaking Fix Implementation & Source Code Adaptation [checkpoint: 95af5dd]
- [x] Task: Audit and adapt plugin source code to Nushell 0.115.0 API
    - [x] Verify `Plugin` trait implementation in `src/lib.rs` for compatibility
    - [x] Verify `SimplePluginCommand` implementations (`CommandPlot`, `CommandHist`, `CommandXyplot`) in `src/lib.rs`
    - [x] Verify `serve_plugin` call in `src/main.rs`
    - [x] Verify internal color plotting modules in `src/color_plot/`
- [x] Task: Build release binary
    - [x] Run `cargo build --release` and confirm zero errors
- [x] Task: Conductor - User Manual Verification 'Breaking Fix Implementation & Source Code Adaptation' (Protocol in workflow.md)

## Phase 3: Test Suite Updates & Verification (TDD/Red-Green) [checkpoint: 2cd3e56]
- [x] Task: Update and execute plugin functional tests
    - [x] Check `tests/` suite for compatibility with `nu-plugin-test-support` 0.115.0
    - [x] Update any test harnesses or assertions as required by the 0.115.0 engine interface
    - [x] Run `cargo test` and verify 100% pass rate
- [x] Task: Conductor - User Manual Verification 'Test Suite Updates & Verification' (Protocol in workflow.md)

## Phase 4: Integration Testing with Nushell 0.115.0 [checkpoint: 0daad8b]
- [x] Task: Register plugin and verify in Nushell runtime
    - [x] Register plugin via `plugin add ./target/release/nu_plugin_plot` and `plugin use plot`
    - [x] Verify line plotting: `(seq 0 0.1 10 | math sin) | plot`
    - [x] Verify histogram plotting: `(seq 1 100 | math random) | hist`
    - [x] Verify XY plotting: `[ (seq 0 0.1 10) ((seq 0 0.1 10) | math sin) ] | xyplot`
    - [x] Verify edge cases: empty list `[] | plot`, single item `[5] | plot`, negative numbers `[-5 0 5] | plot`
- [x] Task: Conductor - User Manual Verification 'Integration Testing with Nushell 0.115.0' (Protocol in workflow.md)

## Phase 5: Documentation & Styleguide Updates [checkpoint: a8eb0c1]
- [x] Task: Update project documentation and Conductor tech stack
    - [x] Update `conductor/tech-stack.md` with Nushell 0.115.0 versions and minimum Rust version
    - [x] Update `README.md` / `GEMINI.md` / `AGENTS.md` version references if applicable
- [x] Task: Conductor - User Manual Verification 'Documentation & Styleguide Updates' (Protocol in workflow.md)

## Phase 6: Remote Push & Repository Cleanup
- [ ] Task: Commit changes, push to remote repository, and clean build artifacts
    - [ ] Stage all modified files (`Cargo.toml`, `src/`, `tests/`, `conductor/`) and create a git commit: `feat: Bump Nushell dependencies to 0.115.0`
    - [ ] Push commit to remote origin (`git push origin <branch>`)
    - [ ] Execute `cargo clean` to reclaim disk space
- [ ] Task: Conductor - User Manual Verification 'Remote Push & Repository Cleanup' (Protocol in workflow.md)
