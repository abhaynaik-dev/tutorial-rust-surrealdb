use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OTPData {
    pub phone_number: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOTPData {
    pub user: OTPData,
    pub code: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OTPResponse {
    pub sid: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyResponse {
    pub status: String,
}
