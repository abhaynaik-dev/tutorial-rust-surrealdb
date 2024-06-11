use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::aws::{self, UploadedFile};

//https://serde.rs/field-attrs.html
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuData {
    pub title: String,
    pub description: String,
    pub menu_items: MenuItem,
    pub menu_id: String,
    pub review_star: u8,
    #[serde(default)]
    pub aws_data: AWSData
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuItem {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AWSData {
    #[serde(default)]
    pub aws_namespace: String,
    #[serde(default)]
    pub cover_image: UploadedFile,
    #[serde(default)]
    pub sub_image: Vec<UploadedFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub menu_data: Vec<MenuData>,
    pub app_config: Vec<AppConfig>
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub supported_pin_code: Vec<String>,
    pub version: String
}

impl MenuData {
    pub fn update_uid(&self, uuid: String)-> Self{
        Self { menu_id: (uuid), ..self.clone() }
     }

    pub fn update_menu_data(&self, uuid: String, aws_uploaded_files: Vec<UploadedFile>, aws_namespace: String) -> Self {

        let mut cover_image_temp: UploadedFile = UploadedFile {filename: "".to_string(), s3_key: "".to_string(), s3_url: "".to_string() };
        let mut sub_image_temp: Vec<UploadedFile> = vec![];

        for aws_file in aws_uploaded_files.iter() {
            if aws_file.filename == "coverImage" {
                cover_image_temp = aws_file.clone();
            }else {
                sub_image_temp.push(aws_file.clone());
            }
        }

        let aws_data_temp = AWSData{
            aws_namespace : aws_namespace,
            cover_image: cover_image_temp,
            sub_image: sub_image_temp
        };

        Self { menu_id: (uuid), aws_data: (aws_data_temp), ..self.clone() }
    }
}

impl DashboardData {
    pub fn create_dashaboard_data(alll_menu_data: Vec<MenuData>, general_config: Vec<AppConfig>)-> Self {
        Self { menu_data: (alll_menu_data) , app_config: (general_config)}
    }
}