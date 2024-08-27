use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use super::{app_handler, auth_handler, link_handler, middleware};

pub fn new() -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let router = Router::new()
        .route("/:key", get(link_handler::redirect))
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/user",
                    Router::new()
                        .route("/signup", post(auth_handler::register))
                        .route("/login", post(auth_handler::login)),
                )
                .nest(
                    "/app",
                    Router::new()
                        .route("/", post(app_handler::create_app))
                        .layer(axum::middleware::from_fn(middleware::jwt::authentication))
                        .route("/list/:page/:size", get(app_handler::get_app_list))
                        .layer(axum::middleware::from_fn(middleware::jwt::authentication)),
                )
                .nest(
                    "",
                    Router::new()
                        .route("/shorten", post(link_handler::shorten))
                        .layer(axum::middleware::from_fn(middleware::jwt::authentication)),
                ),
        )
        .layer(trace_layer);

    return router;
}
