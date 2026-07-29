use clap::Parser;
use url::Url;

fn parse_url(s: &str) -> Result<Url, url::ParseError> {
    if s.starts_with("http://") || s.starts_with("https://") {
        Url::parse(s)
    } else {
        Url::parse(&format!("https://{}", s))
    }
}

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "A CLI tool to check if a website blocks bot crawling and discover in-domain links."
)]
pub struct Args {
    /// The target URL to scan
    #[arg(required = true, value_parser = parse_url)]
    pub url: Url,

    /// Maximum crawl depth (0 for no limit, defaults to 2)
    #[arg(short, long, default_value_t = 2)]
    pub depth: usize,

    /// Maximum concurrent requests
    #[arg(short, long, default_value_t = 10)]
    pub concurrency: usize,

    /// Custom User-Agent string to use for requests
    #[arg(short, long, default_value = "blockscan/1.0.0 (Bot)")]
    pub user_agent: String,

    /// Output results in JSON format
    #[arg(short, long)]
    pub json: bool,

    /// Output results in plain text format without colors or headers
    #[arg(long, conflicts_with = "json")]
    pub plain: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,
}
