mod auth;
mod profile;
mod settings;

use axum::{
    http::HeaderMap,
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    let app = Router::new()
        .route("/", get(home_page))
        .route("/register", get(auth::signup_page).post(auth::register))
        .route("/login", get(auth::login_page).post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/profile", get(profile::profile))
        .route("/profile", post(profile::update_profile))
        .nest("/settings", settings::settings_routes());

    return app;
}

pub async fn home_page(headers_map: HeaderMap) -> String {
    if crate::sessions::check_token(&headers_map) {
        "good".to_string()
    } else {
        "bad".to_string()
    }
}
