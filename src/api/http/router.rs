use axum::{
    routing::{get, post},
    Router,
};

use super::{app_handler, shorten_handler, user_handler};

pub fn new() -> Router {
    let router = Router::new().nest(
        "/api",
        Router::new()
            .nest(
                "/user",
                Router::new()
                    .route("/signup", post(user_handler::registration))
                    .route("/login", post(user_handler::login)),
            )
            .nest(
                "/app",
                Router::new().route("/create", post(app_handler::create_app)),
            )
            .nest(
                "",
                Router::new()
                    .route("/shorten", post(shorten_handler::shorten))
                    .route("/:key", get(shorten_handler::redirect)),
            ),
    );

    return router;
}
