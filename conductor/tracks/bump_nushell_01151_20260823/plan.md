# Implementation Plan: Bump Nushell Dependencies to v0.115.1

**Track ID:** `bump_nushell_01151_20260823`  
**Read howto.md first:** This track's `howto.md` provides step-by-step implementation guidance for each phase.  
**Workflow:** Follows `nu_plugin_plot/conductor/workflow.md` (TDD approach).

---

## Phase 1: Cargo.toml Version Updates & Clean Build

- [ ] Task: **Read howto.md in this track before starting implementation**
- [ ] Task: Update `Cargo.toml` dependencies from 0.115.0 to 0.115.1
    - [ ] `[package].version` = "0.115.1"
    - [ ] `[dependencies].nu-plugin` = "0.115.1"
    - [ ] `[dependencies].nu-protocol` = { version = "0.115.1", features = ["plugin"] }
    - [ ] `[dev-dependencies].nu-plugin-test-support` = "0.115.1"
    - [ ] `[dev-dependencies].nu-plugin-engine` = { version = "0.115.1", features = ["local-socket"] }
- [ ] Task: Clean build
    - [ ] `cargo clean`
    - [ ] `cargo build --release`
- [ ] Task: Check for warnings
    - [ ] `cargo build 2>&1 | grep -i warning`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

## Phase 2: Breaking Fix Implementation

- [ ] Task: Verify compilation with 0.115.1
    - [ ] `cargo build --release` — should succeed without errors
    - [ ] Check `src/lib.rs` — `Plugin` and `SimplePluginCommand` traits compile
    - [ ] Check `src/main.rs` — `serve_plugin` entrypoint compiles
    - [ ] Check `src/color_plot/` — vendored modules compile
- [ ] Task: No breaking changes expected (patch release) — verify zero adaptations needed
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Breaking Fix Implementation' (Protocol in workflow.md)

## Phase 3: Test Suite Updates & Verification

- [ ] Task: Run test suite
    - [ ] `cargo test` — all unit and functional tests pass
- [ ] Task: Verify test compilation with updated dependencies
    - [ ] `tests/functional.rs` compiles with `nu-plugin-test-support` 0.115.1
    - [ ] `nu-plugin-engine` 0.115.1 works for integration tests
- [ ] Task: Check coverage (optional)
    - [ ] `cargo tarpaulin --out Html` if installed
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Test Suite Updates & Verification' (Protocol in workflow.md)

## Phase 4: Integration Testing with Nushell 0.115.1

- [ ] Task: Prerequisite — Nushell 0.115.1 binary installed and in PATH
- [ ] Task: Register plugin
    - [ ] `plugin add ./target/release/nu_plugin_plot`
    - [ ] `plugin use plot`
- [ ] Task: Test basic functionality
    - [ ] Line plots: `let data = (seq 0 0.1 10 | math sin); $data | plot`
    - [ ] Histograms: `let data = (seq 1 100 | math random); $data | hist`
    - [ ] XY plots: `let x = (seq 0 0.1 10); let y = ($x | math sin); [$x $y] | xyplot`
- [ ] Task: Test edge cases
    - [ ] Empty data: `[] | plot`
    - [ ] Single value: `[5] | plot`
    - [ ] Negative values: `[-5 -3 -1 1 3 5] | plot`
- [ ] Task: Verify plugin communication
    - [ ] `plugin list` shows `nu_plugin_plot` as available
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Integration Testing with Nushell' (Protocol in workflow.md)

## Phase 5: Documentation Updates

- [ ] Task: Update `conductor/tech-stack.md`
    - [ ] `nu-plugin (v0.115.1)`
    - [ ] `nu-protocol (v0.115.1)`
    - [ ] `nu-plugin-engine (v0.115.1, local-socket feature)`
    - [ ] Rust minimum version if changed
- [ ] Task: Update README (if version references exist)
    - [ ] Check installation instructions for version references
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Documentation Updates' (Protocol in workflow.md)

## Phase 6: Remote Push & Cleanup

- [ ] Task: Commit changes
    - [ ] `git add Cargo.toml src/ tests/ conductor/tech-stack.md conductor/tracks.md`
    - [ ] `git commit -m "feat: Bump Nushell dependencies to 0.115.1"`
- [ ] Task: Push to remote
    - [ ] `git push origin main`
- [ ] Task: Final cargo clean
    - [ ] `cargo clean`
- [ ] Task: Conductor - User Manual Verification 'Phase 6: Remote Push & Cleanup' (Protocol in workflow.md)