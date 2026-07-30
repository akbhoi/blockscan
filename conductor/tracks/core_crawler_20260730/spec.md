# Track Specification: Core Bot-Blocking Detection & In-Domain Link Spidering CLI Engine

## Overview
Implement the foundational CLI crawler and detection engine for BlockScan in Rust. The engine will inspect target HTTP endpoints to detect bot-blocking defenses and recursively spider in-domain URLs using high-speed async concurrency bounded by Tokio semaphores.

## Functional Requirements
1. **Bot Blocking Detection Engine:**
   - Issue HTTP requests configured with custom `User-Agent` headers.
   - Flag response as `Blocked` if status is 403 Forbidden, 429 Too Many Requests, or if HTML body contains WAF/CAPTCHA challenge signatures (Cloudflare, CAPTCHA, Incapsula).
   - Flag response as `Allowed` if HTTP status is 2xx/3xx and free of blocking challenges.
2. **In-Domain Link Spidering & Extraction:**
   - Parse `Allowed` HTML responses using `scraper` CSS selector (`a[href]`).
   - Use `url` crate to resolve relative URLs and enforce strict exact top-level domain matching.
   - Prevent infinite loops using thread-safe visited tracking (`HashSet` / `Arc<Mutex>`).
3. **Async Concurrency & Limits:**
   - Default depth limit = 2.
   - Default concurrency limit = 10 bounded by `tokio::sync::Semaphore`.
4. **CLI Output & Presentation:**
   - Primary data to `stdout`, diagnostic logs/spinners to `stderr`.
   - Render results in a `comfy-table` with live `indicatif` spinner.

## Non-Functional Requirements
- Strictly follow [clig.dev](https://clig.dev/) guidelines.
- Target >80% unit test coverage for URL normalization and detection modules.

## Out of Scope
- External domain crawling.
- JavaScript execution / headless browser rendering.
