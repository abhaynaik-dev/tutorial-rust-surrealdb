use crate::{db::database::Database};
use crate::user_profile::UserData;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait UserInfoTrait {
    async fn add_user(db: &Data<Database>, new_user: UserData) -> Result<Option<UserData>, surrealdb::Error>;
    async fn get_users(db: &Data<Database>) -> Result<Vec<UserData>, surrealdb::Error>;
}

#[async_trait]
impl UserInfoTrait for Database {
    async fn add_user(db: &Data<Database>, new_user: UserData) -> Result<Option<UserData>, surrealdb::Error> {
        let created_user: Result<Option<UserData>, Error> = db
            .client
            .create(("user", new_user.phone_number.clone()))
            .content(new_user)
            .await;

        match created_user {
            Ok(created) => Ok(created),
            Err(e) => {
                 error!("Error in creating a new user {}", e);
                Err(e)
            },
        }
    }

    async fn get_users(db: &Data<Database>) -> Result<Vec<UserData>, surrealdb::Error>{
        let result = db.client.select("user").await;
        match result {
            Ok(all_users) => Ok(all_users),
            Err(e) => Err(e),
        }
    }
}
