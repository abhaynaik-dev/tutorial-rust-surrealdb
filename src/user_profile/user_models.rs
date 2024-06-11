use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub phone_number: String,
    pub email: String,
    pub name: String,
    pub home_address: String,
    pub other_address: String,
    pub uuid: String,
}

impl UserData {
    pub fn update_uid(&self, uuid: String)-> Self{
       Self { uuid: (uuid), ..self.clone() }
    }
}
