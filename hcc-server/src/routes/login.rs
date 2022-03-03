use tide::prelude::*;
use tide::{http::mime, Request, Response, Result};

use crate::util::encryption::{self, UserEncryptedMessage};
use crate::wiring::ServerWiring;
use domain::session::SessionUser;

#[derive(Debug, Deserialize)]
struct UserLoginDto {
    email: String,
    password: String,
}

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "app.html.j2")] // using the template in this path, relative
struct AppView {
    user: SessionUser,
}

#[derive(Template)] // this will generate the code...
#[template(path = "login.html.j2")] // using the template in this path, relative
struct LoginGetView {}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let maybe_user: Option<&SessionUser> = req.ext();

    if maybe_user.is_some() {
        let user = maybe_user.unwrap().to_owned();

        let jwt_util = &req.state().services.jwt_util.clone();

        let auth_token = jwt_util.sign_auth_token(&user.email);

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();
        let encrypted = secrets
            .encrypt_broadcast(&auth_token.unwrap())
            .await
            .unwrap();

        let app_view = AppView { user: user };

        let encrypted_body = encryption::encrypt_str(&app_view.render().unwrap(), secrets)
            .await
            .unwrap();

        let response = Response::builder(200)
            .content_type(mime::PLAIN)
            .header("x-auth-token", encrypted.message)
            .body_string(encrypted_body)
            .build();

        Ok(response)
    } else {
        let login_get_view = LoginGetView {};
        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let encrypted_body = encryption::encrypt_str(&login_get_view.render().unwrap(), secrets)
            .await
            .unwrap();

        let response = Response::builder(200)
            .content_type(mime::HTML)
            .body_string(encrypted_body)
            .build();
        Ok(response)
    }
}

pub async fn post(mut req: Request<ServerWiring>) -> Result {
    let form = {
        let encrypted_form: UserLoginDto = req.body_form().await?;

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let sender = &secrets.user;

        let encrypted_email = UserEncryptedMessage {
            sender: sender.to_owned(),
            message: encrypted_form.email,
        };

        let encrypted_password = UserEncryptedMessage {
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
        let jwt_util = &req.state().services.jwt_util.clone();
        let session = req.session_mut();

        let user = SessionUser {
            email: String::from(&form.email),
        };

        let auth_token = jwt_util.sign_auth_token(&user.email);

        let _res = session.insert("user", user.clone()).unwrap();

        let app_view = AppView { user: user };

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let encrypted = secrets
            .encrypt_broadcast(&auth_token.unwrap())
            .await
            .unwrap();

        let encrypted_body = encryption::encrypt_str(&app_view.render().unwrap(), secrets)
            .await
            .unwrap();

        let response = Response::builder(200)
            .content_type(mime::PLAIN)
            .header("x-auth-token", encrypted.message)
            .body_string(encrypted_body)
            .build();
        Ok(response)
    } else {
        tide::log::info!("Failed login for user: {}", form.email);
        let response = Response::builder(403).build();
        Ok(response)
    }
}
