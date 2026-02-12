use axum::{Json, Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{Level, event};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let addr = listener.local_addr().unwrap();
    event!(Level::INFO, "Server running on http://{}", addr);

    let app = create_app();
    axum::serve(listener, app).await.unwrap();
}

pub fn create_app() -> Router {
    let api_routes = Router::new()
        .route("/ping", get(ping))
        .layer(TraceLayer::new_for_http())
        .fallback(api_404);

    Router::new().nest("/api", api_routes).fallback_service(
        ServeDir::new("depot/dist").not_found_service(ServeFile::new("depot/dist/index.html")),
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn spa_fallback() {
        let server = TestServer::new(create_app()).unwrap();
        let response = server.get("/dummy").await;

        assert!(response.text().starts_with("<!doctype"));
    }
}
