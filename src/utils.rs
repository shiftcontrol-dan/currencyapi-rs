pub mod baseline {
    use crate::api;
    use crate::error::CurrencyapiError;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use reqwest::{Client, Url};

    const BASE_URL: &str = "https://api.currencyapi.com/v3/";

    pub fn construct_client(
        user_agent: Option<&str>,
        _: &api::Settings,
    ) -> Result<Client, CurrencyapiError> {
        let mut headers = HeaderMap::new();
        let content_type = HeaderValue::from_str("application/json")?;
        headers.insert(CONTENT_TYPE, content_type);
        let agent = user_agent.map_or_else(
            || format!("{}/{}", "", ""),
            String::from,
        );
        let client = Client::builder()
            .user_agent(agent)
            .default_headers(headers)
            .build()
            .map_err(|err| CurrencyapiError::ClientConstruction { source: err })?;
        Ok(client)
    }

    pub fn construct_base_url(
        with_path: Option<&str>,
    ) -> Result<Url, CurrencyapiError> {
        let mut url = Url::parse(BASE_URL).map_err(|_| CurrencyapiError::UrlConstruction)?;
        if let Some(path) = with_path {
            let trimmed_path = path.trim_start_matches('/');
            let new_path = format!("{}/{}", url.path().trim_end_matches('/'), trimmed_path);
            url.set_path(&new_path);
        }
        Ok(url)
    }
}

#[cfg(test)]
mod baseline_test {
    use super::baseline::*;

    #[test]
    fn should_create_base_url_with_api_key() {
        let base_url = construct_base_url(None).unwrap();
        assert_eq!(base_url.path(), "/v3/");
    }

    #[test]
    fn should_create_base_url_with_api_key_and_path() {
        let base_url = construct_base_url(Some("/test/path")).unwrap();
        assert_eq!(base_url.path(), "/v3/test/path");
    }

}
