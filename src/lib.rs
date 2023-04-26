//! # Documentation
//!
//! This crate provides a simple interface for fetching oEmbed data from known providers.
//!
//! ## Example
//! ```rust
//! use oembed_rs::{find_provider, fetch, ConsumerRequest};
//!
//! async fn example() {
//!     let url = "https://twitter.com/user/status/1000000000000000000";
//!     let (_, endpoint) = find_provider(url).expect("unknown provider");
//!
//!     let response = fetch(
//!        &endpoint.url,
//!        ConsumerRequest {
//!            url,
//!            max_width: Some(1000),
//!            max_height: Some(500),
//!            ..ConsumerRequest::default()
//!        },
//!     )
//!     .await
//!     .expect("failed to fetch oembed data");
//! }
//! ```

use lazy_static::lazy_static;

mod error;
mod request;
mod spec;

pub use error::Error;
pub use request::{fetch, ConsumerRequest};
pub use spec::*;

lazy_static! {
    static ref PROVIDERS: Vec<Provider> =
        serde_json::from_slice(include_bytes!("../providers.json"))
            .expect("failed to load providers");
}

/// Find the oEmbed provider and endpoint based on the URL
pub fn find_provider(url: &str) -> Option<(&Provider, &Endpoint)> {
    PROVIDERS.iter().find_map(|p| {
        p.endpoints
            .iter()
            .find(|e| e.schemes.iter().any(|s| matches_scheme(s, url)))
            .map(|e| (p, e))
    })
}

/// Checks if the URL matches the scheme
pub fn matches_scheme(mut scheme: &str, mut url: &str) -> bool {
    let Some(prefix) = scheme.find('*') else {
        return false;
    };

    if !url.starts_with(&scheme[..prefix]) {
        return false;
    }

    scheme = &scheme[prefix + 1..];
    url = &url[prefix..];

    while let Some(n) = scheme.find('*') {
        let prefix = &scheme[..n];

        while !url.starts_with(prefix) {
            let Some(idx) = url.find('/') else {
                return false;
            };

            url = &url[idx + 1..];
        }
    }

    scheme.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_provider() {
        let url = "https://twitter.com/user/status/1640004220000000000?s=20";
        let (provider, endpoint) = find_provider(url).unwrap();

        assert_eq!(provider.provider_name, "Twitter");
        assert_eq!(endpoint.url, "https://publish.twitter.com/oembed");
    }

    #[test]
    fn test_youtube_provider() {
        let url = "https://youtu.be/rAn0MId";
        let (provider, endpoint) = find_provider(url).unwrap();

        assert_eq!(provider.provider_name, "YouTube");
        assert_eq!(endpoint.url, "https://www.youtube.com/oembed");
    }

    #[test]
    fn test_invalid() {
        let url = "https://twitter.nl/user/status/1640004220000000000?s=20";
        assert!(find_provider(url).is_none());
    }
}
