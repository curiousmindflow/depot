use axum::http::StatusCode;
use axum_test::TestServer;
use depot::create_app;

#[tokio::test]
async fn test_spa_fallback() {
    let server = TestServer::new(create_app()).unwrap();
    let response = server.get("/dummy").await;

    // FIXME: when path is not "/" the spa is served but a 404 error is returned
    // assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response.text().starts_with("<!doctype"));
}

#[tokio::test]
async fn test_ping() {
    let server = TestServer::new(create_app()).unwrap();
    let response = server.get("/api/ping").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(response.text().contains("ok"));
}

#[tokio::test]
async fn test_api_404() {
    let server = TestServer::new(create_app()).unwrap();
    let response = server.get("/api/unknown").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
