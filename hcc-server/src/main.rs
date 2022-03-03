
mod routes;
mod util;
mod dao;
mod wiring;
mod middleware;

use wiring::ServerWiring;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let config = ServerWiring::init_server_config();

    // probably need to validate the config...
    // is the session secret legit byte key?
    // is the postgres url reachable?
    // are the jwt signing keys valid?
    let server_wiring = ServerWiring::new(&config).await?;

    dao::user::UserDao::insert_super_user(&config, &server_wiring).await.unwrap();

    let mut app = tide::with_state(server_wiring);

    let session_middleware = middleware::session::init_session_middleware(&config).await?;
    let anti_forgery_middleware = crate::middleware::security::AntiRequestForgeryMiddleware::new();
    let encryption_middleware = crate::middleware::security::SessionEncryptionMiddleware::new();
    let user_middleware = middleware::user::UserExtensionMiddleware::new();

    let user_authorization_middleware = middleware::user::UserAuthorizationMiddleware::new();

    app.with(session_middleware);
    app.with(encryption_middleware);
    app.with(user_middleware);
    app.with(anti_forgery_middleware);

    app.at("/").get(routes::index::get);

    // https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html
    app.at("/hcc_frame.js").get(routes::hcc_frame_js::get);

    app.at("/login")
        .get(routes::login::get)
        .post(routes::login::post);

    app.at("/logout").post(routes::logout::post);

    app.at("/api/secret")
        .with(user_authorization_middleware)
        .get(routes::dummy_secret::get);

    app.at("/hcc/*")
        .serve_dir("../hcc-client/dist/")
        .expect("Failed to load frontend assets");

    app.at("/favicon.svg")
        .serve_file("../hcc-client/assets/favicon.svg")
        .expect("No favicon found");

    app.listen(&config.bind_url).await?;
    
    Ok(())
}
