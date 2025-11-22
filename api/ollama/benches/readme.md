# Benchmarks

Performance benchmarks for the Ollama API client.

## Organization

- `diagnostics_performance.rs` - Diagnostics overhead measurements
- `cache_performance.rs` - Request cache performance measurements

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench --all-features

# Run specific benchmark
cargo bench --bench diagnostics_performance
```

## Interpreting Results

Benchmarks measure actual performance, not functional correctness.
Use for performance regression detection, not test suite validation.

## Why Benchmarks Instead of Tests

Performance measurements were originally in the test suite but caused flaky failures
due to timing variability across different systems and load conditions. Benchmarks
are the proper location for performance measurements per test_organization.rulebook.md.
