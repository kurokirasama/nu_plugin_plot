# Implementation Plan: Bump Nushell Dependencies to v0.117.0

> **Notice:** Read `howto.md` in this track folder before executing implementation tasks.

---

## Phase 1: Cargo.toml Version Updates & Clean Build
- [x] Task: Verify baseline build and test pass on existing versions
    - [x] Run `cargo test` on current dependency tree to ensure clean baseline
- [x] Task: Update dependencies in `Cargo.toml`
    - [x] Update `version = "0.117.0"`
    - [x] Update `nu-plugin = "0.117.0"`
    - [x] Update `nu-protocol = { version = "0.117.0", features = ["plugin"] }`
    - [x] Update `nu-plugin-test-support = "0.117.0"`
    - [x] Update `nu-plugin-engine = { version = "0.117.0", features = ["local-socket"] }`
- [x] Task: Clean build and resolve `Cargo.lock`
    - [x] Run `cargo clean && cargo build --release`
    - [x] Check compiler warnings via `cargo build 2>&1 | grep -i warning`
- [x] Task: Conductor - User Manual Verification 'Phase 1: Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

---

## Phase 2: Breaking Fix Implementation
- [x] Task: Adapt source code to any v0.117.0 API changes
    - [x] Audit `src/lib.rs`, `src/main.rs`, and `src/color_plot/`
    - [x] Verify trait implementations for `Plugin` and `SimplePluginCommand`
    - [x] Verify error handling and value creation
- [x] Task: Conductor - User Manual Verification 'Phase 2: Breaking Fix Implementation' (Protocol in workflow.md)

---

## Phase 3: Test Suite Updates & Verification
- [x] Task: Update test suite for v0.117.0
    - [x] Review `tests/functional.rs` and update any signatures if needed
- [x] Task: Run full test suite
    - [x] Run `cargo test` and verify 100% pass rate
- [x] Task: Conductor - User Manual Verification 'Phase 3: Test Suite Updates & Verification' (Protocol in workflow.md)

---

## Phase 4: Integration Testing with Nushell Binary
- [x] Task: Register plugin with Nushell 0.117.0
    - [x] Run `plugin add ./target/release/nu_plugin_plot`
    - [x] Run `plugin use plot`
- [x] Task: Test plugin commands in Nushell
    - [x] Test line plots: `seq 0 0.1 10 | math sin | plot`
    - [x] Test histograms: `seq 1 100 | math random | hist`
    - [x] Test xyplot: `let x = seq 0 0.1 10; let y = $x | math sin; [$x $y] | xyplot`
    - [x] Verify `plugin list | where name == "nu_plugin_plot"` shows protocol alignment
- [x] Task: Conductor - User Manual Verification 'Phase 4: Integration Testing with Nushell Binary' (Protocol in workflow.md)

---

## Phase 5: Documentation, Remote Push & Cleanup
- [x] Task: Update documentation
    - [x] Update `conductor/tech-stack.md` version references to `0.117.0`
    - [x] Update `README.md` if necessary
- [x] Task: Commit changes locally
    - [x] Stage `Cargo.toml`, `Cargo.lock`, `conductor/tech-stack.md`, `conductor/tracks.md`
    - [x] Commit: `feat: Bump Nushell dependencies to 0.117.0`
- [x] Task: Pull/rebase and push to GitHub remote
    - [x] Run `git pull --rebase origin main`
    - [x] Run `git push origin main`
- [x] Task: Final cleanup
    - [x] Run `cargo clean` to reclaim disk space
- [x] Task: Conductor - User Manual Verification 'Phase 5: Documentation, Remote Push & Cleanup' (Protocol in workflow.md)
