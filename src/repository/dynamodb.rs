use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::types::ReturnValue::AllOld;
use aws_sdk_dynamodb::Client;
use serde_dynamo::{from_item, to_item};
use tracing::error;

use crate::error::Error;
use crate::repository::{Currency, Repository};

pub struct DynamoDbRepository {
    client: Client,
    table_name: String,
}

impl DynamoDbRepository {
    pub async fn new(table_name: String) -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Self { client, table_name }
    }
}

#[async_trait::async_trait]
impl Repository for DynamoDbRepository {
    async fn add_currency(&self, currency: Currency) -> crate::error::Result<Currency> {
        let item = to_item(&currency).unwrap();
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .item("pk", AttributeValue::S(currency.code.to_lowercase()))
            .send()
            .await
            .map_err(|err| {
                error!(err = debug(err), "failed to create currency");
                Error::Other
            })?;

        Ok(currency)
    }

    async fn get_currency(&self, code: &str) -> crate::error::Result<Currency> {
        let output = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(code.to_lowercase()))
            .send()
            .await
            .map_err(|err| {
                error!(err = debug(err), "failed to get currency");
                Error::Other
            })?;

        if let Some(item) = output.item {
            let currency = from_item(item).map_err(|_| Error::Other)?;
            Ok(currency)
        } else {
            Err(Error::NotFound(code.to_string()))
        }
    }

    async fn delete_currency(&self, code: &str) -> crate::error::Result<Currency> {
        let output = self
            .client
            .delete_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(code.to_lowercase()))
            .return_values(AllOld)
            .send()
            .await
            .map_err(|err| {
                error!(err = debug(err), "failed to delete currency");
                Error::Other
            })?;

        if let Some(item) = output.attributes {
            let currency = from_item(item).map_err(|_| Error::Other)?;
            Ok(currency)
        } else {
            Err(Error::NotFound(code.to_string()))
        }
    }
}
