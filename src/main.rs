mod handlers;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world." }))
        .route("/header/:name", get(handlers::header::get))
        .route("/square/:number", get(handlers::square::get))
        .route("/sum", get(handlers::sum::get_query))
        .route("/sum", post(handlers::sum::post))
        .route("/sum/:first/:second", get(handlers::sum::get))
        .route("/user-agent", get(handlers::header::get_user_agent))
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
