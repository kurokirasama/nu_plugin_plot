# Update Conductor Setup to Align with Core Protocols

## Objective
Update the existing conductor setup in `nu_plugin_plot` to fully integrate the `conductor-core-protocols`. This involves:
1. Updating `conductor/workflow.md` to reflect Nushell-first guidelines and Discord reporting.
2. Refining `conductor/product-guidelines.md` and `conductor/tech-stack.md` if necessary.
3. Ensuring the `conductor/tracks.md` and track structure are ready for future work.

## Key Files & Context
- `conductor/workflow.md`: Main source of truth for the development lifecycle.
- `conductor/product-guidelines.md`: Aesthetic and technical standards.
- `conductor/tech-stack.md`: Technology choices.
- `conductor/tracks.md`: Registry of project tracks.

## Implementation Steps

### 1. Update Workflow (`conductor/workflow.md`)
- [ ] Integrate **Nushell-First Guidelines**: Explicitly mandate the use of Nushell pipelines and the `evaluate` tool as the primary interaction method.
- [ ] Integrate **Discord Notifications**: Add the protocol for long-running tasks (5min+) to send reports via `to discord`.
- [ ] Refine **Task Workflow**: Ensure the TDD (Red/Green/Refactor) and Commit/Note protocols are clearly stated and consistent with core protocols.
- [ ] Standardize **Manual Verification**: Keep the structured verification plan format but ensure it aligns with Nushell usage.

### 2. Update Product Guidelines (`conductor/product-guidelines.md`)
- [ ] Reinforce **Nushell-Native Rendering**: Ensure guidelines emphasize integration with Nushell's data types and rendering capabilities.

### 3. Update Tech Stack (`conductor/tech-stack.md`)
- [ ] Explicitly list **Nushell** and its versioning as a core part of the development environment.

### 4. Verify Directory Structure
- [ ] Confirm `conductor/tracks/` is ready for new tracks.
- [ ] Ensure `.gitignore` correctly handles `conductor/` while allowing the agent to work with it (the agent already has mechanisms to bypass ignore if needed, but the user should be aware).

## Verification & Testing
- [ ] **Self-Audit**: Verify all updated files against the `conductor-core-protocols` skill.
- [ ] **Tool Check**: Test a simple Nushell command via `evaluate` to ensure the environment is correctly set up (if not already done).
