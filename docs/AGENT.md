# Agent Instructions: Blockscan Development

This file provides specific instructions, architectural constraints, and context for AI agents working on the blockscan project. Read these instructions carefully before proceeding with implementation.

## Project Context
Blockscan is a Rust-based CLI tool designed to detect if websites actively block bot traffic and to discover crawlable in-domain links.

## Rust Specifics & Architecture
- **Toolchain:** Use the latest stable Rust toolchain.
- **Asynchronous Runtime:** Use `tokio` for handling concurrent network I/O efficiently.
- **HTTP Client:** Use `reqwest` to perform HTTP requests.
  - Must support configuring the `User-Agent` header.
  - Handle redirects appropriately.
  - Implement timeout mechanisms to avoid hanging requests.
- **HTML Parsing:** Use `scraper` to parse HTML responses and extract URLs via CSS selectors (e.g., `a[href]`).
- **URL Handling:** Use the `url` crate for parsing, validating, and normalizing URLs. This is critical for ensuring that discovered links belong to the target's top-level domain and resolving relative paths.
- **CLI Framework:** Use `clap` (with the `derive` feature) for defining and parsing command-line arguments.
- **CLI Guidelines:** Strictly follow [clig.dev](https://clig.dev/) guidelines for CLI design.
  - **Human-First:** Ensure the CLI is conversational, responsive, and robust.
  - **Output Streams:** Direct all primary data to `stdout`. Direct all logs, errors, headers, and decorative elements to `stderr`.
  - **Machine Readable:** Ensure `stdout` is easily consumable by tools like `grep` or `jq`. Support `--plain` and `--json`.
  - **Colors:** Use colors intentionally but provide flags (e.g., `--no-color`) and respect environment variables (e.g., `NO_COLOR`) to disable them.
- **Error Handling:** Use `anyhow` for application-level error handling and `thiserror` for library-level custom errors if necessary. Propagate errors using the `?` operator.

## Web Crawling Specifics & Safety
- **Domain Confinement:** Ensure strict checking so the crawler never leaves the initial top-level domain. Use the `url` crate to extract and compare the host/domain components.
- **Loop Prevention:** Implement a `HashSet` (or concurrent equivalent) to track visited URLs and prevent infinite crawling loops.
- **Concurrency & Rate Limiting:** While `tokio` allows concurrent requests, implement reasonable concurrency limits (e.g., using a semaphore or streams with buffered limits) to avoid inadvertently performing a Denial of Service (DoS) attack on the target server.
- **Blocking Signals:** Identify blocking not just by status codes (like `403` or `429`), but potentially by looking for common WAF/CAPTCHA signatures in the response body if necessary.

## AI / Gemini Model Directives
- **Think-Plan-Act Protocol:** Always formulate a detailed implementation plan before writing or modifying code. Break tasks down into atomic, verifiable steps.
- **No Stubs or Placeholders:** Output complete, functional, and production-ready code. Do not use `// TODO` or `unimplemented!()`.
- **Iterative Verification:** Write the code, compile it (`cargo check` or `cargo clippy`), and verify it before moving to the next step.
- **Dependencies:** Always check `Cargo.toml` before adding new dependencies. Keep the dependency tree as lean as possible without sacrificing robustness.
