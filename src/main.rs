use depot::create_app;
use tokio::net::TcpListener;
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
