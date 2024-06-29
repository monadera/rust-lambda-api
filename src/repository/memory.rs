use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::error::Error;
use crate::repository::{Currency, Repository};

pub struct InMemoryRepository {
    storage: RwLock<HashMap<String, Currency>>,
}

impl Default for InMemoryRepository {
    fn default() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl Repository for InMemoryRepository {
    async fn add_currency(&self, currency: Currency) -> crate::error::Result<Currency> {
        let mut storage = self.storage.write().await;
        storage.insert(currency.code.to_lowercase(), currency.clone());

        Ok(currency)
    }

    async fn get_currency(&self, code: &str) -> crate::error::Result<Currency> {
        let storage = self.storage.read().await;
        storage
            .get(&code.to_lowercase())
            .cloned()
            .ok_or(Error::NotFound(code.to_string()))
    }

    async fn delete_currency(&self, code: &str) -> crate::error::Result<Currency> {
        let mut storage = self.storage.write().await;
        storage
            .remove(&code.to_lowercase())
            .ok_or(Error::NotFound(code.to_string()))
    }
}
