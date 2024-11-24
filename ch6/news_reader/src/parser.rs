//! # Parser Module
//!
//! This module is responsible for parsing the raw news data into structured formats 
//! that can be easily manipulated and displayed.
//!
//! ## Features
//! - Parses JSON responses into `NewsArticle` structs.
//! - Supports additional formats such as XML if required.
//!
//! ## Structures
//! - `NewsArticle`: Represents a single news article.
//! - `NewsApiResponse`: Represents the entire API response.

use serde::{Deserialize, Serialize};

/// Represents a single news article.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewsArticle {
    /// The title of the article.
    pub title: String,

    /// A brief description of the article.
    pub description: Option<String>,

    /// The URL to the full article.
    pub url: String,
}

/// Represents the structure of the API's response.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewsApiResponse {
    /// A list of articles returned by the API.
    pub articles: Vec<NewsArticle>,
}

/// Parses raw news data into a vector of `NewsArticle` structs.
///
/// # Arguments
/// - `data`: A string slice containing the raw JSON response.
///
/// # Returns
/// A vector of `NewsArticle` objects.
///
/// # Examples
/// ```rust
/// let articles = parser::parse_news(json_data);
/// println!("Parsed {} articles.", articles.len());
/// ```
pub fn parse_news(data: &str) -> Vec<NewsArticle> {
    match serde_json::from_str::<NewsApiResponse>(data) {
        Ok(parsed) => parsed.articles,
        Err(_) => Vec::new(),
    }
}
