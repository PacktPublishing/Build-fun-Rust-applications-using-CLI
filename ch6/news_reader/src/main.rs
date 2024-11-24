//! # News Reader CLI Application
//!
//! This is the main entry point for the News Reader CLI application. 
//! The application fetches news from a given API source, parses the data, 
//! and displays it in a user-friendly format on the command line.
//!
//! ## Features
//! - Fetch news from configurable sources via asynchronous HTTP requests.
//! - Parse JSON responses into structured data.
//! - Display news with optional limits on the number of articles shown.
//!
//! ## Usage
//! Use the `--source` argument to specify a custom news source URL.
//! Use the `--limit` argument to restrict the number of displayed articles.

mod config;
mod fetcher;
mod parser;
mod display;

use clap::Parser;

/// CLI argument parser for configuring the news reader.
#[derive(Parser)]
struct Args {
    /// The source URL for fetching news.
    #[arg(short, long, default_value = "https://newsapi.org/v2/top-headlines?country=us")]
    source: String,

    /// Limit the number of articles displayed.
    #[arg(short, long, default_value = "10")]
    limit: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match fetcher::fetch_news(&args.source).await {
        Ok(news_data) => {
            let parsed_news = parser::parse_news(&news_data);
            display::display_news(parsed_news, args.limit);
        }
        Err(e) => eprintln!("Error fetching news: {}", e),
    }
}
