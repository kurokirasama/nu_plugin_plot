# Implementation Plan: Bump Nushell Dependencies to v0.117.0

> **Notice:** Read `howto.md` in this track folder before executing implementation tasks.

---

## Phase 1: Cargo.toml Version Updates & Clean Build
- [ ] Task: Verify baseline build and test pass on existing versions
    - [ ] Run `cargo test` on current dependency tree to ensure clean baseline
- [ ] Task: Update dependencies in `Cargo.toml`
    - [ ] Update `version = "0.117.0"`
    - [ ] Update `nu-plugin = "0.117.0"`
    - [ ] Update `nu-protocol = { version = "0.117.0", features = ["plugin"] }`
    - [ ] Update `nu-plugin-test-support = "0.117.0"`
    - [ ] Update `nu-plugin-engine = { version = "0.117.0", features = ["local-socket"] }`
- [ ] Task: Clean build and resolve `Cargo.lock`
    - [ ] Run `cargo clean && cargo build --release`
    - [ ] Check compiler warnings via `cargo build 2>&1 | grep -i warning`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

---

## Phase 2: Breaking Fix Implementation
- [ ] Task: Adapt source code to any v0.117.0 API changes
    - [ ] Audit `src/lib.rs`, `src/main.rs`, and `src/color_plot/`
    - [ ] Verify trait implementations for `Plugin` and `SimplePluginCommand`
    - [ ] Verify error handling and value creation
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Breaking Fix Implementation' (Protocol in workflow.md)

---

## Phase 3: Test Suite Updates & Verification
- [ ] Task: Update test suite for v0.117.0
    - [ ] Review `tests/functional.rs` and update any signatures if needed
- [ ] Task: Run full test suite
    - [ ] Run `cargo test` and verify 100% pass rate
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Test Suite Updates & Verification' (Protocol in workflow.md)

---

## Phase 4: Integration Testing with Nushell Binary
- [ ] Task: Register plugin with Nushell 0.117.0
    - [ ] Run `plugin add ./target/release/nu_plugin_plot`
    - [ ] Run `plugin use plot`
- [ ] Task: Test plugin commands in Nushell
    - [ ] Test line plots: `seq 0 0.1 10 | math sin | plot`
    - [ ] Test histograms: `seq 1 100 | math random | hist`
    - [ ] Test xyplot: `let x = seq 0 0.1 10; let y = $x | math sin; [$x $y] | xyplot`
    - [ ] Verify `plugin list | where name == "nu_plugin_plot"` shows protocol alignment
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Integration Testing with Nushell Binary' (Protocol in workflow.md)

---

## Phase 5: Documentation, Remote Push & Cleanup
- [ ] Task: Update documentation
    - [ ] Update `conductor/tech-stack.md` version references to `0.117.0`
    - [ ] Update `README.md` if necessary
- [ ] Task: Commit changes locally
    - [ ] Stage `Cargo.toml`, `Cargo.lock`, `conductor/tech-stack.md`, `conductor/tracks.md`
    - [ ] Commit: `feat: Bump Nushell dependencies to 0.117.0`
- [ ] Task: Pull/rebase and push to GitHub remote
    - [ ] Run `git pull --rebase origin main`
    - [ ] Run `git push origin main`
- [ ] Task: Final cleanup
    - [ ] Run `cargo clean` to reclaim disk space
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Documentation, Remote Push & Cleanup' (Protocol in workflow.md)
