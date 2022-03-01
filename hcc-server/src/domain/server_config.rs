
#[derive(Clone)]
pub struct ServerConfig {
    pub domain: String,
    pub session_cookie_name: String,
    pub session_ttl_hours: u32,
    pub session_secret: String,
    pub jwt_rsa_private_key_path: String,
    pub jwt_rsa_public_key_path: String,
    pub postgres_sql_connection_url: String,
    pub bind_url: String,
    pub super_user_email: String,
    pub super_user_password: String
}