use crate::wiring::ServerWiring;
use domain::session::SessionUser;

#[derive(Default)]
pub struct UserExtensionMiddleware {}

impl UserExtensionMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<ServerWiring> for UserExtensionMiddleware {
    async fn handle(
        &self,
        mut req: tide::Request<ServerWiring>,
        next: tide::Next<'_, ServerWiring>,
    ) -> tide::Result {

        // if we find a registered user: 
        // - put the user in the request context
        // - sign an auth token for them 
        //   - give it back to the client in our response header

        let maybe_user: Option<SessionUser> = req.session().get("user");

        let auth_token = 
            if maybe_user.is_some() {
                let user = maybe_user.unwrap();
                let maybe_user = req.state().services.jwt_util.sign_auth_token(&user.email);
                req.set_ext(user);
                match maybe_user {
                    Ok(token) => Some(token),
                    _ => None
                }
            } else {
                None
            };

        let mut response = next.run(req).await;

        if auth_token.is_some() {
            response.insert_header("x-auth-token", auth_token.unwrap());
            Ok(response)
        } else {
            Ok(response)
        }
    }
}


#[derive(Default)]
pub struct UserAuthorizationMiddleware {}

impl UserAuthorizationMiddleware {
    pub fn new() -> Self {
        Self {}
    }

    fn unauthorized() -> tide::Result<tide::Response> {
        Ok(tide::Response::builder(403).build())
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<ServerWiring> for UserAuthorizationMiddleware {
    async fn handle(
        &self,
        req: tide::Request<ServerWiring>,
        next: tide::Next<'_, ServerWiring>,
    ) -> tide::Result {
        let maybe_user: Option<&SessionUser> = req.ext();
        if maybe_user.is_some() {
            let user = maybe_user.unwrap();
            let maybe_header = req.header("x-auth-token");
            if maybe_header.is_some() {
                let maybe_token_text = maybe_header.unwrap().into_iter().next();
                if maybe_token_text.is_some() {
                    let jwt_util = &req.state().services.jwt_util;

                    let verification = jwt_util .verify_auth_token(maybe_token_text.unwrap().as_str(), &user.email);
                    if verification.is_ok() {
                        Ok(next.run(req).await)
                    } else {
                        tide::log::info!("Invalid authorization token");
                        UserAuthorizationMiddleware::unauthorized()
                    }
                } else {
                    tide::log::info!("Missing authorization token");
                    UserAuthorizationMiddleware::unauthorized()
                }
            } else {
                tide::log::info!("Missing authorization token");
                UserAuthorizationMiddleware::unauthorized()
            }
        } else {
            tide::log::info!("Missing required session user");
            UserAuthorizationMiddleware::unauthorized()
        }

    }
}
