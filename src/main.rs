mod api;
mod db;
mod twilio;
mod user_profile;
mod aws;
mod dashabord;
mod util;

use dotenv::dotenv;
use std::fs;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use actix_web::web::Data;
use actix_web::{App, HttpServer, middleware::Logger};
use api::party_panda_api::{add_user, get_users, send_otp, test, verify_otp,
    add_new_menu, get_dashaboard_data, add_app_configuration, upload_to_s3};
use db::database::Database;

use crate::aws::AWSImageUploadData;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    //pretty_env_logger::init();

    pretty_env_logger::init_custom_env(pretty_env_logger::env_logger::DEFAULT_FILTER_ENV);

    let db: Database = Database::init().await.expect("Error in connectin db");

    let db_data = Data::new(db);

    fs::create_dir_all("./tmp").unwrap();

    log::info!("configuring S3 client");

    let aws_region = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_region)
        .load()
        .await;

    // create singleton S3 client
    let s3_client = AWSImageUploadData::new(&aws_config);

    log::info!("using AWS region: {}", aws_config.region().unwrap());

    println!("🚀 Server started successfully at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(Data::new(s3_client.clone()))
            .service(test)
            .service(send_otp)
            .service(verify_otp)
            .service(add_user)
            .service(get_users)
            .service(add_new_menu)
            .service(upload_to_s3)
            .service(get_dashaboard_data)
            .service(add_app_configuration)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
