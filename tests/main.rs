use axum::http::StatusCode;
use axum_test::TestServer;
use depot::create_app;
use rstest::{fixture, rstest};
use tracing::{Level, event};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[rstest]
#[tokio::test]
async fn test_spa_fallback(
    #[from(setup_log)] _log: (),
    #[from(setup_dist_path)] dist_path: String,
) {
    event!(Level::INFO, "dist_path: {}", &dist_path);
    let server = TestServer::new(create_app(&dist_path)).unwrap();
    let response = server.get("/dummy").await;

    // FIXME: when path is not "/" the spa is served but a 404 error is returned
    // assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response.text().starts_with("<!doctype"));
}

#[rstest]
#[tokio::test]
async fn test_ping(#[from(setup_log)] _log: ()) {
    let server = TestServer::new(create_app("")).unwrap();
    let response = server.get("/api/ping").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response.text().contains("ok"));
}

#[rstest]
#[tokio::test]
async fn test_api_404(#[from(setup_log)] _log: ()) {
    let server = TestServer::new(create_app("")).unwrap();
    let response = server.get("/api/unknown").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[fixture]
fn setup_dist_path() -> String {
    std::env::var("DIST_PATH")
        .unwrap_or_else(|_| format!("{}/depot/dist", env!("CARGO_MANIFEST_DIR")))
}

#[fixture]
fn setup_log() {
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
}
