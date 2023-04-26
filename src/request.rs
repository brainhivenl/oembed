use std::collections::HashMap;

use lazy_static::lazy_static;
use reqwest::IntoUrl;

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

        Ok(self.0.get(url).send().await?.json().await?)
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
