use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Object};

use crate::error::{Error, Result};

#[derive(Object)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

impl From<crate::repository::Currency> for Currency {
    fn from(currency: crate::repository::Currency) -> Self {
        Self {
            code: currency.code,
            name: currency.name,
            symbol: currency.symbol,
        }
    }
}

#[derive(ApiResponse)]
pub(crate) enum Response {
    #[oai(status = 200)]
    Currency(Json<Currency>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

impl From<Result<crate::repository::Currency>> for Response {
    fn from(result: Result<crate::repository::Currency>) -> Self {
        match result {
            Ok(currency) => Response::Currency(Json(currency.into())),
            Err(err) => match err {
                Error::NotFound(code) => {
                    let msg = format!("{code} is not found");
                    Response::NotFound(PlainText(msg))
                }
                Error::Other => {
                    let msg = "internal server error".to_string();
                    Response::InternalServerError(PlainText(msg))
                }
            },
        }
    }
}
