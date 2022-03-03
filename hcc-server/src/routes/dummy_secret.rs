use tide::{http::mime, Request, Result};

use crate::wiring::ServerWiring;
use crate::util::encryption;

pub async fn get(req: Request<ServerWiring>) -> Result {
    let secrets = req.ext::<encryption::SharedKeyring>().unwrap();

    let secret_message_server = "hello from server";
    let secret_message_client = "hello from client";

    let broadcast_message = secrets.encrypt_broadcast(secret_message_server).await?;
    let client_message = secrets.encrypt_user(secret_message_client).await?;

    let decrypted_broadcast = broadcast_message.decrypt(secrets)?;
    let decrypted_client = client_message.decrypt(secrets)?;

    println!("decrypted from server: {}", decrypted_broadcast);
    println!("decrypted from client: {}", decrypted_client);

    let body = String::from("<div>YOU ARE AUTHORIZED!</div>");
    let encrypted_body = encryption::encrypt_str(&body, secrets).await.unwrap();

    Ok(tide::Response::builder(200)
        .content_type(mime::PLAIN)
        .body_string(encrypted_body)
        .build())
}
