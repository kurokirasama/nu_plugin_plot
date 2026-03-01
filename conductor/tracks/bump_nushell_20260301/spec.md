# Track Specification: Bump Nushell Version to 0.111.0

## 1. Overview
This track involves upgrading the project's Nushell-related dependencies to version 0.111.0. This ensures compatibility with the latest Nushell features and improvements while maintaining the plugin's core functionality.

## 2. Scope of Work
- **Dependency Update:** Modify `Cargo.toml` to update `nu-plugin` and `nu-protocol` dependencies to version `0.111.0`.
- **Code Audit:** Review the plugin implementation for any compatibility issues or compiler errors introduced by the version bump.
- **Verification:** Run the automated test suite to ensure no regressions in plotting functionality.

## 3. Goals
- Successfully upgrade to Nushell 0.111.0.
- Maintain existing plotting features (`plot`, `hist`, `xyplot`).
- Ensure the plugin builds and runs correctly in the new environment.

## 4. Acceptance Criteria
- `Cargo.toml` reflects version `0.111.0` for core Nushell dependencies.
- The project compiles without errors.
- All automated tests pass successfully.
- Basic plotting functionality is verified as working.
