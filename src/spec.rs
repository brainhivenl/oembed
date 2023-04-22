use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An oEmbed provider
/// 
/// See the [oembed spec](https://oembed.com/#section7.1) for more information
#[derive(Debug, Deserialize)]
pub struct Provider {
    pub provider_name: String,
    pub provider_url: String,
    pub endpoints: Vec<Endpoint>,
}

/// An oEmbed provider endpoint
#[derive(Debug, Deserialize)]
pub struct Endpoint {
    #[serde(default)]
    pub schemes: Vec<String>,
    pub url: String,
    #[serde(default)]
    pub discovery: bool,
}

/// Represents one of the oEmbed data types
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Type {
    /// Photo type
    /// 
    /// See section 2.3.4.1. of the [oembed spec](https://oembed.com) for more information
    #[serde(rename = "photo")]
    Photo(Photo),
    /// Video type
    /// 
    /// See section 2.3.4.2. of the [oembed spec](https://oembed.com) for more information
    #[serde(rename = "video")]
    Video(Video),
    /// Link type
    /// 
    /// See section 2.3.4.3. of the [oembed spec](https://oembed.com) for more information
    #[serde(rename = "link")]
    Link,
    /// Rich type
    /// 
    /// See section 2.3.4.4. of the [oembed spec](https://oembed.com) for more information
    #[serde(rename = "rich")]
    Rich(Rich),
}

/// Video type
/// 
/// See section 2.3.4.2. of the [oembed spec](https://oembed.com) for more information
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Video {
    pub html: String,
    pub width: i32,
    pub height: i32,
}

/// Photo type
/// 
/// See section 2.3.4.1. of the [oembed spec](https://oembed.com) for more information
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Photo {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

/// Rich type
/// 
/// See section 2.3.4.4. of the [oembed spec](https://oembed.com) for more information
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rich {
    pub html: String,
    pub width: i32,
    pub height: Option<i32>,
}

/// oEmbed response
/// 
/// See the [oembed spec](https://oembed.com/#section2.3) for more information
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(flatten)]
    pub oembed_type: Type,
    pub version: String,
    pub title: Option<String>,
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

        assert_eq!(response.title, Some("photo".to_string()));
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
