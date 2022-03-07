use tide::prelude::*;
use tide::{http::mime, Request, Response, Result, Redirect};

use crate::util::encryption::{self, UserEncryptedEmojiMessage};
use crate::wiring::ServerWiring;
use domain::session::SessionUser;

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "login.html.j2")] // using the template in this path, relative
struct LoginGetView {}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let maybe_user: Option<&SessionUser> = req.ext();

    if maybe_user.is_some() {
        Ok(Redirect::new("/app").into())
    } else {
        let login_get_view = LoginGetView {};
        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let encrypted_body = encryption::encrypt_str_emoji(&login_get_view.render().unwrap(), secrets)
            .await
            .unwrap();

        let response = Response::builder(200)
            .content_type(mime::HTML)
            .body_string(encrypted_body)
            .build();
        Ok(response)
    }
}


#[derive(Debug, Deserialize)]
struct UserLoginDto {
    email: String, // emoji encrypted fields
    password: String,
}

pub async fn post(mut req: Request<ServerWiring>) -> Result {
    let form = {
        let encrypted_form: UserLoginDto = req.body_form().await?;

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let sender = &secrets.user;

        let encrypted_email = UserEncryptedEmojiMessage {
            sender: sender.to_owned(),
            message: encrypted_form.email,
        };

        let encrypted_password = UserEncryptedEmojiMessage {
            sender: sender.to_owned(),
            message: encrypted_form.password,
        };

        UserLoginDto {
            email: encrypted_email.decrypt(secrets).unwrap(),
            password: encrypted_password.decrypt(secrets).unwrap(),
        }
    };

    let super_user_email = &req.state().config.super_user_email;
    let super_user_password = &req.state().config.super_user_password;

    if &form.email == super_user_email && &form.password == super_user_password {
        let session = req.session_mut();

        let user = SessionUser {
            email: String::from(&form.email),
        };

        let _res = session.insert("user", user.clone()).unwrap();

        // redirect to app now that we have set user
        Ok(Redirect::new("/app").into())
    } else {
        tide::log::info!("Failed login for user: {}", form.email);
        let response = Response::builder(403).build();
        Ok(response)
    }
}
