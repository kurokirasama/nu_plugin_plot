# Implementation Plan: Bump Nushell Dependencies to v0.116.0

## Phase 1: Cargo.toml Version Updates & Clean Build
- [ ] Task: Read howto.md before starting this phase
- [ ] Task: Update package and dependency versions in Cargo.toml
    - [ ] Set `version = "0.116.0"`
    - [ ] Update `nu-plugin = "0.116.0"`
    - [ ] Update `nu-protocol = { version = "0.116.0", features = ["plugin"] }`
    - [ ] Update `nu-plugin-test-support = "0.116.0"`
    - [ ] Update `nu-plugin-engine = { version = "0.116.0", features = ["local-socket"] }`
- [ ] Task: Execute clean build
    - [ ] Run `cargo clean`
    - [ ] Run `cargo build --release`
    - [ ] Inspect any compiler warnings
- [ ] Task: Conductor - User Manual Verification 'Cargo.toml Version Updates & Clean Build' (Protocol in workflow.md)

## Phase 2: Breaking Fix Implementation
- [ ] Task: Audit and resolve source-level API changes
    - [ ] Inspect `src/main.rs`, `src/lib.rs`, and commands for trait/signature adaptations
    - [ ] Compile and verify zero compilation errors
- [ ] Task: Conductor - User Manual Verification 'Breaking Fix Implementation' (Protocol in workflow.md)

## Phase 3: Test Suite Updates & Verification
- [ ] Task: Run and adapt test suite
    - [ ] Run `cargo test`
    - [ ] Update any test assertions or test fixtures in `tests/`
- [ ] Task: Conductor - User Manual Verification 'Test Suite Updates & Verification' (Protocol in workflow.md)

## Phase 4: Integration Testing with Nushell
- [ ] Task: Verify live plugin registration in Nushell
    - [ ] Run `plugin add ./target/release/nu_plugin_plot`
    - [ ] Verify `seq 1 10 | plot` produces valid ANSI graphs
- [ ] Task: Conductor - User Manual Verification 'Integration Testing with Nushell' (Protocol in workflow.md)

## Phase 5: Documentation Updates & Remote Push
- [ ] Task: Update documentation and tech-stack references
    - [ ] Update `conductor/tech-stack.md` versions to 0.116.0
    - [ ] Update README.md if version numbers are referenced
- [ ] Task: Stage, commit, push, and clean
    - [ ] Stage and commit changes with `feat: Bump Nushell dependencies to 0.116.0`
    - [ ] Push to GitHub remote `origin/main`
    - [ ] Run `cargo clean`
- [ ] Task: Conductor - User Manual Verification 'Documentation Updates & Remote Push' (Protocol in workflow.md)
