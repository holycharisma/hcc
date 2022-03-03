use tide::{http::mime, Request, Result};

use crate::wiring::ServerWiring;
use crate::util::encryption::SharedKeyring;

pub async fn get(req: Request<ServerWiring>) -> Result {
    let bundle = req.ext::<SharedKeyring>().unwrap();

    let secret_message_server = "hello from server";
    let secret_message_client = "hello from client";

    let broadcast_message = bundle.encrypt_broadcast(secret_message_server).await?;
    let client_message = bundle.encrypt_user(secret_message_client).await?;

    let decrypted_broadcast = broadcast_message.decrypt(bundle)?;
    let decrypted_client = client_message.decrypt(bundle)?;

    println!("decrypted from server: {}", decrypted_broadcast);
    println!("decrypted from client: {}", decrypted_client);

    Ok(tide::Response::builder(200)
        .content_type(mime::PLAIN)
        .body_string(String::from("YOU ARE AUTHORIZED!"))
        .build())
}
