use crate::util::encryption;
use crate::wiring::ServerWiring;
use domain::server_config::ServerConfig;

use domain::sea_orm::entities::prelude::UserEmailPassword;
use domain::sea_orm::entities::user_email_password;

use sea_orm::{EntityTrait, Set};

pub struct UserDao {}

impl UserDao {
    pub async fn insert_super_user(config: &ServerConfig, wiring: &ServerWiring) -> Result<(), ()> {
        let already_exists = UserEmailPassword::find_by_id(1)
            .one(&wiring.db)
            .await
            .unwrap();

        tide::log::info!("starting more weird super user stuff!");

        if already_exists.is_some() {
            tide::log::info!("super user already exists!");
        } else {
            tide::log::info!("super user does not exist!");

            let em = encryption::seal_with_view_key_emoji(
                &config.encryption_key_emoji,
                &config.encryption_view_key_emoji,
                &config.super_user_email.as_bytes(),
            )
            .unwrap();

            let encrypted_email = String::from(em.clone());
            let encoded_hash = String::from(config.super_user_pwhash_emoji.clone());

            let s = user_email_password::ActiveModel {
                email: Set(encrypted_email),
                password: Set(encoded_hash),
                active: Set(true),
                ..Default::default()
            };

            let operation = UserEmailPassword::insert(s).exec(&wiring.db).await;

            if operation.is_ok() {
                let item = operation.ok();
                println!("INSERTED ONE: {:?}", item);
            } else {
                println!(
                    "Failed to insert super user... maybe it already exists?? {:?}",
                    operation.err()
                );
            }
        }

        Ok(())
    }
}
