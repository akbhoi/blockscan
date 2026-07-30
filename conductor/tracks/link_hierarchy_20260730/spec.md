# Track Specification: Parent-Child Link Tracking & Deterministic Result Sorting

## Overview
Enhance BlockScan's crawler and output formatter to track parent URL relationships for all discovered links and sort crawl results deterministically by Depth ascending, Parent URL, and Target URL.

## Functional Requirements
1. **Parent URL Metadata Tracking:**
   - Update `CrawlResult` struct to include `parent_url: Option<String>` (or `"-"` for initial root URL).
   - Track parent URL during link discovery in `src/crawler.rs`.
2. **Deterministic Output Sorting:**
   - Sort results array deterministically before outputting: Depth ascending (`0`, `1`, `2`...), then `parent_url`, then `url`.
3. **Tabular & Structured Output Integration:**
   - Add a `Parent URL` column to the `comfy-table` layout in `src/main.rs`.
   - Update `--json` serialization to include `parent_url`.
   - Update `--plain` output to include `parent_url` column (`depth\turl\tparent_url\tstatus`).

## Non-Functional Requirements
- Maintain 100% unit test suite pass rate (`cargo test`).
- Ensure `cargo clippy -- -D warnings` and `cargo fmt --check` pass cleanly.
