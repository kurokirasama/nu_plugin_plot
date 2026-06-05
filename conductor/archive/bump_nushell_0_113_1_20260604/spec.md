# Specification: Bump Nushell and App version to 0.113.1

## Overview
This chore track updates the Nushell SDK dependencies and the package version of `nu_plugin_plot` to `0.113.1` to align with the host Nushell version. An SDK-first compilation audit will be performed immediately to resolve any breaking changes or APIs introduced in `0.113.1`.

## Functional Requirements
1. **Dependency Updates**:
   - Update `nu-plugin` dependency to `0.113.1` in `Cargo.toml`.
   - Update `nu-protocol` dependency to `0.113.1` in `Cargo.toml`.
   - Update `nu-plugin-test-support` dev-dependency to `0.113.1` in `Cargo.toml`.
2. **Crate Version Update**:
   - Update `nu_plugin_plot` version to `0.113.1` in `Cargo.toml`.
3. **Compilation Audit**:
   - Execute `cargo build` and fix any compilation/API compilation errors.
4. **Local Verification**:
   - Register the compiled plugin in the local Nushell `0.113.1` environment.
   - Run a test command (e.g. `let data = (seq 0 0.1 10 | math sin); $data | plot`) to verify it functions as expected.

## Acceptance Criteria
- Crate version is `0.113.1`.
- Dependencies `nu-plugin`, `nu-protocol`, and `nu-plugin-test-support` are set to `0.113.1`.
- `cargo build` compiles without errors.
- `cargo test` runs and passes successfully.
- Plugin registers and executes successfully in Nushell `0.113.1`.

## Out of Scope
- Adding new features or plotting commands.
- Enhancements or performance optimizations unrelated to compatibility.
