# How-To: Implement nu_plugin_plot Nushell Migration Track (v0.116.0)

This guide provides concrete instructions for executing the `bump_nushell_01160_20260930` track in `nu_plugin_plot`.

## Prerequisites
- Rust toolchain installed (stable)
- Target Nushell version 0.116.0 installed
- Remote configured: `origin git@github.com:kurokirasama/nu_plugin_plot.git`

---

## Phase 1: Cargo.toml Updates & Clean Build

### 1.1 Update `Cargo.toml`
Edit `Cargo.toml`:
```toml
[package]
version = "0.116.0"

[dependencies]
nu-plugin = "0.116.0"
nu-protocol = { version = "0.116.0", features = ["plugin"] }

[dev-dependencies]
nu-plugin-test-support = "0.116.0"
nu-plugin-engine = { version = "0.116.0", features = ["local-socket"] }
```

### 1.2 Clean Build
```bash
cargo clean
cargo build --release
```

---

## Phase 2: Breaking Fix Implementation
- Check compiler output for any changes in `Plugin` or `PluginCommand` traits.
- Apply minimal updates in `src/` if any struct field or signature changed.

---

## Phase 3: Test Suite Updates
```bash
cargo test
```

---

## Phase 4: Integration Testing in Nushell
```bash
nu -c 'plugin add ./target/release/nu_plugin_plot; plugin use plot; [1 2 3] | plot'
```

---

## Phase 5: Documentation, Remote Push, and Cargo Clean

### 5.1 Update `conductor/tech-stack.md`
Update `nu-plugin`, `nu-protocol`, and `nu-plugin-engine` version references to `v0.116.0`.

### 5.2 Commit and Push
```bash
git add Cargo.toml src/ tests/ conductor/
git commit -m "feat: Bump Nushell dependencies to 0.116.0"
git push origin main
```

### 5.3 Cargo Clean
```bash
cargo clean
```
