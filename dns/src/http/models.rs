use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub(crate) tld: String,
    pub(crate) ip: String,
    pub(crate) name: String,
    pub(crate) secret_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ResponseDomain {
    pub(crate) tld: String,
    pub(crate) ip: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpdateDomain {
    pub(crate) ip: String,
}

#[derive(Serialize)]
pub(crate) struct Error {
    pub(crate) msg: &'static str,
    pub(crate) error: String,
}

#[derive(Deserialize)]
pub(crate) struct PaginationParams {
    #[serde(alias = "p")]
    pub(crate) page: Option<u32>,
    #[serde(alias = "s")]
    pub(crate) page_size: Option<u32>,
}

#[derive(Serialize)]
pub(crate) struct PaginationResponse {
    pub(crate) domains: Vec<ResponseDomain>,
    pub(crate) page: u32,
    pub(crate) limit: u32,
}
