use anyhow::Result;
use axum::{
    Router,
    extract::{Json, Path, State},
    routing::{get, post},
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sp_core::crypto::{AccountId32, Ss58Codec};
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

async fn list_modules(
    State(state): State<AppState>,
    _: Version,
) -> Result<axum::Json<Vec<Module>>, ApiError> {
    let modules = Module::iter(&state.api).await?;

    Ok(axum::Json(modules))
}

async fn get_module(
    State(state): State<AppState>,
    _: Version,
    Path((_, id)): Path<(String, u64)>,
) -> Result<axum::Json<Module>, ApiError> {
    let module = Module::get(&state.api, id).await?;

    Ok(axum::Json(module))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSignature {
    address: String,
    signature: String,
}

impl From<&serde_json::Value> for ServerSignature {
    fn from(value: &serde_json::Value) -> Self {
        let address = value.get("address");
        let signature = value.get("signature");
        Self {
            address: match address {
                Some(value) => value.as_str().unwrap().to_string(),
                None => String::new(),
            },
            signature: match signature {
                Some(value) => value.as_str().unwrap().to_string(),
                None => String::new(),
            },
        }
    }
}

async fn verify_signature(
    State(_state): State<AppState>,
    _: Version,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let data = payload
        .get("data")
        .ok_or(anyhow::anyhow!("Missing `data` field"))?;
    let server = payload
        .get("server")
        .ok_or(anyhow::anyhow!("Missing `server` field"))?;

    let address = server.get("address").and_then(|v| v.as_str());
    let signature = server.get("signature").and_then(|v| v.as_str());

    if address.is_none() {
        return Err(anyhow::anyhow!("Missing `address` field in `server`").into());
    }
    if signature.is_none() {
        return Err(anyhow::anyhow!("Missing `signature` field in `server`").into());
    }

    let server_sig: ServerSignature = server.into();

    log::info!("{:?}", server_sig.signature.as_bytes());
    log::info!("{:?}", server_sig.signature[2..].as_bytes());
    let bytes = hex::decode(server_sig.signature[2..].as_bytes())?;
    let address =
        AccountId32::from_ss58check(&server_sig.address).expect("Failed to decode SS58 Address");

    let message = data.as_str().expect("`data` should be a string").as_bytes();

    let sig = schnorrkel::Signature::from_bytes(&bytes).expect("Schnorrkel Failed to decode Signature");
    let public = schnorrkel::PublicKey::from_bytes(&*address.as_ref()).expect("Schnorrkel Failed to decode Public Key");
    let valid = public.verify_simple(b"substrate", message, &sig).is_ok();

    log::info!("{:?}", message);
    log::info!("{:?}", sig);
    log::info!("{:?}", public);

    Ok(Json(serde_json::json!({ "valid": valid })))
}

#[derive(Clone)]
pub struct AppState {
    api: OnlineClient<SubstrateConfig>,
}

impl AppState {
    async fn new() -> anyhow::Result<Self> {
        let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;
        Ok(Self { api })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _ = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .try_init();

    let state = AppState::new().await?;

    let module_routes = Router::new()
        .route("/", get(list_modules))
        .route("/{id}", get(get_module));

    let api = Router::new()
        .nest("/modules", module_routes)
        .route("/verify", post(verify_signature));

    let app = Router::new()
        .nest("/{version}", api)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
