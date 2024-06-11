use crate::db;
use crate::db::dashboard_trait::DashboardDataTrait;
use crate::db::user_info_trait::UserInfoTrait;
use actix_web::web::Data;
use actix_web::{get, patch, post, put, web::Json, HttpResponse, Responder};
use actix_web::http::StatusCode;

use crate::db::database::Database;
use crate::user_profile::UserData;
use crate::aws::{AWSImageUploadData, UploadForm};
use crate::dashabord::{MenuData, DashboardData, AppConfig};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use crate::twilio::{OTPData, TwilioService, VerifyOTPData};
use crate::api::{APIErrorStruct, APIResponse};
use validator::Validate;

//TODO: Add delete menu API
//TODO: Add validation for API
//TODO: Add JWT authentication for API

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello Rustacean 🦀")
}

#[post("/addUser")]
pub async fn add_user(
    db: Data<Database>,
    body: Json<UserData>,
) -> HttpResponse {

    let is_valid: Result<(), validator::ValidationErrors> = body.validate();
    match is_valid {
        Ok(_) => {
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_user = UserData::update_uid(&body, String::from(new_uuid));

            let created_user = Database::add_user(&db, new_user).await;

            match created_user {
                Ok(created) => {
                    let response: APIResponse<UserData> = APIResponse::success(true,
                        created, StatusCode::OK.as_u16());
                    HttpResponse::Ok().json(response)
                },
                Err(e) => {
                    let response : APIResponse<UserData> = APIResponse::failure(false,
                        APIErrorStruct{
                                message: e.to_string(),
                                error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                        }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
                    HttpResponse::InternalServerError().json(response)
                },
            }
        }
        Err(e) => {
            info!("Partial info {}", e);
            let response: APIResponse<UserData> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::PARTIAL_CONTENT.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/getUsers")]
pub async fn get_users(db: Data<Database>) -> HttpResponse {
    let users = Database::get_users(&db).await;

    match users {
        Ok(all_users) => {
            let response: APIResponse<Vec<UserData>> = APIResponse::success(true,
                Some(all_users), StatusCode::OK.as_u16());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response : APIResponse<UserData> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[post("/verifyPhoneNumber")]
pub async fn send_otp(new_data: Json<OTPData>) -> HttpResponse {
    let data = OTPData {
        phone_number: new_data.phone_number.clone(),
    };

    let otp_details = TwilioService::send_otp(&data.phone_number).await;

    match otp_details {
        Ok(otp) => {
            let response: APIResponse<String> = APIResponse::success(true,
                Some(otp.sid), StatusCode::OK.as_u16());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response : APIResponse<String> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        },
    }
}

#[post("/verifyOTP")]
pub async fn verify_otp(new_data: Json<VerifyOTPData>) -> HttpResponse {
    let data = VerifyOTPData {
        user: new_data.user.clone(),
        code: new_data.code.clone(),
    };

    let otp_details = TwilioService::verify_otp(&data.user.phone_number, &data.code).await;

    match otp_details {
        Ok(_) => {
            let response: APIResponse<String> = APIResponse::success(true,
                Some("OTP verified successfully".to_string()), StatusCode::OK.as_u16());
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response : APIResponse<String> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        },
    }
}

#[post("/uploadToS3")]
async fn upload_to_s3(
    s3_client: Data<AWSImageUploadData>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> HttpResponse {
    let namespace = form.namespace.into_inner();
    let files = form.files;

    log::info!("namespace = {namespace:?}");
    log::info!("tmp_files = {files:?}");

    // make key prefix (make sure it ends with a forward slash)
    let s3_key_prefix = format!("panda-bucket/{namespace}/");

    // upload temp files to s3 and then remove them
    //let uploaded_files = s3_client.upload_files(files, &s3_key_prefix).await?;
    let uploaded_files = s3_client.upload_files(files, &s3_key_prefix).await;

    let response: APIResponse<String> = APIResponse::success(true,
        Some("This message has be changed ".to_string()), StatusCode::OK.as_u16());
    HttpResponse::Ok().json(response)
}

#[post("/addConfiguration")]
async fn add_app_configuration(
    db: Data<Database>,
    app_config: Json<AppConfig>,
) -> HttpResponse {
    let created_user = Database::aadd_app_configuration(&db, app_config.clone()).await;

    match created_user {
        Ok(created) => {
            let response: APIResponse<AppConfig> = APIResponse::success(true,
                created, StatusCode::OK.as_u16());
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response : APIResponse<AppConfig> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        },
    }
}

#[post("/addNewMenu")]
async fn add_new_menu(
    db: Data<Database>,
    s3_client: Data<AWSImageUploadData>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> HttpResponse {

    let namespace = form.namespace.into_inner();
    let files = form.files;
    let menu_data = form.menu_data;

    log::info!("namespace = {namespace:?}");
    log::info!("tmp_files = {files:?}");
    log::info!("menu_data = {menu_data:?}");

    // make key prefix (make sure it ends with a forward slash)
    let s3_key_prefix = format!("panda-bucket/{namespace}/");

    let uploaded_files = s3_client.upload_files(files, &s3_key_prefix).await;

    match &uploaded_files {
        Ok(files) => {
            // Nothing to do here
            log::info!("aws file result = {files:?}");
        },
        Err(e) => {
            let response : APIResponse<MenuData> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            return HttpResponse::InternalServerError().json(response)
        }
    }

    let mut buffer = uuid::Uuid::encode_buffer();
    let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

    //let new_user = MenuData::update_uid(&menu_data, String::from(new_uuid));
    let new_user = MenuData::update_menu_data(&menu_data, String::from(new_uuid), uploaded_files.unwrap(), namespace);

    let created_user = Database::add_new_menu(&db, new_user.clone()).await;

    match created_user {
        Ok(created) => {
            let response: APIResponse<MenuData> = APIResponse::success(true,
                created, StatusCode::OK.as_u16());
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response : APIResponse<MenuData> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        },
    }
}

#[get("/getDashboardData")]
async fn get_dashaboard_data(db: Data<Database>) -> HttpResponse{
    let menu_data = Database::get_all_menu(&db).await;

    match menu_data {
        Ok(all_menu_data) => {

            let general_config_data: Result<Vec<AppConfig>, surrealdb::Error> = Database::get_general_config(&db).await;

            let dashaboard_data = DashboardData::create_dashaboard_data(all_menu_data,
                general_config_data.unwrap());

            let response: APIResponse<DashboardData> = APIResponse::success(true,
                Some(dashaboard_data), StatusCode::OK.as_u16());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response : APIResponse<DashboardData> = APIResponse::failure(false,
                APIErrorStruct{
                        message: e.to_string(),
                        error: StatusCode::INTERNAL_SERVER_ERROR.as_u16()
                }, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
            HttpResponse::InternalServerError().json(response)
        }
    }
}