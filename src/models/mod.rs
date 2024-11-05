use std::collections::HashMap;
use serde_json::Value;

/// Response of the currencyapi
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct DetailsResponse {
    /// Data source
    pub data: HashMap<String, Value>,
    /// Request status
    pub meta: Option<HashMap<String, Value>>,
}