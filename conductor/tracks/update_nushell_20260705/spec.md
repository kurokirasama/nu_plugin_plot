# Specification: Update Nushell dependencies to version 0.114.0

## Overview
This track updates the core Nushell dependencies from version 0.113.1 to 0.114.0 to maintain compatibility with the latest Nushell release and leverage any improvements or bug fixes in the plugin ecosystem.

## Scope
Update the following core Nushell dependencies:
- `nu-plugin` from 0.113.1 to 0.114.0
- `nu-protocol` from 0.113.1 to 0.114.0
- `nu-plugin-test-support` from 0.113.1 to 0.114.0
- `nu-plugin-engine` from 0.113.1 to 0.114.0 (dev-dependency)

## Compatibility Requirements
- Ensure the plugin compiles successfully with Nushell 0.114.0
- Verify that all existing functionality works as expected
- Maintain compatibility with Rust 1.96.0+ as specified in the tech stack

## Testing Requirements
- Run the existing test suite to verify no regressions
- Test basic plotting functionality (line plots, histograms, XY plots)
- Verify plugin registration and communication with Nushell

## Acceptance Criteria
- [ ] All dependencies are updated to 0.114.0 in Cargo.toml
- [ ] Plugin compiles successfully with the new dependencies
- [ ] All existing tests pass
- [ ] Basic plotting functionality works in Nushell 0.114.0
- [ ] No breaking changes in plugin behavior

## Out of Scope
- Updating local modified copies of textplots and drawille
- Adding new features or functionality
- Updating other dependencies not related to Nushell
- Changing the minimum Rust version requirement
