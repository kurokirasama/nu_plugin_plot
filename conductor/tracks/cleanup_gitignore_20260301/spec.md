# Track Specification: Clean up .gitignore

## Description
Ensure that the `.gitignore` file is cleaned up and only contains the following entries:
- `target/`
- `conductor/`
- `GEMINI.md`

## Goals
- Maintain a clean and consistent repository.
- Avoid committing Conductor-specific or build-specific files.

## Acceptance Criteria
- `.gitignore` file exists.
- `.gitignore` only contains `target/`, `conductor/`, and `GEMINI.md`.
- All other irrelevant entries are removed.
