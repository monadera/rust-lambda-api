mod endpoints;
mod payload;

use poem::{Endpoint, EndpointExt, Route};
use poem_openapi::OpenApiService;

use crate::api::endpoints::CurrencyApi;
use crate::repository::SharedRepository;

pub fn build_app(repository: SharedRepository) -> anyhow::Result<impl Endpoint> {
    let api_service =
        OpenApiService::new(CurrencyApi, "Currencies", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(repository);

    Ok(app)
}

#[cfg(test)]
mod tests {
    use poem::http::StatusCode;
    use poem::test::TestClient;
    use poem::Endpoint;
    use std::sync::Arc;

    use crate::build_app;
    use crate::repository::{Currency, InMemoryRepository};

    fn setup_client() -> TestClient<impl Endpoint> {
        let repository = Arc::new(InMemoryRepository::default());
        let app = build_app(repository).expect("app to be created successfully");
        TestClient::new(app)
    }

    #[tokio::test]
    async fn test_get_currency() {
        let client = setup_client();
        let ccy = Currency {
            code: "GBP".to_string(),
            name: "British Pound".to_string(),
            symbol: "£".to_string(),
        };
        let response = client.post("/api/currencies").body_json(&ccy).send().await;
        response.assert_status_is_ok();

        let response = client.get("/api/currencies/gbp").send().await;
        response.assert_status_is_ok();
        response.assert_json(ccy).await;
    }

    #[tokio::test]
    async fn test_get_non_existent() {
        let client = setup_client();
        let response = client.get("/api/currencies/eur").send().await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_then_delete() {
        let client = setup_client();
        let ccy = Currency {
            code: "GBP".to_string(),
            name: "British Pound".to_string(),
            symbol: "£".to_string(),
        };
        let response = client.post("/api/currencies").body_json(&ccy).send().await;
        response.assert_status_is_ok();

        let response = client.delete("/api/currencies/gbp").send().await;
        response.assert_status_is_ok();
    }

    #[tokio::test]
    async fn test_delete_non_existent() {
        let client = setup_client();
        let response = client.delete("/api/currencies/eur").send().await;
        response.assert_status(StatusCode::NOT_FOUND);
    }
}
