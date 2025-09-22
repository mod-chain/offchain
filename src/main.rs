use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use subxt::{OnlineClient, SubstrateConfig};

mod modchain;
use modchain::{chain, Module};

struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("API Error: {}", self.0)
        ).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}


async fn list_modules() -> Result<axum::Json<Vec<Module>>, ApiError> {
    let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;
    let mut modules: Vec<Module> = Vec::new();
    let storage_query = chain::storage().modules().modules_iter();
    let mut results = api.storage().at_latest().await?.iter(storage_query).await?;

    while let Some(Ok(kv)) = results.next().await {
        modules.push(kv.value.into());
    }
    Ok(axum::Json(modules))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(list_modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}