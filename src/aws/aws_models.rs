use actix_multipart::form::{json::Json, tempfile::TempFile, text::Text, MultipartForm};
use serde::{Deserialize, Serialize};

use crate::dashabord::MenuData;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub namespace: Text<String>,

    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,

    #[multipart(rename = "menuData")]
    pub menu_data: Json<MenuData>
}

//#[derive(Debug, Clone, Serialize)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadedFile {
    pub filename: String,
    pub s3_key: String,
    pub s3_url: String,
}

impl UploadedFile {
    /// Construct new uploaded file info container.
    pub fn new(
        filename: impl Into<String>,
        s3_key: impl Into<String>,
        s3_url: impl Into<String>,
    ) -> Self {
        Self {
            filename: filename.into(),
            s3_key: s3_key.into(),
            s3_url: s3_url.into(),
        }
    }
}