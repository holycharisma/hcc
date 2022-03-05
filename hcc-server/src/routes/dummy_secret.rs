use tide::{http::mime, Request, Result};

use crate::wiring::ServerWiring;
use crate::util::encryption;

pub async fn get(req: Request<ServerWiring>) -> Result {
    let secrets = req.ext::<encryption::SharedKeyring>().unwrap();

    let body = String::from("<div>YOU ARE AUTHORIZED!</div>");
    let encrypted_body = encryption::encrypt_str_emoji(&body, secrets).await.unwrap();

    Ok(tide::Response::builder(200)
        .content_type(mime::PLAIN)
        .body_string(encrypted_body)
        .build())
}
