# How-To: Implement nu_plugin_plot Nushell 0.115.0 Migration Track

This guide details the step-by-step implementation for track `bump_nushell_01150_20260815`.

## Prerequisites

- Rust 1.96.0+ installed (or the minimum required by Nushell 0.115.0)
- Target Nushell version (0.115.0) installed and available in PATH
- `nu_plugin_plot` builds and passes tests with its current dependency versions
- GitHub remote configured for `nu_plugin_plot` (`origin`)

## Phase 1: Cargo.toml Version Updates & Clean Build

### 1.1 Update Dependencies

Edit `Cargo.toml`, setting dependencies and package version to `0.115.0`:

```toml
[package]
version = "0.115.0"

[dependencies]
nu-plugin = "0.115.0"
nu-protocol = { version = "0.115.0", features = ["plugin"] }
owo-colors = "3.5.0"
fnv = "1.0.7"
term_size = "0.3.2"

[dev-dependencies]
nu-plugin-test-support = "0.115.0"
nu-plugin-engine = { version = "0.115.0", features = ["local-socket"] }
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

## Phase 2: Breaking Fix Implementation

### 2.1 Follow Adaptation Tasks

For each change identified in Nushell 0.115.0:
- Check `src/lib.rs` for `Plugin` and `SimplePluginCommand` trait signatures.
- Verify `src/main.rs` for `serve_plugin` invocations.
- Check `src/color_plot/` vendored modules (`textplots`, `drawille`) if any protocol/span/value representations changed.
- Preserve backward compatibility where possible.

### 2.2 Common Patterns

- **Trait signature changes**: Update `impl Plugin for PluginPlot` and `impl SimplePluginCommand for CommandPlot`, `CommandHist`, `CommandXyplot`.
- **Struct field changes**: Update `Value`, `Span`, `Type` construction/access.
- **Feature flags**: Ensure `nu-protocol` retains `features = ["plugin"]`.

### 2.3 Verify Compilation

```bash
cargo build --release
```

## Phase 3: Test Suite Updates & Verification

### 3.1 Update Tests

- Check `tests/` for API changes in `nu-plugin-test-support` 0.115.0.
- Update test expectations for any changed engine behavior.

### 3.2 Run Test Suite

```bash
cargo test
```

## Phase 4: Integration Testing with Nushell 0.115.0

### 4.1 Register Plugin

```nu
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
# Should show nu_plugin_plot as available and matching 0.115.0
```

## Phase 5: Documentation Updates

### 5.1 Update tech-stack.md

Update version references in `conductor/tech-stack.md`:
- `nu-plugin (v0.115.0)`
- `nu-protocol (v0.115.0)`
- `nu-plugin-engine (v0.115.0, local-socket feature)`
- `nu-plugin-test-support (v0.115.0)`

### 5.2 Update README / GEMINI / AGENTS

Check installation and version references across project guidelines.

## Phase 6: Remote Push & Cleanup

### 6.1 Commit Changes

```bash
cd /home/kira/Yandex.Disk/Development/linux/nushell/nu_plugin_plot
git add Cargo.toml src/ tests/ conductor/tech-stack.md conductor/tracks.md
git commit -m "feat: Bump Nushell dependencies to 0.115.0"
```

### 6.2 Push to Remote

```bash
git push origin HEAD
```

### 6.3 Cargo Clean

```bash
cargo clean
```

## Verification Commands

```bash
# Verify Cargo.toml versions
grep -A 10 "\[dependencies\]" Cargo.toml

# Verify build
cargo build --release 2>&1 | tail -5

# Verify tests
cargo test 2>&1 | tail -10

# Verify remote push
git log --oneline -1
git status
```
