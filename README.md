# Rust Core War

Core War implementation in Rust with terminal visualization, plus a C subproject under `core-war-c/`.

## Structure

- `src/` - Rust VM and game logic
- `core-war-c/` - C implementation and tests
- `examples/`, `benches/`, `tests/` - Supporting assets

## Current Status

- Rust crate and C subproject present with detailed specs and plans.
- Implementation not verified in this audit.
- Operational estimate: **40%** (substantial scaffold, unverified runtime).

## API Endpoints

- None. CLI/terminal application.

## Tests

- Rust tests/benches and C tests exist but were not run (avoided long Rust builds).

## Future Work

- Validate VM correctness with test suites.
- Document CLI usage and battle formats.
- Add automated CI for Rust and C builds.
