use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use super::{app_handler, auth, middleware, shorten_handler};

pub fn new() -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let router = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/user",
                    Router::new()
                        .route("/signup", post(auth::register))
                        .route("/login", post(auth::login)),
                )
                .nest(
                    "/app",
                    Router::new()
                        .route("/", post(app_handler::create_app)).layer(axum::middleware::from_fn(middleware::jwt::authentication))
                        .route("/list/:page/:size", get(app_handler::get_app_list)).layer(axum::middleware::from_fn(middleware::jwt::authentication)),
                )
                .nest(
                    "",
                    Router::new()
                        .route("/shorten", post(shorten_handler::shorten)).layer(axum::middleware::from_fn(middleware::jwt::authentication))
                        .route("/:key", get(shorten_handler::redirect)),
                ),
        )
        .layer(trace_layer);

    return router;
}
