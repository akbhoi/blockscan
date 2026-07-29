mod checker;
mod cli;
mod crawler;
mod parser;

use checker::Checker;
use clap::Parser;
use cli::Args;
use crawler::Crawler;
use indicatif::{ProgressBar, ProgressStyle};
use std::process;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let checker = match Checker::new(&args.user_agent) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to initialize HTTP client: {}", e);
            process::exit(1);
        }
    };

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Crawling {}...", args.url));
    spinner.enable_steady_tick(Duration::from_millis(100));

    let crawler = Crawler::new(checker, args.depth, args.concurrency);
    
    let completed_count = crawler.completed_count.clone();
    let url_clone = args.url.clone();
    let spinner_clone = spinner.clone();
    
    tokio::spawn(async move {
        loop {
            let count = completed_count.load(std::sync::atomic::Ordering::Relaxed);
            spinner_clone.set_message(format!("Crawling {}... ({} checked)", url_clone, count));
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    let results = crawler.run(args.url.clone()).await;

    spinner.finish_and_clear();

    if args.no_color {
        colored::control::set_override(false);
    }

    if args.json {
        match serde_json::to_string_pretty(&results) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Failed to serialize results to JSON: {}", e);
                process::exit(1);
            }
        }
    } else if args.plain {
        for res in &results {
            let status_str = match &res.status {
                checker::Status::Allowed => "Allowed".to_string(),
                checker::Status::Blocked(r) => format!("Blocked ({})", r),
                checker::Status::Error(e) => format!("Error ({})", e),
            };
            println!("{}\t{}\t{}", res.depth, res.url, status_str);
        }
    } else {
        use colored::Colorize;
        use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
        use comfy_table::presets::UTF8_FULL;

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Depth").add_attribute(Attribute::Bold),
                Cell::new("URL").add_attribute(Attribute::Bold),
                Cell::new("Status").add_attribute(Attribute::Bold),
            ]);

        for res in &results {
            let (status_text, color) = match &res.status {
                checker::Status::Allowed => ("✅ Allowed".to_string(), Color::Green),
                checker::Status::Blocked(r) => (format!("🚫 Blocked ({})", r), Color::Red),
                checker::Status::Error(e) => (format!("❌ Error ({})", e), Color::Yellow),
            };

            let mut status_cell = Cell::new(status_text);
            if !args.no_color {
                status_cell = status_cell.fg(color);
            }

            table.add_row(vec![
                Cell::new(res.depth.to_string()),
                Cell::new(&res.url),
                status_cell,
            ]);
        }

        eprintln!("🔍 {} {}", "Crawl Results for".bold(), args.url.to_string().cyan());
        println!("{table}");
        eprintln!("📊 {}: {}", "Total URLs checked".bold(), results.len().to_string().cyan());
    }
}
