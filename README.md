<p align="center">
<img src="https://app.currencyapi.com/img/logo/currencyapi.png" width="300"/>
</p>

# currencyapi-rs: Rust Currency Converter

This package is a Rust wrapper for [currencyapi.com] that aims to make the usage of the API as easy as possible in your project.


## Installation

This crate is under development. Especially the response parsing needs some more testing. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml
[dependencies]
currencyapi = "0.1.0"
```

## Requirements

1. API Key for [currencyapi.com](https://currencyapi.com/)
2. Async runtime like [tokio](https://crates.io/crates/tokio)

## Quickstart

```rust
use currencyapi::Currencyapi;
use currencyapi::models;

async fn request_latest() -> Result<models::DetailsResponse, currencyapi::Error> {
    let c_api = Currencyapi::new("<your-api-key>")?;
    let details = c_api.latest("id-of-a-fuel-station").await?;
     Ok(details)
}
```

Endpoints accessible with a free account:
- `status`
- `currencies`
- `latest`
- `historical`

These advanced endpoints currently require a paid subscription:
- `convert`
- `range`

Find out more about our endpoints, parameters and response data structure in the [docs]

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.

[docs]: https://currencyapi.com/docs
[currencyapi.com]: https://currencyapi.com