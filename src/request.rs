use reqwest::IntoUrl;

use crate::{error::Error, spec::Response};

pub struct Params<'a> {
    pub url: &'a str,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
}

pub async fn fetch(endpoint: impl IntoUrl, params: Params<'_>) -> Result<Response, Error> {
    let mut url = endpoint.into_url()?;

    {
        let mut query = url.query_pairs_mut();

        query.append_pair("url", params.url);

        if let Some(max_width) = params.max_width {
            query.append_pair("maxwidth", &max_width.to_string());
        }

        if let Some(max_height) = params.max_height {
            query.append_pair("maxheight", &max_height.to_string());
        }

        query.finish();
    }

    Ok(reqwest::get(url).await?.json().await?)
}
