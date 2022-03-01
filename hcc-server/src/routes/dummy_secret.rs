use tide::{http::mime, Request, Result};

use crate::wiring::ServerWiring;

pub async fn get(_req: Request<ServerWiring>) -> Result {
    Ok(tide::Response::builder(200)
                .content_type(mime::PLAIN)
                .body_string(String::from("YOU ARE AUTHORIZED!"))
                .build())
}
