use tide::prelude::*;
use tide::{http::mime, Request, Response, Result};

use domain::session::SessionUser;
use crate::wiring::ServerWiring;

#[derive(Debug, Deserialize)]
struct UserLoginDto {
    email: String,
    password: String,
}

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "app.html.j2")] // using the template in this path, relative
struct AppView {
    user: SessionUser
}

#[derive(Template)] // this will generate the code...
#[template(path = "login.html.j2")] // using the template in this path, relative
struct LoginGetView {}

pub async fn get(req: Request<ServerWiring>) -> Result {

    let maybe_user: Option<&SessionUser> = req.ext();

    if maybe_user.is_some() {
        let app_view = AppView {
            user: maybe_user.unwrap().to_owned()
        };

        let response = Response::builder(200)
            .content_type(mime::PLAIN)
            .body_string(app_view.render().unwrap())
            .build();
        Ok(response)
    } else {

        let login_get_view = LoginGetView {};
        let response = Response::builder(200)
            .content_type(mime::HTML)
            .body_string(login_get_view.render().unwrap())
            .build();
        Ok(response)
    }

}

pub async fn post(mut req: Request<ServerWiring>) -> Result {
    let form: UserLoginDto = req.body_form().await?;

    let super_user_email = &req.state().config.super_user_email;
    let super_user_password = &req.state().config.super_user_password;

    if &form.email == super_user_email && &form.password == super_user_password {

        let jwt_util = &req.state().services.jwt_util.clone();

        let session = req.session_mut();

        let user = SessionUser {
            email: String::from(&form.email)
        };

        let auth_token = jwt_util.sign_auth_token(&user.email);

        let _res = session.insert("user", user.clone()).unwrap();

        let app_view = AppView {
            user: user
        };

        let response = Response::builder(200)
            .content_type(mime::PLAIN)
            .header("x-auth-token", auth_token.unwrap())
            .body_string(app_view.render().unwrap())
            .build();
        Ok(response)
    } else {
        tide::log::info!("Failed login for user: {}", form.email);
        let response = Response::builder(403).build();
        Ok(response)
    }
}
