# Product Guidelines: BlockScan

## Visual & UX Principles
- **CLI Guidelines (CLIG) Compliance:** Adhere strictly to command-line interface guidelines for options, flags, and help formatting.
- **Terminal Aesthetics & Feedback:** Use clear ANSI color coding for status indicators (green for allowed/success, red/yellow for blocked/rate-limited, cyan for progress spinners).
- **Graceful Progress Display:** Show non-intrusive live progress spinners (`indicatif`) for long-running concurrent network requests, automatically hiding spinners when output is piped.

## Output & Interaction Standards
- **Interactive UI Mode:** Render clean tabular layouts via `comfy-table` with intuitive headers and summary counts.
- **Machine-Readable Pipe Compatibility:** Suppress colored ANSI formatting and interactive UI when stdout is not a TTY or when `--json` / `--plain` flags are supplied.
- **Error Handling & Messaging:** Provide clean, actionable error messages on `stderr` without exposing raw stack traces unless debugging flags are set.

## Tone & Communication Voice
- **Professional & Direct:** Use clear, concise, security-tooling terminology. Avoid jargon overload while maintaining technical precision.
