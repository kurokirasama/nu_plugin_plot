# Specification: Bump Nushell Dependencies to v0.116.0

## 1. Overview
- **Target Nushell Version:** `0.116.0` (Released: 2026-09-26)
- **Previous Nushell Version:** `0.115.1`
- **Objective:** Update `nu_plugin_plot` Rust dependencies to match Nushell v0.116.0, resolve any plugin API shifts, verify full test suite passes, and ensure seamless plugin communication.

---

## 2. Scope & Dependencies
- **Package Version:** Bump `version` in `Cargo.toml` from `0.115.1` to `0.116.0`.
- **Dependencies (`Cargo.toml`):**
  - `nu-plugin = "0.116.0"`
  - `nu-protocol = { version = "0.116.0", features = ["plugin"] }`
- **Dev-Dependencies (`Cargo.toml`):**
  - `nu-plugin-test-support = "0.116.0"`
  - `nu-plugin-engine = { version = "0.116.0", features = ["local-socket"] }`

---

## 3. Compatibility & Breaking Fix Requirements
- Audit source files (`src/lib.rs`, `src/main.rs`, `src/color_plot/`) against v0.116.0 `nu-plugin` API.
- Rebuild cleanly (`cargo clean && cargo build --release`).
- Verify no compiler warnings or deprecated API usage.

---

## 4. Testing & Verification Requirements
- Update `tests/functional.rs` if any test support signatures changed.
- Run `cargo test` and ensure all unit and integration tests pass.
- Test plugin loading and command execution in Nushell:
  - `[1 2 3] | plot`
  - `seq 1 50 | math sin | plot`

---

## 5. Documentation & Archival Requirements
- Update `conductor/tech-stack.md` version references to `0.116.0`.
- Update `README.md` if installation instructions reference `0.115.1`.
- Clean cargo target cache (`cargo clean`) to conserve disk space.

---

## 6. Acceptance Criteria
- [ ] `Cargo.toml` dependencies all point to `0.116.0`.
- [ ] Clean compilation with zero errors or warnings (`cargo build --release`).
- [ ] All tests pass cleanly (`cargo test`).
- [ ] Plugin registered and verified in Nushell.
- [ ] Changes committed locally and pushed to GitHub remote `origin/main`.
- [ ] Target directory cleaned via `cargo clean`.
