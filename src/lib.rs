use axum::{Json, Router, http::StatusCode, routing::get};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub fn create_app() -> Router {
    let dist_path = std::env::var("DIST_PATH")
        .unwrap_or_else(|_| format!("{}/depot/dist", env!("CARGO_MANIFEST_DIR")));

    let api_routes = Router::new()
        .route("/ping", get(ping))
        .layer(TraceLayer::new_for_http())
        .fallback(api_404);

    Router::new().nest("/api", api_routes).fallback_service(
        ServeDir::new(&dist_path)
            .not_found_service(ServeFile::new(format!("{}/index.html", dist_path))),
    )
}

async fn ping() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
}

async fn api_404() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "Not found"})),
    )
}
