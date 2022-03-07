use tide::prelude::*;
use tide::{http::mime, Request, Response, Result, Redirect};

use crate::util::encryption::{self, UserEncryptedEmojiMessage};
use crate::wiring::ServerWiring;
use domain::session::SessionUser;

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "app.html.j2")] // using the template in this path, relative
struct AppView {
    user: SessionUser,
}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let maybe_user: Option<&SessionUser> = req.ext();

    if maybe_user.is_some() {
        let user = maybe_user.unwrap().to_owned();

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();
        
        let app_view = AppView { user: user };

        let encrypted_body = encryption::encrypt_str_emoji(&app_view.render().unwrap(), secrets)
            .await
            .unwrap();

        let response = Response::builder(200)
            .content_type(mime::PLAIN)
            .body_string(encrypted_body)
            .build();

        Ok(response)
    } else {
        Ok(Redirect::new("/login").into())
    }
}