//! # Display Module
//!
//! This module handles displaying news articles in a user-friendly format 
//! on the command line.
//!
//! ## Features
//! - Outputs article titles and descriptions.
//! - Includes links to the full articles.
//! - Supports limiting the number of displayed articles.

use crate::parser::NewsArticle;

/// Displays a list of news articles in the terminal.
///
/// # Arguments
/// - `articles`: A vector of `NewsArticle` objects to display.
/// - `limit`: The maximum number of articles to display.
///
/// # Examples
/// ```rust
/// display::display_news(articles, 5);
/// ```
pub fn display_news(articles: Vec<NewsArticle>, limit: usize) {
    for (i, article) in articles.into_iter().take(limit).enumerate() {
        println!("\n[{}] {}", i + 1, article.title);
        if let Some(description) = article.description {
            println!("    {}", description);
        }
        println!("    Read more: {}", article.url);
    }
}
