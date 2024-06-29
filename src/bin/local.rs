use anyhow::Result;
use poem::listener::TcpListener;
use std::sync::Arc;

use serverless_rust_api::api::build_app;
use serverless_rust_api::repository::InMemoryRepository;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let repository = Arc::new(InMemoryRepository::default());
    let app = build_app(repository)?;
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
