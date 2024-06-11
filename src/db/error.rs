use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum UserInfoError {
    NoPhoneNumberFound,
    UserInfoCreationError,
    UserAlreadyExissts,
    PartialUserInfo,
}

impl ResponseError for UserInfoError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserInfoError::NoPhoneNumberFound => StatusCode::NOT_FOUND,
            UserInfoError::UserInfoCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            UserInfoError::UserAlreadyExissts => StatusCode::IM_USED,
            UserInfoError::PartialUserInfo => StatusCode::PARTIAL_CONTENT,
        }
    }
}
