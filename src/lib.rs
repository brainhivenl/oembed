use lazy_static::lazy_static;
use url::Url;

mod error;
mod request;
mod spec;

pub use error::Error;
pub use request::{fetch, Params};
pub use spec::*;

lazy_static! {
    static ref PROVIDERS: Vec<Provider> =
        serde_json::from_slice(include_bytes!("../providers.json"))
            .expect("failed to load providers");
}

pub fn find_provider(url: &Url) -> Option<(&Provider, &Endpoint)> {
    PROVIDERS.iter().find_map(|p| {
        p.endpoints
            .iter()
            .find(|e| !e.discovery && e.schemes.iter().any(|s| matches_scheme(s, url)))
            .map(|e| (p, e))
    })
}

fn matches_scheme(mut scheme: &str, url: &Url) -> bool {
    let mut url = url.as_str();
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
        let url = Url::parse("https://twitter.com/user/status/1640004220000000000?s=20").unwrap();
        let (provider, _) = find_provider(&url).unwrap();
        assert_eq!(provider.provider_name, "Twitter");
    }

    #[test]
    fn test_invalid() {
        let url = Url::parse("https://twitter.nl/user/status/1640004220000000000?s=20").unwrap();
        assert!(find_provider(&url).is_none());
    }
}
