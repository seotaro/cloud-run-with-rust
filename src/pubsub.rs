//! Google Cloud PubSub Message declaration

use base64;
use base64::{engine::general_purpose, Engine as _};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json;

pub fn parse_data<T: DeserializeOwned>(src: &str) -> Result<T, String> {
    let dest_bytes = general_purpose::STANDARD
        .decode(src)
        .map_err(|e| e.to_string())?;

    let dest_str = String::from_utf8(dest_bytes).map_err(|e| e.to_string())?;

    serde_json::from_str(&dest_str).map_err(|e| e.to_string())
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Attributes {}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub attributes: Attributes,
    pub data: String,
    pub messageId: String,
    pub message_id: String,
    pub publishTime: String,
    pub publish_time: String,
}

#[derive(Deserialize, Debug)]
pub struct DataJsonApiV1 {
    pub kind: String,
    pub id: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
    pub name: String,
    pub bucket: String,
    pub generation: String,
    pub metageneration: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    pub updated: String,
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    #[serde(rename = "timeStorageClassUpdated")]
    pub time_storage_class_updated: String,
    pub size: String,
    #[serde(rename = "md5Hash")]
    pub md5_hash: String,
    #[serde(rename = "mediaLink")]
    pub media_link: String,
    pub crc32c: String,
    pub etag: String,
}
