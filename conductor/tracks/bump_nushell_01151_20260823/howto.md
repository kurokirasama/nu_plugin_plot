# How-To: Implement nu_plugin_plot Nushell Migration Track (v0.115.1)

This guide provides step-by-step implementation instructions for the `bump_nushell_01151_20260823` track, adapted from the reference template for Nushell 0.115.1.

## Prerequisites

- Rust 1.96.0+ installed (or minimum required by Nushell 0.115.1)
- Nushell 0.115.1 installed and available in PATH (`nu --version` shows `0.115.1`)
- `nu_plugin_plot` builds and tests pass with current 0.115.0 dependencies
- GitHub remote configured for `nu_plugin_plot` (`origin`)

---

## Phase 1: Cargo.toml Version Updates & Clean Build

### 1.1 Update Dependencies

Edit `Cargo.toml`, replacing `0.115.0` with `0.115.1`:

```toml
[package]
authors = ["Max Brown"]
description = "Plot graphs in nushell using numerical lists."
repository = "https://github.com/euphrasiologist/nu_plugin_plot"
edition = "2021"
license = "MIT"
name = "nu_plugin_plot"
version = "0.115.1"

[dependencies]
nu-plugin = "0.115.1"
nu-protocol = { version = "0.115.1", features = ["plugin"] }
owo-colors = "3.5.0"
fnv = "1.0.7"
term_size = "0.3.2"

[dev-dependencies]
nu-plugin-test-support = "0.115.1"
nu-plugin-engine = { version = "0.115.1", features = ["local-socket"] }
```

### 1.2 Clean Build

Always start with a clean build to avoid stale-artifact errors:

```bash
cd /home/kira/Yandex.Disk/Development/linux/nushell/nu_plugin_plot
cargo clean
cargo build --release
```

### 1.3 Check Warnings

```bash
cargo build 2>&1 | grep -i warning
```

---

## Phase 2: Breaking Fix Implementation

### 2.1 Verify No Breaking Changes (Patch Release)

Nushell 0.115.1 is a **patch release with no plugin API breaking changes**. The release includes:
- Bug fixes: semver comparisons, `$ans`, SQLite `update`, keybinding merging
- Completion improvements: Helix keybindings, background-completions opt-out
- No trait signature changes, struct field changes, or macro changes

**Action:** Verify compilation succeeds — no code adaptations needed.

```bash
cargo build --release
# Should succeed without errors
```

### 2.2 Common Patterns (for reference if future breaking changes occur)

- **Trait signature changes**: Update `impl Plugin for ...` and `impl PluginCommand for ...`
- **Struct field changes**: Update `Value`, `Span`, `Type` construction/access
- **Macro changes**: Update `plugin_command!`, `register_plugin!` invocations
- **Feature flags**: Adjust `nu-protocol` features in `Cargo.toml`
- **Plugin protocol changes**: Review `nu-plugin` protocol version negotiation if plugins fail to register

### 2.3 Verify Compilation

```bash
cargo build --release
```

---

## Phase 3: Test Suite Updates & Verification

### 3.1 Run Test Suite

```bash
cargo test
```

All unit and functional tests should pass with 0.115.1 dependencies.

### 3.2 Check Coverage (if applicable)

```bash
cargo tarpaulin --out Html  # if installed
```

---

## Phase 4: Integration Testing with Nushell 0.115.1

### 4.1 Register Plugin

```bash
# From nu_plugin_plot project root
plugin add ./target/release/nu_plugin_plot
plugin use plot
```

### 4.2 Test Basic Functionality

```nu
# Line plots
let data = (seq 0 0.1 10 | math sin)
$data | plot

# Histograms
let data = (seq 1 100 | math random)
$data | hist

# XY plots
let x = (seq 0 0.1 10)
let y = ($x | math sin)
[$x $y] | xyplot
```

### 4.3 Test Edge Cases

```nu
# Empty data
[] | plot

# Single value
[5] | plot

# Negative values
[-5 -3 -1 1 3 5] | plot
```

### 4.4 Verify Plugin Communication

```nu
plugin list
# Should show nu_plugin_plot as available
```

---

## Phase 5: Documentation Updates

### 5.1 Update tech-stack.md

Update version references in `conductor/tech-stack.md`:

- `nu-plugin (v0.115.1)`
- `nu-protocol (v0.115.1)`
- `nu-plugin-engine (v0.115.1, local-socket feature)`
- Rust minimum version if changed

### 5.2 Update README (if needed)

Check installation instructions for version references.

---

## Phase 6: Remote Push & Cleanup

### 6.1 Commit Changes

```bash
cd /home/kira/Yandex.Disk/Development/linux/nushell/nu_plugin_plot
git add Cargo.toml src/ tests/ conductor/tech-stack.md conductor/tracks.md
git commit -m "feat: Bump Nushell dependencies to 0.115.1"
```

### 6.2 Push to Remote

```bash
git push origin main
```

### 6.3 Cargo Clean

```bash
cargo clean
```

---

## Verification Commands

```bash
# Verify Cargo.toml versions
grep -A 10 "\[dependencies\]" Cargo.toml

# Verify build
cargo build --release 2>&1 | tail -5

# Verify tests
cargo test 2>&1 | tail -10

# Verify plugin loads in Nushell 0.115.1
nu -c 'plugin add ./target/release/nu_plugin_plot; plugin use plot; [1 2 3] | plot'

# Verify remote push
git log --oneline -1
git status
```

---

## Common Pitfalls

- **Don't forget `cargo clean`** before build — stale artifacts cause confusing errors
- **Version slug format**: `0.115.1` -> `01151` (major*10000 + minor*10 + patch)
- **Remote branch**: Push to `main` (current default branch)
- **Nushell binary**: Must be at 0.115.1 before integration testing
- **Feature flags**: `nu-protocol` requires `features = ["plugin"]` for plugins
- **Test support**: `nu-plugin-test-support` and `nu-plugin-engine` must match version exactly
- **Local copies**: `textplots` and `drawille` in `src/color_plot/` are vendored — don't update unless necessary
- **Hardlinked files**: `~/.agents/skills/` and `llms_configs/skills/` SKILL.md are hardlinked — edits to one affect both

---

## Version-Specific Notes for 0.115.1

This is a **patch release** from 0.115.0 → 0.115.1. Key points:

1. **No breaking changes** to plugin API — `SimplePluginCommand` and `Plugin` traits unchanged
2. **No deprecations** affecting plugin code
3. **Protocol alignment required** — plugins must match host Nushell version exactly
4. **Dependency bump only** — Cargo.toml version updates, clean build, test, integrate, push
5. **Previous 0.115.0 track archived** — this is a fresh track for the patch release