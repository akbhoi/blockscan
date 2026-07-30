use clap::Parser;
use url::Url;

fn parse_url(s: &str) -> Result<Url, url::ParseError> {
    if s.starts_with("http://") || s.starts_with("https://") {
        Url::parse(s)
    } else {
        Url::parse(&format!("https://{}", s))
    }
}

const LONG_ABOUT: &str = r#"
BlockScan is a lightning-fast, concurrent CLI security tool built in Rust to detect whether target websites block automated bot crawling (e.g., via Cloudflare, HTTP 403 Forbidden, 429 Rate Limiting, or CAPTCHA walls).

If the website permits crawling, BlockScan recursively extracts and maps in-domain links up to the configured depth limit while strictly enforcing top-level domain confinement.

OUTPUT STREAMS & PIPING (clig.dev compliance):
  - Primary Data Results are routed to STDOUT.
  - Progress Spinners and Diagnostics are routed to STDERR.
  - When outputting --json or --plain, ANSI styling and progress spinners are automatically suppressed to ensure machine-readability for bash scripts and jq pipelines.

EXAMPLES:
  # Basic scan against a domain (auto-prepends https://):
  blockscan google.com

  # Advanced scan with custom depth, concurrency limit, and spoofed User-Agent:
  blockscan --depth 3 --concurrency 50 --user-agent "MyBot/1.0" https://example.com

  # Pipe machine-readable JSON output into jq:
  blockscan example.com --json | jq '.[] | select(.status == "Allowed")'

  # Enable verbose diagnostic logging to STDERR:
  blockscan example.com -v
"#;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "A lightning-fast CLI tool to check bot-blocking mitigations and discover crawlable in-domain links.",
    long_about = LONG_ABOUT
)]
pub struct Args {
    /// Target URL to scan (e.g. example.com or https://example.com). Auto-prepends https:// if scheme is omitted.
    #[arg(required = true, value_parser = parse_url)]
    pub url: Url,

    /// Maximum crawl depth limit (0 for unlimited depth).
    #[arg(short, long, default_value_t = 2)]
    pub depth: usize,

    /// Maximum concurrent HTTP requests limit (bounded by Tokio semaphore).
    #[arg(short, long, default_value_t = 10)]
    pub concurrency: usize,

    /// Custom HTTP User-Agent header string sent during crawling requests.
    #[arg(short, long, default_value = "blockscan/1.0.0 (Bot)")]
    pub user_agent: String,

    /// Enable verbose diagnostic logging (outputs real-time HTTP requests, response statuses, and extracted link counts to STDERR).
    #[arg(short, long)]
    pub verbose: bool,

    /// Output crawl results in structured, pretty-printed JSON format to STDOUT.
    #[arg(short, long)]
    pub json: bool,

    /// Output crawl results in raw, tab-separated plain text format (depth\turl\tstatus) to STDOUT.
    #[arg(long, conflicts_with = "json")]
    pub plain: bool,

    /// Disable colored ANSI terminal output (also respects NO_COLOR environment variable).
    #[arg(long)]
    pub no_color: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_parse_url_auto_prepends_https() {
        let parsed = parse_url("google.com").unwrap();
        assert_eq!(parsed.as_str(), "https://google.com/");

        let parsed_http = parse_url("http://example.com").unwrap();
        assert_eq!(parsed_http.as_str(), "http://example.com/");
    }

    #[test]
    fn test_cli_help_output_includes_clig_sections() {
        let help_text = Args::command().render_long_help().to_string();
        assert!(help_text.contains("OUTPUT STREAMS & PIPING"));
        assert!(help_text.contains("EXAMPLES"));
        assert!(help_text.contains("--json"));
        assert!(help_text.contains("--plain"));
        assert!(help_text.contains("--verbose"));
    }

    #[test]
    fn test_cli_verbose_flag_parsing() {
        let args = Args::parse_from(["blockscan", "example.com", "-v"]);
        assert!(args.verbose);

        let args_long = Args::parse_from(["blockscan", "example.com", "--verbose"]);
        assert!(args_long.verbose);
    }
}
