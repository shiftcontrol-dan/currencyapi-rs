//! # currencyapi-rs
//!
//! The [Currencyapi][currencyapi] crate provides an easy to use wrapper over the
//! [currencyapi api][currencyapi_api].
//!
//! * Easy to use
//! * Async api calls with [reqwest][reqwest]
//! * Ready deserialized structs of the currencyapi responses
//! * Manages authentication for you, just pass your api token once
//!
//! ## Requirements
//! * Your own [currencyapi api key][currencyapi_api]
//! * Async runtime configured e.g. [tokio][tokio]
//!
//! ## Getting started
//! Install the latest version of this crate by adding the following line to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! currencyapi = { git = "https://github.com/everapihq/currencyapi-rs" }
//! ```
//! Import the crate and create a new instance of the [Currencyapi][currencyapi] struct.
//! ```
//! use currencyapi::Currencyapi;
//! use currencyapi::models;
//!
//! async fn run() -> Result<models::station::DetailsResponse, currencyapi::Error> {
//!   let tanker = Currencyapi::new("your-api-key")?;
//!   let details = tanker.station.fetch_details("id-of-the-fuel-station").await?;
//!   Ok(details)
//! }
//! ```
//!
//!
//! ## Examples
//!
//!
//! ## Optional Features
//!
//!
//! ## Troubleshooting
//! If you get a ResponseParsingError during usage of the crate this is very likely
//! due to an invalid input where the currencyapi api will throw an error or
//! due to some unexpected values that were returned by the api. E.g. sometimes the api
//! will return `false` instead of a number for certain fields or other fields were missing.
//!
//! In this case please check if your input is valid and if so create a bug report on the
//! crate [repository][currencyapi_rs_repo] and provide some information about your input.
//!
//! [currencyapi]: ./api/struct.Currencyapi.html
//! [currencyapi_rs_repo]: https://github.com/everapihq/currencyapi-rs
//! [currencyapi_api]: https://creativecommons.currencyapi.de/
//! [reqwest]: https://crates.io/crates/reqwest
//! [tokio]: https://crates.io/crates/tokio

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate thiserror;

pub mod api;
mod error;
pub mod models;
mod utils;

pub use api::Currencyapi;
pub use error::CurrencyapiError as Error;