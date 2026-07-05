# How-to Guide: Update Nushell Dependencies to 0.114.0

This guide provides step-by-step instructions for updating the nu_plugin_plot dependencies from Nushell 0.113.1 to 0.114.0.

## Prerequisites

- Rust 1.96.0+ installed
- Nushell 0.114.0 installed and available in PATH
- Current project builds and tests pass with 0.113.1

## Step 1: Update Cargo.toml Dependencies

Open `Cargo.toml` and update the following dependencies:

```toml
[dependencies]
nu-plugin = "0.114.0"
nu-protocol = { version = "0.114.0", features = ["plugin"] }
owo-colors = "3.5.0"
fnv = "1.0.7"
term_size = "0.3.2"

[dev-dependencies]
nu-plugin-test-support = "0.114.0"
nu-plugin-engine = { version = "0.114.0", features = ["local-socket"] }
```

Also update the package version if appropriate:
```toml
version = "0.114.0"  # or appropriate semantic version
```

## Step 2: Build and Test

### 2.1 Clean Build
```bash
cargo clean
cargo build --release
```

### 2.2 Run Tests
```bash
cargo test
```

### 2.3 Check for Warnings
```bash
cargo build 2>&1 | grep -i warning
```

## Step 3: Integration Testing with Nushell

### 3.1 Register Plugin
```bash
# From project root
plugin add ./target/release/nu_plugin_plot
plugin use plot
```

### 3.2 Test Basic Functionality

Test line plots:
```nushell
let data = (seq 0 0.1 10 | math sin)
$data | plot
```

Test histograms:
```nushell
let data = (seq 1 100 | math random)
$data | hist
```

Test XY plots:
```nushell
let x = (seq 0 0.1 10)
let y = ($x | math sin)
[$x $y] | xyplot
```

### 3.3 Test Edge Cases
```nushell
# Empty data
[] | plot

# Single value
[5] | plot

# Negative values
[-5 -3 -1 1 3 5] | plot
```

## Step 4: Verify Compatibility

### 4.1 Check Plugin Communication
```nushell
plugin list
# Should show nu_plugin_plot as available
```

### 4.2 Test Error Handling
```nushell
# Invalid input
"not a number" | plot
```

## Step 5: Update Documentation

### 5.1 Update tech-stack.md
Update the version references in `conductor/tech-stack.md`:
- Change `nu-plugin (v0.113.1)` to `nu-plugin (v0.114.0)`
- Change `nu-protocol (v0.113.1)` to `nu-protocol (v0.114.0)`
- Update Rust minimum version if changed

### 5.2 Update README (if needed)
Check if installation instructions need updating.

## Step 6: Final Verification

Run the complete test suite one more time:
```bash
cargo test --all-features
```

Verify all acceptance criteria are met:
- [ ] All dependencies updated to 0.114.0 in Cargo.toml
- [ ] Plugin compiles successfully
- [ ] All existing tests pass
- [ ] Basic plotting functionality works in Nushell 0.114.0
- [ ] No breaking changes in plugin behavior

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Check for API changes in nu-plugin/nu-protocol 0.114.0
   - Review Nushell 0.114.0 release notes
   - Check for breaking changes in plugin API

2. **Test Failures**: 
   - Update test expectations if API changed
   - Check if test-support API changed

3. **Plugin Not Loading**:
   - Ensure Nushell 0.114.0 is running
   - Re-register plugin: `plugin add ./target/release/nu_plugin_plot`

### Rollback Procedure
If issues cannot be resolved:
```bash
git checkout Cargo.toml
cargo clean
cargo build --release
```

## References

- [Nushell 0.114.0 Release Notes](https://github.com/nushell/nushell/releases/tag/0.114.0)
- [nu-plugin API Documentation](https://docs.rs/nu-plugin/0.114.0/nu_plugin/)
- [nu-protocol API Documentation](https://docs.rs/nu-protocol/0.114.0/nu_protocol/)
