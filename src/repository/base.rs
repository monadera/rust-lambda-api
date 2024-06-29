use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type SharedRepository = Arc<dyn Repository>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[async_trait::async_trait]
pub trait Repository: Sync + Send + 'static {
    async fn add_currency(&self, currency: Currency) -> crate::error::Result<Currency>;
    async fn get_currency(&self, code: &str) -> crate::error::Result<Currency>;
    async fn delete_currency(&self, code: &str) -> crate::error::Result<Currency>;
}
