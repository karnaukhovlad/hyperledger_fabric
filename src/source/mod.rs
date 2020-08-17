pub mod cli;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Color")]
    color: String,
    #[serde(rename = "Size")]
    size: i32,
    #[serde(rename = "Owner")]
    owner: String,
    #[serde(rename = "AppraisedValue")]
    appraised_value: i32,
    #[serde(rename = "docType")]
    doc_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsList {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Record")]
    record: Asset,
}

impl From<AssetsList> for Asset {
    fn from(old: AssetsList) -> Self {
        old.record
    }
}
