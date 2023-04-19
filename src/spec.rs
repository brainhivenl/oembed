use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Provider {
    pub provider_name: String,
    pub provider_url: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Deserialize)]
pub struct Endpoint {
    #[serde(default)]
    pub schemes: Vec<String>,
    pub url: String,
    #[serde(default)]
    pub discovery: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Type {
    #[serde(rename = "photo")]
    Photo(Photo),
    #[serde(rename = "video")]
    Video(Video),
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "rich")]
    Rich(Rich),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Video {
    pub html: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Photo {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rich {
    pub html: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(flatten)]
    pub oembed_type: Type,
    pub version: String,
    pub title: String,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub cache_age: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<i32>,
    pub thumbnail_height: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photo() {
        let input = r#"{
            "type": "photo",
            "version": "1.0",
            "title": "photo",
            "width": 100,
            "height": 50,
            "url": "https://example.com/photo.jpg"
        }"#;
        let response: Response = serde_json::from_str(input).unwrap();

        assert_eq!(response.title, "photo");
        assert_eq!(
            response.oembed_type,
            Type::Photo(Photo {
                url: "https://example.com/photo.jpg".to_string(),
                width: 100,
                height: 50
            })
        )
    }
}
