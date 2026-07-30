# Track Specification: Verbose Logging Mode (-v, --verbose)

## Overview
Add a `-v, --verbose` command-line flag to BlockScan to output real-time HTTP request, response, and link extraction diagnostics to `stderr`.

## Functional Requirements
1. **CLI Flag Addition (`src/cli.rs`):**
   - Add `-v, --verbose` flag to `Args` struct with clap attributes.
   - Document `--verbose` in `--help` as routing real-time diagnostic logs to `stderr`.
2. **Verbose Log Diagnostic Output:**
   - When `--verbose` is enabled, log each HTTP request attempt (URL, depth, thread permit).
   - Log HTTP response status, status code, response time, and extracted link count to `stderr`.
   - Ensure `stdout` remains clean for `--json` and `--plain` downstream piping.
3. **Engine & Checker Integration (`src/checker.rs`, `src/crawler.rs`):**
   - Pass verbose flag to `Crawler` and `Checker` to conditionally trigger `eprintln!` diagnostic messages.

## Non-Functional Requirements
- Maintain 100% unit test suite pass rate (`cargo test`).
- Ensure `cargo clippy -- -D warnings` and `cargo fmt --check` pass cleanly.
