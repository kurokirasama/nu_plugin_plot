# Track Specification: update nushell dependency to v0.112.1

## Overview
Update all Nushell-related dependencies (`nu-plugin`, `nu-protocol`) to version `0.112.1` to ensure compatibility with the latest Nushell release and leverage potential performance improvements or bug fixes.

## Type
Chore/Refactor

## Functional Requirements
- [ ] Update `nu-plugin` version to `0.112.1` in `Cargo.toml`.
- [ ] Update `nu-protocol` version to `0.112.1` in `Cargo.toml`.
- [ ] Ensure any other `nu-` crates (if found) are updated to compatible versions.
- [ ] Adjust plugin code if necessary to accommodate any breaking changes in the `nu-plugin` or `nu-protocol` APIs.

## Non-Functional Requirements
- **Performance:** Maintain or improve plugin performance.
- **Reliability:** Ensure all existing commands (`plot`, `hist`, `xyplot`) function correctly after the update.

## Acceptance Criteria
- [ ] The plugin builds successfully using `cargo build`.
- [ ] All unit and integration tests pass with `cargo test`.
- [ ] The plugin is registerable and usable in a Nushell v0.112.1 environment (verified via `cargo test` as primary automated check).

## Out of Scope
- Adding new plotting features or commands.
- Major refactoring unrelated to the dependency update.