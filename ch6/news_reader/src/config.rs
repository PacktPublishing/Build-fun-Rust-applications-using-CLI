//! # Configuration Module
//!
//! This module handles configuration settings for the News Reader application, 
//! including managing API keys and environment variables.
//!
//! ## Features
//! - Retrieves the `NEWS_API_KEY` environment variable, which is required for API authentication.
//! - Provides utility functions for managing other configurations as needed.

use std::env;

/// Retrieves the API key from the environment.
///
/// # Panics
/// This function will panic if the `NEWS_API_KEY` environment variable is not set.
///
/// # Examples
/// ```
/// let api_key = config::get_api_key();
/// println!("API Key: {}", api_key);
/// ```
pub fn get_api_key() -> String {
    env::var("NEWS_API_KEY").expect("Please set the NEWS_API_KEY environment variable")
}
