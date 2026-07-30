# Technology Stack: BlockScan

## Core Language & Runtime
- **Language:** Rust (Edition 2024)
- **Async Engine:** `tokio` (v1.53, full features)
- **Error Handling:** `anyhow` for application context, `thiserror` for library-level error definitions

## Web & Network Processing
- **HTTP Client:** `reqwest` (v0.13, json features, custom User-Agent, timeouts, redirect handling)
- **HTML Extraction:** `scraper` (v0.27, CSS selector parsing for `<a href>`)
- **URL Parsing & Validation:** `url` (v2.5, domain extraction, relative path resolution, top-level domain confinement)

## CLI Architecture & UX
- **Command-Line Parser:** `clap` (v4.6, `derive` feature)
- **Terminal Data Rendering:** `comfy-table` (v7.2)
- **Live Progress & Spinners:** `indicatif` (v0.18)
- **Terminal Color Styling:** `colored` (v3.1)

## Data Pipeline & Serialization
- **Data Schemas:** `serde` (v1.0, `derive` feature)
- **JSON Formatting:** `serde_json` (v1.0)
- **Concurrency Utilities:** `futures` (v0.3), `tokio::sync::Semaphore`

## Development & Quality Assurance
- **Build System:** `cargo`
- **Linting & Formatting:** `cargo clippy`, `cargo fmt`
- **Test Framework:** `cargo test`
