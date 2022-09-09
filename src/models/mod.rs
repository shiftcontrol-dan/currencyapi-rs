/// Response of the currencyapi
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct DetailsResponse {
    /// Data source
    pub data: String,
    /// Request status
    pub meta: String,
}