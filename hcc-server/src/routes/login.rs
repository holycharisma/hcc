use tide::prelude::*;
use tide::{http::mime, Redirect, Request, Response, Result};

use crate::util::emoji;
use crate::util::encryption;
use crate::util::password::PasswordUtil;
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

        let encrypted_body = secrets
            .encrypt_broadcast_emoji(&login_get_view.render().unwrap())
            .await
            .unwrap()
            .message;

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

        let encrypted_email = encryption::UserEncryptedEmojiMessage {
            sender: sender.to_owned(),
            message: encrypted_form.email,
        };

        let encrypted_password = encryption::UserEncryptedEmojiMessage {
            sender: sender.to_owned(),
            message: encrypted_form.password,
        };

        UserLoginDto {
            email: encrypted_email.decrypt(secrets).unwrap(),
            password: encrypted_password.decrypt(secrets).unwrap(),
        }
    };

    let super_user_email = &req.state().config.super_user_email;
    let super_user_password = &req.state().config.super_user_pwhash_emoji;

    let top_secret_email = {
        encryption::seal_with_view_key_emoji(
            &req.state().config.encryption_key_emoji,
            &req.state().config.encryption_view_key_emoji,
            &form.email.as_bytes(),
        )
        .unwrap()
    };

    let user_pwhash = emoji::decode(super_user_password);

    let expected_super_user_email = {
        encryption::seal_with_view_key_emoji(
            &req.state().config.encryption_key_emoji,
            &req.state().config.encryption_view_key_emoji,
            super_user_email.as_bytes(),
        )
        .unwrap()
    };

    let email_is_valid = top_secret_email == expected_super_user_email;

    let pass_is_valid =  PasswordUtil::verify_hashed_bytes(&form.password, &user_pwhash);

    if email_is_valid && pass_is_valid {
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
