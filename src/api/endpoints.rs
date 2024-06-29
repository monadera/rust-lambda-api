use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use tracing::info;

use crate::api::payload::{Currency, Response};
use crate::repository;

pub struct CurrencyApi;

#[OpenApi]
impl CurrencyApi {
    #[oai(path = "/currencies", method = "post")]
    async fn create(
        &self,
        Data(repository): Data<&repository::SharedRepository>,
        Json(payload): Json<Currency>,
    ) -> Response {
        info!(code = payload.code, "creating currency");

        let currency = repository::Currency {
            code: payload.code,
            name: payload.name,
            symbol: payload.symbol,
        };
        repository.add_currency(currency).await.into()
    }

    #[oai(path = "/currencies/:code", method = "get")]
    async fn get(
        &self,
        Data(repository): Data<&repository::SharedRepository>,
        Path(code): Path<String>,
    ) -> Response {
        info!(code, "getting currency");
        repository.get_currency(&code).await.into()
    }

    #[oai(path = "/currencies/:code", method = "delete")]
    async fn delete(
        &self,
        Data(repository): Data<&repository::SharedRepository>,
        Path(code): Path<String>,
    ) -> Response {
        info!(code, "deleting currency");
        repository.delete_currency(&code).await.into()
    }
}
