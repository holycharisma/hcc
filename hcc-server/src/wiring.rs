use std::sync::Arc;
use tide::Result;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::domain::server_config::ServerConfig;
use crate::util::jwt::{JsonWebTokenSecrets, JsonWebTokenUtil};

#[derive(Clone)]
pub struct ServerWiring {
    pub services: ServiceWiring,
    pub config: ServerConfig,
}

impl ServerWiring {
    pub fn init_server_config() -> ServerConfig {
        dotenv().ok();
        ServerConfig {
            domain: env::var("HCC_ORIGIN_DOMAIN")
                .expect("Invalid configuration: HCC_ORIGIN_DOMAIN required"),
            session_cookie_name: env::var("HCC_SESSION_COOKIE_NAME")
                .expect("Invalid configuration: HCC_SESSION_COOKIE_NAME required"),
            session_ttl_hours: env::var("HCC_SESSION_TTL_HOURS")
                .expect("Invalid configuration: HCC_SESSION_TTL_HOURS required")
                .parse()
                .expect("Invalid configuration: HCC_SESSION_TTL_HOURS must be a number"),
            session_secret: env::var("HCC_SESSION_SECRET")
                .expect("Invalid configuration: HCC_SESSION_SECRET required"),
            jwt_rsa_private_key_path: env::var("HCC_JWT_RSA_PRIVATE_KEY_PATH")
                .expect("Invalid configuration: HCC_JWT_RSA_PRIVATE_KEY_PATH required"),
            jwt_rsa_public_key_path: env::var("HCC_JWT_RSA_PUBLIC_KEY_PATH")
                .expect("Invalid configuration: HCC_JWT_RSA_PUBLIC_KEY_PATH required"),
            postgres_sql_connection_url: env::var("HCC_POSTGRES_SQL_CONNECTION_URL")
                .expect("Invalid configuration: HCC_POSTGRES_SQL_CONNECTION_URL required"),
            bind_url: env::var("HCC_BIND_URL")
                .expect("Invalid configuration: HCC_BIND_URL required"),
            super_user_email: env::var("HCC_SUPER_USER_EMAIL")
                .expect("Invalid configuration: HCC_SUPER_USER_EMAIL required"),
            super_user_password: env::var("HCC_SUPER_USER_PASSWORD")
                .expect("Invalid configuration: HCC_SUPER_USER_PASSWORD required"),
        }
    }

    pub async fn new(server_config: &ServerConfig) -> Result<ServerWiring> {
        let config = server_config.to_owned();
        let server_state = ServerWiring {
            services: ServiceWiring {
                jwt_util: Arc::new(ServiceWiring::jwt_util(&config)),
            },
            config: config,
        };
        Ok(server_state)
    }
}

#[derive(Clone)]
pub struct ServiceWiring {
    pub jwt_util: Arc<JsonWebTokenUtil>,
}

impl ServiceWiring {
    pub fn jwt_util(config: &ServerConfig) -> JsonWebTokenUtil {
        let rsa_secrets = JsonWebTokenSecrets::read_keys(
            &config.jwt_rsa_private_key_path,
            &config.jwt_rsa_public_key_path,
        );

        JsonWebTokenUtil {
            secrets: rsa_secrets,
            issuer: String::from(&config.domain),
            expiry_duration_millis: (config.session_ttl_hours * 1000 * 60 * 60) as i64,
        }
    }
}
