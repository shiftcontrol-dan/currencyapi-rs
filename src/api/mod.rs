//! Module that contains the main [Currencyapi] struct

use std::sync::Arc;
use reqwest::Client;
use crate::error::CurrencyapiError;
use crate::{error, models, utils};
use crate::utils::baseline::construct_base_url;

/// Settings struct that contains the api key
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
}

/// The main struct of the crate giving access to the currencyapi.
/// Create a new instance of the struct with your api key as parameter.
#[derive(Debug, Clone)]
pub struct Currencyapi {
    client: Client,
    settings: Arc<Settings>,
}

impl<'a> Currencyapi {
    /// Creates a new instance of the Currencyapi struct by passing your api key as
    /// function parameter.
    pub fn new(api_key: &'a str) -> Result<Self, CurrencyapiError> {
        let settings = std::sync::Arc::new(Settings {
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(None, &settings)?;
        Ok(Self { client, settings })
    }

    /// Fetches the status of the currency API.
    ///
    /// # Returns
    ///
    /// * `Result<models::DetailsResponse, error::CurrencyapiError>` - A result containing either the details response or a currency API error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response cannot be parsed.
    pub async fn status(
        &self,
    ) -> Result<models::DetailsResponse, error::CurrencyapiError> {
        let url = construct_base_url(&self.settings.api_key, Some("status"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::CurrencyapiError::ResponseParsingError { body: res_body })
    }

    /// Fetches the list of available currencies.
    ///
    /// # Returns
    ///
    /// * `Result<models::DetailsResponse, error::CurrencyapiError>` - A result containing either the details response or a currency API error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response cannot be parsed.
    pub async fn currencies(
        &self,
    ) -> Result<models::DetailsResponse, error::CurrencyapiError> {
        let url = construct_base_url(&self.settings.api_key, Some("currencies"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::CurrencyapiError::ResponseParsingError { body: res_body })
    }

    /// Fetches the latest currency data for the specified base currency and target currencies.
    ///
    /// # Arguments
    ///
    /// * `base_currency` - A string slice that holds the base currency code.
    /// * `currencies` - A string slice that holds the target currencies.
    ///
    /// # Returns
    ///
    /// * `Result<models::DetailsResponse, error::CurrencyapiError>` - A result containing either the details response or a currency API error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response cannot be parsed.
    pub async fn latest(
        &self,
        base_currency: &'a str,
        currencies: &'a str,
    ) -> Result<models::DetailsResponse, error::CurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("latest"))?;
        url.query_pairs_mut()
            .append_pair("base_currency", base_currency)
            .append_pair("currencies", currencies);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::CurrencyapiError::ResponseParsingError { body: res_body })
    }

    /// Fetches historical currency data for the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `base_currency` - A string slice that holds the base currency code.
    /// * `date` - A string slice that holds the date for the historical data.
    /// * `currencies` - A string slice that holds the target currencies.
    ///
    /// # Returns
    ///
    /// * `Result<models::DetailsResponse, error::CurrencyapiError>` - A result containing either the details response or a currency API error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response cannot be parsed.
    pub async fn historical(
        &self,
        base_currency: &'a str,
        date: &'a str,
        currencies: &'a str,
    ) -> Result<models::DetailsResponse, error::CurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("historical"))?;
        url.query_pairs_mut()
            .append_pair("base_currency", base_currency)
            .append_pair("date", date)
            .append_pair("currencies", currencies);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::CurrencyapiError::ResponseParsingError { body: res_body })
    }

    /// Converts a value from the base currency to the target currencies for the specified date.
    ///
    /// # Arguments
    ///
    /// * `base_currency` - A string slice that holds the base currency code.
    /// * `date` - A string slice that holds the date for the conversion.
    /// * `value` - An integer that holds the value to be converted.
    /// * `currencies` - A string slice that holds the target currencies.
    ///
    /// # Returns
    ///
    /// * `Result<models::DetailsResponse, error::CurrencyapiError>` - A result containing either the details response or a currency API error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response cannot be parsed.
    pub async fn convert(
        &self,
        base_currency: &'a str,
        date: &'a str,
        value: i8,
        currencies: &'a str,
    ) -> Result<models::DetailsResponse, error::CurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("convert"))?;
        url.query_pairs_mut()
            .append_pair("base_currency", base_currency)
            .append_pair("date", date)
            .append_pair("value", &value.to_string())
            .append_pair("currencies", currencies);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::CurrencyapiError::ResponseParsingError { body: res_body })
    }

    /// Fetches the range of currency data for the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `base_currency` - A string slice that holds the base currency code.
    /// * `datetime_start` - A string slice that holds the start datetime for the range.
    /// * `datetime_end` - A string slice that holds the end datetime for the range.
    /// * `currencies` - A string slice that holds the target currencies.
    /// * `accuracy` - A string slice that holds the accuracy level.
    ///
    /// # Returns
    ///
    /// * `Result<models::DetailsResponse, error::CurrencyapiError>` - A result containing either the details response or a currency API error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response cannot be parsed.
    pub async fn range(
        &self,
        base_currency: &'a str,
        datetime_start: &'a str,
        datetime_end: &'a str,
        currencies: &'a str,
        accuracy: &'a str,
    ) -> Result<models::DetailsResponse, error::CurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("range"))?;
        url.query_pairs_mut()
            .append_pair("base_currency", base_currency)
            .append_pair("datetime_start", datetime_start)
            .append_pair("datetime_end", datetime_end)
            .append_pair("accuracy", accuracy)
            .append_pair("currencies", currencies);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::CurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::CurrencyapiError::ResponseParsingError { body: res_body })
    }
}
