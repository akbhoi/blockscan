# Project Requirements: blockscan

## Overview
blockscan is a Command Line Interface (CLI) application built in Rust. Its primary purpose is to check if a target website blocks bot crawling. If the website permits crawling, the application will discover and list other crawlable links within the same domain.

## Core Features

1. **Bot Blocking Detection:**
   - Attempt to access a provided URL using bot-like characteristics (e.g., custom or common bot `User-Agent` headers).
   - Analyze the HTTP response to determine if the bot is blocked (e.g., HTTP 403 Forbidden, 429 Too Many Requests, or specific CAPTCHA challenge pages).

2. **Crawlability Reporting:**
   - **Blocked:** If the website blocks the request, output the URL and the status: `Blocked`.
   - **Allowed:** If the website permits the request, output the URL and the status: `Allowed`.

3. **In-Domain Link Discovery (Spidering):**
   - If the initial URL is allowed, parse the HTML content of the response.
   - Extract all hyperlinks (`<a>` tags with `href` attributes).
   - **Strict Domain Filtering:** Filter the discovered links to ensure they belong exactly to the **same top-level domain** as the original URL. The crawler must *not* stray to external domains.

4. **Recursive Crawling:**
   - Check the newly discovered in-domain links to determine their availability.
   - Maintain a robust state of visited URLs to prevent infinite loops and redundant requests.

5. **Detailed Output:**
   - Generate a detailed, structured output (to `stdout` or a specified file) listing all discovered in-domain links and whether they are available for crawling.

## Technical Specifications
- **Language:** Rust
- **Interface:** Command Line Interface (CLI) strictly adhering to the [clig.dev](https://clig.dev/) guidelines.
  - Separate standard output (for structured data) and standard error (for logs, progress, and messages).
  - Support `--plain` flag for unformatted, script-friendly output.
  - Support `--no-color` flag and respect `NO_COLOR` environment variable to disable colors.
  - Provide both concise and detailed `--help` output.
- **Input:** Target URL(s), optional flags for depth, concurrency, User-Agent, output formats, etc.
- **Output:** Human-readable text or structured format (e.g., JSON) detailing the URL crawlability status.
