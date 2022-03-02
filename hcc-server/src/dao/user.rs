use crate::util::password::password_bytes;
use crate::wiring::ServerWiring;
use domain::server_config::ServerConfig;

use domain::sea_orm::user::prelude::*;
use sea_orm::{EntityTrait, Set};

pub struct UserDao {}

impl UserDao {
    pub async fn insert_super_user(config: &ServerConfig, wiring: &ServerWiring) -> Result<(), ()> {
        let super_user_model = UserActiveModel {
            created_at: Set(chrono::offset::Utc::now()),
            email: Set(String::from(&config.super_user_email)),
            username: Set(String::from(&config.super_user_email)),
            password: Set(password_bytes(&config.super_user_password)),
            active: Set(true),
            ..Default::default()
        };

        let operation = User::insert_many(vec![super_user_model])
            .exec(&wiring.db)
            .await;

        if operation.is_ok() {
            println!("INSERTED ONE: {:?}", operation.ok());
        } else {
            println!(
                "Failed to insert super user... maybe it already exists?? {:?}",
                operation.err()
            );
        }

        Ok(())
    }
}
