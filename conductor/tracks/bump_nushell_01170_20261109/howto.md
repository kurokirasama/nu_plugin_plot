# How-To: Implement nu_plugin_plot Nushell Migration Track (v0.117.0)

This reference guides the implementation of the `nu_plugin_plot` Nushell migration track (`bump_nushell_01170_20261109`).

## Prerequisites

- Rust 1.96.0+ installed
- Target Nushell version `0.117.0` installed and available in PATH
- `nu_plugin_plot` builds and tests pass with current dependency versions (0.116.0)
- GitHub remote configured for `nu_plugin_plot` (`origin`)

### Baseline Check Before Changes

Before modifying `Cargo.toml` or `src/`, verify the existing baseline passes cleanly:

```bash
cd /home/kira/Yandex.Disk/Development/linux/nushell/nu_plugin_plot
cargo test
cargo build --release
```

## Phase 1: Cargo.toml Version Updates & Clean Build

### 1.1 Update Dependencies

Edit `Cargo.toml`, setting dependencies to `0.117.0`:

```toml
[package]
version = "0.117.0"

[dependencies]
nu-plugin = "0.117.0"
nu-protocol = { version = "0.117.0", features = ["plugin"] }
owo-colors = "3.5.0"
fnv = "1.0.7"
term_size = "0.3.2"

[dev-dependencies]
nu-plugin-test-support = "0.117.0"
nu-plugin-engine = { version = "0.117.0", features = ["local-socket"] }
```

### 1.2 Clean Build & Cargo.lock Update

Always start with a clean build to avoid stale-artifact errors, and verify that `Cargo.lock` resolves to the target `0.117.0` crates:

```bash
cd /home/kira/Yandex.Disk/Development/linux/nushell/nu_plugin_plot
cargo clean
cargo build --release
git status --short Cargo.toml Cargo.lock
```

### 1.3 Check Warnings

```bash
cargo build 2>&1 | grep -i warning
```

## Phase 2: Breaking Fix Implementation

### 2.1 Concrete Adaptation Tasks for v0.117.0
- **Plugin API Audit:** The Step 2.5 audit confirmed `nu_plugin_plot` does not directly use the removed `ShellError::GenericError`, `PipelineData::metadata()`, `current_work_dir()`, or `positional_nth()`.
- **Trait Implementations:** Verify `Plugin` and `SimplePluginCommand` implementations in `src/lib.rs` compile without signature mismatches.
- **Comparison Handling:** If any custom value operations use comparisons, verify exhaustiveness with the new `Comparison::OlderThan` and `Comparison::NewerThan` variants.

### 2.2 Verify Compilation

```bash
cargo build --release
```

## Phase 3: Test Suite Updates & Verification

### 3.1 Update Tests

- Modify `tests/functional.rs` for any test engine signature changes.
- Ensure test cases cover `plot`, `hist`, and `xyplot`.

### 3.2 Run Test Suite

```bash
cargo test
```

## Phase 4: Integration Testing with Nushell

### 4.1 Register Plugin

```bash
# From nu_plugin_plot project root
plugin add ./target/release/nu_plugin_plot
plugin use plot
```

### 4.2 Test Basic Functionality

```nu
# Line plots
let data = seq 0 0.1 10 | math sin
$data | plot

# Histograms
let data = seq 1 100 | math random
$data | hist

# XY plots
let x = seq 0 0.1 10
let y = $x | math sin
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

### 4.4 Verify Plugin Protocol & Communication

```bash
nu -c 'plugin list | where name == "nu_plugin_plot"'
# Should show nu_plugin_plot registered and matching protocol version 0.117.0
```

## Phase 5: Documentation Updates, Remote Push & Cleanup

### 5.1 Update tech-stack.md

Update version references in `conductor/tech-stack.md`:
- `nu-plugin (v0.117.0)`
- `nu-protocol (v0.117.0)`
- `nu-plugin-engine (v0.117.0, local-socket feature)`

### 5.2 Commit Changes Locally

```bash
cd /home/kira/Yandex.Disk/Development/linux/nushell/nu_plugin_plot
git add Cargo.toml Cargo.lock conductor/tech-stack.md conductor/tracks.md
git commit -m "feat: Bump Nushell dependencies to 0.117.0"
```

### 5.3 Pull/Rebase & Push to Remote

```bash
git pull --rebase origin main
git push origin main
```

### 5.4 Cargo Clean

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

# Verify plugin protocol registration and execution in a fresh Nushell process
nu -c 'plugin add ./target/release/nu_plugin_plot; plugin use plot; [1 2 3] | plot'
nu -c 'plugin list | where name == "nu_plugin_plot"'

# Verify remote push
git log --oneline -1
git status
```
