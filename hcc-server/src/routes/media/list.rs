use tide::prelude::*;
use tide::{http::mime, Redirect, Request, Response, Result};

use crate::dao;
use crate::util::emoji;
use crate::util::encryption;
use crate::util::password::PasswordUtil;
use crate::wiring::ServerWiring;

use domain::session::SessionUser;

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "media/list.html.j2")] // using the template in this path, relative
struct ListGetViewModel {}

pub async fn get(req: Request<ServerWiring>) -> Result {
       let view_context = ListGetViewModel {};

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let encrypted_body = secrets
            .encrypt_broadcast_emoji(&view_context.render().unwrap())
            .await
            .unwrap()
            .message;

        let response = Response::builder(200)
            .content_type(mime::HTML)
            .body_string(encrypted_body)
            .build();
        Ok(response)
}


