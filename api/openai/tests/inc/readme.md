# Test Infrastructure

## Purpose

Shared test utilities, test data factories, and common test infrastructure used across the test suite.

## Organization Principles

- **Test utilities**: Common test helpers and utilities (mod.rs, basic_test.rs)
- **Test data factories**: Builders for creating test data (test_data_factories.rs)
- **Component tests**: Tests for shared component types (components_test/)
- **Experimental features**: Experimental test code and prototypes (experiment.rs)

## Navigation Guide

- For basic test utilities: `basic_test.rs`
- For test data factories and builders: `test_data_factories.rs`
- For component serialization/deserialization tests: `components_test/`
- For experimental test code: `experiment.rs`
- For test module organization: `mod.rs`
