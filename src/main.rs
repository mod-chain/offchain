use axum::{Router, routing::get};
use subxt::{OnlineClient, SubstrateConfig};

mod error;
use error::ApiError;
mod modchain;
use modchain::{Module, chain};

async fn list_modules() -> Result<axum::Json<Vec<Module>>, ApiError> {
    let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

    let modules = Module::iter(&api).await?;

    Ok(axum::Json(modules))
}

async fn get_module() -> Result<axum::Json<Module>, ApiError> {
    let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

    let module = Module::get(&api, 1).await?;

    Ok(axum::Json(module))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(list_modules))
        .route("/1", get(get_module));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
