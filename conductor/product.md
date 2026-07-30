# Product Definition: BlockScan

## Vision & Summary
BlockScan is a lightning-fast, concurrent CLI security and web discovery tool built in Rust. It enables security researchers, SEO specialists, and developers to rapidly detect website bot-blocking mitigations (such as 403 Forbidden responses, 429 Rate Limiting, and CAPTCHA challenges) and recursively discover crawlable in-domain URLs.

## Target Audience & Use Cases
- **Security Researchers & Penetration Testers:** Rapidly map target web applications and test bot defense boundaries without full manual proxying.
- **SEO Specialists & Web Scrapers:** Audit domain link structures, check crawl accessibility, and detect anti-scraping walls before launching large jobs.
- **DevOps & Pipeline Automation:** Integrate bot detection and URL discovery checks into CI/CD pipelines via scriptable output flags (`--json`, `--plain`).

## Key Features & Capabilities
1. **Bot Blocking Detection:** Identifies HTTP status flags (403, 429), CAPTCHA walls, and rate limits.
2. **Recursive In-Domain Discovery:** Normalizes domain names (merging www/root domains) and extracts hyperlinks up to configurable depth limits.
3. **High Concurrency Engine:** Powered by Rust `tokio` and `reqwest` async requests with customizable semaphore concurrency limits.
4. **Rich Terminal UI & Scripting Support:** Renders rich terminal tables (`comfy-table`) with live progress spinners (`indicatif`), plus `--json` and `--plain` flags for automated shell pipelines.

## Non-Functional Goals
- **Performance & Safety:** Zero-overhead Rust execution with strict memory safety and clean async execution.
- **Developer Experience:** CLIG-compliant CLI flags and human-friendly colored terminal diagnostics.
