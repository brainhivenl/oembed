use std::collections::HashMap;

use lazy_static::lazy_static;
use reqwest::{header, IntoUrl};

use crate::{error::Error, spec::EmbedResponse};

lazy_static! {
    static ref DEFAULT_CLIENT: reqwest::Client = reqwest::Client::new();
}

/// Request for fetching oEmbed data
///
/// See the [oembed specification](https://oembed.com/#section2.2) for more information
#[derive(Default)]
pub struct ConsumerRequest<'a> {
    pub url: &'a str,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub params: Option<HashMap<&'a str, &'a str>>,
}

/// oEmbed client
#[derive(Clone)]
pub struct Client(reqwest::Client);

impl Client {
    pub fn new(client: reqwest::Client) -> Self {
        Self(client)
    }

    /// Fetch oEmbed data from the endpoint of a provider
    pub async fn fetch(
        &self,
        endpoint: impl IntoUrl,
        request: ConsumerRequest<'_>,
    ) -> Result<EmbedResponse, Error> {
        let mut url = endpoint.into_url()?;

        {
            let mut query = url.query_pairs_mut();

            query.append_pair("url", request.url);

            if let Some(max_width) = request.max_width {
                query.append_pair("maxwidth", &max_width.to_string());
            }

            if let Some(max_height) = request.max_height {
                query.append_pair("maxheight", &max_height.to_string());
            }

            if let Some(params) = request.params {
                for (key, value) in params {
                    query.append_pair(key, value);
                }
            }

            query.finish();
        }

        Ok(self
            .0
            .get(url)
            .header(header::USER_AGENT, "crates/oembed-rs")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map(|mut response: EmbedResponse| {
                // Remove the `type` field from the extra fields as we use #[serde(flatten)] twice
                response.extra.remove("type");
                response
            })?)
    }
}

/// Fetch oEmbed data from the endpoint of a provider
pub async fn fetch(
    endpoint: impl IntoUrl,
    request: ConsumerRequest<'_>,
) -> Result<EmbedResponse, Error> {
    Client::new(DEFAULT_CLIENT.clone())
        .fetch(endpoint, request)
        .await
}

#[cfg(test)]
mod tests {
    use mockito::Server;

    use super::*;

    #[tokio::test]
    async fn test_fetch_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/?url=https%3A%2F%2Fexample.com")
            .with_status(200)
            .with_body(r#"{"version": "1.0", "type": "link"}"#)
            .with_header("content-type", "application/json")
            .create_async()
            .await;

        let result = fetch(
            server.url(),
            ConsumerRequest {
                url: "https://example.com",
                ..ConsumerRequest::default()
            },
        )
        .await;
        assert_eq!(
            result.ok(),
            Some(EmbedResponse {
                oembed_type: crate::EmbedType::Link,
                version: "1.0".to_string(),
                title: None,
                author_name: None,
                author_url: None,
                provider_name: None,
                provider_url: None,
                cache_age: None,
                thumbnail_url: None,
                thumbnail_width: None,
                thumbnail_height: None,
                extra: HashMap::default(),
            })
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fetch_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/?url=https%3A%2F%2Fexample.com")
            .with_status(404)
            .create_async()
            .await;

        let result = fetch(
            server.url(),
            ConsumerRequest {
                url: "https://example.com",
                ..ConsumerRequest::default()
            },
        )
        .await;

        if let Err(Error::Reqwest(err)) = result {
            assert_eq!(err.status(), Some(reqwest::StatusCode::NOT_FOUND))
        } else {
            panic!("unexpected result: {:?}", result);
        }

        mock.assert_async().await;
    }
}
