use actix_web::{web::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<APIErrorStruct>,
    pub status: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorStruct {
    pub message: String,
    pub error: u16,
}

impl <T> APIResponse <T> {
    pub fn success(is_success: bool, data: Option<T>, status: u16) ->Self{
        Self { success: (is_success), data: (data), error: (Option::None), status: (status) }
    }

    pub fn failure(is_success: bool, error: APIErrorStruct, status: u16) ->Self{
        Self { success: (is_success), data: (Option::None), error: (Option::Some(error)), status: (status) }
    }
}