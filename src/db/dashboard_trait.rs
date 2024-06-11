use crate::dashabord::{AppConfig, MenuData};
use crate::db::database::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait DashboardDataTrait {
    async fn add_new_menu(
        db: &Data<Database>,
        new_menu: MenuData,
    ) -> Result<Option<MenuData>, surrealdb::Error>;
    async fn get_all_menu(db: &Data<Database>) -> Result<Vec<MenuData>, surrealdb::Error>;
    async fn aadd_app_configuration(
        db: &Data<Database>,
        app_config: AppConfig,
    ) -> Result<Option<AppConfig>, surrealdb::Error>;
    async fn get_general_config(db: &Data<Database>) -> Result<Vec<AppConfig>, surrealdb::Error>;
}

#[async_trait]
impl DashboardDataTrait for Database {
    async fn add_new_menu(
        db: &Data<Database>,
        new_menu: MenuData,
    ) -> Result<Option<MenuData>, surrealdb::Error> {
        let created_menu: Result<Option<MenuData>, Error> = db
            .client
            .create(("menu", new_menu.title.clone()))
            .content(new_menu)
            .await;

        match created_menu {
            Ok(created) => Ok(created),
            Err(e) => {
                error!("Error in creating a new menu {}", e);
                Err(e)
            }
        }
    }

    async fn get_all_menu(db: &Data<Database>) -> Result<Vec<MenuData>, surrealdb::Error> {
        let result = db.client.select("menu").await;
        match result {
            Ok(all_menus) => Ok(all_menus),
            Err(e) => Err(e),
        }
    }

    async fn aadd_app_configuration(
        db: &Data<Database>,
        app_config: AppConfig,
    ) -> Result<Option<AppConfig>, surrealdb::Error> {
        let new_config: Result<Option<AppConfig>, Error> = db
            .client
            .create(("appconfig", app_config.version.clone()))
            .content(app_config)
            .await;

        match new_config {
            Ok(created) => Ok(created),
            Err(e) => {
                error!("Error in creating a new app config {}", e);
                Err(e)
            }
        }
    }

    async fn get_general_config(db: &Data<Database>) -> Result<Vec<AppConfig>, surrealdb::Error> {
        let result = db.client.select("appconfig").await;
        match result {
            Ok(all_menus) => Ok(all_menus),
            Err(e) => Err(e),
        }
    }
}
