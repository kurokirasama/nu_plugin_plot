# Implementation Plan: Weather Plugin Bug Fix (Type Conversion & Crash)

## Phase 1: Investigation & Reproduction
- [x] Task: Reproduce the plugin crash
    - [x] Create a minimal Nushell script to trigger the crash using various input types (empty list, Int64, etc.)
    - [x] Confirm the "Error converting list to series" message origin.
    - [x] Determine if the error is caused by the plugin's internal handling or the data/invocation method used in the `weather` command.
- [x] Task: Locate crash point in Rust source
    - [x] Run the plugin with RUST_BACKTRACE=1 to identify the panicking line.
- [x] Task: Conductor - User Manual Verification 'Investigation & Reproduction' (Protocol in workflow.md) [checkpoint: Phase1]

## Phase 2: Robustness Implementation
- [x] Task: Implement Red Phase (Write failing tests)
    - [x] Add unit tests in `tests/functional.rs` that simulate the crash-inducing inputs.
- [x] Task: Implement Green Phase (Fix the plugin crash)
    - [x] Add bounds checks for empty lists in `check_equality_of_list` and `plot_nested`.
    - [x] Refactor flag parsing to handle potential type mismatches safely.
    - [x] Improve `Value` to numerical conversion to support a wider range of types (e.g., ensuring `Int64` is handled correctly).
- [x] Task: Fix the `weather` command script
    - [x] Read external conductor protocols at `/home/kira/Yandex.Disk/my_scripts/nushell/conductor/workflow.md`
    - [x] Modify `/home/kira/Yandex.Disk/my_scripts/nushell/weather_tomorrow.nu` to handle the Polars type conversion properly or sanitize inputs before plotting.
- [x] Task: Conductor - User Manual Verification 'Robustness Implementation' (Protocol in workflow.md) [checkpoint: Phase2]

## Phase 3: Verification & QA
- [x] Task: Verify with reproduction script
    - [x] Run the minimal script from Phase 1 and confirm it no longer crashes.
- [x] Task: Verify with weather command
    - [x] Test the `weather` command in a live Nushell instance and ensure it renders plots correctly.
- [x] Task: Conductor - User Manual Verification 'Verification & QA' (Protocol in workflow.md) [checkpoint: 67c42b1]
