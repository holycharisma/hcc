use crate::util::password::PasswordUtil;
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
            password: Set(PasswordUtil::into_password_hash(&config.super_user_password)),
            active: Set(true),
            ..Default::default()
        };

        let operation = User::insert_many(vec![super_user_model])
            .exec(&wiring.db)
            .await;


            /* 
        println!(
            "password check! {}",
            PasswordUtil::verify_hashed_bytes(
                &config.super_user_password, 
                &PasswordUtil::into_password_hash(&config.super_user_password)
            )
        );

        */
        
        if operation.is_ok() {
            let item = operation.ok();
            println!("INSERTED ONE: {:?}", item);
        } else {
            println!(
                "Failed to insert super user... maybe it already exists?? {:?}",
                operation.err()
            );
        }

        Ok(())
    }
}
