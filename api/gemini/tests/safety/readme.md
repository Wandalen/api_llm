# Safety Tests

## Purpose
Contains tests validating Gemini API safety settings, content filtering, and harm category controls.

## Organization Principles
- Split into integration_part1.rs and integration_part2.rs due to file size constraints
- unit.rs contains focused safety configuration tests
- mod.rs provides shared test utilities and mock data structures

## Navigation Guide
- Safety settings API: integration_part1.rs, integration_part2.rs
- Configuration validation: unit.rs
- Test helpers: mod.rs
