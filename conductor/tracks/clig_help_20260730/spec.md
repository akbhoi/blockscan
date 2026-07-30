# Track Specification: CLIG-Compliant CLI Help & Documentation Enhancement

## Overview
Enhance BlockScan's CLI help system (`src/cli.rs`) to strictly follow [clig.dev](https://clig.dev/) guidelines. The updated help output will clearly document all features, parameters, default values, output stream behaviors (`stdout` vs `stderr`), and multi-line usage examples.

## Functional Requirements
1. **CLIG-Compliant Help Header & Description:**
   - Add rich `about` and `long_about` documentation to Clap derive attributes.
   - Include concrete multi-line usage examples directly in help output (Basic scan, depth/concurrency configuration, JSON/jq pipelines).
2. **Comprehensive Option & Flag Documentation:**
   - Document `<URL>` argument requirement with URL auto-prepending (`https://`) behavior.
   - Document `-d, --depth <DEPTH>` (0 for unlimited, default 2).
   - Document `-c, --concurrency <C>` (Tokio Semaphore bounded concurrency, default 10).
   - Document `-u, --user-agent <UA>` (custom User-Agent string).
   - Document `-j, --json` (machine-readable JSON output to `stdout`, suppressing spinners).
   - Document `--plain` (unformatted tab-separated text to `stdout`).
   - Document `--no-color` (disables ANSI color codes, respects `NO_COLOR` env var).
3. **Stream & Pipe Transparency:**
   - Clearly explain in `--help` that data is emitted to `stdout` while progress spinners and logs go to `stderr`.

## Non-Functional Requirements
- Maintain 100% test suite pass rate (`cargo test`).
- Ensure `cargo clippy -- -D warnings` and `cargo fmt --check` pass cleanly.
