use anyhow::Result;
use std::sync::Arc;

use serverless_rust_api::api::build_app;
use serverless_rust_api::repository::DynamoDbRepository;
use serverless_rust_api::settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .json()
        .init();
    let settings = envy::from_env::<Settings>()?;
    let table_name = settings.table_name.expect("table name to be specified");

    let repository = Arc::new(DynamoDbRepository::new(table_name).await);
    let app = build_app(repository)?;
    poem_lambda::run(app).await.expect("app to start correctly");

    Ok(())
}
