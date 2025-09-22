use axum::{Router, extract::Path, routing::get};
use dotenv::dotenv;
use subxt::{OnlineClient, SubstrateConfig};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod error;
use error::ApiError;
mod version;
use version::Version;
mod modchain;
use modchain::Module;

async fn list_modules(_: Version) -> Result<axum::Json<Vec<Module>>, ApiError> {
    let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

    let modules = Module::iter(&api).await?;

    Ok(axum::Json(modules))
}

#[axum::debug_handler]
async fn get_module(_: Version, id: Path<(String, u64)>) -> Result<axum::Json<Module>, ApiError> {
    let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

    let module = Module::get(&api, id.0.1).await?;

    Ok(axum::Json(module))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .try_init();

    let api = Router::new().nest(
        "/modules",
        Router::new()
            .route("/", get(list_modules))
            .route("/{id}", get(get_module)),
    );

    let app = Router::new().nest("/{version}", api).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            ),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
