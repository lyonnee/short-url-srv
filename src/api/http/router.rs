use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use super::{app_handler, shorten_handler, user_handler};

pub fn new() -> Router {
    let router = Router::new().nest(
        "/api",
        Router::new()
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    tracing::info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                }),
            )
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
