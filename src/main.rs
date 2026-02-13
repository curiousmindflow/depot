use depot::create_app;
use tokio::net::TcpListener;
use tracing::{Level, event};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    setup_log();

    let dist_path = std::env::var("DIST_PATH")
        .unwrap_or_else(|_| format!("{}/depot/dist", env!("CARGO_MANIFEST_DIR")));

    event!(Level::INFO, "dist_path: {}", &dist_path);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let addr = listener.local_addr().unwrap();
    event!(Level::INFO, "Server running on http://{}", addr);

    let app = create_app(&dist_path);
    axum::serve(listener, app).await.unwrap();
}

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
