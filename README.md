## Introduction

This crate provides a simple interface for fetching oEmbed data from known providers based on the [oembed](https://oembed.com/) specification.

## Usage

```rust
use oembed_rs::{find_provider, fetch, ConsumerRequest};

async fn example() {
    let url = "https://twitter.com/user/status/1000000000000000000";
    let (_, endpoint) = find_provider(url).expect("unknown provider");

    let response = fetch(
       &endpoint.url,
       ConsumerRequest {
           url,
           max_width: Some(1000),
           max_height: Some(500),
           ..ConsumerRequest::default()
       },
    )
    .await
    .expect("failed to fetch oembed data");
}
```

## Roadmap

- [ ] Add support for custom providers
- [ ] Return known errors from the specification properly
- [ ] Support the discovery flow
