use reqwest::{Client, Error};
use crate::config::get_api_key;

/// Fetches news data from the given URL with an API key.
///
/// # Arguments
/// - `url`: A string slice containing the URL to fetch news from.
///
/// # Returns
/// - `Ok(String)` containing the raw response data on success.
/// - `Err(Error)` if there is an error during the request.
///
/// # Examples
/// ```rust
/// let data = fetcher::fetch_news("https://newsapi.org/v2/top-headlines").await?;
/// println!("{}", data);
/// ```
pub async fn fetch_news(url: &str) -> Result<String, Error> {
    let api_key = get_api_key();
    let client = Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
